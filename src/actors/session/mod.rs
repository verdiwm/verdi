//! Session management, split across two cooperating actors.
//!
//! [`DeviceAuthority`] brokers device fds (open/close/seat name); its mailbox must
//! stay responsive. [`SessionController`] owns the slow, sequential session
//! lifecycle (acquire / release / suspend / VT switch) and the logind active
//! stream. They are deliberately separate actors — see the comment on
//! [`SessionController`] for why merging them deadlocks.
//!
//! Consumers depend on the capability wrappers [`DeviceBroker`] and
//! [`SessionControl`], not on the concrete actors.

mod controller;
mod device;

use std::{ffi::CString, os::fd::OwnedFd, time::Duration};

use stagecraft::{ActorDead, Handle};

pub use controller::{
    SessionController, SessionControllerExt, SessionControllerInit, SessionControllerMessage,
};
pub use device::{DeviceAuthority, DeviceAuthorityExt, DeviceAuthorityInit, DeviceAuthorityMessage};

/// How long the [`SessionController`] waits for each hardware actor to quiesce
/// before giving up on it and proceeding anyway. In the normal flow a participant
/// acks within milliseconds; the timeout only guards against an actor that can no
/// longer make progress, so the session machinery never wedges.
pub const SUSPEND_TIMEOUT: Duration = Duration::from_secs(5);

/// Current session lifecycle state, published on a [`Watch`](stagecraft::Watch) for
/// any actor that needs to know whether the session is live (e.g. the OutputManager
/// rescans on the edge into [`Active`](SessionState::Active)).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionState {
    /// The session owns its devices and may render.
    Active,
    /// A suspend is in progress; participants are releasing their resources.
    Suspending,
    /// The session has been released to the kernel.
    Inactive,
    /// A resume is in progress; the session is being re-acquired.
    Resuming,
}

/// Capability handing out access to the session's device authority (open/close
/// DRM and input device fds). A thin wrapper over the [`DeviceAuthority`] handle so
/// consumers depend on *what* they need, not on the concrete actor.
#[derive(Clone)]
pub struct DeviceBroker {
    device: Handle<DeviceAuthority>,
}

impl DeviceBroker {
    pub fn new(device: Handle<DeviceAuthority>) -> Self {
        Self { device }
    }

    /// Open a device by path, returning an owned fd.
    pub async fn open(&self, path: CString) -> Result<OwnedFd, ActorDead<()>> {
        self.device.open_device(path).await
    }

    /// Release a device fd back to logind.
    pub async fn close(&self, fd: OwnedFd) -> Result<(), ActorDead<()>> {
        self.device.close_device(fd).await
    }

    /// The seat name (used to assign libinput's udev seat).
    pub async fn seat_name(&self) -> Result<String, ActorDead<()>> {
        self.device.seat_name().await
    }
}

/// Capability for session/VT control (querying and switching the active VT). A thin
/// wrapper over the [`SessionController`] handle.
#[derive(Clone)]
pub struct SessionControl {
    controller: Handle<SessionController>,
}

impl SessionControl {
    pub fn new(controller: Handle<SessionController>) -> Self {
        Self { controller }
    }

    /// The VT number the session is currently on.
    pub async fn current_vt(&self) -> Result<u32, ActorDead<()>> {
        self.controller.current_vt().await
    }

    /// Request a switch to another VT.
    pub async fn switch_vt(&self, vt: u32) -> Result<(), ActorDead<()>> {
        self.controller.switch_vt(vt).await
    }
}
