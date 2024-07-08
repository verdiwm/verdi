use async_trait::async_trait;

use crate::{
    wire::{Message, ObjectId},
    Client, Dispatcher, Result,
};

pub use crate::protocol::interfaces::xdg_shell::xdg_wm_base::*;

#[derive(Debug)]
pub struct WmBase {
    id: ObjectId,
}

impl WmBase {
    pub fn new(id: ObjectId) -> Self {
        Self { id }
    }
}

impl XdgWmBase for WmBase {
    fn get_id(&self) -> ObjectId {
        self.id
    }

    async fn destroy(&self, _client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }

    async fn create_positioner(
        &self,
        _client: &mut crate::Client,
        _id: ObjectId,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn get_xdg_surface(
        &self,
        _client: &mut crate::Client,
        _id: ObjectId,
        _surface: ObjectId,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn pong(&self, _client: &mut crate::Client, _serial: u32) -> crate::Result<()> {
        todo!()
    }
}

#[async_trait]
impl Dispatcher for WmBase {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        self.handle_request(client, message).await
    }
}
