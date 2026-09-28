//! Write every table.

use joinn_frame::Verdict;
use joinn_visual::{Tables, table_bytes};

use super::bind_groups::bind_groups;
use super::table_buffer::table_buffer;
use super::{ROW_SIZES, Renderer, Upload};
use crate::gpu::{Gpu, scoped};

impl Renderer {
    /// Each table gets a buffer sized to its rows, filled from `table_bytes`.
    pub fn upload_all(&mut self, gpu: &Gpu, tables: &Tables) -> Verdict<Upload> {
        let b = table_bytes(tables);
        let all = [b.body, b.cell, b.port, b.link, b.incidence, b.style];
        scoped(gpu, "uploading every table", || {
            let mut up = Upload::default();
            for (i, bytes) in all.iter().enumerate() {
                self.buffers[i] = table_buffer(gpu.device(), gpu.queue(), bytes, ROW_SIZES[i]);
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
