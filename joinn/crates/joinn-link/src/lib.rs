//! Links between bodies. A membrane is the boundary of a body, computed, never stored.

#![forbid(unsafe_code)]

mod address;
mod assay;
mod assemble_universe;
mod bind;
mod body_store;
mod capability;
mod check_law4;
mod check_lenses;
mod check_link_types;
mod csr;
mod membrane;
mod structural;

pub(crate) use structural::refuse;
mod universe;
mod universe_state;

pub use address::Address;
pub use assay::{AssayReport, RegionPieces, assay, assay_reference, print_assay};
pub use assemble_universe::assemble_universe;
pub use bind::{Bound, bind};
pub use body_store::BodyStore;
pub use capability::{LinkRuntime, check_capability, grant, revoke};
pub use check_law4::{check_law4, law4_refusals};
pub use check_lenses::check_lenses;
pub use check_link_types::check_link_types;
pub use csr::{Csr, CsrMember, coding_from_csr, csr_from_universe};
pub use membrane::{BoundaryPort, membrane};
pub use universe::{
    BodyBinding, CrossWire, Galaxy, Lens, Link, Mark, Member, Order, System, Universe,
    UniverseCoding, UniverseRegulatory, hash_universe, parse_universe, print_universe,
};
pub use universe_state::{
    LinkRefusal, LinkRefusalKind, UniverseReport, UniverseState, format_link_refusal,
};

#[cfg(test)]
mod tests {
    #[test]
    fn a_string_reason_does_not_compile() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/fail/string_reason.rs");
    }

    #[test]
    fn bound_from_map_does_not_compile() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/fail/bound_from_map.rs");
    }
}
