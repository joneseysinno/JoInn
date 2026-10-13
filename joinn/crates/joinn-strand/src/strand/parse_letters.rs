//! Read a line of letters.

use joinn_frame::Verdict;

use super::Strand;
use crate::letter::Letter;
use crate::refuse::refuse;

/// Spaces group letters and are not letters. Any other character that isn't
/// A, T, G or C refuses the line, naming it and the letter it would have been.
pub fn parse_letters(line: &str) -> Verdict<Strand> {
    let mut letters = Vec::new();
    for c in line.chars() {
        if c == ' ' {
            continue;
        }
        match Letter::from_char(c) {
            Some(l) => letters.push(l),
            None => {
                return refuse(format!(
                    "strand: {c:?} at letter {} is not a letter; acceptance is A, T, G or C",
                    letters.len()
                ));
            }
        }
    }
    Verdict::Ok(Strand::new(letters))
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use super::parse_letters;

    #[test]
    fn a_letter_that_isnt_one_is_refused_where_it_stands() {
        match parse_letters("AAT AXT") {
            Verdict::Refused(r) => assert_eq!(
                r.reason,
                "strand: 'X' at letter 4 is not a letter; acceptance is A, T, G or C"
            ),
            Verdict::Ok(s) => panic!("X read as a letter: {s:?}"),
        }
        assert!(parse_letters("aat").is_refused(), "letters are capitals");
    }

    #[test]
    fn spaces_only_group() {
        let (Verdict::Ok(a), Verdict::Ok(b)) = (parse_letters("AATATT"), parse_letters("AAT ATT"))
        else {
            panic!("both read");
        };
        assert_eq!(a, b);
        assert_eq!(a.len(), 6);
    }
}
