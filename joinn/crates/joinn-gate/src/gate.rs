//! The evolution gate.

mod admit_allele;
mod admit_cell;
mod check_turns;
mod run_opposed;

pub use run_opposed::run_opposed;

use crate::budget::Budget;
use crate::eval::CellOracles;
use crate::natives::NativeRegistry;
use crate::oracle::Oracle;
use crate::testimony::TestimonyStore;
use joinn_frame::FrameRegistry;
use std::sync::Arc;

/// Successful admission.
#[derive(Clone, Debug)]
pub struct Accepted {
    /// Optional degeneracy advisory.
    pub advisory: Option<String>,
}

/// Frames + natives + testimony + budget.
pub struct Gate {
    pub(in crate::gate) frames: FrameRegistry,
    pub(in crate::gate) natives: NativeRegistry,
    pub(in crate::gate) cells: CellOracles,
    /// Accumulated testimony. Never identity.
    pub testimony: TestimonyStore,
    pub(in crate::gate) budget: Budget,
}

impl Gate {
    /// Construct a gate. Alleles are supplied by the caller (`joinn-prim`).
    pub fn new(budget: Budget, natives: NativeRegistry) -> Self {
        Self {
            frames: FrameRegistry::phase1(),
            natives,
            cells: CellOracles::new(),
            testimony: TestimonyStore::memory(),
            budget,
        }
    }

    /// Empty native register. Kept so Phase 1 tests that do not need alleles still compile.
    pub fn phase1(budget: Budget) -> Self {
        Self::new(budget, NativeRegistry::new())
    }

    /// Register another cell's oracle for `CellAt` terms.
    pub fn register_cell_oracle(&mut self, cell_hash: joinn_frame::Hash, oracle: Arc<dyn Oracle>) {
        self.cells.insert(cell_hash, oracle);
    }

    /// Frames.
    pub fn frames(&self) -> &FrameRegistry {
        &self.frames
    }

    /// Budget.
    pub fn budget(&self) -> Budget {
        self.budget
    }

    /// Resolve a native allele as an `Arc`.
    pub fn native_arc(&self, name: &str) -> Option<std::sync::Arc<dyn crate::oracle::Oracle>> {
        self.natives.get_arc(&joinn_dna::NativeId(name.into()))
    }

    /// Admit a coding region, optionally as an evolution of a parent.
    pub fn admit_cell(
        &self,
        proposed: &joinn_dna::Cell,
        parent: Option<&joinn_dna::Cell>,
    ) -> joinn_frame::Verdict<Accepted> {
        admit_cell::admit_cell(self, proposed, parent)
    }

    /// Admit an allele against a cell.
    pub fn admit_allele(
        &self,
        cell: &joinn_dna::Cell,
        allele: &joinn_dna::Allele,
    ) -> joinn_frame::Verdict<Accepted> {
        admit_allele::admit_allele(self, cell, allele)
    }
}

/// The bytes of a control artifact. `joinn-gate` carries them and never opens a file.
pub struct Artifact<'a> {
    /// Repo-relative path the bytes were read from.
    pub path: &'static str,
    /// File contents. A damaged copy is the same path with different bytes.
    pub bytes: &'a [u8],
}

/// One opposed gate item. `check` must be true; `control` must be false on the real artifact.
/// `control_artifact` is a repo-relative path. `joinn-gate` carries the string
/// and never opens it; `xtask` reads the bytes and passes them in.
pub struct GateItem {
    /// Printed name.
    pub name: &'static str,
    /// The witness this item must accept.
    pub check: fn() -> bool,
    /// The witness this item must refuse. True means the instrument is blind.
    pub control: for<'a> fn(&Artifact<'a>) -> bool,
    /// Repo-relative path of the control artifact. Not opened in this crate.
    pub control_artifact: &'static str,
}
