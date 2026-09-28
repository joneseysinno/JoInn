//! The brute-force picker: every shape, every pixel. It exists to be agreed with.

use joinn_frame::Verdict;

use super::class_at::class_at;
use super::geom16::geom16;
use super::{Class, Pick, PickImage, Shape};
use crate::camera::Camera;
use crate::refuse::refuse;

/// `shapes` are in draw order. Each pixel is walked in reverse draw order: the
/// first shape that holds it inside owns it, the first that has it on its edge
/// makes it an edge pixel, and a pixel every shape leaves outside is background.
pub fn cpu_pick_reference(shapes: &[Shape], camera: &Camera) -> Verdict<PickImage> {
    let mut geoms = Vec::with_capacity(shapes.len());
    for shape in shapes {
        match geom16(shape, camera) {
            Some(g) => geoms.push(g),
            None => {
                return refuse(format!(
                    "pick: shape {:?} overflows i128 under the camera; acceptance is a shape whose pixel geometry fits",
                    shape.id
                ));
            }
        }
    }
    let mut pixels = Vec::with_capacity(camera.width as usize * camera.height as usize);
    for y in 0..i128::from(camera.height) {
        for x in 0..i128::from(camera.width) {
            let p = (16 * x + 8, 16 * y + 8);
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
            pixels.push(pick);
        }
    }
    Verdict::Ok(PickImage {
        width: camera.width,
        height: camera.height,
        pixels,
    })
}
