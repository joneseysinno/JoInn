//! One redraw: upload what changed, draw, print the tick, confirm a click.

use joinn_frame::Verdict;
use joinn_gpu::{Upload, read_texel};

use super::ShellApp;
use super::emit::emit;
use crate::session::{gpu_agree, tick_line};

impl ShellApp {
    /// `tick: level <L> step <t> (k <fraction>), anchor <chart>, rows <n>`.
    /// A pending click then prints the GPU owner.
    pub(super) fn paint(&mut self, frame: wgpu::SurfaceTexture) {
        let Some(tick) = self.shell.take_tick() else {
            return;
        };
        let Some(mut renderer) = self.renderer.take() else {
            return;
        };
        let Some(gpu) = self.gpu.take() else {
            self.renderer = Some(renderer);
            return;
        };
        let camera = self.shell.camera();
        let confirm = self.shell.take_confirm();
        let upload = if let Some(upload) = self.forced.take() {
            upload
        } else if !self.uploaded {
            match renderer.upload_all(&gpu, self.shell.tables()) {
                Verdict::Ok(upload) => {
                    self.uploaded = true;
                    upload
                }
                Verdict::Refused(r) => {
                    eprintln!("joinn-desktop: {}", r.reason);
                    self.gpu = Some(gpu);
                    self.renderer = Some(renderer);
                    return;
                }
            }
        } else if let Some(delta) = &tick.delta {
            match renderer.apply(&gpu, self.shell.tables(), delta) {
                Verdict::Ok(upload) => upload,
                Verdict::Refused(r) => {
                    eprintln!("joinn-desktop: {}", r.reason);
                    self.gpu = Some(gpu);
                    self.renderer = Some(renderer);
                    return;
                }
            }
        } else {
            Upload { rows: 0, bytes: 0 }
        };
        let drawn = {
            let view = frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            renderer.draw_at(&gpu, &camera, &view)
        };
        if let Verdict::Refused(r) = drawn {
            eprintln!("joinn-desktop: {}", r.reason);
            self.gpu = Some(gpu);
            self.renderer = Some(renderer);
            return;
        }
        gpu.queue().present(frame);
        emit(&tick_line(&tick, upload.rows));
        if let Some(click) = confirm {
            let gpu_owner = match renderer.id_texture() {
                Some(ids) => match read_texel(&gpu, ids, click.x, click.y) {
                    Verdict::Ok(id) => self.shell.owner_line(id),
                    Verdict::Refused(r) => r.reason,
                },
                None => "no ID target".to_owned(),
            };
            emit(&gpu_agree(click.x, click.y, &click.cpu, &gpu_owner));
        }
        self.gpu = Some(gpu);
        self.renderer = Some(renderer);
    }
}
