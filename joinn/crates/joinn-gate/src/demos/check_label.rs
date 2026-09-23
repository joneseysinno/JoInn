//! CheckId display labels for power output.

use joinn_frame::CheckId;

pub(in crate::demos) fn check_label(c: CheckId) -> &'static str {
    match c {
        CheckId::Laws => "check::laws",
        CheckId::Witnesses => "check::witnesses",
        CheckId::Contract => "check::contract",
        CheckId::Extension => "check::extension",
        CheckId::FrameObligation => "check::frame",
        CheckId::Parse => "check::parse",
        CheckId::Canonicalize => "check::canonicalize",
        CheckId::Advisory => "check::advisory",
        CheckId::Budget => "check::budget",
        CheckId::Join => "check::join",
        CheckId::Grant => "check::grant",
        CheckId::V33 => "check::v33",
        CheckId::Other => "check::other",
    }
}
