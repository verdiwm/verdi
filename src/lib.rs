pub mod error;
pub mod protocol;

pub type Result<T, E = error::Error> = core::result::Result<T, E>;
