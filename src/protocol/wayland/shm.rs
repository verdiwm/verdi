use async_trait::async_trait;
use rustix::fd::OwnedFd;

use crate::{
    protocol::wayland::shm_pool::{ShmPool, WlShmPool},
    wire::{Message, ObjectId},
    Client, Dispatcher, Result,
};

pub use crate::protocol::interfaces::wayland::wl_shm::*;

#[derive(Debug)]
pub struct Shm {
    id: ObjectId,
}

impl Shm {
    pub fn new(id: ObjectId) -> Self {
        Self { id }
    }

    pub async fn advertise_formats(&self, client: &mut Client) -> Result<()> {
        self.format(client, Format::Argb8888).await?;
        self.format(client, Format::Xrgb8888).await?;

        Ok(())
    }
}

impl WlShm for Shm {
    async fn create_pool(
        &self,
        client: &mut Client,
        id: ObjectId,
        fd: OwnedFd,
        size: i32,
    ) -> Result<()> {
        // let shm_pool = ShmPool::new(client, id, fd, size)?;
        let shm_pool = ShmPool::new(id, fd, size)?;

        client.insert(id, shm_pool.into_dispatcher());

        Ok(())
    }

    async fn release(&self, _client: &mut Client) -> Result<()> {
        todo!()
    }
}

#[async_trait]
impl Dispatcher for Shm {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        self.handle_request(client, message).await
    }
}
