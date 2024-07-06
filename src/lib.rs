use anyhow::Result as AnyResult;
use futures_util::SinkExt;
use std::{
    collections::HashMap,
    io,
    path::Path,
    sync::{Arc, Mutex},
};
use tokio::net::{UnixListener, UnixStream};
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;
use tracing::{debug, warn};
// use tracing::{debug, error, info, warn};

pub mod error;
pub mod message;
pub mod proto;

use message::{DecodeError, Message, MessageCodec, ObjectId};
use proto::wayland::WlDisplay;

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

                client.insert(
                    unsafe { ObjectId::from_raw(1) },
                    Box::new(DisplayInterface {}),
                );

                Some(Ok(client))
            }
            Err(err) => Some(Err(err)),
        }
    }
}

#[derive(Debug)]
pub struct Client {
    stream: Framed<UnixStream, MessageCodec>,
    store: Store,
    _next_id: usize,
}

impl Client {
    pub fn new(stream: UnixStream) -> Self {
        Self {
            stream: Framed::new(stream, MessageCodec::new()),
            _next_id: 0xff000000,
            store: Store::new(),
        }
    }

    pub fn insert(&mut self, id: ObjectId, object: Box<dyn Interface + Send + Sync>) {
        self.store.insert(id, object)
    }

    pub fn handle_message(&mut self, message: &mut Message) {
        let object = self.store.get(&message.object_id).unwrap();

        object.handle_request(self, message).unwrap();
    }

    pub async fn next_message(&mut self) -> Result<Option<Message>, DecodeError> {
        self.stream.try_next().await
    }

    pub async fn send_message(&mut self, message: Message) -> Result<(), io::Error> {
        self.stream.send(message).await
    }
}

#[derive(Debug)]
struct Store {
    objects: HashMap<ObjectId, Arc<Box<dyn Interface + Send + Sync>>>,
}

impl Store {
    fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }
    // FIXME: handle possible error if id already exists
    fn insert(&mut self, id: ObjectId, object: Box<dyn Interface + Send + Sync>) {
        self.objects.insert(id, Arc::new(object));
    }

    fn get(&self, id: &ObjectId) -> Option<Arc<Box<dyn Interface + Send + Sync>>> {
        self.objects.get(id).map(|id| id.clone())
    }
}

pub trait Interface: std::fmt::Debug {
    fn handle_request(&self, client: &mut Client, message: &mut Message) -> Result<()>;
}

#[derive(Debug)]
pub struct DisplayInterface {}

impl Interface for DisplayInterface {
    fn handle_request(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        <Self as WlDisplay>::handle_request(client, message)
    }
}

impl WlDisplay for DisplayInterface {
    fn sync(_client: &mut Client, _callback: ObjectId) -> Result<()> {
        debug!("Handling sync");
        todo!()
    }

    fn get_registry(_client: &mut Client, _registry: ObjectId) -> Result<()> {
        debug!("Handling get_registry");
        todo!()
    }
}
