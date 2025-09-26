use std::sync::Arc;

use tokio::sync::RwLock;
use waynest::ObjectId;
use waynest_server::{Connection, RequestDispatcher};

use crate::error::{Result, VerdiError};
use crate::protocol::xdg;

pub use waynest_protocols::server::stable::xdg_shell::xdg_toplevel::*;

#[derive(Debug, RequestDispatcher)]
#[waynest(error = VerdiError)]
pub struct Toplevel {
    xdg_surface: Arc<xdg::surface::Surface>,
    data: RwLock<ToplevelData>,
}

#[derive(Debug, Default)]
pub struct ToplevelData {
    title: Option<String>,
    app_id: Option<String>,
}

impl Toplevel {
    pub fn new(xdg_surface: Arc<xdg::surface::Surface>) -> Self {
        Self {
            xdg_surface,
            data: RwLock::default(),
        }
    }

    pub async fn title(&self) -> Option<String> {
        self.data.read().await.title.clone()
    }

    pub async fn app_id(&self) -> Option<String> {
        self.data.read().await.app_id.clone()
    }
}

impl XdgToplevel for Toplevel {
    type Connection = Connection<VerdiError>;

    async fn destroy(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
    ) -> Result<()> {
        Ok(())
    }

    async fn set_parent(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        _parent: Option<ObjectId>,
    ) -> Result<()> {
        todo!()
    }

    async fn set_title(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        title: String,
    ) -> Result<()> {
        self.data.write().await.title.replace(title);

        Ok(())
    }

    async fn set_app_id(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        app_id: String,
    ) -> Result<()> {
        self.data.write().await.app_id.replace(app_id);

        Ok(())
    }

    async fn show_window_menu(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        _seat: ObjectId,
        _serial: u32,
        _x: i32,
        _y: i32,
    ) -> Result<()> {
        todo!()
    }

    async fn r#move(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        _seat: ObjectId,
        _serial: u32,
    ) -> Result<()> {
        todo!()
    }

    async fn resize(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        _seat: ObjectId,
        _serial: u32,
        _edges: ResizeEdge,
    ) -> Result<()> {
        todo!()
    }

    async fn set_max_size(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        _width: i32,
        _height: i32,
    ) -> Result<()> {
        todo!()
    }

    async fn set_min_size(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        _width: i32,
        _height: i32,
    ) -> Result<()> {
        todo!()
    }

    async fn set_maximized(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
    ) -> Result<()> {
        todo!()
    }

    async fn unset_maximized(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
    ) -> Result<()> {
        todo!()
    }

    async fn set_fullscreen(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        _output: Option<ObjectId>,
    ) -> Result<()> {
        todo!()
    }

    async fn unset_fullscreen(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
    ) -> Result<()> {
        todo!()
    }

    async fn set_minimized(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
    ) -> Result<()> {
        todo!()
    }
}
