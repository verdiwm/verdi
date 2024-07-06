#![allow(unused)]
pub mod wayland {
    use crate::{
        error::Error,
        message::{DecodeError, Fixed, Message, NewId, ObjectId},
        Client, Result,
    };
    use std::os::fd::RawFd;
    pub trait r#WlDisplay {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#sync(message.object()?.ok_or(DecodeError::MalformedPayload)?),
                1 => Self::r#get_registry(message.object()?.ok_or(DecodeError::MalformedPayload)?),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#sync(r#callback: ObjectId) -> Result<()>;
        fn r#get_registry(r#registry: ObjectId) -> Result<()>;
    }
    pub trait r#WlRegistry {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#bind(message.uint()?, message.new_id()?),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#bind(r#name: u32, r#id: NewId) -> Result<()>;
    }
    pub trait r#WlCallback {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                _ => Err(Error::UnknownOpcode),
            }
        }
    }
    pub trait r#WlCompositor {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => {
                    Self::r#create_surface(message.object()?.ok_or(DecodeError::MalformedPayload)?)
                }
                1 => Self::r#create_region(message.object()?.ok_or(DecodeError::MalformedPayload)?),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#create_surface(r#id: ObjectId) -> Result<()>;
        fn r#create_region(r#id: ObjectId) -> Result<()>;
    }
    pub trait r#WlShmPool {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#create_buffer(
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.int()?,
                    message.int()?,
                    message.int()?,
                    message.int()?,
                    message.uint()?,
                ),
                1 => Self::r#destroy(),
                2 => Self::r#resize(message.int()?),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#create_buffer(
            r#id: ObjectId,
            r#offset: i32,
            r#width: i32,
            r#height: i32,
            r#stride: i32,
            r#format: u32,
        ) -> Result<()>;
        fn r#destroy() -> Result<()>;
        fn r#resize(r#size: i32) -> Result<()>;
    }
    pub trait r#WlShm {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#create_pool(
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.int()?,
                    message.int()?,
                ),
                1 => Self::r#release(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#create_pool(r#id: ObjectId, r#fd: RawFd, r#size: i32) -> Result<()>;
        fn r#release() -> Result<()>;
    }
    pub trait r#WlBuffer {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
    }
    pub trait r#WlDataOffer {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#accept(message.uint()?, message.string()?),
                1 => Self::r#receive(
                    message.string()?.ok_or(DecodeError::MalformedPayload)?,
                    message.int()?,
                ),
                2 => Self::r#destroy(),
                3 => Self::r#finish(),
                4 => Self::r#set_actions(message.uint()?, message.uint()?),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#accept(r#serial: u32, r#mime_type: Option<String>) -> Result<()>;
        fn r#receive(r#mime_type: String, r#fd: RawFd) -> Result<()>;
        fn r#destroy() -> Result<()>;
        fn r#finish() -> Result<()>;
        fn r#set_actions(r#dnd_actions: u32, r#preferred_action: u32) -> Result<()>;
    }
    pub trait r#WlDataSource {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#offer(message.string()?.ok_or(DecodeError::MalformedPayload)?),
                1 => Self::r#destroy(),
                2 => Self::r#set_actions(message.uint()?),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#offer(r#mime_type: String) -> Result<()>;
        fn r#destroy() -> Result<()>;
        fn r#set_actions(r#dnd_actions: u32) -> Result<()>;
    }
    pub trait r#WlDataDevice {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#start_drag(
                    message.object()?,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.object()?,
                    message.uint()?,
                ),
                1 => Self::r#set_selection(message.object()?, message.uint()?),
                2 => Self::r#release(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#start_drag(
            r#source: Option<ObjectId>,
            r#origin: ObjectId,
            r#icon: Option<ObjectId>,
            r#serial: u32,
        ) -> Result<()>;
        fn r#set_selection(r#source: Option<ObjectId>, r#serial: u32) -> Result<()>;
        fn r#release() -> Result<()>;
    }
    pub trait r#WlDataDeviceManager {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#create_data_source(
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                1 => Self::r#get_data_device(
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#create_data_source(r#id: ObjectId) -> Result<()>;
        fn r#get_data_device(r#id: ObjectId, r#seat: ObjectId) -> Result<()>;
    }
    pub trait r#WlShell {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#get_shell_surface(
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#get_shell_surface(r#id: ObjectId, r#surface: ObjectId) -> Result<()>;
    }
    pub trait r#WlShellSurface {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#pong(message.uint()?),
                1 => Self::r#move(
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.uint()?,
                ),
                2 => Self::r#resize(
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.uint()?,
                    message.uint()?,
                ),
                3 => Self::r#set_toplevel(),
                4 => Self::r#set_transient(
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.int()?,
                    message.int()?,
                    message.uint()?,
                ),
                5 => Self::r#set_fullscreen(message.uint()?, message.uint()?, message.object()?),
                6 => Self::r#set_popup(
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.uint()?,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.int()?,
                    message.int()?,
                    message.uint()?,
                ),
                7 => Self::r#set_maximized(message.object()?),
                8 => Self::r#set_title(message.string()?.ok_or(DecodeError::MalformedPayload)?),
                9 => Self::r#set_class(message.string()?.ok_or(DecodeError::MalformedPayload)?),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#pong(r#serial: u32) -> Result<()>;
        fn r#move(r#seat: ObjectId, r#serial: u32) -> Result<()>;
        fn r#resize(r#seat: ObjectId, r#serial: u32, r#edges: u32) -> Result<()>;
        fn r#set_toplevel() -> Result<()>;
        fn r#set_transient(r#parent: ObjectId, r#x: i32, r#y: i32, r#flags: u32) -> Result<()>;
        fn r#set_fullscreen(
            r#method: u32,
            r#framerate: u32,
            r#output: Option<ObjectId>,
        ) -> Result<()>;
        fn r#set_popup(
            r#seat: ObjectId,
            r#serial: u32,
            r#parent: ObjectId,
            r#x: i32,
            r#y: i32,
            r#flags: u32,
        ) -> Result<()>;
        fn r#set_maximized(r#output: Option<ObjectId>) -> Result<()>;
        fn r#set_title(r#title: String) -> Result<()>;
        fn r#set_class(r#class: String) -> Result<()>;
    }
    pub trait r#WlSurface {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#attach(message.object()?, message.int()?, message.int()?),
                2 => Self::r#damage(
                    message.int()?,
                    message.int()?,
                    message.int()?,
                    message.int()?,
                ),
                3 => Self::r#frame(message.object()?.ok_or(DecodeError::MalformedPayload)?),
                4 => Self::r#set_opaque_region(message.object()?),
                5 => Self::r#set_input_region(message.object()?),
                6 => Self::r#commit(),
                7 => Self::r#set_buffer_transform(message.int()?),
                8 => Self::r#set_buffer_scale(message.int()?),
                9 => Self::r#damage_buffer(
                    message.int()?,
                    message.int()?,
                    message.int()?,
                    message.int()?,
                ),
                10 => Self::r#offset(message.int()?, message.int()?),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#attach(r#buffer: Option<ObjectId>, r#x: i32, r#y: i32) -> Result<()>;
        fn r#damage(r#x: i32, r#y: i32, r#width: i32, r#height: i32) -> Result<()>;
        fn r#frame(r#callback: ObjectId) -> Result<()>;
        fn r#set_opaque_region(r#region: Option<ObjectId>) -> Result<()>;
        fn r#set_input_region(r#region: Option<ObjectId>) -> Result<()>;
        fn r#commit() -> Result<()>;
        fn r#set_buffer_transform(r#transform: i32) -> Result<()>;
        fn r#set_buffer_scale(r#scale: i32) -> Result<()>;
        fn r#damage_buffer(r#x: i32, r#y: i32, r#width: i32, r#height: i32) -> Result<()>;
        fn r#offset(r#x: i32, r#y: i32) -> Result<()>;
    }
    pub trait r#WlSeat {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#get_pointer(message.object()?.ok_or(DecodeError::MalformedPayload)?),
                1 => Self::r#get_keyboard(message.object()?.ok_or(DecodeError::MalformedPayload)?),
                2 => Self::r#get_touch(message.object()?.ok_or(DecodeError::MalformedPayload)?),
                3 => Self::r#release(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#get_pointer(r#id: ObjectId) -> Result<()>;
        fn r#get_keyboard(r#id: ObjectId) -> Result<()>;
        fn r#get_touch(r#id: ObjectId) -> Result<()>;
        fn r#release() -> Result<()>;
    }
    pub trait r#WlPointer {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#set_cursor(
                    message.uint()?,
                    message.object()?,
                    message.int()?,
                    message.int()?,
                ),
                1 => Self::r#release(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#set_cursor(
            r#serial: u32,
            r#surface: Option<ObjectId>,
            r#hotspot_x: i32,
            r#hotspot_y: i32,
        ) -> Result<()>;
        fn r#release() -> Result<()>;
    }
    pub trait r#WlKeyboard {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#release(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#release() -> Result<()>;
    }
    pub trait r#WlTouch {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#release(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#release() -> Result<()>;
    }
    pub trait r#WlOutput {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#release(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#release() -> Result<()>;
    }
    pub trait r#WlRegion {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#add(
                    message.int()?,
                    message.int()?,
                    message.int()?,
                    message.int()?,
                ),
                2 => Self::r#subtract(
                    message.int()?,
                    message.int()?,
                    message.int()?,
                    message.int()?,
                ),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#add(r#x: i32, r#y: i32, r#width: i32, r#height: i32) -> Result<()>;
        fn r#subtract(r#x: i32, r#y: i32, r#width: i32, r#height: i32) -> Result<()>;
    }
    pub trait r#WlSubcompositor {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#get_subsurface(
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#get_subsurface(r#id: ObjectId, r#surface: ObjectId, r#parent: ObjectId) -> Result<()>;
    }
    pub trait r#WlSubsurface {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#set_position(message.int()?, message.int()?),
                2 => Self::r#place_above(message.object()?.ok_or(DecodeError::MalformedPayload)?),
                3 => Self::r#place_below(message.object()?.ok_or(DecodeError::MalformedPayload)?),
                4 => Self::r#set_sync(),
                5 => Self::r#set_desync(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#set_position(r#x: i32, r#y: i32) -> Result<()>;
        fn r#place_above(r#sibling: ObjectId) -> Result<()>;
        fn r#place_below(r#sibling: ObjectId) -> Result<()>;
        fn r#set_sync() -> Result<()>;
        fn r#set_desync() -> Result<()>;
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
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#create_params(message.object()?.ok_or(DecodeError::MalformedPayload)?),
                2 => Self::r#get_default_feedback(
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                3 => Self::r#get_surface_feedback(
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#create_params(r#params_id: ObjectId) -> Result<()>;
        fn r#get_default_feedback(r#id: ObjectId) -> Result<()>;
        fn r#get_surface_feedback(r#id: ObjectId, r#surface: ObjectId) -> Result<()>;
    }
    pub trait r#ZwpLinuxBufferParamsV1 {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#add(
                    message.int()?,
                    message.uint()?,
                    message.uint()?,
                    message.uint()?,
                    message.uint()?,
                    message.uint()?,
                ),
                2 => Self::r#create(
                    message.int()?,
                    message.int()?,
                    message.uint()?,
                    message.uint()?,
                ),
                3 => Self::r#create_immed(
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.int()?,
                    message.int()?,
                    message.uint()?,
                    message.uint()?,
                ),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#add(
            r#fd: RawFd,
            r#plane_idx: u32,
            r#offset: u32,
            r#stride: u32,
            r#modifier_hi: u32,
            r#modifier_lo: u32,
        ) -> Result<()>;
        fn r#create(r#width: i32, r#height: i32, r#format: u32, r#flags: u32) -> Result<()>;
        fn r#create_immed(
            r#buffer_id: ObjectId,
            r#width: i32,
            r#height: i32,
            r#format: u32,
            r#flags: u32,
        ) -> Result<()>;
    }
    pub trait r#ZwpLinuxDmabufFeedbackV1 {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
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
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#feedback(
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#feedback(r#surface: ObjectId, r#callback: ObjectId) -> Result<()>;
    }
    pub trait r#WpPresentationFeedback {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
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
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#get_tablet_seat(
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                1 => Self::r#destroy(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#get_tablet_seat(r#tablet_seat: ObjectId, r#seat: ObjectId) -> Result<()>;
        fn r#destroy() -> Result<()>;
    }
    pub trait r#ZwpTabletSeatV2 {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
    }
    pub trait r#ZwpTabletToolV2 {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#set_cursor(
                    message.uint()?,
                    message.object()?,
                    message.int()?,
                    message.int()?,
                ),
                1 => Self::r#destroy(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#set_cursor(
            r#serial: u32,
            r#surface: Option<ObjectId>,
            r#hotspot_x: i32,
            r#hotspot_y: i32,
        ) -> Result<()>;
        fn r#destroy() -> Result<()>;
    }
    pub trait r#ZwpTabletV2 {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
    }
    pub trait r#ZwpTabletPadRingV2 {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#set_feedback(
                    message.string()?.ok_or(DecodeError::MalformedPayload)?,
                    message.uint()?,
                ),
                1 => Self::r#destroy(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#set_feedback(r#description: String, r#serial: u32) -> Result<()>;
        fn r#destroy() -> Result<()>;
    }
    pub trait r#ZwpTabletPadStripV2 {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#set_feedback(
                    message.string()?.ok_or(DecodeError::MalformedPayload)?,
                    message.uint()?,
                ),
                1 => Self::r#destroy(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#set_feedback(r#description: String, r#serial: u32) -> Result<()>;
        fn r#destroy() -> Result<()>;
    }
    pub trait r#ZwpTabletPadGroupV2 {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
    }
    pub trait r#ZwpTabletPadV2 {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#set_feedback(
                    message.uint()?,
                    message.string()?.ok_or(DecodeError::MalformedPayload)?,
                    message.uint()?,
                ),
                1 => Self::r#destroy(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#set_feedback(r#button: u32, r#description: String, r#serial: u32) -> Result<()>;
        fn r#destroy() -> Result<()>;
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
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#get_viewport(
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#get_viewport(r#id: ObjectId, r#surface: ObjectId) -> Result<()>;
    }
    pub trait r#WpViewport {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#set_source(
                    message.fixed()?,
                    message.fixed()?,
                    message.fixed()?,
                    message.fixed()?,
                ),
                2 => Self::r#set_destination(message.int()?, message.int()?),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#set_source(r#x: Fixed, r#y: Fixed, r#width: Fixed, r#height: Fixed) -> Result<()>;
        fn r#set_destination(r#width: i32, r#height: i32) -> Result<()>;
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
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#create_positioner(
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                2 => Self::r#get_xdg_surface(
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                3 => Self::r#pong(message.uint()?),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#create_positioner(r#id: ObjectId) -> Result<()>;
        fn r#get_xdg_surface(r#id: ObjectId, r#surface: ObjectId) -> Result<()>;
        fn r#pong(r#serial: u32) -> Result<()>;
    }
    pub trait r#XdgPositioner {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#set_size(message.int()?, message.int()?),
                2 => Self::r#set_anchor_rect(
                    message.int()?,
                    message.int()?,
                    message.int()?,
                    message.int()?,
                ),
                3 => Self::r#set_anchor(message.uint()?),
                4 => Self::r#set_gravity(message.uint()?),
                5 => Self::r#set_constraint_adjustment(message.uint()?),
                6 => Self::r#set_offset(message.int()?, message.int()?),
                7 => Self::r#set_reactive(),
                8 => Self::r#set_parent_size(message.int()?, message.int()?),
                9 => Self::r#set_parent_configure(message.uint()?),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#set_size(r#width: i32, r#height: i32) -> Result<()>;
        fn r#set_anchor_rect(r#x: i32, r#y: i32, r#width: i32, r#height: i32) -> Result<()>;
        fn r#set_anchor(r#anchor: u32) -> Result<()>;
        fn r#set_gravity(r#gravity: u32) -> Result<()>;
        fn r#set_constraint_adjustment(r#constraint_adjustment: u32) -> Result<()>;
        fn r#set_offset(r#x: i32, r#y: i32) -> Result<()>;
        fn r#set_reactive() -> Result<()>;
        fn r#set_parent_size(r#parent_width: i32, r#parent_height: i32) -> Result<()>;
        fn r#set_parent_configure(r#serial: u32) -> Result<()>;
    }
    pub trait r#XdgSurface {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#get_toplevel(message.object()?.ok_or(DecodeError::MalformedPayload)?),
                2 => Self::r#get_popup(
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.object()?,
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                ),
                3 => Self::r#set_window_geometry(
                    message.int()?,
                    message.int()?,
                    message.int()?,
                    message.int()?,
                ),
                4 => Self::r#ack_configure(message.uint()?),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#get_toplevel(r#id: ObjectId) -> Result<()>;
        fn r#get_popup(
            r#id: ObjectId,
            r#parent: Option<ObjectId>,
            r#positioner: ObjectId,
        ) -> Result<()>;
        fn r#set_window_geometry(r#x: i32, r#y: i32, r#width: i32, r#height: i32) -> Result<()>;
        fn r#ack_configure(r#serial: u32) -> Result<()>;
    }
    pub trait r#XdgToplevel {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#set_parent(message.object()?),
                2 => Self::r#set_title(message.string()?.ok_or(DecodeError::MalformedPayload)?),
                3 => Self::r#set_app_id(message.string()?.ok_or(DecodeError::MalformedPayload)?),
                4 => Self::r#show_window_menu(
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.uint()?,
                    message.int()?,
                    message.int()?,
                ),
                5 => Self::r#move(
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.uint()?,
                ),
                6 => Self::r#resize(
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.uint()?,
                    message.uint()?,
                ),
                7 => Self::r#set_max_size(message.int()?, message.int()?),
                8 => Self::r#set_min_size(message.int()?, message.int()?),
                9 => Self::r#set_maximized(),
                10 => Self::r#unset_maximized(),
                11 => Self::r#set_fullscreen(message.object()?),
                12 => Self::r#unset_fullscreen(),
                13 => Self::r#set_minimized(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#set_parent(r#parent: Option<ObjectId>) -> Result<()>;
        fn r#set_title(r#title: String) -> Result<()>;
        fn r#set_app_id(r#app_id: String) -> Result<()>;
        fn r#show_window_menu(r#seat: ObjectId, r#serial: u32, r#x: i32, r#y: i32) -> Result<()>;
        fn r#move(r#seat: ObjectId, r#serial: u32) -> Result<()>;
        fn r#resize(r#seat: ObjectId, r#serial: u32, r#edges: u32) -> Result<()>;
        fn r#set_max_size(r#width: i32, r#height: i32) -> Result<()>;
        fn r#set_min_size(r#width: i32, r#height: i32) -> Result<()>;
        fn r#set_maximized() -> Result<()>;
        fn r#unset_maximized() -> Result<()>;
        fn r#set_fullscreen(r#output: Option<ObjectId>) -> Result<()>;
        fn r#unset_fullscreen() -> Result<()>;
        fn r#set_minimized() -> Result<()>;
    }
    pub trait r#XdgPopup {
        fn handle_request(client: &Client, message: &mut Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#grab(
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.uint()?,
                ),
                2 => Self::r#reposition(
                    message.object()?.ok_or(DecodeError::MalformedPayload)?,
                    message.uint()?,
                ),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#grab(r#seat: ObjectId, r#serial: u32) -> Result<()>;
        fn r#reposition(r#positioner: ObjectId, r#token: u32) -> Result<()>;
    }
}
