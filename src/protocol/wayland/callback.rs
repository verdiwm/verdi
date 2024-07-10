use crate::{Dispatcher, Result};

pub use crate::protocol::interfaces::wayland::wl_callback::*;

#[derive(Debug, Dispatcher)]
pub struct Callback;

impl Callback {
    pub fn new() -> Self {
        Self
    }
}

impl WlCallback for Callback {}
