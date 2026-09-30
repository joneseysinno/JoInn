//! Open a desktop on one body.

use std::collections::BTreeMap;

use joinn_dna::{Body, Cell, Contact};
use joinn_frame::{Hash, Verdict};
use joinn_host::{Signals, check_signals, intent_set};
use joinn_live::BodyState;
use joinn_prim::sealed_natives;
use joinn_visual::{Scene, fit};

use super::Desktop;

impl Desktop {
    /// Grow the scene, fit 1280×720 until the window says otherwise, and refuse
    /// a body that reads a signal this host does not emit. With a contact, the
    /// scene is the contact's cells in contact and `body` is its lowered body,
    /// which only runs.
    pub fn open(
        body: Body,
        cells: BTreeMap<Hash, Cell>,
        contact: Option<&Contact>,
    ) -> Result<Desktop, String> {
        let grown = match contact {
            Some(c) => Scene::grow_contact("body", c, &cells),
            None => Scene::grow("body", &body, &cells),
        };
        let scene = match grown {
            Verdict::Ok(scene) => scene,
            Verdict::Refused(r) => return Err(r.reason),
        };
        let signals = Signals::default();
        match check_signals(&body, &signals) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => return Err(r.reason),
        }
        let state = match BodyState::new(body.clone(), cells.clone(), sealed_natives(), 1) {
            Verdict::Ok(state) => state,
            Verdict::Refused(r) => return Err(r.reason),
        };
        let intents = intent_set(&body, &cells);
        let camera = fit(scene.layout(), 1280, 720);
        Ok(Desktop {
            body,
            cells,
            scene,
            state,
            camera,
            intents,
            selected: None,
            buffer: String::new(),
            epoch: 0,
            signals,
            confirm: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::fixtures::calculator;

    #[test]
    fn a_fresh_desktop_has_nothing_pending() {
        let mut desktop = calculator();
        assert_eq!(desktop.take_pending(), None);
    }
}
