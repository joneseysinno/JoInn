//! Gate 5.2 item 3 control: true when the CLI output lacks ordered.universe's last line.

use super::subject::Subject;
use super::{load_phase5_bodies, load_universe_file};
use joinn_cli::present_universe;

pub(crate) fn g52_ids_control(subject: &Subject) -> bool {
    let Subject::Universe(universe) = subject else {
        return false;
    };
    let Ok(store) = load_phase5_bodies() else {
        return false;
    };
    let Ok(real) = load_universe_file("phase5/ordered.universe") else {
        return false;
    };
    let inputs = ["two", "2", "3", "12"];
    let real_lines = inputs.map(|s| Ok(s.to_owned()));
    let Ok(real_out) = present_universe(&real, &store, &mut real_lines.into_iter()) else {
        return false;
    };
    let Some(last) = real_out.lines().last() else {
        return false;
    };
    let last = last.to_owned();
    let lines = inputs.map(|s| Ok(s.to_owned()));
    let Ok(out) = present_universe(universe, &store, &mut lines.into_iter()) else {
        return false;
    };
    !out.lines().any(|line| line == last)
}
