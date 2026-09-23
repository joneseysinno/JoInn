//! CLI rendering of a description. The template stays here, not in joinn-host.

mod fill_present;
mod prompt;

pub(crate) use fill_present::fill_present;
pub(crate) use prompt::prompt;
