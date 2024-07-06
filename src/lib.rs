use anyhow::Result as AnyResult;
use async_trait::async_trait;
use error::Error;
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

use message::{DecodeError, Message, MessageCodec, NewId, ObjectId};
use proto::wayland::{WlCallback, WlCompositor, WlDisplay, WlRegistry, WlShm};

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

                client.insert(id, Display::create_dispatcher(id));

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
        dbg!(&message);
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
pub struct Display;

impl WlDisplay for Display {
    async fn sync(client: &mut Client, callback: ObjectId) -> Result<()> {
        let serial = client.next_event_serial();

        Callback::done(callback, client, serial).await?;

        Self::delete_id(unsafe { ObjectId::from_raw(1) }, client, callback.as_raw()).await
    }

    async fn get_registry(client: &mut Client, registry_id: ObjectId) -> Result<()> {
        let registry = Registry::create_dispatcher(registry_id);
        client.insert(registry_id, registry);

        Registry::global(
            registry_id,
            client,
            0,
            Compositor::INTERFACE.to_string(),
            Compositor::VERSION,
        )
        .await?;

        Registry::global(
            registry_id,
            client,
            1,
            Shm::INTERFACE.to_string(),
            Shm::VERSION,
        )
        .await?;

        Ok(())
    }

    fn create_dispatcher(id: ObjectId) -> Arc<Box<dyn Dispatcher + Send + Sync>> {
        Arc::new(Box::new(Self {}))
    }
}

#[async_trait]
impl Dispatcher for Display {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        <Self as WlDisplay>::handle_request(client, message).await
    }
}

#[derive(Debug)]
pub struct Registry;

impl WlRegistry for Registry {
    async fn r#bind(client: &mut Client, name: u32, id: NewId) -> Result<()> {
        match name {
            0 => client.insert(id.id, Compositor::create_dispatcher(id.id)),
            1 => client.insert(id.id, Shm::create_dispatcher(id.id)),
            _ => return Err(Error::NotFound),
        }

        Ok(())
    }

    fn create_dispatcher(id: ObjectId) -> Arc<Box<dyn Dispatcher + Send + Sync>> {
        Arc::new(Box::new(Self {}))
    }
}

#[async_trait]
impl Dispatcher for Registry {
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
        Arc::new(Box::new(Self {}))
    }
}

#[async_trait]
impl Dispatcher for Compositor {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        <Self as WlCompositor>::handle_request(client, message).await
    }
}

#[derive(Debug)]
pub struct Callback;

impl WlCallback for Callback {
    fn create_dispatcher(id: ObjectId) -> Arc<Box<dyn Dispatcher + Send + Sync>> {
        Arc::new(Box::new(Self {}))
    }
}

#[async_trait]
impl Dispatcher for Callback {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        <Self as WlCallback>::handle_request(client, message).await
    }
}

#[derive(Debug)]
pub struct Shm;

impl WlShm for Shm {
    async fn r#create_pool(
        client: &mut Client,
        r#id: ObjectId,
        r#fd: std::os::unix::prelude::RawFd,
        r#size: i32,
    ) -> Result<()> {
        todo!()
    }

    async fn r#release(client: &mut Client) -> Result<()> {
        todo!()
    }

    fn create_dispatcher(id: ObjectId) -> Arc<Box<dyn Dispatcher + Send + Sync>> {
        Arc::new(Box::new(Self {}))
    }
}

#[async_trait]
impl Dispatcher for Shm {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        <Self as WlShm>::handle_request(client, message).await
    }
}