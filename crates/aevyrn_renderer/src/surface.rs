//! Render surface management.

use crate::{Error, Result};
use log::{info, debug};
use wgpu::TextureFormat;

/// Configuration for a render surface.
#[derive(Debug, Clone)]
pub struct SurfaceConfig {
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub present_mode: wgpu::PresentMode,
}

impl SurfaceConfig {
    /// Create a new surface configuration.
    pub fn new(width: u32, height: u32, format: TextureFormat) -> Self {
        Self {
            width,
            height,
            format,
            present_mode: wgpu::PresentMode::Fifo, // Vsync enabled by default
        }
    }

    /// Set the present mode (vsync, mailbox, immediate).
    pub fn with_present_mode(mut self, mode: wgpu::PresentMode) -> Self {
        self.present_mode = mode;
        self
    }
}

/// Represents a render surface (the drawable area).
pub struct RenderSurface {
    surface: wgpu::Surface,
    config: SurfaceConfig,
}

impl RenderSurface {
    /// Create a new render surface.
    pub(crate) fn new(surface: wgpu::Surface, config: SurfaceConfig) -> Self {
        info!(
            "Render surface created: {}x{} ({:?})",
            config.width, config.height, config.format
        );
        Self { surface, config }
    }

    /// Get a reference to the wgpu surface.
    pub fn surface(&self) -> &wgpu::Surface {
        &self.surface
    }

    /// Get the surface configuration.
    pub fn config(&self) -> &SurfaceConfig {
        &self.config
    }

    /// Resize the surface.
    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        if width > 0 && height > 0 && (width != self.config.width || height != self.config.height) {
            self.config.width = width;
            self.config.height = height;
            debug!("Surface resized to {}x{}", width, height);
        }
    }

    /// Get the current surface size.
    pub fn size(&self) -> (u32, u32) {
        (self.config.width, self.config.height)
    }
}
