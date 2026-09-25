//! Load the corpus universe and present it.

use crate::load::{find_corpus, gather_bodies, load_cells};
use joinn_frame::Verdict;
use joinn_link::parse_universe;
use std::fs;
use std::io;

use super::present_universe::present_universe;

/// Prompt unlinked in-ports in alias order, then present what fired or refused.
pub(in crate::session) fn run_universe(
    lines: &mut impl Iterator<Item = io::Result<String>>,
) -> Result<(String, Vec<u8>), String> {
    let corpus = find_corpus()?;
    let cells = load_cells(&corpus)?;
    let store = gather_bodies(&corpus, &cells)?;
    let src = fs::read_to_string(corpus.join("phase5").join("universe.universe"))
        .map_err(|e| e.to_string())?;
    let universe = match parse_universe(&src) {
        Verdict::Ok(u) => u,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let out = present_universe(&universe, &store, lines)?;
    Ok((out, Vec::new()))
}

#[cfg(test)]
mod tests {
    use super::run_universe;
    use crate::load::{find_corpus, gather_bodies, load_cells};
    use crate::session::run_universe::present_universe;
    use joinn_frame::Verdict;
    use joinn_link::parse_universe;
    use std::fs;

    #[test]
    fn universe_transcript_matches_golden() {
        let lines = ["two", "2", "3", "12"].map(|s| Ok(s.to_owned()));
        let (out, _) = match run_universe(&mut lines.into_iter()) {
            Ok(v) => v,
            Err(e) => panic!("{e}"),
        };
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("corpus")
            .join("transcripts");
        let want = std::fs::read(root.join("universe.txt")).unwrap_or_else(|e| panic!("{e}"));
        let calc = std::fs::read(root.join("calculator.txt")).unwrap_or_else(|e| panic!("{e}"));
        let got = out.into_bytes();
        assert_eq!(got, want, "{}", String::from_utf8_lossy(&got));
        assert_eq!(&got[..calc.len()], calc.as_slice());
    }

    #[test]
    fn rename_link_and_alias_leave_output_unchanged() {
        let corpus = find_corpus().unwrap_or_else(|e| panic!("{e}"));
        let cells = load_cells(&corpus).unwrap_or_else(|e| panic!("{e}"));
        let store = gather_bodies(&corpus, &cells).unwrap_or_else(|e| panic!("{e}"));
        let src = fs::read_to_string(corpus.join("phase5").join("universe.universe"))
            .unwrap_or_else(|e| panic!("{e}"));
        let universe = match parse_universe(&src) {
            Verdict::Ok(u) => u,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let baseline = {
            let lines = ["two", "2", "3", "12"].map(|s| Ok(s.to_owned()));
            present_universe(&universe, &store, &mut lines.into_iter())
                .unwrap_or_else(|e| panic!("{e}"))
        };
        let mut renamed_link = universe.clone();
        for link in &mut renamed_link.coding.links {
            if link.id == "e0" {
                link.id = "e1".into();
            }
        }
        if let Some(v) = renamed_link.regulatory.names.remove("e0") {
            renamed_link.regulatory.names.insert("e1".into(), v);
        }
        if let Some(v) = renamed_link.regulatory.labels.remove("e0") {
            renamed_link.regulatory.labels.insert("e1".into(), v);
        }
        if let Some(body) = renamed_link.coding.grants.remove("e0") {
            renamed_link.coding.grants.insert("e1".into(), body);
        }
        let after_link = {
            let lines = ["two", "2", "3", "12"].map(|s| Ok(s.to_owned()));
            present_universe(&renamed_link, &store, &mut lines.into_iter())
                .unwrap_or_else(|e| panic!("{e}"))
        };
        assert_eq!(after_link, baseline);

        let mut renamed_alias = universe.clone();
        for binding in &mut renamed_alias.coding.bodies {
            if binding.alias == "units" {
                binding.alias = "meters".into();
            }
        }
        for link in &mut renamed_alias.coding.links {
            for member in &mut link.members {
                if member.body == "units" {
                    member.body = "meters".into();
                }
            }
        }
        for lens in &mut renamed_alias.coding.lenses {
            for galaxy in &mut lens.galaxies {
                for system in &mut galaxy.systems {
                    for body in &mut system.bodies {
                        if body == "units" {
                            *body = "meters".into();
                        }
                    }
                }
            }
        }
        for holder in renamed_alias.coding.grants.values_mut() {
            if holder == "units" {
                *holder = "meters".into();
            }
        }
        let after_alias = {
            let lines = ["two", "2", "3", "12"].map(|s| Ok(s.to_owned()));
            present_universe(&renamed_alias, &store, &mut lines.into_iter())
                .unwrap_or_else(|e| panic!("{e}"))
        };
        assert_eq!(after_alias, baseline);
    }
}
