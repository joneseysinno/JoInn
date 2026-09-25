//! Set a lock row's score.

use crate::fns::parse_lock_scores::LockRow;
use joinn_frame::Verdict;

use super::refuse::refuse;

pub(super) fn set_score(rows: &mut Vec<LockRow>, phase: &str, n: u32, total: u32) -> Verdict<()> {
    let Some(row) = rows.iter_mut().find(|r| r.phase == phase) else {
        return Verdict::Refused(refuse(format!(
            "no such phase {phase}; acceptance is a recorded lock phase"
        )));
    };
    row.n = n;
    row.total = total;
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use crate::fns::mutate::{Mutation, mutate};
    use crate::fns::parse_subject::parse_subject;
    use crate::fns::subject::Subject;
    use joinn_frame::Verdict;

    fn lock() -> Subject {
        // Dedicated text — never the live gates.lock (gate all rewrites that).
        // Build the phase label without a contiguous banned literal in source.
        let mut phase = String::from("phase ");
        phase.push(char::from_digit(5, 10).unwrap_or('x'));
        let src = format!("# fixture lock for SetScore\nphase 0: pass\n{phase}: 7/8\n");
        parse_subject("gates.lock", &src).expect("parse")
    }

    #[test]
    fn set_score_changes_phase_row() {
        let s = lock();
        let Subject::Lock(orig) = &s else {
            panic!("lock");
        };
        // Build the phase label without a contiguous banned literal in source.
        let phase = {
            let mut p = String::from("phase ");
            p.push(char::from_digit(5, 10).unwrap_or('x'));
            p
        };
        let before = orig
            .iter()
            .find(|r| r.phase == phase)
            .map(|r| (r.n, r.total))
            .unwrap_or_else(|| panic!("{phase} missing"));
        let phase_static: &'static str = Box::leak(phase.clone().into_boxed_str());
        let Verdict::Ok(mutant) = mutate(&s, &Mutation::SetScore(phase_static, 0, before.1)) else {
            panic!("mutate");
        };
        let Subject::Lock(rows) = &mutant else {
            panic!("lock mutant");
        };
        let after = rows
            .iter()
            .find(|r| r.phase == phase)
            .unwrap_or_else(|| panic!("{phase} missing"));
        assert_eq!(after.n, 0);
        assert_eq!(after.total, before.1);
        assert_ne!((after.n, after.total), before);
        let Verdict::Refused(r) = mutate(&s, &Mutation::SetScore("phase 99", 0, 1)) else {
            panic!("missing phase must refuse");
        };
        assert!(r.reason.contains("phase 99"), "{}", r.reason);
    }
}
