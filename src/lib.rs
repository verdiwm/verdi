use anyhow::Result as AnyResult;
use async_trait::async_trait;
use futures_util::SinkExt;
use std::{collections::HashMap, io, path::Path, sync::Arc};
use tokio::net::{UnixListener, UnixStream};
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;
use tracing::{debug, warn};
// use tracing::{debug, error, info, warn};

pub mod error;
pub mod message;
pub mod proto;

use message::{DecodeError, Message, MessageCodec, ObjectId};
use proto::wayland::{WlCompositor, WlDisplay, WlRegistry};

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

                client.insert(id, DisplayInterface::create_dispatcher(id));

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
}

impl Client {
    pub fn new(stream: UnixStream) -> Self {
        Self {
            stream: Framed::new(stream, MessageCodec::new()),
            _next_id: 0xff000000,
            store: Store::new(),
        }
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

#[derive(Debug)]
pub struct DisplayInterface;

impl WlDisplay for DisplayInterface {
    async fn sync(_client: &mut Client, _callback: ObjectId) -> Result<()> {
        Ok(())
    }

    async fn get_registry(client: &mut Client, registry_id: ObjectId) -> Result<()> {
        let registry = RegistryInterface::create_dispatcher(registry_id);
        client.insert(registry_id, registry);

        RegistryInterface::global(
            registry_id,
            client,
            0,
            Compositor::INTERFACE.to_string(),
            Compositor::VERSION,
        )
        .await?;

        Ok(())
    }

    fn create_dispatcher(id: ObjectId) -> Arc<Box<dyn Dispatcher + Send + Sync>> {
        Arc::new(Box::new(Self {}))
    }
}

#[async_trait]
impl Dispatcher for DisplayInterface {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        <Self as WlDisplay>::handle_request(client, message).await
    }
}

#[derive(Debug)]
pub struct RegistryInterface;

impl WlRegistry for RegistryInterface {
    async fn r#bind(client: &mut Client, r#name: u32, r#id: message::NewId) -> Result<()> {
        todo!()
    }

    fn create_dispatcher(id: ObjectId) -> Arc<Box<dyn Dispatcher + Send + Sync>> {
        Arc::new(Box::new(Self {}))
    }
}

#[async_trait]
impl Dispatcher for RegistryInterface {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        <Self as WlRegistry>::handle_request(client, message).await
    }
}

#[derive(Debug)]
pub struct Compositor;

impl WlCompositor for Compositor {
    async fn r#create_surface(client: &mut Client, r#id: ObjectId) -> Result<()> {
        todo!()
    }

    async fn r#create_region(client: &mut Client, r#id: ObjectId) -> Result<()> {
        todo!()
    }

    fn create_dispatcher(id: ObjectId) -> Arc<Box<dyn Dispatcher + Send + Sync>> {
        todo!()
    }
}
