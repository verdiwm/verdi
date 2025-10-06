use std::os::fd::OwnedFd;

use waynest::ObjectId;
use waynest_server::{Client as _, RequestDispatcher};

use crate::{
    Client, Result, VerdiError,
    protocol::wayland::shm_pool::{ShmPool, WlShmPool},
};

pub use waynest_protocols::server::core::wayland::wl_shm::*;

#[derive(Debug, RequestDispatcher, Default)]
#[waynest(error = VerdiError, connection = Client)]
pub struct Shm;

impl Shm {
    pub async fn advertise_formats(
        &self,
        client: &mut <Self as WlShm>::Connection,
        sender_id: ObjectId,
    ) -> Result<()> {
        self.format(client, sender_id, Format::Argb8888).await?;
        self.format(client, sender_id, Format::Xrgb8888).await?;

        Ok(())
    }
}

impl WlShm for Shm {
    type Connection = Client;

    async fn create_pool(
        &self,
        client: &mut Self::Connection,
        sender_id: ObjectId,
        id: ObjectId,
        fd: OwnedFd,
        size: i32,
    ) -> Result<()> {
        client.insert(id, ShmPool::new(fd, size)?);

        Ok(())
    }

    async fn release(&self, _client: &mut Self::Connection, _sender_id: ObjectId) -> Result<()> {
        todo!()
    }
}
