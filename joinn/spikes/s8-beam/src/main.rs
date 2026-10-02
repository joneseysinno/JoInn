//! S8 beam spike (Phase 7.1 stage 1): five beams worked exactly in ℚ, with
//! formulas derived and libraries as witnesses. No float, no randomness, and
//! every refusal is a value.

#![forbid(unsafe_code)]

mod beam;
mod bridge;
mod plants;
mod q;
mod run;
mod show;
mod tag;
mod verdict;

fn main() -> std::process::ExitCode {
    match run::run() {
        Ok(run) => {
            print!("{}", run.text());
            if run.clean() {
                std::process::ExitCode::SUCCESS
            } else {
                std::process::ExitCode::FAILURE
            }
        }
        Err(host) => {
            eprintln!("s8: {host}");
            std::process::ExitCode::FAILURE
        }
    }
}
