//! A strand as a view: one chart, so a camera is taken as it is. The arrow
//! keys move along the strand and `+`/`-` move in and out; there is no orbit.

use joinn_frame::Verdict;
use joinn_visual::{Camera, Delta, Tables};

use super::{Confirm, Dna, View};

impl View for Dna {
    fn camera(&self) -> Camera {
        self.camera
    }

    fn set_camera(&mut self, camera: Camera) -> Verdict<()> {
        self.camera = camera;
        Verdict::Ok(())
    }

    fn home(&self, width: u32, height: u32) -> Verdict<Camera> {
        Camera::from_fit(&self.strand.fit(width, height))
    }

    fn anchor_name(&self) -> String {
        "strand".to_owned()
    }

    fn tables(&self) -> &Tables {
        self.strand.tables()
    }

    fn take_pending(&mut self) -> Option<Delta> {
        self.strand.take_pending()
    }

    fn click(&mut self, x: u32, y: u32) -> Vec<String> {
        Dna::click(self, x, y)
    }

    fn take_confirm(&mut self) -> Option<Confirm> {
        self.confirm.take()
    }

    fn owner_line(&self, id: [u32; 4]) -> String {
        self.strand.print_id(id)
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
