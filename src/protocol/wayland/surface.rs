use std::sync::OnceLock;

use waynest::ObjectId;
use waynest_server::RequestDispatcher;

use crate::{Client, Result, VerdiError};

pub use waynest_protocols::server::core::wayland::wl_surface::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Role {
    XdgToplevel,
    XdgPopup,
    Cursor,
    Subsurface,
}

#[derive(Debug, Default)]
struct State {}

#[derive(Debug, Default)]
struct DoubleBuffer {
    current: State,
    pending: State,
}

#[derive(Debug, RequestDispatcher, Default)]
#[waynest(error = VerdiError, connection = Client)]
pub struct Surface {
    role: OnceLock<Role>,
    state: DoubleBuffer,
}

impl Surface {
    pub fn set_role(&self, role: Role) -> Result<()> {
        let _ = self.role.set(role); // FIXME: check if role is already set and don't try to set if it matches

        Ok(())
    }
}

impl WlSurface for Surface {
    type Connection = Client;

    async fn destroy(&self, _client: &mut Self::Connection, _sender_id: ObjectId) -> Result<()> {
        todo!()
    }

    async fn attach(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        _buffer: Option<ObjectId>,
        _x: i32,
        _y: i32,
    ) -> Result<()> {
        todo!()
    }

    async fn damage(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        _x: i32,
        _y: i32,
        _width: i32,
        _height: i32,
    ) -> Result<()> {
        Ok(())
    }

    async fn frame(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        _callback: ObjectId,
    ) -> Result<()> {
        todo!()
    }

    async fn set_opaque_region(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        _region: Option<ObjectId>,
    ) -> Result<()> {
        todo!()
    }

    async fn set_input_region(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        _region: Option<ObjectId>,
    ) -> Result<()> {
        todo!()
    }

    async fn commit(&self, _client: &mut Self::Connection, _sender_id: ObjectId) -> Result<()> {
        // FIXME: commit state

        Ok(())
    }

    async fn set_buffer_transform(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        _transform: waynest_protocols::server::core::wayland::wl_output::Transform,
    ) -> Result<()> {
        todo!()
    }

    async fn set_buffer_scale(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        _scale: i32,
    ) -> Result<()> {
        todo!()
    }

    async fn damage_buffer(
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

    async fn offset(
        &self,
        _client: &mut Self::Connection,
        _sender_id: ObjectId,
        _x: i32,
        _y: i32,
    ) -> Result<()> {
        todo!()
    }
}
