//! The brute-force walk at chosen pixels: every shape, each pixel.

use joinn_frame::Verdict;

use super::class_at::class_at;
use super::geom_px::geom_px;
use super::{Class, Pick, Shape};
use crate::camera::Camera;
use crate::refuse::refuse;

/// `shapes` are in draw order. Each pixel is walked in reverse draw order: the
/// first shape that holds it inside owns it, the first that has it on its edge
/// makes it an edge pixel, and a pixel every shape leaves outside is
/// background. One pick per pixel, in the order given.
pub fn cpu_pick_sample(
    shapes: &[Shape],
    camera: &Camera,
    pixels: &[(u32, u32)],
) -> Verdict<Vec<Pick>> {
    let mut geoms = Vec::with_capacity(shapes.len());
    for shape in shapes {
        match geom_px(shape, camera) {
            Some(g) => geoms.push(g),
            None => {
                return refuse(format!(
                    "pick: shape {:?} overflows i128 under the camera; acceptance is a shape whose pixel geometry fits",
                    shape.id
                ));
            }
        }
    }
    let mut picks = Vec::with_capacity(pixels.len());
    for &(x, y) in pixels {
        let p = (
            (i128::from(x) << 32) + (1 << 31),
            (i128::from(y) << 32) + (1 << 31),
        );
        let mut pick = Pick::Background;
        for (shape, g) in shapes.iter().zip(&geoms).rev() {
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
        picks.push(pick);
    }
    Verdict::Ok(picks)
}
