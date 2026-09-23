//! Capabilities that ride an ordered hyperedge and can be revoked.

mod check;
mod grant;
mod new;
mod revoke;

pub use check::check_capability;
pub use grant::grant;
pub use new::LinkRuntime;
pub use revoke::revoke;
