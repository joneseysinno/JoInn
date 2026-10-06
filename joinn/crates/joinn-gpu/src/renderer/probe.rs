//! One of the shader's own integer decisions, evaluated on the GPU at chosen
//! sizes and zooms.

use std::num::NonZeroU64;

use joinn_frame::Verdict;

use super::TICK_BYTES;
use super::read_target::read_target;
use super::shader_source::shader_source;
use super::target::target;
use crate::gpu::{Gpu, scoped};

const PROBE: &str = include_str!("../wgsl/band_probe.wgsl");

/// For each `(size, level, step)`, what `band_probe.wgsl` returns for
/// `decision` (0 `band(size)`, 1 `form_owner(size)`, 2 `fold_now()`) under a
/// tick of that zoom: one draw into a 1×1 `R32Uint` target per probe, read
/// back as bytes.
pub(super) fn probe(gpu: &Gpu, decision: u32, probes: &[(u32, i32, u32)]) -> Verdict<Vec<u32>> {
    let device = gpu.device();
    let built = scoped(gpu, "building the band probe", || {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("0 tick"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(TICK_BYTES as u64),
                },
                count: None,
            }],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("band probe"),
            bind_group_layouts: &[Some(&layout)],
            immediate_size: 0,
        });
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("band probe"),
            source: wgpu::ShaderSource::Wgsl(shader_source(PROBE, false).into()),
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("band probe"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: Some("vs"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: Some("fs"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::R32Uint,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });
        let tick = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("band probe tick"),
            size: TICK_BYTES as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("band probe tick"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: tick.as_entire_binding(),
            }],
        });
        let texture = target(device, wgpu::TextureFormat::R32Uint, 1, 1);
        (pipeline, tick, group, texture)
    });
    let (pipeline, tick, group, texture) = match built {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut answers = Vec::new();
    for &(size, level, step) in probes {
        let words: [u32; 12] = [
            u32::from_le_bytes(level.to_le_bytes()),
            step,
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            1,
            size,
            decision,
        ];
        let drawn = scoped(gpu, "drawing the band probe", || {
            gpu.queue()
                .write_buffer(&tick, 0, bytemuck::cast_slice(&words[..]));
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("band probe"),
            });
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("band probe"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 9.0,
                                g: 0.0,
                                b: 0.0,
                                a: 0.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });
                pass.set_pipeline(&pipeline);
                pass.set_bind_group(0, &group, &[]);
                pass.draw(0..3, 0..1);
            }
            gpu.queue().submit([encoder.finish()]);
        });
        if let Verdict::Refused(r) = drawn {
            return Verdict::Refused(r);
        }
        match read_target(gpu, &texture, 4) {
            Verdict::Ok(bytes) => match <[u8; 4]>::try_from(bytes.as_slice()) {
                Ok(b) => answers.push(u32::from_le_bytes(b)),
                Err(_) => {
                    return crate::refuse::refuse(format!(
                        "band probe: read {} bytes; acceptance is 4",
                        bytes.len()
                    ));
                }
            },
            Verdict::Refused(r) => return Verdict::Refused(r),
        }
    }
    Verdict::Ok(answers)
}
