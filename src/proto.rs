#![allow(unused)]
#![allow(async_fn_in_trait)]
pub mod wayland {
    use crate::{
        error::Error,
        message::{DecodeError, Fixed, Message, NewId, ObjectId, PayloadBuilder},
        Client, Dispatcher, Result,
    };
    use std::os::fd::RawFd;
    pub trait r#WlDisplay {
        const INTERFACE: &'static str = "wl_display";
        const VERSION: u32 = 1;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#sync(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                1 => Self::r#get_registry(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#sync(client: &mut Client, r#callback: ObjectId) -> Result<()>;
        fn r#get_registry(client: &mut Client, r#registry: ObjectId) -> Result<()>;
        async fn r#error(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#object_id: ObjectId,
            r#code: u32,
            r#message: String,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_object(Some(object_id))
                .put_uint(code)
                .put_string(Some(message))
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#delete_id(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#id: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(id).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#WlRegistry {
        const INTERFACE: &'static str = "wl_registry";
        const VERSION: u32 = 1;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#bind(client, message.uint()?, message.new_id()?),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#bind(client: &mut Client, r#name: u32, r#id: NewId) -> Result<()>;
        async fn r#global(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#name: u32,
            r#interface: String,
            r#version: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(name)
                .put_string(Some(interface))
                .put_uint(version)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#global_remove(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#name: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(name).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#WlCallback {
        const INTERFACE: &'static str = "wl_callback";
        const VERSION: u32 = 1;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        async fn r#done(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#callback_data: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(callback_data).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#WlCompositor {
        const INTERFACE: &'static str = "wl_compositor";
        const VERSION: u32 = 6;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#create_surface(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                1 => Self::r#create_region(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#create_surface(client: &mut Client, r#id: ObjectId) -> Result<()>;
        fn r#create_region(client: &mut Client, r#id: ObjectId) -> Result<()>;
    }
    pub trait r#WlShmPool {
        const INTERFACE: &'static str = "wl_shm_pool";
        const VERSION: u32 = 2;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#create_buffer(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.int()?,
                    message.int()?,
                    message.int()?,
                    message.int()?,
                    message.uint()?,
                ),
                1 => Self::r#destroy(client),
                2 => Self::r#resize(client, message.int()?),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#create_buffer(
            client: &mut Client,
            r#id: ObjectId,
            r#offset: i32,
            r#width: i32,
            r#height: i32,
            r#stride: i32,
            r#format: u32,
        ) -> Result<()>;
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#resize(client: &mut Client, r#size: i32) -> Result<()>;
    }
    pub trait r#WlShm {
        const INTERFACE: &'static str = "wl_shm";
        const VERSION: u32 = 2;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#create_pool(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.int()?,
                    message.int()?,
                ),
                1 => Self::r#release(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#create_pool(
            client: &mut Client,
            r#id: ObjectId,
            r#fd: RawFd,
            r#size: i32,
        ) -> Result<()>;
        fn r#release(client: &mut Client) -> Result<()>;
        async fn r#format(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#format: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(format).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#WlBuffer {
        const INTERFACE: &'static str = "wl_buffer";
        const VERSION: u32 = 1;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#destroy(client: &mut Client) -> Result<()>;
        async fn r#release(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#WlDataOffer {
        const INTERFACE: &'static str = "wl_data_offer";
        const VERSION: u32 = 3;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#accept(client, message.uint()?, message.string()?),
                1 => Self::r#receive(
                    client,
                    message.string()?.ok_or(DecodeError::MalformedPayload)?,
                    message.int()?,
                ),
                2 => Self::r#destroy(client),
                3 => Self::r#finish(client),
                4 => Self::r#set_actions(client, message.uint()?, message.uint()?),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#accept(client: &mut Client, r#serial: u32, r#mime_type: Option<String>) -> Result<()>;
        fn r#receive(client: &mut Client, r#mime_type: String, r#fd: RawFd) -> Result<()>;
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#finish(client: &mut Client) -> Result<()>;
        fn r#set_actions(
            client: &mut Client,
            r#dnd_actions: u32,
            r#preferred_action: u32,
        ) -> Result<()>;
        async fn r#offer(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#mime_type: String,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_string(Some(mime_type)).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#source_actions(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#source_actions: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(source_actions).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#action(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#dnd_action: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(dnd_action).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#WlDataSource {
        const INTERFACE: &'static str = "wl_data_source";
        const VERSION: u32 = 3;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#offer(
                    client,
                    message.string()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                1 => Self::r#destroy(client),
                2 => Self::r#set_actions(client, message.uint()?),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#offer(client: &mut Client, r#mime_type: String) -> Result<()>;
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#set_actions(client: &mut Client, r#dnd_actions: u32) -> Result<()>;
        async fn r#target(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#mime_type: Option<String>,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_string(mime_type).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#send(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#mime_type: String,
            r#fd: RawFd,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_string(Some(mime_type))
                .put_int(fd)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#cancelled(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#dnd_drop_performed(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#dnd_finished(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#action(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#dnd_action: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(dnd_action).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#WlDataDevice {
        const INTERFACE: &'static str = "wl_data_device";
        const VERSION: u32 = 3;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#start_drag(
                    client,
                    message.object()?,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.object()?,
                    message.uint()?,
                ),
                1 => Self::r#set_selection(client, message.object()?, message.uint()?),
                2 => Self::r#release(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#start_drag(
            client: &mut Client,
            r#source: Option<ObjectId>,
            r#origin: ObjectId,
            r#icon: Option<ObjectId>,
            r#serial: u32,
        ) -> Result<()>;
        fn r#set_selection(
            client: &mut Client,
            r#source: Option<ObjectId>,
            r#serial: u32,
        ) -> Result<()>;
        fn r#release(client: &mut Client) -> Result<()>;
        async fn r#data_offer(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#id: ObjectId,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_object(Some(id)).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#enter(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#serial: u32,
            r#surface: ObjectId,
            r#x: Fixed,
            r#y: Fixed,
            r#id: Option<ObjectId>,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(serial)
                .put_object(Some(surface))
                .put_fixed(x)
                .put_fixed(y)
                .put_object(id)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#leave(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#motion(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#time: u32,
            r#x: Fixed,
            r#y: Fixed,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(time)
                .put_fixed(x)
                .put_fixed(y)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#drop(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#selection(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#id: Option<ObjectId>,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_object(id).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#WlDataDeviceManager {
        const INTERFACE: &'static str = "wl_data_device_manager";
        const VERSION: u32 = 3;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#create_data_source(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                1 => Self::r#get_data_device(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#create_data_source(client: &mut Client, r#id: ObjectId) -> Result<()>;
        fn r#get_data_device(client: &mut Client, r#id: ObjectId, r#seat: ObjectId) -> Result<()>;
    }
    pub trait r#WlShell {
        const INTERFACE: &'static str = "wl_shell";
        const VERSION: u32 = 1;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#get_shell_surface(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#get_shell_surface(
            client: &mut Client,
            r#id: ObjectId,
            r#surface: ObjectId,
        ) -> Result<()>;
    }
    pub trait r#WlShellSurface {
        const INTERFACE: &'static str = "wl_shell_surface";
        const VERSION: u32 = 1;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#pong(client, message.uint()?),
                1 => Self::r#move(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.uint()?,
                ),
                2 => Self::r#resize(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.uint()?,
                    message.uint()?,
                ),
                3 => Self::r#set_toplevel(client),
                4 => Self::r#set_transient(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.int()?,
                    message.int()?,
                    message.uint()?,
                ),
                5 => Self::r#set_fullscreen(
                    client,
                    message.uint()?,
                    message.uint()?,
                    message.object()?,
                ),
                6 => Self::r#set_popup(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.uint()?,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.int()?,
                    message.int()?,
                    message.uint()?,
                ),
                7 => Self::r#set_maximized(client, message.object()?),
                8 => Self::r#set_title(
                    client,
                    message.string()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                9 => Self::r#set_class(
                    client,
                    message.string()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#pong(client: &mut Client, r#serial: u32) -> Result<()>;
        fn r#move(client: &mut Client, r#seat: ObjectId, r#serial: u32) -> Result<()>;
        fn r#resize(
            client: &mut Client,
            r#seat: ObjectId,
            r#serial: u32,
            r#edges: u32,
        ) -> Result<()>;
        fn r#set_toplevel(client: &mut Client) -> Result<()>;
        fn r#set_transient(
            client: &mut Client,
            r#parent: ObjectId,
            r#x: i32,
            r#y: i32,
            r#flags: u32,
        ) -> Result<()>;
        fn r#set_fullscreen(
            client: &mut Client,
            r#method: u32,
            r#framerate: u32,
            r#output: Option<ObjectId>,
        ) -> Result<()>;
        fn r#set_popup(
            client: &mut Client,
            r#seat: ObjectId,
            r#serial: u32,
            r#parent: ObjectId,
            r#x: i32,
            r#y: i32,
            r#flags: u32,
        ) -> Result<()>;
        fn r#set_maximized(client: &mut Client, r#output: Option<ObjectId>) -> Result<()>;
        fn r#set_title(client: &mut Client, r#title: String) -> Result<()>;
        fn r#set_class(client: &mut Client, r#class: String) -> Result<()>;
        async fn r#ping(dispatcher_id: ObjectId, client: &mut Client, r#serial: u32) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(serial).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#configure(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#edges: u32,
            r#width: i32,
            r#height: i32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(edges)
                .put_int(width)
                .put_int(height)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#popup_done(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#WlSurface {
        const INTERFACE: &'static str = "wl_surface";
        const VERSION: u32 = 6;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(client),
                1 => Self::r#attach(client, message.object()?, message.int()?, message.int()?),
                2 => Self::r#damage(
                    client,
                    message.int()?,
                    message.int()?,
                    message.int()?,
                    message.int()?,
                ),
                3 => Self::r#frame(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                4 => Self::r#set_opaque_region(client, message.object()?),
                5 => Self::r#set_input_region(client, message.object()?),
                6 => Self::r#commit(client),
                7 => Self::r#set_buffer_transform(client, message.int()?),
                8 => Self::r#set_buffer_scale(client, message.int()?),
                9 => Self::r#damage_buffer(
                    client,
                    message.int()?,
                    message.int()?,
                    message.int()?,
                    message.int()?,
                ),
                10 => Self::r#offset(client, message.int()?, message.int()?),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#attach(
            client: &mut Client,
            r#buffer: Option<ObjectId>,
            r#x: i32,
            r#y: i32,
        ) -> Result<()>;
        fn r#damage(
            client: &mut Client,
            r#x: i32,
            r#y: i32,
            r#width: i32,
            r#height: i32,
        ) -> Result<()>;
        fn r#frame(client: &mut Client, r#callback: ObjectId) -> Result<()>;
        fn r#set_opaque_region(client: &mut Client, r#region: Option<ObjectId>) -> Result<()>;
        fn r#set_input_region(client: &mut Client, r#region: Option<ObjectId>) -> Result<()>;
        fn r#commit(client: &mut Client) -> Result<()>;
        fn r#set_buffer_transform(client: &mut Client, r#transform: i32) -> Result<()>;
        fn r#set_buffer_scale(client: &mut Client, r#scale: i32) -> Result<()>;
        fn r#damage_buffer(
            client: &mut Client,
            r#x: i32,
            r#y: i32,
            r#width: i32,
            r#height: i32,
        ) -> Result<()>;
        fn r#offset(client: &mut Client, r#x: i32, r#y: i32) -> Result<()>;
        async fn r#enter(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#output: ObjectId,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_object(Some(output)).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#leave(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#output: ObjectId,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_object(Some(output)).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#preferred_buffer_scale(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#factor: i32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_int(factor).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#preferred_buffer_transform(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#transform: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(transform).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#WlSeat {
        const INTERFACE: &'static str = "wl_seat";
        const VERSION: u32 = 9;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#get_pointer(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                1 => Self::r#get_keyboard(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                2 => Self::r#get_touch(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                3 => Self::r#release(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#get_pointer(client: &mut Client, r#id: ObjectId) -> Result<()>;
        fn r#get_keyboard(client: &mut Client, r#id: ObjectId) -> Result<()>;
        fn r#get_touch(client: &mut Client, r#id: ObjectId) -> Result<()>;
        fn r#release(client: &mut Client) -> Result<()>;
        async fn r#capabilities(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#capabilities: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(capabilities).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#name(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#name: String,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_string(Some(name)).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#WlPointer {
        const INTERFACE: &'static str = "wl_pointer";
        const VERSION: u32 = 9;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#set_cursor(
                    client,
                    message.uint()?,
                    message.object()?,
                    message.int()?,
                    message.int()?,
                ),
                1 => Self::r#release(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#set_cursor(
            client: &mut Client,
            r#serial: u32,
            r#surface: Option<ObjectId>,
            r#hotspot_x: i32,
            r#hotspot_y: i32,
        ) -> Result<()>;
        fn r#release(client: &mut Client) -> Result<()>;
        async fn r#enter(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#serial: u32,
            r#surface: ObjectId,
            r#surface_x: Fixed,
            r#surface_y: Fixed,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(serial)
                .put_object(Some(surface))
                .put_fixed(surface_x)
                .put_fixed(surface_y)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#leave(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#serial: u32,
            r#surface: ObjectId,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(serial)
                .put_object(Some(surface))
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#motion(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#time: u32,
            r#surface_x: Fixed,
            r#surface_y: Fixed,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(time)
                .put_fixed(surface_x)
                .put_fixed(surface_y)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#button(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#serial: u32,
            r#time: u32,
            r#button: u32,
            r#state: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(serial)
                .put_uint(time)
                .put_uint(button)
                .put_uint(state)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#axis(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#time: u32,
            r#axis: u32,
            r#value: Fixed,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(time)
                .put_uint(axis)
                .put_fixed(value)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#frame(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#axis_source(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#axis_source: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(axis_source).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#axis_stop(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#time: u32,
            r#axis: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(time).put_uint(axis).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#axis_discrete(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#axis: u32,
            r#discrete: i32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(axis)
                .put_int(discrete)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#axis_value120(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#axis: u32,
            r#value120: i32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(axis)
                .put_int(value120)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#axis_relative_direction(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#axis: u32,
            r#direction: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(axis)
                .put_uint(direction)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#WlKeyboard {
        const INTERFACE: &'static str = "wl_keyboard";
        const VERSION: u32 = 9;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#release(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#release(client: &mut Client) -> Result<()>;
        async fn r#keymap(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#format: u32,
            r#fd: RawFd,
            r#size: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(format)
                .put_int(fd)
                .put_uint(size)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#enter(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#serial: u32,
            r#surface: ObjectId,
            r#keys: Vec<u8>,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(serial)
                .put_object(Some(surface))
                .put_array(keys)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#leave(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#serial: u32,
            r#surface: ObjectId,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(serial)
                .put_object(Some(surface))
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#key(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#serial: u32,
            r#time: u32,
            r#key: u32,
            r#state: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(serial)
                .put_uint(time)
                .put_uint(key)
                .put_uint(state)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#modifiers(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#serial: u32,
            r#mods_depressed: u32,
            r#mods_latched: u32,
            r#mods_locked: u32,
            r#group: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(serial)
                .put_uint(mods_depressed)
                .put_uint(mods_latched)
                .put_uint(mods_locked)
                .put_uint(group)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#repeat_info(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#rate: i32,
            r#delay: i32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_int(rate).put_int(delay).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#WlTouch {
        const INTERFACE: &'static str = "wl_touch";
        const VERSION: u32 = 9;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#release(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#release(client: &mut Client) -> Result<()>;
        async fn r#down(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#serial: u32,
            r#time: u32,
            r#surface: ObjectId,
            r#id: i32,
            r#x: Fixed,
            r#y: Fixed,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(serial)
                .put_uint(time)
                .put_object(Some(surface))
                .put_int(id)
                .put_fixed(x)
                .put_fixed(y)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#up(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#serial: u32,
            r#time: u32,
            r#id: i32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(serial)
                .put_uint(time)
                .put_int(id)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#motion(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#time: u32,
            r#id: i32,
            r#x: Fixed,
            r#y: Fixed,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(time)
                .put_int(id)
                .put_fixed(x)
                .put_fixed(y)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#frame(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#cancel(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#shape(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#id: i32,
            r#major: Fixed,
            r#minor: Fixed,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_int(id)
                .put_fixed(major)
                .put_fixed(minor)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#orientation(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#id: i32,
            r#orientation: Fixed,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_int(id)
                .put_fixed(orientation)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#WlOutput {
        const INTERFACE: &'static str = "wl_output";
        const VERSION: u32 = 4;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#release(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#release(client: &mut Client) -> Result<()>;
        async fn r#geometry(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#x: i32,
            r#y: i32,
            r#physical_width: i32,
            r#physical_height: i32,
            r#subpixel: i32,
            r#make: String,
            r#model: String,
            r#transform: i32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_int(x)
                .put_int(y)
                .put_int(physical_width)
                .put_int(physical_height)
                .put_int(subpixel)
                .put_string(Some(make))
                .put_string(Some(model))
                .put_int(transform)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#mode(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#flags: u32,
            r#width: i32,
            r#height: i32,
            r#refresh: i32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(flags)
                .put_int(width)
                .put_int(height)
                .put_int(refresh)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#done(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#scale(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#factor: i32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_int(factor).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#name(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#name: String,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_string(Some(name)).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#description(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#description: String,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_string(Some(description)).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#WlRegion {
        const INTERFACE: &'static str = "wl_region";
        const VERSION: u32 = 1;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(client),
                1 => Self::r#add(
                    client,
                    message.int()?,
                    message.int()?,
                    message.int()?,
                    message.int()?,
                ),
                2 => Self::r#subtract(
                    client,
                    message.int()?,
                    message.int()?,
                    message.int()?,
                    message.int()?,
                ),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#add(
            client: &mut Client,
            r#x: i32,
            r#y: i32,
            r#width: i32,
            r#height: i32,
        ) -> Result<()>;
        fn r#subtract(
            client: &mut Client,
            r#x: i32,
            r#y: i32,
            r#width: i32,
            r#height: i32,
        ) -> Result<()>;
    }
    pub trait r#WlSubcompositor {
        const INTERFACE: &'static str = "wl_subcompositor";
        const VERSION: u32 = 1;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(client),
                1 => Self::r#get_subsurface(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#get_subsurface(
            client: &mut Client,
            r#id: ObjectId,
            r#surface: ObjectId,
            r#parent: ObjectId,
        ) -> Result<()>;
    }
    pub trait r#WlSubsurface {
        const INTERFACE: &'static str = "wl_subsurface";
        const VERSION: u32 = 1;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(client),
                1 => Self::r#set_position(client, message.int()?, message.int()?),
                2 => Self::r#place_above(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                3 => Self::r#place_below(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                4 => Self::r#set_sync(client),
                5 => Self::r#set_desync(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#set_position(client: &mut Client, r#x: i32, r#y: i32) -> Result<()>;
        fn r#place_above(client: &mut Client, r#sibling: ObjectId) -> Result<()>;
        fn r#place_below(client: &mut Client, r#sibling: ObjectId) -> Result<()>;
        fn r#set_sync(client: &mut Client) -> Result<()>;
        fn r#set_desync(client: &mut Client) -> Result<()>;
    }
}
pub mod linux_dmabuf_v1 {
    use crate::{
        error::Error,
        message::{DecodeError, Fixed, Message, NewId, ObjectId, PayloadBuilder},
        Client, Dispatcher, Result,
    };
    use std::os::fd::RawFd;
    pub trait r#ZwpLinuxDmabufV1 {
        const INTERFACE: &'static str = "zwp_linux_dmabuf_v1";
        const VERSION: u32 = 5;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(client),
                1 => Self::r#create_params(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                2 => Self::r#get_default_feedback(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                3 => Self::r#get_surface_feedback(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#create_params(client: &mut Client, r#params_id: ObjectId) -> Result<()>;
        fn r#get_default_feedback(client: &mut Client, r#id: ObjectId) -> Result<()>;
        fn r#get_surface_feedback(
            client: &mut Client,
            r#id: ObjectId,
            r#surface: ObjectId,
        ) -> Result<()>;
        async fn r#format(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#format: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(format).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#modifier(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#format: u32,
            r#modifier_hi: u32,
            r#modifier_lo: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(format)
                .put_uint(modifier_hi)
                .put_uint(modifier_lo)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#ZwpLinuxBufferParamsV1 {
        const INTERFACE: &'static str = "zwp_linux_buffer_params_v1";
        const VERSION: u32 = 5;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(client),
                1 => Self::r#add(
                    client,
                    message.int()?,
                    message.uint()?,
                    message.uint()?,
                    message.uint()?,
                    message.uint()?,
                    message.uint()?,
                ),
                2 => Self::r#create(
                    client,
                    message.int()?,
                    message.int()?,
                    message.uint()?,
                    message.uint()?,
                ),
                3 => Self::r#create_immed(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.int()?,
                    message.int()?,
                    message.uint()?,
                    message.uint()?,
                ),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#add(
            client: &mut Client,
            r#fd: RawFd,
            r#plane_idx: u32,
            r#offset: u32,
            r#stride: u32,
            r#modifier_hi: u32,
            r#modifier_lo: u32,
        ) -> Result<()>;
        fn r#create(
            client: &mut Client,
            r#width: i32,
            r#height: i32,
            r#format: u32,
            r#flags: u32,
        ) -> Result<()>;
        fn r#create_immed(
            client: &mut Client,
            r#buffer_id: ObjectId,
            r#width: i32,
            r#height: i32,
            r#format: u32,
            r#flags: u32,
        ) -> Result<()>;
        async fn r#created(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#buffer: ObjectId,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_object(Some(buffer)).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#failed(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#ZwpLinuxDmabufFeedbackV1 {
        const INTERFACE: &'static str = "zwp_linux_dmabuf_feedback_v1";
        const VERSION: u32 = 5;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#destroy(client: &mut Client) -> Result<()>;
        async fn r#done(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#format_table(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#fd: RawFd,
            r#size: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_int(fd).put_uint(size).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#main_device(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#device: Vec<u8>,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_array(device).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#tranche_done(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#tranche_target_device(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#device: Vec<u8>,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_array(device).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#tranche_formats(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#indices: Vec<u8>,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_array(indices).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#tranche_flags(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#flags: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(flags).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
}
pub mod presentation_time {
    use crate::{
        error::Error,
        message::{DecodeError, Fixed, Message, NewId, ObjectId, PayloadBuilder},
        Client, Dispatcher, Result,
    };
    use std::os::fd::RawFd;
    pub trait r#WpPresentation {
        const INTERFACE: &'static str = "wp_presentation";
        const VERSION: u32 = 1;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(client),
                1 => Self::r#feedback(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#feedback(client: &mut Client, r#surface: ObjectId, r#callback: ObjectId)
            -> Result<()>;
        async fn r#clock_id(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#clk_id: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(clk_id).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#WpPresentationFeedback {
        const INTERFACE: &'static str = "wp_presentation_feedback";
        const VERSION: u32 = 1;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        async fn r#sync_output(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#output: ObjectId,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_object(Some(output)).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#presented(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#tv_sec_hi: u32,
            r#tv_sec_lo: u32,
            r#tv_nsec: u32,
            r#refresh: u32,
            r#seq_hi: u32,
            r#seq_lo: u32,
            r#flags: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(tv_sec_hi)
                .put_uint(tv_sec_lo)
                .put_uint(tv_nsec)
                .put_uint(refresh)
                .put_uint(seq_hi)
                .put_uint(seq_lo)
                .put_uint(flags)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#discarded(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
}
pub mod tablet_v2 {
    use crate::{
        error::Error,
        message::{DecodeError, Fixed, Message, NewId, ObjectId, PayloadBuilder},
        Client, Dispatcher, Result,
    };
    use std::os::fd::RawFd;
    pub trait r#ZwpTabletManagerV2 {
        const INTERFACE: &'static str = "zwp_tablet_manager_v2";
        const VERSION: u32 = 1;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#get_tablet_seat(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                1 => Self::r#destroy(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#get_tablet_seat(
            client: &mut Client,
            r#tablet_seat: ObjectId,
            r#seat: ObjectId,
        ) -> Result<()>;
        fn r#destroy(client: &mut Client) -> Result<()>;
    }
    pub trait r#ZwpTabletSeatV2 {
        const INTERFACE: &'static str = "zwp_tablet_seat_v2";
        const VERSION: u32 = 1;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#destroy(client: &mut Client) -> Result<()>;
        async fn r#tablet_added(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#id: ObjectId,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_object(Some(id)).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#tool_added(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#id: ObjectId,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_object(Some(id)).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#pad_added(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#id: ObjectId,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_object(Some(id)).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#ZwpTabletToolV2 {
        const INTERFACE: &'static str = "zwp_tablet_tool_v2";
        const VERSION: u32 = 1;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#set_cursor(
                    client,
                    message.uint()?,
                    message.object()?,
                    message.int()?,
                    message.int()?,
                ),
                1 => Self::r#destroy(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#set_cursor(
            client: &mut Client,
            r#serial: u32,
            r#surface: Option<ObjectId>,
            r#hotspot_x: i32,
            r#hotspot_y: i32,
        ) -> Result<()>;
        fn r#destroy(client: &mut Client) -> Result<()>;
        async fn r#type(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#tool_type: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(tool_type).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#hardware_serial(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#hardware_serial_hi: u32,
            r#hardware_serial_lo: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(hardware_serial_hi)
                .put_uint(hardware_serial_lo)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#hardware_id_wacom(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#hardware_id_hi: u32,
            r#hardware_id_lo: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(hardware_id_hi)
                .put_uint(hardware_id_lo)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#capability(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#capability: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(capability).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#done(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#removed(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#proximity_in(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#serial: u32,
            r#tablet: ObjectId,
            r#surface: ObjectId,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(serial)
                .put_object(Some(tablet))
                .put_object(Some(surface))
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#proximity_out(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#down(dispatcher_id: ObjectId, client: &mut Client, r#serial: u32) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(serial).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#up(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#motion(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#x: Fixed,
            r#y: Fixed,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_fixed(x).put_fixed(y).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#pressure(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#pressure: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(pressure).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#distance(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#distance: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(distance).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#tilt(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#tilt_x: Fixed,
            r#tilt_y: Fixed,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_fixed(tilt_x)
                .put_fixed(tilt_y)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#rotation(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#degrees: Fixed,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_fixed(degrees).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#slider(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#position: i32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_int(position).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#wheel(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#degrees: Fixed,
            r#clicks: i32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_fixed(degrees)
                .put_int(clicks)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#button(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#serial: u32,
            r#button: u32,
            r#state: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(serial)
                .put_uint(button)
                .put_uint(state)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#frame(dispatcher_id: ObjectId, client: &mut Client, r#time: u32) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(time).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#ZwpTabletV2 {
        const INTERFACE: &'static str = "zwp_tablet_v2";
        const VERSION: u32 = 1;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#destroy(client: &mut Client) -> Result<()>;
        async fn r#name(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#name: String,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_string(Some(name)).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#id(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#vid: u32,
            r#pid: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(vid).put_uint(pid).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#path(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#path: String,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_string(Some(path)).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#done(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#removed(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#ZwpTabletPadRingV2 {
        const INTERFACE: &'static str = "zwp_tablet_pad_ring_v2";
        const VERSION: u32 = 1;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#set_feedback(
                    client,
                    message.string()?.ok_or(DecodeError::MalformedPayload)?,
                    message.uint()?,
                ),
                1 => Self::r#destroy(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#set_feedback(client: &mut Client, r#description: String, r#serial: u32) -> Result<()>;
        fn r#destroy(client: &mut Client) -> Result<()>;
        async fn r#source(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#source: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(source).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#angle(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#degrees: Fixed,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_fixed(degrees).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#stop(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#frame(dispatcher_id: ObjectId, client: &mut Client, r#time: u32) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(time).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#ZwpTabletPadStripV2 {
        const INTERFACE: &'static str = "zwp_tablet_pad_strip_v2";
        const VERSION: u32 = 1;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#set_feedback(
                    client,
                    message.string()?.ok_or(DecodeError::MalformedPayload)?,
                    message.uint()?,
                ),
                1 => Self::r#destroy(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#set_feedback(client: &mut Client, r#description: String, r#serial: u32) -> Result<()>;
        fn r#destroy(client: &mut Client) -> Result<()>;
        async fn r#source(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#source: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(source).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#position(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#position: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(position).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#stop(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#frame(dispatcher_id: ObjectId, client: &mut Client, r#time: u32) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(time).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#ZwpTabletPadGroupV2 {
        const INTERFACE: &'static str = "zwp_tablet_pad_group_v2";
        const VERSION: u32 = 1;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#destroy(client: &mut Client) -> Result<()>;
        async fn r#buttons(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#buttons: Vec<u8>,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_array(buttons).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#ring(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#ring: ObjectId,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_object(Some(ring)).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#strip(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#strip: ObjectId,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_object(Some(strip)).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#modes(dispatcher_id: ObjectId, client: &mut Client, r#modes: u32) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(modes).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#done(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#mode_switch(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#time: u32,
            r#serial: u32,
            r#mode: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(time)
                .put_uint(serial)
                .put_uint(mode)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#ZwpTabletPadV2 {
        const INTERFACE: &'static str = "zwp_tablet_pad_v2";
        const VERSION: u32 = 1;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#set_feedback(
                    client,
                    message.uint()?,
                    message.string()?.ok_or(DecodeError::MalformedPayload)?,
                    message.uint()?,
                ),
                1 => Self::r#destroy(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#set_feedback(
            client: &mut Client,
            r#button: u32,
            r#description: String,
            r#serial: u32,
        ) -> Result<()>;
        fn r#destroy(client: &mut Client) -> Result<()>;
        async fn r#group(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#pad_group: ObjectId,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_object(Some(pad_group)).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#path(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#path: String,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_string(Some(path)).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#buttons(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#buttons: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(buttons).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#done(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#button(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#time: u32,
            r#button: u32,
            r#state: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(time)
                .put_uint(button)
                .put_uint(state)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#enter(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#serial: u32,
            r#tablet: ObjectId,
            r#surface: ObjectId,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(serial)
                .put_object(Some(tablet))
                .put_object(Some(surface))
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#leave(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#serial: u32,
            r#surface: ObjectId,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_uint(serial)
                .put_object(Some(surface))
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#removed(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
}
pub mod viewporter {
    use crate::{
        error::Error,
        message::{DecodeError, Fixed, Message, NewId, ObjectId, PayloadBuilder},
        Client, Dispatcher, Result,
    };
    use std::os::fd::RawFd;
    pub trait r#WpViewporter {
        const INTERFACE: &'static str = "wp_viewporter";
        const VERSION: u32 = 1;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(client),
                1 => Self::r#get_viewport(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#get_viewport(client: &mut Client, r#id: ObjectId, r#surface: ObjectId) -> Result<()>;
    }
    pub trait r#WpViewport {
        const INTERFACE: &'static str = "wp_viewport";
        const VERSION: u32 = 1;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(client),
                1 => Self::r#set_source(
                    client,
                    message.fixed()?,
                    message.fixed()?,
                    message.fixed()?,
                    message.fixed()?,
                ),
                2 => Self::r#set_destination(client, message.int()?, message.int()?),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#set_source(
            client: &mut Client,
            r#x: Fixed,
            r#y: Fixed,
            r#width: Fixed,
            r#height: Fixed,
        ) -> Result<()>;
        fn r#set_destination(client: &mut Client, r#width: i32, r#height: i32) -> Result<()>;
    }
}
pub mod xdg_shell {
    use crate::{
        error::Error,
        message::{DecodeError, Fixed, Message, NewId, ObjectId, PayloadBuilder},
        Client, Dispatcher, Result,
    };
    use std::os::fd::RawFd;
    pub trait r#XdgWmBase {
        const INTERFACE: &'static str = "xdg_wm_base";
        const VERSION: u32 = 6;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(client),
                1 => Self::r#create_positioner(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                2 => Self::r#get_xdg_surface(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                3 => Self::r#pong(client, message.uint()?),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#create_positioner(client: &mut Client, r#id: ObjectId) -> Result<()>;
        fn r#get_xdg_surface(
            client: &mut Client,
            r#id: ObjectId,
            r#surface: ObjectId,
        ) -> Result<()>;
        fn r#pong(client: &mut Client, r#serial: u32) -> Result<()>;
        async fn r#ping(dispatcher_id: ObjectId, client: &mut Client, r#serial: u32) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(serial).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#XdgPositioner {
        const INTERFACE: &'static str = "xdg_positioner";
        const VERSION: u32 = 6;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(client),
                1 => Self::r#set_size(client, message.int()?, message.int()?),
                2 => Self::r#set_anchor_rect(
                    client,
                    message.int()?,
                    message.int()?,
                    message.int()?,
                    message.int()?,
                ),
                3 => Self::r#set_anchor(client, message.uint()?),
                4 => Self::r#set_gravity(client, message.uint()?),
                5 => Self::r#set_constraint_adjustment(client, message.uint()?),
                6 => Self::r#set_offset(client, message.int()?, message.int()?),
                7 => Self::r#set_reactive(client),
                8 => Self::r#set_parent_size(client, message.int()?, message.int()?),
                9 => Self::r#set_parent_configure(client, message.uint()?),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#set_size(client: &mut Client, r#width: i32, r#height: i32) -> Result<()>;
        fn r#set_anchor_rect(
            client: &mut Client,
            r#x: i32,
            r#y: i32,
            r#width: i32,
            r#height: i32,
        ) -> Result<()>;
        fn r#set_anchor(client: &mut Client, r#anchor: u32) -> Result<()>;
        fn r#set_gravity(client: &mut Client, r#gravity: u32) -> Result<()>;
        fn r#set_constraint_adjustment(
            client: &mut Client,
            r#constraint_adjustment: u32,
        ) -> Result<()>;
        fn r#set_offset(client: &mut Client, r#x: i32, r#y: i32) -> Result<()>;
        fn r#set_reactive(client: &mut Client) -> Result<()>;
        fn r#set_parent_size(
            client: &mut Client,
            r#parent_width: i32,
            r#parent_height: i32,
        ) -> Result<()>;
        fn r#set_parent_configure(client: &mut Client, r#serial: u32) -> Result<()>;
    }
    pub trait r#XdgSurface {
        const INTERFACE: &'static str = "xdg_surface";
        const VERSION: u32 = 6;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(client),
                1 => Self::r#get_toplevel(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                2 => Self::r#get_popup(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.object()?,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                3 => Self::r#set_window_geometry(
                    client,
                    message.int()?,
                    message.int()?,
                    message.int()?,
                    message.int()?,
                ),
                4 => Self::r#ack_configure(client, message.uint()?),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#get_toplevel(client: &mut Client, r#id: ObjectId) -> Result<()>;
        fn r#get_popup(
            client: &mut Client,
            r#id: ObjectId,
            r#parent: Option<ObjectId>,
            r#positioner: ObjectId,
        ) -> Result<()>;
        fn r#set_window_geometry(
            client: &mut Client,
            r#x: i32,
            r#y: i32,
            r#width: i32,
            r#height: i32,
        ) -> Result<()>;
        fn r#ack_configure(client: &mut Client, r#serial: u32) -> Result<()>;
        async fn r#configure(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#serial: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(serial).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#XdgToplevel {
        const INTERFACE: &'static str = "xdg_toplevel";
        const VERSION: u32 = 6;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(client),
                1 => Self::r#set_parent(client, message.object()?),
                2 => Self::r#set_title(
                    client,
                    message.string()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                3 => Self::r#set_app_id(
                    client,
                    message.string()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                4 => Self::r#show_window_menu(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.uint()?,
                    message.int()?,
                    message.int()?,
                ),
                5 => Self::r#move(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.uint()?,
                ),
                6 => Self::r#resize(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.uint()?,
                    message.uint()?,
                ),
                7 => Self::r#set_max_size(client, message.int()?, message.int()?),
                8 => Self::r#set_min_size(client, message.int()?, message.int()?),
                9 => Self::r#set_maximized(client),
                10 => Self::r#unset_maximized(client),
                11 => Self::r#set_fullscreen(client, message.object()?),
                12 => Self::r#unset_fullscreen(client),
                13 => Self::r#set_minimized(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#set_parent(client: &mut Client, r#parent: Option<ObjectId>) -> Result<()>;
        fn r#set_title(client: &mut Client, r#title: String) -> Result<()>;
        fn r#set_app_id(client: &mut Client, r#app_id: String) -> Result<()>;
        fn r#show_window_menu(
            client: &mut Client,
            r#seat: ObjectId,
            r#serial: u32,
            r#x: i32,
            r#y: i32,
        ) -> Result<()>;
        fn r#move(client: &mut Client, r#seat: ObjectId, r#serial: u32) -> Result<()>;
        fn r#resize(
            client: &mut Client,
            r#seat: ObjectId,
            r#serial: u32,
            r#edges: u32,
        ) -> Result<()>;
        fn r#set_max_size(client: &mut Client, r#width: i32, r#height: i32) -> Result<()>;
        fn r#set_min_size(client: &mut Client, r#width: i32, r#height: i32) -> Result<()>;
        fn r#set_maximized(client: &mut Client) -> Result<()>;
        fn r#unset_maximized(client: &mut Client) -> Result<()>;
        fn r#set_fullscreen(client: &mut Client, r#output: Option<ObjectId>) -> Result<()>;
        fn r#unset_fullscreen(client: &mut Client) -> Result<()>;
        fn r#set_minimized(client: &mut Client) -> Result<()>;
        async fn r#configure(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#width: i32,
            r#height: i32,
            r#states: Vec<u8>,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_int(width)
                .put_int(height)
                .put_array(states)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#close(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#configure_bounds(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#width: i32,
            r#height: i32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_int(width).put_int(height).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#wm_capabilities(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#capabilities: Vec<u8>,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_array(capabilities).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
    pub trait r#XdgPopup {
        const INTERFACE: &'static str = "xdg_popup";
        const VERSION: u32 = 6;
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(client),
                1 => Self::r#grab(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.uint()?,
                ),
                2 => Self::r#reposition(
                    client,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.uint()?,
                ),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn create_dispatcher(id: ObjectId) -> Dispatcher {
            Dispatcher {
                dipatch_fn: Self::handle_request,
                id,
            }
        }
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#grab(client: &mut Client, r#seat: ObjectId, r#serial: u32) -> Result<()>;
        fn r#reposition(client: &mut Client, r#positioner: ObjectId, r#token: u32) -> Result<()>;
        async fn r#configure(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#x: i32,
            r#y: i32,
            r#width: i32,
            r#height: i32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new()
                .put_int(x)
                .put_int(y)
                .put_int(width)
                .put_int(height)
                .build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#popup_done(dispatcher_id: ObjectId, client: &mut Client) -> Result<()> {
            let payload = PayloadBuilder::new().build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
        async fn r#repositioned(
            dispatcher_id: ObjectId,
            client: &mut Client,
            r#token: u32,
        ) -> Result<()> {
            let payload = PayloadBuilder::new().put_uint(token).build();
            client
                .send_message(Message::new(dispatcher_id, 0, payload))
                .await
                .map_err(Error::IoError)
        }
    }
}
