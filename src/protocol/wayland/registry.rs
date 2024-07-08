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
pub struct Registry {
    id: ObjectId,
}

impl Registry {
    pub fn new(id: ObjectId) -> Self {
        Self { id }
    }

    pub async fn advertise_globals(&self, client: &mut Client) -> Result<()> {
        self.global(
            client,
            RegistryGlobals::COMPOSITOR,
            Compositor::INTERFACE.to_string(),
            Compositor::VERSION,
        )
        .await?;

        self.global(
            client,
            RegistryGlobals::SHM,
            Shm::INTERFACE.to_string(),
            Shm::VERSION,
        )
        .await?;

        self.global(
            client,
            RegistryGlobals::WM_BASE,
            WmBase::INTERFACE.to_string(),
            WmBase::VERSION,
        )
        .await?;

        Ok(())
    }
}

impl WlRegistry for Registry {
    fn get_id(&self) -> ObjectId {
        self.id
    }

    async fn r#bind(&self, client: &mut Client, name: u32, id: NewId) -> Result<()> {
        match name {
            RegistryGlobals::COMPOSITOR => {
                client.insert(id.id, Compositor::new(id.id).into_dispatcher())
            }
            RegistryGlobals::SHM => {
                let shm = Shm::new(id.id);

                shm.advertise_formats(client).await?;

                client.insert(id.id, shm.into_dispatcher())
            }
            RegistryGlobals::WM_BASE => client.insert(id.id, WmBase::new(id.id).into_dispatcher()),
            _ => return Err(Error::NotFound),
        }

        Ok(())
    }
}

#[async_trait]
impl Dispatcher for Registry {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        self.handle_request(client, message).await
    }
}
