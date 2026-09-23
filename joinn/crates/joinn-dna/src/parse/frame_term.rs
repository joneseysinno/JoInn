//! `Parser::frame_term`.

#![allow(clippy::result_large_err)]

use crate::formula::Term_;
use joinn_frame::{OpName, Verdict};

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn frame_term(&mut self) -> Verdict<Term_> {
        let fr = match self.frame_ref() {
            Ok(f) => f,
            Err(r) => return Verdict::Refused(r),
        };
        self.skip();
        if self.peek() == Some('.') {
            self.advance();
            let op = match self.take_ident() {
                Ok(o) => o,
                Err(r) => return Verdict::Refused(r),
            };
            if matches!(op.as_str(), "add" | "mul" | "parse_int" | "format") {
                return Verdict::Refused(self.refuse(format!(
                    "primitive name {}.{} is not allowed in a coding region",
                    fr.id, op
                )));
            }
            let mut args = Vec::new();
            self.skip();
            if self.peek() == Some('(') {
                self.advance();
                loop {
                    self.skip();
                    if self.peek() == Some(')') {
                        self.advance();
                        break;
                    }
                    match self.term() {
                        Verdict::Ok(t) => args.push(t),
                        Verdict::Refused(r) => return Verdict::Refused(r),
                    }
                    self.skip();
                    if self.peek() == Some(',') {
                        self.advance();
                    }
                }
            }
            return Verdict::Ok(Term_::FrameOp {
                frame: fr,
                op: OpName(op),
                args,
            });
        }
        // Literal: frame already consumed; parse the rest as a value using a nested scan.
        // Rewind is hard; parse the literal tail.
        self.skip();
        let text = if self.peek() == Some('"') {
            match self.quoted_string() {
                Ok(s) => s,
                Err(r) => return Verdict::Refused(r),
            }
        } else {
            let start = self.i;
            if self.peek() == Some('+') || self.peek() == Some('-') {
                self.advance();
            }
            while matches!(self.peek(), Some(c) if c.is_ascii_digit() || c == '/') {
                self.advance();
            }
            self.src[start..self.i].to_owned()
        };
        let Some(frame) = self.frames.get(&fr) else {
            return Verdict::Refused(self.refuse(format!("unregistered frame {fr}")));
        };
        match frame.parse(&text) {
            Verdict::Ok(v) => Verdict::Ok(Term_::Lit(v)),
            Verdict::Refused(r) => Verdict::Refused(r),
        }
    }
}
