//! Gate 3 item 2: both hosts' sum descriptions match the golden.

use super::{first_desc_diff, load_calculator, text_val, workspace_root};
use joinn_frame::{Term, Verdict};
use joinn_host::{Address, describe, print_description};
use joinn_live::BodyState;
use joinn_test_host::RawEvent;

pub(crate) fn g3_descriptions() -> bool {
    let Ok(root) = workspace_root() else {
        return false;
    };
    let Ok(golden) = std::fs::read_to_string(
        root.join("corpus")
            .join("descriptions")
            .join("calculator.desc"),
    ) else {
        return false;
    };
    let golden = golden.replace("\r\n", "\n");
    let Ok(planted) = std::fs::read_to_string(
        root.join("corpus")
            .join("phase3")
            .join("controls")
            .join("planted_desc.desc"),
    ) else {
        return false;
    };
    let Some(field) = first_desc_diff(&golden, &planted.replace("\r\n", "\n")) else {
        return false;
    };
    let Ok((body, cells)) = load_calculator() else {
        return false;
    };
    let natives = joinn_prim::sealed_natives();
    let events = vec![
        RawEvent {
            address: Address {
                instance: "cli_a".into(),
                port: 0,
            },
            term: Term::text("2"),
        },
        RawEvent {
            address: Address {
                instance: "cli_b".into(),
                port: 0,
            },
            term: Term::text("3"),
        },
    ];
    let Verdict::Ok(cap) = joinn_test_host::run(body.clone(), cells.clone(), natives.clone(), events)
    else {
        return false;
    };
    let Some(test_txt) = cap
        .descriptions
        .iter()
        .find(|d| d.instance == "sum")
        .map(print_description)
    else {
        return false;
    };
    let Ok(two) = text_val("2") else {
        return false;
    };
    let Ok(three) = text_val("3") else {
        return false;
    };
    let Verdict::Ok(mut state) = BodyState::new(body, cells, natives, 1) else {
        return false;
    };
    if !matches!(state.inject("cli_a", 0, two, 0), Verdict::Ok(())) {
        return false;
    }
    if !matches!(state.inject("cli_b", 0, three, 1), Verdict::Ok(())) {
        return false;
    }
    if !matches!(state.run(), Verdict::Ok(_)) {
        return false;
    }
    let Verdict::Ok(cli_d) = describe(&state, "sum") else {
        return false;
    };
    let cli_txt = print_description(&cli_d);
    if test_txt == golden && cli_txt == golden {
        println!("differing field: {field}");
        true
    } else {
        false
    }
}
