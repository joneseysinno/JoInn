//! Print one adapter the way `cargo xtask adapters` lists it.

/// `<name> · <API> · <device type> · vertex storage <yes|no>`.
pub(super) fn adapter_line(info: &wgpu::AdapterInfo, vertex_storage: bool) -> String {
    format!(
        "{} · {:?} · {:?} · vertex storage {}",
        info.name,
        info.backend,
        info.device_type,
        if vertex_storage { "yes" } else { "no" }
    )
}
