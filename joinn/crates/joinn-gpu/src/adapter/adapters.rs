//! Every adapter the machine offers, sorted by graphics API and name.

use joinn_frame::Verdict;

use super::GpuAdapter;
use super::adapter_line::adapter_line;
use crate::refuse::refuse;

/// Every adapter `enumerate_adapters(Backends::PRIMARY)` finds, plus the one
/// `request_adapter` returns with `force_fallback_adapter` when it isn't already
/// listed. Sorted by (API name, adapter name). An empty list is a refusal.
pub fn adapters() -> Verdict<Vec<GpuAdapter>> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    let mut found = pollster::block_on(instance.enumerate_adapters(wgpu::Backends::PRIMARY));
    if let Ok(fallback) =
        pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: true,
            compatible_surface: None,
            apply_limit_buckets: false,
        }))
    {
        let info = fallback.get_info();
        let listed = found.iter().any(|a| {
            let other = a.get_info();
            other.name == info.name && other.backend == info.backend
        });
        if !listed {
            found.push(fallback);
        }
    }
    let mut out: Vec<(String, String, GpuAdapter)> = found
        .into_iter()
        .map(|adapter| {
            let info = adapter.get_info();
            let vertex_storage = adapter
                .get_downlevel_capabilities()
                .flags
                .contains(wgpu::DownlevelFlags::VERTEX_STORAGE);
            let line = adapter_line(&info, vertex_storage);
            (
                format!("{:?}", info.backend),
                info.name.clone(),
                GpuAdapter {
                    instance: instance.clone(),
                    adapter,
                    line,
                    vertex_storage,
                    cpu: info.device_type == wgpu::DeviceType::Cpu,
                },
            )
        })
        .collect();
    out.sort_by(|a, b| (&a.0, &a.1).cmp(&(&b.0, &b.1)));
    if out.is_empty() {
        return refuse(
            "no GPU adapter; acceptance is at least one adapter (on Linux, install mesa-vulkan-drivers for lavapipe)",
        );
    }
    Verdict::Ok(out.into_iter().map(|(_, _, a)| a).collect())
}
