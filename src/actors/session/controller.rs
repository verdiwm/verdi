use std::pin::Pin;

use futures_core::Stream;
use saddle::Seat;
use stagecraft::{Actor, Context, HasMailbox, StreamActor, SuspendRegistry, Watch};

use super::{SUSPEND_TIMEOUT, SessionState};

/// Owns the session lifecycle: it watches logind's active stream and drives the
/// acquire / release / suspend / VT-switch transitions, in order, on its own
/// message loop.
///
/// These transitions run **inline** here and hold this actor's mailbox for their
/// duration. That is only safe because they never need to call back into *this*
/// actor: suspending calls `InputManager::prepare_suspend`, whose handler runs
/// `libinput.suspend()` synchronously, which closes input fds and routes
/// `CloseDevice` calls into [`DeviceAuthority`](super::DeviceAuthority)'s *separate*
/// mailbox — which stays free. If device brokering and lifecycle orchestration
/// shared one mailbox, the orchestration would block awaiting InputManager while
/// InputManager waited on a `CloseDevice` this actor could never service →
/// deadlock. **Do not merge this actor back together with `DeviceAuthority`.**
///
/// Running on a single message loop also gives sequential ordering for free: a
/// release fully completes before the next acquire, and a switch's teardown
/// completes before the switch.
#[stagecraft::message(SessionController)]
pub enum SessionControllerMessage {
    #[call(u32)]
    CurrentVt,
    SwitchVt {
        vt: u32,
    },
    /// Hand the controller the set of actors to coordinate suspend/resume with.
    /// Sent once by the compositor after every hardware actor is spawned.
    SetSuspendRegistry {
        registry: SuspendRegistry,
    },
}

pub struct SessionControllerInit {
    pub seat: Seat,
    pub state: Watch<SessionState>,
}

pub struct SessionController {
    seat: Seat,
    state: Watch<SessionState>,
    /// Whether the session currently owns its devices. Starts `false`, so the
    /// first emission from `active_stream` drives the initial acquire.
    active: bool,
    /// Empty until the compositor sends the registry; an empty registry makes
    /// prepare/resume no-ops, which is exactly right for the initial acquire
    /// (nothing is holding resources yet).
    registry: SuspendRegistry,
}

impl SessionController {
    /// Publish `Suspending`, ask every participant to quiesce, then release the
    /// session to the kernel and publish `Inactive`. Bounded per-participant by the
    /// registry's timeout, so a stuck actor can't wedge the controller.
    async fn suspend(&mut self) {
        self.state.set(SessionState::Suspending);

        for (name, reason) in self.registry.prepare_all().await {
            tracing::warn!(participant = name, %reason, "suspend participant did not quiesce; proceeding");
        }

        if let Err(e) = self.seat.release_session().await {
            tracing::error!("Failed to release session: {e}");
        }

        self.state.set(SessionState::Inactive);
        self.active = false;
    }
}

impl HasMailbox for SessionController {
    type Message = SessionControllerMessage;

    fn channel_size() -> usize {
        16
    }
}

impl Actor for SessionController {
    type Init = SessionControllerInit;

    async fn init(init: SessionControllerInit, _ctx: &mut Context<Self>) -> Self {
        Self {
            seat: init.seat,
            state: init.state,
            active: false,
            registry: SuspendRegistry::new(SUSPEND_TIMEOUT),
        }
    }

    async fn handle_message(&mut self, msg: SessionControllerMessage, _ctx: &mut Context<Self>) {
        match msg {
            SessionControllerMessage::CurrentVt { respond_to } => {
                match self.seat.current_session().await {
                    Ok(vt) => respond_to.respond(vt),
                    Err(e) => tracing::error!("Failed to get current VT: {e}"),
                }
            }
            SessionControllerMessage::SwitchVt { vt } => {
                // Tear down hardware while the session is still active — the
                // Renderer can drop its swapchain cleanly because DRM master is
                // still held — and only then switch.
                if self.active {
                    self.suspend().await;
                }
                if let Err(e) = self.seat.switch_session(vt).await {
                    tracing::error!("Failed to switch to VT {vt}: {e}");
                }
            }
            SessionControllerMessage::SetSuspendRegistry { registry } => {
                self.registry = registry;
            }
        }
    }
}

impl StreamActor for SessionController {
    type Event = Result<bool, saddle::Error>;
    type Stream = Pin<Box<dyn Stream<Item = Self::Event> + Send>>;

    async fn create_stream(&mut self, _ctx: &mut Context<Self>) -> Self::Stream {
        Box::pin(self.seat.active_stream().await)
    }

    async fn handle_event(&mut self, event: Result<bool, saddle::Error>, _ctx: &mut Context<Self>) {
        match event {
            Ok(true) => {
                if !self.active {
                    self.state.set(SessionState::Resuming);
                    match self.seat.aquire_session().await {
                        Ok(()) => {
                            self.active = true;
                            self.registry.resume_all().await;
                            self.state.set(SessionState::Active);
                        }
                        Err(e) => {
                            tracing::error!("Failed to acquire session: {e}");
                            self.state.set(SessionState::Inactive);
                        }
                    }
                }
            }
            Ok(false) => {
                // Fallback for switches initiated outside verdi. For
                // verdi-initiated switches `SwitchVt` already tore down, so this
                // dedups to a no-op.
                if self.active {
                    self.suspend().await;
                }
            }
            Err(e) => tracing::error!("Session stream error: {e}"),
        }
    }
}
