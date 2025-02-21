use std::{
    ffi::{CStr, c_int},
    os::fd::RawFd,
};

use colpetto::{Error, Libinput, event::AsRawEvent};
use rustix::{
    fd::{FromRawFd, IntoRawFd, OwnedFd},
    fs::{Mode, OFlags, open},
};
use tokio::{
    runtime,
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::LocalSet,
};
use tokio_stream::{StreamExt, wrappers::UnboundedReceiverStream};

#[derive(Debug)]
pub struct Event {
    event_type: &'static str,
    device_name: String,
}

pub struct ShutdownHandle(UnboundedSender<()>);

impl ShutdownHandle {
    pub fn shutdown(&self) {
        let _ = self.0.send(());
    }
}

fn open_restricted(path: &CStr, flags: c_int) -> Result<RawFd, c_int> {
    open(path, OFlags::from_bits_retain(flags as u32), Mode::empty())
        .map(IntoRawFd::into_raw_fd)
        .map_err(|err| err.raw_os_error().wrapping_neg())
}

fn close_restricted(fd: RawFd) {
    drop(unsafe { OwnedFd::from_raw_fd(fd) });
}

pub fn spawn_libinput_task() -> std::io::Result<(
    UnboundedReceiverStream<Result<Event, colpetto::Error>>,
    ShutdownHandle,
)> {
    let (event_sx, event_rx) = mpsc::unbounded_channel();
    let (shutdown_sx, mut shutdown_rx) = mpsc::unbounded_channel();

    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    std::thread::spawn(move || {
        let local = LocalSet::new();

        local.spawn_local(async move {
            let mut libinput = Libinput::new(open_restricted, close_restricted)?;

            libinput.udev_assign_seat(c"seat0")?;

            let mut stream = libinput.event_stream()?;

            loop {
                tokio::select! {
                    Some(()) = shutdown_rx.recv() => {
                        println!("shutting down libinput instance...");
                        break;
                    }
                    Some(res) = stream.next() => {
                        if event_sx
                            .send(res.map(|event| Event {
                                event_type: event.event_type(),
                                device_name: event.device().name().to_string_lossy().to_string(),
                            }))
                            .is_err()
                        {
                            break;
                        }
                    }
                    else => break,
                }
            }

            Ok::<_, Error>(())
        });

        rt.block_on(local);
    });

    Ok((
        UnboundedReceiverStream::new(event_rx),
        ShutdownHandle(shutdown_sx),
    ))
}
