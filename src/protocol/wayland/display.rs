use crate::protocol::wayland::{
    callback::{Callback, WlCallback},
    registry::{Registry, WlRegistry},
};

use waynest::{
    server::{Client, Dispatcher, Result},
    wire::ObjectId,
};

pub use waynest::server::protocol::core::wayland::wl_display::*;

#[derive(Debug, Dispatcher, Default)]
pub struct Display;

impl WlDisplay for Display {
    async fn sync(
        &self,
        client: &mut Client,
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
        client: &mut Client,
        sender_id: ObjectId,
        registry_id: ObjectId,
    ) -> Result<()> {
        let registry = Registry::default();

        registry.advertise_globals(client, registry_id).await?;

        client.insert(registry_id, registry);

        Ok(())
    }
}
