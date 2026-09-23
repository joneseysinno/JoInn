//! An address at a body's membrane: instance and port.

/// An address at a body's membrane: instance and port.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Address {
    /// Instance name.
    pub instance: String,
    /// Port position.
    pub port: u32,
}

impl Address {
    /// `instance@port`.
    pub fn printed(&self) -> String {
        format!("{}@{}", self.instance, self.port)
    }
}
