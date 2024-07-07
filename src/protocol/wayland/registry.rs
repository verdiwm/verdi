use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    protocol::{
        interfaces::wayland::{wl_compositor::WlCompositor, wl_shm::WlShm},
        wayland::{compositor::Compositor, shm::Shm},
    },
    wire::{Message, NewId},
    Client, Dispatcher, Error, Result,
};

pub use crate::protocol::interfaces::wayland::wl_registry::*;

#[derive(Debug)]
pub struct Registry;

impl WlRegistry for Registry {
    async fn r#bind(client: &mut Client, name: u32, id: NewId) -> Result<()> {
        match name {
            0 => client.insert(id.id, Compositor::create_dispatcher()),
            1 => client.insert(id.id, Shm::create_dispatcher()),
            _ => return Err(Error::NotFound),
        }

        Ok(())
    }

    fn create_dispatcher() -> Arc<Box<dyn Dispatcher + Send + Sync>> {
        Arc::new(Box::new(Self {}))
    }
}

#[async_trait]
impl Dispatcher for Registry {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        <Self as WlRegistry>::handle_request(client, message).await
    }
}
