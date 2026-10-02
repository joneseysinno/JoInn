use num_rational::BigRational;
use num_traits::Signed;

use super::Edition;

/// Parses an edition: `edition <source>`, `<material> E <n> ksi` and
/// `<shape> Ix <n> in4`, one line each. A malformed edition is a host error.
pub fn parse(text: &str) -> Result<Edition, String> {
    let mut source = None;
    let mut material = None;
    let mut shape = None;
    let positive = |word: &str, line: &str| -> Result<BigRational, String> {
        match word.parse::<BigRational>() {
            Ok(n) if n.is_positive() => Ok(n),
            _ => Err(format!(
                "edition: `{line}` has no positive value; acceptance is a whole number or a fraction above 0"
            )),
        }
    };
    for line in text.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let words: Vec<&str> = line.split_whitespace().collect();
        let slot = match words.as_slice() {
            ["edition", ..] => {
                let rest = line.trim_start_matches("edition").trim().to_string();
                source.replace(rest).map(|_| "edition")
            }
            [name, "E", n, "ksi"] => material
                .replace((name.to_string(), positive(n, line)?))
                .map(|_| "E"),
            [name, "Ix", n, "in4"] => shape
                .replace((name.to_string(), positive(n, line)?))
                .map(|_| "Ix"),
            _ => {
                return Err(format!(
                    "edition: `{line}` is not an edition line; acceptance is `edition <source>`, `<material> E <n> ksi` or `<shape> Ix <n> in4`"
                ));
            }
        };
        if let Some(what) = slot {
            return Err(format!(
                "edition: a second {what} line `{line}`; acceptance is one of each"
            ));
        }
    }
    match (source, material, shape) {
        (Some(source), Some((material, e)), Some((shape, ix))) => Ok(Edition {
            source,
            material,
            e,
            shape,
            ix,
        }),
        _ => Err(
            "edition: a line is missing; acceptance is `edition <source>`, `<material> E <n> ksi` and `<shape> Ix <n> in4`".to_string(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::super::Edition;
    use super::parse;

    #[test]
    fn the_pinned_edition_parses_to_a992_and_w12x26() {
        let edition = parse(include_str!("../../bridges.edition"));
        assert_eq!(edition, Ok(Edition::aisc_for_tests()));
    }

    #[test]
    fn a_malformed_edition_is_a_host_error() {
        assert_eq!(
            parse("edition AISC\nA992 E 29000 ksi\n"),
            Err("edition: a line is missing; acceptance is `edition <source>`, `<material> E <n> ksi` and `<shape> Ix <n> in4`".to_string())
        );
        assert_eq!(
            parse("edition AISC\nA992 E 0 ksi\nW12x26 Ix 204 in4\n"),
            Err("edition: `A992 E 0 ksi` has no positive value; acceptance is a whole number or a fraction above 0".to_string())
        );
        assert_eq!(
            parse("edition AISC\nA992 E 29000 ksi\nA36 E 29000 ksi\nW12x26 Ix 204 in4\n"),
            Err(
                "edition: a second E line `A36 E 29000 ksi`; acceptance is one of each".to_string()
            )
        );
    }
}
