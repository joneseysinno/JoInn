//! Run a window over one view until it closes.

use joinn_frame::Verdict;
use joinn_gpu::adapters;
use winit::event_loop::{ControlFlow, EventLoop};

use super::ShellApp;
use super::emit::emit;
use super::pick_adapter::pick_adapter;
use crate::session::{Shell, View};

/// `adapter: <line>`, then the event loop: a redraw only when an input owes
/// one (V147).
pub(super) fn run_view(view: Box<dyn View>) -> Result<(), String> {
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
