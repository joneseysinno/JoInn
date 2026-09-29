//! The first adapter that is not a CPU device, or the first adapter otherwise.

use joinn_gpu::GpuAdapter;

/// `adapters()` is already sorted. An empty list is the adapter refusal.
pub(super) fn pick_adapter(list: Vec<GpuAdapter>) -> Result<GpuAdapter, String> {
    let mut chosen = None;
    let mut fallback = None;
    for adapter in list {
        if !adapter.is_cpu() && chosen.is_none() {
            chosen = Some(adapter);
        } else if fallback.is_none() {
            fallback = Some(adapter);
        }
    }
    chosen.or(fallback).ok_or_else(|| {
        "no GPU adapter; acceptance is at least one adapter (on Linux, install mesa-vulkan-drivers for lavapipe)".to_owned()
    })
}
