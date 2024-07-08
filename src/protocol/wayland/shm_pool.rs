use std::{io, ptr::null_mut};

use async_trait::async_trait;
use rustix::fd::OwnedFd;
use rustix::mm::{mmap, mremap, MapFlags, MremapFlags, ProtFlags};
use tokio::sync::RwLock;

use crate::{
    protocol::wayland::shm::Format,
    wire::{Message, ObjectId},
    Client, Dispatcher, Result,
};

pub use crate::protocol::interfaces::wayland::wl_shm_pool::*;

#[derive(Debug)]
pub struct ShmPool {
    id: ObjectId,
    _fd: OwnedFd,
    map: RwLock<Map>,
}

// SAFETY: this is guaranteeed by the use of RwLock above
unsafe impl Send for Map {}
unsafe impl Sync for Map {}

#[derive(Debug)]
struct Map {
    size: usize,
    mem: *mut u8,
}

impl ShmPool {
    pub fn new(id: ObjectId, fd: OwnedFd, size: i32) -> Result<Self>
    where
        Self: Sized,
    {
        // FIXME: error handling when converting size
        let size = size as usize;
        let mem = unsafe {
            mmap(
                null_mut(),
                size,
                ProtFlags::READ | ProtFlags::WRITE,
                MapFlags::SHARED,
                &fd,
                0,
            )
            .map_err(io::Error::from)?
        }
        .cast();

        Ok(Self {
            id,
            _fd: fd,
            map: RwLock::new(Map { size, mem }),
        })
    }
}

impl WlShmPool for ShmPool {
    fn get_id(&self) -> ObjectId {
        self.id
    }

    async fn create_buffer(
        &self,
        _client: &mut crate::Client,
        _id: crate::wire::ObjectId,
        _offset: i32,
        _width: i32,
        _height: i32,
        _stride: i32,
        _format: Format,
    ) -> Result<()> {
        todo!()
    }

    async fn destroy(&self, _client: &mut crate::Client) -> Result<()> {
        todo!()
    }

    async fn resize(&self, _client: &mut crate::Client, size: i32) -> Result<()> {
        let mut write_guard = self.map.write().await;
        let old_size = write_guard.size;
        let new_size = size as usize;

        // FIXME: handle error when resize tries to reduce size

        // FIXME: better error handling
        let mem = unsafe {
            mremap(
                write_guard.mem.cast(),
                old_size,
                new_size,
                MremapFlags::MAYMOVE,
            )
            .unwrap()
        };

        write_guard.size = new_size;
        write_guard.mem = mem.cast();

        Ok(())
    }
}

#[async_trait]
impl Dispatcher for ShmPool {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        self.handle_request(client, message).await
    }
}
