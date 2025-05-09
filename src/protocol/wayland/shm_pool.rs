use std::{io, ptr::null_mut};

use rustix::{
    fd::OwnedFd,
    mm::{MapFlags, MremapFlags, ProtFlags, mmap, mremap},
};
use tokio::sync::RwLock;

use crate::protocol::wayland::shm::Format;

use waynest::{
    server::{Client, Dispatcher, Result},
    wire::ObjectId,
};

pub use waynest::server::protocol::core::wayland::wl_shm_pool::*;

#[derive(Debug, Dispatcher)]
pub struct ShmPool {
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
    pub fn new(fd: OwnedFd, size: i32) -> Result<Self> {
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
            _fd: fd,
            map: RwLock::new(Map { size, mem }),
        })
    }
}

impl WlShmPool for ShmPool {
    async fn create_buffer(
        &self,
        _client: &mut Client,
        _sender_id: ObjectId,
        _id: ObjectId,
        _offset: i32,
        _width: i32,
        _height: i32,
        _stride: i32,
        _format: Format,
    ) -> Result<()> {
        todo!()
    }

    async fn destroy(&self, _client: &mut Client, _sender_id: ObjectId) -> Result<()> {
        todo!()
    }

    async fn resize(&self, _client: &mut Client, _sender_id: ObjectId, size: i32) -> Result<()> {
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
