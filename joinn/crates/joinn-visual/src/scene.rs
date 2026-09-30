//! The Scene: a body's layout, its tables, and the rows changed since the last
//! draw. Deltas keep the tables current; `regrow` rebuilds them from DNA and live
//! state, and the two must agree byte for byte (V122).

mod apply_run;
mod grow;
mod grow_contact;
mod mark_refused;
mod place;
mod present;
mod print_owner;
mod regrow;
mod regrow_contact;
mod replace;
mod resolve;
mod take_pending;

use std::collections::{BTreeMap, BTreeSet};

use joinn_link::Address;

use crate::layout::Layout;
use crate::tables::{RowWrite, Tables};

/// A body drawn as tables.
#[derive(Clone, Debug)]
pub struct Scene {
    alias: String,
    layout: Layout,
    tables: Tables,
    cells: BTreeMap<String, u32>,
    ports: BTreeMap<Address, u32>,
    links: BTreeMap<(Address, Address), u32>,
    pending: BTreeSet<RowWrite>,
}

impl Scene {
    /// The tables as they stand.
    pub fn tables(&self) -> &Tables {
        &self.tables
    }

    /// The layout the tables were placed from.
    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    /// The body's alias in printed owners.
    pub fn alias(&self) -> &str {
        &self.alias
    }
}
