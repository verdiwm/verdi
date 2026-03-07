use std::{collections::HashMap, path::PathBuf, sync::Arc};

use colpetto::event::KeyState;
use input_linux_sys::KEY_ESC;
use saddle::Seat;
use stagecraft::{Actor, ActorDead, Context, Handle, HasMailbox};
use tokio::{net::UnixStream, sync::RwLock};
use tracing::{debug, info};
use waynest_server::Listener;

use crate::{
    Client,
    actors::{
        client::ClientHandle,
        client_listener::{ClientListener, ClientListenerInit},
        input_manager::{InputManager, InputManagerExt, InputManagerInit},
        renderer::{Renderer, RendererExt},
        session::{Session, SessionExt, SessionRef},
    },
    keymap::{KeyMap, ModifierState},
};

#[derive(Debug)]
pub enum CompositorMessage {
    NewClient { stream: UnixStream },
    Input(InputEvent),
    SessionLost,
    SessionResumed,
}

#[derive(Debug)]
pub struct InputEvent {
    pub name: &'static str,
    pub event_type: EventType,
    pub device_name: String,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum EventType {
    Keyboard(KeyboardEvent),
    Unknown,
}

#[derive(Debug)]
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

pub struct CompositorInit {
    pub socket_path: Option<PathBuf>,
}

pub struct Compositor {
    next_client_id: u32,
    clients: HashMap<u32, ClientHandle>,
    key_map: KeyMap,
    modifier_state: Arc<RwLock<ModifierState>>,
    has_control: bool,
    session_ref: SessionRef,
    input_manager_handle: Handle<InputManager>,
    renderer_handle: Handle<Renderer>,
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

        let seat = Seat::new().await.expect("Failed to open seat");
        let seat_name = seat.seat_name().to_owned();

        let session = Session {
            seat,
            compositor_handle: ctx.handle(),
        };

        let session_handle = ctx.spawn_stream::<Session>(session);
        let session_ref = SessionRef::new(session_handle, seat_name);

        let input_manager_handle = ctx.spawn_stream_local::<InputManager>(InputManagerInit {
            compositor_handle: ctx.handle(),
            session_ref: session_ref.clone(),
        });

        let renderer_handle = ctx.spawn::<Renderer>(Renderer::new(session_ref.clone()));

        let listener = if let Some(ref path) = init.socket_path {
            Listener::new_with_path(path).expect("Failed to start client listener")
        } else {
            Listener::new().expect("Failed to start client listener")
        };

        let _listener_handle = ctx.spawn_stream::<ClientListener>(ClientListenerInit {
            listener,
            compositor_handle: ctx.handle(),
        });

        Self {
            next_client_id: 1,
            clients: HashMap::new(),
            key_map: KeyMap::new(),
            modifier_state: Arc::new(RwLock::new(ModifierState::new())),
            has_control: false,
            session_ref,
            input_manager_handle,
            renderer_handle,
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
                        ctx.track(client.run());
                    }
                    Err(e) => {
                        tracing::error!("Failed to create client {client_id}: {e}");
                    }
                }
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
                            if self.has_control {
                                info!("Ctrl+Alt+F{vt} pressed, attempting a VT switch to {vt}");

                                if let Ok(current_vt) = self.session_ref.current_vt().await
                                    && vt != current_vt
                                {
                                    info!(
                                        "Deactivating session - destroying rendering context completely"
                                    );
                                    let _ = self.session_ref.switch_vt(vt).await;
                                }
                            } else {
                                debug!("Not switching VT - session inactive");
                            }
                        }
                    }
                }
                _ => {}
            },
            CompositorMessage::SessionLost => {
                let _ = self.input_manager_handle.suspend().await;
                let _ = self.renderer_handle.suspend().await;
                let _ = self.session_ref.release_session().await;
                self.has_control = false;
            }
            CompositorMessage::SessionResumed => {
                let _ = self.session_ref.acquire_session().await;
                self.has_control = true;
                let _ = self.input_manager_handle.resume().await;
                let _ = self.renderer_handle.resume().await;
            }
        }
    }
}

pub trait CompositorExt {
    fn new_client(&self, stream: UnixStream) -> impl Future<Output = Result<(), ActorDead<()>>>;
    fn session_lost(&self) -> impl Future<Output = Result<(), ActorDead<()>>>;
    fn session_resumed(&self) -> impl Future<Output = Result<(), ActorDead<()>>>;
    fn input(&self, event: InputEvent) -> impl Future<Output = Result<(), ActorDead<()>>>;
}

impl CompositorExt for Handle<Compositor> {
    async fn new_client(&self, stream: UnixStream) -> Result<(), ActorDead<()>> {
        self.cast(CompositorMessage::NewClient { stream })
            .await
            .map_err(|_| ActorDead(()))
    }

    async fn session_lost(&self) -> Result<(), ActorDead<()>> {
        self.cast(CompositorMessage::SessionLost)
            .await
            .map_err(|_| ActorDead(()))
    }

    async fn session_resumed(&self) -> Result<(), ActorDead<()>> {
        self.cast(CompositorMessage::SessionResumed)
            .await
            .map_err(|_| ActorDead(()))
    }

    async fn input(&self, event: InputEvent) -> Result<(), ActorDead<()>> {
        self.cast(CompositorMessage::Input(event))
            .await
            .map_err(|_| ActorDead(()))
    }
}
