//! The camera lives in one cell. Zooming is how you travel into it and back out.
//! Crossing a boundary re-bases the camera so the numbers stay ordinary.

use workspace_model::{CellId, Worksheet};

use crate::layout::{self, find};

/// A child must cover this much of the window before it becomes the current cell.
const ENTER: f64 = 0.72;
/// The current cell must shrink below this before its parent takes over.
/// The gap between the two stops the view flickering.
const LEAVE: f64 = 0.42;

#[derive(Clone, Copy, Debug)]
pub struct Viewport {
    pub w: f64,
    pub h: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub cell: Option<CellId>,
    pub scale: f64,
    pub pan_x: f64,
    pub pan_y: f64,
}

impl Camera {
    pub fn identity() -> Self {
        Self {
            cell: None,
            scale: 1.0,
            pan_x: 32.0,
            pan_y: 32.0,
        }
    }
}

/// Wheel zoom about a cursor, in the same pixel space as `pan`.
pub fn zoom_about(cam: Camera, cursor_x: f64, cursor_y: f64, factor: f64) -> Camera {
    if !(factor.is_finite() && factor > 0.0) || cam.scale <= 0.0 {
        return cam;
    }
    let world_x = (cursor_x - cam.pan_x) / cam.scale;
    let world_y = (cursor_y - cam.pan_y) / cam.scale;
    let scale = (cam.scale * factor).clamp(1.0e-4, 1.0e6);
    Camera {
        cell: cam.cell,
        scale,
        pan_x: cursor_x - world_x * scale,
        pan_y: cursor_y - world_y * scale,
    }
}

/// Frame one cell so the whole of it sits in the window.
pub fn frame_cell(sheet: &Worksheet, id: CellId, viewport: Viewport) -> Camera {
    let Some(laid) = layout::layout_cell(sheet, id, 1.0, 2) else {
        return Camera::identity();
    };
    let fit_w = if laid.w > 1.0 { viewport.w / laid.w } else { 1.0 };
    let fit_h = if laid.h > 1.0 { viewport.h / laid.h } else { 1.0 };
    let scale = (fit_w.min(fit_h) * 0.88).clamp(0.05, 8.0);
    Camera {
        cell: Some(id),
        scale,
        pan_x: (viewport.w - laid.w * scale) / 2.0,
        pan_y: (viewport.h - laid.h * scale) / 2.0,
    }
}

/// Enter or leave a group when it fills the window or shrinks away. One step.
pub fn settle(cam: Camera, sheet: &Worksheet, viewport: Viewport) -> Camera {
    let mut cam = cam;
    for _ in 0..6 {
        let next = settle_once(cam, sheet, viewport);
        let same_cell = next.cell == cam.cell;
        let same_pan = (next.pan_x - cam.pan_x).abs() < 1e-6 && (next.pan_y - cam.pan_y).abs() < 1e-6;
        cam = next;
        if same_cell && same_pan {
            break;
        }
    }
    cam
}

fn settle_once(cam: Camera, sheet: &Worksheet, viewport: Viewport) -> Camera {
    let Some(cell) = cam.cell.or_else(|| sheet.root()) else {
        return cam;
    };
    if sheet.cell(cell).is_none() {
        return Camera {
            cell: sheet.root(),
            ..cam
        };
    }
    let Some(tree) = layout::layout_cell(sheet, cell, cam.scale, 2) else {
        return cam;
    };
    let screen_w = tree.w * cam.scale;
    let screen_h = tree.h * cam.scale;
    if let Some(parent) = sheet.parent_of(cell) {
        if screen_w < viewport.w * LEAVE && screen_h < viewport.h * LEAVE {
            if let Some(parent_tree) = layout::layout_cell(sheet, parent, cam.scale, 2) {
                if let Some(node) = find(&parent_tree, cell) {
                    return Camera {
                        cell: Some(parent),
                        scale: cam.scale,
                        pan_x: cam.pan_x - node.x * cam.scale,
                        pan_y: cam.pan_y - node.y * cam.scale,
                    };
                }
            }
        }
    }
    let center_x = (viewport.w * 0.5 - cam.pan_x) / cam.scale;
    let center_y = (viewport.h * 0.5 - cam.pan_y) / cam.scale;
    for child in &tree.children {
        let group = sheet.cell(child.id).is_some_and(|cell| cell.inside().is_group());
        if !group {
            continue;
        }
        let sw = child.w * cam.scale;
        let sh = child.h * cam.scale;
        let covers = sw >= viewport.w * ENTER && sh >= viewport.h * ENTER;
        let holds_center = center_x >= child.x
            && center_y >= child.y
            && center_x <= child.x + child.w
            && center_y <= child.y + child.h;
        if covers && holds_center {
            return Camera {
                cell: Some(child.id),
                scale: cam.scale,
                pan_x: cam.pan_x + child.x * cam.scale,
                pan_y: cam.pan_y + child.y * cam.scale,
            };
        }
    }
    if cam.cell.is_none() {
        return Camera {
            cell: Some(cell),
            ..cam
        };
    }
    cam
}

/// Where a content point lands on the canvas.
pub fn screen_of(cam: Camera, world_x: f64, world_y: f64) -> (f64, f64) {
    (
        cam.pan_x + world_x * cam.scale,
        cam.pan_y + world_y * cam.scale,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use workspace_model::{Arrangement, Inside, NewCell, Worksheet};

    fn group(name: &str) -> NewCell {
        NewCell {
            name: name.to_string(),
            inside: Inside::Group {
                arrangement: Arrangement::Stack,
                children: Vec::new(),
            },
            x: 0,
            y: 0,
            show_on_face: false,
        }
    }

    #[test]
    fn zoom_four_deep_and_back_returns_to_the_same_view() {
        let mut sheet = Worksheet::empty();
        let root = sheet.add(None, group("Root")).unwrap();
        let a = sheet.add(Some(root), group("A")).unwrap();
        let b = sheet.add(Some(a), group("B")).unwrap();
        let c = sheet.add(Some(b), group("C")).unwrap();
        let deep = sheet.add(Some(c), group("D")).unwrap();
        let viewport = Viewport { w: 1000.0, h: 700.0 };
        let start = frame_cell(&sheet, root, viewport);
        let mut cam = start;
        let mut saw_deep = false;
        let mut aimed = (0.0, 0.0);
        for _ in 0..80 {
            let Some(cell) = cam.cell else { break };
            let crumbs = sheet.path_to(cell);
            assert_eq!(crumbs.first().copied(), Some(root));
            assert_eq!(*crumbs.last().unwrap(), cell);
            if cell == deep {
                saw_deep = true;
                assert_eq!(crumbs, vec![root, a, b, c, deep]);
                break;
            }
            let full = layout::layout_cell(&sheet, cell, 1.0, 8).unwrap();
            let aim = layout::find(&full, deep).expect("deep cell stays inside the current one");
            let world = (aim.x + aim.w / 2.0, aim.y + aim.h / 2.0);
            let (sx, sy) = screen_of(cam, world.0, world.1);
            cam = settle(zoom_about(cam, sx, sy, 1.15), &sheet, viewport);
            let landed = cam.cell.expect("still in a cell");
            let full = layout::layout_cell(&sheet, landed, 1.0, 8).unwrap();
            let aim = layout::find(&full, deep).expect("deep cell after the boundary");
            let (ax, ay) = screen_of(cam, aim.x + aim.w / 2.0, aim.y + aim.h / 2.0);
            assert!(
                (ax - sx).abs() < 1.5 && (ay - sy).abs() < 1.5,
                "the view jumped: ({sx}, {sy}) -> ({ax}, {ay})"
            );
            aimed = (sx, sy);
        }
        assert!(saw_deep, "zooming in reaches the cell four deep");
        let mut back = cam;
        for _ in 0..80 {
            let Some(cell) = back.cell else { break };
            let crumbs = sheet.path_to(cell);
            assert_eq!(crumbs.first().copied(), Some(root));
            assert_eq!(*crumbs.last().unwrap(), cell);
            if cell == root {
                break;
            }
            let full = layout::layout_cell(&sheet, cell, 1.0, 8).unwrap();
            let aim = layout::find(&full, deep).unwrap_or(&full);
            let (sx, sy) = screen_of(back, aim.x + aim.w / 2.0, aim.y + aim.h / 2.0);
            back = settle(zoom_about(back, sx, sy, 1.0 / 1.15), &sheet, viewport);
            let landed = back.cell.expect("still in a cell");
            let full = layout::layout_cell(&sheet, landed, 1.0, 8).unwrap();
            let aim = layout::find(&full, deep).unwrap_or(&full);
            let (ax, ay) = screen_of(back, aim.x + aim.w / 2.0, aim.y + aim.h / 2.0);
            assert!(
                (ax - sx).abs() < 1.5 && (ay - sy).abs() < 1.5,
                "zooming out jumped: ({sx}, {sy}) -> ({ax}, {ay})"
            );
        }
        assert_eq!(back.cell, Some(root));
        let full = layout::layout_cell(&sheet, root, 1.0, 8).unwrap();
        let aim = layout::find(&full, deep).unwrap();
        let (ax, ay) = screen_of(back, aim.x + aim.w / 2.0, aim.y + aim.h / 2.0);
        assert!(
            ax > 0.0 && ay > 0.0 && ax < viewport.w && ay < viewport.h,
            "back at the root, the cell is still in the window at ({ax}, {ay}), last aim {aimed:?}"
        );
        let again = zoom_about(zoom_about(start, 120.0, 80.0, 1.15), 120.0, 80.0, 1.0 / 1.15);
        assert!((again.scale - start.scale).abs() < 1e-9);
        assert!((again.pan_x - start.pan_x).abs() < 1e-6);
        assert!((again.pan_y - start.pan_y).abs() < 1e-6);
    }
}
