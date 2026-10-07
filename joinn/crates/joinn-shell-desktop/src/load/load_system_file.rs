//! Parse the `.system` file the shell was given and the contacts beside it.

use joinn_dna::{Contact, System, hash, parse_contact, parse_system};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// The system, every `.contact` in its folder that parses (keyed by hash, in
/// path order), and how many waiting boxes its body shows. The body it binds
/// must be one of them.
pub(crate) fn load_system_file(
    path: &Path,
) -> Result<(System, BTreeMap<Hash, Contact>, u32), String> {
    let src = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let system = match parse_system(&src) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let dir = path.parent().unwrap_or(Path::new("."));
    let mut paths = Vec::new();
    for ent in fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))? {
        let p = ent.map_err(|e| e.to_string())?.path();
        if p.extension().is_some_and(|e| e == "contact") {
            paths.push(p);
        }
    }
    paths.sort();
    let frames = FrameRegistry::phase1();
    let mut contacts = BTreeMap::new();
    for p in &paths {
        let text = fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()))?;
        if let Verdict::Ok(c) = parse_contact(&text, &frames) {
            contacts.entry(hash(&c.coding)).or_insert(c);
        }
    }
    let Some(body) = system.coding.bodies.first() else {
        return Err("open: the system binds no body; acceptance is one growing body".to_owned());
    };
    if !contacts.contains_key(&body.contact) {
        return Err(format!(
            "open: {}'s body {} {} is not beside it; acceptance is its .contact in the same folder",
            path.display(),
            body.alias,
            body.contact
        ));
    }
    let waiting = system
        .regulatory
        .waiting
        .get(&body.alias)
        .copied()
        .unwrap_or(0);
    Ok((system, contacts, waiting))
}
