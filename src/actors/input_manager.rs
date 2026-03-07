use std::{
    ffi::CString,
    os::fd::{FromRawFd, IntoRawFd, OwnedFd},
    sync::mpsc as std_mpsc,
};

use colpetto::{Libinput, event::AsRawEvent};
use stagecraft::{ActorDead, Context, Handle, HasMailbox, LocalActor, LocalStreamActor};
use tokio::sync::mpsc;
use tracing::{debug, error, trace};

use crate::actors::{
    compositor::{Compositor, CompositorMessage, InputEvent},
    session::{SessionExt, SessionRef},
};

pub enum InputManagerMessage {
    Suspend,
    Resume,
}

pub struct InputManagerInit {
    pub compositor_handle: Handle<Compositor>,
    pub session_ref: SessionRef,
}

pub struct InputManager {
    compositor_handle: Handle<Compositor>,
    libinput: Libinput,
}

impl HasMailbox for InputManager {
    type Message = InputManagerMessage;
}

impl LocalActor for InputManager {
    type Init = InputManagerInit;

    async fn init(init: InputManagerInit, ctx: &mut Context<Self>) -> Self {
        debug!("Called input manager init");
        let (open_request_tx, mut open_request_rx) = mpsc::unbounded_channel::<CString>();
        let (open_response_tx, open_response_rx) = std_mpsc::channel();
        let (close_tx, mut close_rx) = mpsc::unbounded_channel::<OwnedFd>();

        let open_session = init.session_ref.clone();
        let close_session = init.session_ref.clone();

        // IMPORTANT: this need to be spawned in the original runtime otherwise we cause a deadlock
        ctx.track_main(async move {
            while let Some(path) = open_request_rx.recv().await {
                trace!("Bridge: opening device");
                let fd = match open_session.open_device(path).await {
                    Ok(owned_fd) => owned_fd.into_raw_fd(),
                    Err(_) => {
                        error!("Failed to open device, actor  dead");

                        -1
                    }
                };
                let _ = open_response_tx.send(fd);
            }
        });

        ctx.track_main(async move {
            while let Some(fd) = close_rx.recv().await {
                trace!("Bridge: closing device");
                let _ = close_session.close_device(fd).await;
            }
        });

        let mut libinput = Libinput::with_tracing(
            move |path, _| {
                trace!("Trying to open device");
                open_request_tx.send(path.to_owned()).map_err(|_| -1)?;
                open_response_rx.recv().map_err(|_| -1)
            },
            move |fd| {
                trace!("Trying to close device");
                let _ = close_tx.send(unsafe { OwnedFd::from_raw_fd(fd) });
            },
        )
        .unwrap();

        libinput
            .udev_assign_seat(
                CString::new(init.session_ref.seat_name())
                    .unwrap()
                    .as_c_str(),
            )
            .unwrap();

        Self {
            compositor_handle: init.compositor_handle,
            libinput,
        }
    }

    async fn handle_message(&mut self, msg: InputManagerMessage, _ctx: &mut Context<Self>) {
        match msg {
            InputManagerMessage::Suspend => {
                self.libinput.suspend();
            }
            InputManagerMessage::Resume => {
                let _ = self.libinput.resume();
            }
        }
    }
}

impl LocalStreamActor for InputManager {
    type Event = Result<colpetto::Event, colpetto::Error>;
    type Stream = colpetto::EventStream;

    async fn create_stream(&mut self, _ctx: &mut Context<Self>) -> Self::Stream {
        debug!("Called create stream for input manager");
        self.libinput.event_stream().unwrap()
    }

    async fn handle_event(
        &mut self,
        event: Result<colpetto::Event, colpetto::Error>,
        _ctx: &mut Context<Self>,
    ) {
        match event {
            Ok(ref event) => {
                let _ = self
                    .compositor_handle
                    .cast(CompositorMessage::Input(InputEvent {
                        name: event.event_type(),
                        event_type: event.into(),
                        device_name: event.device().name().to_string_lossy().to_string(),
                    }))
                    .await;
            }
            Err(e) => {
                tracing::error!("libinput error: {e}");
            }
        }
    }
}

pub trait InputManagerExt {
    fn suspend(&self) -> impl Future<Output = Result<(), ActorDead<()>>>;
    fn resume(&self) -> impl Future<Output = Result<(), ActorDead<()>>>;
}

impl InputManagerExt for Handle<InputManager> {
    async fn suspend(&self) -> Result<(), ActorDead<()>> {
        self.cast(InputManagerMessage::Suspend)
            .await
            .map_err(|_| ActorDead(()))
    }

    async fn resume(&self) -> Result<(), ActorDead<()>> {
        self.cast(InputManagerMessage::Resume)
            .await
            .map_err(|_| ActorDead(()))
    }
}
