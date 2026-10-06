//! The adapter list every gate 6 GPU check runs on. Empty fails the item.

use std::sync::OnceLock;

use joinn_frame::Verdict;
use joinn_gpu::{GpuAdapter, adapters};

use super::once::{memo, say};

const REFUSAL: &str = "no GPU adapter; acceptance is at least one adapter (on Linux, install mesa-vulkan-drivers for lavapipe)";

type Listed = Result<Vec<GpuAdapter>, String>;

static LISTED: OnceLock<(Listed, Vec<String>)> = OnceLock::new();

/// Every adapter, or `None` after printing the refusal. An empty `Ok` is the
/// same refusal: the item fails, it does not skip. The adapters are listed
/// once per process.
pub(crate) fn g6_adapters() -> Option<Vec<GpuAdapter>> {
    let listed = memo(&LISTED, || match adapters() {
        Verdict::Ok(list) => Ok(list),
        Verdict::Refused(r) => Err(r.reason),
    });
    match listed {
        Ok(list) if !list.is_empty() => Some(list),
        Ok(_) => {
            say(REFUSAL);
            None
        }
        Err(reason) => {
            say(&reason);
            None
        }
    }
}
