//! Create the window, the surface, and the device once the loop has resumed.

use std::sync::Arc;

use joinn_frame::Verdict;
use joinn_gpu::{Renderer, open};
use winit::dpi::PhysicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

use super::ShellApp;
use super::choose_format::choose_format;
use super::emit::emit;

impl ShellApp {
    /// Prints `surface: <format>` after the adapter line. A later resume is ignored.
    pub(super) fn resume(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attrs = Window::default_attributes()
            .with_title("JoInn")
            .with_inner_size(PhysicalSize::new(1280, 720));
        let window = match event_loop.create_window(attrs) {
            Ok(window) => Arc::new(window),
            Err(e) => {
                eprintln!("joinn-desktop: {e}");
                event_loop.exit();
                return;
            }
        };
        let surface = match self.adapter.instance().create_surface(window.clone()) {
            Ok(surface) => surface,
            Err(e) => {
                eprintln!("joinn-desktop: {e}");
                event_loop.exit();
                return;
            }
        };
        let caps = surface.get_capabilities(self.adapter.adapter());
        let Some(format) = choose_format(&caps.formats) else {
            eprintln!(
                "joinn-desktop: the surface offers no color format; acceptance is Bgra8Unorm or Rgba8Unorm"
            );
            event_loop.exit();
            return;
        };
        emit(&format!("surface: {format:?}"));
        let gpu = match open(&self.adapter) {
            Verdict::Ok(gpu) => gpu,
            Verdict::Refused(r) => {
                eprintln!("joinn-desktop: {}", r.reason);
                event_loop.exit();
                return;
            }
        };
        let renderer = match Renderer::new(&gpu, format) {
            Verdict::Ok(renderer) => renderer,
            Verdict::Refused(r) => {
                eprintln!("joinn-desktop: {}", r.reason);
                event_loop.exit();
                return;
            }
        };
        self.format = format;
        self.gpu = Some(gpu);
        self.renderer = Some(renderer);
        self.surface = Some(surface);
        self.window = Some(window);
        if self.reconfigure() {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
    }
}
