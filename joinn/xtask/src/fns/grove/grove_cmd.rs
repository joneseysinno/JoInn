//! `cargo xtask grove [--seed N] [--out PATH]`.

use joinn_link::print_universe;
use std::env;
use std::fs;

use super::{GROVE_SEED, grove_line, grow_grove, out_path};
use crate::fns::corpus_store::corpus_store;
use crate::fns::workspace_root;

/// Print the grove's line. With `--out`, write its canonical text there (never
/// under corpus/ or docs/). Exit 1 when it is not admitted.
pub(crate) fn grove(args: Vec<String>) -> Result<(), String> {
    let mut seed = GROVE_SEED;
    let mut out = None;
    let mut it = args.into_iter();
    while let Some(arg) = it.next() {
        match (arg.as_str(), it.next()) {
            ("--seed", Some(n)) => {
                seed = n
                    .parse()
                    .map_err(|_| format!("grove: --seed {n} is not a whole number"))?;
            }
            ("--out", Some(p)) => out = Some(p),
            _ => return Err("usage: cargo xtask grove [--seed N] [--out PATH]".into()),
        }
    }
    let universe = grow_grove(seed)?;
    let (_, store) = corpus_store()?;
    let (line, admitted) = grove_line(seed, &universe, store);
    println!("{line}");
    if let Some(p) = out {
        let cwd = env::current_dir().map_err(|e| e.to_string())?;
        let path = out_path(&p, &cwd, &workspace_root()?)?;
        fs::write(&path, print_universe(&universe.coding).as_bytes())
            .map_err(|e| format!("{}: {e}", path.display()))?;
    }
    if admitted {
        Ok(())
    } else {
        Err("grove: not admitted".into())
    }
}
