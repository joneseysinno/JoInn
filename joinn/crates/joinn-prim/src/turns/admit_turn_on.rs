//! Round-trip with an explicit forward contract.

use joinn_dna::TurnDecl;
use joinn_frame::{IntFrame, Verdict};
use joinn_gate::Oracle;

/// Round-trip with an explicit forward contract, in ℤ.
pub fn admit_turn_on(
    turn: &TurnDecl,
    forward: &dyn Oracle,
    turn_allele: &dyn Oracle,
    seed: u64,
    n: u32,
    ins: &[u32],
    fwd_out: u32,
) -> Verdict<()> {
    let int = IntFrame::new();
    joinn_gate::check::turn::check(turn, forward, turn_allele, seed, n, ins, fwd_out, &int)
}

#[cfg(test)]
mod tests {
    use super::super::{AddIntTurn1, admit_turn};
    use crate::alleles::AddInt;
    use joinn_dna::TurnDecl;
    use joinn_frame::Verdict;

    #[test]
    fn turn1_from_0_2_admits() {
        let t = TurnDecl {
            out: 1,
            from: vec![0, 2],
        };
        match admit_turn(&t, &AddInt, &AddIntTurn1, 1, 64) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }
}
