use std::sync::{Arc, Weak};

use waynest::ObjectId;
use waynest_server::{Connection, RequestDispatcher};

use crate::{
    error::{Result, VerdiError},
    protocol::wayland::{shm::Format, shm_pool::ShmPool},
};

pub use waynest_protocols::server::core::wayland::wl_buffer::*;

#[derive(Debug, RequestDispatcher)]
#[waynest(error = VerdiError)]
pub struct Buffer {
    pool: Weak<ShmPool>,
    offset: usize,
    width: u32,
    height: u32,
    stride: u32,
    format: Format,
}

impl Buffer {
    pub fn new(
        pool: Weak<ShmPool>,
        offset: i32,
        width: i32,
        height: i32,
        stride: i32,
        format: Format,
    ) -> Self {
        Self {
            pool,
            offset: offset as usize,
            width: width as u32,
            height: height as u32,
            stride: stride as u32,
            format,
        }
    }

    pub fn get_pixel_data(&self) -> Result<Option<&[u8]>> {
        if let Some(pool) = self.pool.upgrade() {
            // Calculate buffer size based on height and stride
            let buffer_size = self.height as usize * self.stride as usize;

            // Get access to the mapped memory (this will need to be implemented)
            // For now, return None as placeholder
            Ok(None)
        } else {
            Ok(None)
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn stride(&self) -> u32 {
        self.stride
    }

    pub fn format(&self) -> Format {
        self.format
    }
}

impl WlBuffer for Buffer {
    type Connection = Connection<VerdiError>;

    async fn destroy(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
    ) -> Result<()> {
        // Buffer is destroyed when this object is dropped
        // In a full implementation, we might need to send release events
        // or notify surfaces that are using this buffer
        Ok(())
    }
}