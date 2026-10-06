//! Bind the uniforms and the table buffers to §7.2's four groups, and the
//! segment pipeline's group 1.

/// Group 0 tick, group 1 body, cell, port, link, incidence, chart, frame,
/// stroke, group 2 style, group 3 pass; fifth, chart, segment and route at
/// bindings 5, 8 and 9.
pub(super) fn bind_groups(
    device: &wgpu::Device,
    layouts: &[wgpu::BindGroupLayout; 5],
    tick: &wgpu::Buffer,
    buffers: &[wgpu::Buffer; 11],
    pass: &wgpu::Buffer,
) -> [wgpu::BindGroup; 5] {
    let group = |label: &str, layout: &wgpu::BindGroupLayout, bufs: &[(u32, &wgpu::Buffer)]| {
        let entries: Vec<wgpu::BindGroupEntry> = bufs
            .iter()
            .map(|(binding, b)| wgpu::BindGroupEntry {
                binding: *binding,
                resource: b.as_entire_binding(),
            })
            .collect();
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(label),
            layout,
            entries: &entries,
        })
    };
    let [
        body,
        cell,
        port,
        link,
        incidence,
        style,
        chart,
        frame,
        stroke,
        route,
        segment,
    ] = buffers;
    let [l0, l1, l2, l3, links] = layouts;
    let universe: Vec<(u32, &wgpu::Buffer)> = (0u32..)
        .zip([body, cell, port, link, incidence, chart, frame, stroke])
        .collect();
    [
        group("0 tick", l0, &[(0, tick)]),
        group("1 universe", l1, &universe),
        group("2 genome", l2, &[(0, style)]),
        group("3 pass", l3, &[(0, pass)]),
        group("1 links", links, &[(5, chart), (8, segment), (9, route)]),
    ]
}
