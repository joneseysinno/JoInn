//! The grid picker the shell uses: 32-pixel buckets, candidates in draw order.

use joinn_frame::Verdict;

use super::bounds16::bounds16;
use super::class_at::class_at;
use super::geom16::geom16;
use super::{Class, Pick, PickImage, Shape};
use crate::camera::FitCamera;
use crate::refuse::refuse;

/// Pixels per bucket side.
const BUCKET: usize = 32;

/// Each shape enters every bucket its bounding box, grown by one pixel, touches.
/// A pixel walks its bucket's candidates in reverse draw order, exactly as
/// `cpu_pick_reference` walks every shape.
pub fn cpu_pick(shapes: &[Shape], camera: &FitCamera) -> Verdict<PickImage> {
    let (width, height) = (camera.width as usize, camera.height as usize);
    let (across, down) = (width.div_ceil(BUCKET), height.div_ceil(BUCKET));
    let mut buckets: Vec<Vec<usize>> = vec![Vec::new(); across * down];
    let mut geoms = Vec::with_capacity(shapes.len());
    for (i, shape) in shapes.iter().enumerate() {
        let Some((g, (x0, y0, x1, y1))) =
            geom16(shape, camera).and_then(|g| Some((g, bounds16(&g)?)))
        else {
            return refuse(format!(
                "pick: shape {:?} overflows i128 under the camera; acceptance is a shape whose pixel geometry fits",
                shape.id
            ));
        };
        geoms.push(g);
        let span = |lo: i128, hi: i128, pixels: usize| -> Option<(usize, usize)> {
            let last = i128::try_from(pixels).ok()? - 1;
            let first_px = lo.div_euclid(16) - 1;
            let last_px = hi.div_euclid(16) + 1;
            if last < 0 || last_px < 0 || first_px > last {
                return None;
            }
            let a = usize::try_from(first_px.max(0)).ok()?;
            let b = usize::try_from(last_px.min(last)).ok()?;
            Some((a / BUCKET, b / BUCKET))
        };
        let (Some((bx0, bx1)), Some((by0, by1))) = (span(x0, x1, width), span(y0, y1, height))
        else {
            continue;
        };
        for by in by0..=by1 {
            for bx in bx0..=bx1 {
                if let Some(bucket) = buckets.get_mut(by * across + bx) {
                    bucket.push(i);
                }
            }
        }
    }
    let mut pixels = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            let p = (16 * x as i128 + 8, 16 * y as i128 + 8);
            let mut pick = Pick::Background;
            let candidates = buckets
                .get((y / BUCKET) * across + x / BUCKET)
                .map_or(&[][..], Vec::as_slice);
            for &i in candidates.iter().rev() {
                let (Some(shape), Some(g)) = (shapes.get(i), geoms.get(i)) else {
                    continue;
                };
                match class_at(g, p) {
                    Some(Class::Outside) => {}
                    Some(Class::Inside) => {
                        pick = Pick::Owned(shape.id);
                        break;
                    }
                    Some(Class::Edge) => {
                        pick = Pick::Edge;
                        break;
                    }
                    None => {
                        return refuse(format!(
                            "pick: shape {:?} overflows i128 at pixel {x},{y}; acceptance is a shape whose pixel geometry fits",
                            shape.id
                        ));
                    }
                }
            }
            pixels.push(pick);
        }
    }
    Verdict::Ok(PickImage {
        width: camera.width,
        height: camera.height,
        pixels,
    })
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
