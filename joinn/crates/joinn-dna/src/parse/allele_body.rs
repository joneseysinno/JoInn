//! `Parser::allele_body`.

#![allow(clippy::result_large_err)]

use crate::model::{Allele, AlleleBody, NativeId};
use joinn_frame::{FrameRef, Verdict};

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn allele_body(&mut self) -> Verdict<Allele> {
        self.skip();
        if self.peek() == Some('{') {
            self.advance();
        }
        let mut frame = FrameRef::int();
        let mut body = AlleleBody::Native(NativeId(String::new()));
        let mut witnesses = Vec::new();
        loop {
            self.skip();
            if self.peek() == Some('}') {
                self.advance();
                break;
            }
            let key = match self.peek_ident() {
                Some(k) => k.to_owned(),
                None => break,
            };
            self.ident();
            match key.as_str() {
                "frame" => match self.frame_ref() {
                    Ok(f) => frame = f,
                    Err(r) => return Verdict::Refused(r),
                },
                "native" => match self.native_id() {
                    Ok(n) => body = AlleleBody::Native(NativeId(n)),
                    Err(r) => return Verdict::Refused(r),
                },
                "dna" => match self.hash_hex() {
                    Ok(h) => body = AlleleBody::Dna(h),
                    Err(r) => return Verdict::Refused(r),
                },
                "witnesses" => match self.founding_body() {
                    Verdict::Ok(w) => witnesses = w,
                    Verdict::Refused(r) => return Verdict::Refused(r),
                },
                _ => {}
            }
        }
        Verdict::Ok(Allele {
            frame,
            body,
            witnesses,
        })
    }
}
