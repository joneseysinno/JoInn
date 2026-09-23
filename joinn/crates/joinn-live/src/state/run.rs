//! Step until quiescent or refusal.

use joinn_frame::Verdict;

use crate::report::StepReport;
use crate::state::BodyState;

impl BodyState {
    pub fn run(&mut self) -> Verdict<Vec<StepReport>> {
        let mut reports = Vec::new();
        loop {
            match self.step() {
                Verdict::Ok(r) => {
                    let done = r.checks.iter().any(|c| c == "quiescent") && self.stack.len() == 1;
                    reports.push(r);
                    if done {
                        self.last_refusal = None;
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
