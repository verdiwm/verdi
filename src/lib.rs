#![feature(unix_socket_ancillary_data)]

use async_trait::async_trait;
use downcast_rs::{impl_downcast, DowncastSync};
use error::Error;
use futures_util::SinkExt;
use std::{collections::HashMap, io, sync::Arc};
use tokio::net::UnixStream;
use tokio_stream::StreamExt;

pub mod error;
pub mod protocol;
pub mod wire;

use wire::{DecodeError, Message, ObjectId, Socket};

pub type Result<T, E = error::Error> = core::result::Result<T, E>;

pub struct Client {
    socket: Socket,
    store: Store,
    _next_id: usize,
    event_serial: u32,
}

impl Client {
    pub fn new(stream: UnixStream) -> Self {
        // let fd = stream.as_fd().as_raw_fd();

        Self {
            // stream: Framed::new(stream, MessageCodec::new(fd)),
            socket: Socket::new(stream.into_std().unwrap()),
            _next_id: 0xff000000,
            store: Store::new(),
            event_serial: 0,
        }
    }

    pub fn next_event_serial(&mut self) -> u32 {
        let prev = self.event_serial;
        self.event_serial = self.event_serial.wrapping_add(1);

        prev
    }

    pub fn insert(&mut self, object: Arc<Object>) {
        self.store.insert(object)
    }

    pub async fn handle_message(&mut self, message: &mut Message) -> Result<()> {
        let object = self.store.get(&message.object_id).unwrap();

        object.dispatch(self, message).await
    }

    pub async fn next_message(&mut self) -> Result<Option<Message>, DecodeError> {
        self.socket.try_next().await
    }

    pub async fn send_message(&mut self, message: Message) -> Result<(), io::Error> {
        self.socket.send(message).await
    }
}

pub struct Object {
    pub id: ObjectId,
    dispatcher: Box<dyn Dispatcher>,
}

impl Object {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        self.dispatcher.dispatch(self.id, client, message).await
    }
}

struct Store {
    objects: HashMap<ObjectId, Arc<Object>>,
}

impl Store {
    fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }
    // FIXME: handle possible error if id already exists
    fn insert(&mut self, object: Arc<Object>) {
        self.objects.insert(object.id, object);
    }

    fn get(&self, id: &ObjectId) -> Option<Arc<Object>> {
        self.objects.get(id).map(|id| id.clone())
    }
}

#[async_trait]
pub trait Dispatcher: DowncastSync {
    async fn dispatch(
        &self,
        object_id: ObjectId,
        client: &mut Client,
        message: &mut Message,
    ) -> Result<()>;
}

impl_downcast!(sync Dispatcher);
