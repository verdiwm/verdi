use waynest_server::{Connection, RequestDispatcher};

pub use waynest_protocols::server::core::wayland::wl_callback::*;

use crate::error::VerdiError;

#[derive(Debug, Default, RequestDispatcher)]
#[waynest(error = VerdiError)]
pub struct Callback;

impl WlCallback for Callback {
    type Connection = Connection<VerdiError>;
}
