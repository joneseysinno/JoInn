//! Read libtest's summary lines out of a command's output.

/// Sum every `test result:` line's passed and failed counts, and name every
/// test libtest reported as `FAILED`.
pub(crate) fn test_counts(lines: &[String]) -> (u64, u64, Vec<String>) {
    let mut passed = 0;
    let mut failed = 0;
    let mut names = Vec::new();
    for line in lines {
        let line = line.trim_end();
        if let Some(rest) = line.strip_prefix("test result: ") {
            for part in rest.split(';') {
                let mut words = part.split_whitespace().rev();
                let (Some(label), Some(count)) = (words.next(), words.next()) else {
                    continue;
                };
                let count = count.parse::<u64>().unwrap_or(0);
                match label {
                    "passed" => passed += count,
                    "failed" => failed += count,
                    _ => {}
                }
            }
        } else if let Some(name) = line
            .strip_prefix("test ")
            .and_then(|rest| rest.strip_suffix(" ... FAILED"))
        {
            names.push(name.to_owned());
        }
    }
    (passed, failed, names)
}

#[cfg(test)]
mod tests {
    use super::test_counts;

    #[test]
    fn sums_every_binary_and_names_failures() {
        let text = "\
running 2 tests
test a::one ... ok
test a::two ... FAILED
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
running 3 tests
test result: ok. 3 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 0.00s";
        let lines: Vec<String> = text.lines().map(str::to_owned).collect();
        let (passed, failed, names) = test_counts(&lines);
        assert_eq!((passed, failed), (4, 1));
        assert_eq!(names, vec!["a::two".to_owned()]);
    }
}
