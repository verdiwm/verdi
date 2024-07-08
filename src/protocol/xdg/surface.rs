use async_trait::async_trait;

use crate::{
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

    async fn r#destroy(&self, client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }

    async fn r#get_toplevel(
        &self,
        client: &mut crate::Client,
        r#id: crate::wire::ObjectId,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#get_popup(
        &self,
        client: &mut crate::Client,
        r#id: crate::wire::ObjectId,
        r#parent: Option<crate::wire::ObjectId>,
        r#positioner: crate::wire::ObjectId,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#set_window_geometry(
        &self,
        client: &mut crate::Client,
        r#x: i32,
        r#y: i32,
        r#width: i32,
        r#height: i32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#ack_configure(
        &self,
        client: &mut crate::Client,
        r#serial: u32,
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
