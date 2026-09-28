//! Wait for the device to finish submitted work.

use joinn_frame::Verdict;

use super::Gpu;
use crate::refuse::refuse;

/// Blocks until the queue is idle.
pub(crate) fn wait(gpu: &Gpu) -> Verdict<()> {
    match gpu.device.poll(wgpu::PollType::wait_indefinitely()) {
        Ok(_) => Verdict::Ok(()),
        Err(e) => refuse(format!(
            "gpu: waiting on {} failed ({e}); acceptance is a device that finishes its work",
            gpu.line
        )),
    }
}
