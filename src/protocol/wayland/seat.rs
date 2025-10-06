use waynest::ObjectId;
use waynest_server::RequestDispatcher;

use crate::{Client, Result, VerdiError};

pub use waynest_protocols::server::core::wayland::wl_seat::*;

#[derive(Debug, RequestDispatcher, Default)]
#[waynest(error = VerdiError, connection = Client)]
pub struct Seat;

impl WlSeat for Seat {
    type Connection = Client;

    async fn get_pointer(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        _id: ObjectId,
    ) -> Result<()> {
        todo!()
    }

    async fn get_keyboard(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        _id: ObjectId,
    ) -> Result<()> {
        todo!()
    }

    async fn get_touch(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        _id: ObjectId,
    ) -> Result<()> {
        todo!()
    }

    async fn release(&self, _client: &mut Self::Connection, _sender_id: ObjectId) -> Result<()> {
        todo!()
    }
}
