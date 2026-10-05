//! A buffer holding one table.

use super::{STYLE, STYLE_UNIFORM_BYTES};

/// Sized to the table's bytes, or one zeroed row when the table has no rows.
/// Table `STYLE` is a uniform of `STYLE_UNIFORM_BYTES`, zero past its rows;
/// every other table is a storage buffer.
pub(super) fn table_buffer(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    table: usize,
    bytes: &[u8],
    row: usize,
) -> wgpu::Buffer {
    let (size, usage) = if table == STYLE {
        (
            STYLE_UNIFORM_BYTES.max(bytes.len()),
            wgpu::BufferUsages::UNIFORM,
        )
    } else {
        (bytes.len().max(row), wgpu::BufferUsages::STORAGE)
    };
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("table"),
        size: size as u64,
        usage: usage | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    if !bytes.is_empty() {
        queue.write_buffer(&buffer, 0, bytes);
    }
    buffer
}
