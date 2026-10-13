//! `+`/`-` in the fold: one floor down or up.

use joinn_frame::Verdict;

use super::Dna;

impl Dna {
    /// `-` climbs a floor (the picture keeps the order below), `+` comes back
    /// down: `floor <f>: <n> segments`. Past either end the tower's refusal
    /// prints and nothing changes. `None` in the strand view: the camera zooms.
    pub fn floor_step(&mut self, out: bool) -> Option<(Vec<String>, bool)> {
        if !self.folded {
            return None;
        }
        let now = self.fold.floor();
        let next = if out {
            now.checked_add(1)
        } else {
            now.checked_sub(1)
        };
        let Some(next) = next else {
            return Some((
                vec![
                    "refused: fold view: floor 0 is every segment; acceptance is - to climb a floor"
                        .to_owned(),
                ],
                false,
            ));
        };
        match self.fold.set_floor(next) {
            Verdict::Ok(()) => {
                let segments = (1u64 << (2 * (self.fold.chromosome().order() - next))) - 1;
                Some((vec![format!("floor {next}: {segments} segments")], true))
            }
            Verdict::Refused(r) => Some((vec![format!("refused: {}", r.reason)], false)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::View;
    use super::super::dna_fixture::twelve;

    #[test]
    fn each_out_key_climbs_a_floor_and_each_in_key_comes_back() {
        let mut d = twelve(true);
        let mut lines = Vec::new();
        for out in [true, true, true, false, false, false] {
            let Some((printed, _)) = d.floor_key(out) else {
                panic!("the fold zooms by floors");
            };
            lines.extend(printed);
        }
        assert_eq!(
            lines,
            [
                "floor 1: 3 segments",
                "floor 2: 0 segments",
                "refused: fold view: floor 3 is above order 2's top floor; acceptance is a floor from 0 to 2",
                "floor 1: 3 segments",
                "floor 0: 15 segments",
                "refused: fold view: floor 0 is every segment; acceptance is - to climb a floor",
            ]
        );
        let mut s = twelve(false);
        assert_eq!(s.floor_key(true), None, "the strand view zooms its camera");
    }
}
