use rustix::fd::OwnedFd;

use crate::protocol::wayland::shm_pool::{ShmPool, WlShmPool};

use waynest::{
    server::{Client, Dispatcher, Result},
    wire::ObjectId,
};

pub use waynest::server::protocol::core::wayland::wl_shm::*;

#[derive(Debug, Dispatcher, Default)]
pub struct Shm;

impl Shm {
    pub async fn advertise_formats(&self, client: &mut Client, sender_id: ObjectId) -> Result<()> {
        self.format(client, sender_id, Format::Argb8888).await?;
        self.format(client, sender_id, Format::Xrgb8888).await?;

        Ok(())
    }
}

impl WlShm for Shm {
    async fn create_pool(
        &self,
        client: &mut Client,
        sender_id: ObjectId,
        id: ObjectId,
        fd: OwnedFd,
        size: i32,
    ) -> Result<()> {
        client.insert(id, ShmPool::new(fd, size)?);

        Ok(())
    }

    async fn release(&self, _client: &mut Client, _sender_id: ObjectId) -> Result<()> {
        todo!()
    }
}
