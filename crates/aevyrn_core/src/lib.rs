//! # Aevyrn Engine Core
//!
//! The core module provides the fundamental engine initialization and configuration.
//! This module is responsible for setting up the engine state and providing the main
//! engine struct that coordinates with other subsystems.
//!
//! ## Design
//!
//! The core module follows a clean separation of concerns:
//! - **EngineConfig**: Defines engine settings and initialization parameters
//! - **EngineState**: Tracks the runtime state of the engine
//! - **AevyrnEngine**: The main engine coordinator
//!
//! Future subsystems (Renderer, Scene, Physics, etc.) will register with the engine
//! through well-defined interfaces rather than being tightly coupled.

pub mod config;
pub mod engine;
pub mod error;

pub use config::EngineConfig;
pub use engine::AevyrnEngine;
pub use error::{Error, Result};
