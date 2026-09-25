//! Replace a text subject's entire contents.

use joinn_frame::Verdict;

pub(super) fn replace(text: &mut String, with: &str) -> Verdict<()> {
    *text = with.to_owned();
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use crate::fns::mutate::{Mutation, mutate};
    use crate::fns::parse_subject::parse_subject;
    use crate::fns::subject::Subject;
    use joinn_frame::Verdict;

    fn inner_reason() -> Subject {
        let src = include_str!("../../../../corpus/phase5/controls/inner_reason.txt");
        parse_subject("phase5/controls/inner_reason.txt", src)
            .unwrap_or_else(|e| panic!("parse inner_reason: {e}"))
    }

    #[test]
    fn replace_puts_e0_in_the_text() {
        let s = inner_reason();
        let Subject::Text(orig) = &s else {
            panic!("text");
        };
        assert!(!orig.contains("e0"), "subject must not already be e0");
        let Verdict::Ok(mutant) = mutate(&s, &Mutation::Replace("e0")) else {
            panic!("mutate");
        };
        let Subject::Text(text) = &mutant else {
            panic!("text mutant");
        };
        assert_eq!(text, "e0");
        // Downstream: a host far-side check looking for this string would fire.
        assert!(text.contains("e0"));
        // Replace has no "missing target"; a second replace still succeeds.
        let Verdict::Ok(again) = mutate(&mutant, &Mutation::Replace("other")) else {
            panic!("replace again");
        };
        let Subject::Text(t2) = again else {
            panic!("text");
        };
        assert_eq!(t2, "other");
        assert_ne!(t2, *orig);
    }
}
