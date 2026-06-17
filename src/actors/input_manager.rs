use std::{
    ffi::CString,
    os::fd::{AsRawFd, FromRawFd, IntoRawFd, OwnedFd},
    sync::mpsc as std_mpsc,
};

use colpetto::{Libinput, event::AsRawEvent};
use stagecraft::{
    Bus, Context, HasMailbox, LocalActor, LocalStreamActor, Responder, SuspendRefused, Suspendable,
};
use tokio::sync::mpsc;
use tracing::{debug, error, trace};

use crate::actors::session::DeviceBroker;

/// A raw input event produced by a device managed by libinput.
#[derive(Debug, Clone)]
pub struct InputEvent {
    pub name: &'static str,
    pub event_type: EventType,
    pub device_name: String,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum EventType {
    Keyboard(KeyboardEvent),
    Unknown,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum KeyboardEvent {
    Key {
        key: u32,
        state: colpetto::event::KeyState,
        time: u64,
    },
}

impl From<&colpetto::Event> for EventType {
    fn from(value: &colpetto::Event) -> Self {
        match value {
            colpetto::Event::Keyboard(colpetto::event::KeyboardEvent::Key(event)) => {
                EventType::Keyboard(KeyboardEvent::Key {
                    key: event.key(),
                    state: event.key_state(),
                    time: event.time_usec(),
                })
            }
            _ => EventType::Unknown,
        }
    }
}

#[stagecraft::message(InputManager)]
pub enum InputManagerMessage {
    /// Suspend coordination: suspend libinput (closing device fds) and acknowledge.
    #[call(Result<(), SuspendRefused>)]
    PrepareSuspend,
    /// Suspend coordination: resume libinput.
    Resume,
}

impl Suspendable for InputManager {
    fn prepare_suspend(responder: Responder<Result<(), SuspendRefused>>) -> Self::Message {
        InputManagerMessage::PrepareSuspend {
            respond_to: responder,
        }
    }

    fn resume() -> Self::Message {
        InputManagerMessage::Resume
    }
}

pub struct InputManagerInit {
    pub device_broker: DeviceBroker,
    pub input_bus: Bus<InputEvent>,
}

pub struct InputManager {
    input_bus: Bus<InputEvent>,
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
        let (close_request_tx, mut close_request_rx) = mpsc::unbounded_channel::<OwnedFd>();
        let (close_response_tx, close_response_rx) = std_mpsc::channel();

        let open_devices = init.device_broker.clone();
        let close_devices = init.device_broker.clone();

        // IMPORTANT: these must be spawned on the original runtime to avoid deadlock.
        ctx.track_main(async move {
            while let Some(path) = open_request_rx.recv().await {
                trace!(
                    "Bridge: attempting to open device {}",
                    path.to_string_lossy()
                );
                let fd: i32 = match open_devices.open(path).await {
                    Ok(owned_fd) => owned_fd.into_raw_fd(),
                    Err(_) => {
                        error!("Failed to open device, actor dead");
                        -1
                    }
                };
                let _ = open_response_tx.send(fd);
            }
        });

        ctx.track_main(async move {
            while let Some(fd) = close_request_rx.recv().await {
                trace!("Bridge: Attempting to close device {}", fd.as_raw_fd());
                let _ = close_devices.close(fd).await;
                let _ = close_response_tx.send(());
            }
        });

        let mut libinput = Libinput::with_tracing(
            move |path, _| {
                trace!("Trying to open device");
                open_request_tx.send(path.to_owned()).map_err(|_| -1)?;
                open_response_rx.recv().map_err(|_| -1)
            },
            move |fd| {
                trace!("Trying to close device: {}", fd.as_raw_fd());
                let _ = close_request_tx.send(unsafe { OwnedFd::from_raw_fd(fd) });
                let _ = close_response_rx.recv();
                trace!("closed device: {}", fd.as_raw_fd());
            },
        )
        .unwrap();

        let seat_name = init
            .device_broker
            .seat_name()
            .await
            .expect("Session actor alive during InputManager init");

        libinput
            .udev_assign_seat(CString::new(seat_name).unwrap().as_c_str())
            .unwrap();

        Self {
            input_bus: init.input_bus,
            libinput,
        }
    }

    async fn handle_message(&mut self, msg: InputManagerMessage, _ctx: &mut Context<Self>) {
        match msg {
            InputManagerMessage::PrepareSuspend { respond_to } => {
                self.libinput.suspend();
                respond_to.respond(Ok(()));
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
                self.input_bus.publish(InputEvent {
                    name: event.event_type(),
                    event_type: event.into(),
                    device_name: event.device().name().to_string_lossy().to_string(),
                });
            }
            Err(e) => {
                tracing::error!("libinput error: {e}");
            }
        }
    }
}
