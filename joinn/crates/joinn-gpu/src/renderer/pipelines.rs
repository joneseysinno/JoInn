//! The two organelles' pipelines, each as an owner and a ghost.

use super::shader_source::shader_source;
use super::{ID_FORMAT, Pipelines};

const SHAPE: &str = include_str!("../wgsl/shape.wgsl");
const CURVE: &str = include_str!("../wgsl/curve.wgsl");

/// Every pipeline shares one layout and blends color by alpha, with no
/// multisampling. The ID target is never blended (an integer target cannot
/// be): the owner writes it, the ghost leaves it alone. An opaque color blends
/// to itself, so an owner drawn at full opacity writes exactly its style.
pub(crate) fn pipelines(
    device: &wgpu::Device,
    layouts: &[wgpu::BindGroupLayout; 4],
    format: wgpu::TextureFormat,
) -> Pipelines {
    let refs: Vec<Option<&wgpu::BindGroupLayout>> = layouts.iter().map(Some).collect();
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("tables"),
        bind_group_layouts: &refs,
        immediate_size: 0,
    });
    let build = |label: &str, organelle: &str, ghost: bool| {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(shader_source(organelle, ghost).into()),
        });
        let ids = if ghost {
            wgpu::ColorWrites::empty()
        } else {
            wgpu::ColorWrites::ALL
        };
        let targets = [
            Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            }),
            Some(wgpu::ColorTargetState {
                format: ID_FORMAT,
                blend: None,
                write_mask: ids,
            }),
        ];
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: Some(&layout),
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
                targets: &targets,
            }),
            multiview_mask: None,
            cache: None,
        })
    };
    Pipelines {
        shape: build("shape", SHAPE, false),
        shape_ghost: build("shape ghost", SHAPE, true),
        curve: build("curve", CURVE, false),
        curve_ghost: build("curve ghost", CURVE, true),
    }
}
