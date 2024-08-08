use crate::{
    protocol::xdg::surface::{Surface, XdgSurface},
    Dispatcher, Object, Result,
};

use waynest::wire::ObjectId;

pub use crate::protocol::interfaces::xdg_shell::xdg_wm_base::*;

#[derive(Debug, Dispatcher)]
pub struct WmBase;

impl WmBase {
    pub fn new() -> Self {
        Self
    }
}

impl XdgWmBase for WmBase {
    async fn destroy(&self, _object: &Object, _client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }

    async fn create_positioner(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
        _id: ObjectId,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn get_xdg_surface(
        &self,
        _object: &Object,
        client: &mut crate::Client,
        id: ObjectId,
        surface: ObjectId,
    ) -> Result<()> {
        client.insert(Surface::new(surface).into_object(id));

        Ok(())
    }

    async fn pong(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
        _serial: u32,
    ) -> crate::Result<()> {
        todo!()
    }
}
