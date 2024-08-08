use crate::{
    protocol::wayland::surface::{Surface, WlSurface},
    Client, Dispatcher, Object, Result,
};

use waynest::wire::ObjectId;

pub use crate::protocol::interfaces::wayland::wl_compositor::*;

#[derive(Debug, Dispatcher)]
pub struct Compositor;

impl Compositor {
    pub fn new() -> Self {
        Self
    }
}

impl WlCompositor for Compositor {
    async fn create_surface(
        &self,
        _object: &Object,
        client: &mut Client,
        id: ObjectId,
    ) -> Result<()> {
        client.insert(Surface::new().into_object(id));

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
