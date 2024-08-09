use waynest::server::{Dispatcher, Result};

pub use waynest::server::protocol::wayland::wl_callback::*;

#[derive(Debug, Dispatcher)]
pub struct Callback;

impl Callback {
    pub fn new() -> Self {
        Self
    }
}

impl WlCallback for Callback {}
