use waynest::ObjectId;
use waynest_server::{Connection, RequestDispatcher};

use crate::error::{Result, VerdiError};

pub use waynest_protocols::server::core::wayland::wl_output::*;

#[derive(Debug, RequestDispatcher, Default)]
#[waynest(error = VerdiError)]
pub struct Output;

impl WlOutput for Output {
    type Connection = Connection<VerdiError>;

    async fn release(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
    ) -> Result<()> {
        todo!()
    }
}
