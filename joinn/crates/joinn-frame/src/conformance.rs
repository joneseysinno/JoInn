//! Frame obligation harness. The obligation set is a test, not a comment.

mod check;
mod fo1;
mod fo10;
mod fo2;
mod fo3;
mod fo4;
mod fo5;
mod fo6;
mod fo7;
mod fo8;
mod fo9;

pub use check::check;

use crate::frame::{Case, Frame, FrameRef, OpName};
use crate::term::Term;
use crate::verdict::{CheckId, Refusal, Subject, Verdict};

/// Named obligation. A refusal names the one that failed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Obligation {
    /// Equality coincides with canonical identity.
    Fo1,
    /// `parse(print(v)) = v`.
    Fo2,
    /// `canonicalize` is idempotent.
    Fo3,
    /// Generated values satisfy `contains`.
    Fo4,
    /// `shrink` terminates, never grows, never cycles.
    Fo5,
    /// Signature ops never panic; out-of-frame args refuse.
    Fo6,
    /// For each extension: ρ(ι(x)) = x; ι preserves signature ops and eq.
    Fo7,
    /// `case` inverts the constructors.
    Fo8,
    /// `case` is well-founded: parts are strictly smaller; a bounded walk reaches a generator.
    Fo9,
    /// `case`'s reported constructor is `constructors()` at that tag.
    Fo10,
}

impl Obligation {
    /// Stable name for refusals and tests.
    pub fn name(self) -> &'static str {
        match self {
            Obligation::Fo1 => "FO1",
            Obligation::Fo2 => "FO2",
            Obligation::Fo3 => "FO3",
            Obligation::Fo4 => "FO4",
            Obligation::Fo5 => "FO5",
            Obligation::Fo6 => "FO6",
            Obligation::Fo7 => "FO7",
            Obligation::Fo8 => "FO8",
            Obligation::Fo9 => "FO9",
            Obligation::Fo10 => "FO10",
        }
    }
}

/// Outcome of a harness run.
#[derive(Clone, Debug)]
pub struct Report {
    /// True when every obligation passed.
    pub passed: bool,
    /// Failures in the order they were found.
    pub failures: Vec<String>,
}

impl Report {
    /// Convert a failed report into a refusal naming the first obligation.
    pub fn into_verdict(self, seed: u64) -> Verdict<Report> {
        if self.passed {
            Verdict::Ok(self)
        } else {
            let reason = self
                .failures
                .first()
                .cloned()
                .unwrap_or_else(|| "frame obligation failed".into());
            Verdict::Refused(Refusal {
                check: CheckId::FrameObligation,
                subject: Subject::Frame(String::new()),
                reason,
                counterexample: None,
                seed,
            })
        }
    }
}

pub(in crate::conformance) fn shrink_ok(frame: &dyn Frame, start: &crate::value::Value) -> bool {
    let mut current = start.clone();
    let mut seen = Vec::new();
    for _ in 0..64 {
        if seen.iter().any(|s| frame.eq(s, &current)) {
            return false;
        }
        seen.push(current.clone());
        let kids = frame.shrink(&current);
        if kids.is_empty() {
            return true;
        }
        for k in &kids {
            if value_size(k) > value_size(&current) {
                return false;
            }
        }
        current = kids[0].clone();
    }
    false
}

pub(in crate::conformance) fn value_size(v: &crate::value::Value) -> usize {
    term_size(v.term())
}

pub(in crate::conformance) fn term_size(t: &Term) -> usize {
    match t {
        Term::Int(n) => n.to_string().len(),
        Term::Text(s) => s.len(),
        Term::Seq(xs) => xs.iter().map(term_size).sum::<usize>().saturating_add(1),
    }
}

pub(in crate::conformance) fn preserves_op(
    wide: &dyn Frame,
    narrow: &dyn Frame,
    ext: &FrameRef,
    op: &OpName,
    arity: usize,
    seed: u64,
    i: u32,
) -> bool {
    let mut args_n = Vec::new();
    let mut args_w = Vec::new();
    for k in 0..arity {
        let v = narrow.generate(seed.wrapping_add(u64::from(i + k as u32) + 303), 8);
        let ev = match wide.embed(ext, &v) {
            Verdict::Ok(x) => x,
            Verdict::Refused(_) => return false,
        };
        args_n.push(v);
        args_w.push(ev);
    }
    match (narrow.apply_op(op, &args_n), wide.apply_op(op, &args_w)) {
        (Verdict::Ok(rn), Verdict::Ok(rw)) => match wide.embed(ext, &rn) {
            Verdict::Ok(ern) => wide.eq(&ern, &rw),
            Verdict::Refused(_) => false,
        },
        (Verdict::Refused(_), Verdict::Refused(_)) => true,
        _ => false,
    }
}

pub(in crate::conformance) fn fo8_one(
    frame: &dyn Frame,
    v: &crate::value::Value,
    generators: &[OpName],
    seed: u64,
    failures: &mut Vec<String>,
) -> bool {
    match frame.case(v) {
        Case::Generator => {
            let mut ok = false;
            for op in generators {
                if let Verdict::Ok(g) = frame.apply_op(op, &[])
                    && frame.eq(&g, v)
                {
                    ok = true;
                    break;
                }
            }
            if !ok {
                failures.push(format!(
                    "FO8 case(v) = Generator but v is not a declared generator (seed {seed})"
                ));
                return false;
            }
        }
        Case::Built { op, parts } => {
            if !frame.signature().contains(&op) {
                failures.push(format!(
                    "FO8 case named unknown op {} (seed {seed})",
                    op.as_str()
                ));
                return false;
            }
            match frame.apply_op(&op, &parts) {
                Verdict::Ok(back) if frame.eq(&back, v) => {}
                Verdict::Ok(_) => {
                    failures.push(format!(
                        "FO8 apply_op({}) on case parts ≠ v (seed {seed})",
                        op.as_str()
                    ));
                    return false;
                }
                Verdict::Refused(_) => {
                    failures.push(format!(
                        "FO8 apply_op({}) on case parts refused (seed {seed})",
                        op.as_str()
                    ));
                    return false;
                }
            }
        }
    }
    true
}

pub(in crate::conformance) fn fo9_ok(frame: &dyn Frame, start: &crate::value::Value) -> bool {
    let mut current = start.clone();
    for _ in 0..64 {
        match frame.case(&current) {
            Case::Generator => return true,
            Case::Built { op, parts } => {
                if parts.is_empty() {
                    return false;
                }
                for (i, p) in parts.iter().enumerate() {
                    if !part_smaller(frame, &op, i, p, &current) {
                        return false;
                    }
                }
                // Spine: first part in the same frame, else first part (ℚ → ℤ).
                let next = parts
                    .iter()
                    .find(|p| p.frame() == current.frame())
                    .or_else(|| parts.first());
                let Some(next) = next else {
                    return false;
                };
                if next.frame() != current.frame() {
                    // Cross-frame parts: check each with its own frame, one level.
                    for p in &parts {
                        if !fo9_cross(p) {
                            return false;
                        }
                    }
                    return true;
                }
                // Character atom of cons: rest is the spine.
                if op.as_str() == "cons" && parts.len() == 2 {
                    current = parts[1].clone();
                    continue;
                }
                current = next.clone();
            }
        }
    }
    // Bound reached with strictly decreasing parts: the measure is still well-founded.
    true
}

pub(in crate::conformance) fn fo9_cross(p: &crate::value::Value) -> bool {
    match p.frame().id {
        crate::frame::FrameId::Int => {
            let f = crate::frames::IntFrame::new();
            fo9_ok(&f, p)
        }
        crate::frame::FrameId::Text => {
            let f = crate::frames::TextFrame::new();
            fo9_ok(&f, p)
        }
        crate::frame::FrameId::Rat => {
            let f = crate::frames::RatFrame::new();
            fo9_ok(&f, p)
        }
    }
}

pub(in crate::conformance) fn part_smaller(
    frame: &dyn Frame,
    op: &OpName,
    idx: usize,
    part: &crate::value::Value,
    whole: &crate::value::Value,
) -> bool {
    if op.as_str() == "cons" && idx == 0 {
        return matches!(part.term(), Term::Text(s) if s.chars().count() == 1);
    }
    if part.frame() != whole.frame() {
        return true;
    }
    match (part.term(), whole.term()) {
        (Term::Int(pn), Term::Int(wn)) => {
            use num_traits::Signed;
            pn.abs() < wn.abs()
        }
        (Term::Text(ps), Term::Text(ws)) => ps.chars().count() < ws.chars().count(),
        _ => value_size(part) < value_size(whole) && !frame.eq(part, whole),
    }
}

pub(in crate::conformance) fn fo10_one(
    frame: &dyn Frame,
    v: &crate::value::Value,
    ctors: &[OpName],
    seed: u64,
    failures: &mut Vec<String>,
    saw_built: &mut bool,
) -> bool {
    match frame.case(v) {
        Case::Generator => {
            if ctors.is_empty() {
                failures.push(format!(
                    "FO10 case(v) = Generator but constructors() is empty (seed {seed})"
                ));
                return false;
            }
            true
        }
        Case::Built { op, .. } => {
            *saw_built = true;
            if ctors.is_empty() {
                failures.push(format!(
                    "FO10 constructors() is empty for a Built case (seed {seed})"
                ));
                return false;
            }
            match ctors.iter().position(|c| c.as_str() == op.as_str()) {
                Some(0) => {
                    failures.push(format!(
                        "FO10 constructor {} is at tag 0; tag 0 is the generator (seed {seed})",
                        op.as_str()
                    ));
                    false
                }
                Some(_) => true,
                None => {
                    failures.push(format!(
                        "FO10 case named {} which is not in constructors() (seed {seed})",
                        op.as_str()
                    ));
                    false
                }
            }
        }
    }
}
