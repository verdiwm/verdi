use waynest::{
    server::{Client, Dispatcher, Object, Result},
    wire::ObjectId,
};

pub use waynest::server::protocol::wayland::wl_surface::*;

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
    async fn destroy(&self, _object: &Object, _client: &mut Client) -> Result<()> {
        todo!()
    }

    async fn attach(
        &self,
        _object: &Object,
        _client: &mut Client,
        _buffer: Option<ObjectId>,
        _x: i32,
        _y: i32,
    ) -> Result<()> {
        todo!()
    }

    async fn damage(
        &self,
        _object: &Object,
        _client: &mut Client,
        _x: i32,
        _y: i32,
        _width: i32,
        _height: i32,
    ) -> Result<()> {
        todo!()
    }

    async fn frame(
        &self,
        _object: &Object,
        _client: &mut Client,
        _callback: ObjectId,
    ) -> Result<()> {
        todo!()
    }

    async fn set_opaque_region(
        &self,
        _object: &Object,
        _client: &mut Client,
        _region: Option<ObjectId>,
    ) -> Result<()> {
        todo!()
    }

    async fn set_input_region(
        &self,
        _object: &Object,
        _client: &mut Client,
        _region: Option<ObjectId>,
    ) -> Result<()> {
        todo!()
    }

    async fn commit(&self, _object: &Object, _client: &mut Client) -> Result<()> {
        // FIXME: commit state

        Ok(())
    }

    async fn set_buffer_transform(
        &self,
        _object: &Object,
        _client: &mut Client,
        _transform: waynest::server::protocol::wayland::wl_output::Transform,
    ) -> Result<()> {
        todo!()
    }

    async fn set_buffer_scale(
        &self,
        _object: &Object,
        _client: &mut Client,
        _scale: i32,
    ) -> Result<()> {
        todo!()
    }

    async fn damage_buffer(
        &self,
        _object: &Object,
        _client: &mut Client,
        _x: i32,
        _y: i32,
        _width: i32,
        _height: i32,
    ) -> Result<()> {
        todo!()
    }

    async fn offset(&self, _object: &Object, _client: &mut Client, _x: i32, _y: i32) -> Result<()> {
        todo!()
    }
}
