use std::sync::Arc;

use async_trait::async_trait;
use rustix::fd::OwnedFd;

use crate::{
    protocol::wayland::shm_pool::{ShmPool, WlShmPool},
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
    fn new(_id: crate::wire::ObjectId) -> crate::Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn get_id(&self) -> crate::wire::ObjectId {
        todo!()
    }

    async fn r#create_pool(
        &self,
        client: &mut Client,
        id: ObjectId,
        _fd: OwnedFd,
        _size: i32,
    ) -> Result<()> {
        // let shm_pool = ShmPool::new(client, id, fd, size)?;
        let shm_pool = ShmPool::new(id)?;

        client.insert(id, shm_pool.into_dispatcher());

        Ok(())
    }

    async fn r#release(&self, _client: &mut Client) -> Result<()> {
        todo!()
    }
}

#[async_trait]
impl Dispatcher for Shm {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        self.handle_request(client, message).await
    }
}
