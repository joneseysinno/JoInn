use super::Run;

pub fn run() -> Run {
    let mut text = String::new();
    text.push_str("s8 beam · exact in ℚ · kip and inch inside, feet shown\n");
    Run { text, clean: true }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn the_run_prints_the_plan_block() {
        let want = include_str!("../../expected.txt");
        let run = run();
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
