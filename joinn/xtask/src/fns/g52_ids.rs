//! Gate 5.2 item 3: host output does not depend on link ids or aliases.

use super::mutate::{Mutation, mutate};
use super::subject::Subject;
use super::{load_phase5_bodies, run_body_bin, workspace_root};
use joinn_cli::present_universe;
use joinn_frame::Verdict;
use joinn_link::parse_universe;
use std::fs;

pub(crate) fn g52_ids() -> bool {
    let Ok(cli) = run_body_bin("universe", b"two\n2\n3\n12\n") else {
        return false;
    };
    let Ok(root) = workspace_root() else {
        return false;
    };
    let Ok(want) = fs::read_to_string(root.join("corpus").join("transcripts").join("universe.txt"))
    else {
        return false;
    };
    if cli != want {
        return false;
    }
    if cli.contains("e0") {
        return false;
    }
    let Ok(src) = fs::read_to_string(root.join("corpus").join("phase5").join("universe.universe"))
    else {
        return false;
    };
    let Verdict::Ok(universe) = parse_universe(&src) else {
        return false;
    };
    let Ok(store) = load_phase5_bodies() else {
        return false;
    };
    let subject = Subject::Universe(universe.clone());
    let baseline = {
        let lines = ["two", "2", "3", "12"].map(|s| Ok(s.to_owned()));
        match present_universe(&universe, &store, &mut lines.into_iter()) {
            Ok(s) => s,
            Err(_) => return false,
        }
    };
    let Verdict::Ok(link_mutant) = mutate(&subject, &Mutation::RenameLink("e0", "e1")) else {
        return false;
    };
    let Subject::Universe(renamed_link) = link_mutant else {
        return false;
    };
    let after_link = {
        let lines = ["two", "2", "3", "12"].map(|s| Ok(s.to_owned()));
        match present_universe(&renamed_link, &store, &mut lines.into_iter()) {
            Ok(s) => s,
            Err(_) => return false,
        }
    };
    if after_link != baseline {
        return false;
    }
    let Verdict::Ok(alias_mutant) = mutate(&subject, &Mutation::RenameAlias("units", "meters"))
    else {
        return false;
    };
    let Subject::Universe(renamed_alias) = alias_mutant else {
        return false;
    };
    let after_alias = {
        let lines = ["two", "2", "3", "12"].map(|s| Ok(s.to_owned()));
        match present_universe(&renamed_alias, &store, &mut lines.into_iter()) {
            Ok(s) => s,
            Err(_) => return false,
        }
    };
    after_alias == baseline
}
