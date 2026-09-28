//! Every corpus body as a scene, or the reason it isn't one.

use joinn_frame::Verdict;
use joinn_visual::Scene;

use crate::fns::corpus_bodies;

/// A body that doesn't bind, or that layout or `Scene::grow` refuses, carries
/// its reason and is printed `not measured` (Amendment A3).
pub(crate) fn measured() -> Result<Vec<(String, Verdict<Scene>)>, String> {
    Ok(corpus_bodies()?
        .into_iter()
        .map(|(rel, bound)| {
            let scene = bound.and_then(|(body, cells)| Scene::grow("body", &body, &cells));
            (rel, scene)
        })
        .collect())
}
