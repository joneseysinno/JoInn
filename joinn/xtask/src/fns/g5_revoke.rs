//! Gate 5 item 7: grant, units fires; revoke, the next delivery does not.

use super::{int_val, load_phase5_bodies, load_universe_file, text_val};
use joinn_frame::Verdict;
use joinn_host::describe;
use joinn_link::{
    bind_bodies, grant, revoke, Address, LinkRefusalKind, UniverseReport, UniverseState,
};

pub(crate) fn g5_revoke() -> bool {
    let Ok(u) = load_universe_file("phase5/universe.universe") else {
        return false;
    };
    let Ok(supplied) = load_phase5_bodies() else {
        return false;
    };
    let Verdict::Ok(bound) = bind_bodies(&u, &supplied) else {
        return false;
    };
    let mut state = match UniverseState::new(&u, bound, joinn_prim::sealed_natives()) {
        Verdict::Ok(s) => s,
        Verdict::Refused(_) => return false,
    };
    if !matches!(grant(state.link_runtime(), &u, "e0", "units"), Verdict::Ok(())) {
        return false;
    }
    let Ok(two) = text_val("2") else {
        return false;
    };
    let Ok(three) = text_val("3") else {
        return false;
    };
    let Ok(twelve) = int_val(12) else {
        return false;
    };
    let Ok(four) = text_val("4") else {
        return false;
    };
    let Ok(five) = text_val("5") else {
        return false;
    };
    let Ok(again) = int_val(12) else {
        return false;
    };
    let first_feed = [
        ("calc", "cli_a", 0u32, two),
        ("calc", "cli_b", 0, three),
        ("units", "scale", 1, twelve),
    ];
    for (i, (body, instance, port, value)) in first_feed.into_iter().enumerate() {
        let addr = Address {
            instance: instance.into(),
            port,
        };
        if !matches!(state.inject(body, &addr, value, i as u64), Verdict::Ok(())) {
            return false;
        }
    }
    let first = match state.run() {
        Verdict::Ok(r) => r,
        Verdict::Refused(_) => return false,
    };
    let fired = first
        .iter()
        .filter(|r| matches!(r, UniverseReport::Fired { body, .. } if body == "units"))
        .count();
    if fired != 1 {
        return false;
    }
    let before = state.body("units").and_then(|body| match describe(body, "scale") {
        Verdict::Ok(d) => d
            .ports
            .iter()
            .find(|p| p.position == 2)
            .and_then(|p| p.value.clone()),
        Verdict::Refused(_) => None,
    });
    if !matches!(revoke(state.link_runtime(), "e0", "units"), Verdict::Ok(())) {
        return false;
    }
    let second_feed = [
        ("calc", "cli_a", 0u32, four),
        ("calc", "cli_b", 0, five),
        ("units", "scale", 1, again),
    ];
    for (i, (body, instance, port, value)) in second_feed.into_iter().enumerate() {
        let addr = Address {
            instance: instance.into(),
            port,
        };
        if !matches!(state.inject(body, &addr, value, i as u64), Verdict::Ok(())) {
            return false;
        }
    }
    let second = match state.run() {
        Verdict::Ok(r) => r,
        Verdict::Refused(_) => return false,
    };
    let later = second
        .iter()
        .filter(|r| matches!(r, UniverseReport::Fired { body, .. } if body == "units"))
        .count();
    let after = state.body("units").and_then(|body| match describe(body, "scale") {
        Verdict::Ok(d) => d
            .ports
            .iter()
            .find(|p| p.position == 2)
            .and_then(|p| p.value.clone()),
        Verdict::Refused(_) => None,
    });
    later == 0
        && after == before
        && second.iter().any(|r| {
            matches!(
                r,
                UniverseReport::Link(refusal)
                    if refusal.link == "e0" && refusal.kind == LinkRefusalKind::CapabilityNotHeld
            )
        })
}
