//! `cargo xtask regrow`: tables built by deltas equal tables regrown from DNA and
//! live state (V122), and GPU state dropped and regrown draws the same bytes (VH2).

mod cleared_elsewhere;
mod contact_pass;
mod drive;
mod event;
mod grove_pass;
mod run;
mod system_pass;

pub(crate) use cleared_elsewhere::cleared_elsewhere;
pub(crate) use event::event;
pub(crate) use grove_pass::grove_pass;
pub(crate) use run::regrow;

use std::collections::BTreeMap;

use joinn_dna::{Body, Cell, Contact};
use joinn_frame::{Hash, Verdict};
use joinn_live::BodyState;
use joinn_visual::Scene;

/// One scripted input: the instance whose port 0 it types into, and the text.
pub(crate) type Input = (&'static str, &'static str);

/// What a scene draws. The live state always runs a wired body: a contact's is
/// its lowered body, whose instances are the contact's cells.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Form<'a> {
    /// The wired body itself.
    Wired,
    /// The contact, drawn as cells in contact (R80).
    Contact(&'a Contact),
}

impl Form<'_> {
    /// `Scene::grow`, or `Scene::grow_contact`.
    pub(crate) fn grow(self, body: &Body, cells: &BTreeMap<Hash, Cell>) -> Verdict<Scene> {
        match self {
            Form::Wired => Scene::grow("body", body, cells),
            Form::Contact(c) => Scene::grow_contact("body", c, cells),
        }
    }

    /// `Scene::regrow`, or `Scene::regrow_contact`.
    pub(crate) fn regrow(
        self,
        body: &Body,
        cells: &BTreeMap<Hash, Cell>,
        state: &BodyState,
    ) -> Verdict<Scene> {
        match self {
            Form::Wired => Scene::regrow("body", body, cells, state),
            Form::Contact(c) => Scene::regrow_contact("body", c, cells, state),
        }
    }

    /// Printed before `event` and `regrow` lines.
    pub(crate) fn prefix(self) -> &'static str {
        match self {
            Form::Wired => "",
            Form::Contact(_) => "contact ",
        }
    }
}
