//! Renderer error types.

use thiserror::Error;

/// Result type for renderer operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Renderer errors.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Renderer initialization failed: {0}")]
    InitializationFailed(String),

    #[error("GPU error: {0}")]
    GpuError(String),

    #[error("Surface error: {0}")]
    SurfaceError(String),

    #[error("Rendering error: {0}")]
    RenderError(String),
}

impl Error {
    pub fn init_failed<S: Into<String>>(msg: S) -> Self {
        Error::InitializationFailed(msg.into())
    }

    pub fn gpu_error<S: Into<String>>(msg: S) -> Self {
        Error::GpuError(msg.into())
    }

    pub fn surface_error<S: Into<String>>(msg: S) -> Self {
        Error::SurfaceError(msg.into())
    }

    pub fn render_error<S: Into<String>>(msg: S) -> Self {
        Error::RenderError(msg.into())
    }
}
