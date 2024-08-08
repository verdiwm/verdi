use crate::{Dispatcher, Object, Result};

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

#[derive(Debug, Dispatcher)]
pub struct Surface {
    state: DoubleBuffer,
}

impl Surface {
    pub fn new() -> Self {
        Self {
            state: DoubleBuffer::new(),
        }
    }
}

impl WlSurface for Surface {
    async fn destroy(&self, _object: &Object, _client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }

    async fn attach(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
        _buffer: Option<waynest::wire::ObjectId>,
        _x: i32,
        _y: i32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn damage(
        &self,
        _object: &Object,
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
        _object: &Object,
        _client: &mut crate::Client,
        _callback: waynest::wire::ObjectId,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn set_opaque_region(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
        _region: Option<waynest::wire::ObjectId>,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn set_input_region(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
        _region: Option<waynest::wire::ObjectId>,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn commit(&self, _object: &Object, _client: &mut crate::Client) -> crate::Result<()> {
        // FIXME: commit state

        Ok(())
    }

    async fn set_buffer_transform(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
        _transform: crate::protocol::interfaces::wayland::wl_output::Transform,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn set_buffer_scale(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
        _scale: i32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn damage_buffer(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
        _x: i32,
        _y: i32,
        _width: i32,
        _height: i32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn offset(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
        _x: i32,
        _y: i32,
    ) -> crate::Result<()> {
        todo!()
    }
}
