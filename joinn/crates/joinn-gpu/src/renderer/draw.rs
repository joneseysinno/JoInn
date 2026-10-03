//! Draw the tables through a camera into a color view and the ID target.

use joinn_frame::Verdict;
use joinn_visual::{FitCamera, STYLE_BACKGROUND, STYLE_TABLE};

use super::target::target;
use super::tick_bytes::tick_bytes;
use super::{ID_FORMAT, KIND_BODY, KIND_CELL, KIND_PORT, KIND_SHIFT, Renderer};
use crate::gpu::{Gpu, scoped};
use crate::refuse::refuse;

impl Renderer {
    /// Writes the tick uniform, then one pass: surfaces, cells, wires, ports,
    /// each by slot (§2.5's draw order). The color view must match the camera's
    /// viewport and this renderer's format. Submits and returns without waiting.
    pub fn draw(
        &mut self,
        gpu: &Gpu,
        camera: &FitCamera,
        color: &wgpu::TextureView,
    ) -> Verdict<()> {
        let tick = match tick_bytes(camera) {
            Verdict::Ok(t) => t,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let room = 1usize << KIND_SHIFT;
        if let Some(n) = self.rows[..4].iter().find(|n| **n > room) {
            return refuse(format!(
                "draw: a table of {n} rows exceeds the {room} slots an instance index can name; acceptance is a smaller table"
            ));
        }
        let span = |kind: u32, rows: usize| {
            let first = kind << KIND_SHIFT;
            first..first + rows as u32
        };
        let [r, g, b, a] = STYLE_TABLE
            .get(STYLE_BACKGROUND as usize)
            .map_or([0; 4], |c| c.to_le_bytes())
            .map(|c| f64::from(c) / 255.0);
        let (width, height) = (camera.width, camera.height);
        scoped(gpu, "drawing", || {
            let resize = self
                .ids
                .as_ref()
                .is_none_or(|t| t.width() != width || t.height() != height);
            if resize {
                self.ids = Some(target(gpu.device(), ID_FORMAT, width, height));
            }
            let Some(ids) = self.ids.as_ref() else {
                return;
            };
            let id_view = ids.create_view(&wgpu::TextureViewDescriptor::default());
            gpu.queue()
                .write_buffer(&self.tick, 0, bytemuck::cast_slice(&tick[..]));
            let mut encoder =
                gpu.device()
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("tick"),
                    });
            {
                let attachment = |view, clear| {
                    Some(wgpu::RenderPassColorAttachment {
                        view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(clear),
                            store: wgpu::StoreOp::Store,
                        },
                    })
                };
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("tables"),
                    color_attachments: &[
                        attachment(color, wgpu::Color { r, g, b, a }),
                        attachment(&id_view, wgpu::Color::TRANSPARENT),
                    ],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });
                for (i, group) in (0u32..).zip(&self.groups) {
                    pass.set_bind_group(i, group, &[]);
                }
                pass.set_pipeline(&self.shape);
                pass.draw(0..6, span(KIND_BODY, self.rows[0]));
                pass.draw(0..6, span(KIND_CELL, self.rows[1]));
                pass.set_pipeline(&self.curve);
                pass.draw(0..6, 0..self.rows[3] as u32);
                pass.set_pipeline(&self.shape);
                pass.draw(0..6, span(KIND_PORT, self.rows[2]));
            }
            gpu.queue().submit([encoder.finish()]);
        })
    }
}
