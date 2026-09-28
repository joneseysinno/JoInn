//! Map a readback buffer and copy its bytes out.

use std::sync::mpsc;

use joinn_frame::Verdict;

use super::Gpu;
use super::wait::wait;
use crate::refuse::refuse;

/// The buffer must have `MAP_READ` and hold finished work.
pub(crate) fn read_buffer(gpu: &Gpu, buffer: &wgpu::Buffer) -> Verdict<Vec<u8>> {
    let (tx, rx) = mpsc::channel();
    buffer.slice(..).map_async(wgpu::MapMode::Read, move |r| {
        let _ = tx.send(r.is_ok());
    });
    if let Verdict::Refused(r) = wait(gpu) {
        return Verdict::Refused(r);
    }
    if !rx.recv().unwrap_or(false) {
        return refuse(format!(
            "gpu: mapping a readback buffer on {} failed; acceptance is a buffer the device maps",
            gpu.line
        ));
    }
    let bytes = buffer.slice(..).get_mapped_range().map(|v| v.to_vec());
    buffer.unmap();
    match bytes {
        Ok(bytes) => Verdict::Ok(bytes),
        Err(e) => refuse(format!(
            "gpu: reading a mapped buffer on {} failed ({e}); acceptance is a buffer the device maps",
            gpu.line
        )),
    }
}
