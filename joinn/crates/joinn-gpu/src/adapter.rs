//! Adapters: every one the machine offers, the software one included.

mod adapter_line;
mod adapters;

pub use adapters::adapters;

/// An adapter, the instance that found it, and its printed line.
#[derive(Clone, Debug)]
pub struct GpuAdapter {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    line: String,
    vertex_storage: bool,
    cpu: bool,
}

impl GpuAdapter {
    /// `<name> · <API> · <device type> · vertex storage <yes|no>`.
    pub fn line(&self) -> &str {
        &self.line
    }

    /// The wgpu adapter.
    pub fn adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }

    /// The instance that found it; a surface must come from the same instance.
    pub fn instance(&self) -> &wgpu::Instance {
        &self.instance
    }

    /// Whether the vertex stage can read storage buffers.
    pub fn vertex_storage(&self) -> bool {
        self.vertex_storage
    }

    /// Whether the adapter is a CPU device (lavapipe, WARP).
    pub fn is_cpu(&self) -> bool {
        self.cpu
    }
}
