use std::{collections::HashMap, path::PathBuf, sync::Arc};

use colpetto::event::KeyState;
use input_linux_sys::KEY_ESC;
use saddle::Seat;
use stagecraft::{self, Actor, Bus, BusConfig, Context, HasMailbox, SuspendRegistry, Watch};
use tokio::{net::UnixStream, sync::RwLock};
use tracing::{debug, info};
use waynest_server::Listener;

use crate::{
    Client,
    actors::{
        client::ClientHandle,
        client_listener::{ClientListener, ClientListenerInit},
        input_manager::{
            EventType, InputEvent, InputManager, InputManagerInit, KeyboardEvent,
        },
        output_manager::{OutputEvent, OutputManager, OutputManagerInit},
        renderer::{Renderer, RendererInit},
        session::{
            self, DeviceAuthority, DeviceAuthorityInit, DeviceBroker, SessionControl,
            SessionController, SessionControllerExt, SessionControllerInit, SessionState,
        },
    },
    keymap::{KeyMap, ModifierState},
};

#[derive(Debug)]
#[stagecraft::message(Compositor)]
pub enum CompositorMessage {
    NewClient { stream: UnixStream },
    /// A client task has finished; drop its handle from the registry.
    ClientGone { client_id: u32 },
    /// Forwarded from the input bus.
    Input(InputEvent),
}

pub struct CompositorInit {
    pub socket_path: Option<PathBuf>,
}

pub struct Compositor {
    next_client_id: u32,
    clients: HashMap<u32, ClientHandle>,
    key_map: KeyMap,
    modifier_state: Arc<RwLock<ModifierState>>,
    session_control: SessionControl,
}

impl Compositor {
    fn next_client_id(&mut self) -> u32 {
        let prev = self.next_client_id;
        self.next_client_id = self.next_client_id.wrapping_add(1);
        prev
    }
}

impl HasMailbox for Compositor {
    type Message = CompositorMessage;

    fn channel_size() -> usize {
        256
    }
}

impl Actor for Compositor {
    type Init = CompositorInit;

    async fn init(init: CompositorInit, ctx: &mut Context<Self>) -> Self {
        debug!("Started compositor");

        // Create buses with explicit capacity.
        let input_bus: Bus<InputEvent> = Bus::new(BusConfig { capacity: 256 });
        let output_bus: Bus<OutputEvent> = Bus::new(BusConfig { capacity: 16 });

        // Retained session state. Session owns the publish side; the OutputManager
        // reads it to rescan outputs whenever the session becomes active.
        let session_state = Watch::new(SessionState::Inactive);
        let state_reader = session_state.reader();

        let seat = Seat::new().await.expect("Failed to open seat");

        // Two cooperating actors back the session: DeviceAuthority brokers device
        // fds (its mailbox must stay free for CloseDevice during suspend), while
        // SessionController drives the lifecycle. Both hold a clone of the seat.
        let device_handle = ctx.spawn::<DeviceAuthority>(DeviceAuthorityInit {
            seat: seat.clone(),
        });
        let controller_handle = ctx.spawn_stream::<SessionController>(SessionControllerInit {
            seat,
            state: session_state,
        });

        // Capabilities over the actor handles: hardware actors depend on what
        // they need (device access / VT control), not the concrete actors.
        let device_broker = DeviceBroker::new(device_handle);
        let session_control = SessionControl::new(controller_handle.clone());

        let input_manager_handle = ctx.spawn_stream_local::<InputManager>(InputManagerInit {
            device_broker: device_broker.clone(),
            input_bus: input_bus.clone(),
        });

        let output_manager_handle = ctx.spawn::<OutputManager>(OutputManagerInit {
            device_broker: device_broker.clone(),
            state: state_reader,
            output_bus: output_bus.clone(),
        });

        let renderer_handle = ctx.spawn::<Renderer>(RendererInit {
            device_broker: device_broker.clone(),
            output_bus: output_bus.clone(),
        });

        // Assemble the suspend/resume barrier and hand it to the SessionController.
        // Registration order is prepare order: the Renderer releases DRM master
        // first, then the (resourceless) OutputManager, then InputManager closes
        // its device fds.
        let mut suspend_registry = SuspendRegistry::new(session::SUSPEND_TIMEOUT);
        suspend_registry.register("renderer", renderer_handle);
        suspend_registry.register("output", output_manager_handle);
        suspend_registry.register("input", input_manager_handle);
        let _ = controller_handle.set_suspend_registry(suspend_registry).await;

        let listener = if let Some(ref path) = init.socket_path {
            Listener::new_with_path(path).expect("Failed to start client listener")
        } else {
            Listener::new().expect("Failed to start client listener")
        };

        let _listener_handle = ctx.spawn_stream::<ClientListener>(ClientListenerInit {
            listener,
            compositor_handle: ctx.handle(),
        });

        // Bridge: input_bus → CompositorMessage::Input
        ctx.forward_bus(&input_bus, |event| Some(CompositorMessage::Input(event)));

        Self {
            next_client_id: 1,
            clients: HashMap::new(),
            key_map: KeyMap::new(),
            modifier_state: Arc::new(RwLock::new(ModifierState::new())),
            session_control,
        }
    }

    async fn on_stop(&mut self, _ctx: &mut Context<Self>) {
        debug!("Compositor stopped");
    }

    async fn handle_message(&mut self, msg: CompositorMessage, ctx: &mut Context<Self>) {
        match msg {
            CompositorMessage::NewClient { stream } => {
                let client_id = self.next_client_id();

                let token = ctx.child_token();
                match Client::new(stream, client_id, token) {
                    Ok(client) => {
                        self.clients.insert(client_id, client.handle());
                        let compositor = ctx.handle();
                        ctx.track(async move {
                            client.run().await;
                            let _ = compositor
                                .cast(CompositorMessage::ClientGone { client_id })
                                .await;
                        });
                    }
                    Err(e) => {
                        tracing::error!("Failed to create client {client_id}: {e}");
                    }
                }
            }
            CompositorMessage::ClientGone { client_id } => {
                self.clients.remove(&client_id);
                debug!("Client {client_id} removed; {} active", self.clients.len());
            }
            #[allow(clippy::single_match)]
            CompositorMessage::Input(event) => match event.event_type {
                EventType::Keyboard(KeyboardEvent::Key { key, state, .. }) => {
                    let (should_check_vt_switch, is_ctrl_alt_pressed) = {
                        let mut ms = self.modifier_state.write().await;
                        ms.update(key, state);
                        (state == KeyState::Pressed, ms.is_ctrl_alt_pressed())
                    };

                    if should_check_vt_switch {
                        if key as i32 == KEY_ESC {
                            ctx.shutdown();
                        }

                        if is_ctrl_alt_pressed && let Some(vt) = self.key_map.get_vt(key) {
                            info!("Ctrl+Alt+F{vt} pressed, attempting a VT switch to {vt}");

                            if let Ok(current_vt) = self.session_control.current_vt().await
                                && vt != current_vt
                            {
                                info!(
                                    "Deactivating session - destroying rendering context completely"
                                );
                                let _ = self.session_control.switch_vt(vt).await;
                            }
                        }
                    }
                }
                _ => {}
            },
        }
    }
}
