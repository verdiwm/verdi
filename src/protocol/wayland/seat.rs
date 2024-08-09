use waynest::{
    server::{Client, Dispatcher, Object, Result},
    wire::ObjectId,
};

pub use waynest::server::protocol::wayland::wl_seat::*;

#[derive(Debug, Dispatcher)]
pub struct Seat;

impl Seat {
    pub fn new() -> Self {
        Self
    }
}

impl WlSeat for Seat {
    async fn get_pointer(
        &self,
        _object: &Object,
        _client: &mut Client,
        _id: ObjectId,
    ) -> Result<()> {
        todo!()
    }

    async fn get_keyboard(
        &self,
        _object: &Object,
        _client: &mut Client,
        _id: ObjectId,
    ) -> Result<()> {
        todo!()
    }

    async fn get_touch(&self, _object: &Object, _client: &mut Client, _id: ObjectId) -> Result<()> {
        todo!()
    }

    async fn release(&self, _object: &Object, _client: &mut Client) -> Result<()> {
        todo!()
    }
}
