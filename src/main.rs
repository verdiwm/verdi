use anyhow::{Context, Result};
use bytes::{Bytes, BytesMut};
use futures_util::SinkExt;
use std::{fs, io, path::Path, process::exit, sync::Arc};
use tokio::{
    net::{UnixListener, UnixStream},
    sync::Mutex,
    task::JoinSet,
};
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;
use tracing::{debug, error, info, warn};
use verdi::{
    message::{DecodeError, Message, MessageCodec},
    proto::wayland::WlDisplay,
};

// mod winit_backend;

const SERVER_ID_START: usize = 0xff000000;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

const SOCKET_PATH: &str = "verdi.sock";

/// Register a ctrl+c handler that ensures the socket is removed
fn register_ctrl_c_handler() {
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for signals");

        // Check if the socket actually exists
        if fs::metadata(SOCKET_PATH).is_ok() {
            // We still want to gracefully exit even on error
            if fs::remove_file(SOCKET_PATH).is_err() {
                error!("Failed to remove old socket");
                exit(1)
            }
        }

        exit(0)
    });
}

#[derive(Debug)]
struct Verdi {
    state: Arc<State>,
    listener: UnixListener,
}

#[derive(Debug)]
struct State {}

impl Verdi {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Self {
            state: Arc::new(State {}),
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
struct Client {
    stream: Framed<UnixStream, MessageCodec>,
    store: Store,
}

impl Client {
    pub async fn next_message(&mut self) -> Result<Option<Message>, DecodeError> {
        self.stream.try_next().await
    }

    pub async fn send_message(&mut self, message: Message) -> Result<(), io::Error> {
        self.stream.send(message).await
    }
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // Create the tokio runtime manually instead of using a macro for better controll
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("Failed to create tokio runtime")?;

    runtime.block_on(async move {
        register_ctrl_c_handler();

        let verdi = Verdi::new(SOCKET_PATH)?;

        let mut set: JoinSet<Result<(), anyhow::Error>> = JoinSet::new();

        while let Some(client) = verdi.new_client().await {
            info!("Got client");

            match client {
                Ok(mut client) => {
                    set.spawn(async move {
                        while let Some(ref msg) = client.next_message().await? {
                            dbg!(msg);

                            if let Some(object) = client.store.get(msg.object_id) {
                                object.handle_request(msg)?;
                            } else {
                                warn!("Unknown object requested");
                            }
                        }

                        Ok(())
                    });
                }
                Err(_) => error!("Client failed to connect"),
            }
        }

        anyhow::Ok(())
    })?;

    Ok(())
}

#[derive(Debug)]
struct Store {
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
    fn handle_request(&self, message: &Message) -> verdi::Result<()>;
}

#[derive(Debug)]
pub struct DisplayInterface {}

impl Interface for DisplayInterface {
    fn handle_request(&self, message: &Message) -> verdi::Result<()> {
        <Self as WlDisplay>::handle_request(message)
    }
}

impl WlDisplay for DisplayInterface {
    fn sync() -> verdi::Result<()> {
        todo!()
    }

    fn get_registry() -> verdi::Result<()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct CallbackInterface {}

impl CallbackInterface {
    pub fn call(&self, opcode: u16) -> Message {
        todo!()
    }
}

impl Interface for CallbackInterface {
    fn handle_request(&self, message: &Message) -> verdi::Result<()> {
        todo!()
    }
}

pub struct Request {}
