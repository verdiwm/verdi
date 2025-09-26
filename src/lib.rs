#![allow(clippy::default_constructed_unit_structs)]

use std::{
    ffi::CString,
    os::fd::{BorrowedFd, IntoRawFd},
    path::PathBuf,
    sync::Arc,
};

use colpetto::{
    event::KeyState,
    helper::{
        Handle as LibinputHandle,
        event::{EventType, KeyboardEvent},
    },
};
use input_linux_sys::KEY_ESC;
use saddle::Seat;
use serde::{Deserialize, Serialize};
use tokio::{
    pin,
    sync::{RwLock, watch},
    task::JoinSet,
};
use tokio_stream::{
    StreamExt,
    wrappers::{UnboundedReceiverStream, WatchStream},
};
use tracing::{debug, error, info};

pub mod error;

pub mod keymap;
pub mod protocol;
pub mod wgpu_context;

use waynest::ObjectId;
use waynest_server::{Connection, Listener};
use wgpu_context::WgpuContext;

use crate::{
    error::VerdiError,
    keymap::{KeyMap, ModifierState},
    protocol::wayland::display::Display,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// Custom wayland socket path
    pub socket: Option<PathBuf>,
}

pub struct Verdi<'a> {
    seat: Seat,
    libinput_handle: colpetto::helper::Handle,
    libinput_event_stream:
        UnboundedReceiverStream<Result<colpetto::helper::event::Event, colpetto::Error>>,
    control_stream: WatchStream<bool>,
    wgpu_context: Option<WgpuContext<'a>>,
    listener: Listener,
    config: Config,
    clients: JoinSet<Result<(), error::VerdiError>>,
}

impl Verdi<'_> {
    pub async fn new(listener: Listener, config: Config) -> error::Result<Self> {
        let seat = Seat::new().await?;
        let seat_name = CString::new(seat.seat_name()).expect("Invalid seat name");

        let (libinput_handle, libinput_event_stream) = {
            let open_seat = seat.clone();
            let close_seat = seat.clone();

            LibinputHandle::new(
                move |path| {
                    let seat = open_seat.clone();

                    async move {
                        match seat.open_device(path).await {
                            Ok(fd) => fd.into_raw_fd(),
                            Err(err) => {
                                error!("Failed to open device: {err}");
                                -1
                            }
                        }
                    }
                },
                move |fd| {
                    let seat = close_seat.clone();

                    async move {
                        let _ = seat.close_device(unsafe { BorrowedFd::borrow_raw(fd) });
                    }
                },
                seat_name,
            )?
        };

        let (control_sx, control_rx) = watch::channel::<bool>(false);

        tokio::spawn({
            let seat = seat.clone();
            let libinput_handle = libinput_handle.clone();

            async move {
                let stream = seat.active_stream().await;

                pin!(stream);

                while let Some(is_active) = stream.try_next().await? {
                    if is_active {
                        info!("Session became active, taking control");
                        seat.aquire_session().await?;
                        control_sx.send(true)?;
                        libinput_handle.resume()?;
                    } else {
                        info!("Session became inactive");
                        seat.release_session().await?;
                        control_sx.send(false)?;
                        libinput_handle.suspend()?;
                    }
                }

                anyhow::Ok(())
            }
        });

        Ok(Self {
            seat,
            libinput_handle,
            libinput_event_stream,
            control_stream: WatchStream::new(control_rx),
            wgpu_context: None,
            listener,
            config,
            clients: JoinSet::new(),
        })
    }

    pub async fn run(self) -> error::Result<()> {
        let Self {
            seat,
            libinput_handle,
            mut libinput_event_stream,
            mut control_stream,
            mut wgpu_context,
            mut listener,
            config,
            mut clients,
        } = self;

        drop(config);

        tokio::spawn(async move {
            while let Some(stream) = listener.try_next().await.unwrap() {
                let mut client: Connection<VerdiError> = Connection::new(stream).unwrap();

                client.insert(ObjectId::DISPLAY, Display::default());

                clients.spawn(async move {
                    while let Some(mut message) = client.try_next().await? {
                        if let Err(err) = client
                            .get_raw(message.object_id())
                            .ok_or(VerdiError::MissingObject(message.object_id()))?
                            .dispatch_request(&mut client, message.object_id(), &mut message)
                            .await
                        {
                            error!("Error while handling message: {err}");
                            return Err(err.into());
                        }
                    }

                    Ok(())
                });
            }
        });

        let key_map = KeyMap::new();
        let modifier_state = Arc::new(RwLock::new(ModifierState::new()));

        let mut has_control = false;

        loop {
            tokio::select! {
                biased;
                Some(control) = control_stream.next() => {
                    if control != has_control {
                        has_control = control;

                        if has_control {
                            info!("Session activated - creating fresh rendering context");
                            match WgpuContext::new().await {
                                Ok(ctx) => wgpu_context = Some(ctx),
                                Err(e) => error!("Failed to create context: {}", e),
                            }
                        } else {
                            // Reset modifier state when we lose control to avoid stuck keys
                            *modifier_state.write().await = ModifierState::new();
                        }
                    }
                }
                Some(event) = libinput_event_stream.next() => {
                    match event {
                        Ok(event) => match event.event_type {
                            EventType::Keyboard(KeyboardEvent::Key { key, state, .. }) => {
                                let (should_check_vt_switch, is_ctrl_alt_pressed) = {
                                    let mut ms = modifier_state.write().await;
                                    ms.update(key, state);
                                    (state == KeyState::Pressed, ms.is_ctrl_alt_pressed())
                                };

                                if should_check_vt_switch {
                                    // Handle ESC for exit
                                    if key as i32 == KEY_ESC {
                                        libinput_handle.shutdown();
                                        break;
                                    }

                                    // Only process function keys when Ctrl+Alt are held
                                    if is_ctrl_alt_pressed && let Some(vt) = key_map.get_vt(key) {
                                        if has_control {
                                            info!("Ctrl+Alt+F{vt} pressed, attempting a VT switch to {vt}");

                                            if let Ok(current_vt) = seat.current_session().await && vt != current_vt {
                                                info!("Deactivating session - destroying rendering context completely");

                                                wgpu_context = None;

                                                if let Err(e) = seat.switch_session(vt).await {
                                                    error!("Failed to switch to VT {vt}: {e}");
                                                }
                                            }
                                        } else {
                                            debug!("Not switching VT - session inactive");
                                        }
                                    }
                                }
                            }
                            _ => {}
                        },
                        Err(_) => break,
                    }
                }
                else => {}
            };

            if let Some(ref context) = wgpu_context {
                if let Err(e) = context.present() {
                    error!("Present failed: {}", e);
                }
            }
        }

        Ok(())
    }
}
