//! The adapter list every gate 6 GPU check runs on. Empty fails the item.

use joinn_frame::Verdict;
use joinn_gpu::{GpuAdapter, adapters};

const REFUSAL: &str = "no GPU adapter; acceptance is at least one adapter (on Linux, install mesa-vulkan-drivers for lavapipe)";

/// Every adapter, or `None` after printing the refusal. An empty `Ok` is the
/// same refusal: the item fails, it does not skip.
pub(crate) fn g6_adapters() -> Option<Vec<GpuAdapter>> {
    match adapters() {
        Verdict::Ok(list) if !list.is_empty() => Some(list),
        Verdict::Ok(_) => {
            println!("{REFUSAL}");
            None
        }
        Verdict::Refused(r) => {
            println!("{}", r.reason);
            None
        }
    }
}
