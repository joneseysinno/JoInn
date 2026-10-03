//! `cargo xtask grove`: the generated universe of plan 7.2 §2.4. It is grown
//! from its seed wherever it is needed and never stored under corpus/ or docs/
//! (rule 73).

mod admit_universe;
mod grove_cmd;
mod grove_layout;
mod grove_line;
mod grow_grove;
mod kind_name;
mod load_universe_arg;
mod out_path;
mod splitmix64;

pub(crate) use admit_universe::admit_universe;
pub(crate) use grove_cmd::grove;
pub(crate) use grove_layout::grove_layout;
pub(crate) use grove_line::grove_line;
pub(crate) use grow_grove::grow_grove;
pub(crate) use kind_name::kind_name;
pub(crate) use load_universe_arg::load_universe_arg;
pub(crate) use out_path::out_path;
pub(crate) use splitmix64::splitmix64;

/// The default seed.
pub(crate) const GROVE_SEED: u64 = 7;
/// Galaxies in the grove.
pub(crate) const GALAXIES: usize = 8;
/// Systems in each galaxy.
pub(crate) const SYSTEMS: usize = 16;
/// Bodies in each system.
pub(crate) const SLOTS: usize = 24;

/// The three corpus bodies the grove binds, by coding hash.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) enum GroveKind {
    /// `phase2/calculator.body`.
    Calc,
    /// `phase5/units.body`.
    Units,
    /// `phase5/bus.body`.
    Bus,
}

/// Every kind, in the order the line prints them.
pub(crate) const KINDS: [GroveKind; 3] = [GroveKind::Calc, GroveKind::Units, GroveKind::Bus];

impl GroveKind {
    /// `calc`, `units` or `bus`.
    pub(crate) fn name(self) -> &'static str {
        match self {
            GroveKind::Calc => "calc",
            GroveKind::Units => "units",
            GroveKind::Bus => "bus",
        }
    }

    /// The coding hash the grove binds this kind by.
    pub(crate) fn hash_hex(self) -> &'static str {
        match self {
            GroveKind::Calc => "b55fba1eff65942099f6daf84b8bc47d605be05f637d805d260d3fcfd8c3ebde",
            GroveKind::Units => "2d5fcc96689b26df93fa86ace7525b4820d1bd7f68b46e9ce75ee77d7e2c9fda",
            GroveKind::Bus => "fa812abd9ab0b1b6a3aef72b3c8ca1e2641c6f307350a8cc86ebea56187b8123",
        }
    }
}
