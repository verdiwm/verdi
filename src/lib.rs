pub mod error;
pub mod message;
pub mod proto;

pub type Result<T, E = error::Error> = core::result::Result<T, E>;

pub struct Dispatcher {}
