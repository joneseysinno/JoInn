//! Read a whole render target back, row by row.

use joinn_frame::Verdict;

use crate::gpu::{Gpu, read_buffer, scoped};

/// Rows are copied at a pitch that is a multiple of 256 bytes and trimmed to
/// `width × texel` bytes each.
pub(super) fn read_target(gpu: &Gpu, texture: &wgpu::Texture, texel: u32) -> Verdict<Vec<u8>> {
    let (width, height) = (texture.width(), texture.height());
    let row = width * texel;
    let pitch = row.div_ceil(256) * 256;
    let copied = scoped(gpu, "copying a target for readback", || {
        let buffer = gpu.device().create_buffer(&wgpu::BufferDescriptor {
            label: Some("readback"),
            size: u64::from(pitch) * u64::from(height),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let mut encoder = gpu
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("readback"),
            });
        encoder.copy_texture_to_buffer(
            texture.as_image_copy(),
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(pitch),
                    rows_per_image: Some(height),
                },
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
        gpu.queue().submit([encoder.finish()]);
        buffer
    });
    let buffer = match copied {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    read_buffer(gpu, &buffer).map(|raw| {
        raw.chunks(pitch as usize)
            .flat_map(|line| line.iter().take(row as usize).copied())
            .collect()
    })
}
