//! The shell's host: clicks, typing, and the scene. No window.

mod click;
#[cfg(test)]
mod contact_fixture;
mod enter;
mod escape;
mod fit_viewport;
#[cfg(test)]
mod fixtures;
mod gpu_agree;
mod host;
mod open;
mod owner_line;
mod take_confirm;
mod type_backspace;
mod type_char;

pub(crate) use gpu_agree::gpu_agree;

use std::collections::{BTreeMap, BTreeSet};

use joinn_dna::{Body, Cell};
use joinn_frame::Hash;
use joinn_host::{Address, Signals};
use joinn_live::BodyState;
use joinn_visual::{Camera, Delta, Scene, Tables};

/// What Enter prints on an empty buffer: no intent, no run, no tick.
pub const NOTHING_SENT: &str = "  (empty: nothing sent)";

/// A click waiting for the ID target to name the same owner.
pub struct Confirm {
    /// Pixel x.
    pub x: u32,
    /// Pixel y.
    pub y: u32,
    /// What the CPU pick printed.
    pub cpu: String,
}

/// One body, its live state, and the picture of it.
pub struct Desktop {
    body: Body,
    cells: BTreeMap<Hash, Cell>,
    scene: Scene,
    state: BodyState,
    camera: Camera,
    intents: BTreeSet<Address>,
    selected: Option<Address>,
    buffer: String,
    epoch: u64,
    signals: Signals,
    confirm: Option<Confirm>,
}

impl Desktop {
    /// The camera the last fit wrote.
    pub fn camera(&self) -> Camera {
        self.camera
    }

    /// The tables as they stand.
    pub fn tables(&self) -> &Tables {
        self.scene.tables()
    }

    /// Rows changed since the last draw. `None`: nothing to draw.
    pub fn take_pending(&mut self) -> Option<Delta> {
        self.scene.take_pending()
    }
}
