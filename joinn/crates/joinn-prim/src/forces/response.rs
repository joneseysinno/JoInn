//! Look up how a frame responds to a force.

use joinn_frame::{FrameRef, Hash};

use super::{ForceKind, register};

/// The registered response to `force` on `frame`, or `None` when the register
/// holds no row for that pair.
pub fn response(force: ForceKind, frame: &FrameRef) -> Option<Hash> {
    register()
        .iter()
        .find(|row| row.force == force && row.frame_ref().as_ref() == Some(frame))
        .map(|row| row.response)
}

#[cfg(test)]
mod tests {
    use super::response;
    use crate::forces::ForceKind;
    use joinn_frame::FrameRef;

    #[test]
    fn int_has_the_sum_cell_and_rat_has_no_row() {
        assert_eq!(
            response(ForceKind::Combine, &FrameRef::int()).map(|h| h.to_hex()),
            Some("6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39".to_owned())
        );
        assert_eq!(response(ForceKind::Combine, &FrameRef::rat()), None);
    }
}
