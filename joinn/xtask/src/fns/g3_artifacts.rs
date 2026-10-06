//! Gate 3 item 8: every live control artifact resolves, and the fixture does not.

use super::legacy_tables::legacy_tables;
use super::opposed_tables::opposed_tables;
use super::{fixture_line, resolve_named};

pub(crate) fn g3_artifacts() -> bool {
    for (_, table) in legacy_tables() {
        for (i, item) in table.iter().enumerate() {
            if let Err(msg) = resolve_named(i + 1, item.name, item.control_artifact) {
                println!("{msg}");
                return false;
            }
        }
    }
    for (_, table) in opposed_tables() {
        for (i, item) in table.iter().enumerate() {
            if let Err(msg) = resolve_named(i + 1, item.name, item.control_artifact) {
                println!("{msg}");
                return false;
            }
        }
    }
    let Ok((name, rel)) = fixture_line() else {
        return false;
    };
    match resolve_named(1, &name, &rel) {
        Err(msg) => msg.contains(&name),
        Ok(_) => false,
    }
}
