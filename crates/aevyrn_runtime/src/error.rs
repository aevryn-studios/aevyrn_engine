//! Runtime error types.

use thiserror::Error;

/// Result type for runtime operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Runtime errors.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Runtime initialization failed: {0}")]
    InitializationFailed(String),

    #[error("Runtime error: {0}")]
    RuntimeError(String),
}

impl Error {
    pub fn init_failed<S: Into<String>>(msg: S) -> Self {
        Error::InitializationFailed(msg.into())
    }

    pub fn runtime_error<S: Into<String>>(msg: S) -> Self {
        Error::RuntimeError(msg.into())
    }
}
