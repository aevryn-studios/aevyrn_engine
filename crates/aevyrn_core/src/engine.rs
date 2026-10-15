//! Engine state and lifecycle management.

use crate::{config::EngineConfig, error::{Error, Result}};
use std::time::Instant;

/// The runtime state of the engine.
#[derive(Debug, Clone)]
pub struct EngineState {
    pub initialized: bool,
    pub running: bool,
    pub frame_count: u64,
    pub start_time: Option<Instant>,
}

impl Default for EngineState {
    fn default() -> Self {
        Self {
            initialized: false,
            running: false,
            frame_count: 0,
            start_time: None,
        }
    }
}

/// Core engine coordinator.
#[derive(Debug, Clone)]
pub struct AevyrnEngine {
    config: EngineConfig,
    state: EngineState,
}

impl AevyrnEngine {
    pub fn new(config: EngineConfig) -> Self {
        Self {
            config,
            state: EngineState::default(),
        }
    }

    pub fn config(&self) -> &EngineConfig {
        &self.config
    }

    pub fn state(&self) -> &EngineState {
        &self.state
    }

    pub fn is_initialized(&self) -> bool {
        self.state.initialized
    }

    pub fn is_running(&self) -> bool {
        self.state.running
    }

    pub fn initialize(&mut self) -> Result<()> {
        if self.state.initialized {
            return Err(Error::AlreadyRunning);
        }

        self.state.initialized = true;
        self.state.running = false;
        self.state.frame_count = 0;
        self.state.start_time = Some(Instant::now());

        Ok(())
    }

    pub fn set_running(&mut self, running: bool) -> Result<()> {
        if !self.state.initialized {
            return Err(Error::NotInitialized);
        }

        self.state.running = running;
        Ok(())
    }

    pub fn tick_frame(&mut self) -> Result<()> {
        if !self.state.initialized {
            return Err(Error::NotInitialized);
        }

        self.state.frame_count += 1;
        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.state.running = false;
        self.state.initialized = false;
        self.state.frame_count = 0;
        self.state.start_time = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_initializes_and_tracks_lifecycle() {
        let mut engine = AevyrnEngine::new(EngineConfig::default());

        engine.initialize().unwrap();
        assert!(engine.is_initialized());
        assert!(!engine.is_running());

        engine.set_running(true).unwrap();
        assert!(engine.is_running());

        engine.tick_frame().unwrap();
        assert_eq!(engine.state().frame_count, 1);

        engine.shutdown();
        assert!(!engine.is_initialized());
        assert!(!engine.is_running());
    }
}
