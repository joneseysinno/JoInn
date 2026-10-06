//! Closed mutation catalogue: damage that keeps the form.

mod add_wire;
mod apply;
mod coding_hash;
#[cfg(test)]
mod contact_admission;
#[cfg(test)]
mod contact_fixture;
mod copy_member;
#[cfg(test)]
mod corpus_fixture;
mod corrupt_hash;
mod drop_contact_genome;
mod drop_declaration;
mod drop_force;
mod drop_genome;
mod drop_grant;
mod drop_lens;
mod drop_line;
mod drop_link;
mod drop_member;
mod drop_wire;
mod flip_mark;
mod mutate_fn;
mod neutral;
mod parse_contact_member;
mod parse_endpoint;
mod parse_member_ref;
mod refuse;
mod rename_alias;
mod rename_contact_alias;
mod rename_link;
mod reparse;
mod replace;
mod reprint;
mod reprint_regulatory;
mod set_score;
mod shift_member;
mod shift_port;
mod swap_binding;
mod swap_cell;
mod swap_lines;
mod swap_response;
mod wire_across;

pub(crate) use mutate_fn::mutate;
pub(crate) use neutral::neutral;
pub(crate) use reparse::reparse;
pub(crate) use reprint::reprint;

/// One catalogue mutation. Closed; every non-legacy control opposes one.
#[allow(dead_code)] // closed catalogue; not every variant is declared by a gate row yet
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Mutation {
    DropLink(&'static str),
    FlipMark(&'static str, &'static str),
    ShiftPort(&'static str, &'static str, u32),
    SwapBinding(&'static str, &'static str),
    CorruptHash(&'static str),
    CopyMember(&'static str, &'static str, &'static str),
    DropLens(&'static str),
    WireAcross(&'static str),
    DropGrant(&'static str),
    RenameLink(&'static str, &'static str),
    RenameAlias(&'static str, &'static str),
    DropWire(&'static str, &'static str),
    DropGenome(&'static str),
    SwapCell(&'static str, &'static str),
    AddWire(&'static str, &'static str),
    DropDeclaration,
    SetScore(&'static str, u32, u32),
    DropLine(usize),
    SwapLines(usize, usize),
    Replace(&'static str),
    DropForce(&'static str),
    SwapResponse(&'static str, &'static str),
    ShiftMember(&'static str, &'static str, u32),
    DropMember(&'static str, &'static str),
}

#[cfg(test)]
mod tests {
    use crate::fns::{
        gate_five_items, gate_five_one_items, gate_five_two_items, gate_four_items, gate_one_items,
        gate_seven_items, gate_seven_three_items, gate_seven_two_items, gate_six_items,
        gate_three_items, gate_two_items, gate_two_one_items, gate_two_two_items,
    };

    #[test]
    fn only_gate_seven_names_a_contact_artifact() {
        let legacy = [
            gate_one_items(),
            gate_two_items(),
            gate_two_one_items(),
            gate_two_two_items(),
            gate_three_items(),
        ];
        let opposed = [
            gate_four_items(),
            gate_five_items(),
            gate_five_one_items(),
            gate_five_two_items(),
            gate_six_items(),
            gate_seven_two_items(),
            gate_seven_three_items(),
        ];
        let artifacts: Vec<&str> = legacy
            .iter()
            .flat_map(|t| t.iter().map(|i| i.control_artifact))
            .chain(
                opposed
                    .iter()
                    .flat_map(|t| t.iter().map(|i| i.control_artifact)),
            )
            .collect();
        assert!(artifacts.len() > 40, "{} rows", artifacts.len());
        let contacts: Vec<&&str> = artifacts
            .iter()
            .filter(|a| a.ends_with(".contact"))
            .collect();
        assert!(contacts.is_empty(), "{contacts:?}");
        let seven: Vec<&str> = gate_seven_items()
            .iter()
            .map(|i| i.control_artifact)
            .collect();
        assert_eq!(seven, vec!["corpus/phase7/calculator.contact"; 3]);
    }
}
