//! Print a strand's letters.

use super::Strand;

/// Grouped in threes by single spaces from the first letter; a last group may
/// be short. The empty strand prints nothing.
pub fn print_letters(strand: &Strand) -> String {
    let mut out = String::new();
    for (i, l) in strand.letters().iter().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(' ');
        }
        out.push(l.to_char());
    }
    out
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use super::print_letters;
    use crate::strand::parse_letters;

    #[test]
    fn letters_group_in_threes_from_the_head() {
        let Verdict::Ok(s) = parse_letters("AATAAATTTATTGC") else {
            panic!("reads");
        };
        assert_eq!(print_letters(&s), "AAT AAA TTT ATT GC");
    }
}
