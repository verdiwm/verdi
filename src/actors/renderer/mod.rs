use tokio::sync::{mpsc, oneshot};
use tokio_util::sync::CancellationToken;
use tracing::debug;

use crate::actors::{renderer::wgpu_context::WgpuContext, session::SessionHandle};

mod wgpu_context;

pub enum RendererMessage {
    Suspend { respond_to: oneshot::Sender<()> },
    Resume { respond_to: oneshot::Sender<()> },
}

pub struct Renderer<'s> {
    sender: mpsc::Sender<RendererMessage>,
    receiver: mpsc::Receiver<RendererMessage>,
    session_handle: SessionHandle,
    shutdown_token: CancellationToken,
    wgpu_context: Option<WgpuContext<'s>>,
}

pub struct RendererHandle {
    sender: mpsc::Sender<RendererMessage>,
}

impl Renderer<'_> {
    pub fn new(session_handle: SessionHandle, shutdown_token: CancellationToken) -> Self {
        let (sender, receiver) = mpsc::channel(128);

        Self {
            sender,
            receiver,
            session_handle,
            shutdown_token,
            wgpu_context: None,
        }
    }

    pub fn handle(&self) -> RendererHandle {
        RendererHandle::new(self.sender.clone())
    }

    pub async fn run(mut self) {
        loop {
            tokio::select! {
                biased;
                _ = self.shutdown_token.cancelled() => {
                    break
                }
                Some(msg) = self.receiver.recv() => {
                    match msg {
                        RendererMessage::Suspend { respond_to }=> {
                            debug!("Suspending renderer");
                            self.wgpu_context = None;

                            let _ = respond_to.send(());

                        }
                        RendererMessage::Resume {respond_to} => {
                            debug!("Resuming renderer");

                            if self.wgpu_context.is_none() {
                                debug!("Creating wgpu context");
                                self.wgpu_context =Some( WgpuContext::new(&self.session_handle).await.unwrap());
                            }

                            let _ = respond_to.send(());
                        }
                    }
                }
            }

            if let Some(ref context) = self.wgpu_context {
                let _ = context.present();
                continue;
            }

            tokio::task::yield_now().await;
        }
    }
}

impl RendererHandle {
    fn new(sender: mpsc::Sender<RendererMessage>) -> Self {
        Self { sender }
    }

    pub async fn suspend(&self) {
        let (send, recv) = oneshot::channel();

        let _ = self
            .sender
            .send(RendererMessage::Suspend { respond_to: send })
            .await;

        let _ = recv.await;
    }

    pub async fn resume(&self) {
        let (send, recv) = oneshot::channel();

        let _ = self
            .sender
            .send(RendererMessage::Resume { respond_to: send })
            .await;

        let _ = recv.await;
    }
}
