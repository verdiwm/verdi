use async_trait::async_trait;

use crate::{
    wire::{Message, ObjectId},
    Client, Dispatcher, Result,
};

pub use crate::protocol::interfaces::xdg_shell::xdg_toplevel::*;

#[derive(Debug)]
pub struct Toplevel {
    id: ObjectId,
}

impl Toplevel {
    pub fn new(id: ObjectId) -> Self {
        Self { id }
    }
}

impl XdgToplevel for Toplevel {
    fn get_id(&self) -> ObjectId {
        self.id
    }

    async fn destroy(&self, client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }

    async fn set_parent(
        &self,
        client: &mut crate::Client,
        parent: Option<crate::wire::ObjectId>,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn set_title(&self, client: &mut crate::Client, title: String) -> crate::Result<()> {
        // FIXME: change  state

        Ok(())
    }

    async fn set_app_id(&self, client: &mut crate::Client, app_id: String) -> crate::Result<()> {
        // FIXME: change  state

        Ok(())
    }

    async fn show_window_menu(
        &self,
        client: &mut crate::Client,
        seat: crate::wire::ObjectId,
        serial: u32,
        x: i32,
        y: i32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#move(
        &self,
        client: &mut crate::Client,
        seat: crate::wire::ObjectId,
        serial: u32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn resize(
        &self,
        client: &mut crate::Client,
        seat: crate::wire::ObjectId,
        serial: u32,
        edges: ResizeEdge,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn set_max_size(
        &self,
        client: &mut crate::Client,
        width: i32,
        height: i32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn set_min_size(
        &self,
        client: &mut crate::Client,
        width: i32,
        height: i32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn set_maximized(&self, client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }

    async fn unset_maximized(&self, client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }

    async fn set_fullscreen(
        &self,
        client: &mut crate::Client,
        output: Option<crate::wire::ObjectId>,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn unset_fullscreen(&self, client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }

    async fn set_minimized(&self, client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }
}

#[async_trait]
impl Dispatcher for Toplevel {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        self.handle_request(client, message).await
    }
}
