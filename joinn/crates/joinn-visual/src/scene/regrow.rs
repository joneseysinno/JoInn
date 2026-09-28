//! Rebuild a scene from DNA and live state alone. The test of every delta (V122).

use std::collections::BTreeMap;

use joinn_dna::{Body, Cell};
use joinn_frame::{Hash, Verdict};
use joinn_host::describe;
use joinn_live::BodyState;

use super::Scene;

impl Scene {
    /// `grow`, then present `describe(state, i)` for every instance in name
    /// order, then mark the instance at `refusal_site` refused when
    /// `last_refusal` is `Some`. Nothing is left pending.
    pub fn regrow(
        alias: &str,
        body: &Body,
        cells: &BTreeMap<Hash, Cell>,
        state: &BodyState,
    ) -> Verdict<Scene> {
        let mut scene = match Scene::grow(alias, body, cells) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let names: Vec<String> = scene.cells.keys().cloned().collect();
        for name in &names {
            let d = match describe(state, name) {
                Verdict::Ok(d) => d,
                Verdict::Refused(r) => return Verdict::Refused(r),
            };
            if let Verdict::Refused(r) = scene.present(&d) {
                return Verdict::Refused(r);
            }
        }
        if state.last_refusal().is_some() {
            if let Some((site, _)) = state.refusal_site() {
                if let Verdict::Refused(r) = scene.mark_refused(site) {
                    return Verdict::Refused(r);
                }
            }
        }
        scene.pending.clear();
        Verdict::Ok(scene)
    }
}
