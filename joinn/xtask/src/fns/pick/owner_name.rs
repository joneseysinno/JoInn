//! Name an ID texel against a scene, for a printed line.

use joinn_frame::Verdict;
use joinn_visual::Scene;

/// The printed owner, or the refusal's reason when the ID names nothing.
pub(crate) fn owner_name(scene: &Scene, id: [u32; 4]) -> String {
    match scene.resolve(id) {
        Verdict::Ok(o) => scene.print_owner(&o),
        Verdict::Refused(r) => format!("{id:?} ({})", r.reason),
    }
}
