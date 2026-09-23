//! `BodyParser::coding_body`.

#![allow(clippy::result_large_err)]

use joinn_frame::Verdict;
use std::collections::{BTreeMap, BTreeSet};

use crate::body::{BodyCoding, BodyParser, GenomeEntry, GenomeTarget, Wire};
impl<'a> BodyParser<'a> {
    pub(in crate::body) fn coding_body(&mut self) -> Verdict<BodyCoding> {
        let mut codex = 1u16;
        let mut genome: Vec<GenomeEntry> = Vec::new();
        let mut grants = BTreeMap::new();
        let mut reads = BTreeSet::new();
        let mut wires = Vec::new();
        let mut budget_steps = 100_000u64;
        let mut lineage = None;
        while !self.eof() {
            self.skip();
            if self.peek() == Some('}') {
                break;
            }
            match self.peek_ident() {
                Some("codex") => {
                    self.ident();
                    match self.number_u32() {
                        Ok(n) => codex = n as u16,
                        Err(r) => return Verdict::Refused(r),
                    }
                }
                Some("budget") => {
                    self.ident();
                    let braced = self.take_brace();
                    while !self.eof() && !self.section_end(braced) {
                        self.skip();
                        if self.peek_ident() == Some("steps") {
                            self.ident();
                            match self.number_u64() {
                                Ok(n) => budget_steps = n,
                                Err(r) => return Verdict::Refused(r),
                            }
                        } else {
                            break;
                        }
                    }
                    self.close_section(braced);
                }
                Some("steps") => {
                    self.ident();
                    match self.number_u64() {
                        Ok(n) => budget_steps = n,
                        Err(r) => return Verdict::Refused(r),
                    }
                }
                Some("genome") => {
                    self.ident();
                    let braced = self.take_brace();
                    while !self.eof() && !self.section_end(braced) {
                        self.skip();
                        if self.peek_ident() == Some("cell") {
                            self.ident();
                            if self.peek() == Some(':') {
                                self.advance();
                            }
                            let cell = match self.hex_hash() {
                                Ok(h) => h,
                                Err(r) => return Verdict::Refused(r),
                            };
                            self.skip();
                            if self.peek_ident() == Some("as") {
                                self.ident();
                            }
                            let mut instances = Vec::new();
                            loop {
                                self.skip();
                                if self.peek() == Some(',') {
                                    self.advance();
                                    self.skip();
                                }
                                match self.peek_ident() {
                                    Some("cell") | Some("prim") | Some("grants")
                                    | Some("wires") | Some("budget") | Some("lineage")
                                    | Some("read") | Some("codex") | None => {
                                        break;
                                    }
                                    Some(_) => instances.push(self.ident()),
                                }
                            }
                            genome.push(GenomeEntry {
                                target: GenomeTarget::Cell(cell),
                                instances,
                            });
                        } else if self.peek_ident() == Some("prim") {
                            self.ident();
                            if self.peek() == Some(':') {
                                self.advance();
                            }
                            self.skip();
                            let name = self.ident();
                            if name.is_empty() {
                                return Verdict::Refused(self.refuse("prim: missing name"));
                            }
                            self.skip();
                            if self.peek_ident() == Some("as") {
                                self.ident();
                            }
                            let mut instances = Vec::new();
                            loop {
                                self.skip();
                                if self.peek() == Some(',') {
                                    self.advance();
                                    self.skip();
                                }
                                match self.peek_ident() {
                                    Some("cell") | Some("prim") | Some("grants")
                                    | Some("wires") | Some("budget") | Some("lineage")
                                    | Some("read") | Some("codex") | None => {
                                        break;
                                    }
                                    Some(_) => instances.push(self.ident()),
                                }
                            }
                            genome.push(GenomeEntry {
                                target: GenomeTarget::Prim(name),
                                instances,
                            });
                        } else {
                            break;
                        }
                    }
                    self.close_section(braced);
                }
                Some("grants") => {
                    self.ident();
                    let braced = self.take_brace();
                    while !self.eof() && !self.section_end(braced) {
                        self.skip();
                        if self.section_end(braced) {
                            break;
                        }
                        if self.peek_ident().is_none() {
                            break;
                        }
                        if matches!(
                            self.peek_ident(),
                            Some("wires")
                                | Some("lineage")
                                | Some("budget")
                                | Some("codex")
                                | Some("genome")
                                | Some("read")
                        ) {
                            break;
                        }
                        let cap = self.ident();
                        if self.peek() == Some(':') {
                            self.advance();
                        }
                        let mut insts = Vec::new();
                        loop {
                            self.skip();
                            if self.peek() == Some(',') {
                                self.advance();
                                self.skip();
                            }
                            match self.peek_ident() {
                                Some(id)
                                    if id != "wires"
                                        && id != "lineage"
                                        && id != "genome"
                                        && id != "codex"
                                        && id != "budget"
                                        && id != "read" =>
                                {
                                    insts.push(self.ident());
                                }
                                _ => break,
                            }
                        }
                        grants.insert(cap, insts);
                    }
                    self.close_section(braced);
                }
                Some("read") => {
                    self.ident();
                    let braced = self.take_brace();
                    while !self.eof() && !self.section_end(braced) {
                        self.skip();
                        if self.section_end(braced) {
                            break;
                        }
                        match self.peek_ident() {
                            Some(name)
                                if name != "wires"
                                    && name != "lineage"
                                    && name != "genome"
                                    && name != "codex"
                                    && name != "budget"
                                    && name != "grants" =>
                            {
                                reads.insert(self.ident());
                            }
                            _ => break,
                        }
                    }
                    self.close_section(braced);
                }
                Some("wires") => {
                    self.ident();
                    let braced = self.take_brace();
                    while !self.eof() && !self.section_end(braced) {
                        self.skip();
                        if self.section_end(braced) {
                            break;
                        }
                        if self.peek_ident().is_none() {
                            break;
                        }
                        if matches!(
                            self.peek_ident(),
                            Some("lineage")
                                | Some("budget")
                                | Some("codex")
                                | Some("genome")
                                | Some("grants")
                        ) {
                            break;
                        }
                        let src = self.ident();
                        if let Err(r) = self.expect('@') {
                            return Verdict::Refused(r);
                        }
                        let sp = match self.number_u32() {
                            Ok(n) => n,
                            Err(r) => return Verdict::Refused(r),
                        };
                        self.skip();
                        if self.peek() == Some('-') {
                            self.advance();
                            if self.peek() == Some('>') {
                                self.advance();
                            }
                        }
                        self.skip();
                        let dst = self.ident();
                        if let Err(r) = self.expect('@') {
                            return Verdict::Refused(r);
                        }
                        let dp = match self.number_u32() {
                            Ok(n) => n,
                            Err(r) => return Verdict::Refused(r),
                        };
                        wires.push(Wire {
                            src_instance: src,
                            src_port: sp,
                            dst_instance: dst,
                            dst_port: dp,
                        });
                    }
                    self.close_section(braced);
                }
                Some("lineage") => {
                    self.ident();
                    self.skip();
                    if self.peek_ident() == Some("none") {
                        self.ident();
                        lineage = None;
                    } else {
                        match self.hex_hash() {
                            Ok(h) => lineage = Some(h),
                            Err(r) => return Verdict::Refused(r),
                        }
                    }
                }
                Some(_) => {
                    self.advance();
                }
                None => {
                    if self.eof() {
                        break;
                    }
                    self.advance();
                }
            }
        }
        Verdict::Ok(BodyCoding {
            codex,
            genome,
            grants,
            reads,
            wires,
            budget_steps,
            lineage,
        })
    }
}
