use std::{io, os::fd::RawFd, ptr::null_mut, sync::Arc};

use async_trait::async_trait;
use rustix::{
    fd::OwnedFd,
    mm::{mmap, MapFlags, ProtFlags},
};

use crate::{
    wire::{Message, ObjectId},
    Client, Dispatcher, Result,
};

pub use crate::protocol::interfaces::wayland::wl_shm_pool::*;

#[derive(Debug)]
pub struct ShmPool;

impl ShmPool {
    pub async fn new(client: &mut Client, id: ObjectId, fd: OwnedFd, size: i32) -> Result<Self> {
        // client.insert(id, Self::create_dispatcher());

        let map = unsafe {
            mmap(
                null_mut(),
                size as usize,
                ProtFlags::READ | ProtFlags::WRITE,
                MapFlags::SHARED,
                fd,
                0,
            )
            .map_err(io::Error::from)?
        };

        todo!();

        // Ok(())
    }
}

impl WlShmPool for ShmPool {
    async fn r#create_buffer(
        client: &mut crate::Client,
        r#id: crate::wire::ObjectId,
        r#offset: i32,
        r#width: i32,
        r#height: i32,
        r#stride: i32,
        r#format: crate::protocol::interfaces::wayland::wl_shm::Format,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#destroy(client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }

    async fn r#resize(client: &mut crate::Client, r#size: i32) -> crate::Result<()> {
        todo!()
    }
}

#[async_trait]
impl Dispatcher for ShmPool {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        <Self as WlShmPool>::handle_request(client, message).await
    }
}
