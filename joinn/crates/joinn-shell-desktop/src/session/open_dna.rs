//! Open a strand: draw it beside the reader's partner and fit it.

use joinn_frame::Verdict;
use joinn_strand::Strand;
use joinn_visual::{Camera, StrandScene};

use super::Dna;

impl Dna {
    /// The strand and `strand.partner()`, fitted to 1280x720 until the window
    /// says otherwise.
    pub fn open(strand: &Strand) -> Result<Dna, String> {
        let scene = match StrandScene::new(strand, &strand.partner()) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => return Err(r.reason),
        };
        let camera = match Camera::from_fit(&scene.fit(1280, 720)) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => return Err(r.reason),
        };
        Ok(Dna {
            strand: scene,
            camera,
            confirm: None,
        })
    }
}
