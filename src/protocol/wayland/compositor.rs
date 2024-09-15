use crate::protocol::wayland::surface::{Surface, WlSurface};

use waynest::{
    server::{Client, Dispatcher, Object, Result},
    wire::ObjectId,
};

pub use waynest::server::protocol::core::wayland::wl_compositor::*;

#[derive(Debug, Dispatcher, Default)]
pub struct Compositor;

impl WlCompositor for Compositor {
    async fn create_surface(
        &self,
        _object: &Object,
        client: &mut Client,
        id: ObjectId,
    ) -> Result<()> {
        client.insert(Surface::default().into_object(id));

        Ok(())
    }

    async fn create_region(
        &self,
        _object: &Object,
        _client: &mut Client,
        _id: ObjectId,
    ) -> Result<()> {
        todo!()
    }
}
