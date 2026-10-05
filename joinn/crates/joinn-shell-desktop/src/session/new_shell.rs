//! A shell over one view.

use super::{Shell, View};

impl Shell {
    /// Nothing is owed until an input arrives; the first resize frames the view.
    pub fn new(view: Box<dyn View>) -> Shell {
        Shell {
            view,
            framed: false,
            dirty: false,
            press: None,
            last: (0, 0),
            dragging: false,
        }
    }
}
