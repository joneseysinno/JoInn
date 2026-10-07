//! Load the body, contact, system or universe and run the window until it closes.

use std::path::Path;

use joinn_frame::Verdict;
use joinn_gpu::adapters;
use winit::event_loop::{ControlFlow, EventLoop};

use super::ShellApp;
use super::emit::emit;
use super::pick_adapter::pick_adapter;
use crate::load::{
    find_corpus, load_body_file, load_cells, load_contact_file, load_store, load_system_file,
    load_universe_file,
};
use crate::session::{Atlas, Desktop, Grower, Shell, View};

/// `adapter: <line>`, then `surface: <format>`, then one tick per change. A
/// `.universe` opens `lens`, or its first lens in canonical order; a `.system`
/// opens at size 0 with its waiting boxes.
pub fn run(path: &str, lens: Option<&str>) -> Result<(), String> {
    let corpus = find_corpus()?;
    let cells = load_cells(&corpus)?;
    let view: Box<dyn View> = if path.ends_with(".universe") {
        let universe = load_universe_file(Path::new(path))?;
        let store = load_store(&corpus, &cells)?;
        Box::new(Atlas::open(&universe, &store, lens, 1280, 720)?)
    } else if path.ends_with(".system") {
        let (system, contacts, waiting) = load_system_file(Path::new(path))?;
        Box::new(Grower::open(&system, &contacts, &cells, waiting)?)
    } else if path.ends_with(".contact") {
        let (contact, body) = load_contact_file(Path::new(path), &cells)?;
        Box::new(Desktop::open(body, cells, Some(&contact))?)
    } else {
        Box::new(Desktop::open(
            load_body_file(Path::new(path))?,
            cells,
            None,
        )?)
    };
    let list = match adapters() {
        Verdict::Ok(list) => list,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let adapter = pick_adapter(list)?;
    emit(&format!("adapter: {}", adapter.line()));
    let event_loop = EventLoop::new().map_err(|e| e.to_string())?;
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = ShellApp {
        shell: Shell::new(view),
        adapter,
        gpu: None,
        renderer: None,
        window: None,
        surface: None,
        format: wgpu::TextureFormat::Rgba8Unorm,
        uploaded: false,
        cursor: None,
        configured: false,
        forced: None,
    };
    event_loop.run_app(&mut app).map_err(|e| e.to_string())
}
