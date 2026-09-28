//! The pick path: read one texel of the ID target.

use joinn_frame::Verdict;

use crate::gpu::{Gpu, read_buffer, scoped};
use crate::refuse::refuse;
use crate::renderer::ID_FORMAT;

/// Copies texel `(x, y)` into a 256-byte buffer, maps it, and decodes R G B A.
pub fn read_texel(gpu: &Gpu, ids: &wgpu::Texture, x: u32, y: u32) -> Verdict<[u32; 4]> {
    if ids.format() != ID_FORMAT {
        return refuse(format!(
            "read_texel: the texture is {:?}; acceptance is the {ID_FORMAT:?} ID target",
            ids.format()
        ));
    }
    if x >= ids.width() || y >= ids.height() {
        return refuse(format!(
            "read_texel: pixel {x},{y} is outside the {}x{} ID target; acceptance is a pixel inside it",
            ids.width(),
            ids.height()
        ));
    }
    let copied = scoped(gpu, "copying one ID texel", || {
        let buffer = gpu.device().create_buffer(&wgpu::BufferDescriptor {
            label: Some("texel"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let mut encoder = gpu
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("texel"),
            });
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: ids,
                mip_level: 0,
                origin: wgpu::Origin3d { x, y, z: 0 },
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(256),
                    rows_per_image: Some(1),
                },
            },
            wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
        );
        gpu.queue().submit([encoder.finish()]);
        buffer
    });
    let buffer = match copied {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    read_buffer(gpu, &buffer).and_then(|raw| {
        let word = |i: usize| -> Option<u32> {
            Some(u32::from_le_bytes(raw.get(i..i + 4)?.try_into().ok()?))
        };
        match (word(0), word(4), word(8), word(12)) {
            (Some(r), Some(g), Some(b), Some(a)) => Verdict::Ok([r, g, b, a]),
            _ => refuse("read_texel: the readback held fewer than 16 bytes; acceptance is one Rgba32Uint texel"),
        }
    })
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use joinn_visual::{Scene, fit};

    use super::read_texel;
    use crate::adapter::adapters;
    use crate::fixtures::calculator;
    use crate::gpu::open;
    use crate::renderer::{OFFSCREEN_FORMAT, Renderer};

    #[test]
    fn one_texel_names_the_same_owner_as_the_full_readback() {
        let (body, cells) = calculator();
        let scene = match Scene::grow("body", &body, &cells) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let camera = fit(scene.layout(), 1280, 720);
        let all = match adapters() {
            Verdict::Ok(a) => a,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        for adapter in &all {
            let gpu = match open(adapter) {
                Verdict::Ok(g) => g,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            let mut renderer = match Renderer::new(&gpu, OFFSCREEN_FORMAT) {
                Verdict::Ok(r) => r,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            assert!(matches!(
                renderer.upload_all(&gpu, scene.tables()),
                Verdict::Ok(_)
            ));
            let picture = match renderer.picture(&gpu, &camera) {
                Verdict::Ok(p) => p,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            let Some(ids) = renderer.id_texture() else {
                panic!("a drawn renderer holds its ID target");
            };
            for (x, y, want) in [
                (920, 276, "body.sum"),
                (640, 416, "body wire cli_b@1 -> sum@1"),
            ] {
                let texel = match read_texel(&gpu, ids, x, y) {
                    Verdict::Ok(t) => t,
                    Verdict::Refused(r) => panic!("{}", r.reason),
                };
                assert_eq!(texel, picture.ids[(y * 1280 + x) as usize]);
                let owner = match scene.resolve(texel) {
                    Verdict::Ok(o) => scene.print_owner(&o),
                    Verdict::Refused(r) => panic!("{}", r.reason),
                };
                assert_eq!(owner, want, "{}", adapter.line());
            }
            assert!(
                matches!(read_texel(&gpu, ids, 1280, 0), Verdict::Refused(_)),
                "a pixel outside the target is refused"
            );
        }
    }
}
