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
    Config,
    Verdi,
    error::VerdiError,
    // protocol::wayland::display::{Display, WlDisplay},
};

use waynest::ObjectId;
use waynest_server::Listener;

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

fn main() -> AnyResult<()> {
    let format = tracing_subscriber::fmt::format()
        .with_level(false)
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(false)
        .without_time()
        .compact();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_env("VERDI_LOG_LEVEL"))
        .event_format(format)
        .init();

    if geteuid().is_root() {
        bail!("Tried running as root")
    }

    let args = Args::parse();

    let config_path = if let Some(ref config) = args.config {
        config.clone() // FIXME: avoid this clone
    } else if let Ok(path) = std::env::var("XDG_CONFIG_HOME") {
        Path::new(&path).join("verdi/verdi.corn")
    } else if let Some(path) = home::home_dir() {
        path.join(".config/verdi/verdi.corn")
    } else {
        todo!()
    };

    let config: Config = corn::from_slice(&fs::read(config_path)?)?;

    // Create the tokio runtime manually instead of using a macro for better controll
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("Failed to create tokio runtime")?;

    debug!("Created runtime");

    runtime.block_on(async move {
        let listener = {
            if let Some(ref socket) = args.socket {
                Listener::new_with_path(socket)
            } else if let Some(ref socket) = config.socket {
                Listener::new_with_path(socket)
            } else {
                Listener::new()
            }
        }?;

        let socket_path = listener.socket_path().canonicalize()?;

        debug!("Started listener at path {}", socket_path.display());

        let mut verdi = Verdi::new(listener, config).await?;

        debug!("Started new verdi instance");

        verdi.run().await?;

        anyhow::Ok(())
    })?;

    Ok(())
}
