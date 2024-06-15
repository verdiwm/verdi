use anyhow::Result;
use bytes::{Bytes, BytesMut};
use futures::SinkExt;
use message::{Message, MessageCodec};
use std::{fs, process::exit};
use tokio::net::UnixListener;
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, FramedRead};

mod message;

const SERVER_ID_START: usize = 0xff000000;

#[tokio::main]
async fn main() -> Result<()> {
    let socket_path = "verdi.sock";

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();

        if fs::metadata(socket_path).is_ok() {
            fs::remove_file(socket_path).expect("Fuck");
        }

        exit(0)
    });

    let socket = UnixListener::bind(socket_path)?;

    let mut server_objects = Objects::new_server_store();

    loop {
        match socket.accept().await {
            Ok((stream, _addr)) => {
                println!("Got client");

                let mut stream = Framed::new(stream, MessageCodec::new());

                while let Some(ref msg) = stream.try_next().await? {
                    dbg!(msg);

                    if let Some(object) = server_objects.get(msg.object_id) {
                        println!(
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

                                stream
                                    .send(Message {
                                        object_id: new_id,
                                        opcode: 0,
                                        payload: Bytes::copy_from_slice(&1u32.to_ne_bytes()),
                                    })
                                    .await?;
                            }
                        }
                    } else {
                        println!("Unknown object requested");
                    }
                }
            }
            Err(e) => {
                println!("Client failed to connect")
            }
        }
    }

    Ok(())
}

pub enum Argument {
    Int(i32),
    Uint(u32),
    Fixed(i32),
}

struct Objects {
    objects: Vec<Box<dyn Interface>>,
}

impl Objects {
    pub fn new_server_store() -> Self {
        Self {
            objects: vec![Box::new(DisplayInterface {})],
        }
    }

    pub fn get(&self, id: u32) -> Option<&Box<dyn Interface>> {
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
