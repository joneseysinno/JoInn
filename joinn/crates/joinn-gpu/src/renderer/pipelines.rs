//! The three organelles' pipelines, each as an owner and a ghost.

use super::shader_source::shader_source;
use super::{ID_FORMAT, Pipelines};

const SHAPE: &str = include_str!("../wgsl/shape.wgsl");
const CURVE: &str = include_str!("../wgsl/curve.wgsl");
const SEGMENT: &str = include_str!("../wgsl/segment.wgsl");

/// Shape and curve share §7.2's layout; the segment organelle's group 1 is
/// the fifth layout. Every pipeline blends color by alpha, with no
/// multisampling. The ID target is never blended (an integer target cannot
/// be): the owner writes it, the ghost leaves it alone; the segment owner
/// leaves it alone too until links are picked (P73-08). An opaque color blends
/// to itself, so an owner drawn at full opacity writes exactly its style.
pub(crate) fn pipelines(
    device: &wgpu::Device,
    layouts: &[wgpu::BindGroupLayout; 5],
    format: wgpu::TextureFormat,
) -> Pipelines {
    let [l0, l1, l2, l3, links] = layouts;
    let pipeline_layout = |label: &str, groups: [&wgpu::BindGroupLayout; 4]| {
        let refs: Vec<Option<&wgpu::BindGroupLayout>> = groups.into_iter().map(Some).collect();
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(label),
            bind_group_layouts: &refs,
            immediate_size: 0,
        })
    };
    let tables = pipeline_layout("tables", [l0, l1, l2, l3]);
    let segments = pipeline_layout("segments", [l0, links, l2, l3]);
    let build = |label: &str,
                 layout: &wgpu::PipelineLayout,
                 organelle: &str,
                 ghost: bool,
                 ids: wgpu::ColorWrites| {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(shader_source(organelle, ghost).into()),
        });
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
            layout: Some(layout),
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
    let (all, none) = (wgpu::ColorWrites::ALL, wgpu::ColorWrites::empty());
    Pipelines {
        shape: build("shape", &tables, SHAPE, false, all),
        shape_ghost: build("shape ghost", &tables, SHAPE, true, none),
        curve: build("curve", &tables, CURVE, false, all),
        curve_ghost: build("curve ghost", &tables, CURVE, true, none),
        segment: build("segment", &segments, SEGMENT, false, none),
        segment_ghost: build("segment ghost", &segments, SEGMENT, true, none),
    }
}
