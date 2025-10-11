use std::collections::HashMap;

use tokio::{net::UnixStream, sync::mpsc};
use tokio_util::sync::CancellationToken;

use crate::actors::client::ClientHandle;

#[derive(Debug)]
pub enum CompositorMessage {
    NewClient { stream: UnixStream },
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
    shutdown_token: CancellationToken,
}

impl Compositor {
    pub fn new(shutdown_token: CancellationToken) -> Self {
        let (sender, receiver) = mpsc::channel(128);

        Self {
            sender,
            receiver,
            next_client_id: 1,
            clients: HashMap::new(),
            shutdown_token,
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
                    // Handle shutdown here
                    break
                }
                Some(msg) = self.receiver.recv() => {
                    self.handle_message(msg);
                }
            }
        }
    }

    fn handle_message(&mut self, msg: CompositorMessage) {
        match msg {
            CompositorMessage::NewClient { stream } => {
                let client_id = self.next_client_id();

                // FIXME: how the hell do we handle errors
                let (client_handle, client) = ClientHandle::new(stream, client_id).unwrap();

                self.clients.insert(client_id, client_handle);

                tokio::spawn(client.run());
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
}
