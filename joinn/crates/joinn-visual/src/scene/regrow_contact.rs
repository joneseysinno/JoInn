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

    use crate::camera::fit;
    use crate::pick::{Pick, PickImage, cpu_pick};
    use crate::scene::Scene;
    use crate::script::event;
    use crate::tables::{
        FILLED, LATENT, REFUSED, STYLE_BACKGROUND, STYLE_CELL_REFUSED, STYLE_PORT_EMPTY,
        STYLE_PORT_FILLED, STYLE_RESPONSE_LATENT, STYLE_SURFACE, STYLE_TABLE, shapes_of_tables,
        table_bytes,
    };

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

    /// The owner at one pixel of a 1280×720 CPU pick, printed.
    fn owner_at(scene: &Scene, image: &PickImage, (x, y): (u32, u32)) -> String {
        match image.pixels.get((y * image.width + x) as usize) {
            Some(Pick::Owned(id)) => match scene.resolve(*id) {
                Verdict::Ok(o) => scene.print_owner(&o),
                Verdict::Refused(r) => panic!("{}", r.reason),
            },
            Some(Pick::Edge) => "edge".to_owned(),
            Some(Pick::Background) | None => "background".to_owned(),
        }
    }

    /// The RGB the shape shader gives a probe's owner: surface 1, a cell refused
    /// (3), else latent (7), else its row's style, a port 4 or 5 by `filled`.
    fn color_at(scene: &Scene, owner: &str) -> String {
        let t = scene.tables();
        let style = if owner == "background" {
            STYLE_BACKGROUND
        } else if owner == "body surface" {
            STYLE_SURFACE
        } else if let Some((cell, port)) =
            owner.strip_prefix("body.").and_then(|a| a.split_once('@'))
        {
            let slot = scene.cells.get(cell).copied().unwrap_or(u32::MAX);
            let position: u32 = port.parse().unwrap_or(u32::MAX);
            let row = t
                .port
                .iter()
                .find(|p| p.cell == slot && p.position == position);
            match row {
                Some(p) if p.flags & FILLED != 0 => STYLE_PORT_FILLED,
                Some(_) => STYLE_PORT_EMPTY,
                None => panic!("{owner} has no port row"),
            }
        } else {
            let name = owner.strip_prefix("body.").unwrap_or(owner);
            let slot = scene.cells.get(name).copied().unwrap_or(u32::MAX);
            match t.cell.get(slot as usize) {
                Some(c) if c.flags & REFUSED != 0 => STYLE_CELL_REFUSED,
                Some(c) if c.flags & LATENT != 0 => STYLE_RESPONSE_LATENT,
                Some(c) => c.style,
                None => panic!("{owner} has no cell row"),
            }
        };
        let [r, g, b, _] = STYLE_TABLE[style as usize].to_le_bytes();
        format!("{r:02X}{g:02X}{b:02X}")
    }

    #[test]
    fn the_contact_picture_meets_the_probes_seams_and_colors() {
        let (contact, body, cells) = calculator_contact();
        let mut scene = match Scene::grow_contact("body", &contact, &cells) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let camera = fit(scene.layout(), 1280, 720);
        let image = match cpu_pick(&shapes_of_tables(scene.tables()), &camera) {
            Verdict::Ok(i) => i,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let probes = [
            ((192, 104), "body surface"),
            ((448, 264), "body.cli_a"),
            ((448, 456), "body.cli_b"),
            ((832, 360), "body.sum"),
            ((256, 264), "body.cli_a@0"),
            ((256, 456), "body.cli_b@0"),
            ((1024, 264), "body.sum@2"),
            ((0, 0), "background"),
        ];
        for ((x, y), want) in probes {
            let got = owner_at(&scene, &image, (x, y));
            println!("probe {x},{y}: {got}");
            assert_eq!(got, want, "probe {x},{y}");
        }
        for y in [232, 264, 296] {
            let pair = (
                owner_at(&scene, &image, (639, y)),
                owner_at(&scene, &image, (640, y)),
            );
            println!("seam column 639|640 at row {y}: {} | {}", pair.0, pair.1);
            assert_eq!(pair, ("body.cli_a".to_owned(), "body.sum".to_owned()));
        }
        for x in [320, 448, 576] {
            let pair = (
                owner_at(&scene, &image, (x, 359)),
                owner_at(&scene, &image, (x, 360)),
            );
            println!("seam row 359|360 at column {x}: {} | {}", pair.0, pair.1);
            assert_eq!(pair, ("body.cli_a".to_owned(), "body.cli_b".to_owned()));
        }

        let owners = [
            "body.cli_a",
            "body.cli_b",
            "body.sum",
            "body.cli_a@0",
            "body.cli_b@0",
            "body.sum@2",
            "body surface",
            "background",
        ];
        let colors = |s: &Scene| -> Vec<String> { owners.iter().map(|o| color_at(s, o)).collect() };
        let mut seen = vec![colors(&scene)];
        let mut state = match BodyState::new(body.clone(), cells.clone(), sealed_natives(), 1) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        for (epoch, input) in (0u64..).zip([("cli_a", "two"), ("cli_a", "2"), ("cli_b", "3")]) {
            event(&mut scene, &mut state, (&body, &cells), input, epoch);
            seen.push(colors(&scene));
        }
        for (label, row) in ["grow", "event 1", "event 2", "event 3"].iter().zip(&seen) {
            println!("colors {label}: {}", row.join(" "));
        }
        let plan: [[&str; 8]; 4] = [
            [
                "2F5D8A", "2F5D8A", "39414D", "C9CED6", "C9CED6", "C9CED6", "22262E", "15171C",
            ],
            [
                "B03A2E", "2F5D8A", "39414D", "F2B134", "C9CED6", "C9CED6", "22262E", "15171C",
            ],
            [
                "2F5D8A", "2F5D8A", "39414D", "F2B134", "C9CED6", "C9CED6", "22262E", "15171C",
            ],
            [
                "2F5D8A", "2F5D8A", "2F5D8A", "F2B134", "F2B134", "F2B134", "22262E", "15171C",
            ],
        ];
        for (got, want) in seen.iter().zip(plan) {
            assert_eq!(got, &want.map(str::to_owned).to_vec());
        }
    }
}
