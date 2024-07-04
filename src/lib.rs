use anyhow::Result as AnyResult;
use futures_util::SinkExt;
use std::{io, path::Path, sync::Arc};
use tokio::net::{UnixListener, UnixStream};
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;
// use tracing::{debug, error, info, warn};

pub mod error;
pub mod message;
pub mod proto;

use message::{DecodeError, Message, MessageCodec};
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
                let mut store = Store::new();

                store.insert(Box::new(DisplayInterface {}));

                Some(Ok(Client {
                    stream: Framed::new(stream, MessageCodec::new()),
                    store,
                }))
            }
            Err(err) => Some(Err(err)),
        }
    }
}

#[derive(Debug)]
pub struct Client {
    stream: Framed<UnixStream, MessageCodec>,
    pub store: Store,
}

impl Client {
    pub async fn next_message(&mut self) -> Result<Option<Message>, DecodeError> {
        self.stream.try_next().await
    }

    pub async fn send_message(&mut self, message: Message) -> Result<(), io::Error> {
        self.stream.send(message).await
    }
}

#[derive(Debug)]
pub struct Store {
    objects: Vec<Box<dyn Interface + Send>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn insert(&mut self, object: Box<dyn Interface + Send>) {
        self.objects.push(object);
    }

    pub fn get(&self, id: u32) -> Option<&Box<dyn Interface + Send>> {
        if id == 0 {
            return None;
        }

        self.objects.get((id - 1) as usize)
    }
}

pub trait Interface: std::fmt::Debug {
    fn handle_request(&self, message: &Message) -> Result<()>;
}

#[derive(Debug)]
pub struct DisplayInterface {}

impl Interface for DisplayInterface {
    fn handle_request(&self, message: &Message) -> Result<()> {
        <Self as WlDisplay>::handle_request(message)
    }
}

impl WlDisplay for DisplayInterface {
    fn sync() -> Result<()> {
        todo!()
    }

    fn get_registry() -> Result<()> {
        todo!()
    }
}

// #[derive(Debug)]
// pub struct CallbackInterface {}

// impl CallbackInterface {
//     pub fn call(&self, opcode: u16) -> Message {
//         todo!()
//     }
// }

// impl Interface for CallbackInterface {
//     fn handle_request(&self, message: &Message) -> Result<()> {
//         todo!()
//     }
// }

// pub struct Request {}
