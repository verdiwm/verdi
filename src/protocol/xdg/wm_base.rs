use std::sync::Arc;

use async_trait::async_trait;

use crate::{wire::Message, Client, Dispatcher, Result};

pub use crate::protocol::interfaces::xdg_shell::xdg_wm_base::*;

#[derive(Debug)]
pub struct WmBase;

impl WmBase {
    pub fn new() -> Arc<Box<dyn Dispatcher + Send + Sync>> {
        Arc::new(Box::new(Self {}))
    }
}

impl XdgWmBase for WmBase {
    fn new(id: crate::wire::ObjectId) -> crate::Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn get_id(&self) -> crate::wire::ObjectId {
        todo!()
    }

    async fn r#destroy(&self, client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }

    async fn r#create_positioner(
        &self,
        client: &mut crate::Client,
        r#id: crate::wire::ObjectId,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#get_xdg_surface(
        &self,
        client: &mut crate::Client,
        r#id: crate::wire::ObjectId,
        r#surface: crate::wire::ObjectId,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#pong(&self, client: &mut crate::Client, r#serial: u32) -> crate::Result<()> {
        todo!()
    }
}

#[async_trait]
impl Dispatcher for WmBase {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        self.handle_request(client, message).await
    }
}
