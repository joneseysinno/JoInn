//! Bind the uniforms and the table buffers to §7.2's four groups.

/// Group 0 tick, group 1 body, cell, port, link, incidence, chart, frame,
/// stroke, group 2 style, group 3 pass.
pub(super) fn bind_groups(
    device: &wgpu::Device,
    layouts: &[wgpu::BindGroupLayout; 4],
    tick: &wgpu::Buffer,
    buffers: &[wgpu::Buffer; 9],
    pass: &wgpu::Buffer,
) -> [wgpu::BindGroup; 4] {
    let group = |label: &str, layout: &wgpu::BindGroupLayout, bufs: &[&wgpu::Buffer]| {
        let entries: Vec<wgpu::BindGroupEntry> = (0u32..)
            .zip(bufs)
            .map(|(binding, b)| wgpu::BindGroupEntry {
                binding,
                resource: b.as_entire_binding(),
            })
            .collect();
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(label),
            layout,
            entries: &entries,
        })
    };
    let [body, cell, port, link, incidence, style, chart, frame, stroke] = buffers;
    let [l0, l1, l2, l3] = layouts;
    [
        group("0 tick", l0, &[tick]),
        group(
            "1 universe",
            l1,
            &[body, cell, port, link, incidence, chart, frame, stroke],
        ),
        group("2 genome", l2, &[style]),
        group("3 pass", l3, &[pass]),
    ]
}
