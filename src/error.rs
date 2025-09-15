#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Input error: {0}")]
    Input(#[from] colpetto::Error),
    #[error("{0}")]
    Protocol(#[from] waynest::server::Error),
    #[error("{0}")]
    Seat(#[from] saddle::Error),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
