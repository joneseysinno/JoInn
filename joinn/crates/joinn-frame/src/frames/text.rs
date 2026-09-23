//! The `Text` frame.

mod print_term;

pub use print_term::print_term;

use crate::canon::nfc;
use crate::frame::{Frame, FrameRef, OpName, Signature};
use crate::frames::DEFAULT_SIZE;
use crate::rng::SeedRng;
use crate::term::Term;
use crate::value::Value;
use crate::verdict::{CheckId, Refusal, Subject, Verdict};
use num_traits::ToPrimitive;

/// Unicode text, NFC, no floating point, no ambient locale.
pub struct TextFrame {
    signature: Signature,
    constructors: Vec<OpName>,
    extensions: Vec<FrameRef>,
}

impl TextFrame {
    /// Construct the Phase 1 `Text` frame.
    pub fn new() -> Self {
        let mut signature = Signature::new();
        signature.insert("empty", 0);
        signature.insert("cons", 2);
        signature.insert("chr", 1);
        signature.insert("eq", 2);
        Self {
            signature,
            constructors: vec![
                OpName("empty".into()),
                OpName("cons".into()),
                OpName("chr".into()),
            ],
            extensions: Vec::new(),
        }
    }
}

impl Default for TextFrame {
    fn default() -> Self {
        Self::new()
    }
}

pub(in crate::frames::text) fn quote(s: &str) -> String {
    let mut out = String::from("\"");
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn refuse(reason: impl Into<String>) -> Refusal {
    Refusal {
        check: CheckId::Canonicalize,
        subject: Subject::Frame("Text 1".into()),
        reason: reason.into(),
        counterexample: None,
        seed: 0,
    }
}

impl Frame for TextFrame {
    fn reference(&self) -> FrameRef {
        FrameRef::text()
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn apply_op(&self, op: &OpName, args: &[Value]) -> Verdict<Value> {
        match op.as_str() {
            "empty" if args.is_empty() => self.canonicalize(Term::text("")),
            "cons" if args.len() == 2 => {
                if args.iter().any(|a| a.frame() != &self.reference()) {
                    return Verdict::Refused(refuse("cons expects Text arguments"));
                }
                let (Term::Text(h), Term::Text(t)) = (args[0].term(), args[1].term()) else {
                    return Verdict::Refused(refuse("cons expects Text arguments"));
                };
                if h.chars().count() != 1 {
                    return Verdict::Refused(refuse("cons head must be a single character"));
                }
                let mut s = String::new();
                s.push_str(h);
                s.push_str(t);
                self.canonicalize(Term::Text(s))
            }
            "chr" if args.len() == 1 => {
                let Term::Int(n) = args[0].term() else {
                    return Verdict::Refused(refuse("chr expects a ℤ codepoint"));
                };
                if args[0].frame().id != crate::frame::FrameId::Int {
                    return Verdict::Refused(refuse("chr expects a ℤ codepoint"));
                }
                let Some(u) = n.to_u32() else {
                    return Verdict::Refused(refuse("chr codepoint is negative or too large"));
                };
                let Some(ch) = char::from_u32(u) else {
                    return Verdict::Refused(refuse("chr codepoint is not a Unicode scalar"));
                };
                self.canonicalize(Term::Text(ch.to_string()))
            }
            "eq" if args.len() == 2 => {
                if args.iter().any(|a| a.frame() != &self.reference()) {
                    return Verdict::Refused(refuse("eq expects Text arguments"));
                }
                let yes = self.eq(&args[0], &args[1]);
                self.canonicalize(Term::text(if yes { "1" } else { "" }))
            }
            _ => Verdict::Refused(refuse(format!(
                "Text has no op {} of arity {}",
                op.as_str(),
                args.len()
            ))),
        }
    }

    fn contains(&self, t: &Term) -> bool {
        matches!(t, Term::Text(_))
    }

    fn canonicalize(&self, t: Term) -> Verdict<Value> {
        match t {
            Term::Text(s) => {
                let norm = nfc(&s);
                Verdict::Ok(Value::from_canonical(self.reference(), Term::Text(norm)))
            }
            other => Verdict::Refused(refuse(format!(
                "Text cannot canonicalize a non-text term: {other:?}"
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
        let body = if let Some(inner) = unquote(s) {
            inner
        } else {
            nfc(s)
        };
        self.canonicalize(Term::Text(body))
    }

    fn generate(&self, seed: u64, size: u8) -> Value {
        let mut rng = SeedRng::new(seed);
        let size = if size == 0 { DEFAULT_SIZE } else { size };
        let raw = match seed % 16 {
            0 => String::new(),
            1 => " ".into(),
            2 => "\t\n".into(),
            3 => "e\u{0301}".into(), // NFD é
            4 => "é".into(),         // NFC é
            5 => "\u{1F600}".into(), // astral grinning face
            6 => "\u{10000}".into(), // linear B syllable
            7 => "007".into(),
            _ => {
                let n = rng.next_bounded(u64::from(size) + 1) as usize;
                let mut s = String::new();
                for _ in 0..n {
                    let cp = match rng.next_bounded(4) {
                        0 => b'a' + (rng.next_bounded(26) as u8),
                        1 => b'0' + (rng.next_bounded(10) as u8),
                        2 => b' ',
                        _ => 0xE9, // é in latin-1, then we push as char
                    };
                    if cp == 0xE9 {
                        s.push('é');
                    } else {
                        s.push(cp as char);
                    }
                }
                s
            }
        };
        match self.canonicalize(Term::Text(raw)) {
            Verdict::Ok(v) => v,
            Verdict::Refused(_) => Value::from_canonical(self.reference(), Term::text("")),
        }
    }

    fn shrink(&self, v: &Value) -> Vec<Value> {
        let Term::Text(s) = v.term() else {
            return Vec::new();
        };
        if s.is_empty() {
            return Vec::new();
        }
        let mut out = Vec::new();
        if let Verdict::Ok(empty) = self.canonicalize(Term::text("")) {
            out.push(empty);
        }
        if s.chars().count() > 1 {
            let shorter: String = s.chars().take(s.chars().count() - 1).collect();
            if let Verdict::Ok(v) = self.canonicalize(Term::Text(shorter)) {
                out.push(v);
            }
        }
        out
    }

    fn extensions(&self) -> &[FrameRef] {
        &self.extensions
    }

    fn embed(&self, from: &FrameRef, _v: &Value) -> Verdict<Value> {
        Verdict::Refused(refuse(format!("Text does not embed {from}")))
    }

    fn restrict(&self, to: &FrameRef, _v: &Value) -> Verdict<Value> {
        Verdict::Refused(refuse(format!("Text does not restrict to {to}")))
    }

    fn constructors(&self) -> &[OpName] {
        &self.constructors
    }

    fn case(&self, v: &Value) -> crate::frame::Case {
        use crate::frame::Case;
        if v.frame() != &self.reference() {
            return Case::Generator;
        }
        let Term::Text(s) = v.term() else {
            return Case::Generator;
        };
        if s.is_empty() {
            return Case::Generator;
        }
        let mut chars = s.chars();
        let Some(first) = chars.next() else {
            return Case::Generator;
        };
        let rest: String = chars.collect();
        if rest.is_empty() {
            let int = crate::frames::IntFrame::new();
            let cp = match int.canonicalize(Term::int(u32::from(first) as i64)) {
                Verdict::Ok(v) => v,
                Verdict::Refused(_) => return Case::Generator,
            };
            return Case::Built {
                op: OpName("chr".into()),
                parts: vec![cp],
            };
        }
        let head = match self.canonicalize(Term::Text(first.to_string())) {
            Verdict::Ok(h) => h,
            Verdict::Refused(_) => return Case::Generator,
        };
        let tail = match self.canonicalize(Term::Text(rest)) {
            Verdict::Ok(t) => t,
            Verdict::Refused(_) => return Case::Generator,
        };
        Case::Built {
            op: OpName("cons".into()),
            parts: vec![head, tail],
        }
    }
}

fn unquote(s: &str) -> Option<String> {
    let s = s.strip_prefix('"')?.strip_suffix('"')?;
    let mut out = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next()? {
                '"' => out.push('"'),
                '\\' => out.push('\\'),
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                other => out.push(other),
            }
        } else {
            out.push(c);
        }
    }
    Some(nfc(&out))
}
