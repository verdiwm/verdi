use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    wire::{Message, ObjectId},
    Client, Dispatcher, Result,
};

pub use crate::protocol::interfaces::wayland::wl_compositor::*;

#[derive(Debug)]
pub struct Compositor;

impl WlCompositor for Compositor {
    async fn r#create_surface(_client: &mut Client, _id: ObjectId) -> Result<()> {
        todo!()
    }

    async fn r#create_region(_client: &mut Client, _id: ObjectId) -> Result<()> {
        todo!()
    }

    fn create_dispatcher(_id: ObjectId) -> Arc<Box<dyn Dispatcher + Send + Sync>> {
        Arc::new(Box::new(Self {}))
    }
}

#[async_trait]
impl Dispatcher for Compositor {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        <Self as WlCompositor>::handle_request(client, message).await
    }
}
