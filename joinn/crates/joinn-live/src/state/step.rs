//! One fold step. Never recurses.

use joinn_frame::{CheckId, Refusal, Verdict};

use crate::report::StepReport;
use crate::state::BodyState;

impl BodyState {
    pub fn step(&mut self) -> Verdict<StepReport> {
        if self.step >= self.budget {
            return self.budget_refusal();
        }
        let top = self.stack.len().saturating_sub(1);
        let empty = self
            .stack
            .get(top)
            .map(|a| a.queue.is_empty())
            .unwrap_or(true);
        if empty {
            if self.stack.len() > 1 {
                return self.pop_nested();
            }
            self.step += 1;
            return Verdict::Ok(StepReport {
                step: self.step,
                fired: None,
                direction: None,
                delivered: Vec::new(),
                checks: vec!["quiescent".into()],
                depth: 0,
            });
        }
        self.step += 1;
        let key = {
            let Some(act) = self.stack.get(top) else {
                return Verdict::Refused(Refusal::structural(CheckId::Other, "empty stack"));
            };
            match act.queue.iter().next().cloned() {
                Some(k) => k,
                None => {
                    return Verdict::Ok(StepReport {
                        step: self.step,
                        fired: None,
                        direction: None,
                        delivered: Vec::new(),
                        checks: vec!["quiescent".into()],
                        depth: self.depth(),
                    });
                }
            }
        };
        if let Some(act) = self.stack.get_mut(top) {
            act.queue.remove(&key);
        }
        let Some(mail) = self.stack.get_mut(top).and_then(|a| a.mail.remove(&key)) else {
            return Verdict::Refused(Refusal::structural(CheckId::Other, "missing mail"));
        };
        self.last_mail = Some(format!(
            "{}@{} {}",
            mail.dest,
            mail.port,
            mail.value.print_literal()
        ));
        self.deliver(top, mail)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::fixtures::{natives, sum_cli_body, text};
    use std::collections::BTreeSet;

    #[test]
    fn cli_success_steps_are_unique() {
        let (body, cells) = sum_cli_body();
        let mut state = match BodyState::new(body, cells, natives(), 1) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let _ = state.inject("cli_a", 0, text("2"), 0);
        let _ = state.inject("cli_b", 0, text("3"), 1);
        match state.run() {
            Verdict::Ok(rs) => {
                let steps: Vec<u64> = rs.iter().map(|r| r.step).collect();
                let unique: BTreeSet<u64> = steps.iter().copied().collect();
                assert_eq!(steps.len(), unique.len(), "duplicate step in {steps:?}");
                assert!(
                    rs.last()
                        .is_some_and(|r| r.checks.iter().any(|c| c == "quiescent")),
                    "last report must be quiescent"
                );
            }
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    #[test]
    fn determinism_100_runs_two_seeds() {
        let (body, cells) = sum_cli_body();
        let natives = natives();
        let run = |seed: u64| {
            let mut state = match BodyState::new(body.clone(), cells.clone(), natives.clone(), seed)
            {
                Verdict::Ok(s) => s,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            let _ = state.inject("cli_a", 0, text("2"), 0);
            let _ = state.inject("cli_b", 0, text("3"), 1);
            match state.run() {
                Verdict::Ok(rs) => rs.iter().flat_map(|r| r.to_bytes()).collect::<Vec<_>>(),
                Verdict::Refused(r) => panic!("{}", r.reason),
            }
        };
        let a = run(1);
        for _ in 0..100 {
            assert_eq!(run(1), a);
        }
        let b = run(2);
        assert_eq!(a, b);
    }
}
