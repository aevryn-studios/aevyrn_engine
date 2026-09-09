//! # Aevyrn Runtime
//!
//! This crate provides the main runtime loop that coordinates the engine,
//! window system, and renderer. It handles events, updates, and frame rendering.
//!
//! ## Architecture
//!
//! The runtime:
//! - Owns the main event loop
//! - Coordinates frame timing
//! - Dispatches events to subsystems
//! - Manages the render loop
//! - Handles shutdown gracefully
//!
//! Future versions will extend this to support scripting, physics updates,
//! audio, animation, and other subsystems.

pub mod error;
pub mod runtime;

pub use error::{Error, Result};
pub use runtime::Runtime;
