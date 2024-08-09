#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Input error: {0}")]
    Input(#[from] colpetto::Error),
    #[error("")]
    Protocol(#[from] waynest::server::Error),
}
