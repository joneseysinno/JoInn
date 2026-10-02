use num_traits::Signed;

use super::Bridge;
use crate::q::Q;
use crate::tag::Tag;
use crate::verdict::{Verdict, refuse};

impl Bridge {
    /// κ = M / (E·I), counting one read. `what` names the quantity that needed it.
    pub fn bridge(&mut self, m: &Q, what: &str) -> Verdict<Q> {
        if m.tag() != Tag::MOMENT {
            return refuse(format!(
                "bridge: {} is not a moment; acceptance is {}",
                m.tag(),
                Tag::MOMENT
            ));
        }
        let Some(edition) = &self.edition else {
            return refuse(format!(
                "{what} needs the bridge E·I; acceptance is a pinned edition"
            ));
        };
        let ei = &edition.e * &edition.ix;
        if !ei.is_positive() {
            return refuse(format!(
                "bridge: E·I is {ei} in the edition; acceptance is a positive E·I"
            ));
        }
        self.reads = self.reads.saturating_add(1);
        Verdict::Admitted(Q::new(m.value() / ei, Tag::CURVATURE))
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Bridge, Edition};
    use crate::q::{Q, frac};
    use crate::tag::Tag;
    use crate::verdict::{admitted, refused};

    fn aisc() -> Edition {
        Edition {
            source: "AISC Manual, 16th ed.".to_string(),
            material: "A992".to_string(),
            e: frac!(29000, 1),
            shape: "W12x26".to_string(),
            ix: frac!(204, 1),
        }
    }

    #[test]
    fn a_moment_crosses_to_a_curvature_and_counts_one_read() {
        let mut bridge = Bridge::new(Some(aisc()));
        assert_eq!(bridge.reads(), 0);
        let m = Q::new(frac!(5916000, 1), Tag::MOMENT);
        assert_eq!(
            admitted(bridge.bridge(&m, "v(12 ft)")),
            Q::new(frac!(1, 1), Tag::CURVATURE)
        );
        assert_eq!(bridge.reads(), 1);
    }

    #[test]
    fn without_an_edition_the_bridge_refuses_and_reads_nothing() {
        let mut bridge = Bridge::new(None);
        let m = Q::new(frac!(1, 1), Tag::MOMENT);
        assert_eq!(
            refused(bridge.bridge(&m, "v(12 ft)")),
            "v(12 ft) needs the bridge E·I; acceptance is a pinned edition"
        );
        assert_eq!(bridge.reads(), 0);
    }

    #[test]
    fn only_a_moment_crosses() {
        let mut bridge = Bridge::new(Some(aisc()));
        let f = Q::new(frac!(1, 1), Tag::FORCE);
        assert_eq!(
            refused(bridge.bridge(&f, "v(12 ft)")),
            "bridge: source · length 0 · y is not a moment; acceptance is source · length 1 · plane"
        );
        assert_eq!(bridge.reads(), 0);
    }
}
