//! CLI host. stdin/stdout. A description is rendered here, never built here.

#![forbid(unsafe_code)]

use joinn_cli::session_run;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let cmd = args.next().unwrap_or_else(|| "help".into());
    let target = args.next();
    let result = match (cmd.as_str(), target.as_deref()) {
        ("run", Some(name)) => session_run(name),
        _ => {
            eprintln!("usage: joinn run <body>");
            Err("usage".into())
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            if e != "usage" {
                eprintln!("{e}");
            }
            ExitCode::FAILURE
        }
    }
}
