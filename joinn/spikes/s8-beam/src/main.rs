//! S8 beam spike (Phase 7.1 stage 1): five beams worked exactly in ℚ, with
//! formulas derived and libraries as witnesses. No float, no randomness, and
//! every refusal is a value.

#![forbid(unsafe_code)]

mod bridge;
mod q;
mod run;
mod tag;
mod verdict;

fn main() -> std::process::ExitCode {
    let run = run::run();
    print!("{}", run.text());
    if run.clean() {
        std::process::ExitCode::SUCCESS
    } else {
        std::process::ExitCode::FAILURE
    }
}
