//! Dispatch a catalogue mutation onto a subject value.

use crate::fns::subject::Subject;
use joinn_frame::Verdict;

use super::Mutation;
use super::accepts::accepts;
use super::add_wire::add_wire;
use super::copy_member::copy_member;
use super::corrupt_hash::corrupt_hash;
use super::drop_contact_genome::drop_contact_genome;
use super::drop_contact_lineage::drop_contact_lineage;
use super::drop_declaration::drop_declaration;
use super::drop_force::drop_force;
use super::drop_genome::drop_genome;
use super::drop_grant::drop_grant;
use super::drop_lens::drop_lens;
use super::drop_line::drop_line;
use super::drop_lineage::drop_lineage;
use super::drop_link::drop_link;
use super::drop_member::drop_member;
use super::drop_system_force::drop_system_force;
use super::drop_wire::drop_wire;
use super::flip_mark::flip_mark;
use super::force_on::force_on;
use super::refuse::refuse;
use super::rename_alias::rename_alias;
use super::rename_contact_alias::rename_contact_alias;
use super::rename_link::rename_link;
use super::replace::replace;
use super::set_score::set_score;
use super::shift_member::shift_member;
use super::shift_port::shift_port;
use super::swap_binding::swap_binding;
use super::swap_cell::swap_cell;
use super::swap_lines::swap_lines;
use super::swap_response::swap_response;
use super::system_accepts::system_accepts;
use super::wire_across::wire_across;

pub(super) fn apply(s: &mut Subject, m: &Mutation) -> Verdict<()> {
    match (m, s) {
        (Mutation::DropLink(id), Subject::Universe(u)) => drop_link(u, id),
        (Mutation::FlipMark(link, member), Subject::Universe(u)) => flip_mark(u, link, member),
        (Mutation::ShiftPort(link, member, to), Subject::Universe(u)) => {
            shift_port(u, link, member, *to)
        }
        (Mutation::SwapBinding(alias, to_hash), Subject::Universe(u)) => {
            swap_binding(u, alias, to_hash)
        }
        (Mutation::CorruptHash(alias), Subject::Universe(u)) => corrupt_hash(u, alias),
        (Mutation::CopyMember(lens, alias, into), Subject::Universe(u)) => {
            copy_member(u, lens, alias, into)
        }
        (Mutation::DropLens(name), Subject::Universe(u)) => drop_lens(u, name),
        (Mutation::WireAcross(link), Subject::Universe(u)) => wire_across(u, link),
        (Mutation::DropGrant(link), Subject::Universe(u)) => drop_grant(u, link),
        (Mutation::RenameLink(from, to), Subject::Universe(u)) => rename_link(u, from, to),
        (Mutation::RenameAlias(from, to), Subject::Universe(u)) => rename_alias(u, from, to),
        (Mutation::DropWire(src, dst), Subject::Body(b)) => drop_wire(b, src, dst),
        (Mutation::DropGenome(inst), Subject::Body(b)) => drop_genome(b, inst),
        (Mutation::DropGenome(inst), Subject::Contact(c)) => drop_contact_genome(c, inst),
        (Mutation::RenameAlias(from, to), Subject::Contact(c)) => rename_contact_alias(c, from, to),
        (Mutation::DropForce(response), Subject::Contact(c)) => drop_force(c, response),
        (Mutation::SwapResponse(response, to_hash), Subject::Contact(c)) => {
            swap_response(c, response, to_hash)
        }
        (Mutation::ShiftMember(response, member, to), Subject::Contact(c)) => {
            shift_member(c, response, member, *to)
        }
        (Mutation::DropMember(response, member), Subject::Contact(c)) => {
            drop_member(c, response, member)
        }
        (Mutation::SwapCell(inst, to_hash), Subject::Body(b)) => swap_cell(b, inst, to_hash),
        (Mutation::AddWire(src, dst), Subject::Body(b)) => add_wire(b, src, dst),
        (Mutation::DropDeclaration, Subject::Body(b)) => {
            drop_declaration(&mut b.coding.declarations)
        }
        (Mutation::DropDeclaration, Subject::Universe(u)) => {
            drop_declaration(&mut u.coding.declarations)
        }
        (Mutation::SetScore(phase, n, total), Subject::Lock(rows)) => {
            set_score(rows, phase, *n, *total)
        }
        (Mutation::DropLine(i), Subject::Transcript(lines)) => drop_line(lines, *i),
        (Mutation::SwapLines(i, j), Subject::Transcript(lines)) => swap_lines(lines, *i, *j),
        (Mutation::Replace(text), Subject::Text(t)) => replace(t, text),
        (Mutation::Accepts(name, accept), Subject::Contact(c)) => accepts(c, name, *accept),
        (Mutation::Accepts(alias, accept), Subject::System(s)) => system_accepts(s, alias, *accept),
        (Mutation::DropForce(response), Subject::System(s)) => drop_system_force(s, response),
        (Mutation::DropLineage(name), Subject::Contact(c)) => drop_contact_lineage(c, name),
        (Mutation::DropLineage(name), Subject::System(s)) => drop_lineage(s, name),
        (Mutation::ForceOn(response, alias), Subject::System(s)) => force_on(s, response, alias),
        (m, other) => {
            let kind = match other {
                Subject::Body(_) => "body",
                Subject::Contact(_) => "contact",
                Subject::System(_) => "system",
                Subject::Universe(_) => "universe",
                Subject::Lock(_) => "lock",
                Subject::Transcript(_) => "transcript",
                Subject::Text(_) => "text",
            };
            let acceptance = match m {
                Mutation::Accepts(..) => "; acceptance is a system or a contact that grows",
                Mutation::DropForce(_) => "; acceptance is a contact or a system",
                Mutation::DropLineage(_) => "; acceptance is a system or a contact that grows",
                Mutation::ForceOn(..) => "; acceptance is a system",
                _ => "",
            };
            Verdict::Refused(refuse(format!(
                "mutation {m:?} does not apply to this subject kind ({kind}){acceptance}"
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::apply;
    use crate::fns::mutate::Mutation;
    use crate::fns::parse_subject::parse_subject;
    use joinn_dna::Accept;
    use joinn_frame::Verdict;

    #[test]
    fn each_system_mutation_on_a_universe_names_the_kind_it_needs() {
        let cases = [
            (
                Mutation::Accepts("numbers", Accept::Any),
                "a system or a contact that grows",
            ),
            (Mutation::DropForce("count"), "a contact or a system"),
            (
                Mutation::DropLineage("numbers"),
                "a system or a contact that grows",
            ),
            (Mutation::ForceOn("count", "numbers"), "a system"),
        ];
        for (m, kind) in cases {
            let src = include_str!("../../../../corpus/phase5/universe.universe");
            let mut u =
                parse_subject("phase5/universe.universe", src).unwrap_or_else(|e| panic!("{e}"));
            let Verdict::Refused(r) = apply(&mut u, &m) else {
                panic!("{m:?} applied to a universe");
            };
            assert!(
                r.reason
                    .ends_with(&format!("(universe); acceptance is {kind}")),
                "{}",
                r.reason
            );
        }
    }
}
