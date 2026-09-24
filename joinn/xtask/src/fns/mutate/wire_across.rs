//! Turn a hyperedge into a cross-body wire (wrong container).

use joinn_frame::Verdict;
use joinn_link::{CrossWire, Universe};

use super::refuse::refuse;

pub(super) fn wire_across(u: &mut Universe, link_id: &str) -> Verdict<()> {
    let Some(pos) = u.coding.links.iter().position(|l| l.id == link_id) else {
        return Verdict::Refused(refuse(format!(
            "no such link {link_id}; acceptance is a declared link id"
        )));
    };
    let link = u.coding.links.remove(pos);
    if link.members.len() < 2 {
        return Verdict::Refused(refuse(format!(
            "link {link_id} has fewer than two members; acceptance is a pair to wire"
        )));
    }
    let src = &link.members[0];
    let dst = &link.members[1];
    u.coding.cross_wires.push(CrossWire {
        src_body: src.body.clone(),
        src_instance: src.instance.clone(),
        src_port: src.port,
        dst_body: dst.body.clone(),
        dst_instance: dst.instance.clone(),
        dst_port: dst.port,
    });
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use crate::fns::mutate::{mutate, Mutation};
    use crate::fns::parse_subject::parse_subject;
    use crate::fns::subject::Subject;
    use joinn_frame::Verdict;
    use joinn_link::{check_law4, hash_universe};

    fn universe() -> Subject {
        let src = include_str!("../../../../corpus/phase5/universe.universe");
        parse_subject("phase5/universe.universe", src).expect("parse")
    }

    #[test]
    fn wire_across_law4_refuses_container() {
        let s = universe();
        let Subject::Universe(orig) = &s else {
            panic!("universe");
        };
        let orig_hash = hash_universe(&orig.coding);
        let Verdict::Ok(mutant) = mutate(&s, &Mutation::WireAcross("e0")) else {
            panic!("mutate");
        };
        let Subject::Universe(u) = &mutant else {
            panic!("universe mutant");
        };
        assert_ne!(hash_universe(&u.coding), orig_hash);
        assert!(u.coding.links.iter().all(|l| l.id != "e0"));
        assert!(!u.coding.cross_wires.is_empty());
        match check_law4(u) {
            Verdict::Refused(r) => {
                assert!(
                    r.reason.contains("wire") || r.reason.contains("hyperedge"),
                    "{}",
                    r.reason
                );
            }
            Verdict::Ok(()) => panic!("Law 4 must refuse a cross-body wire"),
        }
        let Verdict::Refused(r) = mutate(&s, &Mutation::WireAcross("ghost")) else {
            panic!("missing link must refuse");
        };
        assert!(r.reason.contains("ghost"), "{}", r.reason);
    }
}
