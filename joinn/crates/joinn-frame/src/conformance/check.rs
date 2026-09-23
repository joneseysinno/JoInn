//! Run FO1–FO10 on a frame.

use super::Report;
use crate::frame::Frame;
use crate::verdict::Verdict;

pub fn check(frame: &dyn Frame, seed: u64, n: u32) -> Verdict<Report> {
    let mut failures = Vec::new();
    super::fo1::fo1(frame, seed, n, &mut failures);
    super::fo2::fo2(frame, seed, n, &mut failures);
    super::fo3::fo3(frame, seed, n, &mut failures);
    super::fo4::fo4(frame, seed, n, &mut failures);
    super::fo5::fo5(frame, seed, n, &mut failures);
    super::fo6::fo6(frame, seed, n, &mut failures);
    super::fo7::fo7(frame, seed, n, &mut failures);
    super::fo8::fo8(frame, seed, n, &mut failures);
    super::fo9::fo9(frame, seed, n, &mut failures);
    super::fo10::fo10(frame, seed, n, &mut failures);
    let report = Report {
        passed: failures.is_empty(),
        failures,
    };
    report.into_verdict(seed)
}

#[cfg(test)]
mod tests {
    use super::super::{Obligation, Report};
    use super::check;
    use crate::Frame;
    use crate::frame::OpName;
    use crate::frames::{IntFrame, RatFrame, TextFrame};
    use crate::term::Term;
    use crate::verdict::{CheckId, Refusal, Verdict};

    struct BrokenFrame {
        inner: IntFrame,
        which: Obligation,
    }

    impl BrokenFrame {
        fn new(which: Obligation) -> Self {
            Self {
                inner: IntFrame::new(),
                which,
            }
        }
    }

    impl Frame for BrokenFrame {
        fn reference(&self) -> crate::FrameRef {
            self.inner.reference()
        }
        fn signature(&self) -> &crate::Signature {
            self.inner.signature()
        }
        fn apply_op(&self, op: &OpName, args: &[crate::Value]) -> Verdict<crate::Value> {
            self.inner.apply_op(op, args)
        }
        fn contains(&self, t: &Term) -> bool {
            self.inner.contains(t)
        }
        fn canonicalize(&self, t: Term) -> Verdict<crate::Value> {
            match self.inner.canonicalize(t) {
                Verdict::Ok(v) if self.which == Obligation::Fo3 => {
                    if let Term::Int(n) = v.term() {
                        return self.inner.canonicalize(Term::Int(n + 1));
                    }
                    Verdict::Ok(v)
                }
                other => other,
            }
        }
        fn eq(&self, a: &crate::Value, b: &crate::Value) -> bool {
            if self.which == Obligation::Fo1 {
                true
            } else {
                self.inner.eq(a, b)
            }
        }
        fn print(&self, v: &crate::Value) -> String {
            if self.which == Obligation::Fo2 {
                format!("{}!", self.inner.print(v))
            } else {
                self.inner.print(v)
            }
        }
        fn parse(&self, s: &str) -> Verdict<crate::Value> {
            self.inner.parse(s)
        }
        fn generate(&self, seed: u64, size: u8) -> crate::Value {
            self.inner.generate(seed, size)
        }
        fn shrink(&self, v: &crate::Value) -> Vec<crate::Value> {
            if self.which == Obligation::Fo5 {
                let grown = self.inner.generate(99, 32);
                vec![grown]
            } else {
                self.inner.shrink(v)
            }
        }
        fn extensions(&self) -> &[crate::FrameRef] {
            self.inner.extensions()
        }
        fn embed(&self, from: &crate::FrameRef, v: &crate::Value) -> Verdict<crate::Value> {
            if self.which == Obligation::Fo7 {
                Verdict::Refused(Refusal::structural(
                    CheckId::FrameObligation,
                    "broken embed",
                ))
            } else {
                self.inner.embed(from, v)
            }
        }
        fn restrict(&self, to: &crate::FrameRef, v: &crate::Value) -> Verdict<crate::Value> {
            self.inner.restrict(to, v)
        }
        fn case(&self, v: &crate::Value) -> crate::Case {
            if self.which == Obligation::Fo8 {
                crate::Case::Generator
            } else if self.which == Obligation::Fo9 {
                match self.inner.case(v) {
                    crate::Case::Generator => crate::Case::Generator,
                    crate::Case::Built { .. } => {
                        match self
                            .inner
                            .apply_op(&OpName("succ".into()), std::slice::from_ref(v))
                        {
                            Verdict::Ok(bigger) => crate::Case::Built {
                                op: OpName("pred".into()),
                                parts: vec![bigger],
                            },
                            Verdict::Refused(_) => self.inner.case(v),
                        }
                    }
                }
            } else {
                self.inner.case(v)
            }
        }
        fn constructors(&self) -> &[OpName] {
            if self.which == Obligation::Fo10 {
                static LIE: std::sync::OnceLock<Vec<OpName>> = std::sync::OnceLock::new();
                LIE.get_or_init(|| {
                    vec![
                        OpName("succ".into()),
                        OpName("zero".into()),
                        OpName("pred".into()),
                    ]
                })
            } else {
                self.inner.constructors()
            }
        }
    }

    fn refused_named(v: Verdict<Report>, name: &str) {
        match v {
            Verdict::Refused(r) => {
                assert!(r.reason.contains(name), "expected {name} in {}", r.reason)
            }
            Verdict::Ok(_) => panic!("harness accepted a broken frame; wanted {name}"),
        }
    }

    #[test]
    fn harness_refuses_each_broken_obligation() {
        refused_named(check(&BrokenFrame::new(Obligation::Fo1), 1, 32), "FO1");
        refused_named(check(&BrokenFrame::new(Obligation::Fo2), 1, 32), "FO2");
        refused_named(check(&BrokenFrame::new(Obligation::Fo3), 1, 64), "FO3");
        refused_named(check(&BrokenFrame::new(Obligation::Fo5), 1, 16), "FO5");
        // FO7: claim to extend ℤ without a working embed.
        struct ExtBroken(BrokenFrame);
        impl Frame for ExtBroken {
            fn reference(&self) -> crate::FrameRef {
                crate::FrameRef::rat()
            }
            fn signature(&self) -> &crate::Signature {
                self.0.signature()
            }
            fn apply_op(&self, op: &OpName, args: &[crate::Value]) -> Verdict<crate::Value> {
                self.0.apply_op(op, args)
            }
            fn contains(&self, t: &Term) -> bool {
                self.0.contains(t)
            }
            fn canonicalize(&self, t: Term) -> Verdict<crate::Value> {
                self.0.canonicalize(t)
            }
            fn eq(&self, a: &crate::Value, b: &crate::Value) -> bool {
                self.0.eq(a, b)
            }
            fn print(&self, v: &crate::Value) -> String {
                self.0.print(v)
            }
            fn parse(&self, s: &str) -> Verdict<crate::Value> {
                self.0.parse(s)
            }
            fn generate(&self, seed: u64, size: u8) -> crate::Value {
                self.0.generate(seed, size)
            }
            fn shrink(&self, v: &crate::Value) -> Vec<crate::Value> {
                self.0.shrink(v)
            }
            fn extensions(&self) -> &[crate::FrameRef] {
                static EXT: [crate::FrameRef; 1] = [crate::FrameRef {
                    id: crate::FrameId::Int,
                    version: 1,
                }];
                &EXT
            }
            fn embed(&self, from: &crate::FrameRef, v: &crate::Value) -> Verdict<crate::Value> {
                self.0.embed(from, v)
            }
            fn restrict(&self, to: &crate::FrameRef, v: &crate::Value) -> Verdict<crate::Value> {
                self.0.restrict(to, v)
            }
            fn case(&self, v: &crate::Value) -> crate::Case {
                self.0.case(v)
            }
            fn constructors(&self) -> &[OpName] {
                self.0.constructors()
            }
        }
        refused_named(
            check(&ExtBroken(BrokenFrame::new(Obligation::Fo7)), 1, 8),
            "FO7",
        );
        refused_named(check(&BrokenFrame::new(Obligation::Fo8), 1, 32), "FO8");
        refused_named(check(&BrokenFrame::new(Obligation::Fo9), 1, 16), "FO9");
        refused_named(check(&BrokenFrame::new(Obligation::Fo10), 1, 32), "FO10");
    }

    #[test]
    fn text_passes_at_10000() {
        let r = check(&TextFrame::new(), 7, 10_000);
        assert!(r.is_ok(), "{r:?}");
    }

    #[test]
    fn int_passes() {
        let r = check(&IntFrame::new(), 11, 10_000);
        assert!(r.is_ok(), "{r:?}");
    }

    #[test]
    fn rat_passes_including_fo7() {
        let r = check(&RatFrame::new(), 13, 10_000);
        assert!(r.is_ok(), "{r:?}");
    }
}
