use super::Run;
use crate::bridge::pinned;
use crate::show::fraction;

/// The whole run. An edition that cannot be read is a host error.
pub fn run() -> Result<Run, String> {
    let edition = pinned()?;
    let mut text = String::new();
    text.push_str("s8 beam · exact in ℚ · kip and inch inside, feet shown\n");
    text.push_str(&format!(
        "bridges: {} E {} ksi · {} Ix {} in⁴ (pinned: {})\n",
        edition.material(),
        fraction(edition.e()),
        edition.shape(),
        fraction(edition.ix()),
        edition.source()
    ));
    Ok(Run { text, clean: true })
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn the_run_prints_the_plan_block() {
        let want = include_str!("../../expected.txt");
        let run = match run() {
            Ok(run) => run,
            Err(e) => panic!("host error: {e}"),
        };
        let got = run.text();
        if got != want {
            let mut got_lines = got.lines();
            let mut want_lines = want.lines();
            let mut n = 1;
            loop {
                match (got_lines.next(), want_lines.next()) {
                    (Some(g), Some(w)) if g == w => n += 1,
                    (g, w) => panic!(
                        "first differing line {n}:\n  got:  {}\n  want: {}",
                        g.unwrap_or("<end of output>"),
                        w.unwrap_or("<end of block>")
                    ),
                }
            }
        }
        assert!(run.clean());
    }
}
