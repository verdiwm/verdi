#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("Malformed header")]
    MalformedHeader,
    #[error("Invalid payload lenght")]
    InvalidLenght,
    #[error("Malformed payload")]
    MalformedPayload,
    #[error("{0}")]
    IoError(#[from] std::io::Error),
}
