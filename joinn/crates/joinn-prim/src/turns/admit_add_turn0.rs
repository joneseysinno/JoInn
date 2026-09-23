//! The honest turn of `add@ℤ`.

use crate::alleles::AddInt;
use joinn_dna::TurnDecl;
use joinn_frame::Verdict;

use super::add_int_turn0::AddIntTurn0;
use super::admit_turn::admit_turn;

/// The honest turn of `add@ℤ`.
pub fn admit_add_turn0() -> Verdict<()> {
    let t = TurnDecl {
        out: 0,
        from: vec![1, 2],
    };
    admit_turn(&t, &AddInt, &AddIntTurn0, 1, 128)
}

#[cfg(test)]
mod tests {
    use super::admit_add_turn0;
    use joinn_frame::Verdict;

    #[test]
    fn add_turn0_is_admitted() {
        match admit_add_turn0() {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }
}
