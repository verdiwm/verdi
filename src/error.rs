use std::io;

use waynest::wire::DecodeError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Internal Error")]
    Internal,
    #[error("Not found")]
    NotFound,
    #[error("Input error: {0}")]
    Input(colpetto::Error),
    #[error("Malformed")]
    Malformed(#[from] DecodeError),
    #[error("Io Error: {0}")]
    IoError(#[from] io::Error),
    #[error("Unknown Opcode")]
    UnknownOpcode,
}
