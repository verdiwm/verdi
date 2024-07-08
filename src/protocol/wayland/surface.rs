use async_trait::async_trait;

use crate::{
    wire::{Message, ObjectId},
    Client, Dispatcher, Result,
};

pub use crate::protocol::interfaces::wayland::wl_surface::*;

#[derive(Debug)]
pub struct Surface {
    id: ObjectId,
}

impl Surface {
    pub fn new(id: ObjectId) -> Self {
        Self { id }
    }
}

impl WlSurface for Surface {
    fn get_id(&self) -> ObjectId {
        self.id
    }

    async fn r#destroy(&self, _client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }

    async fn r#attach(
        &self,
        _client: &mut crate::Client,
        _buffer: Option<crate::wire::ObjectId>,
        _x: i32,
        _y: i32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#damage(
        &self,
        _client: &mut crate::Client,
        _x: i32,
        _y: i32,
        _width: i32,
        _height: i32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#frame(
        &self,
        _client: &mut crate::Client,
        _callback: crate::wire::ObjectId,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#set_opaque_region(
        &self,
        _client: &mut crate::Client,
        _region: Option<crate::wire::ObjectId>,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#set_input_region(
        &self,
        _client: &mut crate::Client,
        _region: Option<crate::wire::ObjectId>,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#commit(&self, _client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }

    async fn r#set_buffer_transform(
        &self,
        _client: &mut crate::Client,
        _transform: crate::protocol::interfaces::wayland::wl_output::Transform,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#set_buffer_scale(
        &self,
        _client: &mut crate::Client,
        _scale: i32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#damage_buffer(
        &self,
        _client: &mut crate::Client,
        _x: i32,
        _y: i32,
        _width: i32,
        _height: i32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#offset(&self, _client: &mut crate::Client, _x: i32, _y: i32) -> crate::Result<()> {
        todo!()
    }
}

#[async_trait]
impl Dispatcher for Surface {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        self.handle_request(client, message).await
    }
}