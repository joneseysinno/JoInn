//! Rebuild a universe scene from its layout and the bodies' descriptions.

use joinn_frame::Verdict;
use joinn_host::Description;

use super::UniverseScene;
use crate::camera::ChartId;
use crate::charts::UniverseLayout;

impl UniverseScene {
    /// `grow` at `anchor`, then `present` each `(alias, description)` in order.
    /// Nothing is left pending.
    pub fn regrow(
        layout: UniverseLayout,
        anchor: ChartId,
        descriptions: &[(String, Description)],
    ) -> Verdict<UniverseScene> {
        let mut scene = match UniverseScene::grow(layout, anchor) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        for (alias, d) in descriptions {
            if let Verdict::Refused(r) = scene.present(alias, d) {
                return Verdict::Refused(r);
            }
        }
        scene.pending.clear();
        Verdict::Ok(scene)
    }
}
