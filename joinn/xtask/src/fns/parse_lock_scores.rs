//! Parse a gates.lock into the scores it recorded.

#[derive(Clone, Debug)]
pub(crate) struct LockRow {
    pub phase: String,
    pub n: u32,
    pub total: u32,
}

pub(crate) fn parse_lock_scores(text: &str) -> Result<Vec<LockRow>, String> {
    let mut rows = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((phase, score)) = line.split_once(':') else {
            return Err(format!("lock line is not a score: {line}"));
        };
        let phase = phase.trim().to_owned();
        let score = score.trim();
        if score == "pass" {
            rows.push(LockRow {
                phase,
                n: 1,
                total: 1,
            });
            continue;
        }
        if score == "fail" {
            rows.push(LockRow {
                phase,
                n: 0,
                total: 1,
            });
            continue;
        }
        let Some((num, den)) = score.split_once('/') else {
            return Err(format!("{phase}: score is not n/total: {score}"));
        };
        let n: u32 = num
            .parse()
            .map_err(|_| format!("{phase}: score numerator is not a count: {num}"))?;
        let total: u32 = den
            .parse()
            .map_err(|_| format!("{phase}: score denominator is not a count: {den}"))?;
        rows.push(LockRow { phase, n, total });
    }
    if rows.is_empty() {
        return Err("lock has no scores".into());
    }
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::parse_lock_scores;

    #[test]
    fn bad_lock_names_phase_3() {
        let src = include_str!("../../../corpus/phase3/controls/bad.lock");
        match parse_lock_scores(src) {
            Ok(rows) => {
                assert!(
                    rows.iter()
                        .any(|row| row.phase.contains("phase 3") && row.n != row.total),
                    "{rows:?}"
                );
            }
            Err(msg) => panic!("{msg}"),
        }
    }
}
