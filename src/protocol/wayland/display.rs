use crate::{
    protocol::wayland::{
        callback::{Callback, WlCallback},
        registry::{Registry, WlRegistry},
    },
    wire::ObjectId,
    Client, Dispatcher, Object, Result,
};

pub use crate::protocol::interfaces::wayland::wl_display::*;

#[derive(Debug, Dispatcher)]
pub struct Display;

impl Display {
    pub fn new() -> Self {
        Self
    }
}

impl WlDisplay for Display {
    async fn sync(
        &self,
        object: &Object,
        client: &mut Client,
        callback_id: ObjectId,
    ) -> Result<()> {
        let serial = client.next_event_serial();

        let callback = Callback::new().into_object(callback_id);

        callback
            .as_dispatcher::<Callback>()?
            .done(&callback, client, serial)
            .await?;

        self.delete_id(object, client, callback_id.as_raw()).await
    }

    async fn get_registry(
        &self,
        _object: &Object,
        client: &mut Client,
        registry_id: ObjectId,
    ) -> Result<()> {
        let registry = Registry::new().into_object(registry_id);

        registry
            .as_dispatcher::<Registry>()?
            .advertise_globals(&registry, client)
            .await?;

        client.insert(registry);

        Ok(())
    }
}
