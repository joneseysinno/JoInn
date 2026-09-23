//! `Parser::regulatory_body`.

#![allow(clippy::result_large_err)]

use crate::model::RegulatoryRegion;
use joinn_frame::Verdict;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn regulatory_body(&mut self) -> Verdict<RegulatoryRegion> {
        let mut r = RegulatoryRegion::default();
        loop {
            self.skip();
            if self.peek() == Some('}') {
                break;
            }
            let key = match self.peek_ident() {
                Some(k) => k.to_owned(),
                None => break,
            };
            self.ident();
            match key.as_str() {
                "names" => match self.name_map() {
                    Verdict::Ok(m) => r.names = m,
                    Verdict::Refused(e) => return Verdict::Refused(e),
                },
                "literals" => match self.string_map() {
                    Verdict::Ok(m) => r.literals = m,
                    Verdict::Refused(e) => return Verdict::Refused(e),
                },
                "styles" => match self.string_map() {
                    Verdict::Ok(m) => r.styles = m,
                    Verdict::Refused(e) => return Verdict::Refused(e),
                },
                "labels" => match self.name_map() {
                    Verdict::Ok(m) => r.labels = m,
                    Verdict::Refused(e) => return Verdict::Refused(e),
                },
                _ => {}
            }
        }
        Verdict::Ok(r)
    }
}
