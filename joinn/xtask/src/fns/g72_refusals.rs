//! Gate 7.2 item 1: every camera refusal of plan 7.2 §2.2, word for word.

use joinn_frame::Verdict;
use joinn_visual::{Camera, ChartId, LEVEL_MAX, LEVEL_MIN, Zoom};

/// One line per refusal that didn't happen or whose words differ: levels −5
/// and 10, step 256, whole `k` 0 and 512, and a notch past either end of the
/// level range.
pub(crate) fn g72_refusals() -> Vec<String> {
    let level =
        |n: i32| format!("zoom: level {n} is outside −4 … 9; acceptance is a level from −4 to 9");
    let whole = |k: i64| {
        format!("zoom: k {k} has no level and step; acceptance is a whole k from 1 to 511")
    };
    let at = |level: i32, step: u32| Camera {
        zoom: Zoom { level, step },
        anchor: ChartId(0),
        focus: (0, 0),
        pin: (0, 0),
        width: 1920,
        height: 1080,
    };
    let cases: Vec<(String, Verdict<Zoom>, String)> = vec![
        ("Zoom::new(-5, 0)".into(), Zoom::new(-5, 0), level(-5)),
        ("Zoom::new(10, 0)".into(), Zoom::new(10, 0), level(10)),
        (
            "Zoom::new(0, 256)".into(),
            Zoom::new(0, 256),
            "zoom: step 256 is outside 0 … 255; acceptance is a step from 0 to 255".into(),
        ),
        (
            "Zoom::from_whole_k(0)".into(),
            Zoom::from_whole_k(0),
            whole(0),
        ),
        (
            "Zoom::from_whole_k(512)".into(),
            Zoom::from_whole_k(512),
            whole(512),
        ),
        (
            "one notch in at level 9 step 224".into(),
            at(LEVEL_MAX, 224).zoom_about((0, 0), 1).map(|c| c.zoom),
            level(10),
        ),
        (
            "one notch out at level −4 step 0".into(),
            at(LEVEL_MIN, 0).zoom_about((0, 0), -1).map(|c| c.zoom),
            level(-5),
        ),
    ];
    cases
        .into_iter()
        .filter_map(|(what, got, want)| match got {
            Verdict::Refused(r) if r.reason == want => None,
            Verdict::Refused(r) => Some(format!(
                "{what}: refused as {:?}; acceptance is {want:?}",
                r.reason
            )),
            Verdict::Ok(z) => Some(format!(
                "{what}: gave {z:?}; acceptance is the refusal {want:?}"
            )),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::g72_refusals;

    #[test]
    fn every_refusal_has_the_plan_s_words() {
        assert_eq!(g72_refusals(), Vec::<String>::new());
    }
}
