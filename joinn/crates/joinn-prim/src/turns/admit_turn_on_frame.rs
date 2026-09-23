//! Round-trip in an explicit frame.

use joinn_dna::TurnDecl;
use joinn_frame::{Frame, Verdict};
use joinn_gate::Oracle;

/// Round-trip in an explicit frame.
pub fn admit_turn_on_frame(
    turn: &TurnDecl,
    forward: &dyn Oracle,
    turn_allele: &dyn Oracle,
    seed: u64,
    n: u32,
    ins: &[u32],
    fwd_out: u32,
    frame: &dyn Frame,
) -> Verdict<()> {
    joinn_gate::check::turn::check(turn, forward, turn_allele, seed, n, ins, fwd_out, frame)
}
