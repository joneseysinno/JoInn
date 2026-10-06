//! `parse_system_regulatory`.

use crate::body::BodyParser;
use crate::body::system::SystemRegulatory;

/// `names`, `present` and `waiting`, each `{ <name> <value> … }`. Never hashed;
/// an entry it can't read is skipped.
pub(in crate::body::system) fn parse_system_regulatory(src: &str) -> SystemRegulatory {
    let mut p = BodyParser::new(src);
    let mut reg = SystemRegulatory::default();
    p.skip();
    if p.peek_ident() != Some("regulatory") {
        return reg;
    }
    p.ident();
    let _ = p.expect('{');
    loop {
        p.skip();
        let section = match p.peek_ident() {
            Some(s @ ("names" | "present" | "waiting")) => s.to_owned(),
            _ => break,
        };
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
            if section == "waiting" {
                if let Ok(n) = p.number_u32() {
                    reg.waiting.insert(name, n);
                }
            } else if let Ok(s) = p.quoted() {
                if section == "names" {
                    reg.names.insert(name, s);
                } else {
                    reg.present.insert(name, s);
                }
            }
        }
        let _ = p.expect('}');
    }
    let _ = p.expect('}');
    reg
}
