//! Check one reference's stated opposition on samples.

use joinn_frame::{Frame, IntFrame, RatFrame, Term, TextFrame, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::Reference;
use super::build_prim::BuildPrim;
use super::case_prim::CasePrim;
use super::eq_prim::EqPrim;
use super::hash_prim::HashPrim;
use super::pair::Pair;
use super::resolve_prim::ResolvePrim;
use super::split::Split;

pub(in crate::floor) fn check_one(p: &dyn Reference, seed: u64, n: u32) -> Option<String> {
    let int = IntFrame::new();
    let text = TextFrame::new();
    let rat = RatFrame::new();
    match p.name() {
        "pair" | "split" => {
            let pair = Pair;
            let split = Split;
            for i in 0..n {
                let x = int.generate(seed.wrapping_add(u64::from(i)), 8);
                let y = int.generate(seed.wrapping_add(u64::from(i) + 1), 8);
                let mid = match pair.apply(&BTreeMap::from([(0, x.clone()), (1, y.clone())])) {
                    Verdict::Ok(m) => m,
                    Verdict::Refused(r) => return Some(r.reason),
                };
                let back = match split.apply(&mid) {
                    Verdict::Ok(m) => m,
                    Verdict::Refused(r) => return Some(r.reason),
                };
                if back.get(&0) != Some(&x) || back.get(&1) != Some(&y) {
                    return Some(format!("split(pair(x,y)) ≠ (x,y) (seed {seed})"));
                }
            }
            None
        }
        "choose" => {
            let eq = EqPrim;
            for i in 0..n {
                let x = int.generate(seed.wrapping_add(u64::from(i)), 8);
                let a = int.generate(seed.wrapping_add(u64::from(i) + 3), 8);
                let b = int.generate(seed.wrapping_add(u64::from(i) + 7), 8);
                let cond = match eq.apply(&BTreeMap::from([(0, x.clone()), (1, x.clone())])) {
                    Verdict::Ok(m) => m,
                    Verdict::Refused(r) => return Some(r.reason),
                };
                let Some(c) = cond.get(&0) else {
                    return Some("eq produced no out-port".into());
                };
                match p.apply(&BTreeMap::from([
                    (0, c.clone()),
                    (1, a.clone()),
                    (2, b.clone()),
                ])) {
                    Verdict::Ok(out) if out.get(&0) == Some(&a) => {}
                    _ => return Some(format!("choose(eq(x,x), a, b) ≠ a (seed {seed})")),
                }
            }
            None
        }
        "eq" => {
            for i in 0..n {
                let x = int.generate(seed.wrapping_add(u64::from(i)), 8);
                match p.apply(&BTreeMap::from([(0, x.clone()), (1, x.clone())])) {
                    Verdict::Ok(m) => match m.get(&0).map(|v| v.term()) {
                        Some(Term::Int(n)) if n == &1.into() => {}
                        _ => return Some(format!("eq(x,x) is not 1 (seed {seed})")),
                    },
                    Verdict::Refused(r) => return Some(r.reason),
                }
            }
            None
        }
        "build" | "case" => {
            let build = BuildPrim;
            let case = CasePrim;
            for v in (0..n).flat_map(|i| {
                let k = seed.wrapping_add(u64::from(i));
                [int.generate(k, 8), text.generate(k, 8), rat.generate(k, 6)]
            }) {
                let cased = match case.apply(&BTreeMap::from([(0, v.clone())])) {
                    Verdict::Ok(m) => m,
                    Verdict::Refused(r) => return Some(r.reason),
                };
                let Some(tag) = cased.get(&0) else {
                    return Some("case produced no tag".into());
                };
                let mut bins = BTreeMap::from([(0, v.clone()), (1, tag.clone())]);
                if let Some(p0) = cased.get(&1) {
                    bins.insert(2, p0.clone());
                }
                if let Some(p1) = cased.get(&2) {
                    bins.insert(3, p1.clone());
                }
                match build.apply(&bins) {
                    Verdict::Ok(out) if out.get(&0) == Some(&v) => {}
                    Verdict::Ok(_) => {
                        return Some(format!("build(frame, case(v)) ≠ v (seed {seed})"));
                    }
                    Verdict::Refused(r) => return Some(r.reason),
                }
            }
            None
        }
        "hash" | "resolve" => {
            let hash = HashPrim;
            let resolve = ResolvePrim;
            for i in 0..n {
                let x = int.generate(seed.wrapping_add(u64::from(i)), 8);
                let h = match hash.apply(&BTreeMap::from([(0, x.clone())])) {
                    Verdict::Ok(m) => m,
                    Verdict::Refused(r) => return Some(r.reason),
                };
                let Some(hv) = h.get(&0) else {
                    return Some("hash produced no digest".into());
                };
                match resolve.apply(&BTreeMap::from([(0, hv.clone())])) {
                    Verdict::Ok(back) if back.get(&0) == Some(&x) => {}
                    Verdict::Ok(_) => {
                        return Some(format!("resolve(hash(v)) ≠ v (seed {seed})"));
                    }
                    Verdict::Refused(r) => return Some(r.reason),
                }
            }
            match resolve.apply(&BTreeMap::from([(
                0,
                match TextFrame::new().canonicalize(Term::text(
                    "0000000000000000000000000000000000000000000000000000000000000000",
                )) {
                    Verdict::Ok(t) => t,
                    Verdict::Refused(r) => return Some(r.reason),
                },
            )])) {
                Verdict::Refused(r) if r.reason.contains("absent") => None,
                Verdict::Refused(r) => Some(r.reason),
                Verdict::Ok(_) => Some("resolve on an absent hash must refuse".into()),
            }
        }
        _ => Some(format!("{} has no opposition check", p.name())),
    }
}
