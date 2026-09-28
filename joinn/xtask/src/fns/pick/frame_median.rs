//! Frame time, as information only (rule 4): no check reads it.

use std::time::Instant;

use joinn_frame::Verdict;
use joinn_gpu::{Gpu, Renderer};
use joinn_visual::Camera;

const WARMUP: usize = 5;
const FRAMES: usize = 31;

/// The median of `FRAMES` offscreen frames after `WARMUP`, each drawn and
/// finished on the GPU, printed as milliseconds with three decimals.
// Rule 4: xtask MAY measure wall time.
#[allow(clippy::disallowed_methods)]
pub(super) fn frame_median(
    renderer: &mut Renderer,
    gpu: &Gpu,
    camera: &Camera,
) -> Result<String, String> {
    let mut micros: Vec<u128> = Vec::with_capacity(FRAMES);
    for i in 0..WARMUP + FRAMES {
        let start = Instant::now();
        if let Verdict::Refused(r) = renderer.frame(gpu, camera) {
            return Err(r.reason);
        }
        if i >= WARMUP {
            micros.push(start.elapsed().as_micros());
        }
    }
    micros.sort_unstable();
    let median = micros.get(FRAMES / 2).copied().unwrap_or(0);
    Ok(format!("{}.{:03} ms", median / 1000, median % 1000))
}
