use waynest::{
    server::{Client, Dispatcher, Result},
    wire::ObjectId,
};

pub use waynest::server::protocol::core::wayland::wl_surface::*;

#[derive(Debug, Default)]
struct State {}

#[derive(Debug, Default)]
struct DoubleBuffer {
    current: State,
    pending: State,
}

#[derive(Debug, Dispatcher, Default)]
pub struct Surface {
    state: DoubleBuffer,
}

impl WlSurface for Surface {
    async fn destroy(&self, _client: &mut Client, _sender_id: ObjectId) -> Result<()> {
        todo!()
    }

    async fn attach(
        &self,
        _client: &mut Client,
        _sender_id: ObjectId,
        _buffer: Option<ObjectId>,
        _x: i32,
        _y: i32,
    ) -> Result<()> {
        todo!()
    }

    async fn damage(
        &self,
        _client: &mut Client,
        _sender_id: ObjectId,
        _x: i32,
        _y: i32,
        _width: i32,
        _height: i32,
    ) -> Result<()> {
        todo!()
    }

    async fn frame(
        &self,
        _client: &mut Client,
        _sender_id: ObjectId,
        _callback: ObjectId,
    ) -> Result<()> {
        todo!()
    }

    async fn set_opaque_region(
        &self,
        _client: &mut Client,
        _sender_id: ObjectId,
        _region: Option<ObjectId>,
    ) -> Result<()> {
        todo!()
    }

    async fn set_input_region(
        &self,
        _client: &mut Client,
        _sender_id: ObjectId,
        _region: Option<ObjectId>,
    ) -> Result<()> {
        todo!()
    }

    async fn commit(&self, _client: &mut Client, _sender_id: ObjectId) -> Result<()> {
        // FIXME: commit state

        Ok(())
    }

    async fn set_buffer_transform(
        &self,
        _client: &mut Client,
        _sender_id: ObjectId,
        _transform: waynest::server::protocol::core::wayland::wl_output::Transform,
    ) -> Result<()> {
        todo!()
    }

    async fn set_buffer_scale(
        &self,
        _client: &mut Client,
        _sender_id: ObjectId,
        _scale: i32,
    ) -> Result<()> {
        todo!()
    }

    async fn damage_buffer(
        &self,
        _client: &mut Client,
        _sender_id: ObjectId,
        _x: i32,
        _y: i32,
        _width: i32,
        _height: i32,
    ) -> Result<()> {
        todo!()
    }

    async fn offset(
        &self,
        _client: &mut Client,
        _sender_id: ObjectId,
        _x: i32,
        _y: i32,
    ) -> Result<()> {
        todo!()
    }
}
