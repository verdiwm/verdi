#![allow(unused)]
#![allow(clippy::default_constructed_unit_structs)]

use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::{Context, Result as AnyResult, bail};
use clap::Parser;
use rustix::process::geteuid;
use tokio_util::sync::CancellationToken;
use tracing::debug;

use tracing_subscriber::EnvFilter;
use verdi::{CompositorInit, Config};

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
        .with_level(true)
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(false)
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

    debug!("Created tokio runtime");

    runtime.block_on(async move {
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(5)).await;

            std::process::exit(1);
        });

        let socket_path = args.socket.or(config.socket);

        let token = CancellationToken::new();

        stagecraft::spawn::<verdi::Compositor>(token.clone(), CompositorInit { socket_path });

        token.cancelled().await;
    });

    Ok(())
}
