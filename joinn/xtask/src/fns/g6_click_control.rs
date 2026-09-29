//! Gate 6 item 3 control: no pixel resolves to `cli_b` or one of its ports.

use joinn_frame::Verdict;
use joinn_gpu::{OFFSCREEN_FORMAT, Renderer, adapters, open};
use joinn_visual::{Owner, Scene, fit};

use super::load_calculator::load_calculator;
use super::subject::Subject;

/// True when the 1280×720 ID image on the first adapter never names `cli_b`.
/// With no adapter, the body's genome answers, so the check is what fails.
pub(crate) fn g6_click_control(subject: &Subject) -> bool {
    let Subject::Body(body) = subject else {
        return false;
    };
    let gone = !body
        .coding
        .genome
        .iter()
        .any(|entry| entry.instances.iter().any(|name| name == "cli_b"));
    let list = match adapters() {
        Verdict::Ok(list) if !list.is_empty() => list,
        Verdict::Ok(_) | Verdict::Refused(_) => return gone,
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
    let names_cli_b = |id: [u32; 4]| match scene.resolve(id) {
        Verdict::Ok(Owner::Cell(name)) => name == "cli_b",
        Verdict::Ok(Owner::Port(address)) => address.instance == "cli_b",
        _ => false,
    };
    !picture.ids.iter().copied().any(names_cli_b)
}
