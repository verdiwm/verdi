use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    protocol::{
        wayland::{
            compositor::{Compositor, WlCompositor},
            shm::{Shm, WlShm},
        },
        xdg::wm_base::{WmBase, XdgWmBase},
    },
    wire::{Message, NewId, ObjectId},
    Client, Dispatcher, Error, Result,
};

pub use crate::protocol::interfaces::wayland::wl_registry::*;

struct RegistryGlobals;

impl RegistryGlobals {
    pub const COMPOSITOR: u32 = 0;
    pub const SHM: u32 = 1;
    pub const WM_BASE: u32 = 2;
}

#[derive(Debug)]
pub struct Registry;

impl WlRegistry for Registry {
    async fn r#bind(client: &mut Client, name: u32, id: NewId) -> Result<()> {
        match name {
            RegistryGlobals::COMPOSITOR => client.insert(id.id, Compositor::new()),
            RegistryGlobals::SHM => client.insert(id.id, Shm::new()),
            RegistryGlobals::WM_BASE => client.insert(id.id, WmBase::new()),
            _ => return Err(Error::NotFound),
        }

        Ok(())
    }
}

#[async_trait]
impl Dispatcher for Registry {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        <Self as WlRegistry>::handle_request(client, message).await
    }
}

impl Registry {
    pub async fn new(
        client: &mut Client,
        id: ObjectId,
    ) -> Result<Arc<Box<dyn Dispatcher + Send + Sync>>> {
        Registry::global(
            id,
            client,
            RegistryGlobals::COMPOSITOR,
            Compositor::INTERFACE.to_string(),
            Compositor::VERSION,
        )
        .await?;

        Registry::global(
            id,
            client,
            RegistryGlobals::SHM,
            Shm::INTERFACE.to_string(),
            Shm::VERSION,
        )
        .await?;

        Registry::global(
            id,
            client,
            RegistryGlobals::WM_BASE,
            WmBase::INTERFACE.to_string(),
            WmBase::VERSION,
        )
        .await?;

        Ok(Arc::new(Box::new(Self {})))
    }
}
