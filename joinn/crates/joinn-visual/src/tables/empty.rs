//! Tables with no rows.

use super::{STYLE_TABLE, Tables};

impl Tables {
    /// No rows; the style table of §2.8.
    pub fn empty() -> Self {
        Tables {
            body: Vec::new(),
            cell: Vec::new(),
            port: Vec::new(),
            link: Vec::new(),
            incidence: Vec::new(),
            style: STYLE_TABLE.to_vec(),
            chart: Vec::new(),
            frame: Vec::new(),
            stroke: Vec::new(),
            route: Vec::new(),
            segment: Vec::new(),
        }
    }
}
