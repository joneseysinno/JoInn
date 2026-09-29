//! The GPU half of a click, printed on the tick after the CPU pick.

/// Eight spaces, then the owner. A disagreement is a truth violation in a debug build.
pub fn gpu_agree(x: u32, y: u32, cpu: &str, gpu: &str) -> String {
    if cpu == gpu {
        format!("        {gpu} (gpu) · agree")
    } else if cfg!(debug_assertions) {
        format!("TRUTH VIOLATION at {x},{y}: cpu {cpu}, gpu {gpu}")
    } else {
        format!("        {gpu} (gpu)")
    }
}
