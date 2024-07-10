use async_trait::async_trait;

use crate::{wire::Message, Client, Dispatcher, Object, Result};

pub use crate::protocol::interfaces::wayland::wl_callback::*;

#[derive(Debug)]
pub struct Callback;

impl Callback {
    pub fn new() -> Self {
        Self
    }
}

impl WlCallback for Callback {}

#[async_trait]
impl Dispatcher for Callback {
    async fn dispatch(
        &self,
        object: &Object,
        client: &mut Client,
        message: &mut Message,
    ) -> Result<()> {
        self.handle_request(object, client, message).await
    }
}
