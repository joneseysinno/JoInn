//! Draw the tables through a camera into a color view and the ID target.

use joinn_frame::Verdict;
use joinn_visual::{Camera, STYLE_BACKGROUND, STYLE_TABLE};

use super::target::target;
use super::tick_bytes::tick_bytes;
use super::{
    ID_FORMAT, KIND_BODY, KIND_CELL, KIND_DOT, KIND_FRAME, KIND_LINK, KIND_NODE, KIND_PORT,
    KIND_SHIFT, KIND_STROKE, Renderer, SEGMENT, STYLE,
};
use crate::gpu::{Gpu, scoped};
use crate::refuse::refuse;

impl Renderer {
    /// Writes the tick uniform, then one pass by slot: open frames, link
    /// segments, lens nodes, dots, surfaces, cells, wires, ports, strokes
    /// (plan 7.3 §2.5's draw order: links under every node and body, text
    /// above). Each kind is drawn by its owner pipeline, then its ghost. The
    /// color view must match the camera's viewport and this renderer's format.
    /// Submits and returns without waiting.
    pub fn draw_at(
        &mut self,
        gpu: &Gpu,
        camera: &Camera,
        color: &wgpu::TextureView,
    ) -> Verdict<()> {
        let tick = match tick_bytes(camera, self.extent) {
            Verdict::Ok(t) => t,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let room = 1usize << KIND_SHIFT;
        if let Some((_, n)) = self
            .rows
            .iter()
            .enumerate()
            .find(|(i, n)| *i != STYLE && **n > room)
        {
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
        let p = &self.pipelines;
        let shapes = [
            (KIND_NODE, self.rows[7]),
            (KIND_DOT, self.rows[0]),
            (KIND_BODY, self.rows[0]),
            (KIND_CELL, self.rows[1]),
        ];
        let segments = u32::try_from(self.rows[SEGMENT]).unwrap_or(u32::MAX);
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
                let [g0, g1, g2, g3, links] = &self.groups;
                for (i, group) in (0u32..).zip([g0, g1, g2, g3]) {
                    pass.set_bind_group(i, group, &[]);
                }
                for pipeline in [&p.shape, &p.shape_ghost] {
                    pass.set_pipeline(pipeline);
                    pass.draw(0..6, span(KIND_FRAME, self.rows[7]));
                }
                pass.set_bind_group(1, links, &[]);
                for pipeline in [&p.segment, &p.segment_ghost] {
                    pass.set_pipeline(pipeline);
                    pass.draw(0..6, 0..segments);
                }
                pass.set_bind_group(1, g1, &[]);
                let mut both = |owner, ghost, kind, rows| {
                    for pipeline in [owner, ghost] {
                        pass.set_pipeline(pipeline);
                        pass.draw(0..6, span(kind, rows));
                    }
                };
                for (kind, rows) in shapes {
                    both(&p.shape, &p.shape_ghost, kind, rows);
                }
                both(&p.curve, &p.curve_ghost, KIND_LINK, self.rows[3]);
                both(&p.shape, &p.shape_ghost, KIND_PORT, self.rows[2]);
                both(&p.curve, &p.curve_ghost, KIND_STROKE, self.rows[8]);
            }
            gpu.queue().submit([encoder.finish()]);
        })
    }
}
