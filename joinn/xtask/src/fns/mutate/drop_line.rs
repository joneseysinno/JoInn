//! Drop a transcript line by index.

use joinn_frame::Verdict;

use super::refuse::refuse;

pub(super) fn drop_line(lines: &mut Vec<String>, i: usize) -> Verdict<()> {
    if i >= lines.len() {
        return Verdict::Refused(refuse(format!(
            "no such line {i}; acceptance is an index in 0..{}",
            lines.len()
        )));
    }
    lines.remove(i);
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use crate::fns::mutate::{Mutation, mutate};
    use crate::fns::parse_subject::parse_subject;
    use crate::fns::subject::Subject;
    use joinn_frame::Verdict;

    fn transcript() -> Subject {
        let src = include_str!("../../../../corpus/transcripts/universe.txt");
        parse_subject("corpus/transcripts/universe.txt", src)
            .unwrap_or_else(|e| panic!("parse transcript: {e}"))
    }

    #[test]
    fn drop_line_removes_refusal_line() {
        let s = transcript();
        let Subject::Transcript(orig) = &s else {
            panic!("transcript");
        };
        let before_len = orig.len();
        assert!(
            orig.get(1).is_some_and(|l| l.contains("refused")),
            "line 1 should be the refusal line: {:?}",
            orig.get(1)
        );
        let Verdict::Ok(mutant) = mutate(&s, &Mutation::DropLine(1)) else {
            panic!("mutate");
        };
        let Subject::Transcript(lines) = &mutant else {
            panic!("transcript mutant");
        };
        assert_eq!(lines.len(), before_len - 1);
        assert!(
            lines.iter().all(|l| !l.contains("refused")),
            "refusal line must be gone: {lines:?}"
        );
        let Verdict::Refused(r) = mutate(&s, &Mutation::DropLine(999)) else {
            panic!("missing line must refuse");
        };
        assert!(r.reason.contains("999"), "{}", r.reason);
    }
}
