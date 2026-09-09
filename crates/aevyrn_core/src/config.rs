//! Engine configuration.

use serde::{Deserialize, Serialize};

/// Core engine configuration.
///
/// This struct defines the basic settings for engine initialization.
/// It can be extended with additional fields as new subsystems are added.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    /// Human-readable engine name
    pub name: String,

    /// Engine version
    pub version: String,

    /// Enable verbose logging
    pub verbose: bool,

    /// Maximum frame time in milliseconds (for frame pacing)
    pub max_frame_time_ms: f32,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            name: "Aevyrn Engine".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            verbose: false,
            max_frame_time_ms: 33.33, // ~30 FPS minimum
        }
    }
}

impl EngineConfig {
    /// Create a new engine configuration with custom name and version.
    pub fn new<S: Into<String>>(name: S, version: S) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            ..Default::default()
        }
    }

    /// Enable or disable verbose logging.
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Set the maximum frame time.
    pub fn with_max_frame_time(mut self, ms: f32) -> Self {
        self.max_frame_time_ms = ms;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = EngineConfig::default();
        assert_eq!(config.name, "Aevyrn Engine");
        assert!(!config.verbose);
        assert!(config.max_frame_time_ms > 0.0);
    }

    #[test]
    fn test_config_builder() {
        let config = EngineConfig::new("Test Engine", "0.1.0")
            .with_verbose(true)
            .with_max_frame_time(16.67);

        assert_eq!(config.name, "Test Engine");
        assert_eq!(config.version, "0.1.0");
        assert!(config.verbose);
        assert_eq!(config.max_frame_time_ms, 16.67);
    }
}
