use std::io;

use crate::message::DecodeError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("")]
    Malformed(#[from] DecodeError),
    #[error("")]
    IoError(#[from] io::Error),
    #[error("")]
    UnknownOpcode,
}
