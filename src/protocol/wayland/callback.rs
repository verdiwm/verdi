use std::sync::Arc;

use async_trait::async_trait;

use crate::{wire::Message, Client, Dispatcher, Result};

pub use crate::protocol::interfaces::wayland::wl_callback::*;

#[derive(Debug)]
pub struct Callback;

impl WlCallback for Callback {
    fn create_dispatcher() -> Arc<Box<dyn Dispatcher + Send + Sync>> {
        Arc::new(Box::new(Self {}))
    }
}

#[async_trait]
impl Dispatcher for Callback {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        <Self as WlCallback>::handle_request(client, message).await
    }
}
