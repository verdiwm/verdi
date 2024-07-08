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

    async fn destroy(&self, client: &mut crate::Client) -> crate::Result<()> {
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
        client: &mut crate::Client,
        id: crate::wire::ObjectId,
        parent: Option<crate::wire::ObjectId>,
        positioner: crate::wire::ObjectId,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn set_window_geometry(
        &self,
        client: &mut crate::Client,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn ack_configure(
        &self,
        client: &mut crate::Client,
        serial: u32,
    ) -> crate::Result<()> {
        todo!()
    }
}

#[async_trait]
impl Dispatcher for Surface {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        self.handle_request(client, message).await
    }
}
