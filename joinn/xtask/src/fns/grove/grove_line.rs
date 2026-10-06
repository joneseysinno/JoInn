//! The grove's one line: its counts, its admission and its hash.

use joinn_frame::Verdict;
use joinn_link::{BodyStore, Universe, hash_universe};

use super::{KINDS, admit_universe};

/// `grove seed <n>: … , admitted; hash <hex>`, and whether it was admitted. A
/// refusal prints `refused: <reason>` where `admitted` stands.
pub(crate) fn grove_line(seed: u64, universe: &Universe, store: &BodyStore) -> (String, bool) {
    let lens = universe.coding.lenses.first();
    let galaxies = lens.map_or(0, |l| l.galaxies.len());
    let systems: usize = lens.map_or(0, |l| l.galaxies.iter().map(|g| g.systems.len()).sum());
    let kinds: Vec<String> = KINDS
        .iter()
        .map(|k| {
            let n = universe
                .coding
                .bodies
                .iter()
                .filter(|b| b.hash.to_hex() == k.hash_hex())
                .count();
            format!("{} {n}", k.name())
        })
        .collect();
    let members: usize = universe.coding.links.iter().map(|l| l.members.len()).sum();
    let (verdict, admitted) = match admit_universe(universe, store) {
        Verdict::Ok(_) => ("admitted".to_owned(), true),
        Verdict::Refused(r) => (format!("refused: {}", r.reason), false),
    };
    let line = format!(
        "grove seed {seed}: {galaxies} galaxies, {systems} systems, {} bodies ({}), {} links, {members} members, {verdict}; hash {}",
        universe.coding.bodies.len(),
        kinds.join(", "),
        universe.coding.links.len(),
        hash_universe(&universe.coding).to_hex()
    );
    (line, admitted)
}

#[cfg(test)]
mod tests {
    use joinn_link::print_universe;

    use super::grove_line;
    use crate::fns::corpus_store::corpus_store;
    use crate::fns::grove::grow_grove;

    fn line(seed: u64) -> (String, String, bool) {
        let u = match grow_grove(seed) {
            Ok(u) => u,
            Err(e) => panic!("{e}"),
        };
        let store = match corpus_store() {
            Ok((_, store)) => store,
            Err(e) => panic!("{e}"),
        };
        let (l, admitted) = grove_line(seed, &u, store);
        (l, print_universe(&u.coding), admitted)
    }

    fn hash_of(line: &str) -> &str {
        line.rsplit("hash ").next().unwrap_or("")
    }

    #[test]
    fn seed_7_twice_prints_the_same_hash_and_text_and_seed_8_another_hash() {
        let (a, text_a, _) = line(7);
        let (b, text_b, _) = line(7);
        let (c, _, _) = line(8);
        assert_eq!(a, b);
        assert_eq!(text_a, text_b);
        assert_eq!(hash_of(&a).len(), 64);
        assert_ne!(hash_of(&a), hash_of(&c));
    }

    #[test]
    fn seed_7_is_admitted_with_the_plan_s_counts() {
        let (l, _, admitted) = line(7);
        assert!(
            l.starts_with(
                "grove seed 7: 8 galaxies, 128 systems, 3072 bodies (calc 1181, units 982, bus 909), 137 links, 1182 members, admitted; hash "
            ),
            "{l}"
        );
        assert!(admitted, "{l}");
    }
}
