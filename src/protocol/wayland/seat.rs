use async_trait::async_trait;

use crate::{wire::Message, Client, Dispatcher, Object, Result};

pub use crate::protocol::interfaces::wayland::wl_seat::*;

#[derive(Debug)]
pub struct Seat;

impl Seat {
    pub fn new() -> Self {
        Self
    }
}

impl WlSeat for Seat {
    async fn get_pointer(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
        _id: crate::wire::ObjectId,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn get_keyboard(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
        _id: crate::wire::ObjectId,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn get_touch(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
        _id: crate::wire::ObjectId,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn release(&self, _object: &Object, _client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }
}

#[async_trait]
impl Dispatcher for Seat {
    async fn dispatch(
        &self,
        object: &Object,
        client: &mut Client,
        message: &mut Message,
    ) -> Result<()> {
        self.handle_request(object, client, message).await
    }
}
