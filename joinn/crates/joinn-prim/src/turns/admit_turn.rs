//! Generated round-trip over a cell.

use joinn_dna::TurnDecl;
use joinn_frame::Verdict;
use joinn_gate::Oracle;

use super::admit_turn_on::admit_turn_on;

/// Generated round-trip over a cell whose forward ins are `ins` and out is `fwd_out`.
pub fn admit_turn(
    turn: &TurnDecl,
    forward: &dyn Oracle,
    turn_allele: &dyn Oracle,
    seed: u64,
    n: u32,
) -> Verdict<()> {
    admit_turn_on(turn, forward, turn_allele, seed, n, &[0, 1], 2)
}
