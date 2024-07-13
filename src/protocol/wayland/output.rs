use crate::{Dispatcher, Result};

pub use crate::protocol::interfaces::wayland::wl_output::*;

#[derive(Debug, Dispatcher)]
pub struct Output;

impl Output {
    pub fn new() -> Self {
        Self
    }
}

impl WlOutput for Output {
    async fn release(
        &self,
        _object: &crate::Object,
        _client: &mut crate::Client,
    ) -> crate::Result<()> {
        todo!()
    }
}
