use std::{
    // ffi::CString,
    fs,
    path::{Path, PathBuf},
    process::exit,
};

use anyhow::{bail, Context, Result as AnyResult};
use clap::Parser;
// use colpetto::Libinput;
use futures_util::TryStreamExt;
use reconciler::EventListener;
use rustix::process::geteuid;
use serde::{Deserialize, Serialize};
use tokio::{net::UnixListener, task::JoinSet};
use tracing::error;

use verdi::{
    error::Error,
    protocol::wayland::display::{Display, WlDisplay},
};

use waynest::{server::Client, wire::ObjectId};

mod context;
mod state;

// use context::WgpuContext;
// use state::State;

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
    
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR")?;
    dbg!(runtime_dir);

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
        register_ctrl_c_handler();

        let mut verdi = Verdi::new(SOCKET_PATH).await?;

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
    pub async fn new<P: AsRef<Path>>(path: P) -> AnyResult<Self> {
        let mut event_listener = EventListener::new();

        let listener = UnixListener::bind(path)?;

        let client_loop = async_stream::stream! {
            loop {
                match listener.accept().await {
                    Ok((stream, _addr)) => {
                        // FIXME: handle errors instead of unwraping
                        let mut client = Client::new(stream).unwrap();

                        client.insert(Display::new().into_object(ObjectId::DISPLAY));

                        yield Ok(Event::NewClient(client))
                    }
                    Err(_err) => {error!("Client failed to connect")}
                }
            }
        };

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
