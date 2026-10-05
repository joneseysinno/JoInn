//! A body as a view: one chart, so a camera is taken as it is.

use joinn_frame::Verdict;
use joinn_visual::{Camera, Delta, Tables, fit};

use super::{Confirm, Desktop, View};

impl View for Desktop {
    fn camera(&self) -> Camera {
        self.camera
    }

    fn set_camera(&mut self, camera: Camera) -> Verdict<()> {
        self.camera = camera;
        Verdict::Ok(())
    }

    fn home(&self, width: u32, height: u32) -> Verdict<Camera> {
        Camera::from_fit(&fit(self.scene.layout(), width, height))
    }

    fn anchor_name(&self) -> String {
        "body".to_owned()
    }

    fn tables(&self) -> &Tables {
        self.scene.tables()
    }

    fn take_pending(&mut self) -> Option<Delta> {
        self.scene.take_pending()
    }

    fn click(&mut self, x: u32, y: u32) -> Vec<String> {
        Desktop::click(self, x, y)
    }

    fn take_confirm(&mut self) -> Option<Confirm> {
        Desktop::take_confirm(self)
    }

    fn owner_line(&self, id: [u32; 4]) -> String {
        Desktop::owner_line(self, id)
    }

    fn typing(&self) -> bool {
        self.selected.is_some()
    }

    fn enter(&mut self) -> Vec<String> {
        Desktop::enter(self)
    }

    fn backspace(&mut self) -> Option<String> {
        Desktop::backspace(self)
    }

    fn type_char(&mut self, text: &str) -> Option<String> {
        Desktop::type_char(self, text)
    }

    fn escape(&mut self) {
        Desktop::escape(self)
    }
}
