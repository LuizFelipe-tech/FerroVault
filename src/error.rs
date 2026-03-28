use thiserror::Error;

#[derive(Error, Debug)]
pub enum FerroError {
    #[error("System I/O failure: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid input provided by the user: {0}")]
    InvalidInput(String),

    #[error("Cryptographic operation failed {0}")]
    CryptoError(String),
}

pub type Result<T> = std::result::Result<T, FerroError>;
