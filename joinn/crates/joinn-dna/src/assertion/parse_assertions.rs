//! `parse_assertions`.

use joinn_frame::{CheckId, Refusal, Verdict};

use crate::assertion::Assertion;

/// Parse the text of a `declarations` section, one sentence per line.
pub fn parse_assertions(text: &str) -> Verdict<Vec<Assertion>> {
    let mut out = Vec::new();
    for line in text.lines() {
        let sentence = line.trim();
        if sentence.is_empty() {
            continue;
        }
        let parsed = if sentence == Assertion::H1Zero.sentence() {
            Assertion::H1Zero
        } else {
            return Verdict::Refused(Refusal::structural(
                CheckId::Parse,
                format!("declaration {sentence} is not admitted; acceptance is assert H₁ = 0"),
            ));
        };
        if out.contains(&parsed) {
            return Verdict::Refused(Refusal::structural(
                CheckId::Parse,
                format!("declaration {sentence} declared twice; acceptance is each sentence once"),
            ));
        }
        out.push(parsed);
    }
    Verdict::Ok(out)
}

#[cfg(test)]
mod tests {
    use super::parse_assertions;
    use crate::assertion::Assertion;
    use joinn_frame::Verdict;

    #[test]
    fn the_one_sentence_parses() {
        match parse_assertions("\n  assert H₁ = 0  \n") {
            Verdict::Ok(list) => assert_eq!(list, vec![Assertion::H1Zero]),
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
        match parse_assertions("  \n") {
            Verdict::Ok(list) => assert!(list.is_empty()),
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    #[test]
    fn other_text_is_refused_naming_it() {
        for text in ["assert H₂ = 0", "assert  H₁ = 0", "assert H1 = 0", "H₁ = 0"] {
            match parse_assertions(text) {
                Verdict::Refused(r) => {
                    assert!(r.reason.contains(text.trim()), "{}", r.reason);
                    assert!(
                        r.reason.contains("acceptance is assert H₁ = 0"),
                        "{}",
                        r.reason
                    );
                }
                Verdict::Ok(_) => panic!("{text} must be refused"),
            }
        }
    }

    #[test]
    fn a_sentence_twice_is_refused() {
        match parse_assertions("assert H₁ = 0\nassert H₁ = 0\n") {
            Verdict::Refused(r) => {
                assert!(r.reason.contains("assert H₁ = 0"), "{}", r.reason);
                assert!(r.reason.contains("declared twice"), "{}", r.reason);
            }
            Verdict::Ok(_) => panic!("a repeated sentence must be refused"),
        }
    }
}
