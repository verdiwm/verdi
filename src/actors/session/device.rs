use std::{
    ffi::CString,
    os::fd::{AsRawFd, OwnedFd},
};

use saddle::Seat;
use stagecraft::{Actor, Context, HasMailbox};

/// Brokers access to the seat's devices. Kept separate from
/// [`SessionController`](super::SessionController) so its mailbox stays free to
/// service `CloseDevice` calls *while* the controller is orchestrating a suspend
/// (which synchronously closes input fds). See the comment on `SessionController`.
#[stagecraft::message(DeviceAuthority)]
pub enum DeviceAuthorityMessage {
    #[call(String)]
    SeatName,
    #[call(OwnedFd)]
    OpenDevice {
        path: CString,
    },
    #[call]
    CloseDevice {
        fd: OwnedFd,
    },
}

pub struct DeviceAuthorityInit {
    pub seat: Seat,
}

pub struct DeviceAuthority {
    seat: Seat,
}

impl HasMailbox for DeviceAuthority {
    type Message = DeviceAuthorityMessage;

    fn channel_size() -> usize {
        16
    }
}

impl Actor for DeviceAuthority {
    type Init = DeviceAuthorityInit;

    async fn init(init: DeviceAuthorityInit, _ctx: &mut Context<Self>) -> Self {
        Self { seat: init.seat }
    }

    async fn handle_message(&mut self, msg: DeviceAuthorityMessage, _ctx: &mut Context<Self>) {
        match msg {
            DeviceAuthorityMessage::SeatName { respond_to } => {
                respond_to.respond(self.seat.seat_name().to_owned());
            }
            DeviceAuthorityMessage::OpenDevice { path, respond_to } => {
                match self.seat.open_device(path).await {
                    Ok((fd, _)) => respond_to.respond(fd),
                    Err(e) => tracing::error!("Failed to open device: {e}"),
                }
            }
            DeviceAuthorityMessage::CloseDevice { fd, respond_to } => {
                if let Err(e) = self.seat.close_device(&fd).await {
                    tracing::error!("Failed to close device {}: {e}", fd.as_raw_fd());
                }
                respond_to.respond(());
            }
        }
    }
}
