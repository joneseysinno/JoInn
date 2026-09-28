//! `cargo xtask adapters`: every adapter a GPU check will run on.

use joinn_frame::Verdict;

/// One line per adapter, then `adapters: <n>`. An empty list fails.
pub(crate) fn adapters() -> Result<(), String> {
    match joinn_gpu::adapters() {
        Verdict::Ok(all) => {
            for a in &all {
                println!("{}", a.line());
            }
            println!("adapters: {}", all.len());
            Ok(())
        }
        Verdict::Refused(r) => Err(r.reason),
    }
}
