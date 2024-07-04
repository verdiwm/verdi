#![allow(unused)]
pub mod wayland {
    use crate::{error::Error, message::Message, Result};
    pub trait r#WlDisplay {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#sync(),
                1 => Self::r#get_registry(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#sync() -> Result<()>;
        fn r#get_registry() -> Result<()>;
    }
    pub trait r#WlRegistry {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#bind(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#bind() -> Result<()>;
    }
    pub trait r#WlCallback {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                _ => Err(Error::UnknownOpcode),
            }
        }
    }
    pub trait r#WlCompositor {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#create_surface(),
                1 => Self::r#create_region(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#create_surface() -> Result<()>;
        fn r#create_region() -> Result<()>;
    }
    pub trait r#WlShmPool {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#create_buffer(),
                1 => Self::r#destroy(),
                2 => Self::r#resize(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#create_buffer() -> Result<()>;
        fn r#destroy() -> Result<()>;
        fn r#resize() -> Result<()>;
    }
    pub trait r#WlShm {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#create_pool(),
                1 => Self::r#release(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#create_pool() -> Result<()>;
        fn r#release() -> Result<()>;
    }
    pub trait r#WlBuffer {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
    }
    pub trait r#WlDataOffer {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#accept(),
                1 => Self::r#receive(),
                2 => Self::r#destroy(),
                3 => Self::r#finish(),
                4 => Self::r#set_actions(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#accept() -> Result<()>;
        fn r#receive() -> Result<()>;
        fn r#destroy() -> Result<()>;
        fn r#finish() -> Result<()>;
        fn r#set_actions() -> Result<()>;
    }
    pub trait r#WlDataSource {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#offer(),
                1 => Self::r#destroy(),
                2 => Self::r#set_actions(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#offer() -> Result<()>;
        fn r#destroy() -> Result<()>;
        fn r#set_actions() -> Result<()>;
    }
    pub trait r#WlDataDevice {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#start_drag(),
                1 => Self::r#set_selection(),
                2 => Self::r#release(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#start_drag() -> Result<()>;
        fn r#set_selection() -> Result<()>;
        fn r#release() -> Result<()>;
    }
    pub trait r#WlDataDeviceManager {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#create_data_source(),
                1 => Self::r#get_data_device(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#create_data_source() -> Result<()>;
        fn r#get_data_device() -> Result<()>;
    }
    pub trait r#WlShell {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#get_shell_surface(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#get_shell_surface() -> Result<()>;
    }
    pub trait r#WlShellSurface {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#pong(),
                1 => Self::r#move(),
                2 => Self::r#resize(),
                3 => Self::r#set_toplevel(),
                4 => Self::r#set_transient(),
                5 => Self::r#set_fullscreen(),
                6 => Self::r#set_popup(),
                7 => Self::r#set_maximized(),
                8 => Self::r#set_title(),
                9 => Self::r#set_class(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#pong() -> Result<()>;
        fn r#move() -> Result<()>;
        fn r#resize() -> Result<()>;
        fn r#set_toplevel() -> Result<()>;
        fn r#set_transient() -> Result<()>;
        fn r#set_fullscreen() -> Result<()>;
        fn r#set_popup() -> Result<()>;
        fn r#set_maximized() -> Result<()>;
        fn r#set_title() -> Result<()>;
        fn r#set_class() -> Result<()>;
    }
    pub trait r#WlSurface {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#attach(),
                2 => Self::r#damage(),
                3 => Self::r#frame(),
                4 => Self::r#set_opaque_region(),
                5 => Self::r#set_input_region(),
                6 => Self::r#commit(),
                7 => Self::r#set_buffer_transform(),
                8 => Self::r#set_buffer_scale(),
                9 => Self::r#damage_buffer(),
                10 => Self::r#offset(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#attach() -> Result<()>;
        fn r#damage() -> Result<()>;
        fn r#frame() -> Result<()>;
        fn r#set_opaque_region() -> Result<()>;
        fn r#set_input_region() -> Result<()>;
        fn r#commit() -> Result<()>;
        fn r#set_buffer_transform() -> Result<()>;
        fn r#set_buffer_scale() -> Result<()>;
        fn r#damage_buffer() -> Result<()>;
        fn r#offset() -> Result<()>;
    }
    pub trait r#WlSeat {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#get_pointer(),
                1 => Self::r#get_keyboard(),
                2 => Self::r#get_touch(),
                3 => Self::r#release(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#get_pointer() -> Result<()>;
        fn r#get_keyboard() -> Result<()>;
        fn r#get_touch() -> Result<()>;
        fn r#release() -> Result<()>;
    }
    pub trait r#WlPointer {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#set_cursor(),
                1 => Self::r#release(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#set_cursor() -> Result<()>;
        fn r#release() -> Result<()>;
    }
    pub trait r#WlKeyboard {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#release(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#release() -> Result<()>;
    }
    pub trait r#WlTouch {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#release(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#release() -> Result<()>;
    }
    pub trait r#WlOutput {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#release(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#release() -> Result<()>;
    }
    pub trait r#WlRegion {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#add(),
                2 => Self::r#subtract(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#add() -> Result<()>;
        fn r#subtract() -> Result<()>;
    }
    pub trait r#WlSubcompositor {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#get_subsurface(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#get_subsurface() -> Result<()>;
    }
    pub trait r#WlSubsurface {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#set_position(),
                2 => Self::r#place_above(),
                3 => Self::r#place_below(),
                4 => Self::r#set_sync(),
                5 => Self::r#set_desync(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#set_position() -> Result<()>;
        fn r#place_above() -> Result<()>;
        fn r#place_below() -> Result<()>;
        fn r#set_sync() -> Result<()>;
        fn r#set_desync() -> Result<()>;
    }
}
pub mod linux_dmabuf_v1 {
    use crate::{error::Error, message::Message, Result};
    pub trait r#ZwpLinuxDmabufV1 {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#create_params(),
                2 => Self::r#get_default_feedback(),
                3 => Self::r#get_surface_feedback(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#create_params() -> Result<()>;
        fn r#get_default_feedback() -> Result<()>;
        fn r#get_surface_feedback() -> Result<()>;
    }
    pub trait r#ZwpLinuxBufferParamsV1 {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#add(),
                2 => Self::r#create(),
                3 => Self::r#create_immed(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#add() -> Result<()>;
        fn r#create() -> Result<()>;
        fn r#create_immed() -> Result<()>;
    }
    pub trait r#ZwpLinuxDmabufFeedbackV1 {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
    }
}
pub mod presentation_time {
    use crate::{error::Error, message::Message, Result};
    pub trait r#WpPresentation {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#feedback(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#feedback() -> Result<()>;
    }
    pub trait r#WpPresentationFeedback {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                _ => Err(Error::UnknownOpcode),
            }
        }
    }
}
pub mod tablet_v2 {
    use crate::{error::Error, message::Message, Result};
    pub trait r#ZwpTabletManagerV2 {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#get_tablet_seat(),
                1 => Self::r#destroy(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#get_tablet_seat() -> Result<()>;
        fn r#destroy() -> Result<()>;
    }
    pub trait r#ZwpTabletSeatV2 {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
    }
    pub trait r#ZwpTabletToolV2 {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#set_cursor(),
                1 => Self::r#destroy(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#set_cursor() -> Result<()>;
        fn r#destroy() -> Result<()>;
    }
    pub trait r#ZwpTabletV2 {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
    }
    pub trait r#ZwpTabletPadRingV2 {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#set_feedback(),
                1 => Self::r#destroy(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#set_feedback() -> Result<()>;
        fn r#destroy() -> Result<()>;
    }
    pub trait r#ZwpTabletPadStripV2 {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#set_feedback(),
                1 => Self::r#destroy(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#set_feedback() -> Result<()>;
        fn r#destroy() -> Result<()>;
    }
    pub trait r#ZwpTabletPadGroupV2 {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
    }
    pub trait r#ZwpTabletPadV2 {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#set_feedback(),
                1 => Self::r#destroy(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#set_feedback() -> Result<()>;
        fn r#destroy() -> Result<()>;
    }
}
pub mod viewporter {
    use crate::{error::Error, message::Message, Result};
    pub trait r#WpViewporter {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#get_viewport(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#get_viewport() -> Result<()>;
    }
    pub trait r#WpViewport {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#set_source(),
                2 => Self::r#set_destination(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#set_source() -> Result<()>;
        fn r#set_destination() -> Result<()>;
    }
}
pub mod xdg_shell {
    use crate::{error::Error, message::Message, Result};
    pub trait r#XdgWmBase {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#create_positioner(),
                2 => Self::r#get_xdg_surface(),
                3 => Self::r#pong(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#create_positioner() -> Result<()>;
        fn r#get_xdg_surface() -> Result<()>;
        fn r#pong() -> Result<()>;
    }
    pub trait r#XdgPositioner {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#set_size(),
                2 => Self::r#set_anchor_rect(),
                3 => Self::r#set_anchor(),
                4 => Self::r#set_gravity(),
                5 => Self::r#set_constraint_adjustment(),
                6 => Self::r#set_offset(),
                7 => Self::r#set_reactive(),
                8 => Self::r#set_parent_size(),
                9 => Self::r#set_parent_configure(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#set_size() -> Result<()>;
        fn r#set_anchor_rect() -> Result<()>;
        fn r#set_anchor() -> Result<()>;
        fn r#set_gravity() -> Result<()>;
        fn r#set_constraint_adjustment() -> Result<()>;
        fn r#set_offset() -> Result<()>;
        fn r#set_reactive() -> Result<()>;
        fn r#set_parent_size() -> Result<()>;
        fn r#set_parent_configure() -> Result<()>;
    }
    pub trait r#XdgSurface {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#get_toplevel(),
                2 => Self::r#get_popup(),
                3 => Self::r#set_window_geometry(),
                4 => Self::r#ack_configure(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#get_toplevel() -> Result<()>;
        fn r#get_popup() -> Result<()>;
        fn r#set_window_geometry() -> Result<()>;
        fn r#ack_configure() -> Result<()>;
    }
    pub trait r#XdgToplevel {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#set_parent(),
                2 => Self::r#set_title(),
                3 => Self::r#set_app_id(),
                4 => Self::r#show_window_menu(),
                5 => Self::r#move(),
                6 => Self::r#resize(),
                7 => Self::r#set_max_size(),
                8 => Self::r#set_min_size(),
                9 => Self::r#set_maximized(),
                10 => Self::r#unset_maximized(),
                11 => Self::r#set_fullscreen(),
                12 => Self::r#unset_fullscreen(),
                13 => Self::r#set_minimized(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#set_parent() -> Result<()>;
        fn r#set_title() -> Result<()>;
        fn r#set_app_id() -> Result<()>;
        fn r#show_window_menu() -> Result<()>;
        fn r#move() -> Result<()>;
        fn r#resize() -> Result<()>;
        fn r#set_max_size() -> Result<()>;
        fn r#set_min_size() -> Result<()>;
        fn r#set_maximized() -> Result<()>;
        fn r#unset_maximized() -> Result<()>;
        fn r#set_fullscreen() -> Result<()>;
        fn r#unset_fullscreen() -> Result<()>;
        fn r#set_minimized() -> Result<()>;
    }
    pub trait r#XdgPopup {
        fn handle_request(message: &Message) -> Result<()> {
            match message.opcode {
                0 => Self::r#destroy(),
                1 => Self::r#grab(),
                2 => Self::r#reposition(),
                _ => Err(Error::UnknownOpcode),
            }
        }
        fn r#destroy() -> Result<()>;
        fn r#grab() -> Result<()>;
        fn r#reposition() -> Result<()>;
    }
}
