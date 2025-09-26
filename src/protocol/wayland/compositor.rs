use waynest::ObjectId;
use waynest_server::{Connection, RequestDispatcher};

use crate::{
    error::{Result, VerdiError},
    protocol::wayland::surface::{Surface, WlSurface},
};

pub use waynest_protocols::server::core::wayland::wl_compositor::*;

#[derive(Debug, RequestDispatcher, Default)]
#[waynest(error = VerdiError)]
pub struct Compositor;

impl WlCompositor for Compositor {
    type Connection = Connection<VerdiError>;

    async fn create_surface(
        &self,
        connection: &mut Self::Connection,
        _sender_id: ObjectId,
        id: ObjectId,
    ) -> Result<()> {
        connection.insert(id, Surface::default());

        Ok(())
    }

    async fn create_region(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        _id: ObjectId,
    ) -> Result<()> {
        todo!()
    }
}
