//! A lost device: drop the renderer, open the adapter again, upload every table.

use joinn_frame::Verdict;
use joinn_gpu::{Renderer, open};

use super::ShellApp;
use super::emit::emit;

impl ShellApp {
    /// Prints `regrow: device lost (<reason>), tables re-uploaded` after the
    /// tables are back.
    pub(super) fn regrow(&mut self, reason: wgpu::DeviceLostReason) -> bool {
        self.renderer = None;
        self.gpu = None;
        self.uploaded = false;
        self.configured = false;
        let gpu = match open(&self.adapter) {
            Verdict::Ok(gpu) => gpu,
            Verdict::Refused(r) => {
                eprintln!("joinn-desktop: {}", r.reason);
                return false;
            }
        };
        let mut renderer = match Renderer::new(&gpu, self.format) {
            Verdict::Ok(renderer) => renderer,
            Verdict::Refused(r) => {
                eprintln!("joinn-desktop: {}", r.reason);
                return false;
            }
        };
        let upload = match renderer.upload_all(&gpu, self.shell.tables()) {
            Verdict::Ok(upload) => upload,
            Verdict::Refused(r) => {
                eprintln!("joinn-desktop: {}", r.reason);
                return false;
            }
        };
        self.gpu = Some(gpu);
        self.renderer = Some(renderer);
        self.uploaded = true;
        self.forced = Some(upload);
        if !self.reconfigure() {
            return false;
        }
        emit(&format!(
            "regrow: device lost ({reason:?}), tables re-uploaded"
        ));
        true
    }
}
