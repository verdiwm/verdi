use anyhow::{Context, Result};
use bytes::{Bytes, BytesMut};
use futures_util::SinkExt;
use std::{fs, io, path::Path, process::exit, sync::Arc};
use tokio::{
    net::{unix::SocketAddr, UnixListener, UnixStream},
    sync::Mutex,
    task::JoinSet,
};
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;
use tracing::{debug, error, info, warn};
use tracing_subscriber::prelude::*;
use verdi::message::{DecodeError, Message, MessageCodec};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_backend::WinitBackend;

mod core;
mod winit_backend;

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
struct State {
    store: Mutex<Store>,
}

impl Verdi {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut store = Store::new();

        store.insert(Box::new(DisplayInterface {}));

        Ok(Self {
            state: Arc::new(State {
                store: Mutex::new(store),
            }),
            listener: UnixListener::bind(path)?,
        })
    }

    pub async fn new_client(&self) -> Option<Result<Client, io::Error>> {
        match self.listener.accept().await {
            Ok((stream, _)) => Some(Ok(Client {
                stream: Framed::new(stream, MessageCodec::new()),
            })),
            Err(err) => Some(Err(err)),
        }
    }
}

#[derive(Debug)]
struct Client {
    stream: Framed<UnixStream, MessageCodec>,
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

    // let event_loop = EventLoop::new()?;
    // event_loop.set_control_flow(ControlFlow::Wait);

    // let mut app = WinitBackend::new();

    runtime.block_on(async move {
        register_ctrl_c_handler();

        let verdi = Verdi::new(SOCKET_PATH)?;

        let mut set: JoinSet<Result<(), anyhow::Error>> = JoinSet::new();

        while let Some(client) = verdi.new_client().await {
            info!("Got client");

            match client {
                Ok(mut client) => {
                    let state = verdi.state.clone();

                    set.spawn(async move {
                        while let Some(ref msg) = client.next_message().await? {
                            dbg!(msg);

                            if let Some(object) = state.store.lock().await.get(msg.object_id) {
                                debug!(
                                    "\"{}\" object requested with request \"{}\"",
                                    object.name(),
                                    object.get_request(msg.opcode).unwrap_or("unknown")
                                );

                                if let Some(request) = object.get_request(msg.opcode) {
                                    if request == "sync" {
                                        let new_id = u32::from_ne_bytes([
                                            msg.payload[0],
                                            msg.payload[1],
                                            msg.payload[2],
                                            msg.payload[3],
                                        ]);

                                        dbg!(new_id);

                                        // client
                                        //     .send_message(Message {
                                        //         object_id: new_id,
                                        //         opcode: 0,
                                        //         payload: Bytes::copy_from_slice(
                                        //             &1u32.to_ne_bytes(),
                                        //         ),
                                        //     })
                                        //     .await?;
                                    }

                                    if request == "get_registry" {
                                        let new_id = u32::from_ne_bytes([
                                            msg.payload[0],
                                            msg.payload[1],
                                            msg.payload[2],
                                            msg.payload[3],
                                        ]);

                                        dbg!(new_id);
                                    }
                                }
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

    // event_loop.run_app(&mut app)?;

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
    fn name(&self) -> &'static str;
    fn get_request(&self, opcode: u16) -> Option<&'static str>;
}

#[derive(Debug)]
pub struct DisplayInterface {}

impl Interface for DisplayInterface {
    fn name(&self) -> &'static str {
        "wl_display"
    }

    fn get_request(&self, opcode: u16) -> Option<&'static str> {
        match opcode {
            0 => Some("sync"),
            1 => Some("get_registry"),
            _ => None,
        }
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
    fn name(&self) -> &'static str {
        "wl_callback"
    }

    fn get_request(&self, opcode: u16) -> Option<&'static str> {
        match opcode {
            0 => Some("done"),
            _ => None,
        }
    }
}

pub struct Request {}
