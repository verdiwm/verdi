use waynest::server::{Client, Dispatcher, Object, Result};

pub use waynest::server::protocol::wayland::wl_output::*;

#[derive(Debug, Dispatcher)]
pub struct Output;

impl Output {
    pub fn new() -> Self {
        Self
    }
}

impl WlOutput for Output {
    async fn release(&self, _object: &Object, _client: &mut Client) -> Result<()> {
        todo!()
    }
}
