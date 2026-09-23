//! The `ℚ` frame. Exact rationals, lowest terms, positive denominator. Extends ℤ.

mod print_term;

pub use print_term::print_term;

use crate::frame::{Frame, FrameRef, OpName, Signature};
use crate::frames::int::IntFrame;
use crate::rng::SeedRng;
use crate::term::Term;
use crate::value::Value;
use crate::verdict::{CheckId, Refusal, Subject, Verdict};
use num_bigint::BigInt;
use num_integer::Integer;
use num_rational::BigRational;
use num_traits::{Signed, Zero};

/// Exact rationals.
pub struct RatFrame {
    signature: Signature,
    constructors: Vec<OpName>,
    extensions: Vec<FrameRef>,
}

impl RatFrame {
    /// Construct the Phase 1 `ℚ` frame.
    pub fn new() -> Self {
        let mut signature = Signature::new();
        signature.insert("zero", 0);
        signature.insert("succ", 1);
        signature.insert("pred", 1);
        signature.insert("eq", 2);
        signature.insert("ratio", 2);
        Self {
            signature,
            constructors: vec![OpName("zero".into()), OpName("ratio".into())],
            extensions: vec![FrameRef::int()],
        }
    }
}

impl Default for RatFrame {
    fn default() -> Self {
        Self::new()
    }
}

/// `numer/denom` in lowest terms, positive denominator.
pub(in crate::frames::rat) fn as_ratio(term: &Term) -> Option<BigRational> {
    match term {
        Term::Seq(items) if items.len() == 2 => match (&items[0], &items[1]) {
            (Term::Int(n), Term::Int(d)) if !d.is_zero() => {
                Some(BigRational::new(n.clone(), d.clone()))
            }
            _ => None,
        },
        Term::Int(n) => Some(BigRational::from(n.clone())),
        _ => None,
    }
}

fn term_from_ratio(r: BigRational) -> Term {
    Term::Seq(vec![
        Term::Int(r.numer().clone()),
        Term::Int(r.denom().clone()),
    ])
}

fn refuse(reason: impl Into<String>) -> Refusal {
    Refusal {
        check: CheckId::Canonicalize,
        subject: Subject::Frame("ℚ 1".into()),
        reason: reason.into(),
        counterexample: None,
        seed: 0,
    }
}

fn value_ratio(v: &Value) -> Option<BigRational> {
    if v.frame().id != crate::frame::FrameId::Rat {
        return None;
    }
    as_ratio(v.term())
}

impl Frame for RatFrame {
    fn reference(&self) -> FrameRef {
        FrameRef::rat()
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn apply_op(&self, op: &OpName, args: &[Value]) -> Verdict<Value> {
        match op.as_str() {
            "zero" if args.is_empty() => {
                self.canonicalize(term_from_ratio(BigRational::from(BigInt::from(0))))
            }
            "succ" if args.len() == 1 => {
                let Some(r) = value_ratio(&args[0]) else {
                    return Verdict::Refused(refuse("succ expects a ℚ value"));
                };
                self.canonicalize(term_from_ratio(r + BigRational::from(BigInt::from(1))))
            }
            "pred" if args.len() == 1 => {
                let Some(r) = value_ratio(&args[0]) else {
                    return Verdict::Refused(refuse("pred expects a ℚ value"));
                };
                self.canonicalize(term_from_ratio(r - BigRational::from(BigInt::from(1))))
            }
            "eq" if args.len() == 2 => {
                if value_ratio(&args[0]).is_none() || value_ratio(&args[1]).is_none() {
                    return Verdict::Refused(refuse("eq expects ℚ arguments"));
                }
                let yes = self.eq(&args[0], &args[1]);
                self.canonicalize(term_from_ratio(BigRational::from(BigInt::from(if yes {
                    1
                } else {
                    0
                }))))
            }
            "ratio" if args.len() == 2 => {
                let int = IntFrame::new();
                let n = if args[0].frame() == &int.reference() {
                    match args[0].term() {
                        Term::Int(n) => n.clone(),
                        _ => return Verdict::Refused(refuse("ratio numer is not an integer")),
                    }
                } else {
                    return Verdict::Refused(refuse("ratio expects ℤ arguments"));
                };
                let d = if args[1].frame() == &int.reference() {
                    match args[1].term() {
                        Term::Int(n) => n.clone(),
                        _ => return Verdict::Refused(refuse("ratio denom is not an integer")),
                    }
                } else {
                    return Verdict::Refused(refuse("ratio expects ℤ arguments"));
                };
                if d.is_zero() {
                    return Verdict::Refused(refuse("ratio denominator must not be zero"));
                }
                self.canonicalize(Term::Seq(vec![Term::Int(n), Term::Int(d)]))
            }
            _ => Verdict::Refused(refuse(format!(
                "ℚ has no op {} of arity {}",
                op.as_str(),
                args.len()
            ))),
        }
    }

    fn contains(&self, t: &Term) -> bool {
        as_ratio(t).is_some()
    }

    fn canonicalize(&self, t: Term) -> Verdict<Value> {
        match as_ratio(&t) {
            Some(r) => {
                // BigRational already reduces; force positive denom.
                let n = r.numer().clone();
                let d = r.denom().clone();
                let (n, d) = if d.is_negative() { (-n, -d) } else { (n, d) };
                if d.is_zero() {
                    return Verdict::Refused(refuse("denominator must not be zero"));
                }
                let g = n.gcd(&d);
                let n = n / &g;
                let d = d / g;
                Verdict::Ok(Value::from_canonical(
                    self.reference(),
                    Term::Seq(vec![Term::Int(n), Term::Int(d)]),
                ))
            }
            None => Verdict::Refused(refuse(format!(
                "ℚ cannot canonicalize a non-ratio term: {t:?}"
            ))),
        }
    }

    fn eq(&self, a: &Value, b: &Value) -> bool {
        a.frame() == b.frame() && a.term() == b.term()
    }

    fn print(&self, v: &Value) -> String {
        print_term(v.term())
    }

    fn parse(&self, s: &str) -> Verdict<Value> {
        let s = s.trim();
        let (num, den) = if let Some((a, b)) = s.split_once('/') {
            (a, b)
        } else {
            (s, "1")
        };
        let int = IntFrame::new();
        let n = match int.parse(num) {
            Verdict::Ok(v) => match v.term() {
                Term::Int(n) => n.clone(),
                _ => return Verdict::Refused(refuse("numerator is not an integer")),
            },
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let d = match int.parse(den) {
            Verdict::Ok(v) => match v.term() {
                Term::Int(n) => n.clone(),
                _ => return Verdict::Refused(refuse("denominator is not an integer")),
            },
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        self.canonicalize(Term::Seq(vec![Term::Int(n), Term::Int(d)]))
    }

    fn generate(&self, seed: u64, size: u8) -> Value {
        let int = IntFrame::new();
        let mut rng = SeedRng::new(seed);
        let n = int.generate(seed, size);
        let d_seed = rng.next_u64();
        let d = int.generate(d_seed, size);
        let Term::Int(numer) = n.term() else {
            return Value::from_canonical(
                self.reference(),
                Term::Seq(vec![Term::int(0), Term::int(1)]),
            );
        };
        let Term::Int(denom) = d.term() else {
            return Value::from_canonical(
                self.reference(),
                Term::Seq(vec![Term::int(0), Term::int(1)]),
            );
        };
        let denom = if denom.is_zero() {
            BigInt::from(1)
        } else {
            denom.clone()
        };
        match self.canonicalize(Term::Seq(vec![Term::Int(numer.clone()), Term::Int(denom)])) {
            Verdict::Ok(v) => v,
            Verdict::Refused(_) => Value::from_canonical(
                self.reference(),
                Term::Seq(vec![Term::int(0), Term::int(1)]),
            ),
        }
    }

    fn shrink(&self, v: &Value) -> Vec<Value> {
        let Some(r) = value_ratio(v) else {
            return Vec::new();
        };
        let mut out = Vec::new();
        if let Verdict::Ok(z) =
            self.canonicalize(term_from_ratio(BigRational::from(BigInt::from(0))))
            && !self.eq(&z, v)
        {
            out.push(z);
        }
        let int = IntFrame::new();
        if let Verdict::Ok(n) = int.canonicalize(Term::Int(r.numer().clone())) {
            for s in int.shrink(&n) {
                if let Term::Int(sn) = s.term()
                    && let Verdict::Ok(rv) = self.canonicalize(Term::Seq(vec![
                        Term::Int(sn.clone()),
                        Term::Int(r.denom().clone()),
                    ]))
                    && !self.eq(&rv, v)
                {
                    out.push(rv);
                }
            }
        }
        out
    }

    fn extensions(&self) -> &[FrameRef] {
        &self.extensions
    }

    fn embed(&self, from: &FrameRef, v: &Value) -> Verdict<Value> {
        if *from != FrameRef::int() {
            return Verdict::Refused(refuse(format!("ℚ embeds only ℤ, not {from}")));
        }
        if v.frame() != from {
            return Verdict::Refused(refuse("embed: value is not in the source frame"));
        }
        let Term::Int(n) = v.term() else {
            return Verdict::Refused(refuse("embed: ℤ value is not an int term"));
        };
        self.canonicalize(Term::Seq(vec![Term::Int(n.clone()), Term::int(1)]))
    }

    fn restrict(&self, to: &FrameRef, v: &Value) -> Verdict<Value> {
        if *to != FrameRef::int() {
            return Verdict::Refused(refuse(format!("ℚ restricts only to ℤ, not {to}")));
        }
        let Some(r) = value_ratio(v) else {
            return Verdict::Refused(refuse("restrict: not a ℚ value"));
        };
        if r.denom() != &BigInt::from(1) {
            return Verdict::Refused(refuse("restrict: not an integer rational"));
        }
        let int = IntFrame::new();
        int.canonicalize(Term::Int(r.numer().clone()))
    }

    fn constructors(&self) -> &[OpName] {
        &self.constructors
    }

    fn case(&self, v: &Value) -> crate::frame::Case {
        use crate::frame::Case;
        if v.frame() != &self.reference() {
            return Case::Generator;
        }
        let Some(r) = value_ratio(v) else {
            return Case::Generator;
        };
        if r.numer().is_zero() && r.denom() == &BigInt::from(1) {
            return Case::Generator;
        }
        let int = IntFrame::new();
        let n = match int.canonicalize(Term::Int(r.numer().clone())) {
            Verdict::Ok(v) => v,
            Verdict::Refused(_) => return Case::Generator,
        };
        let d = match int.canonicalize(Term::Int(r.denom().clone())) {
            Verdict::Ok(v) => v,
            Verdict::Refused(_) => return Case::Generator,
        };
        Case::Built {
            op: OpName("ratio".into()),
            parts: vec![n, d],
        }
    }
}
