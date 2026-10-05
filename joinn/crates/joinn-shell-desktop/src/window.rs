//! The winit window. Redraws only when something changed.

mod after_input;
mod app;
mod choose_format;
mod emit;
mod on_key;
mod on_moved;
mod on_press;
mod on_redraw;
mod on_release;
mod on_wheel;
mod paint;
mod pick_adapter;
mod pixel;
mod reconfigure;
mod regrow;
mod resume;
mod run;

pub use run::run;

use std::sync::Arc;

use joinn_gpu::{Gpu, GpuAdapter, Renderer, Upload};
use winit::window::Window;

use crate::session::Shell;

pub struct ShellApp {
    shell: Shell,
    adapter: GpuAdapter,
    gpu: Option<Gpu>,
    renderer: Option<Renderer>,
    window: Option<Arc<Window>>,
    surface: Option<wgpu::Surface<'static>>,
    format: wgpu::TextureFormat,
    uploaded: bool,
    cursor: Option<(f64, f64)>,
    configured: bool,
    forced: Option<Upload>,
}
