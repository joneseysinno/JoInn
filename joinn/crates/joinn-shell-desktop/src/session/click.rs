//! A click: the CPU pick, then selection when the owner is an in-port.

use joinn_dna::Direction;
use joinn_frame::Verdict;
use joinn_visual::{Owner, Pick, cpu_pick, shapes_of_tables};

use super::{Confirm, Desktop};

impl Desktop {
    /// Floor is the caller's job. An edge pixel is named and not confirmed.
    /// An in-port in the intent set is selected; any other in-port is refused
    /// as an intent address.
    pub fn click(&mut self, x: u32, y: u32) -> Vec<String> {
        let mut lines = Vec::new();
        self.confirm = None;
        if x >= self.camera.width || y >= self.camera.height {
            lines.push(format!("pick {x},{y}: background (cpu)"));
            return lines;
        }
        let shapes = shapes_of_tables(self.scene.tables());
        let image = match cpu_pick(&shapes, &self.camera) {
            Verdict::Ok(image) => image,
            Verdict::Refused(r) => {
                lines.push(format!("pick {x},{y}: refused: {}", r.reason));
                return lines;
            }
        };
        let index = (y as usize) * (image.width as usize) + (x as usize);
        let Some(pick) = image.pixels.get(index).copied() else {
            lines.push(format!("pick {x},{y}: background (cpu)"));
            return lines;
        };
        match pick {
            Pick::Edge => lines.push(format!("pick {x},{y}: edge (cpu)")),
            Pick::Background => {
                let owner = "background".to_owned();
                lines.push(format!("pick {x},{y}: {owner} (cpu)"));
                self.confirm = Some(Confirm { x, y, cpu: owner });
                self.selected = None;
                self.buffer.clear();
            }
            Pick::Owned(id) => {
                let resolved = self.scene.resolve(id);
                let owner = match &resolved {
                    Verdict::Ok(o) => self.scene.print_owner(o),
                    Verdict::Refused(r) => {
                        lines.push(format!("pick {x},{y}: refused: {}", r.reason));
                        return lines;
                    }
                };
                lines.push(format!("pick {x},{y}: {owner} (cpu)"));
                self.confirm = Some(Confirm {
                    x,
                    y,
                    cpu: owner.clone(),
                });
                if let Verdict::Ok(Owner::Port(address)) = resolved {
                    let in_port = self
                        .scene
                        .layout()
                        .ports
                        .iter()
                        .any(|port| port.address == address && port.direction == Direction::In);
                    if in_port && self.intents.contains(&address) {
                        self.selected = Some(address);
                        self.buffer.clear();
                        lines.push(format!("selected {owner}"));
                    } else if in_port {
                        self.selected = None;
                        self.buffer.clear();
                        lines.push(format!("not an intent address: {owner}"));
                    } else {
                        self.selected = None;
                        self.buffer.clear();
                    }
                } else {
                    self.selected = None;
                    self.buffer.clear();
                }
            }
        }
        lines
    }
}

#[cfg(test)]
mod tests {
    use super::super::fixtures::calculator;

    /// §2.13, at 1280×720.
    const PROBES: &[(u32, u32, &str)] = &[
        (136, 80, "body surface"),
        (360, 220, "body.cli_a"),
        (360, 500, "body.cli_b"),
        (920, 276, "body.sum"),
        (192, 220, "body.cli_a@0"),
        (528, 220, "body.cli_a@1"),
        (192, 500, "body.cli_b@0"),
        (528, 500, "body.cli_b@1"),
        (752, 220, "body.sum@0"),
        (752, 332, "body.sum@1"),
        (1088, 220, "body.sum@2"),
        (640, 220, "body wire cli_a@1 -> sum@0"),
        (640, 416, "body wire cli_b@1 -> sum@1"),
        (0, 0, "background"),
    ];

    #[test]
    fn each_probe_click_names_its_owner() {
        let mut desktop = calculator();
        for (x, y, owner) in PROBES {
            let lines = desktop.click(*x, *y);
            assert_eq!(
                lines.first().map(String::as_str),
                Some(format!("pick {x},{y}: {owner} (cpu)")).as_deref(),
                "{lines:?}"
            );
        }
    }

    #[test]
    fn an_in_port_outside_the_intent_set_is_not_an_address() {
        let mut desktop = calculator();
        let lines = desktop.click(752, 220);
        assert!(
            lines
                .iter()
                .any(|line| line == "not an intent address: body.sum@0"),
            "{lines:?}"
        );
    }
}
