//! Gate 3 item 8: every live control artifact resolves, and the fixture does not.

use super::{
    fixture_line, gate_five_items, gate_five_one_items, gate_one_items, gate_three_items,
    gate_two_items, gate_two_one_items, gate_two_two_items, resolve_named,
};

pub(crate) fn g3_artifacts() -> bool {
    let legacy = [
        gate_one_items(),
        gate_two_items(),
        gate_two_one_items(),
        gate_two_two_items(),
        gate_three_items(),
    ];
    for table in legacy {
        for (i, item) in table.iter().enumerate() {
            if let Err(msg) = resolve_named(i + 1, item.name, item.control_artifact) {
                println!("{msg}");
                return false;
            }
        }
    }
    let non_legacy = [gate_five_items(), gate_five_one_items()];
    for table in non_legacy {
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
