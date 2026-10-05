//! Name the owner an ID texel of a universe scene names.

use joinn_frame::Verdict;

use super::UniverseScene;
use crate::charts::ChartKind;
use crate::pick::{GALAXY_TAG, PORT_TAG, SYSTEM_TAG, TAG_MASK, WIRE_TAG};
use crate::refuse::refuse;

impl UniverseScene {
    /// `background`, `galaxy <name>`, `system <name>`, `<alias> surface`,
    /// `<alias>.<cell>`, `<alias>.<cell>@<position>`, or
    /// `<alias> wire <cell>@<p> -> <cell>@<p>`. A text stroke carries the ID
    /// of what it labels, so it prints as that.
    pub fn print_id(&self, id: [u32; 4]) -> Verdict<String> {
        let [r, g, b, _] = id;
        if id == [0; 4] {
            return Verdict::Ok("background".to_owned());
        }
        let (tag, index) = (b & TAG_MASK, b & !TAG_MASK);
        let unknown = || {
            refuse(format!(
                "pick: ID {id:?} names nothing in this scene; acceptance is an ID the scene's tables wrote"
            ))
        };
        if r == 0 {
            let (kind, word) = match tag {
                GALAXY_TAG => (ChartKind::Galaxy, "galaxy"),
                SYSTEM_TAG => (ChartKind::System, "system"),
                _ => return unknown(),
            };
            return match self
                .layout
                .charts
                .iter()
                .find(|c| c.kind == kind && c.index == index)
            {
                Some(c) => Verdict::Ok(format!("{word} {}", c.name)),
                None => unknown(),
            };
        }
        let Some(alias) = self
            .bodies
            .iter()
            .find(|(_, s)| **s == r - 1)
            .map(|(a, _)| a.as_str())
        else {
            return unknown();
        };
        let cell_name = |slot: u32| {
            self.cells
                .iter()
                .find(|(_, s)| **s == slot)
                .map(|((_, name), _)| name.as_str())
        };
        let port_name = |entry: u32| {
            let port = self
                .tables
                .incidence
                .get(entry as usize)
                .and_then(|p| self.tables.port.get(*p as usize))?;
            Some(format!("{}@{}", cell_name(port.cell)?, port.position))
        };
        match (g, tag) {
            (0, 0) => Verdict::Ok(format!("{alias} surface")),
            (0, WIRE_TAG) => {
                let ends =
                    self.tables.link.get(index as usize).and_then(|l| {
                        Some((port_name(l.start)?, port_name(l.start.checked_add(1)?)?))
                    });
                match ends {
                    Some((src, dst)) => Verdict::Ok(format!("{alias} wire {src} -> {dst}")),
                    None => unknown(),
                }
            }
            (0, _) => unknown(),
            (c, 0) => match cell_name(c - 1) {
                Some(name) => Verdict::Ok(format!("{alias}.{name}")),
                None => unknown(),
            },
            (c, PORT_TAG) => match cell_name(c - 1) {
                Some(name) => Verdict::Ok(format!("{alias}.{name}@{index}")),
                None => unknown(),
            },
            _ => unknown(),
        }
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;

    use crate::camera::ChartId;
    use crate::fixtures::phase5_universe;
    use crate::pick::{GALAXY_TAG, PORT_TAG, SYSTEM_TAG, WIRE_TAG};
    use crate::universe_scene::UniverseScene;

    #[test]
    fn every_kind_of_id_prints_its_owner() {
        let scene = match UniverseScene::grow(phase5_universe(), ChartId(0)) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let Some(calc) = scene.body_slot("calc") else {
            panic!("calc has a body slot");
        };
        let sum = scene
            .cells
            .get(&(calc, "sum".to_owned()))
            .copied()
            .unwrap_or(u32::MAX);
        let wire = scene
            .tables
            .link
            .iter()
            .position(|l| l.body == calc)
            .unwrap_or(usize::MAX) as u32;
        let printed = |id: [u32; 4]| match scene.print_id(id) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => r.reason,
        };
        assert_eq!(printed([0; 4]), "background");
        assert_eq!(printed([0, 0, GALAXY_TAG, 1]), "galaxy app");
        assert_eq!(printed([0, 0, SYSTEM_TAG | 1, 1]), "system measurement");
        assert_eq!(printed([calc + 1, 0, 0, 1]), "calc surface");
        assert_eq!(printed([calc + 1, sum + 1, 0, 1]), "calc.sum");
        assert_eq!(printed([calc + 1, sum + 1, PORT_TAG | 2, 1]), "calc.sum@2");
        assert_eq!(
            printed([calc + 1, 0, WIRE_TAG | wire, 1]),
            "calc wire cli_a@1 -> sum@0"
        );
        assert_eq!(
            printed([0, 0, SYSTEM_TAG | 99, 1]),
            "pick: ID [0, 0, 805306467, 1] names nothing in this scene; acceptance is an ID the scene's tables wrote"
        );
    }
}
