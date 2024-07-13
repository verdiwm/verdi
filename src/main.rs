use std::{fs, io, path::Path, process::exit, sync::Arc};

use anyhow::{Context, Result as AnyResult};
use clap::Parser;
use rustix::process::geteuid;
use serde::{Deserialize, Serialize};
use tokio::{net::UnixListener, task::JoinSet};
use tracing::{error, info};

use verdi::{
    error::Error,
    protocol::wayland::display::{Display, WlDisplay},
    wire::ObjectId,
    Client,
};

const SOCKET_PATH: &str = "verdi.sock";

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
            Ok((stream, _addr)) => {
                // FIXME: handle error instead of unwraping
                let mut client = Client::new(stream).unwrap();

                client.insert(Display::new().into_object(ObjectId::DISPLAY));

                Some(Ok(client))
            }
            Err(err) => Some(Err(err)),
        }
    }
}

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

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Optional path to config file
    #[arg(short, long)]
    config: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {}

fn main() -> AnyResult<()> {
    tracing_subscriber::fmt::init();

    if geteuid().is_root() {
        error!("Tried running as root");
        exit(1)
    }

    let args = Args::parse();

    // FIXME: Try to look into XDG_CONFIG_HOME and have some defaults
    let config = fs::read(args.config.expect("Missing config"))?;
    let config: Config = toml_edit::de::from_slice(&config)?;

    dbg!(config);

    // Create the tokio runtime manually instead of using a macro for better controll
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("Failed to create tokio runtime")?;

    runtime.block_on(async move {
        register_ctrl_c_handler();

        let verdi = Verdi::new(SOCKET_PATH)?;

        let mut set: JoinSet<Result<(), Error>> = JoinSet::new();

        while let Some(client) = verdi.new_client().await {
            info!("Got client");

            match client {
                Ok(mut client) => {
                    set.spawn(async move {
                        while let Some(mut message) = client.next_message().await? {
                            match client.handle_message(&mut message).await {
                                Ok(_) => {}
                                Err(err) => {
                                    error!("Error while handling message: {err}");
                                    return Err(err);
                                }
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
