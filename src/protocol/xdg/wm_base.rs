use waynest::ObjectId;
use waynest_server::{Connection, RequestDispatcher};

use crate::error::{Result, VerdiError};
use crate::protocol::{
    wayland,
    xdg::surface::{Surface, XdgSurface},
};

pub use waynest_protocols::server::stable::xdg_shell::xdg_wm_base::*;

#[derive(Debug, RequestDispatcher, Default)]
#[waynest(error = VerdiError)]
pub struct WmBase;

impl XdgWmBase for WmBase {
    type Connection = Connection<VerdiError>;

    async fn destroy(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
    ) -> Result<()> {
        todo!()
    }

    async fn create_positioner(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        _id: ObjectId,
    ) -> Result<()> {
        todo!()
    }

    async fn get_xdg_surface(
        &self,
        client: &mut Self::Connection,
        _sender_id: ObjectId,
        id: ObjectId,
        surface: ObjectId,
    ) -> Result<()> {
        let surface = client.get::<wayland::surface::Surface>(surface).unwrap();

        client.insert(id, Surface::new(surface));

        Ok(())
    }

    async fn pong(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        _serial: u32,
    ) -> Result<()> {
        Ok(())
    }
}
