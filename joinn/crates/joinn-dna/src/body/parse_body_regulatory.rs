//! `parse_body_regulatory`.

#![allow(clippy::result_large_err)]

use crate::body::{BodyParser, BodyRegulatory};

pub(in crate::body) fn parse_body_regulatory(src: &str) -> BodyRegulatory {
    let mut p = BodyParser::new(src);
    p.skip();
    let mut reg = BodyRegulatory::default();
    if p.peek_ident() == Some("regulatory") {
        p.ident();
        let _ = p.expect('{');
        loop {
            p.skip();
            match p.peek_ident() {
                Some("prompts") => {
                    p.ident();
                    let _ = p.expect('{');
                    loop {
                        p.skip();
                        if p.peek() == Some('}') || p.eof() {
                            break;
                        }
                        let Some(name) = p.peek_ident().map(str::to_owned) else {
                            p.advance();
                            continue;
                        };
                        p.ident();
                        p.skip();
                        if let Ok(s) = p.quoted() {
                            reg.prompts.insert(name, s);
                        }
                    }
                    let _ = p.expect('}');
                }
                Some("present") => {
                    p.ident();
                    let _ = p.expect('{');
                    loop {
                        p.skip();
                        if p.peek() == Some('}') || p.eof() {
                            break;
                        }
                        let Some(name) = p.peek_ident().map(str::to_owned) else {
                            p.advance();
                            continue;
                        };
                        p.ident();
                        p.skip();
                        if let Ok(s) = p.quoted() {
                            reg.present.insert(name, s);
                        }
                    }
                    let _ = p.expect('}');
                }
                Some("names") => {
                    p.ident();
                    let _ = p.expect('{');
                    loop {
                        p.skip();
                        if p.peek() == Some('}') || p.eof() {
                            break;
                        }
                        let Some(name) = p.peek_ident().map(str::to_owned) else {
                            p.advance();
                            continue;
                        };
                        p.ident();
                        p.skip();
                        if let Ok(s) = p.quoted() {
                            reg.names.insert(name, s);
                        } else if let Some(disp) = p.peek_ident().map(str::to_owned) {
                            p.ident();
                            reg.names.insert(name, disp);
                        }
                    }
                    let _ = p.expect('}');
                }
                Some("labels") => {
                    p.ident();
                    let _ = p.expect('{');
                    loop {
                        p.skip();
                        if p.peek() == Some('}') || p.eof() {
                            break;
                        }
                        let Some(name) = p.peek_ident().map(str::to_owned) else {
                            p.advance();
                            continue;
                        };
                        p.ident();
                        p.skip();
                        if let Ok(s) = p.quoted() {
                            reg.labels.insert(name, s);
                        } else if let Some(disp) = p.peek_ident().map(str::to_owned) {
                            p.ident();
                            reg.labels.insert(name, disp);
                        }
                    }
                    let _ = p.expect('}');
                }
                _ => break,
            }
        }
        let _ = p.expect('}');
    }
    reg
}
