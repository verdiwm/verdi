use crate::{
    protocol::{
        wayland::{
            compositor::{Compositor, WlCompositor},
            seat::{Seat, WlSeat},
            shm::{Shm, WlShm},
        },
        xdg::wm_base::{WmBase, XdgWmBase},
    },
    wire::NewId,
    Client, Dispatcher, Error, Object, Result,
};

pub use crate::protocol::interfaces::wayland::wl_registry::*;

struct RegistryGlobals;

impl RegistryGlobals {
    pub const COMPOSITOR: u32 = 0;
    pub const SHM: u32 = 1;
    pub const WM_BASE: u32 = 2;
    pub const SEAT: u32 = 3;
}

#[derive(Debug, Dispatcher)]
pub struct Registry;

impl Registry {
    pub fn new() -> Self {
        Self
    }

    pub async fn advertise_globals(&self, object: &Object, client: &mut Client) -> Result<()> {
        self.global(
            object,
            client,
            RegistryGlobals::COMPOSITOR,
            Compositor::INTERFACE.to_string(),
            Compositor::VERSION,
        )
        .await?;

        self.global(
            object,
            client,
            RegistryGlobals::SHM,
            Shm::INTERFACE.to_string(),
            Shm::VERSION,
        )
        .await?;

        self.global(
            object,
            client,
            RegistryGlobals::WM_BASE,
            WmBase::INTERFACE.to_string(),
            WmBase::VERSION,
        )
        .await?;

        self.global(
            object,
            client,
            RegistryGlobals::SEAT,
            Seat::INTERFACE.to_string(),
            Seat::VERSION,
        )
        .await?;

        Ok(())
    }
}

impl WlRegistry for Registry {
    async fn bind(
        &self,
        _object: &Object,
        client: &mut Client,
        name: u32,
        new_id: NewId,
    ) -> Result<()> {
        match name {
            RegistryGlobals::COMPOSITOR => {
                client.insert(Compositor::new().into_object(new_id.object_id))
            }
            RegistryGlobals::SHM => {
                let shm = Shm::new().into_object(new_id.object_id);

                shm.as_dispatcher::<Shm>()?
                    .advertise_formats(&shm, client)
                    .await?;

                client.insert(shm);
            }
            RegistryGlobals::WM_BASE => client.insert(WmBase::new().into_object(new_id.object_id)),
            RegistryGlobals::SEAT => client.insert(Seat::new().into_object(new_id.object_id)),
            _ => return Err(Error::NotFound),
        }

        Ok(())
    }
}
