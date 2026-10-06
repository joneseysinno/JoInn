//! The shader's own `band()`, evaluated on the GPU at chosen sizes and zooms.

use joinn_frame::Verdict;

use super::probe::probe;
use crate::gpu::Gpu;

/// For each `(size, level, step)`, the band `common.wgsl`'s `band(size)`
/// returns under a tick of that zoom (0 dot, 1 glyph, 2 summary, 3 full):
/// one draw into a 1×1 `R32Uint` target per probe, read back as bytes.
pub fn band_probe(gpu: &Gpu, probes: &[(u32, i32, u32)]) -> Verdict<Vec<u32>> {
    probe(gpu, 0, probes)
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use joinn_visual::{Band, Zoom, owner_band};

    use super::band_probe;
    use crate::adapter::adapters;
    use crate::gpu::open;

    #[test]
    fn the_gpu_band_meets_the_cpu_band_at_the_240_px_boundary() {
        let probes = [(48, 2, 96), (48, 2, 95), (33, 3, 0), (33, 2, 255)];
        let all = match adapters() {
            Verdict::Ok(a) => a,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let cpu: Vec<u32> = probes
            .iter()
            .map(
                |&(size, level, step)| match owner_band(Zoom { level, step }, i64::from(size)) {
                    Band::Dot => 0,
                    Band::Glyph => 1,
                    Band::Summary => 2,
                    Band::Full => 3,
                },
            )
            .collect();
        assert_eq!(cpu, [3, 2, 3, 2]);
        for adapter in &all {
            let gpu = match open(adapter) {
                Verdict::Ok(g) => g,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            match band_probe(&gpu, &probes) {
                Verdict::Ok(gpu_bands) => assert_eq!(gpu_bands, cpu, "{}", adapter.line()),
                Verdict::Refused(r) => panic!("{}: {}", adapter.line(), r.reason),
            }
        }
    }
}
