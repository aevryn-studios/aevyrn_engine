//! Render context that coordinates GPU operations.

use crate::{Error, Result, RenderDevice, RenderSurface};
use log::info;
use std::sync::Arc;

/// The render context that manages the GPU connection and surfaces.
///
/// This is the main interface for rendering operations. It holds the GPU instance,
/// adapter, device, queue, and surfaces.
pub struct RenderContext {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: Arc<RenderDevice>,
    surface: Option<RenderSurface>,
}

impl RenderContext {
    /// Initialize the rendering context.
    ///
    /// This performs GPU discovery and initialization, but does not create a surface.
    pub async fn new() -> Result<Self> {
        info!("Initializing render context");

        // Create the GPU instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
            ..Default::default()
        });

        // Select an adapter (GPU)
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback: false,
                compatible_surface: None,
            })
            .await
            .ok_or_else(|| Error::init_failed("No GPU adapter found"))?;

        info!("GPU adapter selected: {}", adapter.get_info().name);

        // Create device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Aevyrn Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|e| Error::init_failed(format!("Failed to create device: {}", e)))?;

        let adapter_info = adapter.get_info();
        let render_device = RenderDevice::new(device, queue, adapter_info);

        Ok(Self {
            instance,
            adapter,
            device: Arc::new(render_device),
            surface: None,
        })
    }

    /// Create a render surface from a winit window.
    pub fn create_surface(
        &mut self,
        window: &winit::window::Window,
        width: u32,
        height: u32,
    ) -> Result<()> {
        info!("Creating render surface");

        // Create the wgpu surface from the window
        let surface = unsafe { self.instance.create_surface(window) }
            .map_err(|e| Error::surface_error(format!("Failed to create surface: {}", e)))?;

        // Get the surface format capabilities
        let capabilities = surface.get_capabilities(&self.adapter);
        let format = capabilities
            .formats
            .first()
            .copied()
            .ok_or_else(|| Error::surface_error("No supported surface formats"))?;

        info!("Surface format: {:?}", format);

        let mut config = crate::surface::SurfaceConfig::new(width, height, format);
        config = config.with_present_mode(wgpu::PresentMode::Fifo); // Vsync

        // Configure the surface
        surface.configure(self.device.device(), &wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: config.format,
            width: config.width,
            height: config.height,
            present_mode: config.present_mode,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
        });

        self.surface = Some(RenderSurface::new(surface, config));
        info!("Render surface created successfully");

        Ok(())
    }

    /// Get a reference to the render device.
    pub fn device(&self) -> &RenderDevice {
        &self.device
    }

    /// Get a reference to the render surface.
    pub fn surface(&self) -> Option<&RenderSurface> {
        self.surface.as_ref()
    }

    /// Get a mutable reference to the render surface.
    pub fn surface_mut(&mut self) -> Option<&mut RenderSurface> {
        self.surface.as_mut()
    }

    /// Get the current render surface size.
    pub fn size(&self) -> Option<(u32, u32)> {
        self.surface.as_ref().map(|s| s.size())
    }

    /// Prepare for rendering a frame.
    pub fn begin_frame(&self) -> Result<wgpu::SurfaceTexture> {
        let surface = self
            .surface
            .as_ref()
            .ok_or_else(|| Error::render_error("No render surface"))?;

        surface
            .surface()
            .get_current_texture()
            .map_err(|e| Error::render_error(format!("Failed to acquire frame: {}", e)))
    }

    /// Present the completed frame.
    pub fn present_frame(&self, frame: wgpu::SurfaceTexture) {
        frame.present();
    }
}
