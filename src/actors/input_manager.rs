use std::{
    ffi::CString,
    os::fd::{FromRawFd, IntoRawFd, OwnedFd},
};

use colpetto::{Libinput, event::AsRawEvent};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;

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
    libinput: Libinput,
    compositor_handle: CompositorHandle,
    shutdown_token: CancellationToken,
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

        let open_handle = session_handle.clone();
        let close_handle = session_handle.clone();

        let mut libinput = Libinput::with_tracing(
            move |path, _| {
                open_handle
                    .open_device_blocking(path)
                    .map_err(|_| -1)
                    .map(IntoRawFd::into_raw_fd)
            },
            move |fd| close_handle.close_device_blocking(unsafe { OwnedFd::from_raw_fd(fd) }),
        )
        .unwrap();

        libinput
            .udev_assign_seat(CString::new(session_handle.seat_name()).unwrap().as_c_str())
            .unwrap();

        Self {
            sender,
            receiver,
            libinput,
            compositor_handle,
            shutdown_token,
        }
    }

    pub fn handle(&self) -> InputManagerHandle {
        InputManagerHandle::new(self.sender.clone())
    }

    pub async fn run(mut self) {
        let mut stream = self.libinput.event_stream().unwrap();

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
                    self.handle_message(msg).await;
                }
            }
        }
    }

    async fn handle_message(&mut self, msg: InputManagerMessage) {
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
