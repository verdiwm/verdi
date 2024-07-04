#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("")]
    UnknownOpcode,
}
