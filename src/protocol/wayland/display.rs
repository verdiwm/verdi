use async_trait::async_trait;

use crate::{
    protocol::wayland::{
        callback::{Callback, WlCallback},
        registry::{Registry, WlRegistry},
    },
    wire::{Message, ObjectId},
    Client, Dispatcher, Result,
};

pub use crate::protocol::interfaces::wayland::wl_display::*;

#[derive(Debug)]
pub struct Display {
    id: ObjectId,
}

impl WlDisplay for Display {
    fn new(id: crate::wire::ObjectId) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self { id })
    }

    fn get_id(&self) -> crate::wire::ObjectId {
        self.id
    }

    async fn sync(&self, client: &mut Client, callback: ObjectId) -> Result<()> {
        let serial = client.next_event_serial();

        let callback = Callback::new(callback)?;

        callback.done(client, serial).await?;

        self.delete_id(client, callback.get_id().as_raw()).await
    }

    async fn get_registry(&self, client: &mut Client, registry_id: ObjectId) -> Result<()> {
        let registry = Registry::new(registry_id)?;

        registry.advertise_globals(client).await?;

        client.insert(registry_id, registry.into_dispatcher());

        Ok(())
    }
}

#[async_trait]
impl Dispatcher for Display {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        self.handle_request(client, message).await
    }
}
