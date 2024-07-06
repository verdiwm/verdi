use anyhow::{Context, Result};
use std::{fs, process::exit};
use tokio::task::JoinSet;
use tracing::{error, info, warn};
use verdi::Verdi;

// const SERVER_ID_START: usize = 0xff000000;

// const WIDTH: usize = 1280;
// const HEIGHT: usize = 720;

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

    runtime.block_on(async move {
        register_ctrl_c_handler();

        let verdi = Verdi::new(SOCKET_PATH)?;

        let mut set: JoinSet<Result<(), anyhow::Error>> = JoinSet::new();

        while let Some(client) = verdi.new_client().await {
            info!("Got client");

            match client {
                Ok(mut client) => {
                    set.spawn(async move {
                        while let Some(mut message) = client.next_message().await? {
                            client.handle_message(&mut message);
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
