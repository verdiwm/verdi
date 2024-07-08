use async_trait::async_trait;

use crate::{
    protocol::wayland::surface::{Surface, WlSurface},
    wire::{Message, ObjectId},
    Client, Dispatcher, Result,
};

pub use crate::protocol::interfaces::wayland::wl_compositor::*;

#[derive(Debug)]
pub struct Compositor {
    id: ObjectId,
}

impl Compositor {
    pub fn new(id: ObjectId) -> Self {
        Self { id }
    }
}

impl WlCompositor for Compositor {
    fn get_id(&self) -> ObjectId {
        self.id
    }

    async fn create_surface(&self, client: &mut Client, id: ObjectId) -> Result<()> {
        client.insert(id, Surface::new(id).into_dispatcher());

        Ok(())
    }

    async fn create_region(&self, _client: &mut Client, _id: ObjectId) -> Result<()> {
        todo!()
    }
}

#[async_trait]
impl Dispatcher for Compositor {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        self.handle_request(client, message).await
    }
}
