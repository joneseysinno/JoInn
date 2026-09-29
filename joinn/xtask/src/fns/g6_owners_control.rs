//! Gate 6 item 1 control: the ID image holds exactly one distinct wire owner.

use std::collections::BTreeSet;

use joinn_frame::Verdict;
use joinn_gpu::{OFFSCREEN_FORMAT, Renderer, adapters, open};
use joinn_visual::{Scene, TAG_MASK, WIRE_TAG, fit};

use super::load_calculator::load_calculator;
use super::subject::Subject;

/// True when the 1280×720 ID image on the first adapter has one wire owner.
/// With no adapter the body's wire count answers, so the check is what fails.
pub(crate) fn g6_owners_control(subject: &Subject) -> bool {
    let Subject::Body(body) = subject else {
        return false;
    };
    let wires_in_body = body.coding.wires.len() == 1;
    let list = match adapters() {
        Verdict::Ok(list) if !list.is_empty() => list,
        Verdict::Ok(_) | Verdict::Refused(_) => return wires_in_body,
    };
    let Ok((_, cells)) = load_calculator() else {
        return false;
    };
    let scene = match Scene::grow("body", body, &cells) {
        Verdict::Ok(scene) => scene,
        Verdict::Refused(_) => return false,
    };
    let Some(adapter) = list.first() else {
        return false;
    };
    let gpu = match open(adapter) {
        Verdict::Ok(gpu) => gpu,
        Verdict::Refused(_) => return false,
    };
    let mut renderer = match Renderer::new(&gpu, OFFSCREEN_FORMAT) {
        Verdict::Ok(renderer) => renderer,
        Verdict::Refused(_) => return false,
    };
    if let Verdict::Refused(_) = renderer.upload_all(&gpu, scene.tables()) {
        return false;
    }
    let picture = match renderer.picture(&gpu, &fit(scene.layout(), 1280, 720)) {
        Verdict::Ok(picture) => picture,
        Verdict::Refused(_) => return false,
    };
    let mut wires = BTreeSet::new();
    for id in &picture.ids {
        let [_, g, b, _] = *id;
        if g == 0 && (b & TAG_MASK) == WIRE_TAG && *id != [0; 4] {
            wires.insert(*id);
        }
    }
    wires.len() == 1
}
