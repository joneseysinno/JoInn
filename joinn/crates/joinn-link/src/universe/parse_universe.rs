//! Parse a `.universe` file.
//! allow(modules): recursive-descent walker over the universe grammar

#![allow(clippy::result_large_err)]

use joinn_frame::{CheckId, Hash, Refusal, Subject, Verdict, nfc};
use std::collections::{BTreeMap, BTreeSet};

use super::{
    BodyBinding, CrossWire, Galaxy, Lens, Link, Mark, Member, Order, System, Universe,
    UniverseCoding, UniverseRegulatory,
};

/// Parse a `.universe` file. An undeclared body alias is refused by name.
pub fn parse_universe(src: &str) -> Verdict<Universe> {
    let src = nfc(&src.replace("\r\n", "\n"));
    let stripped = strip_comments(&src);
    let (coding_src, rest) = split_delimiter(&stripped);
    let mut p = Cursor::new(coding_src);
    p.skip();
    if p.take_ident() != "universe" {
        return Verdict::Refused(p.refuse("expected universe"));
    }
    p.skip();
    if !p.take_char('{') {
        return Verdict::Refused(p.refuse("expected { after universe"));
    }
    let mut codex = 1u16;
    let mut bodies = Vec::new();
    let mut links = Vec::new();
    let mut cross_wires = Vec::new();
    let mut grants = BTreeMap::new();
    let mut lenses = Vec::new();
    loop {
        p.skip();
        if p.peek() == Some('}') {
            p.advance();
            break;
        }
        if p.eof() {
            return Verdict::Refused(p.refuse("unclosed universe"));
        }
        match p.peek_ident().as_deref() {
            Some("codex") => {
                p.take_ident();
                match p.number_u32() {
                    Ok(n) => codex = n as u16,
                    Err(r) => return Verdict::Refused(r),
                }
            }
            Some("bodies") => match parse_bodies(&mut p) {
                Verdict::Ok(b) => bodies = b,
                Verdict::Refused(r) => return Verdict::Refused(r),
            },
            Some("links") => match parse_links(&mut p) {
                Verdict::Ok((l, w)) => {
                    links = l;
                    cross_wires = w;
                }
                Verdict::Refused(r) => return Verdict::Refused(r),
            },
            Some("grants") => match parse_grants(&mut p) {
                Verdict::Ok(g) => grants = g,
                Verdict::Refused(r) => return Verdict::Refused(r),
            },
            Some("lenses") => match parse_lenses(&mut p) {
                Verdict::Ok(l) => lenses = l,
                Verdict::Refused(r) => return Verdict::Refused(r),
            },
            Some(other) => {
                return Verdict::Refused(p.refuse(format!("unknown universe section {other}")));
            }
            None => return Verdict::Refused(p.refuse("expected a universe section")),
        }
    }
    let aliases: BTreeSet<String> = bodies.iter().map(|b| b.alias.clone()).collect();
    for link in &links {
        for member in &link.members {
            if !aliases.contains(&member.body) {
                return Verdict::Refused(Refusal::structural(
                    CheckId::Parse,
                    format!(
                        "body alias {} is not declared; acceptance is a name in bodies",
                        member.body
                    ),
                ));
            }
        }
    }
    for lens in &lenses {
        for galaxy in &lens.galaxies {
            for system in &galaxy.systems {
                for alias in &system.bodies {
                    if !aliases.contains(alias) {
                        return Verdict::Refused(Refusal::structural(
                            CheckId::Parse,
                            format!(
                                "body alias {alias} is not declared; acceptance is a name in bodies"
                            ),
                        ));
                    }
                }
            }
        }
    }
    for (link_id, body) in &grants {
        if !aliases.contains(body) {
            return Verdict::Refused(Refusal::structural(
                CheckId::Parse,
                format!("body alias {body} is not declared; acceptance is a name in bodies"),
            ));
        }
        let Some(link) = links.iter().find(|l| l.id == *link_id) else {
            return Verdict::Refused(Refusal::structural(
                CheckId::Parse,
                format!("link {link_id} is not in this universe; acceptance is a declared link"),
            ));
        };
        if link.order != Order::Ordered {
            return Verdict::Refused(Refusal::structural(
                CheckId::Parse,
                format!(
                    "link {link_id} is not ordered for grant to {body}; acceptance is an ordered hyperedge for a capability"
                ),
            ));
        }
        if !link.members.iter().any(|m| m.body == *body) {
            return Verdict::Refused(Refusal::structural(
                CheckId::Parse,
                format!(
                    "body {body} is not a member of link {link_id}; acceptance is a member of that link"
                ),
            ));
        }
    }
    let coding = UniverseCoding {
        codex,
        bodies,
        links,
        cross_wires,
        grants,
        lenses,
    };
    let regulatory = parse_regulatory(rest);
    Verdict::Ok(Universe { coding, regulatory })
}

fn parse_bodies(p: &mut Cursor<'_>) -> Verdict<Vec<BodyBinding>> {
    p.take_ident();
    p.skip();
    if !p.take_char('{') {
        return Verdict::Refused(p.refuse("expected { after bodies"));
    }
    let mut out = Vec::new();
    loop {
        p.skip();
        if p.peek() == Some('}') {
            p.advance();
            break;
        }
        if p.take_ident() != "body" {
            return Verdict::Refused(p.refuse("expected body"));
        }
        p.skip();
        if !p.take_char(':') {
            return Verdict::Refused(p.refuse("expected : after body"));
        }
        let hash = match p.hex_hash() {
            Ok(h) => h,
            Err(r) => return Verdict::Refused(r),
        };
        p.skip();
        if p.take_ident() != "as" {
            return Verdict::Refused(p.refuse("expected as after body hash"));
        }
        p.skip();
        let alias = p.take_ident();
        if alias.is_empty() {
            return Verdict::Refused(p.refuse("expected body alias"));
        }
        out.push(BodyBinding { hash, alias });
    }
    Verdict::Ok(out)
}

fn parse_links(p: &mut Cursor<'_>) -> Verdict<(Vec<Link>, Vec<CrossWire>)> {
    p.take_ident();
    p.skip();
    if !p.take_char('{') {
        return Verdict::Refused(p.refuse("expected { after links"));
    }
    let mut out = Vec::new();
    let mut wires = Vec::new();
    loop {
        p.skip();
        if p.peek() == Some('}') {
            p.advance();
            break;
        }
        match p.peek_ident().as_deref() {
            Some("wire") => match parse_cross_wire(p) {
                Verdict::Ok(w) => wires.push(w),
                Verdict::Refused(r) => return Verdict::Refused(r),
            },
            Some("link") => {
                p.take_ident();
                p.skip();
                let id = p.take_ident();
                if id.is_empty() {
                    return Verdict::Refused(p.refuse("expected link id"));
                }
                p.skip();
                if p.take_ident() != "order" {
                    return Verdict::Refused(p.refuse("expected order after link id"));
                }
                p.skip();
                let order = match p.take_ident().as_str() {
                    "none" => Order::None,
                    "ordered" => Order::Ordered,
                    other => {
                        return Verdict::Refused(p.refuse(format!(
                            "order {other} is not none or ordered; acceptance is a declared order"
                        )));
                    }
                };
                p.skip();
                if !p.take_char('{') {
                    return Verdict::Refused(p.refuse("expected { after link order"));
                }
                let mut members = Vec::new();
                loop {
                    p.skip();
                    if p.peek() == Some('}') {
                        p.advance();
                        break;
                    }
                    match parse_member(p) {
                        Verdict::Ok(m) => members.push(m),
                        Verdict::Refused(r) => return Verdict::Refused(r),
                    }
                }
                out.push(Link { id, order, members });
            }
            Some(other) => {
                return Verdict::Refused(p.refuse(format!("expected link or wire, found {other}")));
            }
            None => return Verdict::Refused(p.refuse("expected link or wire")),
        }
    }
    Verdict::Ok((out, wires))
}

fn parse_cross_wire(p: &mut Cursor<'_>) -> Verdict<CrossWire> {
    p.take_ident();
    p.skip();
    let src = match parse_member(p) {
        Verdict::Ok(m) => m,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    p.skip();
    if !(p.take_char('-') && p.take_char('>')) {
        return Verdict::Refused(p.refuse("expected -> in a cross-body wire"));
    }
    p.skip();
    let dst = match parse_member(p) {
        Verdict::Ok(m) => m,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    Verdict::Ok(CrossWire {
        src_body: src.body,
        src_instance: src.instance,
        src_port: src.port,
        dst_body: dst.body,
        dst_instance: dst.instance,
        dst_port: dst.port,
    })
}

fn parse_member(p: &mut Cursor<'_>) -> Verdict<Member> {
    let body = p.take_ident();
    if body.is_empty() {
        return Verdict::Refused(p.refuse("expected member body alias"));
    }
    if !p.take_char('.') {
        return Verdict::Refused(p.refuse(format!("expected . after body alias {body}")));
    }
    let instance = p.take_ident();
    if instance.is_empty() {
        return Verdict::Refused(p.refuse("expected member instance"));
    }
    if !p.take_char('@') {
        return Verdict::Refused(p.refuse(format!(
            "expected @ after {body}.{instance}; acceptance is body.instance@port"
        )));
    }
    let port = match p.number_u32() {
        Ok(n) => n,
        Err(r) => return Verdict::Refused(r),
    };
    p.skip();
    let mark = match p.peek_ident().as_deref() {
        Some("tail") => {
            p.take_ident();
            Mark::Tail
        }
        Some("head") => {
            p.take_ident();
            Mark::Head
        }
        _ => Mark::None,
    };
    Verdict::Ok(Member {
        body,
        instance,
        port,
        mark,
    })
}

fn parse_grants(p: &mut Cursor<'_>) -> Verdict<BTreeMap<String, String>> {
    p.take_ident();
    p.skip();
    if !p.take_char('{') {
        return Verdict::Refused(p.refuse("expected { after grants"));
    }
    let mut out = BTreeMap::new();
    loop {
        p.skip();
        if p.peek() == Some('}') {
            p.advance();
            break;
        }
        let link_id = p.take_ident();
        if link_id.is_empty() {
            return Verdict::Refused(p.refuse("expected grant link id"));
        }
        p.skip();
        if !p.take_char(':') {
            return Verdict::Refused(p.refuse(format!("expected : after grant link {link_id}")));
        }
        p.skip();
        let body = p.take_ident();
        if body.is_empty() {
            return Verdict::Refused(
                p.refuse(format!("expected body alias for grant on {link_id}")),
            );
        }
        if out.insert(link_id.clone(), body).is_some() {
            return Verdict::Refused(p.refuse(format!(
                "grant on link {link_id} is declared twice; acceptance is one line per link"
            )));
        }
    }
    Verdict::Ok(out)
}

fn parse_lenses(p: &mut Cursor<'_>) -> Verdict<Vec<Lens>> {
    p.take_ident();
    p.skip();
    if !p.take_char('{') {
        return Verdict::Refused(p.refuse("expected { after lenses"));
    }
    let mut out = Vec::new();
    loop {
        p.skip();
        if p.peek() == Some('}') {
            p.advance();
            break;
        }
        if p.take_ident() != "lens" {
            return Verdict::Refused(p.refuse("expected lens"));
        }
        p.skip();
        let name = p.take_ident();
        if name.is_empty() {
            return Verdict::Refused(p.refuse("expected lens name"));
        }
        p.skip();
        if !p.take_char('{') {
            return Verdict::Refused(p.refuse("expected { after lens name"));
        }
        let mut galaxies = Vec::new();
        loop {
            p.skip();
            if p.peek() == Some('}') {
                p.advance();
                break;
            }
            match parse_galaxy(p) {
                Verdict::Ok(g) => galaxies.push(g),
                Verdict::Refused(r) => return Verdict::Refused(r),
            }
        }
        out.push(Lens { name, galaxies });
    }
    Verdict::Ok(out)
}

fn parse_galaxy(p: &mut Cursor<'_>) -> Verdict<Galaxy> {
    if p.take_ident() != "galaxy" {
        return Verdict::Refused(p.refuse("expected galaxy"));
    }
    p.skip();
    let name = p.take_ident();
    if name.is_empty() {
        return Verdict::Refused(p.refuse("expected galaxy name"));
    }
    p.skip();
    if !p.take_char('{') {
        return Verdict::Refused(p.refuse("expected { after galaxy name"));
    }
    let mut systems = Vec::new();
    loop {
        p.skip();
        if p.peek() == Some('}') {
            p.advance();
            break;
        }
        match parse_system(p) {
            Verdict::Ok(s) => systems.push(s),
            Verdict::Refused(r) => return Verdict::Refused(r),
        }
    }
    Verdict::Ok(Galaxy { name, systems })
}

fn parse_system(p: &mut Cursor<'_>) -> Verdict<System> {
    if p.take_ident() != "system" {
        return Verdict::Refused(p.refuse("expected system"));
    }
    p.skip();
    let name = p.take_ident();
    if name.is_empty() {
        return Verdict::Refused(p.refuse("expected system name"));
    }
    p.skip();
    if !p.take_char('{') {
        return Verdict::Refused(p.refuse("expected { after system name"));
    }
    let mut bodies = Vec::new();
    loop {
        p.skip();
        if p.peek() == Some('}') {
            p.advance();
            break;
        }
        let alias = p.take_ident();
        if alias.is_empty() {
            return Verdict::Refused(p.refuse("expected body alias in system"));
        }
        bodies.push(alias);
    }
    Verdict::Ok(System { name, bodies })
}

fn parse_regulatory(src: &str) -> UniverseRegulatory {
    let mut names = BTreeMap::new();
    let mut labels = BTreeMap::new();
    let mut p = Cursor::new(src);
    p.skip();
    if p.take_ident() != "regulatory" {
        return UniverseRegulatory::default();
    }
    p.skip();
    if !p.take_char('{') {
        return UniverseRegulatory::default();
    }
    loop {
        p.skip();
        if p.peek() == Some('}') || p.eof() {
            break;
        }
        match p.peek_ident().as_deref() {
            Some("names") => {
                p.take_ident();
                names = parse_string_map(&mut p);
            }
            Some("labels") => {
                p.take_ident();
                labels = parse_string_map(&mut p);
            }
            _ => {
                p.take_ident();
            }
        }
    }
    UniverseRegulatory { names, labels }
}

fn parse_string_map(p: &mut Cursor<'_>) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    p.skip();
    if !p.take_char('{') {
        return out;
    }
    loop {
        p.skip();
        if p.peek() == Some('}') {
            p.advance();
            break;
        }
        let key = p.take_ident();
        if key.is_empty() {
            break;
        }
        p.skip();
        let value = match p.quoted() {
            Ok(v) => v,
            Err(_) => break,
        };
        out.insert(key, value);
    }
    out
}

fn strip_comments(src: &str) -> String {
    let mut out = String::new();
    for line in src.lines() {
        let cut = match line.find('#') {
            Some(i) => &line[..i],
            None => line,
        };
        out.push_str(cut);
        out.push('\n');
    }
    out
}

fn split_delimiter(src: &str) -> (&str, &str) {
    match src.find("\n---\n") {
        Some(i) => (&src[..i], &src[i + 5..]),
        None => match src.find("\n---") {
            Some(i) => (&src[..i], &src[i + 4..]),
            None => (src, ""),
        },
    }
}

struct Cursor<'a> {
    src: &'a str,
    i: usize,
}

impl<'a> Cursor<'a> {
    fn new(src: &'a str) -> Self {
        Self { src, i: 0 }
    }

    fn eof(&self) -> bool {
        self.i >= self.src.len()
    }

    fn rest(&self) -> &'a str {
        &self.src[self.i..]
    }

    fn peek(&self) -> Option<char> {
        self.rest().chars().next()
    }

    fn advance(&mut self) {
        if let Some(c) = self.peek() {
            self.i += c.len_utf8();
        }
    }

    fn skip(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_whitespace()) {
            self.advance();
        }
    }

    fn take_char(&mut self, want: char) -> bool {
        if self.peek() == Some(want) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn peek_ident(&self) -> Option<String> {
        let mut chars = self.rest().chars();
        let first = chars.next()?;
        if !(first.is_ascii_alphabetic() || first == '_') {
            return None;
        }
        let mut s = String::from(first);
        for c in chars {
            if c.is_ascii_alphanumeric() || c == '_' {
                s.push(c);
            } else {
                break;
            }
        }
        Some(s)
    }

    fn take_ident(&mut self) -> String {
        self.skip();
        let Some(id) = self.peek_ident() else {
            return String::new();
        };
        self.i += id.len();
        id
    }

    fn number_u32(&mut self) -> Result<u32, Refusal> {
        self.skip();
        let start = self.i;
        while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
            self.advance();
        }
        if self.i == start {
            return Err(self.refuse("expected a number"));
        }
        self.src[start..self.i]
            .parse()
            .map_err(|_| self.refuse("number out of range"))
    }

    fn hex_hash(&mut self) -> Result<Hash, Refusal> {
        self.skip();
        let start = self.i;
        while matches!(self.peek(), Some(c) if c.is_ascii_hexdigit()) {
            self.advance();
        }
        let hex = &self.src[start..self.i];
        Hash::parse_hex(hex).ok_or_else(|| self.refuse(format!("not a body hash: {hex}")))
    }

    fn quoted(&mut self) -> Result<String, Refusal> {
        self.skip();
        if !self.take_char('"') {
            return Err(self.refuse("expected a quoted string"));
        }
        let mut out = String::new();
        loop {
            match self.peek() {
                None => return Err(self.refuse("unclosed string")),
                Some('"') => {
                    self.advance();
                    return Ok(out);
                }
                Some('\\') => {
                    self.advance();
                    match self.peek() {
                        Some(c) => {
                            out.push(c);
                            self.advance();
                        }
                        None => return Err(self.refuse("unclosed string")),
                    }
                }
                Some(c) => {
                    out.push(c);
                    self.advance();
                }
            }
        }
    }

    fn refuse(&self, reason: impl Into<String>) -> Refusal {
        Refusal {
            check: CheckId::Parse,
            subject: Subject::Other(String::new()),
            reason: reason.into(),
            counterexample: None,
            seed: 0,
        }
    }
}
