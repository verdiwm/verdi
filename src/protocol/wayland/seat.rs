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
    
    async fn r#get_pointer(
        &self,
        client: &mut crate::Client,
        r#id: crate::wire::ObjectId,
    ) -> crate::Result<()> {
        todo!()
    }
    
    async fn r#get_keyboard(
        &self,
        client: &mut crate::Client,
        r#id: crate::wire::ObjectId,
    ) -> crate::Result<()> {
        todo!()
    }
    
    async fn r#get_touch(
        &self,
        client: &mut crate::Client,
        r#id: crate::wire::ObjectId,
    ) -> crate::Result<()> {
        todo!()
    }
    
    async fn r#release(&self, client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }
}

#[async_trait]
impl Dispatcher for Seat {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        self.handle_request(client, message).await
    }
}
