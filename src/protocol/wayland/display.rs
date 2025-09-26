use waynest::ObjectId;
use waynest_server::{Connection, RequestDispatcher};

use crate::{
    error::{Result, VerdiError},
    protocol::wayland::{
        callback::{Callback, WlCallback},
        registry::{Registry, WlRegistry},
    },
};

pub use waynest_protocols::server::core::wayland::wl_display::*;

#[derive(Debug, Default, RequestDispatcher)]
#[waynest(error = VerdiError)]
pub struct Display;

impl WlDisplay for Display {
    type Connection = Connection<VerdiError>;

    async fn sync(
        &self,
        client: &mut Self::Connection,
        sender_id: ObjectId,
        callback_id: ObjectId,
    ) -> Result<()> {
        let serial = client.next_event_serial();

        let callback = Callback::default();

        callback.done(client, callback_id, serial).await?;

        self.delete_id(client, sender_id, callback_id.as_raw())
            .await
    }

    async fn get_registry(
        &self,
        client: &mut Self::Connection,
        sender_id: ObjectId,
        registry_id: ObjectId,
    ) -> Result<()> {
        let registry = Registry::default();

        registry.advertise_globals(client, registry_id).await?;

        client.insert(registry_id, registry);

        Ok(())
    }
}
