//! Write only the rows a delta names.

use joinn_frame::Verdict;
use joinn_visual::{Delta, Table, Tables, row_bytes};

use super::all_bytes::all_bytes;
use super::bind_groups::bind_groups;
use super::chart_extent::chart_extent;
use super::table_buffer::table_buffer;
use super::{ROW_SIZES, Renderer, Upload};
use crate::gpu::{Gpu, scoped};
use crate::refuse::refuse;

impl Renderer {
    /// Adjacent slots of one table are merged into one write. A table whose row
    /// count changed since the last upload is written whole, into a new buffer.
    pub fn apply(&mut self, gpu: &Gpu, tables: &Tables, delta: &Delta) -> Verdict<Upload> {
        let lens = [
            tables.body.len(),
            tables.cell.len(),
            tables.port.len(),
            tables.link.len(),
            tables.incidence.len(),
            tables.style.len(),
            tables.chart.len(),
            tables.frame.len(),
            tables.stroke.len(),
        ];
        let index = |t: Table| match t {
            Table::Body => Some(0),
            Table::Cell => Some(1),
            Table::Port => Some(2),
            Table::Link => Some(3),
            Table::Incidence => Some(4),
            Table::Chart => Some(6),
            Table::Frame => Some(7),
            Table::Stroke => Some(8),
            Table::Route | Table::Segment => None,
        };
        let resized: Vec<usize> = (0..9).filter(|&i| lens[i] != self.rows[i]).collect();
        let whole = if resized.is_empty() {
            None
        } else {
            Some(all_bytes(tables))
        };
        let mut rows: Vec<(Table, u32)> = delta
            .rows
            .iter()
            .filter(|w| index(w.table).is_some_and(|i| !resized.contains(&i)))
            .map(|w| (w.table, w.slot))
            .collect();
        rows.sort();
        rows.dedup();
        let mut runs: Vec<(Table, u32, Vec<u8>, usize)> = Vec::new();
        for (table, slot) in rows {
            let Some(bytes) = row_bytes(tables, table, slot) else {
                return refuse(format!(
                    "apply: {table:?} slot {slot} is past the end of its table; acceptance is a delta drawn from these tables"
                ));
            };
            match runs.last_mut() {
                Some((t, first, run, n))
                    if *t == table
                        && u32::try_from(*n).ok().and_then(|n| first.checked_add(n))
                            == Some(slot) =>
                {
                    run.extend(bytes);
                    *n += 1;
                }
                _ => runs.push((table, slot, bytes, 1)),
            }
        }
        self.extent = chart_extent(tables);
        scoped(gpu, "applying a delta", || {
            let mut up = Upload::default();
            if let Some(whole) = &whole {
                for &i in &resized {
                    self.buffers[i] =
                        table_buffer(gpu.device(), gpu.queue(), i, &whole[i], ROW_SIZES[i]);
                    self.rows[i] = lens[i];
                    up.rows += lens[i];
                    up.bytes += whole[i].len();
                }
                self.groups = bind_groups(
                    gpu.device(),
                    &self.layouts,
                    &self.tick,
                    &self.buffers,
                    &self.pass,
                );
            }
            for (table, first, bytes, n) in &runs {
                let Some(i) = index(*table) else {
                    continue;
                };
                let offset = u64::from(*first) * ROW_SIZES[i] as u64;
                gpu.queue().write_buffer(&self.buffers[i], offset, bytes);
                up.rows += n;
                up.bytes += bytes.len();
            }
            up
        })
    }
}
