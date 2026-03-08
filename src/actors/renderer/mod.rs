use stagecraft::{Actor, Context, HasMailbox};
use tracing::debug;

use crate::actors::session::SessionRef;

use self::wgpu_context::WgpuContext;

mod wgpu_context;

#[stagecraft::message(Renderer)]
pub enum RendererMessage {
    #[call]
    Suspend,
    #[call]
    Resume,
    Render,
}

pub struct Renderer {
    session_ref: SessionRef,
    wgpu_context: Option<WgpuContext<'static>>,
}

impl Renderer {
    pub fn new(session_ref: SessionRef) -> Self {
        Self {
            session_ref,
            wgpu_context: None,
        }
    }
}

impl HasMailbox for Renderer {
    type Message = RendererMessage;
}

impl Actor for Renderer {
    type Init = Self;

    async fn init(init: Self, _ctx: &mut Context<Self>) -> Self {
        init
    }

    async fn handle_message(&mut self, msg: RendererMessage, ctx: &mut Context<Self>) {
        match msg {
            RendererMessage::Suspend { respond_to } => {
                debug!("Suspending renderer");
                self.wgpu_context = None;
                let _ = respond_to.send(());
            }
            RendererMessage::Resume { respond_to } => {
                debug!("Resuming renderer");

                if self.wgpu_context.is_none() {
                    debug!("Creating wgpu context");
                    match WgpuContext::new(&self.session_ref).await {
                        Ok(wgpu_ctx) => self.wgpu_context = Some(wgpu_ctx),
                        Err(e) => tracing::error!("Failed to create wgpu context: {e}"),
                    }
                }

                let _ = respond_to.send(());

                let _ = ctx.handle().cast(RendererMessage::Render).await;
            }
            RendererMessage::Render => {
                if let Some(ref context) = self.wgpu_context {
                    if let Err(e) = context.present() {
                        tracing::error!("Present failed: {e}");
                    }
                    let _ = ctx.handle().cast(RendererMessage::Render).await;
                }
            }
        }
    }

    async fn on_stop(&mut self, _ctx: &mut Context<Self>) {
        debug!("Renderer stopping, dropping wgpu context");
        self.wgpu_context = None;
    }
}
