//! The slot a new row takes: the lowest free one, or a new one at the end.

use super::{LIVE, Table, Tables};

impl Tables {
    /// A slot is free when its row is not live. Reuse takes the lowest.
    pub(crate) fn free_slot(&self, table: Table) -> u32 {
        let flags: Vec<u32> = match table {
            Table::Body => self.body.iter().map(|r| r.flags).collect(),
            Table::Cell => self.cell.iter().map(|r| r.flags).collect(),
            Table::Port => self.port.iter().map(|r| r.flags).collect(),
            Table::Link => self.link.iter().map(|r| r.flags).collect(),
            Table::Chart => self.chart.iter().map(|r| r.flags).collect(),
            Table::Frame => self.frame.iter().map(|r| r.flags).collect(),
            Table::Stroke => self.stroke.iter().map(|r| r.flags).collect(),
            Table::Route => self.route.iter().map(|r| r.flags).collect(),
            Table::Segment => self.segment.iter().map(|r| r.flags).collect(),
            Table::Incidence => Vec::new(),
        };
        let free = flags
            .iter()
            .position(|f| f & LIVE == 0)
            .unwrap_or(flags.len());
        u32::try_from(free).unwrap_or(u32::MAX)
    }
}
