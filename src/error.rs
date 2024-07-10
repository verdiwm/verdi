use std::io;

use crate::wire::DecodeError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Internal Error")]
    Internal,
    #[error("Not found")]
    NotFound,
    #[error("Malformed")]
    Malformed(#[from] DecodeError),
    #[error("Io Error: {0}")]
    IoError(#[from] io::Error),
    #[error("Unknown Opcode")]
    UnknownOpcode,
}
