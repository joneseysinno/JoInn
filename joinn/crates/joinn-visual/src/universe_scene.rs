//! The universe scene: one lens of a universe drawn as tables (plan 7.2 §2.9).
//! Chart slots put bodies first (a body's chart slot is its body slot), then
//! systems, then galaxies, then the universe. Rows hold layout units in their
//! own chart; only chart rows know where a chart sits, relative to the anchor.
//! A pan or zoom writes no row; a rebase writes chart rows only (V142).

mod fit;
mod grow;
mod present;
mod print_id;
mod rebase;
mod regrow;
mod shapes_at;
mod take_pending;
mod write_charts;
mod write_values;

use std::collections::{BTreeMap, BTreeSet};

use joinn_link::Address;

use crate::camera::ChartId;
use crate::charts::UniverseLayout;
use crate::tables::{RowWrite, Tables};

/// A laid-out lens drawn as tables.
#[derive(Clone, Debug)]
pub struct UniverseScene {
    layout: UniverseLayout,
    tables: Tables,
    anchor: ChartId,
    slots: Vec<u32>,
    bodies: BTreeMap<String, u32>,
    cells: BTreeMap<(u32, String), u32>,
    ports: BTreeMap<(u32, Address), u32>,
    values: BTreeMap<(u32, Address), String>,
    value_start: u32,
    pending: BTreeSet<RowWrite>,
}

impl UniverseScene {
    /// The tables as they stand.
    pub fn tables(&self) -> &Tables {
        &self.tables
    }

    /// The laid-out lens.
    pub fn layout(&self) -> &UniverseLayout {
        &self.layout
    }

    /// The chart every chart row is relative to.
    pub fn anchor(&self) -> ChartId {
        self.anchor
    }

    /// The chart table slot of a layout chart.
    pub fn chart_slot(&self, chart: ChartId) -> Option<u32> {
        self.slots.get(chart.0 as usize).copied()
    }

    /// The body slot of a body alias.
    pub fn body_slot(&self, alias: &str) -> Option<u32> {
        self.bodies.get(alias).copied()
    }
}
