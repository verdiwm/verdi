use futures_sink::Sink;
use pin_project_lite::pin_project;
use tokio::{net::UnixStream, sync::mpsc};
use tokio_stream::{Stream, StreamExt};
use tracing::error;

use waynest::{Message, ObjectId, ProtocolError, Socket};
use waynest_server::{Client as _, Store};

use crate::{VerdiError, protocol::wayland::display::Display};

pub enum ClientMessage {}

#[derive(Clone)]
#[allow(unused)]
pub struct ClientHandle {
    sender: mpsc::Sender<ClientMessage>,
    client_id: u32,
}

pin_project! {
    pub struct Client {
        #[pin]
        socket: Socket,
        store: Store<Client, VerdiError>,
        next_object_id: ObjectId,
        next_event_serial: u32,
        receiver: Option<mpsc::Receiver<ClientMessage>>,
        sender: mpsc::Sender<ClientMessage>,
        client_id: u32,
    }
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Client").field(&self.client_id).finish()
    }
}

impl waynest::Connection for Client {
    type Error = VerdiError;

    fn fd(&mut self) -> std::result::Result<std::os::unix::prelude::OwnedFd, VerdiError> {
        self.socket.fd().map_err(VerdiError::from)
    }
}

impl Stream for Client {
    type Item = Result<Message, ProtocolError>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.project().socket.poll_next(cx)
    }
}

impl Sink<Message> for Client {
    type Error = ProtocolError;

    fn poll_ready(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        self.project().socket.poll_ready(cx)
    }

    fn start_send(
        self: std::pin::Pin<&mut Self>,
        item: Message,
    ) -> std::result::Result<(), Self::Error> {
        self.project().socket.start_send(item)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        self.project().socket.poll_flush(cx)
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        self.project().socket.poll_close(cx)
    }
}

impl waynest_server::Client for Client {
    type Store = Store<Client, VerdiError>;

    fn store(&self) -> &Self::Store {
        &self.store
    }

    fn store_mut(&mut self) -> &mut Self::Store {
        &mut self.store
    }
}

impl Client {
    pub fn new(stream: UnixStream, client_id: u32) -> Result<Self, VerdiError> {
        let (sender, receiver) = mpsc::channel(128);

        let mut client = Self {
            socket: Socket::new(stream.into_std()?)?,
            store: Store::new(),
            next_object_id: unsafe { ObjectId::from_raw(0xff000000) },
            next_event_serial: 0,
            receiver: Some(receiver),
            sender,
            client_id,
        };

        let _ = client.insert(ObjectId::DISPLAY, Display::default());

        tracing::debug!("Created new client with id {client_id}");

        Ok(client)
    }

    pub fn handle(&self) -> ClientHandle {
        ClientHandle::new(self.sender.clone(), self.client_id)
    }

    pub fn next_event_serial(&mut self) -> u32 {
        let prev = self.next_event_serial;
        self.next_event_serial = self.next_event_serial.wrapping_add(1);

        prev
    }

    pub async fn run(mut self) {
        let mut receiver = self.receiver.take().expect("Internal error");

        loop {
            tokio::select! {
                biased;
                msg = self.try_next() => {
                    match msg {
                        Ok(Some(mut msg)) => {
                            if let Err(err) = self
                                .get_raw(msg.object_id())
                                .ok_or(VerdiError::MissingObject(msg.object_id())).unwrap()
                                .dispatch_request(&mut self, msg.object_id(), &mut msg)
                                .await
                            {
                                error!("Error while handling message: {err}");
                                // return Err(err.into());
                            }
                        },
                        Ok(None) => todo!(),
                        Err(_) => todo!(),
                    }
                }
                Some(msg) = receiver.recv() => {
                    self.handle_message(msg);
                }
            }
        }
    }

    fn handle_message(&mut self, msg: ClientMessage) {
        match msg {}
    }
}

impl ClientHandle {
    pub fn new(sender: mpsc::Sender<ClientMessage>, client_id: u32) -> Self {
        Self { sender, client_id }
    }
}
