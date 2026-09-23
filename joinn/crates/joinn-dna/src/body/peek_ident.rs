//! `BodyParser::peek_ident`.

#![allow(clippy::result_large_err)]

use crate::body::BodyParser;
impl<'a> BodyParser<'a> {
    pub(in crate::body) fn peek_ident(&self) -> Option<&str> {
        let r = self.rest().trim_start();
        let mut end = 0;
        for (idx, c) in r.char_indices() {
            if idx == 0 {
                if c == '_' || c.is_alphabetic() {
                    end = c.len_utf8();
                } else {
                    return None;
                }
            } else if c == '_' || c.is_alphanumeric() {
                end = idx + c.len_utf8();
            } else {
                break;
            }
        }
        if end == 0 { None } else { Some(&r[..end]) }
    }
}
