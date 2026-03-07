use tokio::net::UnixStream;
use waynest_server::Listener;

use stagecraft::{Actor, Context, Handle, HasMailbox, StreamActor};

use super::compositor::{Compositor, CompositorMessage};

pub enum ClientListenerMessage {}

pub struct ClientListenerInit {
    pub listener: Listener,
    pub compositor_handle: Handle<Compositor>,
}

pub struct ClientListener {
    listener: Option<Listener>,
    compositor_handle: Handle<Compositor>,
}

impl HasMailbox for ClientListener {
    type Message = ClientListenerMessage;
}

impl Actor for ClientListener {
    type Init = ClientListenerInit;

    async fn init(init: ClientListenerInit, _ctx: &mut Context<Self>) -> Self {
        Self {
            listener: Some(init.listener),
            compositor_handle: init.compositor_handle,
        }
    }

    async fn handle_message(&mut self, msg: ClientListenerMessage, _ctx: &mut Context<Self>) {
        match msg {}
    }
}

impl StreamActor for ClientListener {
    type Event = Result<UnixStream, std::io::Error>;
    type Stream = Listener;

    async fn create_stream(&mut self, _ctx: &mut Context<Self>) -> Self::Stream {
        self.listener.take().expect("create_stream called twice")
    }

    async fn handle_event(
        &mut self,
        event: Result<UnixStream, std::io::Error>,
        _ctx: &mut Context<Self>,
    ) {
        match event {
            Ok(stream) => {
                let _ = self
                    .compositor_handle
                    .cast(CompositorMessage::NewClient { stream })
                    .await;
            }
            Err(e) => {
                tracing::error!("Failed to accept client connection: {e}");
            }
        }
    }
}
