use stagecraft::{
    Actor, Bus, Context, HasMailbox, Responder, SuspendRefused, Suspendable,
};
use tracing::debug;

use crate::actors::{output_manager::OutputEvent, session::DeviceBroker};

use self::wgpu_context::WgpuContext;

mod wgpu_context;

#[stagecraft::message(Renderer)]
pub enum RendererMessage {
    /// Suspend coordination: drop the WGPU context and acknowledge.
    #[call(Result<(), SuspendRefused>)]
    PrepareSuspend,
    /// Suspend coordination: the session is back. The Renderer rebuilds from the
    /// OutputManager's rescan (an `OutputEvent::Added`), so this is a no-op.
    Resume,
    /// An output event from the output bus (Added/Removed).
    Output(OutputEvent),
    Render,
}

impl Suspendable for Renderer {
    fn prepare_suspend(responder: Responder<Result<(), SuspendRefused>>) -> Self::Message {
        RendererMessage::PrepareSuspend {
            respond_to: responder,
        }
    }

    fn resume() -> Self::Message {
        RendererMessage::Resume
    }
}

pub struct RendererInit {
    pub device_broker: DeviceBroker,
    pub output_bus: Bus<OutputEvent>,
}

pub struct Renderer {
    device_broker: DeviceBroker,
    wgpu_context: Option<WgpuContext<'static>>,
}

impl HasMailbox for Renderer {
    type Message = RendererMessage;
}

impl Actor for Renderer {
    type Init = RendererInit;

    async fn init(init: RendererInit, ctx: &mut Context<Self>) -> Self {
        ctx.forward_bus(&init.output_bus, |event| {
            Some(RendererMessage::Output(event))
        });

        Self {
            device_broker: init.device_broker,
            wgpu_context: None,
        }
    }

    async fn handle_message(&mut self, msg: RendererMessage, ctx: &mut Context<Self>) {
        match msg {
            RendererMessage::PrepareSuspend { respond_to } => {
                debug!("Renderer: preparing to suspend, dropping WGPU context");
                self.wgpu_context = None;
                respond_to.respond(Ok(()));
            }
            RendererMessage::Resume => {}
            RendererMessage::Output(OutputEvent::Added(info)) => {
                debug!("Renderer: output added, creating WGPU context");
                if self.wgpu_context.is_none() {
                    match WgpuContext::new(&self.device_broker, &info).await {
                        Ok(wgpu_ctx) => {
                            self.wgpu_context = Some(wgpu_ctx);
                            let _ = ctx.handle().cast(RendererMessage::Render).await;
                        }
                        Err(e) => tracing::error!("Failed to create WGPU context: {e}"),
                    }
                }
            }
            RendererMessage::Output(OutputEvent::Removed) => {
                debug!("Renderer: output removed, dropping WGPU context");
                self.wgpu_context = None;
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
        debug!("Renderer stopping, dropping WGPU context");
        self.wgpu_context = None;
    }
}
