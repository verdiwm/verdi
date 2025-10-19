use std::path::Path;

use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use waynest_server::{Listener, ListenerError};

use crate::CompositorHandle;

pub struct ClientListener {
    listener: Listener,
    compositor_handle: CompositorHandle,
    shutdown_token: CancellationToken,
}

impl ClientListener {
    pub fn new<P: AsRef<Path>>(
        compositor_handle: CompositorHandle,
        shutdown_token: CancellationToken,
        path: Option<P>,
    ) -> Result<Self, ListenerError> {
        let listener = if let Some(ref socket) = path {
            Listener::new_with_path(socket)?
        } else {
            Listener::new()?
        };

        Ok(Self {
            listener,
            compositor_handle,
            shutdown_token,
        })
    }

    pub async fn run(mut self) {
        loop {
            tokio::select! {
                biased;
                _ = self.shutdown_token.cancelled() => {
                    break
                }
                Some(res) = self.listener.next() => {
                    match res {
                        Ok(stream) => self.compositor_handle.new_client(stream).await,
                        Err(_) => todo!(),
                    }
                }
            }
        }
    }
}
