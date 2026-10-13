//! Open a chromosome: draw its strand and its fold and fit the one shown.

use joinn_frame::Verdict;
use joinn_tower::{Chromosome, Pool};
use joinn_visual::{Camera, FoldScene, StrandScene};

use super::Dna;

impl Dna {
    /// The strand beside `strand.partner()`, and the fold at floor 0 with
    /// nothing opened; `folded` picks the one shown first, fitted to 1280x720
    /// until the window says otherwise. The pool is read only when Enter opens
    /// a gene.
    pub fn open(chromosome: Chromosome, pool: Pool, folded: bool) -> Result<Dna, String> {
        let strand = match StrandScene::new(chromosome.strand(), &chromosome.strand().partner()) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => return Err(r.reason),
        };
        let fold = match FoldScene::new(chromosome) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => return Err(r.reason),
        };
        let fit = if folded {
            fold.fit(1280, 720)
        } else {
            strand.fit(1280, 720)
        };
        let camera = match Camera::from_fit(&fit) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => return Err(r.reason),
        };
        Ok(Dna {
            strand,
            fold,
            pool,
            folded,
            camera,
            other: None,
            whole: false,
            selected: None,
            confirm: None,
        })
    }
}
