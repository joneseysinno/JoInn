//! Rebuild a contact scene from DNA and live state alone (V122 on a contact).

use std::collections::BTreeMap;

use joinn_dna::{Cell, Contact};
use joinn_frame::{Hash, Verdict};
use joinn_host::describe;
use joinn_live::BodyState;

use super::Scene;

impl Scene {
    /// `grow_contact`, then present `describe(state, i)` for every instance in
    /// name order, then mark the instance at `refusal_site` refused when
    /// `last_refusal` is `Some`. `state` runs the contact's lowered body, whose
    /// instances are the contact's cells. Nothing is left pending.
    pub fn regrow_contact(
        alias: &str,
        contact: &Contact,
        cells: &BTreeMap<Hash, Cell>,
        state: &BodyState,
    ) -> Verdict<Scene> {
        let mut scene = match Scene::grow_contact(alias, contact, cells) {
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use joinn_dna::{Body, Cell, Contact, hash, parse_cell, parse_contact};
    use joinn_frame::{FrameRegistry, Hash, Verdict};
    use joinn_link::lower;
    use joinn_live::BodyState;
    use joinn_prim::sealed_natives;

    use crate::scene::Scene;
    use crate::script::event;
    use crate::tables::{LATENT, REFUSED, table_bytes};

    fn calculator_contact() -> (Contact, Body, BTreeMap<Hash, Cell>) {
        let mut cells = BTreeMap::new();
        for src in [
            include_str!("../../../../corpus/phase0/sum.cell"),
            include_str!("../../../../corpus/phase0/cli_input.cell"),
        ] {
            match parse_cell(src, &FrameRegistry::phase1()) {
                Verdict::Ok(c) => {
                    cells.insert(hash(&c.coding), c);
                }
                Verdict::Refused(r) => panic!("{}", r.reason),
            }
        }
        let contact = match parse_contact(
            include_str!("../../../../corpus/phase7/calculator.contact"),
            &FrameRegistry::phase1(),
        ) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let body = match lower(&contact, &cells, &FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        (contact, body, cells)
    }

    #[test]
    fn the_contact_script_keeps_delta_tables_equal_to_regrow() {
        let (contact, body, cells) = calculator_contact();
        let mut scene = match Scene::grow_contact("body", &contact, &cells) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        assert!(
            scene.tables().link.is_empty() && scene.tables().incidence.is_empty(),
            "a contact picture has no link rows (R80)"
        );
        let sum = scene.cells.get("sum").copied().unwrap_or(u32::MAX);
        let latent = |s: &Scene| {
            s.tables
                .cell
                .get(sum as usize)
                .is_some_and(|c| c.flags & LATENT != 0)
        };
        assert!(latent(&scene), "sum grows latent");
        let mut state = match BodyState::new(body.clone(), cells.clone(), sealed_natives(), 1) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let script = [("cli_a", "two"), ("cli_a", "2"), ("cli_b", "3")];
        let mut rows = Vec::new();
        for (epoch, (instance, text)) in (0u64..).zip(script) {
            let (delta, _) = event(
                &mut scene,
                &mut state,
                (&body, &cells),
                (instance, text),
                epoch,
            );
            rows.push(delta.rows.len());
            let regrown = match Scene::regrow_contact("body", &contact, &cells, &state) {
                Verdict::Ok(s) => s,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            assert_eq!(
                table_bytes(scene.tables()),
                table_bytes(regrown.tables()),
                "event {epoch}: V122 on a contact"
            );
            assert_eq!(scene.take_pending().unwrap_or_default(), delta);
        }
        assert_eq!(rows, [2, 1, 3], "§2.9 predicts rows 2, 1, 3");
        assert!(!latent(&scene), "sum@2 holds 5");
        assert!(scene.tables.cell.iter().all(|c| c.flags & REFUSED == 0));
    }
}
