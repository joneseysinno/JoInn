//! The shader's own fold state and form, evaluated on the GPU.

use joinn_frame::Verdict;

use super::probe::probe;
use crate::gpu::Gpu;

/// For each `(size, level, step)`: the fold state `fold_now()` gives that
/// zoom (0 open, 1 systems folded, 2 galaxies folded) and the form
/// `form_owner(size)` gives an unordered link of that size (0 region, 1 hub,
/// 2 bundle), each one draw into a 1×1 `R32Uint` target.
pub fn link_probe(gpu: &Gpu, probes: &[(u32, i32, u32)]) -> Verdict<Vec<(u32, u32)>> {
    let folds = match probe(gpu, 2, probes) {
        Verdict::Ok(f) => f,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    match probe(gpu, 1, probes) {
        Verdict::Ok(forms) => Verdict::Ok(folds.into_iter().zip(forms).collect()),
        Verdict::Refused(r) => Verdict::Refused(r),
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use joinn_visual::{Fold, Route, Zoom, fold_at, form_of};

    use super::link_probe;
    use crate::adapter::adapters;
    use crate::gpu::open;

    #[test]
    fn the_gpu_fold_and_form_meet_the_cpu_at_both_form_boundaries_and_the_fold() {
        let probes = [
            (33, 3, 0),
            (33, 2, 255),
            (33, 6, 0),
            (33, 5, 255),
            (1296, -4, 0),
            (40, -4, 219),
            (40, -4, 218),
        ];
        let cpu: Vec<(u32, u32)> = probes
            .iter()
            .map(|&(size, level, step)| {
                let zoom = Zoom { level, step };
                let route = Route {
                    link: 0,
                    fold: Fold::Open,
                    ordered: false,
                    touches: Vec::new(),
                    knot: None,
                    legs: 0,
                    stubs: 0,
                    size: i64::from(size),
                    pieces: Vec::new(),
                };
                (fold_at(zoom).number(), form_of(zoom, &route).0.number())
            })
            .collect();
        // A system (304) opens at level −4 when 10·304·(256 + step) ≥
        // 11·32·4096: from step 219.
        assert_eq!(
            cpu,
            [(0, 1), (0, 0), (0, 2), (0, 1), (1, 0), (0, 0), (1, 0)]
        );
        let all = match adapters() {
            Verdict::Ok(a) => a,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        for adapter in &all {
            let gpu = match open(adapter) {
                Verdict::Ok(g) => g,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            match link_probe(&gpu, &probes) {
                Verdict::Ok(got) => assert_eq!(got, cpu, "{}", adapter.line()),
                Verdict::Refused(r) => panic!("{}: {}", adapter.line(), r.reason),
            }
        }
    }
}
