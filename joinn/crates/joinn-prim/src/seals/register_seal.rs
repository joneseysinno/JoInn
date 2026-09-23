//! Register a seal.

use super::Seal;
use joinn_frame::Verdict;

/// Register a seal. `drives` is total, so the missing-drive refusal is unrepresentable.
pub fn register_seal(seal: &Seal) -> Verdict<()> {
    let _ = seal;
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::{BodyRef, Drive, Seal};
    use super::register_seal;
    use joinn_dna::{NativeId, hash, sum_cell};
    use joinn_frame::Verdict;
    use std::num::NonZeroU32;

    #[test]
    fn one_way_is_data_on_the_seal() {
        let seal = Seal {
            cell: hash(&sum_cell().coding),
            reference: BodyRef {
                hash: hash(&sum_cell().coding),
            },
            counterfeit: BodyRef {
                hash: hash(&sum_cell().coding),
            },
            sealed: NativeId("parse@Text".into()),
            drives: Drive::new(0, NonZeroU32::new(32).unwrap_or(NonZeroU32::MIN)),
            one_way: true,
        };
        assert!(seal.one_way);
        match register_seal(&seal) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }
}
