// use std::{io, ptr::null_mut};

use async_trait::async_trait;
// use rustix::{
//     mm::{mmap, MapFlags, ProtFlags},
// };

use crate::{
    protocol::wayland::shm::Format,
    wire::{Message, ObjectId},
    Client, Dispatcher, Result,
};

pub use crate::protocol::interfaces::wayland::wl_shm_pool::*;

#[derive(Debug)]
pub struct ShmPool;

impl WlShmPool for ShmPool {
    fn new(_id: ObjectId) -> Result<Self>
    where
        Self: Sized,
    {
        // let map = unsafe {
        //     mmap(
        //         null_mut(),
        //         size as usize,
        //         ProtFlags::READ | ProtFlags::WRITE,
        //         MapFlags::SHARED,
        //         fd,
        //         0,
        //     )
        //     .map_err(io::Error::from)?
        // };

        todo!();
    }

    fn get_id(&self) -> ObjectId {
        todo!()
    }

    async fn r#create_buffer(
        &self,
        _client: &mut crate::Client,
        _id: crate::wire::ObjectId,
        _offset: i32,
        _width: i32,
        _height: i32,
        _stride: i32,
        _format: Format,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#destroy(&self, _client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }

    async fn r#resize(&self, _client: &mut crate::Client, _size: i32) -> crate::Result<()> {
        todo!()
    }
}

#[async_trait]
impl Dispatcher for ShmPool {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        self.handle_request(client, message).await
    }
}
