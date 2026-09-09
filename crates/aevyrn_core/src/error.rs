//! Error types for the Aevyrn Engine core.

use std::fmt;
use thiserror::Error;

/// Result type for engine operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Core engine errors.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Engine initialization failed: {0}")]
    InitializationFailed(String),

    #[error("Engine is not initialized")]
    NotInitialized,

    #[error("Engine is already running")]
    AlreadyRunning,

    #[error("Engine error: {0}")]
    Other(String),
}

impl Error {
    pub fn initialization_failed<S: Into<String>>(msg: S) -> Self {
        Error::InitializationFailed(msg.into())
    }

    pub fn other<S: Into<String>>(msg: S) -> Self {
        Error::Other(msg.into())
    }
}
