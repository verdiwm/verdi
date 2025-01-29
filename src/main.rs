#![allow(unused)]
#![allow(clippy::default_constructed_unit_structs)]

use std::{
    // ffi::CString,
    fs,
    os::fd::{FromRawFd, IntoRawFd, OwnedFd},
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result as AnyResult};
use clap::Parser;
use colpetto::Libinput;
// use colpetto::Libinput;
use rustix::{
    fs::{Mode, OFlags},
    io::Errno,
    process::geteuid,
};
use serde::{Deserialize, Serialize};
use tokio::{net::UnixListener, sync::mpsc, task::JoinSet};
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt};
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
mod state;

// use context::WgpuContext;
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
        Path::new(&path).join("verdi/verdi.toml")
    } else if let Some(path) = home::home_dir() {
        path.join(".config/verdi/verdi.toml")
    } else {
        todo!()
    };

    let config: Config = toml_edit::de::from_slice(&fs::read(config_path)?)?;
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

        debug!("Started listener");

        let mut verdi = Verdi::new(listener).await?;

        debug!("Started new verdi instance");

        while let Some(event) = verdi.next_event().await? {
            dbg!(&event);
            #[allow(clippy::single_match)]
            match event {
                Event::NewClient(client) => verdi.spawn_client(client)?,
                _ => {}
            }
        }

        // let state = verdi.state.clone();

        // tokio::spawn(async move {
        //     loop {
        //         state.render().unwrap();
        //     }
        // });

        anyhow::Ok(())
    })?;

    Ok(())
}

pub struct Verdi {
    // state: Arc<State<'s>>,
    // listener: UnixListener,
    events_receiver: UnboundedReceiverStream<Result<Event, Error>>,
    clients: JoinSet<Result<(), Error>>,
}

#[derive(Debug)]
pub enum Event {
    NewClient(Client),
    SessionPaused,
    SessionResumed,
    Input,
    // Input(colpetto::Event),
}

impl Verdi {
    pub async fn new(mut listener: Listener) -> AnyResult<Self> {
        let (tx, mut rx) = mpsc::unbounded_channel();

        debug!("Creating libinput instance");

        let mut libinput = Libinput::new(
            |path, flags| {
                rustix::fs::open(path, OFlags::from_bits_retain(flags as u32), Mode::empty())
                    .map(IntoRawFd::into_raw_fd)
                    .map_err(|err| err.raw_os_error().wrapping_neg())
            },
            |fd| drop(unsafe { OwnedFd::from_raw_fd(fd) }),
        )?;

        libinput.udev_assign_seat(c"seat0")?;

        debug!("Created libinputinstance");

        let mut event_stream = libinput.event_stream()?;

        tokio::spawn({
            let tx = tx.clone();

            async move {
                while let Some(stream) = event_stream.try_next().await.unwrap() {
                    tx.send(Ok(Event::Input)).unwrap();
                }
            }
        });

        // FIXME: handle errors instead of unwraping
        tokio::spawn(async move {
            while let Some(stream) = listener.try_next().await.unwrap() {
                let mut client = Client::new(stream).unwrap();

                client.insert(Display::default().into_object(ObjectId::DISPLAY));

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
