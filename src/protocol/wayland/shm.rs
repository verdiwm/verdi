use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    wire::{Message, ObjectId},
    Client, Dispatcher, Result,
};

pub use crate::protocol::interfaces::wayland::wl_shm::*;

#[derive(Debug)]
pub struct Shm;

impl WlShm for Shm {
    async fn r#create_pool(
        _client: &mut Client,
        _id: ObjectId,
        _fd: std::os::unix::prelude::RawFd,
        _size: i32,
    ) -> Result<()> {
        todo!()
    }

    async fn r#release(_client: &mut Client) -> Result<()> {
        todo!()
    }

    fn create_dispatcher() -> Arc<Box<dyn Dispatcher + Send + Sync>> {
        Arc::new(Box::new(Self {}))
    }
}

#[async_trait]
impl Dispatcher for Shm {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        <Self as WlShm>::handle_request(client, message).await
    }
}
