//! One engine step. Probe is this, printed.

use std::fmt::Write as _;

/// One engine step. Probe is this, printed.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StepReport {
    /// Step index, starting at 1.
    pub step: u64,
    /// Instance that fired, if any.
    pub fired: Option<String>,
    /// Declared turn used, if any.
    pub direction: Option<u32>,
    /// Delivered (wire identity, value print).
    pub delivered: Vec<(String, String)>,
    /// Require/ensure outcomes.
    pub checks: Vec<String>,
    /// Nested activation depth. Printed only when non-zero.
    pub depth: u32,
}

impl StepReport {
    /// Canonical bytes for determinism tests.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut s = String::new();
        let _ = writeln!(s, "step {}", self.step);
        match &self.fired {
            Some(f) => {
                let _ = writeln!(s, "fired {f}");
            }
            None => {
                let _ = writeln!(s, "fired none");
            }
        }
        match self.direction {
            Some(d) => {
                let _ = writeln!(s, "direction {d}");
            }
            None => {
                let _ = writeln!(s, "direction none");
            }
        }
        for (w, v) in &self.delivered {
            let _ = writeln!(s, "delivered {w} {v}");
        }
        for c in &self.checks {
            let _ = writeln!(s, "check {c}");
        }
        if self.depth != 0 {
            let _ = writeln!(s, "depth {}", self.depth);
        }
        s.into_bytes()
    }
}
