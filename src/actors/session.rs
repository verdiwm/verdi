use std::{ffi::CString, os::fd::OwnedFd};

use saddle::Seat;
use tokio::{
    pin,
    sync::{
        mpsc,
        oneshot::{self, error::RecvError},
    },
};
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;

use crate::CompositorHandle;

pub enum SessionMessage {
    CurrentVt {
        respond_to: oneshot::Sender<u32>,
    },
    SwitchVt {
        vt: u32,
    },
    OpenDevice {
        path: CString,
        respond_to: oneshot::Sender<OwnedFd>,
    },
    CloseDevice {
        fd: OwnedFd,
    },
}

pub struct Session {
    sender: mpsc::Sender<SessionMessage>,
    receiver: mpsc::Receiver<SessionMessage>,
    seat: Seat,
    compositor_handle: CompositorHandle,
    shutdown_token: CancellationToken,
}

#[derive(Clone)]
pub struct SessionHandle {
    sender: mpsc::Sender<SessionMessage>,
    seat_name: String,
}

impl Session {
    pub async fn new(
        compositor_handle: CompositorHandle,
        shutdown_token: CancellationToken,
    ) -> Self {
        let seat = Seat::new().await.expect("Failed to open seat");

        let (sender, receiver) = mpsc::channel(16);

        Self {
            sender,
            receiver,
            seat,
            compositor_handle,
            shutdown_token,
        }
    }

    pub fn handle(&self) -> SessionHandle {
        SessionHandle::new(self.sender.clone(), self.seat_name().to_owned())
    }

    pub fn seat_name(&self) -> &str {
        &self.seat.seat_name()
    }

    // FIXME: we basically aren't handling errors
    pub async fn run(mut self) {
        let stream = self.seat.active_stream().await;

        pin!(stream);

        loop {
            tokio::select! {
                biased;
                _ = self.shutdown_token.cancelled() => {
                    break
                }
                Some(res) = stream.next() => {
                    match res {
                        Ok(is_active) => {
                            if is_active {
                                let _ = self.seat.aquire_session().await;
                                self.compositor_handle.session_resumed().await;
                            } else {
                                let _ = self.seat.release_session().await;
                                self.compositor_handle.session_lost().await;
                            }
                        },
                        Err(_) => break,
                    }
                }
                Some(msg) = self.receiver.recv() => {
                    self.handle_message(msg).await;
                }
            }
        }
    }

    async fn handle_message(&mut self, msg: SessionMessage) {
        match msg {
            SessionMessage::CurrentVt { respond_to } => {
                let current_vt = self.seat.current_session().await.unwrap();
                let _ = respond_to.send(current_vt);
            }
            SessionMessage::SwitchVt { vt } => {
                let _ = self.seat.switch_session(vt).await;
            }
            SessionMessage::OpenDevice { path, respond_to } => {
                let fd = self.seat.open_device(path).await.unwrap();

                let _ = respond_to.send(fd);
            }
            SessionMessage::CloseDevice { fd } => {
                let _ = self.seat.close_device(fd).await;
            }
        }
    }
}

impl SessionHandle {
    fn new(sender: mpsc::Sender<SessionMessage>, seat_name: String) -> Self {
        Self { sender, seat_name }
    }

    pub fn seat_name(&self) -> &str {
        &self.seat_name
    }

    pub async fn switch_vt(&self, vt: u32) {
        let _ = self.sender.send(SessionMessage::SwitchVt { vt }).await;
    }

    pub async fn open_device(&self, path: CString) -> Result<OwnedFd, RecvError> {
        let (send, recv) = oneshot::channel();

        let _ = self
            .sender
            .send(SessionMessage::OpenDevice {
                path,
                respond_to: send,
            })
            .await;

        recv.await
    }

    pub async fn close_device(&self, fd: OwnedFd) {
        let _ = self.sender.send(SessionMessage::CloseDevice { fd }).await;
    }

    pub async fn current_vt(&self) -> Result<u32, RecvError> {
        let (send, recv) = oneshot::channel();

        let _ = self
            .sender
            .send(SessionMessage::CurrentVt { respond_to: send })
            .await;

        recv.await
    }
}
