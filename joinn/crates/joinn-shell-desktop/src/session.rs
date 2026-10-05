//! The shell's host: clicks, typing, zoom and pan, and the scene. No window.

mod apply_camera;
mod atlas_view;
mod click;
#[cfg(test)]
mod contact_fixture;
mod desktop_view;
mod drag_to;
mod enter;
mod escape;
#[cfg(test)]
mod fixtures;
mod gpu_agree;
mod home;
mod host;
mod key;
mod new_shell;
mod open;
mod open_atlas;
mod owner_line;
mod press;
mod release;
mod resize;
mod take_confirm;
mod take_tick;
mod tick_line;
mod type_backspace;
mod type_char;
mod wheel;

pub(crate) use gpu_agree::gpu_agree;
pub use tick_line::tick_line;

use std::collections::{BTreeMap, BTreeSet};

use joinn_dna::{Body, Cell};
use joinn_frame::{Hash, Verdict};
use joinn_host::{Address, Signals};
use joinn_live::BodyState;
use joinn_visual::{Camera, Delta, Scene, Tables, UniverseScene, Zoom};

/// What Enter prints on an empty buffer: no intent, no run, no tick.
pub const NOTHING_SENT: &str = "  (empty: nothing sent)";

/// Whole pixels a press must move before release to be a drag, not a click.
pub const DRAG_PIXELS: i64 = 4;

/// Whole pixels one arrow key pans.
pub const ARROW_PIXELS: i64 = 64;

/// A click waiting for the ID target to name the same owner.
pub struct Confirm {
    /// Pixel x.
    pub x: u32,
    /// Pixel y.
    pub y: u32,
    /// What the CPU pick printed.
    pub cpu: String,
}

/// What a window shows: a body or a universe lens, its tables, and its camera.
pub trait View {
    /// The camera the last input wrote.
    fn camera(&self) -> Camera;
    /// Take `camera`, rebased onto the scene's anchor. A rebase leaves its
    /// chart rows pending; a pan or zoom leaves none.
    fn set_camera(&mut self, camera: Camera) -> Verdict<()>;
    /// The camera that frames the whole scene in `width` × `height`.
    fn home(&self, width: u32, height: u32) -> Verdict<Camera>;
    /// The name of the chart the camera is anchored in.
    fn anchor_name(&self) -> String;
    /// The tables as they stand.
    fn tables(&self) -> &Tables;
    /// Rows changed since the last draw. `None`: nothing to draw.
    fn take_pending(&mut self) -> Option<Delta>;
    /// The CPU pick at a pixel, and what selecting it printed.
    fn click(&mut self, x: u32, y: u32) -> Vec<String>;
    /// The click waiting for the GPU's confirmation.
    fn take_confirm(&mut self) -> Option<Confirm>;
    /// The owner an ID texel names, printed.
    fn owner_line(&self, id: [u32; 4]) -> String;
    /// True while an in-port is selected: characters type into it.
    fn typing(&self) -> bool;
    /// Enter: run what was typed.
    fn enter(&mut self) -> Vec<String>;
    /// Backspace in the typing buffer.
    fn backspace(&mut self) -> Option<String>;
    /// A character into the typing buffer.
    fn type_char(&mut self, text: &str) -> Option<String>;
    /// Clear the selection.
    fn escape(&mut self);
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

/// One lens of a universe, its camera, and the click waiting to be confirmed.
/// Nothing runs: a universe is looked at, zoomed and picked.
pub struct Atlas {
    scene: UniverseScene,
    camera: Camera,
    confirm: Option<Confirm>,
}

/// A key the shell acts on, named without the window's key type.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Key {
    /// Printed text: `+`, `-` and `F` drive the camera unless a port is selected.
    Text(String),
    /// Enter.
    Enter,
    /// Backspace.
    Backspace,
    /// Escape.
    Escape,
    /// Arrow left.
    Left,
    /// Arrow right.
    Right,
    /// Arrow up.
    Up,
    /// Arrow down.
    Down,
}

/// What one tick draws: the zoom and anchor it prints, and the rows it uploads.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Tick {
    /// The camera's zoom.
    pub zoom: Zoom,
    /// The anchor chart's name.
    pub anchor: String,
    /// Rows written since the last tick. `None`: none.
    pub delta: Option<Delta>,
}

/// The inputs a window delivers, turned into camera changes and picks. A tick
/// is owed only after an input changed something (V147).
pub struct Shell {
    view: Box<dyn View>,
    framed: bool,
    dirty: bool,
    press: Option<(i64, i64)>,
    last: (i64, i64),
    dragging: bool,
}

impl Shell {
    /// The camera to draw through.
    pub fn camera(&self) -> Camera {
        self.view.camera()
    }

    /// The tables as they stand.
    pub fn tables(&self) -> &Tables {
        self.view.tables()
    }

    /// True when an input changed something since the last tick.
    pub fn dirty(&self) -> bool {
        self.dirty
    }

    /// The click waiting for the GPU's confirmation.
    pub fn take_confirm(&mut self) -> Option<Confirm> {
        self.view.take_confirm()
    }

    /// The owner an ID texel names, printed.
    pub fn owner_line(&self, id: [u32; 4]) -> String {
        self.view.owner_line(id)
    }
}
