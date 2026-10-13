//! A chromosome as a view: one chart per picture, so a camera is taken as it
//! is. In the strand the arrow keys move along it and `+`/`-` move in and out;
//! there is no orbit. In the fold `+`/`-` change the floor.

use joinn_frame::Verdict;
use joinn_visual::{Camera, Delta, Tables};

use super::whole_delta::whole_delta;
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
        if self.folded {
            Camera::from_fit(&self.fold.fit(width, height))
        } else {
            Camera::from_fit(&self.strand.fit(width, height))
        }
    }

    fn anchor_name(&self) -> String {
        if self.folded { "fold" } else { "strand" }.to_owned()
    }

    fn tables(&self) -> &Tables {
        if self.folded {
            self.fold.tables()
        } else {
            self.strand.tables()
        }
    }

    fn take_pending(&mut self) -> Option<Delta> {
        let pending = if self.folded {
            self.fold.take_pending()
        } else {
            self.strand.take_pending()
        };
        if std::mem::take(&mut self.whole) {
            return Some(whole_delta(self.tables()));
        }
        pending
    }

    fn click(&mut self, x: u32, y: u32) -> Vec<String> {
        Dna::click(self, x, y)
    }

    fn take_confirm(&mut self) -> Option<Confirm> {
        self.confirm.take()
    }

    fn owner_line(&self, id: [u32; 4]) -> String {
        if self.folded {
            self.fold.print_id(id)
        } else {
            self.strand.print_id(id)
        }
    }

    fn typing(&self) -> bool {
        false
    }

    fn enter(&mut self) -> Vec<String> {
        self.open_gene()
    }

    fn backspace(&mut self) -> Option<String> {
        None
    }

    fn type_char(&mut self, _text: &str) -> Option<String> {
        None
    }

    fn escape(&mut self) {
        self.selected = None;
    }

    fn tab(&mut self) -> Option<Vec<String>> {
        Some(self.switch())
    }

    fn floor_key(&mut self, out: bool) -> Option<(Vec<String>, bool)> {
        self.floor_step(out)
    }
}
