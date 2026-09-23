//! The `ℤ` frame. Arbitrary precision. Signature `{zero, succ, pred, eq}`.

mod print_term;

pub use print_term::print_term;

use crate::frame::{Frame, FrameRef, OpName, Signature};
use crate::frames::DEFAULT_SIZE;
use crate::rng::SeedRng;
use crate::term::Term;
use crate::value::Value;
use crate::verdict::{CheckId, Refusal, Subject, Verdict};
use num_bigint::BigInt;
use num_traits::{Signed, Zero};

/// Arbitrary-precision integers.
pub struct IntFrame {
    signature: Signature,
    constructors: Vec<OpName>,
    extensions: Vec<FrameRef>,
}

impl IntFrame {
    /// Construct the Phase 1 `ℤ` frame.
    pub fn new() -> Self {
        let mut signature = Signature::new();
        signature.insert("zero", 0);
        signature.insert("succ", 1);
        signature.insert("pred", 1);
        signature.insert("eq", 2);
        Self {
            signature,
            constructors: vec![
                OpName("zero".into()),
                OpName("succ".into()),
                OpName("pred".into()),
            ],
            extensions: Vec::new(),
        }
    }
}

impl Default for IntFrame {
    fn default() -> Self {
        Self::new()
    }
}

fn refuse(reason: impl Into<String>) -> Refusal {
    Refusal {
        check: CheckId::Canonicalize,
        subject: Subject::Frame("ℤ 1".into()),
        reason: reason.into(),
        counterexample: None,
        seed: 0,
    }
}

fn as_int<'a>(v: &'a Value, frame: &IntFrame) -> Option<&'a BigInt> {
    if v.frame() != &frame.reference() {
        return None;
    }
    match v.term() {
        Term::Int(n) => Some(n),
        _ => None,
    }
}

impl Frame for IntFrame {
    fn reference(&self) -> FrameRef {
        FrameRef::int()
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn apply_op(&self, op: &OpName, args: &[Value]) -> Verdict<Value> {
        match op.as_str() {
            "zero" if args.is_empty() => self.canonicalize(Term::int(0)),
            "succ" if args.len() == 1 => {
                let Some(n) = as_int(&args[0], self) else {
                    return Verdict::Refused(refuse("succ expects a ℤ value"));
                };
                self.canonicalize(Term::Int(n + 1))
            }
            "pred" if args.len() == 1 => {
                let Some(n) = as_int(&args[0], self) else {
                    return Verdict::Refused(refuse("pred expects a ℤ value"));
                };
                self.canonicalize(Term::Int(n - 1))
            }
            "eq" if args.len() == 2 => {
                if as_int(&args[0], self).is_none() || as_int(&args[1], self).is_none() {
                    return Verdict::Refused(refuse("eq expects ℤ arguments"));
                }
                let yes = self.eq(&args[0], &args[1]);
                self.canonicalize(Term::int(if yes { 1 } else { 0 }))
            }
            _ => Verdict::Refused(refuse(format!(
                "ℤ has no op {} of arity {}",
                op.as_str(),
                args.len()
            ))),
        }
    }

    fn contains(&self, t: &Term) -> bool {
        matches!(t, Term::Int(_))
    }

    fn canonicalize(&self, t: Term) -> Verdict<Value> {
        match t {
            Term::Int(n) => Verdict::Ok(Value::from_canonical(self.reference(), Term::Int(n))),
            other => Verdict::Refused(refuse(format!(
                "ℤ cannot canonicalize a non-int term: {other:?}"
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
        let body = s.strip_prefix('+').unwrap_or(s);
        if body.is_empty() {
            return Verdict::Refused(refuse("empty integer literal"));
        }
        let digits = body.strip_prefix('-').unwrap_or(body);
        if digits.len() > 1 && digits.starts_with('0') {
            return Verdict::Refused(refuse(format!(
                "leading zeros are not an integer literal: {s}"
            )));
        }
        if !digits.bytes().all(|b| b.is_ascii_digit()) {
            return Verdict::Refused(refuse(format!("not an integer literal: {s}")));
        }
        match body.parse::<BigInt>() {
            Ok(n) => self.canonicalize(Term::Int(n)),
            Err(_) => Verdict::Refused(refuse(format!("not an integer literal: {s}"))),
        }
    }

    fn generate(&self, seed: u64, size: u8) -> Value {
        let mut rng = SeedRng::new(seed);
        let size = if size == 0 { DEFAULT_SIZE } else { size };
        let case = if seed < 16 {
            seed
        } else {
            rng.next_bounded(16)
        };
        let n: BigInt = match case {
            0 => BigInt::from(0),
            1 => BigInt::from(1),
            2 => BigInt::from(-1),
            3 => BigInt::from(i32::MAX),
            4 => BigInt::from(i32::MIN),
            5 => BigInt::from(i64::MAX),
            6 => BigInt::from(i64::MIN),
            7 => BigInt::from(i64::MAX) + BigInt::from(1) + BigInt::from(rng.next_bounded(64)),
            8 => {
                let extra =
                    BigInt::from(i64::MAX) + BigInt::from(1) + BigInt::from(rng.next_bounded(64));
                -extra
            }
            _ => {
                let bits = u64::from(size).saturating_mul(4).max(8);
                let mag =
                    BigInt::from(rng.next_u64()) % (BigInt::from(2).pow(bits.min(128) as u32));
                if rng.next_bool() { -mag } else { mag }
            }
        };
        match self.canonicalize(Term::Int(n)) {
            Verdict::Ok(v) => v,
            Verdict::Refused(_) => Value::from_canonical(self.reference(), Term::int(0)),
        }
    }

    fn shrink(&self, v: &Value) -> Vec<Value> {
        let Term::Int(n) = v.term() else {
            return Vec::new();
        };
        let mut out = Vec::new();
        if !n.is_zero()
            && let Verdict::Ok(z) = self.canonicalize(Term::int(0))
        {
            out.push(z);
        }
        if n.is_negative()
            && let Verdict::Ok(p) = self.canonicalize(Term::Int(-n))
        {
            out.push(p);
        }
        if !n.is_zero() {
            let half = n / 2;
            if &half != n
                && let Verdict::Ok(h) = self.canonicalize(Term::Int(half))
            {
                out.push(h);
            }
            let step = if n.is_negative() { n + 1 } else { n - 1 };
            if &step != n
                && let Verdict::Ok(s) = self.canonicalize(Term::Int(step))
            {
                out.push(s);
            }
        }
        out
    }

    fn extensions(&self) -> &[FrameRef] {
        &self.extensions
    }

    fn embed(&self, from: &FrameRef, _v: &Value) -> Verdict<Value> {
        Verdict::Refused(refuse(format!("ℤ does not embed {from}")))
    }

    fn restrict(&self, to: &FrameRef, _v: &Value) -> Verdict<Value> {
        Verdict::Refused(refuse(format!("ℤ does not restrict to {to}")))
    }

    fn constructors(&self) -> &[OpName] {
        &self.constructors
    }

    fn case(&self, v: &Value) -> crate::frame::Case {
        use crate::frame::Case;
        if v.frame() != &self.reference() {
            return Case::Generator;
        }
        let Term::Int(n) = v.term() else {
            return Case::Generator;
        };
        if n.is_zero() {
            return Case::Generator;
        }
        if n.is_positive() {
            let inner = match self.canonicalize(Term::Int(n - 1)) {
                Verdict::Ok(p) => p,
                Verdict::Refused(_) => return Case::Generator,
            };
            Case::Built {
                op: OpName("succ".into()),
                parts: vec![inner],
            }
        } else {
            let inner = match self.canonicalize(Term::Int(n + 1)) {
                Verdict::Ok(p) => p,
                Verdict::Refused(_) => return Case::Generator,
            };
            Case::Built {
                op: OpName("pred".into()),
                parts: vec![inner],
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Frame;

    #[test]
    fn generator_exceeds_i64_at_default_size() {
        let frame = IntFrame::new();
        let mut any = false;
        for seed in 0u64..64 {
            let v = frame.generate(seed, DEFAULT_SIZE);
            if let Term::Int(n) = v.term()
                && n > &BigInt::from(i64::MAX)
            {
                any = true;
                break;
            }
        }
        assert!(any, "ℤ generator at default size must exceed i64::MAX");
    }

    #[test]
    fn parse_refuses_leading_zeros() {
        let frame = IntFrame::new();
        assert!(frame.parse("007").is_refused());
        assert!(frame.parse("0").is_ok());
        assert!(frame.parse("-1").is_ok());
    }
}
