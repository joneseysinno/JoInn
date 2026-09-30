//! Gate 7 item 3 control: no force's response owns a pixel.

use std::collections::BTreeSet;

use joinn_frame::{FrameRegistry, Verdict};
use joinn_gpu::{OFFSCREEN_FORMAT, Renderer, open};
use joinn_visual::{Owner, Scene, fit};

use super::forces::corpus_cells;
use super::g6_adapters::g6_adapters;
use super::subject::Subject;

/// True when no pixel of the 1280×720 ID image, on the first adapter, is owned
/// by a cell whose layout box is a response. A subject that doesn't grow has
/// no picture, so no response owns a pixel. No adapter answers `false`.
pub(crate) fn g7_picture_control(subject: &Subject) -> bool {
    let Subject::Contact(contact) = subject else {
        return false;
    };
    let Ok(cells) = corpus_cells(&FrameRegistry::phase1()) else {
        return false;
    };
    let scene = match Scene::grow_contact("body", contact, &cells) {
        Verdict::Ok(scene) => scene,
        Verdict::Refused(_) => return true,
    };
    let responses: BTreeSet<&str> = scene
        .layout()
        .cells
        .iter()
        .filter(|c| c.response)
        .map(|c| c.instance.as_str())
        .collect();
    let Some(adapter) = g6_adapters().and_then(|all| all.into_iter().next()) else {
        return false;
    };
    let Verdict::Ok(gpu) = open(&adapter) else {
        return false;
    };
    let Verdict::Ok(mut renderer) = Renderer::new(&gpu, OFFSCREEN_FORMAT) else {
        return false;
    };
    if let Verdict::Refused(_) = renderer.upload_all(&gpu, scene.tables()) {
        return false;
    }
    let Verdict::Ok(picture) = renderer.picture(&gpu, &fit(scene.layout(), 1280, 720)) else {
        return false;
    };
    let ids: BTreeSet<[u32; 4]> = picture.ids.iter().copied().collect();
    !ids.iter().any(|id| {
        matches!(scene.resolve(*id), Verdict::Ok(Owner::Cell(name)) if responses.contains(name.as_str()))
    })
}
