//! A redraw requested by a resize, a click, a key that ran, or a lost surface.

use super::ShellApp;

impl ShellApp {
    /// `Lost` reconfigures once, then regrows the device. `Outdated` reconfigures.
    pub(super) fn on_redraw(&mut self) {
        if !self.configured {
            return;
        }
        let status = {
            let Some(surface) = self.surface.as_ref() else {
                return;
            };
            surface.get_current_texture()
        };
        match status {
            wgpu::CurrentSurfaceTexture::Success(frame)
            | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => {
                self.tried_lost = false;
                self.paint(frame);
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                if self.reconfigure() {
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                if !self.tried_lost {
                    self.tried_lost = true;
                    if self.reconfigure() {
                        if let Some(window) = &self.window {
                            window.request_redraw();
                        }
                    }
                } else {
                    self.tried_lost = false;
                    if self.regrow() {
                        if let Some(window) = &self.window {
                            window.request_redraw();
                        }
                    }
                }
            }
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => {}
        }
    }
}
