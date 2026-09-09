//! # Aevyrn Renderer
//!
//! This crate provides the rendering subsystem using wgpu as the graphics abstraction layer.
//! wgpu allows us to target Metal (macOS), DirectX (Windows), Vulkan, and WebGPU.
//!
//! ## Architecture
//!
//! The renderer is designed to be:
//! - **Cross-platform**: Uses wgpu for platform-independent GPU access
//! - **Modular**: Can be extended with additional rendering features
//! - **Future-proof**: Structured to support advanced rendering features later
//!
//! Future rendering features (PBR, shadows, global illumination, ray-tracing, etc.)
//! will be added as rendering pipelines within this system.

pub mod error;
pub mod context;
pub mod device;
pub mod surface;

pub use context::RenderContext;
pub use device::RenderDevice;
pub use error::{Error, Result};
