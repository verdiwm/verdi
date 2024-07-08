use async_trait::async_trait;

use crate::{
    wire::{Message, ObjectId},
    Client, Dispatcher, Result,
};

pub use crate::protocol::interfaces::wayland::wl_callback::*;

#[derive(Debug)]
pub struct Callback {
    id: ObjectId,
}

impl Callback {
    pub fn new(id: ObjectId) -> Self {
        Self { id }
    }
}

impl WlCallback for Callback {
    fn get_id(&self) -> ObjectId {
        self.id
    }
}

#[async_trait]
impl Dispatcher for Callback {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        self.handle_request(client, message).await
    }
}
