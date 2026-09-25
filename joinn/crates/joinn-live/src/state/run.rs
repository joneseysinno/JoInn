//! Step until quiescent or refusal.

use joinn_frame::Verdict;

use crate::report::StepReport;
use crate::state::BodyState;

impl BodyState {
    pub fn run(&mut self) -> Verdict<Vec<StepReport>> {
        self.refusal_site = None;
        let mut reports = Vec::new();
        loop {
            match self.step() {
                Verdict::Ok(r) => {
                    if r.fired.is_some() {
                        self.last_refusal = None;
                    }
                    let done = r.checks.iter().any(|c| c == "quiescent") && self.stack.len() == 1;
                    reports.push(r);
                    if done {
                        return Verdict::Ok(reports);
                    }
                }
                Verdict::Refused(r) => {
                    self.last_refusal = Some(r.reason.clone());
                    return Verdict::Refused(r);
                }
            }
        }
    }
}
