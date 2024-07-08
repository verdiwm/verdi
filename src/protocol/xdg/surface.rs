use async_trait::async_trait;

use crate::{
    protocol::xdg::toplevel::{Toplevel, XdgToplevel},
    wire::{Message, ObjectId},
    Client, Dispatcher, Result,
};

pub use crate::protocol::interfaces::xdg_shell::xdg_surface::*;

#[derive(Debug)]
pub struct Surface {
    id: ObjectId,
    wl_surface: ObjectId,
}

impl Surface {
    pub fn new(id: ObjectId, wl_surface: ObjectId) -> Self {
        Self { id, wl_surface }
    }
}

impl XdgSurface for Surface {
    fn get_id(&self) -> ObjectId {
        self.id
    }

    async fn destroy(&self, _client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }

    async fn get_toplevel(
        &self,
        client: &mut crate::Client,
        id: crate::wire::ObjectId,
    ) -> crate::Result<()> {
        client.insert(id, Toplevel::new(id).into_dispatcher());

        Ok(())
    }

    async fn get_popup(
        &self,
        _client: &mut crate::Client,
        _id: crate::wire::ObjectId,
        _parent: Option<crate::wire::ObjectId>,
        _positioner: crate::wire::ObjectId,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn set_window_geometry(
        &self,
        _client: &mut crate::Client,
        _x: i32,
        _y: i32,
        _width: i32,
        _height: i32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn ack_configure(&self, _client: &mut crate::Client, _serial: u32) -> crate::Result<()> {
        todo!()
    }
}

#[async_trait]
impl Dispatcher for Surface {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        self.handle_request(client, message).await
    }
}
