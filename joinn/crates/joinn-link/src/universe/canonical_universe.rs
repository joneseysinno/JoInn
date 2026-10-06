//! A universe with its links in print order.

use super::{Order, Universe};

/// `universe` with its links sorted by id and the members of each unordered
/// link sorted, the order `print_universe` writes them. The hash does not
/// move (it is the hash of that text); what a picture numbers by slot (link
/// slots, `member <i>`) is then a function of the hash alone.
pub fn canonical_universe(universe: &Universe) -> Universe {
    let mut out = universe.clone();
    out.coding.links.sort_by(|a, b| a.id.cmp(&b.id));
    for link in &mut out.coding.links {
        if link.order == Order::None {
            link.members.sort();
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use super::canonical_universe;
    use crate::universe::{
        Link, Mark, Member, Order, hash_universe, parse_universe, print_universe,
    };

    fn member(body: &str, port: u32) -> Member {
        Member {
            body: body.to_owned(),
            instance: "sum".to_owned(),
            port,
            mark: Mark::None,
        }
    }

    #[test]
    fn links_by_id_and_unordered_members_sorted_keep_the_text_and_the_hash() {
        let mut u =
            match parse_universe(include_str!("../../../../corpus/phase5/universe.universe")) {
                Verdict::Ok(u) => u,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
        u.coding.links.push(Link {
            id: "a0".to_owned(),
            order: Order::None,
            members: vec![member("units", 1), member("calc", 2), member("calc", 0)],
        });
        let c = canonical_universe(&u);
        let ids: Vec<&str> = c.coding.links.iter().map(|l| l.id.as_str()).collect();
        assert_eq!(ids, ["a0", "e0"]);
        let a0: Vec<(&str, u32)> = c.coding.links[0]
            .members
            .iter()
            .map(|m| (m.body.as_str(), m.port))
            .collect();
        assert_eq!(a0, [("calc", 0), ("calc", 2), ("units", 1)]);
        let e0: Vec<&str> = c.coding.links[1]
            .members
            .iter()
            .map(|m| m.body.as_str())
            .collect();
        assert_eq!(e0, ["calc", "units"], "ordered members keep their order");
        assert_eq!(print_universe(&c.coding), print_universe(&u.coding));
        assert_eq!(hash_universe(&c.coding), hash_universe(&u.coding));
        assert_eq!(canonical_universe(&c).coding, c.coding);
    }
}
