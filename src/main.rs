#![allow(unused)]
#![allow(clippy::default_constructed_unit_structs)]

use std::{
    fs,
    os::fd::{FromRawFd, IntoRawFd, OwnedFd},
    path::{Path, PathBuf},
    process::{Stdio, exit},
    time::Duration,
};

use anyhow::{Context, Result as AnyResult, bail};
use clap::Parser;
use rustix::{
    fs::{Mode, OFlags},
    io::Errno,
    process::geteuid,
};
use serde::{Deserialize, Serialize};
use tokio::{net::UnixListener, process::Command, sync::mpsc, task::JoinSet, time::sleep};
use tokio_stream::{StreamExt, wrappers::UnboundedReceiverStream};
use tracing::{debug, error};

use tracing_subscriber::EnvFilter;
use verdi::{
    error::Error,
    protocol::wayland::display::{Display, WlDisplay},
};

use waynest::{
    server::{Client, Listener},
    wire::ObjectId,
};

mod context;
mod libinput;
mod state;

use context::WgpuContext;
// use state::State;

const fn version() -> &'static str {
    concat!(
        env!("CARGO_PKG_VERSION"),
        " (",
        env!("GIT_HASH"),
        " ",
        env!("COMMIT_DATE"),
        ")"
    )
}

#[derive(Parser, Debug)]
#[command(version = version(), about, long_about = None)]
struct Args {
    /// Custom config file path
    #[arg(short, long)]
    config: Option<PathBuf>,
    /// Custom wayland socket path
    #[arg(short, long)]
    socket: Option<PathBuf>,
    /// A client to exec on startup
    #[arg(short, long)]
    exec: Option<PathBuf>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {}

fn main() -> AnyResult<()> {
    let format = tracing_subscriber::fmt::format()
        .with_level(false)
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(false)
        .without_time()
        .compact();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .event_format(format)
        .init();

    if geteuid().is_root() {
        error!("Tried running as root");
        bail!("")
    }

    let args = Args::parse();

    let config_path = if let Some(config) = args.config {
        config
    } else if let Ok(path) = std::env::var("XDG_CONFIG_HOME") {
        Path::new(&path).join("verdi/verdi.corn")
    } else if let Some(path) = home::home_dir() {
        path.join(".config/verdi/verdi.corn")
    } else {
        todo!()
    };

    let config: Config = corn::from_slice(&fs::read(config_path)?)?;
    dbg!(config);

    // Create the tokio runtime manually instead of using a macro for better controll
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("Failed to create tokio runtime")?;

    debug!("Created runtime");

    runtime.block_on(async move {
        let listener = {
            if let Some(socket) = args.socket {
                Listener::new_with_path(socket)
            } else {
                Listener::new()
            }
        }?;

        dbg!(listener.socket_path());

        let socket_path = listener.socket_path().canonicalize()?;

        debug!("Started listener at path {}", socket_path.display());

        let mut verdi = Verdi::new(listener).await?;

        debug!("Started new verdi instance");

        tokio::spawn(async move {
            while let Some(event) = verdi.next_event().await? {
                dbg!(&event);
                #[allow(clippy::single_match)]
                match event {
                    Event::NewClient(client) => verdi.spawn_client(client)?,
                    _ => {}
                }
            }

            anyhow::Ok(())
        });

        // FAILSAFE
        tokio::spawn({
            // let session = session.clone();

            async move {
                sleep(Duration::from_secs(5)).await;
                exit(-1)

                // let _ = session.release_control().await;
            }
        });

        let render_context = WgpuContext::new().await?;

        if let Some(exec) = args.exec {
            tokio::spawn(async move {
                let mut child = Command::new(exec)
                    .env("WAYLAND_DISPLAY", socket_path)
                    .env("WAYLAND_DEBUG", "1")
                    .stdout(Stdio::null())
                    .stdin(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()?;

                child.wait().await?;

                anyhow::Ok(())
            });
        }

        loop {
            render_context.present()?;
        }

        anyhow::Ok(())
    })?;

    Ok(())
}

pub struct Verdi {
    events_receiver: UnboundedReceiverStream<Result<Event, Error>>,
    clients: JoinSet<Result<(), Error>>,
}

#[derive(Debug)]
pub enum Event {
    NewClient(Client),
    SessionPaused,
    SessionResumed,
    Input(libinput::Event),
}

impl Verdi {
    pub async fn new(mut listener: Listener) -> AnyResult<Self> {
        let (tx, mut rx) = mpsc::unbounded_channel();

        debug!("Creating libinput instance");

        let (mut event_stream, shutdown_handle) = libinput::spawn_libinput_task()?;

        // FIXME: handle errors instead of unwraping
        tokio::spawn({
            let tx = tx.clone();

            async move {
                while let Some(event) = event_stream.try_next().await.unwrap() {
                    tx.send(Ok(Event::Input(event))).unwrap();
                }
            }
        });

        debug!("Created libinput instance");

        // FIXME: handle errors instead of unwraping
        tokio::spawn(async move {
            while let Some(stream) = listener.try_next().await.unwrap() {
                let mut client = Client::new(stream).unwrap();

                client.insert(ObjectId::DISPLAY, Display::default());

                tx.send(Ok(Event::NewClient(client))).unwrap();
            }
        });

        debug!("Spawned stuff");

        Ok(Self {
            events_receiver: UnboundedReceiverStream::new(rx),
            clients: JoinSet::new(),
        })
    }

    pub async fn next_event(&mut self) -> Result<Option<Event>, Error> {
        self.events_receiver.try_next().await
    }

    pub fn spawn_client(&mut self, mut client: Client) -> Result<(), Error> {
        self.clients.spawn(async move {
            while let Some(mut message) = client.next_message().await? {
                match client.handle_message(&mut message).await {
                    Ok(_) => {}
                    Err(err) => {
                        error!("Error while handling message: {err}");
                        return Err(err.into());
                    }
                }
            }

            Ok(())
        });

        Ok(())
    }
}
