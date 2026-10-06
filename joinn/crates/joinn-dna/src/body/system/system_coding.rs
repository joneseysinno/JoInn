//! `BodyParser::system_coding`.

use joinn_frame::Verdict;
use std::collections::BTreeSet;

use crate::body::BodyParser;
use crate::body::system::SystemCoding;

impl<'a> BodyParser<'a> {
    /// The sections of a system's coding region, in any order, braced or flat.
    pub(in crate::body::system) fn system_coding(&mut self) -> Verdict<SystemCoding> {
        let mut coding = SystemCoding {
            codex: 1,
            bodies: Vec::new(),
            forces: Vec::new(),
            lineage: None,
        };
        loop {
            self.skip();
            if self.eof() || self.peek() == Some('}') {
                break;
            }
            let Some(word) = self.peek_ident().map(str::to_owned) else {
                let got = self.rest().chars().next().unwrap_or(' ');
                return Verdict::Refused(self.refuse(format!(
                    "system: unexpected {got:?}; acceptance is a section: bodies, codex, forces, lineage"
                )));
            };
            self.ident();
            let step = match word.as_str() {
                "codex" => self.number_u32().map(|n| coding.codex = n as u16),
                "bodies" => self.system_bodies().map(|b| coding.bodies.extend(b)),
                "forces" => self.system_forces().map(|f| coding.forces.extend(f)),
                "lineage" => {
                    self.skip();
                    if self.peek_ident() == Some("none") {
                        self.ident();
                        coding.lineage = None;
                        Ok(())
                    } else {
                        self.hex_hash().map(|h| coding.lineage = Some(h))
                    }
                }
                other => Err(self.refuse(format!(
                    "system: {other} is not a section; acceptance is bodies, codex, forces or lineage"
                ))),
            };
            if let Err(r) = step {
                return Verdict::Refused(r);
            }
        }
        if coding.bodies.len() != 1 {
            return Verdict::Refused(self.refuse(format!(
                "system: {} bodies; acceptance is one body in Phase 7.4",
                coding.bodies.len()
            )));
        }
        let aliases: BTreeSet<&str> = coding.bodies.iter().map(|b| b.alias.as_str()).collect();
        let mut responses = BTreeSet::new();
        for force in &coding.forces {
            if aliases.contains(force.name.as_str()) {
                return Verdict::Refused(self.refuse(format!(
                    "system: {} is both a body and a force's response; acceptance is a new name",
                    force.name
                )));
            }
            if !responses.insert(force.name.as_str()) {
                return Verdict::Refused(self.refuse(format!(
                    "system: {} is the response of two forces; acceptance is a new name",
                    force.name
                )));
            }
        }
        Verdict::Ok(coding)
    }
}
