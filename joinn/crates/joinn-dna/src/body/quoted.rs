//! `BodyParser::quoted`.

#![allow(clippy::result_large_err)]

use joinn_frame::{Refusal, nfc};

use crate::body::BodyParser;
impl<'a> BodyParser<'a> {
    pub(in crate::body) fn quoted(&mut self) -> Result<String, Refusal> {
        self.skip();
        if self.peek() != Some('"') {
            return Err(self.refuse("expected string"));
        }
        self.advance();
        let mut out = String::new();
        while let Some(c) = self.peek() {
            self.advance();
            match c {
                '"' => return Ok(nfc(&out)),
                '\\' => match self.peek() {
                    Some(n) => {
                        self.advance();
                        match n {
                            'n' => out.push('\n'),
                            't' => out.push('\t'),
                            other => out.push(other),
                        }
                    }
                    None => return Err(self.refuse("unterminated escape")),
                },
                other => out.push(other),
            }
        }
        Err(self.refuse("unterminated string"))
    }
}
