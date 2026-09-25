//! Run a named body against stdin.

use std::io::{self, BufRead};

/// Run a named body against stdin and print what the host presented.
pub fn run_named(name: &str) -> Result<(), String> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    if name == "universe" {
        let (text, _) = super::run_universe::run_universe(&mut lines)?;
        print!("{text}");
        return Ok(());
    }
    let (text, _) = super::run_session::run_session(name, &mut lines)?;
    print!("{text}");
    Ok(())
}
