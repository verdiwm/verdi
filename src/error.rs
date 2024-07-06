use std::io;

use crate::message::DecodeError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Not found")]
    NotFound,
    #[error("Malformed")]
    Malformed(#[from] DecodeError),
    #[error("Io error")]
    IoError(#[from] io::Error),
    #[error("Unknown Opcode")]
    UnknownOpcode,
}
