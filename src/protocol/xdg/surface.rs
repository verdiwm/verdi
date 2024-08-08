use crate::{
    protocol::xdg::toplevel::{Toplevel, XdgToplevel},
    Dispatcher, Object, Result,
};

use waynest::wire::ObjectId;

pub use crate::protocol::interfaces::xdg_shell::xdg_surface::*;

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
    async fn destroy(&self, _object: &Object, _client: &mut crate::Client) -> crate::Result<()> {
        todo!()
    }

    async fn get_toplevel(
        &self,
        _object: &Object,
        client: &mut crate::Client,
        id: waynest::wire::ObjectId,
    ) -> crate::Result<()> {
        client.insert(Toplevel::new().into_object(id));

        Ok(())
    }

    async fn get_popup(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
        _id: waynest::wire::ObjectId,
        _parent: Option<waynest::wire::ObjectId>,
        _positioner: waynest::wire::ObjectId,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn set_window_geometry(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
        _x: i32,
        _y: i32,
        _width: i32,
        _height: i32,
    ) -> crate::Result<()> {
        todo!()
    }

    async fn ack_configure(
        &self,
        _object: &Object,
        _client: &mut crate::Client,
        _serial: u32,
    ) -> crate::Result<()> {
        todo!()
    }
}
