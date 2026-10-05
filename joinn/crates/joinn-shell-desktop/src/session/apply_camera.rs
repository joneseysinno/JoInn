//! Hand a new camera to the view.

use joinn_frame::Verdict;
use joinn_visual::Camera;

use super::Shell;

impl Shell {
    /// The view takes `camera` (rebasing it) and a tick is owed. A refusal
    /// changes nothing and is the line returned.
    pub(super) fn apply(&mut self, camera: Camera) -> Vec<String> {
        match self.view.set_camera(camera) {
            Verdict::Ok(()) => {
                self.dirty = true;
                Vec::new()
            }
            Verdict::Refused(r) => vec![format!("refused: {}", r.reason)],
        }
    }
}
