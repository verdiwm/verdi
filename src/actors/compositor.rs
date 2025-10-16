use std::{collections::HashMap, path::Path, sync::Arc};

use colpetto::event::KeyState;
use input_linux_sys::KEY_ESC;
use tokio::{
    net::UnixStream,
    sync::{RwLock, mpsc},
};
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tracing::{debug, info};

use crate::{
    actors::{
        client::ClientHandle,
        client_listener::ClientListener,
        input_manager::{InputManager, InputManagerHandle},
        session::{Session, SessionHandle},
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

#[derive(Clone)]
pub struct CompositorHandle {
    sender: mpsc::Sender<CompositorMessage>,
}

pub struct Compositor {
    sender: mpsc::Sender<CompositorMessage>,
    receiver: mpsc::Receiver<CompositorMessage>,
    next_client_id: u32,
    clients: HashMap<u32, ClientHandle>,
    actors_tracker: TaskTracker,
    shutdown_token: CancellationToken,

    key_map: KeyMap,
    modifier_state: Arc<RwLock<ModifierState>>,
    has_control: bool,

    // Handles
    session_handle: SessionHandle,
    input_manager_handle: InputManagerHandle,
}

impl Compositor {
    pub async fn new<P: AsRef<Path>>(socket_path: Option<P>) -> Self {
        let shutdown_token = CancellationToken::new();

        let (sender, receiver) = mpsc::channel(256);

        let actors_tracker = TaskTracker::new();

        let compositor_handle = CompositorHandle::new(sender.clone());

        let client_listener = ClientListener::new(
            compositor_handle.clone(),
            shutdown_token.child_token(),
            socket_path,
        )
        .expect("Failed to start client listener");

        let session = Session::new(compositor_handle.clone(), shutdown_token.child_token()).await;
        let session_handle = session.handle();

        let input_manager = InputManager::new(
            compositor_handle,
            session_handle.clone(),
            shutdown_token.child_token(),
        );
        let input_manager_handle = input_manager.handle();

        actors_tracker.spawn(session.run());
        actors_tracker.spawn(input_manager.run());
        actors_tracker.spawn(client_listener.run());

        let key_map = KeyMap::new();
        let modifier_state = Arc::new(RwLock::new(ModifierState::new()));

        Self {
            sender,
            receiver,
            next_client_id: 1,
            clients: HashMap::new(),
            actors_tracker,
            shutdown_token,
            key_map,
            modifier_state,
            has_control: false,
            session_handle,
            input_manager_handle,
        }
    }

    pub fn handle(&self) -> CompositorHandle {
        CompositorHandle::new(self.sender.clone())
    }

    fn next_client_id(&mut self) -> u32 {
        let prev = self.next_client_id;
        self.next_client_id = self.next_client_id.wrapping_add(1);

        prev
    }

    pub async fn run(mut self) {
        tracing::debug!("Started compositor");

        loop {
            tokio::select! {
                biased;
                _ = self.shutdown_token.cancelled() => {
                    self.actors_tracker.close();
                    break
                }
                Some(msg) = self.receiver.recv() => {
                    self.handle_message(msg).await;
                }
            }
        }

        self.actors_tracker.wait().await;
    }

    async fn handle_message(&mut self, msg: CompositorMessage) {
        match msg {
            CompositorMessage::NewClient { stream } => {
                let client_id = self.next_client_id();

                // FIXME: how the hell do we handle errors
                let (client_handle, client) = ClientHandle::new(stream, client_id).unwrap();

                self.clients.insert(client_id, client_handle);

                tokio::spawn(client.run());
            }
            CompositorMessage::Input(event) => match event.event_type {
                EventType::Keyboard(KeyboardEvent::Key { key, state, .. }) => {
                    let (should_check_vt_switch, is_ctrl_alt_pressed) = {
                        let mut ms = self.modifier_state.write().await;
                        ms.update(key, state);
                        (state == KeyState::Pressed, ms.is_ctrl_alt_pressed())
                    };

                    if should_check_vt_switch {
                        // Handle ESC for exit
                        if key as i32 == KEY_ESC {
                            // libinput_handle.shutdown();
                            self.shutdown_token.cancel();
                        }

                        // Only process function keys when Ctrl+Alt are held
                        if is_ctrl_alt_pressed && let Some(vt) = self.key_map.get_vt(key) {
                            if self.has_control {
                                info!("Ctrl+Alt+F{vt} pressed, attempting a VT switch to {vt}");

                                if let Ok(current_vt) = self.session_handle.current_vt().await
                                    && vt != current_vt
                                {
                                    info!(
                                        "Deactivating session - destroying rendering context completely"
                                    );

                                    // wgpu_context = None;

                                    self.session_handle.switch_vt(vt).await;

                                    // if let Err(e) = self.session_handle.switch_vt(vt).await {
                                    //     error!("Failed to switch to VT {vt}: {e}");
                                    // }
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
                self.has_control = false;
                self.input_manager_handle.suspend().await;
            }
            CompositorMessage::SessionResumed => {
                self.has_control = true;
                self.input_manager_handle.resume().await;
            }
        }
    }
}

impl CompositorHandle {
    fn new(sender: mpsc::Sender<CompositorMessage>) -> Self {
        Self { sender }
    }

    pub async fn new_client(&self, stream: UnixStream) {
        let _ = self
            .sender
            .send(CompositorMessage::NewClient { stream })
            .await;
    }

    pub async fn session_lost(&self) {
        let _ = self.sender.send(CompositorMessage::SessionLost).await;
    }

    pub async fn session_resumed(&self) {
        let _ = self.sender.send(CompositorMessage::SessionResumed).await;
    }

    pub async fn input(&self, input_event: InputEvent) {
        let _ = self
            .sender
            .send(CompositorMessage::Input(input_event))
            .await;
    }
}
