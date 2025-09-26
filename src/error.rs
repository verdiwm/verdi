use std::io;

use waynest::ObjectId;

#[derive(thiserror::Error, Debug)]
pub enum VerdiError {
    #[error("Client requested unknown global: {0}")]
    UnknownGlobal(u32),
    #[error("No object found with ID: {0}")]
    MissingObject(ObjectId),
    #[error("Protocol error: {0}")]
    Protocol(#[from] waynest::ProtocolError),
    #[error("Input error: {0}")]
    Input(#[from] colpetto::Error),
    #[error("Seat error: {0}")]
    Seat(#[from] saddle::Error),
}

impl From<io::Error> for VerdiError {
    fn from(err: io::Error) -> Self {
        Self::Protocol(err.into())
    }
}

pub type Result<T, E = VerdiError> = core::result::Result<T, E>;
