//! The cut: what one camera draws of a laid-out lens. Each galaxy, then if
//! open each system, then if open each body; only visible things count. A
//! system or galaxy whose owner band is below Summary is a lens node and owns
//! its pixels. Every decision is an integer test (V141).

mod cut;
mod touches;

pub use cut::cut;
pub use touches::touches;

use crate::bands::Band;
use crate::camera::ChartId;

/// How a visible chart is drawn.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CutForm {
    /// A system or galaxy drawn as its frame, title and children.
    Open,
    /// A system or galaxy folded into one node with its name.
    Node,
    /// A body, in its owner band.
    Drawn(Band),
}

/// One visible chart.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CutEntry {
    /// The chart.
    pub chart: ChartId,
    /// How it is drawn.
    pub form: CutForm,
    /// In a crossfade window: a body at any threshold, a frame at 32 px.
    pub fading: bool,
}

/// The visible charts below the root, in chart order.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Cut {
    /// Every visible galaxy, system and body that is walked.
    pub entries: Vec<CutEntry>,
}

impl Cut {
    /// How `chart` is drawn, if it is in the cut.
    pub fn form(&self, chart: ChartId) -> Option<CutForm> {
        self.entries
            .iter()
            .find(|e| e.chart == chart)
            .map(|e| e.form)
    }
}
