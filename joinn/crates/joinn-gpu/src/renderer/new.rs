//! Build the pipelines and empty tables for one color format.

use joinn_frame::Verdict;

use super::bind_groups::bind_groups;
use super::layouts::layouts;
use super::pipelines::pipelines;
use super::table_buffer::table_buffer;
use super::{PASS_BYTES, ROW_SIZES, Renderer, TICK_BYTES};
use crate::gpu::{Gpu, scoped};

impl Renderer {
    /// Every table starts empty, bound as one zeroed row, until `upload_all`.
    pub fn new(gpu: &Gpu, format: wgpu::TextureFormat) -> Verdict<Renderer> {
        scoped(gpu, "building the renderer", || {
            let device = gpu.device();
            let layouts = layouts(device, TICK_BYTES, ROW_SIZES);
            let (shape, curve) = pipelines(device, &layouts, format);
            let uniform = |label: &str, size: usize| {
                device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some(label),
                    size: size as u64,
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                })
            };
            let tick = uniform("tick", TICK_BYTES);
            let pass = uniform("pass", PASS_BYTES);
            let buffers = ROW_SIZES.map(|row| table_buffer(device, gpu.queue(), &[], row));
            let groups = bind_groups(device, &layouts, &tick, &buffers, &pass);
            Renderer {
                format,
                layouts,
                shape,
                curve,
                tick,
                pass,
                buffers,
                rows: [0; 6],
                groups,
                ids: None,
            }
        })
    }
}
