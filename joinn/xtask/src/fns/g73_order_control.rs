//! Gate 7.3 item 2 control: the subject is refused, or an arrowhead is drawn
//! against its declared marks.

use std::collections::BTreeSet;

use joinn_link::{Mark, Order};
use joinn_visual::PieceKind;

use super::g72_layouts::g72_layouts;
use super::subject::Subject;

/// True when admission or a lens's layout refuses the subject, or some route
/// of some lens has an arrowhead mid-path while its link declares no order,
/// or its stub arrowheads are not exactly on the stubs of its tail members.
pub(crate) fn g73_order_control(subject: &Subject) -> bool {
    let Subject::Universe(u) = subject else {
        return false;
    };
    let Ok(layouts) = g72_layouts(u) else {
        return true;
    };
    layouts.iter().any(|layout| {
        layout.routes.routes.iter().any(|r| {
            let Some(link) = u.coding.links.get(r.link) else {
                return true;
            };
            let tail = |member: u32| {
                (member as usize)
                    .checked_sub(1)
                    .and_then(|i| link.members.get(i))
                    .is_some_and(|m| m.mark == Mark::Tail)
            };
            let arrows: BTreeSet<u32> = r
                .pieces
                .iter()
                .filter(|p| p.kind == PieceKind::Arrow && p.member > 0)
                .map(|p| p.member)
                .collect();
            let tails: BTreeSet<u32> = r
                .pieces
                .iter()
                .filter(|p| p.kind == PieceKind::Stub && tail(p.member))
                .map(|p| p.member)
                .collect();
            let mid_path = r
                .pieces
                .iter()
                .any(|p| p.kind == PieceKind::Arrow && p.member == 0);
            arrows != tails || (mid_path && link.order != Order::Ordered)
        })
    })
}
