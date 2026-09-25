//! Dispatch a catalogue mutation onto a subject value.

use crate::fns::subject::Subject;
use joinn_frame::Verdict;

use super::Mutation;
use super::copy_member::copy_member;
use super::corrupt_hash::corrupt_hash;
use super::drop_genome::drop_genome;
use super::drop_grant::drop_grant;
use super::drop_lens::drop_lens;
use super::drop_line::drop_line;
use super::drop_link::drop_link;
use super::drop_wire::drop_wire;
use super::flip_mark::flip_mark;
use super::refuse::refuse;
use super::rename_alias::rename_alias;
use super::rename_link::rename_link;
use super::replace::replace;
use super::set_score::set_score;
use super::shift_port::shift_port;
use super::swap_binding::swap_binding;
use super::swap_cell::swap_cell;
use super::swap_lines::swap_lines;
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
        (Mutation::SwapCell(inst, to_hash), Subject::Body(b)) => swap_cell(b, inst, to_hash),
        (Mutation::SetScore(phase, n, total), Subject::Lock(rows)) => {
            set_score(rows, phase, *n, *total)
        }
        (Mutation::DropLine(i), Subject::Transcript(lines)) => drop_line(lines, *i),
        (Mutation::SwapLines(i, j), Subject::Transcript(lines)) => swap_lines(lines, *i, *j),
        (Mutation::Replace(text), Subject::Text(t)) => replace(t, text),
        (m, other) => {
            let kind = match other {
                Subject::Body(_) => "body",
                Subject::Universe(_) => "universe",
                Subject::Lock(_) => "lock",
                Subject::Transcript(_) => "transcript",
                Subject::Text(_) => "text",
            };
            Verdict::Refused(refuse(format!(
                "mutation {m:?} does not apply to this subject kind ({kind})"
            )))
        }
    }
}
