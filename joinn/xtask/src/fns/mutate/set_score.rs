//! Set a lock row's score.

use crate::fns::parse_lock_scores::LockRow;
use joinn_frame::Verdict;

use super::refuse::refuse;

pub(super) fn set_score(rows: &mut [LockRow], phase: &str, n: u32, total: u32) -> Verdict<()> {
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
    use crate::fns::gate_all::PHASE_LABELS;
    use crate::fns::mutate::{Mutation, mutate};
    use crate::fns::parse_subject::parse_subject;
    use crate::fns::scores_match::scores_match;
    use crate::fns::subject::Subject;
    use joinn_frame::Verdict;

    fn lock() -> Subject {
        let phase = PHASE_LABELS[6];
        let src = format!("# fixture lock for SetScore\nphase 0: pass\n{phase}: 7/8\n");
        parse_subject("gates.lock", &src).unwrap_or_else(|e| panic!("parse gates.lock: {e}"))
    }

    #[test]
    fn set_score_breaks_scores_match() {
        let s = lock();
        let Subject::Lock(orig) = &s else {
            panic!("lock");
        };
        let phase = PHASE_LABELS[6];
        let before = orig
            .iter()
            .find(|r| r.phase == phase)
            .map(|r| (r.n, r.total))
            .unwrap_or_else(|| panic!("{phase} missing"));
        let Verdict::Ok(mutant) = mutate(&s, &Mutation::SetScore(phase, 0, before.1)) else {
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
        let returned = [(phase, true, before.0, before.1, false)];
        match scores_match(&returned, rows) {
            Err(msg) => assert!(msg.contains(phase), "{msg}"),
            Ok(()) => panic!("scores_match must refuse naming {phase}"),
        }
        let Verdict::Refused(r) = mutate(&s, &Mutation::SetScore("phase 99", 0, 1)) else {
            panic!("missing phase must refuse");
        };
        assert!(r.reason.contains("phase 99"), "{}", r.reason);
    }
}
