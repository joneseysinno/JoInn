//! Open a device on an adapter at WebGPU core limits.

use std::sync::{Arc, Mutex};

use joinn_frame::Verdict;

use super::Gpu;
use crate::adapter::GpuAdapter;
use crate::refuse::refuse;

/// `Limits::defaults()` (WebGPU core) and no features. An adapter whose vertex
/// stage cannot read storage buffers is refused: the tables are read there.
pub fn open(adapter: &GpuAdapter) -> Verdict<Gpu> {
    if !adapter.vertex_storage() {
        return refuse(format!(
            "open: {} lacks DownlevelFlags::VERTEX_STORAGE; acceptance is an adapter whose vertex stage reads storage buffers (the vertex-buffer fallback is R72)",
            adapter.line()
        ));
    }
    let requested = pollster::block_on(adapter.adapter().request_device(&wgpu::DeviceDescriptor {
        label: Some("joinn"),
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::defaults(),
        experimental_features: wgpu::ExperimentalFeatures::disabled(),
        memory_hints: wgpu::MemoryHints::Performance,
        trace: wgpu::Trace::Off,
    }));
    let (device, queue) = match requested {
        Ok(pair) => pair,
        Err(e) => {
            return refuse(format!(
                "open: {} refused a device at WebGPU core limits ({e}); acceptance is an adapter that grants Limits::defaults()",
                adapter.line()
            ));
        }
    };
    let uncaptured: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&uncaptured);
    device.on_uncaptured_error(Arc::new(move |e: wgpu::Error| {
        if let Ok(mut errors) = sink.lock() {
            errors.push(e.to_string());
        }
    }));
    Verdict::Ok(Gpu {
        device,
        queue,
        line: adapter.line().to_owned(),
        uncaptured,
    })
}
