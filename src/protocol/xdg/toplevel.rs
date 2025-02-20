use waynest::{
    server::{Client, Dispatcher, Result},
    wire::ObjectId,
};

pub use waynest::server::protocol::stable::xdg_shell::xdg_toplevel::*;

#[derive(Debug, Dispatcher, Default)]
pub struct Toplevel;

impl XdgToplevel for Toplevel {
    async fn destroy(&self, _client: &mut Client, _sender_id: ObjectId) -> Result<()> {
        todo!()
    }

    async fn set_parent(
        &self,
        _client: &mut Client,
        _sender_id: ObjectId,
        _parent: Option<ObjectId>,
    ) -> Result<()> {
        todo!()
    }

    async fn set_title(
        &self,
        _client: &mut Client,
        _sender_id: ObjectId,
        _title: String,
    ) -> Result<()> {
        // FIXME: change  state

        Ok(())
    }

    async fn set_app_id(
        &self,
        _client: &mut Client,
        _sender_id: ObjectId,
        _app_id: String,
    ) -> Result<()> {
        // FIXME: change  state

        Ok(())
    }

    async fn show_window_menu(
        &self,
        _client: &mut Client,
        _sender_id: ObjectId,
        _seat: ObjectId,
        _serial: u32,
        _x: i32,
        _y: i32,
    ) -> Result<()> {
        todo!()
    }

    async fn r#move(
        &self,
        _client: &mut Client,
        _sender_id: ObjectId,
        _seat: ObjectId,
        _serial: u32,
    ) -> Result<()> {
        todo!()
    }

    async fn resize(
        &self,
        _client: &mut Client,
        _sender_id: ObjectId,
        _seat: ObjectId,
        _serial: u32,
        _edges: ResizeEdge,
    ) -> Result<()> {
        todo!()
    }

    async fn set_max_size(
        &self,
        _client: &mut Client,
        _sender_id: ObjectId,
        _width: i32,
        _height: i32,
    ) -> Result<()> {
        todo!()
    }

    async fn set_min_size(
        &self,
        _client: &mut Client,
        _sender_id: ObjectId,
        _width: i32,
        _height: i32,
    ) -> Result<()> {
        todo!()
    }

    async fn set_maximized(&self, _client: &mut Client, _sender_id: ObjectId) -> Result<()> {
        todo!()
    }

    async fn unset_maximized(&self, _client: &mut Client, _sender_id: ObjectId) -> Result<()> {
        todo!()
    }

    async fn set_fullscreen(
        &self,
        _client: &mut Client,
        _sender_id: ObjectId,
        _output: Option<ObjectId>,
    ) -> Result<()> {
        todo!()
    }

    async fn unset_fullscreen(&self, _client: &mut Client, _sender_id: ObjectId) -> Result<()> {
        todo!()
    }

    async fn set_minimized(&self, _client: &mut Client, _sender_id: ObjectId) -> Result<()> {
        todo!()
    }
}
