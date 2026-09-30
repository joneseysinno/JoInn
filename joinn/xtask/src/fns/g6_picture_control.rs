//! Gate 6 item 2 control: after the script, no live `sum@2` row is filled.

use super::g6_drive::g6_drive;
use super::load_calculator::load_calculator;
use super::pick::port_slot;
use super::regrow::Form;
use super::subject::Subject;
use joinn_visual::FILLED;

/// True when the script leaves no live port row `sum@2` with `filled` set.
pub(crate) fn g6_picture_control(subject: &Subject) -> bool {
    let Subject::Body(body) = subject else {
        return false;
    };
    let Ok((_, cells)) = load_calculator() else {
        return false;
    };
    let Ok(driven) = g6_drive(Form::Wired, body, &cells) else {
        return false;
    };
    let filled = port_slot(&driven.scene, "body.sum@2")
        .and_then(|slot| driven.scene.tables().port.get(slot as usize))
        .is_some_and(|row| (row.flags & FILLED) != 0);
    !filled
}
