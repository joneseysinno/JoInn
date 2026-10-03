//! Render tables offscreen through a camera and read both targets back.

use joinn_frame::Verdict;
use joinn_visual::{FitCamera, Tables};

use crate::gpu::Gpu;
use crate::renderer::{OFFSCREEN_FORMAT, Picture, Renderer};

/// A fresh renderer, every table uploaded, one draw at the camera's viewport.
pub fn render_offscreen(gpu: &Gpu, tables: &Tables, camera: &FitCamera) -> Verdict<Picture> {
    let mut renderer = match Renderer::new(gpu, OFFSCREEN_FORMAT) {
        Verdict::Ok(r) => r,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    if let Verdict::Refused(r) = renderer.upload_all(gpu, tables) {
        return Verdict::Refused(r);
    }
    renderer.picture(gpu, camera)
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use joinn_visual::{Scene, fit};

    use super::render_offscreen;
    use crate::adapter::adapters;
    use crate::fixtures::calculator;
    use crate::gpu::open;

    #[test]
    fn the_calculator_at_640x360_names_every_probe_owner_on_every_adapter() {
        let (body, cells) = calculator();
        let scene = match Scene::grow("body", &body, &cells) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let camera = fit(scene.layout(), 640, 360);
        assert_eq!((camera.k, camera.ox, camera.oy), (12, 80, 36));
        let probes: [((i64, i64), &str); 13] = [
            ((2, 2), "body surface"),
            ((10, 7), "body.cli_a"),
            ((10, 17), "body.cli_b"),
            ((30, 9), "body.sum"),
            ((4, 7), "body.cli_a@0"),
            ((16, 7), "body.cli_a@1"),
            ((4, 17), "body.cli_b@0"),
            ((16, 17), "body.cli_b@1"),
            ((24, 7), "body.sum@0"),
            ((24, 11), "body.sum@1"),
            ((36, 7), "body.sum@2"),
            ((20, 7), "body wire cli_a@1 -> sum@0"),
            ((20, 14), "body wire cli_b@1 -> sum@1"),
        ];
        let all = match adapters() {
            Verdict::Ok(a) => a,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        for adapter in &all {
            let gpu = match open(adapter) {
                Verdict::Ok(g) => g,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            let picture = match render_offscreen(&gpu, scene.tables(), &camera) {
                Verdict::Ok(p) => p,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            assert_eq!(picture.ids.len(), 640 * 360);
            assert_eq!(picture.color.len(), 640 * 360 * 4);
            let named = |x: i64, y: i64| -> String {
                let id = picture.ids[(y * 640 + x) as usize];
                match scene.resolve(id) {
                    Verdict::Ok(o) => scene.print_owner(&o),
                    Verdict::Refused(r) => panic!("{}: {}", adapter.line(), r.reason),
                }
            };
            for ((u, v), want) in probes {
                let (x, y) = (camera.ox + camera.k * u, camera.oy + camera.k * v);
                assert_eq!(named(x, y), want, "{}: pixel {x},{y}", adapter.line());
            }
            assert_eq!(named(0, 0), "background", "{}", adapter.line());
            println!("{}: 14 probes at 640x360 name their owners", adapter.line());
        }
    }
}
