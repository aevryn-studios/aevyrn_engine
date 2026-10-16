//! # Aevyrn Engine CLI
//!
//! This is the entry point for the Aevyrn Engine.
//! It initializes logging, creates the engine and runtime, and starts the main loop.

use aevyrn_core::EngineConfig;
use aevyrn_window::WindowConfig;
use aevyrn_runtime::Runtime;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .format_timestamp_millis()
        .init();

    info!("═════════════════════════════════════════════════════════════");
    info!("           Aevyrn Engine v{}", env!("CARGO_PKG_VERSION"));
    info!("════════════════════════════════════════════════════════════=");
    info!("Starting Aevyrn Engine Foundation");
    info!("");

    // Create engine configuration
    let engine_config = EngineConfig::default()
        .with_verbose(false)
        .with_max_frame_time(16.67); // ~60 FPS

    // Create window configuration
    let window_config = WindowConfig::default();

    // Create and initialize runtime
    let mut runtime = Runtime::new(engine_config);
    // Initialize runtime (renderer and other async subsystems)
    runtime.initialize().await?;

    info!("Engine initialized. Opening main loop...");
    info!("");

    // Create the window and event loop after async initialization to avoid holding
    // a Winit EventLoop across await points.
    let (window, event_loop) = aevyrn_window::Window::new(window_config)?;

    // Attach the created window to the runtime (creates GPU surface, etc.)
    runtime.attach_window(window)?;

    // Run the main loop (this will block until the window is closed)
    runtime.run(event_loop)?;

    info!("");
    info!("════════════════════════════════════════════════════════════=");
    info!("                    Aevyrn Engine Shutdown");
    info!("════════════════════════════════════════════════════════════=");

    Ok(())
}
