//! The main runtime loop implementation.

use crate::{Error, Result};
use aevyrn_core::{AevyrnEngine, EngineConfig};
use aevyrn_window::{Window, WindowConfig, WindowEvent};
use aevyrn_renderer::RenderContext;
use log::{info, debug, error};
use winit::event_loop::ControlFlow;
use std::time::{Instant, Duration};

/// The main runtime that orchestrates the engine.
///
/// This struct manages the event loop, window, renderer, and engine state.
/// It coordinates frame timing and handles the main game loop.
pub struct Runtime {
    engine: AevyrnEngine,
    window: Option<Window>,
    render_context: Option<RenderContext>,
}

impl Runtime {
    /// Create a new runtime with an engine configuration.
    pub fn new(config: EngineConfig) -> Self {
        let engine = AevyrnEngine::new(config);
        Self {
            engine,
            window: None,
            render_context: None,
        }
    }

    /// Initialize the runtime.
    ///
    /// This sets up the engine and renderer. Window creation is performed by the caller
    /// to avoid holding a Winit EventLoop across await points.
    pub async fn initialize(&mut self) -> Result<()> {
        info!("Initializing Aevyrn Runtime");

        // Initialize the engine
        self.engine
            .initialize()
            .map_err(|e| Error::init_failed(format!("Engine initialization failed: {}", e)))?;

        // Initialize the renderer (async)
        let render_context = RenderContext::new()
            .await
            .map_err(|e| Error::init_failed(format!("Renderer initialization failed: {}", e)))?;

        self.render_context = Some(render_context);

        info!("Runtime initialized successfully (renderer ready)");
        Ok(())
    }

    /// Attach a created window to the runtime and create the GPU surface.
    ///
    /// The caller is expected to create the window (and its EventLoop) after calling
    /// initialize(), to avoid holding the EventLoop across await points.
    pub fn attach_window(&mut self, window: Window) -> Result<()> {
        info!("Attaching window to runtime");

        // Create render surface if renderer was initialized
        if let Some(rc) = self.render_context.as_mut() {
            if let Some(win_ref) = window.inner() {
                let (width, height) = window.size();
                rc.create_surface(win_ref, width, height)
                    .map_err(|e| Error::init_failed(format!("Surface creation failed: {}", e)))?;
            }
        } else {
            return Err(Error::runtime_error(
                "Renderer not initialized before attaching window",
            ));
        }

        self.window = Some(window);
        Ok(())
    }

    /// Run the main event loop.
    ///
    /// This is the main blocking call that starts the runtime. The caller must provide the
    /// EventLoop used to create the window.
    pub fn run(mut self, event_loop: winit::event_loop::EventLoop<()>) -> Result<()> {
        info!("Starting Aevyrn Runtime event loop");

        let max_frame_time = Duration::from_secs_f32(self.engine.config().max_frame_time_ms / 1000.0);
        let mut last_frame = Instant::now();
        let mut should_exit = false;

        self.engine
            .set_running(true)
            .map_err(|e| Error::runtime_error(format!("Failed to set running state: {}", e)))?;

        let result = event_loop.run(move |event, _, control_flow| {
            if should_exit {
                *control_flow = ControlFlow::Exit;
                return;
            }

            match event {
                winit::event::Event::WindowEvent { event, .. } => {
                    let window_event = WindowEvent::from(&event);
                    debug!("Window event: {:?}", window_event);

                    match window_event {
                        WindowEvent::CloseRequested => {
                            info!("Close requested");
                            should_exit = true;
                            *control_flow = ControlFlow::Exit;
                        }
                        WindowEvent::Resized { width, height } => {
                            debug!("Window resized to {}x{}", width, height);
                            if let Some(surface) = self.render_context.as_mut().and_then(|rc| rc.surface_mut()) {
                                // Note: Full resize handling with device reconfiguration will be added later
                                surface.resize(surface.device().device(), width, height);
                            }
                        }
                        _ => {}
                    }
                }
                winit::event::Event::MainEventsCleared => {
                    // Frame timing
                    let now = Instant::now();
                    let elapsed = now.duration_since(last_frame);

                    if elapsed >= max_frame_time {
                        // Render frame
                        if let Err(e) = self.render_frame() {
                            error!("Render error: {}", e);
                        }
                        last_frame = now;
                    }
                }
                _ => {}
            }
        });

        self.engine.shutdown();
        info!("Aevyrn Runtime event loop ended");

        result.map_err(|e| Error::runtime_error(format!("Event loop error: {}", e)))
    }

    /// Render a single frame.
    fn render_frame(&self) -> Result<()> {
        // Frame rendering will be properly implemented in the next phase
        // For now, this is just a placeholder that keeps the GPU alive

        if let Some(renderer) = &self.render_context {
            if let Ok(frame) = renderer.begin_frame() {
                // In a real implementation, we would:
                // 1. Create a render pass
                // 2. Execute rendering commands
                // 3. Submit to GPU

                renderer.present_frame(frame);
            }
        }

        Ok(())
    }
}
