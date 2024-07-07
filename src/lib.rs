use anyhow::Result as AnyResult;
use async_trait::async_trait;
use error::Error;
use futures_util::SinkExt;
use std::{collections::HashMap, io, path::Path, sync::Arc};
use tokio::net::{UnixListener, UnixStream};
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

pub mod error;
pub mod protocol;
pub mod wire;

use protocol::wayland::display::{Display, WlDisplay};
use wire::{DecodeError, Message, MessageCodec, ObjectId};

pub type Result<T, E = error::Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub struct Verdi {
    _state: Arc<State>,
    listener: UnixListener,
}

#[derive(Debug)]
struct State {}

impl Verdi {
    pub fn new<P: AsRef<Path>>(path: P) -> AnyResult<Self> {
        Ok(Self {
            _state: Arc::new(State {}),
            listener: UnixListener::bind(path)?,
        })
    }

    pub async fn new_client(&self) -> Option<Result<Client, io::Error>> {
        match self.listener.accept().await {
            Ok((stream, _)) => {
                let mut client = Client::new(stream);

                let id = unsafe { ObjectId::from_raw(1) };

                client.insert(id, Display::create_dispatcher());

                Some(Ok(client))
            }
            Err(err) => Some(Err(err)),
        }
    }
}

pub struct Client {
    stream: Framed<UnixStream, MessageCodec>,
    store: Store,
    _next_id: usize,
    event_serial: u32,
}

impl Client {
    pub fn new(stream: UnixStream) -> Self {
        Self {
            stream: Framed::new(stream, MessageCodec::new()),
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

    pub fn insert(&mut self, id: ObjectId, object: Arc<Box<dyn Dispatcher + Send + Sync>>) {
        self.store.insert(id, object)
    }

    pub async fn handle_message(&mut self, message: &mut Message) -> Result<()> {
        let object = self.store.get(&message.object_id).unwrap();

        object.dispatch(self, message).await
    }

    pub async fn next_message(&mut self) -> Result<Option<Message>, DecodeError> {
        self.stream.try_next().await
    }

    pub async fn send_message(&mut self, message: Message) -> Result<(), io::Error> {
        self.stream.send(message).await
    }
}

struct Store {
    objects: HashMap<ObjectId, Arc<Box<dyn Dispatcher + Send + Sync>>>,
}

impl Store {
    fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }
    // FIXME: handle possible error if id already exists
    fn insert(&mut self, id: ObjectId, object: Arc<Box<dyn Dispatcher + Send + Sync>>) {
        self.objects.insert(id, object);
    }

    fn get(&self, id: &ObjectId) -> Option<Arc<Box<dyn Dispatcher + Send + Sync>>> {
        self.objects.get(id).map(|id| id.clone())
    }
}

#[async_trait]
pub trait Dispatcher {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()>;
}
