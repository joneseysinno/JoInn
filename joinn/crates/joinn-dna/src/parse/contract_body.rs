//! `Parser::contract_body`.

#![allow(clippy::result_large_err)]

use crate::model::{Contract, JoinPolicy};
use joinn_frame::Verdict;
use std::collections::BTreeMap;

use crate::parse::Parser;
impl<'a> Parser<'a> {
    pub(in crate::parse) fn contract_body(&mut self) -> Verdict<Contract> {
        self.skip();
        let braced = self.peek() == Some('{');
        if braced {
            self.advance();
        }
        let mut c = Contract {
            ports: Vec::new(),
            retired: Vec::new(),
            join_policy: JoinPolicy::Refuse,
            require: BTreeMap::new(),
            ensure: BTreeMap::new(),
        };
        loop {
            self.skip();
            if braced && self.peek() == Some('}') {
                self.advance();
                break;
            }
            let key = match self.peek_ident() {
                Some(k) => k.to_owned(),
                None => break,
            };
            if !braced
                && matches!(
                    key.as_str(),
                    "declarations" | "founding" | "frame" | "laws" | "lineage" | "codex"
                )
            {
                break;
            }
            self.ident();
            match key.as_str() {
                "join" => match self.join_policy() {
                    Ok(j) => c.join_policy = j,
                    Err(r) => return Verdict::Refused(r),
                },
                "retired" => match self.retired_list() {
                    Ok(r) => c.retired = r,
                    Err(e) => return Verdict::Refused(e),
                },
                "port" => match self.port_decl() {
                    Ok(p) => c.ports.push(p),
                    Err(r) => return Verdict::Refused(r),
                },
                "require" => match self.formula_map() {
                    Verdict::Ok(m) => c.require = m,
                    Verdict::Refused(r) => return Verdict::Refused(r),
                },
                "ensure" => match self.formula_map() {
                    Verdict::Ok(m) => c.ensure = m,
                    Verdict::Refused(r) => return Verdict::Refused(r),
                },
                _ => {
                    return Verdict::Refused(self.refuse(format!("unknown contract field {key}")));
                }
            }
        }
        Verdict::Ok(c)
    }
}
