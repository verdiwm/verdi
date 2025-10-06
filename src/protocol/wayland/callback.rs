use waynest_server::RequestDispatcher;

pub use waynest_protocols::server::core::wayland::wl_callback::*;

use crate::{Client, VerdiError};

#[derive(Debug, Default, RequestDispatcher)]
#[waynest(error = VerdiError, connection = Client)]
pub struct Callback;

impl WlCallback for Callback {
    type Connection = Client;
}
