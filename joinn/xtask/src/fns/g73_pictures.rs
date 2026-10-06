//! Gate 7.3: the GPU's ID images of chosen views, on one adapter.

use joinn_frame::Verdict;
use joinn_gpu::{Gpu, OFFSCREEN_FORMAT, Renderer};
use joinn_visual::{Camera, UniverseScene};

/// Per `(scene, camera)`, the ID image the GPU draws of that scene from that
/// camera: one renderer per scene, its tables uploaded once.
pub(crate) fn g73_pictures(
    gpu: &Gpu,
    scenes: &[UniverseScene],
    views: &[(usize, Camera)],
) -> Result<Vec<Vec<[u32; 4]>>, String> {
    let mut renderers = Vec::new();
    for scene in scenes {
        let mut r = match Renderer::new(gpu, OFFSCREEN_FORMAT) {
            Verdict::Ok(r) => r,
            Verdict::Refused(r) => return Err(r.reason),
        };
        if let Verdict::Refused(r) = r.upload_all(gpu, scene.tables()) {
            return Err(r.reason);
        }
        renderers.push(r);
    }
    let mut images = Vec::new();
    for (scene, camera) in views {
        let Some(renderer) = renderers.get_mut(*scene) else {
            return Err(format!("gate 7.3: no scene {scene}"));
        };
        match renderer.picture_at(gpu, camera) {
            Verdict::Ok(p) => images.push(p.ids),
            Verdict::Refused(r) => return Err(r.reason),
        }
    }
    Ok(images)
}
