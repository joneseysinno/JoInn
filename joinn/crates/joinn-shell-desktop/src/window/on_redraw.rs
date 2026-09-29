//! A redraw requested by a resize, a click, a key that ran, or a lost surface.

use super::ShellApp;

impl ShellApp {
    /// wgpu's device-lost flag is read first and regrows the device.
    /// `Lost` and `Outdated` surfaces only reconfigure.
    pub(super) fn on_redraw(&mut self) {
        if let Some(reason) = self.gpu.as_ref().and_then(|gpu| gpu.lost()) {
            if self.regrow(reason) {
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            return;
        }
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
                self.paint(frame);
            }
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                if self.reconfigure() {
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
            }
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => {}
        }
    }
}
