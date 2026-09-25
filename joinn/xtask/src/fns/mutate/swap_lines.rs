//! Swap two transcript lines by index.

use joinn_frame::Verdict;

use super::refuse::refuse;

pub(super) fn swap_lines(lines: &mut Vec<String>, i: usize, j: usize) -> Verdict<()> {
    if i >= lines.len() {
        return Verdict::Refused(refuse(format!(
            "no such line {i}; acceptance is an index in 0..{}",
            lines.len()
        )));
    }
    if j >= lines.len() {
        return Verdict::Refused(refuse(format!(
            "no such line {j}; acceptance is an index in 0..{}",
            lines.len()
        )));
    }
    lines.swap(i, j);
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use crate::fns::mutate::{Mutation, mutate};
    use crate::fns::parse_subject::parse_subject;
    use crate::fns::subject::Subject;
    use crate::fns::workspace_root::workspace_root;
    use joinn_frame::Verdict;
    use std::fs;

    fn transcript() -> Subject {
        let src = include_str!("../../../../corpus/transcripts/universe.txt");
        parse_subject("corpus/transcripts/universe.txt", src).expect("parse")
    }

    #[test]
    fn swap_lines_breaks_calculator_prefix() {
        let s = transcript();
        let Subject::Transcript(orig) = &s else {
            panic!("transcript");
        };
        let calc = fs::read_to_string(
            workspace_root()
                .expect("root")
                .join("corpus/transcripts/calculator.txt"),
        )
        .expect("calculator.txt");
        let calc_lines: Vec<&str> = calc.lines().collect();
        assert!(
            orig.len() >= calc_lines.len()
                && orig.iter().zip(calc_lines.iter()).all(|(a, b)| a == *b),
            "subject must start as calculator.txt"
        );
        let Verdict::Ok(mutant) = mutate(&s, &Mutation::SwapLines(2, 3)) else {
            panic!("mutate");
        };
        let Subject::Transcript(lines) = &mutant else {
            panic!("transcript mutant");
        };
        let still_prefix = lines.len() >= calc_lines.len()
            && lines.iter().zip(calc_lines.iter()).all(|(a, b)| a == *b);
        assert!(
            !still_prefix,
            "swapped transcript must not start as calculator.txt"
        );
        let Verdict::Refused(r) = mutate(&s, &Mutation::SwapLines(2, 999)) else {
            panic!("missing line must refuse");
        };
        assert!(r.reason.contains("999"), "{}", r.reason);
    }
}
