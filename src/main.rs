use anyhow::Result;
use message::MessageCodec;
use std::{fs, process::exit};
use tokio::net::UnixListener;
use tokio_stream::StreamExt;
use tokio_util::codec::FramedRead;

mod message;

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

    loop {
        match socket.accept().await {
            Ok((stream, _addr)) => {
                println!("Got client");

                let mut messages = FramedRead::new(stream, MessageCodec::new());

                while let Some(msg) = messages.try_next().await? {
                    dbg!(msg);
                }
            }
            Err(e) => {
                println!("Client failed to connect")
            }
        }
    }

    Ok(())
}

pub struct DisplayInterface {}
