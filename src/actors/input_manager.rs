use std::{
    ffi::CString,
    os::fd::{FromRawFd, IntoRawFd, OwnedFd},
    sync::mpsc as std_mpsc,
};

use colpetto::{Libinput, event::AsRawEvent};
use tokio::{sync::mpsc, task::LocalSet};
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::debug;

use crate::{
    CompositorHandle,
    actors::{compositor::InputEvent, session::SessionHandle},
};

pub enum InputManagerMessage {
    Suspend,
    Resume,
}

pub struct InputManager {
    sender: mpsc::Sender<InputManagerMessage>,
    receiver: mpsc::Receiver<InputManagerMessage>,
    compositor_handle: CompositorHandle,
    shutdown_token: CancellationToken,
    session_handle: SessionHandle,
}

#[derive(Clone)]
pub struct InputManagerHandle {
    sender: mpsc::Sender<InputManagerMessage>,
}

impl InputManager {
    pub fn new(
        compositor_handle: CompositorHandle,
        session_handle: SessionHandle,
        shutdown_token: CancellationToken,
    ) -> Self {
        let (sender, receiver) = mpsc::channel(128);

        Self {
            sender,
            receiver,
            compositor_handle,
            shutdown_token,
            session_handle,
        }
    }

    pub fn handle(&self) -> InputManagerHandle {
        InputManagerHandle::new(self.sender.clone())
    }

    pub async fn run(mut self) {
        let (open_request_sx, mut open_request_rx) = mpsc::unbounded_channel();
        let (open_response_sx, open_response_rx) = std_mpsc::channel();
        let (close_sx, mut close_rx) = mpsc::unbounded_channel();

        let session_handle = self.session_handle;
        let open_session = session_handle.clone();
        let close_session = session_handle.clone();

        tokio::spawn(async move {
            while let Some(path) = open_request_rx.recv().await {
                debug!("Bridge: opening device");
                let fd = match open_session.open_device(path).await {
                    Ok(owned_fd) => owned_fd.into_raw_fd(),
                    Err(_) => -1,
                };

                let _ = open_response_sx.send(fd);
            }
        });

        tokio::spawn(async move {
            while let Some(fd) = close_rx.recv().await {
                debug!("Bridge: closing device");
                let _ = close_session.close_device(fd).await;
            }
        });

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        std::thread::spawn(move || {
            let local = LocalSet::new();

            local.spawn_local(async move {
                let mut libinput = Libinput::with_tracing(
                    move |path, _| {
                        debug!("Trying to open device");
                        open_request_sx.send(path.to_owned()).map_err(|_| -1)?;
                        open_response_rx.recv().map_err(|_| -1)
                    },
                    move |fd| {
                        debug!("Trying to close device");
                        let _ = close_sx.send(unsafe { OwnedFd::from_raw_fd(fd) });
                    },
                )
                .unwrap();

                libinput
                    .udev_assign_seat(CString::new(session_handle.seat_name()).unwrap().as_c_str())
                    .unwrap();

                let mut stream = libinput.event_stream().unwrap();

                loop {
                    tokio::select! {
                        biased;
                        _ = self.shutdown_token.cancelled() => {
                            break
                        }
                        Some(res) = stream.next() => {
                            match res {
                                Ok(ref event) => {
                                     let _ = self
                                        .compositor_handle
                                        .input(InputEvent {
                                            name: event.event_type(),
                                            event_type: event.into(),
                                            device_name: event.device().name().to_string_lossy().to_string(),
                                        })
                                        .await;
                                },
                                Err(_) => break,
                            }
                        }
                        Some(msg) = self.receiver.recv() => {
                            match msg {
                                InputManagerMessage::Suspend => {
                                    libinput.suspend();
                                }
                                InputManagerMessage::Resume => {
                                    let _ = libinput.resume();
                                }
                            }
                        }
                    }
                }
            });

            rt.block_on(local);
        });
    }
}

impl InputManagerHandle {
    fn new(sender: mpsc::Sender<InputManagerMessage>) -> Self {
        Self { sender }
    }

    pub async fn suspend(&self) {
        let _ = self.sender.send(InputManagerMessage::Suspend).await;
    }

    pub async fn resume(&self) {
        let _ = self.sender.send(InputManagerMessage::Resume).await;
    }
}
