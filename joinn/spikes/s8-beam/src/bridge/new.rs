use super::{Bridge, Edition};

impl Bridge {
    /// A bridge over `edition`, or over none. No read has happened yet.
    pub fn new(edition: Option<Edition>) -> Bridge {
        Bridge { edition, reads: 0 }
    }
}
