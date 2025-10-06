use waynest::ObjectId;
use waynest_server::{Client as _, RequestDispatcher};

use crate::{
    Client, Result, VerdiError,
    protocol::wayland::surface::{Surface, WlSurface},
};

pub use waynest_protocols::server::core::wayland::wl_compositor::*;

#[derive(Debug, RequestDispatcher, Default)]
#[waynest(error = VerdiError, connection = Client)]
pub struct Compositor;

impl WlCompositor for Compositor {
    type Connection = Client;

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
