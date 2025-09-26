use std::sync::Arc;

use waynest::ObjectId;
use waynest_server::{Connection, RequestDispatcher};

use crate::{
    error::{Result, VerdiError},
    protocol::{
        wayland::{self, surface::Role},
        xdg::toplevel::{Toplevel, XdgToplevel},
    },
};

pub use waynest_protocols::server::stable::xdg_shell::xdg_surface::*;

#[derive(Debug, RequestDispatcher)]
#[waynest(error = VerdiError)]
pub struct Surface {
    wl_surface: Arc<wayland::surface::Surface>,
}

impl Surface {
    pub fn new(wl_surface: Arc<wayland::surface::Surface>) -> Self {
        Self { wl_surface }
    }
}

impl XdgSurface for Surface {
    type Connection = Connection<VerdiError>;

    async fn destroy(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
    ) -> Result<()> {
        Ok(())
    }

    async fn get_toplevel(
        &self,
        client: &mut Self::Connection,
        sender_id: ObjectId,
        id: ObjectId,
    ) -> Result<()> {
        client.insert(id, Toplevel::new(client.get::<Self>(sender_id).unwrap()));

        self.wl_surface.set_role(Role::XdgToplevel)?;

        Ok(())
    }

    async fn get_popup(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        _id: ObjectId,
        _parent: Option<ObjectId>,
        _positioner: ObjectId,
    ) -> Result<()> {
        todo!()
    }

    async fn set_window_geometry(
        &self,
        _client: &mut Self::Connection,
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
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        _serial: u32,
    ) -> Result<()> {
        todo!()
    }
}
