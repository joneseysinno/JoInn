//! The event loop. `ControlFlow::Wait`: a redraw is requested, never polled.

use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

use super::ShellApp;

impl ApplicationHandler for ShellApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.resume(event_loop);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(_) if self.reconfigure() => {
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::CursorMoved { position, .. } => self.on_moved(position.x, position.y),
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => match state {
                ElementState::Pressed => self.on_press(),
                ElementState::Released => self.on_release(),
            },
            WindowEvent::MouseWheel { delta, .. } => self.on_wheel(delta),
            WindowEvent::KeyboardInput { event, .. } => {
                self.on_key(event.state, event.logical_key);
            }
            WindowEvent::RedrawRequested => self.on_redraw(),
            _ => {}
        }
    }
}
