//! GPU device abstraction.

use log::{info, debug};

/// Information about the GPU device.
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    /// Device name
    pub name: String,
    /// Vendor name
    pub vendor: String,
    /// Device type (e.g., "Discrete GPU", "Integrated GPU")
    pub device_type: String,
}

/// Represents the GPU device.
///
/// This struct holds the wgpu device and queue, which are used for
/// GPU command submission and resource management.
pub struct RenderDevice {
    device: wgpu::Device,
    queue: wgpu::Queue,
    adapter_info: wgpu::AdapterInfo,
}

impl RenderDevice {
    /// Create a new render device from wgpu device and queue.
    pub(crate) fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        adapter_info: wgpu::AdapterInfo,
    ) -> Self {
        let device_info = DeviceInfo {
            name: adapter_info.name.clone(),
            vendor: format!("{:?}", adapter_info.vendor),
            device_type: format!("{:?}", adapter_info.device_type),
        };

        info!(
            "GPU Device: {} ({})",
            device_info.name, device_info.device_type
        );
        debug!("GPU Vendor: {}", device_info.vendor);

        Self {
            device,
            queue,
            adapter_info,
        }
    }

    /// Get a reference to the wgpu device.
    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    /// Get a reference to the wgpu queue.
    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    /// Get GPU device information.
    pub fn info(&self) -> DeviceInfo {
        DeviceInfo {
            name: self.adapter_info.name.clone(),
            vendor: format!("{:?}", self.adapter_info.vendor),
            device_type: format!("{:?}", self.adapter_info.device_type),
        }
    }
}
