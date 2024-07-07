use std::sync::Arc;

use async_trait::async_trait;

use crate::{wire::Message, Client, Dispatcher, Result};

pub use crate::protocol::interfaces::xdg_shell::xdg_wm_base::*;

#[derive(Debug)]
pub struct WmBase;

impl XdgWmBase for WmBase {
    fn create_dispatcher() -> Arc<Box<dyn Dispatcher + Send + Sync>> {
        Arc::new(Box::new(Self {}))
    }

    async fn r#destroy(client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }

    async fn r#create_positioner(
        client: &mut crate::Client,
        r#id: crate::wire::ObjectId,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#get_xdg_surface(
        client: &mut crate::Client,
        r#id: crate::wire::ObjectId,
        r#surface: crate::wire::ObjectId,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#pong(client: &mut crate::Client, r#serial: u32) -> crate::Result<()> {
        todo!()
    }
}

#[async_trait]
impl Dispatcher for WmBase {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        <Self as XdgWmBase>::handle_request(client, message).await
    }
}
