//! `Parser::coding_body`.

#![allow(clippy::result_large_err)]

use crate::model::{CodingRegion, Contract, JoinPolicy};
use joinn_frame::{FrameRef, Verdict};
use std::collections::BTreeMap;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn coding_body(&mut self) -> Verdict<CodingRegion> {
        let mut codex = 1u16;
        let mut frame = FrameRef::int();
        let mut contract = Contract {
            ports: Vec::new(),
            retired: Vec::new(),
            join_policy: JoinPolicy::Refuse,
            require: BTreeMap::new(),
            ensure: BTreeMap::new(),
        };
        let mut laws = BTreeMap::new();
        let mut founding = Vec::new();
        let mut declarations = Vec::new();
        let mut lineage = None;
        let mut turns = Vec::new();
        self.skip();
        while !self.eof() {
            if self.peek() == Some('}') {
                break;
            }
            let key = self.ident();
            if key.is_empty() {
                break;
            }
            match key.as_str() {
                "codex" => match self.number() {
                    Ok(n) => codex = n as u16,
                    Err(r) => return Verdict::Refused(r),
                },
                "frame" => match self.frame_ref() {
                    Ok(f) => frame = f,
                    Err(r) => return Verdict::Refused(r),
                },
                "contract" => match self.contract_body() {
                    Verdict::Ok(c) => contract = c,
                    Verdict::Refused(r) => return Verdict::Refused(r),
                },
                "laws" => match self.laws_body() {
                    Verdict::Ok(l) => laws = l,
                    Verdict::Refused(r) => return Verdict::Refused(r),
                },
                "founding" => match self.founding_body() {
                    Verdict::Ok(w) => founding = w,
                    Verdict::Refused(r) => return Verdict::Refused(r),
                },
                "declarations" => match self.declarations_body() {
                    Verdict::Ok(d) => declarations = d,
                    Verdict::Refused(r) => return Verdict::Refused(r),
                },
                "lineage" => {
                    self.skip();
                    if self.peek_ident() == Some("none") {
                        self.ident();
                        lineage = None;
                    } else {
                        match self.hash_hex() {
                            Ok(h) => lineage = Some(h),
                            Err(r) => return Verdict::Refused(r),
                        }
                    }
                }
                "turn" => match self.turn_block() {
                    Verdict::Ok(ts) => turns.extend(ts),
                    Verdict::Refused(r) => return Verdict::Refused(r),
                },
                // Flat canonical form: contract fields may appear at top level.
                "ensure" | "join" | "port" | "require" | "retired" => {
                    // Handle contract fields that appear at the coding-region top level.
                    match key.as_str() {
                        "join" => match self.join_policy() {
                            Ok(j) => contract.join_policy = j,
                            Err(r) => return Verdict::Refused(r),
                        },
                        "retired" => match self.retired_list() {
                            Ok(r) => contract.retired = r,
                            Err(e) => return Verdict::Refused(e),
                        },
                        "port" => match self.port_decl() {
                            Ok(p) => contract.ports.push(p),
                            Err(r) => return Verdict::Refused(r),
                        },
                        "require" => match self.formula_map() {
                            Verdict::Ok(m) => contract.require = m,
                            Verdict::Refused(r) => return Verdict::Refused(r),
                        },
                        "ensure" => match self.formula_map() {
                            Verdict::Ok(m) => contract.ensure = m,
                            Verdict::Refused(r) => return Verdict::Refused(r),
                        },
                        _ => {}
                    }
                }
                other => {
                    return Verdict::Refused(self.refuse(format!("unknown coding field {other}")));
                }
            }
            self.skip();
        }
        contract.ports.sort_by_key(|p| p.position);
        Verdict::Ok(CodingRegion {
            codex,
            frame,
            contract,
            laws,
            founding,
            declarations,
            lineage,
            turns,
        })
    }
}
