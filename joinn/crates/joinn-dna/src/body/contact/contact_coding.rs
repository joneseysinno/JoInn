//! `BodyParser::contact_coding`.

use joinn_frame::Verdict;
use std::collections::{BTreeMap, BTreeSet};

use crate::body::BodyParser;
use crate::body::contact::ContactCoding;

impl<'a> BodyParser<'a> {
    /// The sections of a contact's coding region, in any order, braced or flat.
    pub(in crate::body::contact) fn contact_coding(&mut self) -> Verdict<ContactCoding> {
        let mut coding = ContactCoding {
            codex: 1,
            genome: Vec::new(),
            grants: BTreeMap::new(),
            reads: BTreeSet::new(),
            forces: Vec::new(),
            grows: None,
            budget_steps: 100_000,
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
                    "contact: unexpected {got:?}; acceptance is a section: budget, codex, forces, genome, grants, lineage, read"
                )));
            };
            self.ident();
            let step = match word.as_str() {
                "codex" => self.number_u32().map(|n| coding.codex = n as u16),
                "budget" => {
                    let braced = self.take_brace();
                    self.skip();
                    let got = if self.peek_ident() == Some("steps") {
                        self.ident();
                        self.number_u64().map(|n| coding.budget_steps = n)
                    } else {
                        Ok(())
                    };
                    self.close_section(braced);
                    got
                }
                "steps" => self.number_u64().map(|n| coding.budget_steps = n),
                "genome" => self.contact_genome().map(|g| coding.genome.extend(g)),
                "grants" => {
                    coding.grants.extend(self.contact_grants());
                    Ok(())
                }
                "read" => {
                    let braced = self.take_brace();
                    coding.reads.extend(self.contact_names());
                    self.close_section(braced);
                    Ok(())
                }
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
                "forces" => self.contact_forces().map(|f| coding.forces.extend(f)),
                "wires" => Err(self.refuse(
                    "contact: wires belong to systems; a body's cells touch. acceptance is a forces section",
                )),
                "declarations" => Err(self.refuse(
                    "contact codex 1 has no declarations; acceptance is a .body or .universe for an assert",
                )),
                other => Err(self.refuse(format!(
                    "contact: {other} is not a section; acceptance is budget, codex, forces, genome, grants, lineage or read"
                ))),
            };
            if let Err(r) = step {
                return Verdict::Refused(r);
            }
        }
        let instances: BTreeSet<&str> = coding
            .genome
            .iter()
            .flat_map(|g| g.instances.iter().map(String::as_str))
            .collect();
        let mut responses = BTreeSet::new();
        for force in &coding.forces {
            if instances.contains(force.name.as_str()) {
                return Verdict::Refused(self.refuse(format!(
                    "contact: {} is both a cell and a force's response; acceptance is a new name",
                    force.name
                )));
            }
            if !responses.insert(force.name.as_str()) {
                return Verdict::Refused(self.refuse(format!(
                    "contact: {} is the response of two forces; acceptance is a new name",
                    force.name
                )));
            }
        }
        Verdict::Ok(coding)
    }
}
