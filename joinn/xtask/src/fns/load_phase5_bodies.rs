//! Load phase-5 bodies into a store.

use super::workspace_root;
use joinn_dna::{Body, Cell, hash, parse_body, parse_cell};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use joinn_link::BodyStore;
use std::collections::BTreeMap;
use std::fs;

pub(crate) fn load_phase5_bodies() -> Result<BodyStore, String> {
    let root = workspace_root()?;
    let frames = FrameRegistry::phase1();
    let mut cells: BTreeMap<Hash, Cell> = BTreeMap::new();
    for dir in ["phase0", "phase21", "phase22", "phase2"] {
        let path = root.join("corpus").join(dir);
        let rd = match fs::read_dir(&path) {
            Ok(rd) => rd,
            Err(_) => continue,
        };
        let mut ents: Vec<_> = rd.flatten().collect();
        ents.sort_by_key(|e| e.file_name());
        for ent in ents {
            let file = ent.path();
            if file.extension().and_then(|e| e.to_str()) != Some("cell") {
                continue;
            }
            let src = fs::read_to_string(&file).map_err(|e| e.to_string())?;
            let Verdict::Ok(cell) = parse_cell(&src, &frames) else {
                continue;
            };
            let id = hash(&cell.coding);
            let replace = match cells.get(&id) {
                Some(prev) => prev.alleles.is_empty() && !cell.alleles.is_empty(),
                None => true,
            };
            if replace {
                cells.insert(id, cell);
            }
        }
    }
    let load = |rel: &str| -> Result<Body, String> {
        let src = fs::read_to_string(root.join("corpus").join(rel)).map_err(|e| e.to_string())?;
        match parse_body(&src, &frames) {
            Verdict::Ok(b) => Ok(b),
            Verdict::Refused(r) => Err(r.reason),
        }
    };
    let mut store = BodyStore::new();
    for rel in [
        "phase2/calculator.body",
        "phase5/units.body",
        "phase5/controls/echo.body",
        "phase5/bus.body",
    ] {
        let body = load(rel)?;
        let path = root.join("corpus").join(rel);
        let source = path.display().to_string();
        match store.insert(body, cells.clone(), &source) {
            Verdict::Ok(_) => {}
            Verdict::Refused(r) => return Err(r.reason),
        }
    }
    Ok(store)
}
