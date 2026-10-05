//! The grid picker through a Phase 6 camera.

use joinn_frame::Verdict;

use super::cpu_pick_at::cpu_pick_at;
use super::{PickImage, Shape};
use crate::camera::{Camera, FitCamera};

/// `cpu_pick_at` through the exact camera the fit camera converts to.
pub fn cpu_pick(shapes: &[Shape], camera: &FitCamera) -> Verdict<PickImage> {
    match Camera::from_fit(camera) {
        Verdict::Ok(c) => cpu_pick_at(shapes, &c),
        Verdict::Refused(r) => Verdict::Refused(r),
    }
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use std::collections::BTreeSet;

    use super::cpu_pick;
    use crate::camera::{STANDARD_VIEWPORTS, fit};
    use crate::fixtures::calculator;
    use crate::layout::layout;
    use crate::pick::{Pick, cpu_pick_reference, shapes_of_layout};

    #[test]
    fn the_grid_and_the_reference_agree_on_every_pixel_of_the_calculator() {
        let (body, cells) = calculator();
        let l = match layout(&body, &cells) {
            Verdict::Ok(l) => l,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let shapes = shapes_of_layout(&l);
        for (w, h) in STANDARD_VIEWPORTS {
            let camera = fit(&l, w, h);
            let grid = match cpu_pick(&shapes, &camera) {
                Verdict::Ok(p) => p,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            let reference = match cpu_pick_reference(&shapes, &camera) {
                Verdict::Ok(p) => p,
                Verdict::Refused(r) => panic!("{}", r.reason),
            };
            let differ = grid
                .pixels
                .iter()
                .zip(&reference.pixels)
                .filter(|(a, b)| a != b)
                .count();
            let edge = grid.pixels.iter().filter(|p| **p == Pick::Edge).count();
            let owners: BTreeSet<[u32; 4]> = grid
                .pixels
                .iter()
                .filter_map(|p| match p {
                    Pick::Owned(id) => Some(*id),
                    _ => None,
                })
                .collect();
            println!(
                "calculator {w}x{h}: agree {}, edge {edge}, disagree {differ}, owners {}/13",
                grid.pixels.len() - differ,
                owners.len()
            );
            assert_eq!(grid.pixels.len(), (w * h) as usize);
            assert_eq!(
                differ, 0,
                "{w}x{h}: the pickers disagree on {differ} pixel(s)"
            );
            assert_eq!(owners.len(), 13, "{w}x{h}: every owner owns a pixel");
        }
    }

    #[test]
    fn a_moved_port_makes_the_pickers_disagree() {
        let (body, cells) = calculator();
        let l = match layout(&body, &cells) {
            Verdict::Ok(l) => l,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let camera = fit(&l, 640, 360);
        let shapes = shapes_of_layout(&l);
        let mut moved = shapes.clone();
        if let Some(last) = moved.last_mut() {
            if let crate::pick::Geom::Circle { y, .. } = &mut last.geom {
                *y += 1;
            }
        }
        let (Verdict::Ok(a), Verdict::Ok(b)) = (
            cpu_pick(&moved, &camera),
            cpu_pick_reference(&shapes, &camera),
        ) else {
            panic!("both pickers must run");
        };
        assert!(a.pixels.iter().zip(&b.pixels).any(|(x, y)| x != y));
    }
}
