//! Write every table.

use joinn_frame::Verdict;
use joinn_visual::Tables;

use super::all_bytes::all_bytes;
use super::bind_groups::bind_groups;
use super::chart_extent::chart_extent;
use super::table_buffer::table_buffer;
use super::{ROW_SIZES, Renderer, Upload};
use crate::gpu::{Gpu, scoped};

impl Renderer {
    /// Each table gets a buffer sized to its rows, filled from `table_bytes`.
    pub fn upload_all(&mut self, gpu: &Gpu, tables: &Tables) -> Verdict<Upload> {
        let all = all_bytes(tables);
        self.extent = chart_extent(tables);
        scoped(gpu, "uploading every table", || {
            let mut up = Upload::default();
            for (i, bytes) in all.iter().enumerate() {
                self.buffers[i] = table_buffer(gpu.device(), gpu.queue(), i, bytes, ROW_SIZES[i]);
                self.rows[i] = bytes.len() / ROW_SIZES[i];
                up.rows += self.rows[i];
                up.bytes += bytes.len();
            }
            self.groups = bind_groups(
                gpu.device(),
                &self.layouts,
                &self.tick,
                &self.buffers,
                &self.pass,
            );
            up
        })
    }
}
