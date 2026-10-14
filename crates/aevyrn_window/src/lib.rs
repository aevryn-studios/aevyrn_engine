//! # Aevyrn Window
//!
//! Provides the platform window abstraction used by the runtime and CLI.

pub mod error;
pub mod window;

pub use error::{Error, Result};
pub use window::{Window, WindowConfig, WindowEvent};
