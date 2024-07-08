use async_trait::async_trait;

use crate::{
    wire::{Message, ObjectId},
    Client, Dispatcher, Result,
};

pub use crate::protocol::interfaces::wayland::wl_seat::*;

#[derive(Debug)]
pub struct Seat {
    id: ObjectId,
}

impl Seat {
    pub fn new(id: ObjectId) -> Self {
        Self { id }
    }
}

impl WlSeat for Seat {
    fn get_id(&self) -> ObjectId {
        self.id
    }

    async fn get_pointer(
        &self,
        _client: &mut crate::Client,
        _id: crate::wire::ObjectId,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn get_keyboard(
        &self,
        _client: &mut crate::Client,
        _id: crate::wire::ObjectId,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn get_touch(
        &self,
        _client: &mut crate::Client,
        _id: crate::wire::ObjectId,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn release(&self, _client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }
}

#[async_trait]
impl Dispatcher for Seat {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        self.handle_request(client, message).await
    }
}
