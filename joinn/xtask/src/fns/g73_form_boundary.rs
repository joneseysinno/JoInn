//! Gate 7.3 item 2: the form boundaries on the GPU, against the CPU (§2.12 F2).

use joinn_frame::Verdict;
use joinn_gpu::{Gpu, link_probe};
use joinn_visual::{Fold, Form, Route, Zoom, form_of};

/// `(size, level, step)` and the owner form there: `10·s = 11·T` exactly,
/// then a notch below, for sizes 48 and 33, at T = 240 and T = 1920.
const PROBES: [((u32, i32, u32), Form); 8] = [
    ((48, 2, 96), Form::Hub),
    ((48, 2, 95), Form::Region),
    ((33, 3, 0), Form::Hub),
    ((33, 2, 255), Form::Region),
    ((48, 5, 96), Form::Bundle),
    ((48, 5, 95), Form::Hub),
    ((33, 6, 0), Form::Bundle),
    ((33, 5, 255), Form::Hub),
];

/// The `form boundary` line for one adapter, and one failure per probe where
/// the shader's `form_owner`, or `form_of` on the CPU, is not the form §2.5
/// gives there.
pub(crate) fn g73_form_boundary(gpu: &Gpu, adapter: &str) -> Result<(String, Vec<String>), String> {
    let at: Vec<(u32, i32, u32)> = PROBES.iter().map(|(p, _)| *p).collect();
    let got = match link_probe(gpu, &at) {
        Verdict::Ok(g) => g,
        Verdict::Refused(r) => return Err(r.reason),
    };
    if got.len() != PROBES.len() {
        return Err(format!(
            "form boundary {adapter}: {} answers; acceptance is {}",
            got.len(),
            PROBES.len()
        ));
    }
    let forms = [Form::Region, Form::Hub, Form::Bundle, Form::Spine];
    let name = |n: u32| forms.get(n as usize).map_or("no form", |f| f.name());
    let mut failures = Vec::new();
    let mut said = Vec::new();
    for (((size, level, step), want), (_, gpu_form)) in PROBES.iter().zip(&got) {
        let route = Route {
            link: 0,
            fold: Fold::Open,
            ordered: false,
            touches: Vec::new(),
            knot: None,
            legs: 0,
            stubs: 0,
            size: i64::from(*size),
            pieces: Vec::new(),
        };
        let cpu = form_of(
            Zoom {
                level: *level,
                step: *step,
            },
            &route,
        )
        .0;
        if *gpu_form != want.number() || cpu != *want {
            failures.push(format!(
                "form boundary {adapter}: size {size} at level {level} step {step}: gpu {}, cpu {}; acceptance is {}",
                name(*gpu_form),
                cpu.name(),
                want.name()
            ));
        }
        said.push(format!(
            "{size} at level {level} step {step} {}",
            name(*gpu_form)
        ));
    }
    let verdict = if failures.is_empty() {
        "cpu agrees"
    } else {
        "cpu differs"
    };
    let (low, high) = said.split_at(said.len().min(4));
    let line = format!(
        "form boundary {adapter}: 240: {}; 1920: {}; {verdict}",
        low.join(", "),
        high.join(", ")
    );
    Ok((line, failures))
}
