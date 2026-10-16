//! # Aevyrn Engine CLI
//!
//! This is the entry point for the Aevyrn Engine.
//! It initializes logging, creates the engine and runtime, and starts the main loop.

use aevyrn_core::EngineConfig;
use aevyrn_runtime::Runtime;
use aevyrn_window::WindowConfig;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .format_timestamp_millis()
        .init();

    info!("═════════════════════════════════════════════════════════════");
    info!("           Aevyrn Engine v{}", env!("CARGO_PKG_VERSION"));
    info!("═════════════════════════════════════════════════════════════");
    info!("Starting Aevyrn Engine Foundation");
    info!("");

    let engine_config = EngineConfig::default()
        .with_verbose(false)
        .with_max_frame_time(16.67);

    let window_config = WindowConfig::default();

    let mut runtime = Runtime::new(engine_config);
    runtime.initialize().await?;

    info!("Engine initialized. Opening main loop...");
    info!("");

    let (window, event_loop) = aevyrn_window::Window::new(window_config)?;
    runtime.attach_window(window)?;
    runtime.run(event_loop)?;

    info!("");
    info!("═════════════════════════════════════════════════════════════");
    info!("                    Aevyrn Engine Shutdown");
    info!("═════════════════════════════════════════════════════════════");

    Ok(())
}
