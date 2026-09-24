//! Gate 5 item 3: every phase-5 universe assembles; missing ports are named.

use super::{load_phase5_bodies, load_universe_file, workspace_root};
use joinn_frame::Verdict;
use joinn_link::{assemble_universe, bind_bodies};
use std::fs;

pub(crate) fn g5_assemble() -> bool {
    let Ok(root) = workspace_root() else {
        return false;
    };
    let Ok(supplied) = load_phase5_bodies() else {
        return false;
    };
    let dir = root.join("corpus").join("phase5");
    let Ok(entries) = fs::read_dir(&dir) else {
        return false;
    };
    let mut ents: Vec<_> = entries.flatten().collect();
    ents.sort_by_key(|e| e.file_name());
    for ent in ents {
        let path = ent.path();
        if path.extension().and_then(|e| e.to_str()) != Some("universe") {
            continue;
        }
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            return false;
        };
        let Ok(u) = load_universe_file(&format!("phase5/{name}")) else {
            return false;
        };
        let bound = match bind_bodies(&u, &supplied) {
            Verdict::Ok(b) => b,
            Verdict::Refused(_) => return false,
        };
        match assemble_universe(&u, &bound) {
            Verdict::Ok(()) => {}
            Verdict::Refused(_) => return false,
        }
    }
    let Ok(transit) = load_universe_file("phase5/controls/transits.universe") else {
        return false;
    };
    let Ok(bound) = (match bind_bodies(&transit, &supplied) {
        Verdict::Ok(b) => Ok(b),
        Verdict::Refused(_) => Err(()),
    }) else {
        return false;
    };
    match assemble_universe(&transit, &bound) {
        Verdict::Refused(r) => {
            r.reason.contains("interior") && r.reason.contains("cli_a@1 -> sum@0")
        }
        Verdict::Ok(()) => false,
    }
}
