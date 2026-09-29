//! Load the body and run the window until it closes.

use std::path::Path;

use joinn_frame::Verdict;
use joinn_gpu::adapters;
use winit::event_loop::{ControlFlow, EventLoop};

use super::ShellApp;
use super::emit::emit;
use super::pick_adapter::pick_adapter;
use crate::load::{find_corpus, load_body_file, load_cells};
use crate::session::Desktop;

/// `adapter: <line>`, then `surface: <format>`, then one tick per redraw.
pub fn run(path: &str) -> Result<(), String> {
    let corpus = find_corpus()?;
    let body = load_body_file(Path::new(path))?;
    let cells = load_cells(&corpus)?;
    let desktop = Desktop::open(body, cells)?;
    let list = match adapters() {
        Verdict::Ok(list) => list,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let adapter = pick_adapter(list)?;
    emit(&format!("adapter: {}", adapter.line()));
    let event_loop = EventLoop::new().map_err(|e| e.to_string())?;
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = ShellApp {
        desktop,
        adapter,
        gpu: None,
        renderer: None,
        window: None,
        surface: None,
        format: wgpu::TextureFormat::Rgba8Unorm,
        tick: 0,
        uploaded: false,
        cursor: None,
        configured: false,
        tried_lost: false,
        forced: None,
    };
    event_loop.run_app(&mut app).map_err(|e| e.to_string())
}
