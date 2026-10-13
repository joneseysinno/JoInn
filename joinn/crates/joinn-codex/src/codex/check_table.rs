//! Check a table against the codon algebra.

use std::collections::BTreeMap;

use joinn_frame::Verdict;

use super::Row;
use crate::codon::Codon;
use crate::meaning::Meaning;
use crate::refuse::refuse;

/// Every codon's partner has the opposite parity and is another codon; every
/// row's right codon is its left codon's partner and means the opposite; no
/// codon or meaning is named twice; every reserved codon's partner is
/// reserved. The count of named codons, or the first fact that fails.
pub fn check_table(rows: &[Row]) -> Verdict<usize> {
    for c in Codon::all() {
        let p = c.partner();
        if p == c || p.parity() == c.parity() {
            return refuse(format!(
                "codex: {} ({}) pairs with {} ({}); acceptance is a partner of the opposite parity",
                c.text(),
                c.parity().name(),
                p.text(),
                p.parity().name()
            ));
        }
    }
    let mut named: BTreeMap<Codon, Meaning> = BTreeMap::new();
    for row in rows {
        let left = Codon::from_letters(row.codon);
        let right = Codon::from_letters(row.partner);
        if right != left.partner() {
            return refuse(format!(
                "codex: row {} lists partner {}; acceptance is {}, {}'s partner",
                left.text(),
                right.text(),
                left.partner().text(),
                left.text()
            ));
        }
        if row.partner_means != row.means.opposite() {
            return refuse(format!(
                "codex: {} means {} and its partner {} means {}; acceptance is {} meaning the opposite, {}",
                left.text(),
                row.means.name(),
                right.text(),
                row.partner_means.name(),
                right.text(),
                row.means.opposite().name()
            ));
        }
        for (c, m) in [(left, row.means), (right, row.partner_means)] {
            if let Some((other, _)) = named.iter().find(|(o, n)| **o == c || **n == m) {
                return refuse(format!(
                    "codex: {} meaning {} meets {} already named; acceptance is one codon per meaning",
                    c.text(),
                    m.name(),
                    other.text()
                ));
            }
            named.insert(c, m);
        }
    }
    for c in Codon::all() {
        if !named.contains_key(&c) && named.contains_key(&c.partner()) {
            return refuse(format!(
                "codex: {} is reserved but its partner {} is named; acceptance is reserved pairing with reserved",
                c.text(),
                c.partner().text()
            ));
        }
    }
    Verdict::Ok(named.len())
}
