use async_trait::async_trait;

use crate::{
    wire::{Message, ObjectId},
    Client, Dispatcher, Result,
};

pub use crate::protocol::interfaces::wayland::wl_compositor::*;

#[derive(Debug)]
pub struct Compositor {
    id: ObjectId,
}

impl WlCompositor for Compositor {
    fn new(id: ObjectId) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self { id })
    }

    fn get_id(&self) -> crate::wire::ObjectId {
        self.id
    }

    async fn r#create_surface(&self, _client: &mut Client, _id: ObjectId) -> Result<()> {
        todo!()
    }

    async fn r#create_region(&self, _client: &mut Client, _id: ObjectId) -> Result<()> {
        todo!()
    }
}

#[async_trait]
impl Dispatcher for Compositor {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        self.handle_request(client, message).await
    }
}
