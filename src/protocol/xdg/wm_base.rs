use crate::protocol::xdg::surface::{Surface, XdgSurface};

use waynest::{
    server::{Client, Dispatcher, Result},
    wire::ObjectId,
};

pub use waynest::server::protocol::stable::xdg_shell::xdg_wm_base::*;

#[derive(Debug, Dispatcher, Default)]
pub struct WmBase;

impl XdgWmBase for WmBase {
    async fn destroy(&self, _client: &mut Client, _sender_id: ObjectId) -> Result<()> {
        todo!()
    }

    async fn create_positioner(
        &self,
        _client: &mut Client,
        _sender_id: ObjectId,
        _id: ObjectId,
    ) -> Result<()> {
        todo!()
    }

    async fn get_xdg_surface(
        &self,
        client: &mut Client,
        _sender_id: ObjectId,
        id: ObjectId,
        surface: ObjectId,
    ) -> Result<()> {
        client.insert(id, Surface::new(surface));

        Ok(())
    }

    async fn pong(&self, _client: &mut Client, _sender_id: ObjectId, _serial: u32) -> Result<()> {
        todo!()
    }
}
