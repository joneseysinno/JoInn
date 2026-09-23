//! `Parser::literal_value`.

#![allow(clippy::result_large_err)]

use joinn_frame::Verdict;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn literal_value(&mut self) -> Verdict<joinn_frame::Value> {
        let fr = match self.frame_ref() {
            Ok(f) => f,
            Err(r) => return Verdict::Refused(r),
        };
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
        // Text parse accepts quoted or raw; we already have the quoted form for Text.
        frame.parse(&text)
    }
}
