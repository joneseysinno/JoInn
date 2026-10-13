//! Tab: the other picture of the same chromosome.

use joinn_frame::Verdict;
use joinn_tower::short_hash;
use joinn_visual::Camera;

use super::Dna;

impl Dna {
    /// `view: fold of <hash>, floor <f>` or `view: strand <hash>`. Each picture
    /// keeps its camera; the fold's first showing is framed at the current
    /// viewport. The renderer holds the other picture's rows, so every row of
    /// this one is owed. The selection clears.
    pub fn switch(&mut self) -> Vec<String> {
        let head = short_hash(&self.strand.hash());
        let next = match self.other.take() {
            Some(c) => c,
            None => {
                let (w, h) = (self.camera.width, self.camera.height);
                let fit = if self.folded {
                    self.strand.fit(w, h)
                } else {
                    self.fold.fit(w, h)
                };
                match Camera::from_fit(&fit) {
                    Verdict::Ok(c) => c,
                    Verdict::Refused(r) => return vec![format!("refused: {}", r.reason)],
                }
            }
        };
        self.other = Some(std::mem::replace(&mut self.camera, next));
        self.folded = !self.folded;
        self.whole = true;
        self.selected = None;
        self.confirm = None;
        if self.folded {
            vec![format!("view: fold of {head}, floor {}", self.fold.floor())]
        } else {
            vec![format!("view: strand {head}")]
        }
    }
}

#[cfg(test)]
mod tests {
    use joinn_visual::{CODON_TAG, Pick, cpu_pick_at};

    use super::super::View;
    use super::super::dna_fixture::twelve;
    use crate::session::Dna;

    /// The first pixel the CPU pick gives to codon 7, clicked.
    fn click_codon_7(d: &mut Dna) -> Vec<String> {
        let shapes = if d.folded {
            d.fold.shapes()
        } else {
            d.strand.shapes()
        };
        let picked = match cpu_pick_at(&shapes, &d.camera) {
            joinn_frame::Verdict::Ok(p) => p,
            joinn_frame::Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let width = d.camera.width as usize;
        let Some(at) = picked
            .pixels
            .iter()
            .position(|p| matches!(p, Pick::Owned(id) if id[1] == 0 && id[2] == CODON_TAG | 7))
        else {
            panic!("codon 7 owns no pixel");
        };
        let (x, y) = ((at % width) as u32, (at / width) as u32);
        d.click(x, y)
    }

    #[test]
    fn tab_shows_the_fold_and_codon_7_picks_the_same_address_in_both() {
        let mut d = twelve(false);
        assert!(d.take_pending().is_some(), "the strand's rows");
        let in_strand = click_codon_7(&mut d);
        assert_eq!(
            View::tab(&mut d),
            Some(vec!["view: fold of edf1529a6fc2, floor 0".to_owned()])
        );
        assert_eq!(d.anchor_name(), "fold");
        let Some(delta) = d.take_pending() else {
            panic!("the fold's rows are owed");
        };
        let t = d.tables();
        assert_eq!(
            delta.rows.len(),
            t.body.len() + t.chart.len() + t.stroke.len()
        );
        let in_fold = click_codon_7(&mut d);
        let tail = |lines: &[String]| -> Vec<String> {
            lines
                .iter()
                .map(|l| l.split(": ").nth(1).unwrap_or("").to_owned())
                .collect()
        };
        assert_eq!(tail(&in_strand), ["edf1529a6fc2.codon7 (cpu)"]);
        assert_eq!(tail(&in_fold), tail(&in_strand));
        assert_eq!(
            View::tab(&mut d),
            Some(vec!["view: strand edf1529a6fc2".to_owned()])
        );
        assert_eq!(d.anchor_name(), "strand");
    }
}
