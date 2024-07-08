use std::{os::fd::RawFd, sync::Arc};

use async_trait::async_trait;
use rustix::fd::OwnedFd;

use crate::{
    protocol::wayland::shm_pool::ShmPool,
    wire::{Message, ObjectId},
    Client, Dispatcher, Result,
};

pub use crate::protocol::interfaces::wayland::wl_shm::*;

#[derive(Debug)]
pub struct Shm;

impl Shm {
    pub fn new() -> Arc<Box<dyn Dispatcher + Send + Sync>> {
        Arc::new(Box::new(Self {}))
    }
}

impl WlShm for Shm {
    async fn r#create_pool(
        client: &mut Client,
        id: ObjectId,
        fd: OwnedFd,
        size: i32,
    ) -> Result<()> {
        // ShmPool::new(client, id, fd, size).await
        todo!()
    }

    async fn r#release(_client: &mut Client) -> Result<()> {
        todo!()
    }
}

#[async_trait]
impl Dispatcher for Shm {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        <Self as WlShm>::handle_request(client, message).await
    }
}
