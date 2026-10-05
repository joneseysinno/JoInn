//! A universe lens as a view: every camera is rebased onto the chart under
//! the centre pixel, and a click is picked from the CPU cut's shapes.

use joinn_frame::Verdict;
use joinn_visual::{Camera, ChartId, Delta, Pick, Rect, Tables, cpu_pick_sample};

use super::{Atlas, Confirm, View};

impl View for Atlas {
    fn camera(&self) -> Camera {
        self.camera
    }

    fn set_camera(&mut self, camera: Camera) -> Verdict<()> {
        match self.scene.rebase(camera) {
            Verdict::Ok((next, _)) => {
                self.camera = next;
                Verdict::Ok(())
            }
            Verdict::Refused(r) => Verdict::Refused(r),
        }
    }

    fn home(&self, width: u32, height: u32) -> Verdict<Camera> {
        let (w, h) = self
            .scene
            .layout()
            .chart(ChartId(0))
            .map_or((0, 0), |c| c.size);
        Verdict::Ok(Camera::frame(
            Rect { x: 0, y: 0, w, h },
            ChartId(0),
            width,
            height,
        ))
    }

    fn anchor_name(&self) -> String {
        self.scene.layout().chart(self.camera.anchor).map_or_else(
            || format!("chart {}", self.camera.anchor.0),
            |c| c.name.clone(),
        )
    }

    fn tables(&self) -> &Tables {
        self.scene.tables()
    }

    fn take_pending(&mut self) -> Option<Delta> {
        self.scene.take_pending()
    }

    fn click(&mut self, x: u32, y: u32) -> Vec<String> {
        self.confirm = None;
        if x >= self.camera.width || y >= self.camera.height {
            return vec![format!("pick {x},{y}: background (cpu)")];
        }
        let picked = match self.scene.shapes_at(&self.camera) {
            Verdict::Ok(shapes) => cpu_pick_sample(&shapes, &self.camera, &[(x, y)]),
            Verdict::Refused(r) => Verdict::Refused(r),
        };
        let owner = match picked {
            Verdict::Ok(picks) => match picks.first() {
                Some(Pick::Edge) => return vec![format!("pick {x},{y}: edge (cpu)")],
                Some(Pick::Owned(id)) => self.scene.print_id(*id),
                Some(Pick::Background) | None => Verdict::Ok("background".to_owned()),
            },
            Verdict::Refused(r) => Verdict::Refused(r),
        };
        match owner {
            Verdict::Ok(owner) => {
                let line = format!("pick {x},{y}: {owner} (cpu)");
                self.confirm = Some(Confirm { x, y, cpu: owner });
                vec![line]
            }
            Verdict::Refused(r) => vec![format!("pick {x},{y}: refused: {}", r.reason)],
        }
    }

    fn take_confirm(&mut self) -> Option<Confirm> {
        self.confirm.take()
    }

    fn owner_line(&self, id: [u32; 4]) -> String {
        match self.scene.print_id(id) {
            Verdict::Ok(owner) => owner,
            Verdict::Refused(r) => r.reason,
        }
    }

    fn typing(&self) -> bool {
        false
    }

    fn enter(&mut self) -> Vec<String> {
        Vec::new()
    }

    fn backspace(&mut self) -> Option<String> {
        None
    }

    fn type_char(&mut self, _text: &str) -> Option<String> {
        None
    }

    fn escape(&mut self) {}
}
