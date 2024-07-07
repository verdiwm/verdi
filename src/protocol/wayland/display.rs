use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    protocol::wayland::{
        callback::{Callback, WlCallback},
        compositor::{Compositor, WlCompositor},
        registry::{Registry, WlRegistry},
        shm::{Shm, WlShm},
    },
    wire::{Message, ObjectId},
    Client, Dispatcher, Result,
};

pub use crate::protocol::interfaces::wayland::wl_display::*;

#[derive(Debug)]
pub struct Display;

impl WlDisplay for Display {
    async fn sync(client: &mut Client, callback: ObjectId) -> Result<()> {
        let serial = client.next_event_serial();

        Callback::done(callback, client, serial).await?;

        Self::delete_id(unsafe { ObjectId::from_raw(1) }, client, callback.as_raw()).await
    }

    async fn get_registry(client: &mut Client, registry_id: ObjectId) -> Result<()> {
        let registry = Registry::create_dispatcher(registry_id);
        client.insert(registry_id, registry);

        Registry::global(
            registry_id,
            client,
            0,
            Compositor::INTERFACE.to_string(),
            Compositor::VERSION,
        )
        .await?;

        Registry::global(
            registry_id,
            client,
            1,
            Shm::INTERFACE.to_string(),
            Shm::VERSION,
        )
        .await?;

        Ok(())
    }

    fn create_dispatcher(_id: ObjectId) -> Arc<Box<dyn Dispatcher + Send + Sync>> {
        Arc::new(Box::new(Self {}))
    }
}

#[async_trait]
impl Dispatcher for Display {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        <Self as WlDisplay>::handle_request(client, message).await
    }
}
