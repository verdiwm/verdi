use rustix::fd::OwnedFd;

use crate::protocol::wayland::shm_pool::{ShmPool, WlShmPool};

use waynest::{
    server::{Client, Dispatcher, Object, Result},
    wire::ObjectId,
};

pub use waynest::server::protocol::core::wayland::wl_shm::*;

#[derive(Debug, Dispatcher, Default)]
pub struct Shm;

impl Shm {
    pub async fn advertise_formats(&self, object: &Object, client: &mut Client) -> Result<()> {
        self.format(object, client, Format::Argb8888).await?;
        self.format(object, client, Format::Xrgb8888).await?;

        Ok(())
    }
}

impl WlShm for Shm {
    async fn create_pool(
        &self,
        _object: &Object,
        client: &mut Client,
        id: ObjectId,
        fd: OwnedFd,
        size: i32,
    ) -> Result<()> {
        client.insert(ShmPool::new(fd, size)?.into_object(id));

        Ok(())
    }

    async fn release(&self, _object: &Object, _client: &mut Client) -> Result<()> {
        todo!()
    }
}
