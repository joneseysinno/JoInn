//! §7.2's four bind group layouts.

use std::num::NonZeroU64;

use super::PASS_BYTES;

/// Group 0 the tick uniform, group 1 the five universe tables, group 2 the
/// style table, group 3 the pass uniform. `sizes` are the tables' row sizes in
/// `ROW_SIZES` order, and each is its binding's `min_binding_size`.
pub(crate) fn layouts(
    device: &wgpu::Device,
    tick: usize,
    sizes: [usize; 6],
) -> [wgpu::BindGroupLayout; 4] {
    let min = |n: usize| NonZeroU64::new(n as u64);
    let entry =
        |binding: u32, ty: wgpu::BufferBindingType, size: usize| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty,
                has_dynamic_offset: false,
                min_binding_size: min(size),
            },
            count: None,
        };
    let storage = wgpu::BufferBindingType::Storage { read_only: true };
    let layout = |label: &str, entries: &[wgpu::BindGroupLayoutEntry]| {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(label),
            entries,
        })
    };
    let universe: Vec<wgpu::BindGroupLayoutEntry> = (0u32..)
        .zip(&sizes[..5])
        .map(|(binding, size)| entry(binding, storage, *size))
        .collect();
    [
        layout(
            "0 tick",
            &[entry(0, wgpu::BufferBindingType::Uniform, tick)],
        ),
        layout("1 universe", &universe),
        layout("2 genome", &[entry(0, storage, sizes[5])]),
        layout(
            "3 pass",
            &[entry(0, wgpu::BufferBindingType::Uniform, PASS_BYTES)],
        ),
    ]
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use super::layouts;
    use crate::adapter::adapters;
    use crate::gpu::{open, scoped};
    use crate::renderer::pipelines::pipelines;
    use crate::renderer::{OFFSCREEN_FORMAT, ROW_SIZES, TICK_BYTES};

    #[test]
    fn the_pipelines_take_each_row_size_as_min_binding_size_and_refuse_less() {
        let all = match adapters() {
            Verdict::Ok(a) => a,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        for adapter in &all {
            let gpu = match open(adapter) {
                Verdict::Ok(g) => g,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            let exact = scoped(&gpu, "pipelines at the row sizes", || {
                let l = layouts(gpu.device(), TICK_BYTES, ROW_SIZES);
                pipelines(gpu.device(), &l, OFFSCREEN_FORMAT)
            });
            if let Verdict::Refused(r) = &exact {
                panic!(
                    "{}: the row sizes must be accepted: {}",
                    adapter.line(),
                    r.reason
                );
            }
            // Incidence and style rows are one u32: a smaller minimum is zero,
            // which a layout cannot state.
            for table in 0..4 {
                let mut short = ROW_SIZES;
                short[table] -= 4;
                let refused = scoped(&gpu, "pipelines one field short", || {
                    let l = layouts(gpu.device(), TICK_BYTES, short);
                    pipelines(gpu.device(), &l, OFFSCREEN_FORMAT)
                });
                assert!(
                    matches!(refused, Verdict::Refused(_)),
                    "{}: table {table} with min_binding_size {} must be refused",
                    adapter.line(),
                    short[table]
                );
            }
            let refused = scoped(&gpu, "pipelines with a short tick", || {
                let l = layouts(gpu.device(), TICK_BYTES - 16, ROW_SIZES);
                pipelines(gpu.device(), &l, OFFSCREEN_FORMAT)
            });
            assert!(
                matches!(refused, Verdict::Refused(_)),
                "{}: a tick uniform one row short must be refused",
                adapter.line()
            );
        }
    }
}
