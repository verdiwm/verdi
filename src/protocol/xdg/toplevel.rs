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

    async fn r#destroy(&self, client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }

    async fn r#set_parent(
        &self,
        client: &mut crate::Client,
        r#parent: Option<crate::wire::ObjectId>,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#set_title(&self, client: &mut crate::Client, r#title: String) -> crate::Result<()> {
        // FIXME: change  state

        Ok(())
    }

    async fn r#set_app_id(
        &self,
        client: &mut crate::Client,
        r#app_id: String,
    ) -> crate::Result<()> {
        // FIXME: change  state

        Ok(())
    }

    async fn r#show_window_menu(
        &self,
        client: &mut crate::Client,
        r#seat: crate::wire::ObjectId,
        r#serial: u32,
        r#x: i32,
        r#y: i32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#move(
        &self,
        client: &mut crate::Client,
        r#seat: crate::wire::ObjectId,
        r#serial: u32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#resize(
        &self,
        client: &mut crate::Client,
        r#seat: crate::wire::ObjectId,
        r#serial: u32,
        r#edges: ResizeEdge,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#set_max_size(
        &self,
        client: &mut crate::Client,
        r#width: i32,
        r#height: i32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#set_min_size(
        &self,
        client: &mut crate::Client,
        r#width: i32,
        r#height: i32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#set_maximized(&self, client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }

    async fn r#unset_maximized(&self, client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }

    async fn r#set_fullscreen(
        &self,
        client: &mut crate::Client,
        r#output: Option<crate::wire::ObjectId>,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#unset_fullscreen(&self, client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }

    async fn r#set_minimized(&self, client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }
}

#[async_trait]
impl Dispatcher for Toplevel {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        self.handle_request(client, message).await
    }
}
