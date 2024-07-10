use async_trait::async_trait;

use crate::{
    wire::{Message, ObjectId},
    Client, Dispatcher, Result,
};

pub use crate::protocol::interfaces::wayland::wl_surface::*;

#[derive(Debug)]
struct State {}

impl State {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug)]
struct DoubleBuffer {
    current: State,
    pending: State,
}

impl DoubleBuffer {
    pub fn new() -> Self {
        Self {
            current: State::new(),
            pending: State::new(),
        }
    }
}

#[derive(Debug)]
pub struct Surface {
    id: ObjectId,
    state: DoubleBuffer,
}

impl Surface {
    pub fn new(id: ObjectId) -> Self {
        Self {
            id,
            state: DoubleBuffer::new(),
        }
    }
}

impl WlSurface for Surface {
    async fn destroy(&self, _client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }

    async fn attach(
        &self,
        _client: &mut crate::Client,
        _buffer: Option<crate::wire::ObjectId>,
        _x: i32,
        _y: i32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn damage(
        &self,
        _client: &mut crate::Client,
        _x: i32,
        _y: i32,
        _width: i32,
        _height: i32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn frame(
        &self,
        _client: &mut crate::Client,
        _callback: crate::wire::ObjectId,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn set_opaque_region(
        &self,
        _client: &mut crate::Client,
        _region: Option<crate::wire::ObjectId>,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn set_input_region(
        &self,
        _client: &mut crate::Client,
        _region: Option<crate::wire::ObjectId>,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn commit(&self, _client: &mut crate::Client) -> crate::Result<()> {
        // FIXME: commit state

        Ok(())
    }

    async fn set_buffer_transform(
        &self,
        _client: &mut crate::Client,
        _transform: crate::protocol::interfaces::wayland::wl_output::Transform,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn set_buffer_scale(
        &self,
        _client: &mut crate::Client,
        _scale: i32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn damage_buffer(
        &self,
        _client: &mut crate::Client,
        _x: i32,
        _y: i32,
        _width: i32,
        _height: i32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn offset(&self, _client: &mut crate::Client, _x: i32, _y: i32) -> crate::Result<()> {
        todo!()
    }
}

#[async_trait]
impl Dispatcher for Surface {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        self.handle_request(client, message).await
    }
}
