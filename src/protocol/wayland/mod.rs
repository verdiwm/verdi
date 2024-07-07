use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    protocol::interfaces::wayland::{
        wl_callback::WlCallback, wl_compositor::WlCompositor, wl_display::WlDisplay,
        wl_registry::WlRegistry, wl_shm::WlShm,
    },
    wire::{Message, NewId, ObjectId},
    Client, Dispatcher, Error, Result,
};

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

    fn create_dispatcher(id: ObjectId) -> Arc<Box<dyn Dispatcher + Send + Sync>> {
        Arc::new(Box::new(Self {}))
    }
}

#[async_trait]
impl Dispatcher for Display {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        <Self as WlDisplay>::handle_request(client, message).await
    }
}

#[derive(Debug)]
pub struct Registry;

impl WlRegistry for Registry {
    async fn r#bind(client: &mut Client, name: u32, id: NewId) -> Result<()> {
        match name {
            0 => client.insert(id.id, Compositor::create_dispatcher(id.id)),
            1 => client.insert(id.id, Shm::create_dispatcher(id.id)),
            _ => return Err(Error::NotFound),
        }

        Ok(())
    }

    fn create_dispatcher(id: ObjectId) -> Arc<Box<dyn Dispatcher + Send + Sync>> {
        Arc::new(Box::new(Self {}))
    }
}

#[async_trait]
impl Dispatcher for Registry {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        <Self as WlRegistry>::handle_request(client, message).await
    }
}

#[derive(Debug)]
pub struct Compositor;

impl WlCompositor for Compositor {
    async fn r#create_surface(client: &mut Client, r#id: ObjectId) -> Result<()> {
        todo!()
    }

    async fn r#create_region(client: &mut Client, r#id: ObjectId) -> Result<()> {
        todo!()
    }

    fn create_dispatcher(id: ObjectId) -> Arc<Box<dyn Dispatcher + Send + Sync>> {
        Arc::new(Box::new(Self {}))
    }
}

#[async_trait]
impl Dispatcher for Compositor {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        <Self as WlCompositor>::handle_request(client, message).await
    }
}

#[derive(Debug)]
pub struct Callback;

impl WlCallback for Callback {
    fn create_dispatcher(id: ObjectId) -> Arc<Box<dyn Dispatcher + Send + Sync>> {
        Arc::new(Box::new(Self {}))
    }
}

#[async_trait]
impl Dispatcher for Callback {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        <Self as WlCallback>::handle_request(client, message).await
    }
}

#[derive(Debug)]
pub struct Shm;

impl WlShm for Shm {
    async fn r#create_pool(
        client: &mut Client,
        r#id: ObjectId,
        r#fd: std::os::unix::prelude::RawFd,
        r#size: i32,
    ) -> Result<()> {
        todo!()
    }

    async fn r#release(client: &mut Client) -> Result<()> {
        todo!()
    }

    fn create_dispatcher(id: ObjectId) -> Arc<Box<dyn Dispatcher + Send + Sync>> {
        Arc::new(Box::new(Self {}))
    }
}

#[async_trait]
impl Dispatcher for Shm {
    async fn dispatch(&self, client: &mut Client, message: &mut Message) -> Result<()> {
        <Self as WlShm>::handle_request(client, message).await
    }
}
