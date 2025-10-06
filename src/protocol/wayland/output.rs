use waynest::ObjectId;
use waynest_server::RequestDispatcher;

use crate::{Client, Result, VerdiError};

pub use waynest_protocols::server::core::wayland::wl_output::*;

#[derive(Debug, RequestDispatcher, Default)]
#[waynest(error = VerdiError, connection = Client)]
pub struct Output;

impl WlOutput for Output {
    type Connection = Client;

    async fn release(&self, _client: &mut Self::Connection, _sender_id: ObjectId) -> Result<()> {
        todo!()
    }
}
