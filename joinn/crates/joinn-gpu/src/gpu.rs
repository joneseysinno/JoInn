//! A device opened at WebGPU core limits, and the plumbing every GPU step shares.

mod open;
mod read_buffer;
mod scoped;
mod wait;

pub use open::open;
pub(crate) use read_buffer::read_buffer;
pub(crate) use scoped::scoped;
pub use wait::wait;

use std::sync::{Arc, Mutex};

/// A device and its queue, opened on one adapter.
#[derive(Debug)]
pub struct Gpu {
    device: wgpu::Device,
    queue: wgpu::Queue,
    line: String,
    uncaptured: Arc<Mutex<Vec<String>>>,
    lost: Arc<Mutex<Option<wgpu::DeviceLostReason>>>,
}

impl Gpu {
    /// The device.
    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    /// The queue.
    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    /// The adapter's line, as `cargo xtask adapters` prints it.
    pub fn line(&self) -> &str {
        &self.line
    }

    /// The reason wgpu's device-lost callback gave, once it has fired.
    pub fn lost(&self) -> Option<wgpu::DeviceLostReason> {
        self.lost.lock().ok().and_then(|reason| *reason)
    }
}
