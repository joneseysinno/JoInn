//! Four opposition fixtures the harness must get right before any table.

use super::check_ok::check_ok;
use super::honest_crossing::honest_crossing;
use super::ignores_subject::ignores_subject;
use super::oversensitive::oversensitive;
use crate::fns::grade_opposed::grade_opposed;
use crate::fns::mutate::Mutation;
use joinn_gate::GateItem;

const UNIVERSE: &str = "corpus/phase5/universe.universe";
const MISSING: &str = "no_such_link";

/// Refuse ignoring / oversensitive / missing-target; admit an honest control.
pub(super) fn check_opposing_fixtures() -> Result<(), String> {
    let ignores = GateItem {
        name: "ignores its subject",
        check: check_ok,
        control: ignores_subject,
        control_artifact: UNIVERSE,
        opposes: Mutation::DropLink("e0"),
    };
    match grade_opposed(1, &ignores) {
        Err(msg) if msg.contains("ignores its subject") => {}
        Ok(()) => {
            return Err(
                "harness fixture 1: a control that ignores its subject must be refused".into(),
            );
        }
        Err(msg) => {
            return Err(format!(
                "harness fixture 1: expected refuse naming ignores its subject, got {msg}"
            ));
        }
    }

    let any_change = GateItem {
        name: "true on any change",
        check: check_ok,
        control: oversensitive,
        control_artifact: UNIVERSE,
        opposes: Mutation::DropLink("e0"),
    };
    match grade_opposed(2, &any_change) {
        Err(msg) if msg.contains("true on any change") && msg.contains("neutral") => {}
        Ok(()) => {
            return Err(
                "harness fixture 2: a control that answers true on the neutral edit must be refused"
                    .into(),
            );
        }
        Err(msg) => {
            return Err(format!(
                "harness fixture 2: expected refuse on neutral edit, got {msg}"
            ));
        }
    }

    let missing = GateItem {
        name: "missing mutation target",
        check: check_ok,
        control: honest_crossing,
        control_artifact: UNIVERSE,
        opposes: Mutation::DropLink(MISSING),
    };
    match grade_opposed(3, &missing) {
        Err(msg) if msg.contains(MISSING) => {}
        Ok(()) => {
            return Err(format!(
                "harness fixture 3: a missing mutation target must be refused naming {MISSING}"
            ));
        }
        Err(msg) => {
            return Err(format!(
                "harness fixture 3: expected refuse naming {MISSING}, got {msg}"
            ));
        }
    }

    let honest = GateItem {
        name: "honest crossing",
        check: check_ok,
        control: honest_crossing,
        control_artifact: UNIVERSE,
        opposes: Mutation::DropLink("e0"),
    };
    match grade_opposed(4, &honest) {
        Ok(()) => {}
        Err(msg) => {
            return Err(format!(
                "harness fixture 4: an honest control must be admitted, got {msg}"
            ));
        }
    }

    Ok(())
}
