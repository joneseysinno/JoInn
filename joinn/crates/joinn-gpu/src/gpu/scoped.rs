//! Run GPU work inside a validation scope, and refuse what the device refused.

use joinn_frame::Verdict;

use super::Gpu;
use crate::refuse::refuse;

/// Any validation error raised by `work`, or left uncaptured on the device since
/// the last scope, is a refusal naming `what` and the adapter.
pub(crate) fn scoped<T>(gpu: &Gpu, what: &str, work: impl FnOnce() -> T) -> Verdict<T> {
    let scope = gpu.device.push_error_scope(wgpu::ErrorFilter::Validation);
    let out = work();
    let caught = pollster::block_on(scope.pop());
    let mut errors: Vec<String> = caught.map(|e| e.to_string()).into_iter().collect();
    if let Ok(mut uncaptured) = gpu.uncaptured.lock() {
        errors.append(&mut uncaptured);
    }
    if errors.is_empty() {
        Verdict::Ok(out)
    } else {
        refuse(format!(
            "gpu: {what} on {} raised {}; acceptance is GPU work the device validates",
            gpu.line,
            errors.join(" / ")
        ))
    }
}
