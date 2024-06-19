use anyhow::{Context, Result};
use bytes::{Bytes, BytesMut};
use futures_util::SinkExt;
use std::{fs, process::exit, sync::Arc};
use tokio::{net::UnixListener, sync::Mutex, task::JoinSet};
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;
use tracing::{debug, error, info, warn};
use tracing_subscriber::prelude::*;
use verdi::message::{Message, MessageCodec};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_backend::WinitBackend;

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

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // Create the tokio runtime manually instead of using a macro for better controll
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("Failed to create tokio runtime")?;

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = WinitBackend::new();

    runtime.block_on(async move {
        register_ctrl_c_handler();

        let socket = UnixListener::bind(SOCKET_PATH)?;

        let server_objects = Arc::new(Mutex::new(Objects::new_server_store()));

        let mut set: JoinSet<Result<(), anyhow::Error>> = JoinSet::new();

        let listener_handle = tokio::spawn(async move {
            loop {
                match socket.accept().await {
                    Ok((stream, _addr)) => {
                        info!("Got client");

                        let server_objects = server_objects.clone();
                        let mut stream = Framed::new(stream, MessageCodec::new());

                        set.spawn(async move {
                            while let Some(ref msg) = stream.try_next().await? {
                                dbg!(msg);

                                if let Some(object) = server_objects.lock().await.get(msg.object_id)
                                {
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

                                            // stream
                                            //     .send(Message {
                                            //         object_id: new_id,
                                            //         opcode: 0,
                                            //         payload: Bytes::copy_from_slice(&1u32.to_ne_bytes()),
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
                    Err(e) => {
                        error!("Client failed to connect")
                    }
                }
            }
        });

        listener_handle.await?;

        anyhow::Ok(())
    })?;

    event_loop.run_app(&mut app)?;

    Ok(())
}

pub enum Argument {
    Int(i32),
    Uint(u32),
    Fixed(i32),
}

struct Objects {
    objects: Vec<Box<dyn Interface + Send>>,
}

impl Objects {
    pub fn new_server_store() -> Self {
        Self {
            objects: vec![Box::new(DisplayInterface {})],
        }
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
