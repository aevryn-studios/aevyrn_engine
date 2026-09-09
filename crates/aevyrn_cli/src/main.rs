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

    info!("═══════════════════════════════════════════════════════════════");
    info!("           Aevyrn Engine v{}", env!("CARGO_PKG_VERSION"));
    info!("═══════════════════════════════════════════════════════════════");
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
    runtime.initialize(window_config).await?;

    info!("Engine initialized. Opening main loop...");
    info!("");

    // Run the main loop
    // Note: In Rust, the event loop can only be run once per process
    // This will block until the window is closed
    let (window, event_loop) = aevyrn_window::Window::new(WindowConfig::default())?;
    runtime.run(event_loop)?;

    info!("");
    info!("═══════════════════════════════════════════════════════════════");
    info!("                    Aevyrn Engine Shutdown");
    info!("═══════════════════════════════════════════════════════════════");

    Ok(())
}
