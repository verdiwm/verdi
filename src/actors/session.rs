use std::{ffi::CString, os::fd::OwnedFd, pin::Pin};

use futures_core::Stream;
use saddle::Seat;

use stagecraft::{Actor, Context, Handle, HasMailbox, StreamActor};

use super::compositor::Compositor;

#[stagecraft::message(Session)]
pub enum SessionMessage {
    #[call(u32)]
    CurrentVt,
    SwitchVt {
        vt: u32,
    },
    #[call(OwnedFd)]
    OpenDevice {
        path: CString,
    },
    CloseDevice {
        fd: OwnedFd,
    },
    #[call]
    AcquireSession,
    #[call]
    ReleaseSession,
}

pub struct Session {
    pub(crate) seat: Seat,
    pub(crate) compositor_handle: Handle<Compositor>,
}

#[derive(Clone)]
pub struct SessionRef {
    handle: Handle<Session>,
    seat_name: String,
}

impl SessionRef {
    pub fn new(handle: Handle<Session>, seat_name: String) -> Self {
        Self { handle, seat_name }
    }

    pub fn seat_name(&self) -> &str {
        &self.seat_name
    }
}

impl std::ops::Deref for SessionRef {
    type Target = Handle<Session>;
    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl HasMailbox for Session {
    type Message = SessionMessage;

    fn channel_size() -> usize {
        16
    }
}

impl Actor for Session {
    type Init = Self;

    async fn init(init: Self, _ctx: &mut Context<Self>) -> Self {
        init
    }

    async fn handle_message(&mut self, msg: SessionMessage, _ctx: &mut Context<Self>) {
        match msg {
            SessionMessage::CurrentVt { respond_to } => match self.seat.current_session().await {
                Ok(vt) => {
                    let _ = respond_to.send(vt);
                }
                Err(e) => {
                    tracing::error!("Failed to get current VT: {e}");
                }
            },
            SessionMessage::SwitchVt { vt } => {
                if let Err(e) = self.seat.switch_session(vt).await {
                    tracing::error!("Failed to switch to VT {vt}: {e}");
                }
            }
            SessionMessage::OpenDevice { path, respond_to } => {
                match self.seat.open_device(path).await {
                    Ok(fd) => {
                        let _ = respond_to.send(fd);
                    }
                    Err(e) => {
                        tracing::error!("Failed to open device: {e}");
                    }
                }
            }
            SessionMessage::CloseDevice { fd } => {
                if let Err(e) = self.seat.close_device(fd).await {
                    tracing::error!("Failed to close device: {e}");
                }
            }
            SessionMessage::AcquireSession { respond_to } => {
                if let Err(e) = self.seat.aquire_session().await {
                    tracing::error!("Failed to acquire session: {e}");
                }
                let _ = respond_to.send(());
            }
            SessionMessage::ReleaseSession { respond_to } => {
                if let Err(e) = self.seat.release_session().await {
                    tracing::error!("Failed to release session: {e}");
                }
                let _ = respond_to.send(());
            }
        }
    }
}

impl StreamActor for Session {
    type Event = Result<bool, saddle::Error>;
    type Stream = Pin<Box<dyn Stream<Item = Self::Event> + Send>>;

    async fn create_stream(&mut self, _ctx: &mut Context<Self>) -> Self::Stream {
        Box::pin(self.seat.active_stream().await)
    }

    async fn handle_event(&mut self, event: Result<bool, saddle::Error>, _ctx: &mut Context<Self>) {
        match event {
            Ok(true) => {
                let _ = self
                    .compositor_handle
                    .cast(super::compositor::CompositorMessage::SessionResumed)
                    .await;
            }
            Ok(false) => {
                let _ = self
                    .compositor_handle
                    .cast(super::compositor::CompositorMessage::SessionLost)
                    .await;
            }
            Err(e) => {
                tracing::error!("Session stream error: {e}");
            }
        }
    }
}
