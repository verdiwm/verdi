#![allow(unused)]
#![allow(clippy::default_constructed_unit_structs)]

use std::{
    // ffi::CString,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result as AnyResult};
use clap::Parser;
// use colpetto::Libinput;
use futures_util::{StreamExt, TryStreamExt};
use reconciler::EventListener;
use rustix::process::geteuid;
use serde::{Deserialize, Serialize};
use tokio::{net::UnixListener, task::JoinSet};
use tracing::error;

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

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
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
    tracing_subscriber::fmt::init();

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

    runtime.block_on(async move {
        let listener = {
            if let Some(socket) = args.socket {
                Listener::new_with_path(socket)
            } else {
                Listener::new()
            }
        }?;

        let mut verdi = Verdi::new(listener).await?;

        while let Some(event) = verdi.next_event().await? {
            dbg!(&event);
            match event {
                Event::NewClient(client) => verdi.spawn_client(client)?,
                Event::Input(colpetto::Event::Keyboard(_)) => {
                    break;
                }
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
    event_listener: EventListener<Result<Event, Error>>,
    clients: JoinSet<Result<(), Error>>,
}

#[derive(Debug)]
pub enum Event {
    NewClient(Client),
    SessionPaused,
    SessionResumed,
    Input(colpetto::Event),
}

impl Verdi {
    pub async fn new(listener: Listener) -> AnyResult<Self> {
        let mut event_listener = EventListener::new();

        let client_loop = listener.map(|stream| {
            match stream {
                Ok(stream) => {
                    // FIXME: handle errors instead of unwraping
                    let mut client = Client::new(stream).unwrap();

                    client.insert(Display::default().into_object(ObjectId::DISPLAY));

                    Ok(Event::NewClient(client))
                }
                Err(err) => {
                    error!("Client failed to connect");
                    Err(Error::Protocol(waynest::server::Error::IoError(err)))
                }
            }
        });

        event_listener.add_listener(client_loop);

        // let libinput = Libinput::new()?;
        // libinput.assign_seat(CString::new("seat0").unwrap().as_c_str())?;

        // event_listener.add_listener(libinput.map_err(Error::Input).map_ok(Event::Input));

        Ok(Self {
            // state: Arc::new(State::new().await?),
            event_listener,
            clients: JoinSet::new(),
        })
    }

    pub async fn next_event(&mut self) -> Result<Option<Event>, Error> {
        self.event_listener.try_next().await
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
