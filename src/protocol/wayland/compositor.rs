use async_trait::async_trait;

use crate::{
    protocol::wayland::surface::{Surface, WlSurface},
    wire::{Message, ObjectId},
    Client, Dispatcher, Object, Result,
};

pub use crate::protocol::interfaces::wayland::wl_compositor::*;

#[derive(Debug)]
pub struct Compositor;

impl Compositor {
    pub fn new() -> Self {
        Self
    }
}

impl WlCompositor for Compositor {
    async fn create_surface(
        &self,
        _object: &Object,
        client: &mut Client,
        id: ObjectId,
    ) -> Result<()> {
        client.insert(Surface::new().into_object(id));

        Ok(())
    }

    async fn create_region(
        &self,
        _object: &Object,
        _client: &mut Client,
        _id: ObjectId,
    ) -> Result<()> {
        todo!()
    }
}

#[async_trait]
impl Dispatcher for Compositor {
    async fn dispatch(
        &self,
        object: &Object,
        client: &mut Client,
        message: &mut Message,
    ) -> Result<()> {
        self.handle_request(object, client, message).await
    }
}
