use waynest::server::{Dispatcher, Result};

pub use waynest::server::protocol::wayland::wl_callback::*;

#[derive(Debug, Dispatcher, Default)]
pub struct Callback;

impl WlCallback for Callback {}
