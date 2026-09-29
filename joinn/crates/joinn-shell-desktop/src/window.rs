//! The winit window. Redraws only when something changed.

mod app;
mod choose_format;
mod emit;
mod on_click;
mod on_key;
mod on_redraw;
mod paint;
mod pick_adapter;
mod reconfigure;
mod regrow;
mod resume;
mod run;

pub use run::run;

use std::sync::Arc;

use joinn_gpu::{Gpu, GpuAdapter, Renderer, Upload};
use winit::window::Window;

use crate::session::Desktop;

pub struct ShellApp {
    desktop: Desktop,
    adapter: GpuAdapter,
    gpu: Option<Gpu>,
    renderer: Option<Renderer>,
    window: Option<Arc<Window>>,
    surface: Option<wgpu::Surface<'static>>,
    format: wgpu::TextureFormat,
    tick: u64,
    uploaded: bool,
    cursor: Option<(f64, f64)>,
    configured: bool,
    forced: Option<Upload>,
}
