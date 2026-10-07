//! A system as a view: one chart, so a camera is taken as it is.

use joinn_frame::Verdict;
use joinn_visual::{Camera, Delta, Tables};

use super::{Confirm, Grower, View};

impl View for Grower {
    fn camera(&self) -> Camera {
        self.camera
    }

    fn set_camera(&mut self, camera: Camera) -> Verdict<()> {
        self.camera = camera;
        Verdict::Ok(())
    }

    fn home(&self, width: u32, height: u32) -> Verdict<Camera> {
        Camera::from_fit(&self.scene.fit(width, height))
    }

    fn anchor_name(&self) -> String {
        "system".to_owned()
    }

    fn tables(&self) -> &Tables {
        self.scene.tables()
    }

    fn take_pending(&mut self) -> Option<Delta> {
        self.scene.take_pending()
    }

    fn click(&mut self, x: u32, y: u32) -> Vec<String> {
        Grower::click(self, x, y)
    }

    fn take_confirm(&mut self) -> Option<Confirm> {
        self.confirm.take()
    }

    fn owner_line(&self, id: [u32; 4]) -> String {
        self.scene.print_id(id)
    }

    fn typing(&self) -> bool {
        self.selected.is_some()
    }

    fn enter(&mut self) -> Vec<String> {
        Grower::enter(self)
    }

    fn backspace(&mut self) -> Option<String> {
        self.selected.as_ref()?;
        self.buffer.pop();
        Some(format!("  typing: {}", self.buffer))
    }

    fn type_char(&mut self, text: &str) -> Option<String> {
        if self.selected.is_none() || text.is_empty() {
            return None;
        }
        self.buffer.push_str(text);
        Some(format!("  typing: {}", self.buffer))
    }

    fn escape(&mut self) {
        self.selected = None;
        self.buffer.clear();
    }
}
