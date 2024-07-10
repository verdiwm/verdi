use async_trait::async_trait;

use crate::{wire::Message, Client, Dispatcher, Object, Result};

pub use crate::protocol::interfaces::xdg_shell::xdg_toplevel::*;

#[derive(Debug)]
pub struct Toplevel;

impl Toplevel {
    pub fn new() -> Self {
        Self
    }
}

impl XdgToplevel for Toplevel {
    async fn destroy(&self, _object: &Object, _client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }

    async fn set_parent(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
        _parent: Option<crate::wire::ObjectId>,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn set_title(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
        _title: String,
    ) -> crate::Result<()> {
        // FIXME: change  state

        Ok(())
    }

    async fn set_app_id(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
        _app_id: String,
    ) -> crate::Result<()> {
        // FIXME: change  state

        Ok(())
    }

    async fn show_window_menu(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
        _seat: crate::wire::ObjectId,
        _serial: u32,
        _x: i32,
        _y: i32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn r#move(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
        _seat: crate::wire::ObjectId,
        _serial: u32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn resize(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
        _seat: crate::wire::ObjectId,
        _serial: u32,
        _edges: ResizeEdge,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn set_max_size(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
        _width: i32,
        _height: i32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn set_min_size(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
        _width: i32,
        _height: i32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn set_maximized(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn unset_maximized(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn set_fullscreen(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
        _output: Option<crate::wire::ObjectId>,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn unset_fullscreen(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn set_minimized(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
    ) -> crate::Result<()> {
        todo!()
    }
}

#[async_trait]
impl Dispatcher for Toplevel {
    async fn dispatch(
        &self,
        object: &Object,
        client: &mut Client,
        message: &mut Message,
    ) -> Result<()> {
        self.handle_request(object, client, message).await
    }
}
