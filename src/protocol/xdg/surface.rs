use crate::protocol::xdg::toplevel::{Toplevel, XdgToplevel};

use waynest::{
    server::{Client, Dispatcher, Result},
    wire::ObjectId,
};

pub use waynest::server::protocol::stable::xdg_shell::xdg_surface::*;

#[derive(Debug, Dispatcher)]
pub struct Surface {
    wl_surface: ObjectId,
}

impl Surface {
    pub fn new(wl_surface: ObjectId) -> Self {
        Self { wl_surface }
    }
}

impl XdgSurface for Surface {
    async fn destroy(&self, _client: &mut Client, _sender_id: ObjectId) -> Result<()> {
        todo!()
    }

    async fn get_toplevel(
        &self,
        client: &mut Client,
        _sender_id: ObjectId,
        id: ObjectId,
    ) -> Result<()> {
        client.insert(id, Toplevel::default());

        Ok(())
    }

    async fn get_popup(
        &self,
        _client: &mut Client,
        _sender_id: ObjectId,
        _id: ObjectId,
        _parent: Option<ObjectId>,
        _positioner: ObjectId,
    ) -> Result<()> {
        todo!()
    }

    async fn set_window_geometry(
        &self,
        _client: &mut Client,
        _sender_id: ObjectId,
        _x: i32,
        _y: i32,
        _width: i32,
        _height: i32,
    ) -> Result<()> {
        todo!()
    }

    async fn ack_configure(
        &self,
        _client: &mut Client,
        _sender_id: ObjectId,
        _serial: u32,
    ) -> Result<()> {
        todo!()
    }
}
