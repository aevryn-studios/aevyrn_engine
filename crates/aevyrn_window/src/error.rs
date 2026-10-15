//! Window-specific error types.

use thiserror::Error;

/// Result type for window operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during window creation or management.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Window initialization failed: {0}")]
    InitializationFailed(String),

    #[error("Window error: {0}")]
    WindowError(String),
}

impl Error {
    pub fn init_failed<S: Into<String>>(msg: S) -> Self {
        Error::InitializationFailed(msg.into())
    }

    pub fn window_error<S: Into<String>>(msg: S) -> Self {
        Error::WindowError(msg.into())
    }
}
