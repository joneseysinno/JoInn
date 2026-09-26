//! The body at the head of each link, from the mark, not from an alias or a link id.

use joinn_link::{Mark, Universe};

/// One head body, none, or heads that name different bodies.
pub(crate) enum HeadRole {
    /// No link member is a head.
    Absent,
    /// Every head member names this body.
    One(String),
    /// Two head members name different bodies.
    Split,
}

pub(crate) fn head_role(universe: &Universe) -> HeadRole {
    let mut role: Option<String> = None;
    for link in &universe.coding.links {
        for member in &link.members {
            if member.mark != Mark::Head {
                continue;
            }
            match &role {
                None => role = Some(member.body.clone()),
                Some(found) if found != &member.body => return HeadRole::Split,
                Some(_) => {}
            }
        }
    }
    match role {
        Some(body) => HeadRole::One(body),
        None => HeadRole::Absent,
    }
}
