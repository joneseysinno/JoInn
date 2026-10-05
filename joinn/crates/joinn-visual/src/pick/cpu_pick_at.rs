//! The grid picker the shell uses: 32-pixel buckets, candidates in draw order.

use joinn_frame::Verdict;

use super::bounds_px::bounds_px;
use super::class_at::class_at;
use super::geom_px::geom_px;
use super::{Class, Pick, PickImage, Shape};
use crate::camera::Camera;
use crate::refuse::refuse;

/// Pixels per bucket side.
const BUCKET: usize = 32;

/// Each shape enters every bucket its bounding box, grown by one pixel, touches.
/// A pixel walks its bucket's candidates in reverse draw order, exactly as
/// `cpu_pick_reference_at` walks every shape.
pub fn cpu_pick_at(shapes: &[Shape], camera: &Camera) -> Verdict<PickImage> {
    let (width, height) = (camera.width as usize, camera.height as usize);
    let (across, down) = (width.div_ceil(BUCKET), height.div_ceil(BUCKET));
    let mut buckets: Vec<Vec<usize>> = vec![Vec::new(); across * down];
    let mut geoms = Vec::with_capacity(shapes.len());
    for (i, shape) in shapes.iter().enumerate() {
        let Some((g, (x0, y0, x1, y1))) =
            geom_px(shape, camera).and_then(|g| Some((g, bounds_px(&g)?)))
        else {
            return refuse(format!(
                "pick: shape {:?} overflows i128 under the camera; acceptance is a shape whose pixel geometry fits",
                shape.id
            ));
        };
        geoms.push(g);
        let span = |lo: i128, hi: i128, pixels: usize| -> Option<(usize, usize)> {
            let last = i128::try_from(pixels).ok()? - 1;
            let first_px = (lo >> 32) - 1;
            let last_px = (hi >> 32) + 1;
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
            let p = (((x as i128) << 32) + (1 << 31), ((y as i128) << 32) + (1 << 31));
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
