//! Window creation and event handling.

use crate::{Error, Result};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use winit::{
    dpi::PhysicalSize,
    event::WindowEvent as WinitWindowEvent,
    event_loop::EventLoop,
    window::{Window as WinitWindow, WindowBuilder},
};

/// Configuration for the engine window.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
    pub visible: bool,
    pub maximized: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Aevyrn Engine".to_string(),
            width: 1280,
            height: 720,
            resizable: true,
            visible: true,
            maximized: false,
        }
    }
}

impl WindowConfig {
    pub fn with_title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn with_maximized(mut self, maximized: bool) -> Self {
        self.maximized = maximized;
        self
    }
}

/// Events emitted by the engine window wrapper.
#[derive(Debug, Clone)]
pub enum WindowEvent {
    CloseRequested,
    Resized { width: u32, height: u32 },
    Focused(bool),
    KeyboardInput {
        key: String,
        state: String,
        repeat: bool,
    },
    Other(String),
}

impl From<&WinitWindowEvent<'_>> for WindowEvent {
    fn from(event: &WinitWindowEvent<'_>) -> Self {
        match event {
            WinitWindowEvent::CloseRequested => WindowEvent::CloseRequested,
            WinitWindowEvent::Resized(size) => WindowEvent::Resized {
                width: size.width,
                height: size.height,
            },
            WinitWindowEvent::Focused(focused) => WindowEvent::Focused(*focused),
            WinitWindowEvent::KeyboardInput { event, .. } => WindowEvent::KeyboardInput {
                key: format!("{:?}", event.physical_key),
                state: format!("{:?}", event.state),
                repeat: event.repeat,
            },
            other => WindowEvent::Other(format!("{:?}", other)),
        }
    }
}

/// Engine window wrapper around a Winit window.
pub struct Window {
    inner: Option<WinitWindow>,
}

impl Window {
    /// Create a new window and return it alongside the Winit event loop.
    pub fn new(config: WindowConfig) -> Result<(Self, EventLoop<()>)> {
        info!(
            "Creating window: {} ({}x{})",
            config.title, config.width, config.height
        );

        // EventLoop::new() is infallible; do not treat it as a fallible operation.
        let event_loop = EventLoop::new();

        let inner = Some(
            WindowBuilder::new()
                .with_title(config.title)
                .with_inner_size(PhysicalSize::new(config.width, config.height))
                .with_resizable(config.resizable)
                .with_visible(config.visible)
                .with_maximized(config.maximized)
                .build(&event_loop)
                .map_err(|e| Error::init_failed(format!("Failed to create window: {}", e)))?,
        );

        debug!("Window created successfully");
        Ok((Self { inner }, event_loop))
    }

    /// Return the raw Winit window reference.
    pub fn inner(&self) -> Option<&WinitWindow> {
        self.inner.as_ref()
    }

    /// Get the current size of the window.
    pub fn size(&self) -> (u32, u32) {
        let size = self
            .inner
            .as_ref()
            .map(|window| window.inner_size())
            .unwrap_or(PhysicalSize::new(0, 0));

        (size.width, size.height)
    }

    /// Get the window title.
    pub fn title(&self) -> Option<String> {
        self.inner
            .as_ref()
            .map(|window| window.title().to_string())
    }

    /// Resize the window.
    pub fn resize(&self, width: u32, height: u32) {
        if let Some(window) = self.inner.as_ref() {
            window.set_inner_size(PhysicalSize::new(width, height));
        }
    }

    /// Show or hide the window.
    pub fn set_visible(&self, visible: bool) {
        if let Some(window) = self.inner.as_ref() {
            window.set_visible(visible);
        }
    }
}
