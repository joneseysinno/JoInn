//! The 240 px band boundary, on the GPU, against the CPU (plan 7.3 §2.12 F2).

use joinn_frame::Verdict;
use joinn_gpu::{Gpu, band_probe};
use joinn_visual::{Band, Zoom, owner_band};

/// `(size, level, step)`: `10·s = 11·240` exactly, then a notch below, for
/// sizes 48 and 33 (the only kind of size that reaches it).
const PROBES: [(u32, i32, u32); 4] = [(48, 2, 96), (48, 2, 95), (33, 3, 0), (33, 2, 255)];

/// The `band boundary` line for one adapter, and one failure per probe whose
/// GPU band differs from `owner_band`'s, or per boundary whose band is not
/// above the band a notch below it.
pub(crate) fn band_boundary(gpu: &Gpu, adapter: &str) -> Result<(String, Vec<String>), String> {
    let name = |b: u32| match b {
        0 => "dot",
        1 => "glyph",
        2 => "summary",
        3 => "full",
        _ => "no band",
    };
    let cpu: Vec<u32> = PROBES
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
    let got = match band_probe(gpu, &PROBES) {
        Verdict::Ok(g) => g,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let mut failures = Vec::new();
    for ((&(size, level, step), g), c) in PROBES.iter().zip(&got).zip(&cpu) {
        if g != c {
            failures.push(format!(
                "band boundary {adapter}: size {size} at level {level} step {step}: gpu {}, cpu {}; acceptance is the cpu's band",
                name(*g),
                name(*c)
            ));
        }
    }
    for pair in got.chunks(2).zip(PROBES.chunks(2)) {
        if let ([at, below], [(size, level, step), _]) = pair {
            if at <= below {
                failures.push(format!(
                    "band boundary {adapter}: size {size} at level {level} step {step} is {}, not above {} a notch below",
                    name(*at),
                    name(*below)
                ));
            }
        }
    }
    let [(s0, l0, t0), (_, _, t1), (s2, l2, t2), (_, l3, t3)] = PROBES;
    let band = |i: usize| got.get(i).map_or("no band", |b| name(*b));
    let verdict = if failures.is_empty() {
        "cpu agrees"
    } else {
        "cpu differs"
    };
    let line = format!(
        "band boundary {adapter}: {s0} at level {l0} step {t0} {}, step {t1} {}; {s2} at level {l2} step {t2} {}, level {l3} step {t3} {}; {verdict}",
        band(0),
        band(1),
        band(2),
        band(3)
    );
    Ok((line, failures))
}
