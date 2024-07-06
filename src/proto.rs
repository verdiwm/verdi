#![allow(unused)]
pub mod wayland {
    use crate::{
        error::Error,
        message::{DecodeError, Fixed, Message, NewId, ObjectId},
        Client, Result,
    };
    use std::os::fd::RawFd;
    pub trait r#WlDisplay {
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
        fn r#sync(client: &mut Client, r#callback: ObjectId) -> Result<()>;
        fn r#get_registry(client: &mut Client, r#registry: ObjectId) -> Result<()>;
    }
    pub trait r#WlRegistry {
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#bind(client, message.uint()?, message.new_id()?),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#bind(client: &mut Client, r#name: u32, r#id: NewId) -> Result<()>;
    }
    pub trait r#WlCallback {
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                _ => Err(Error::UnknownOpcode),
            }
        }
    }
    pub trait r#WlCompositor {
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
        fn r#create_surface(client: &mut Client, r#id: ObjectId) -> Result<()>;
        fn r#create_region(client: &mut Client, r#id: ObjectId) -> Result<()>;
    }
    pub trait r#WlShmPool {
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
        fn r#create_pool(
            client: &mut Client,
            r#id: ObjectId,
            r#fd: RawFd,
            r#size: i32,
        ) -> Result<()>;
        fn r#release(client: &mut Client) -> Result<()>;
    }
    pub trait r#WlBuffer {
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy(client: &mut Client) -> Result<()>;
    }
    pub trait r#WlDataOffer {
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
        fn r#accept(client: &mut Client, r#serial: u32, r#mime_type: Option<String>) -> Result<()>;
        fn r#receive(client: &mut Client, r#mime_type: String, r#fd: RawFd) -> Result<()>;
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#finish(client: &mut Client) -> Result<()>;
        fn r#set_actions(
            client: &mut Client,
            r#dnd_actions: u32,
            r#preferred_action: u32,
        ) -> Result<()>;
    }
    pub trait r#WlDataSource {
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
        fn r#offer(client: &mut Client, r#mime_type: String) -> Result<()>;
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#set_actions(client: &mut Client, r#dnd_actions: u32) -> Result<()>;
    }
    pub trait r#WlDataDevice {
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
    }
    pub trait r#WlDataDeviceManager {
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
        fn r#create_data_source(client: &mut Client, r#id: ObjectId) -> Result<()>;
        fn r#get_data_device(client: &mut Client, r#id: ObjectId, r#seat: ObjectId) -> Result<()>;
    }
    pub trait r#WlShell {
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
        fn r#get_shell_surface(
            client: &mut Client,
            r#id: ObjectId,
            r#surface: ObjectId,
        ) -> Result<()>;
    }
    pub trait r#WlShellSurface {
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
    }
    pub trait r#WlSurface {
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
    }
    pub trait r#WlSeat {
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
        fn r#get_pointer(client: &mut Client, r#id: ObjectId) -> Result<()>;
        fn r#get_keyboard(client: &mut Client, r#id: ObjectId) -> Result<()>;
        fn r#get_touch(client: &mut Client, r#id: ObjectId) -> Result<()>;
        fn r#release(client: &mut Client) -> Result<()>;
    }
    pub trait r#WlPointer {
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
        fn r#set_cursor(
            client: &mut Client,
            r#serial: u32,
            r#surface: Option<ObjectId>,
            r#hotspot_x: i32,
            r#hotspot_y: i32,
        ) -> Result<()>;
        fn r#release(client: &mut Client) -> Result<()>;
    }
    pub trait r#WlKeyboard {
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#release(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#release(client: &mut Client) -> Result<()>;
    }
    pub trait r#WlTouch {
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#release(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#release(client: &mut Client) -> Result<()>;
    }
    pub trait r#WlOutput {
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#release(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#release(client: &mut Client) -> Result<()>;
    }
    pub trait r#WlRegion {
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
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#get_subsurface(
            client: &mut Client,
            r#id: ObjectId,
            r#surface: ObjectId,
            r#parent: ObjectId,
        ) -> Result<()>;
    }
    pub trait r#WlSubsurface {
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
        message::{DecodeError, Fixed, Message, NewId, ObjectId},
        Client, Result,
    };
    use std::os::fd::RawFd;
    pub trait r#ZwpLinuxDmabufV1 {
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
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#create_params(client: &mut Client, r#params_id: ObjectId) -> Result<()>;
        fn r#get_default_feedback(client: &mut Client, r#id: ObjectId) -> Result<()>;
        fn r#get_surface_feedback(
            client: &mut Client,
            r#id: ObjectId,
            r#surface: ObjectId,
        ) -> Result<()>;
    }
    pub trait r#ZwpLinuxBufferParamsV1 {
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
    }
    pub trait r#ZwpLinuxDmabufFeedbackV1 {
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy(client: &mut Client) -> Result<()>;
    }
}
pub mod presentation_time {
    use crate::{
        error::Error,
        message::{DecodeError, Fixed, Message, NewId, ObjectId},
        Client, Result,
    };
    use std::os::fd::RawFd;
    pub trait r#WpPresentation {
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
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#feedback(client: &mut Client, r#surface: ObjectId, r#callback: ObjectId)
            -> Result<()>;
    }
    pub trait r#WpPresentationFeedback {
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                _ => Err(Error::UnknownOpcode),
            }
        }
    }
}
pub mod tablet_v2 {
    use crate::{
        error::Error,
        message::{DecodeError, Fixed, Message, NewId, ObjectId},
        Client, Result,
    };
    use std::os::fd::RawFd;
    pub trait r#ZwpTabletManagerV2 {
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
        fn r#get_tablet_seat(
            client: &mut Client,
            r#tablet_seat: ObjectId,
            r#seat: ObjectId,
        ) -> Result<()>;
        fn r#destroy(client: &mut Client) -> Result<()>;
    }
    pub trait r#ZwpTabletSeatV2 {
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy(client: &mut Client) -> Result<()>;
    }
    pub trait r#ZwpTabletToolV2 {
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
        fn r#set_cursor(
            client: &mut Client,
            r#serial: u32,
            r#surface: Option<ObjectId>,
            r#hotspot_x: i32,
            r#hotspot_y: i32,
        ) -> Result<()>;
        fn r#destroy(client: &mut Client) -> Result<()>;
    }
    pub trait r#ZwpTabletV2 {
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy(client: &mut Client) -> Result<()>;
    }
    pub trait r#ZwpTabletPadRingV2 {
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
        fn r#set_feedback(client: &mut Client, r#description: String, r#serial: u32) -> Result<()>;
        fn r#destroy(client: &mut Client) -> Result<()>;
    }
    pub trait r#ZwpTabletPadStripV2 {
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
        fn r#set_feedback(client: &mut Client, r#description: String, r#serial: u32) -> Result<()>;
        fn r#destroy(client: &mut Client) -> Result<()>;
    }
    pub trait r#ZwpTabletPadGroupV2 {
        fn handle_request(client: &mut Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(client),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy(client: &mut Client) -> Result<()>;
    }
    pub trait r#ZwpTabletPadV2 {
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
        fn r#set_feedback(
            client: &mut Client,
            r#button: u32,
            r#description: String,
            r#serial: u32,
        ) -> Result<()>;
        fn r#destroy(client: &mut Client) -> Result<()>;
    }
}
pub mod viewporter {
    use crate::{
        error::Error,
        message::{DecodeError, Fixed, Message, NewId, ObjectId},
        Client, Result,
    };
    use std::os::fd::RawFd;
    pub trait r#WpViewporter {
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
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#get_viewport(client: &mut Client, r#id: ObjectId, r#surface: ObjectId) -> Result<()>;
    }
    pub trait r#WpViewport {
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
        message::{DecodeError, Fixed, Message, NewId, ObjectId},
        Client, Result,
    };
    use std::os::fd::RawFd;
    pub trait r#XdgWmBase {
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
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#create_positioner(client: &mut Client, r#id: ObjectId) -> Result<()>;
        fn r#get_xdg_surface(
            client: &mut Client,
            r#id: ObjectId,
            r#surface: ObjectId,
        ) -> Result<()>;
        fn r#pong(client: &mut Client, r#serial: u32) -> Result<()>;
    }
    pub trait r#XdgPositioner {
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
    }
    pub trait r#XdgToplevel {
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
    }
    pub trait r#XdgPopup {
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
        fn r#destroy(client: &mut Client) -> Result<()>;
        fn r#grab(client: &mut Client, r#seat: ObjectId, r#serial: u32) -> Result<()>;
        fn r#reposition(client: &mut Client, r#positioner: ObjectId, r#token: u32) -> Result<()>;
    }
}
