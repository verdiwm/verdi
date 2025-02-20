use crate::protocol::wayland::surface::{Surface, WlSurface};

use waynest::{
    server::{Client, Dispatcher, Result},
    wire::ObjectId,
};

pub use waynest::server::protocol::core::wayland::wl_compositor::*;

#[derive(Debug, Dispatcher, Default)]
pub struct Compositor;

impl WlCompositor for Compositor {
    async fn create_surface(
        &self,
        client: &mut Client,
        _sender_id: ObjectId,
        id: ObjectId,
    ) -> Result<()> {
        client.insert(id, Surface::default());

        Ok(())
    }

    async fn create_region(
        &self,
        _client: &mut Client,
        _sender_id: ObjectId,
        _id: ObjectId,
    ) -> Result<()> {
        todo!()
    }
}
