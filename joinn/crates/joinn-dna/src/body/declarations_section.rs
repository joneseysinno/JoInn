//! `BodyParser::declarations_section`.

#![allow(clippy::result_large_err)]

use joinn_frame::Verdict;

use crate::assertion::{Assertion, parse_assertions};
use crate::body::BodyParser;
impl<'a> BodyParser<'a> {
    /// Braced: the text up to `}`. Flat: each line until the next section keyword.
    pub(in crate::body) fn declarations_section(&mut self) -> Verdict<Vec<Assertion>> {
        let braced = self.take_brace();
        let mut text = String::new();
        if braced {
            let Some(end) = self.rest().find('}') else {
                return Verdict::Refused(self.refuse("unclosed declarations section"));
            };
            text.push_str(&self.rest()[..end]);
            self.i += end + 1;
        } else {
            loop {
                self.skip();
                if self.eof() || self.peek() == Some('}') || self.section_end(false) {
                    break;
                }
                let line = match self.rest().find('\n') {
                    Some(end) => &self.rest()[..end],
                    None => self.rest(),
                };
                text.push_str(line);
                text.push('\n');
                self.i += line.len();
            }
        }
        parse_assertions(&text)
    }
}
