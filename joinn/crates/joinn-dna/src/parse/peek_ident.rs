//! `Parser::peek_ident`.

#![allow(clippy::result_large_err)]

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn peek_ident(&self) -> Option<&str> {
        let r = self.rest().trim_start();
        let mut end = 0;
        for (idx, c) in r.char_indices() {
            if idx == 0 {
                if c == '_' || c.is_alphabetic() || c == 'ℤ' || c == 'ℚ' {
                    end = c.len_utf8();
                } else {
                    return None;
                }
            } else if c == '_' || c.is_alphanumeric() || c == 'ℤ' || c == 'ℚ' {
                end = idx + c.len_utf8();
            } else {
                break;
            }
        }
        if end == 0 { None } else { Some(&r[..end]) }
    }
}
