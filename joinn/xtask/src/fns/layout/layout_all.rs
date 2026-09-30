//! `cargo xtask layout --all`: one block per corpus body or contact that binds.

use joinn_frame::Verdict;
use joinn_visual::{Layout, layout, layout_contact};

use super::corpus_bodies::corpus_bodies;
use super::corpus_contacts::corpus_contacts;
use super::layout_block::layout_block;

/// A body that doesn't bind, or whose layout is refused, is a `not measured`
/// line. Bodies and contacts merge in path order.
pub(crate) fn layout_all() -> Result<String, String> {
    let mut placed: Vec<(String, Verdict<Layout>)> = Vec::new();
    for (rel, bound) in corpus_bodies()? {
        let l = match bound {
            Verdict::Ok((body, cells)) => layout(&body, &cells),
            Verdict::Refused(r) => Verdict::Refused(r),
        };
        placed.push((rel, l));
    }
    let (contacts, cells) = corpus_contacts()?;
    for (rel, parsed) in contacts {
        let l = match parsed {
            Verdict::Ok(c) => layout_contact(&c, &cells),
            Verdict::Refused(r) => Verdict::Refused(r),
        };
        placed.push((rel, l));
    }
    placed.sort_by(|a, b| a.0.cmp(&b.0));
    let mut out = String::new();
    for (rel, l) in placed {
        match l {
            Verdict::Ok(l) => {
                out.push_str(&rel);
                out.push('\n');
                out.push_str(&layout_block(&l));
            }
            Verdict::Refused(r) => {
                out.push_str(&format!("{rel}: not measured: {}\n", r.reason));
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use joinn_visual::{layout, print_layout};

    use super::super::corpus_bodies::corpus_bodies;
    use crate::fns::mutate::neutral;
    use crate::fns::subject::Subject;

    fn printed(
        body: &joinn_dna::Body,
        cells: &std::collections::BTreeMap<joinn_frame::Hash, joinn_dna::Cell>,
    ) -> String {
        match layout(body, cells) {
            Verdict::Ok(l) => print_layout(&l),
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    #[test]
    fn a_layout_is_blind_to_the_regulatory_region_and_to_alleles() {
        let mut checked = Vec::new();
        for (rel, bound) in corpus_bodies().unwrap_or_else(|e| panic!("{e}")) {
            let Verdict::Ok((body, cells)) = bound else {
                continue;
            };
            let Verdict::Ok(before) = layout(&body, &cells) else {
                continue;
            };
            let before = print_layout(&before);

            let edited = match neutral(&Subject::Body(body.clone())) {
                Some(Subject::Body(edited)) => edited,
                _ => {
                    let mut edited = body.clone();
                    let first = body
                        .coding
                        .genome
                        .iter()
                        .flat_map(|e| e.instances.iter())
                        .next()
                        .cloned()
                        .unwrap_or_else(|| panic!("{rel}: no instance to label"));
                    edited.regulatory.labels.insert(first, "(neutral)".into());
                    edited
                }
            };
            assert_ne!(
                edited.regulatory, body.regulatory,
                "{rel}: no regulatory edit"
            );
            assert_eq!(
                printed(&edited, &cells),
                before,
                "{rel}: the neutral edit moved the layout"
            );

            let mut stripped = cells.clone();
            for cell in stripped.values_mut() {
                cell.alleles.clear();
            }
            assert_eq!(
                printed(&body, &stripped),
                before,
                "{rel}: stripping alleles moved the layout"
            );
            checked.push(rel);
        }
        assert!(
            checked.iter().any(|r| r == "phase2/calculator.body"),
            "the calculator must be among the checked bodies: {checked:?}"
        );
    }

    #[test]
    fn a_coding_edit_moves_the_layout() {
        let bodies = corpus_bodies().unwrap_or_else(|e| panic!("{e}"));
        let Some((_, Verdict::Ok((body, cells)))) = bodies
            .into_iter()
            .find(|(r, _)| r == "phase2/calculator.body")
        else {
            panic!("calculator must bind");
        };
        let before = printed(&body, &cells);
        let mut dropped = body.clone();
        for entry in &mut dropped.coding.genome {
            entry.instances.retain(|i| i != "cli_b");
        }
        dropped.coding.genome.retain(|e| !e.instances.is_empty());
        dropped
            .coding
            .wires
            .retain(|w| w.src_instance != "cli_b" && w.dst_instance != "cli_b");
        assert_ne!(printed(&dropped, &cells), before);
    }
}
