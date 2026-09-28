//! The two organelles' pipelines: shape and curve.

use super::ID_FORMAT;
use super::shader_source::shader_source;

const SHAPE: &str = include_str!("../wgsl/shape.wgsl");
const CURVE: &str = include_str!("../wgsl/curve.wgsl");

/// Both pipelines share one layout and write the color and ID targets, with no
/// blending and no multisampling: an integer ID target cannot be blended.
pub(crate) fn pipelines(
    device: &wgpu::Device,
    layouts: &[wgpu::BindGroupLayout; 4],
    format: wgpu::TextureFormat,
) -> (wgpu::RenderPipeline, wgpu::RenderPipeline) {
    let refs: Vec<Option<&wgpu::BindGroupLayout>> = layouts.iter().map(Some).collect();
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("tables"),
        bind_group_layouts: &refs,
        immediate_size: 0,
    });
    let targets = [Some(format.into()), Some(ID_FORMAT.into())];
    let build = |label: &str, organelle: &str| {
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(shader_source(organelle).into()),
        });
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
    (build("shape", SHAPE), build("curve", CURVE))
}
