//! Where each cell sits. Sizes are layout units; the screen multiplies by the camera scale.

use workspace_model::{Arrangement, CellId, Inside, Worksheet};

pub const LEAF_W: f64 = 220.0;
pub const LEAF_H: f64 = 76.0;
pub const HEADER: f64 = 36.0;
pub const PAD: f64 = 12.0;
pub const GAP: f64 = 10.0;

/// How big a cell is on screen decides the drawing. The gap between bands avoids flicker.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Band {
    /// A small coloured bar, no text.
    Dot,
    /// The face only.
    Face,
    /// A group may show its children.
    Open,
}

pub fn band_for(height_px: f64) -> Band {
    if height_px < 26.0 {
        Band::Dot
    } else if height_px < 68.0 {
        Band::Face
    } else {
        Band::Open
    }
}

#[derive(Clone, Debug)]
pub struct Laid {
    pub id: CellId,
    /// Top-left inside the parent, in the parent's units.
    pub local_x: f64,
    pub local_y: f64,
    /// Top-left in the root cell's units.
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub open: bool,
    pub band: Band,
    pub children: Vec<Laid>,
}

impl Laid {
    pub fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.x && y >= self.y && x <= self.x + self.w && y <= self.y + self.h
    }
}

pub fn layout(sheet: &Worksheet, scale: f64, open_depth: u32) -> Option<Laid> {
    layout_cell(sheet, sheet.root()?, scale, open_depth)
}

/// Lay a cell out in its own coordinates. `open_depth` is how many levels below it may draw children.
pub fn layout_cell(sheet: &Worksheet, id: CellId, scale: f64, open_depth: u32) -> Option<Laid> {
    if sheet.cell(id).is_none() {
        return None;
    }
    Some(place(
        sheet, id, 0.0, 0.0, 0.0, 0.0, scale, 0, open_depth,
    ))
}

fn place(
    sheet: &Worksheet,
    id: CellId,
    local_x: f64,
    local_y: f64,
    origin_x: f64,
    origin_y: f64,
    scale: f64,
    depth: u32,
    open_depth: u32,
) -> Laid {
    let x = origin_x + local_x;
    let y = origin_y + local_y;
    let Some(cell) = sheet.cell(id) else {
        return Laid {
            id,
            local_x,
            local_y,
            x,
            y,
            w: LEAF_W,
            h: LEAF_H,
            open: false,
            band: band_for(LEAF_H * scale),
            children: Vec::new(),
        };
    };
    if !cell.inside().is_group() {
        let h = LEAF_H;
        return Laid {
            id,
            local_x,
            local_y,
            x,
            y,
            w: LEAF_W,
            h,
            open: false,
            band: band_for(h * scale),
            children: Vec::new(),
        };
    }
    let arrangement = match cell.inside() {
        Inside::Group { arrangement, .. } => *arrangement,
        _ => Arrangement::Stack,
    };
    let placed = sheet.children_of(id);
    let mut children = Vec::with_capacity(placed.len());
    match arrangement {
        Arrangement::Stack => {
            let mut cursor = HEADER + PAD;
            let mut widest = LEAF_W;
            for child in placed {
                let laid = place(
                    sheet,
                    child.id,
                    PAD,
                    cursor,
                    x,
                    y,
                    scale,
                    depth + 1,
                    open_depth,
                );
                widest = widest.max(laid.w);
                cursor += laid.h + GAP;
                children.push(laid);
            }
            let h = if children.is_empty() {
                HEADER + PAD + 48.0 + PAD
            } else {
                cursor - GAP + PAD
            };
            let w = widest + PAD * 2.0;
            finish_group(id, local_x, local_y, x, y, w, h, scale, depth, open_depth, children)
        }
        Arrangement::Free => {
            let mut w: f64 = 280.0;
            let mut h: f64 = 160.0;
            for child in placed {
                let lx = child.x as f64;
                let ly = child.y as f64;
                let laid = place(sheet, child.id, lx, ly, x, y, scale, depth + 1, open_depth);
                w = w.max(laid.local_x + laid.w + PAD);
                h = h.max(laid.local_y + laid.h + PAD);
                children.push(laid);
            }
            h = h.max(HEADER + PAD);
            finish_group(id, local_x, local_y, x, y, w, h, scale, depth, open_depth, children)
        }
    }
}

fn finish_group(
    id: CellId,
    local_x: f64,
    local_y: f64,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    scale: f64,
    depth: u32,
    open_depth: u32,
    mut children: Vec<Laid>,
) -> Laid {
    let band = band_for(h * scale);
    let open = band == Band::Open && depth <= open_depth;
    if !open {
        children.clear();
    }
    Laid {
        id,
        local_x,
        local_y,
        x,
        y,
        w,
        h,
        open,
        band,
        children,
    }
}

pub fn find<'a>(node: &'a Laid, id: CellId) -> Option<&'a Laid> {
    if node.id == id {
        return Some(node);
    }
    for child in &node.children {
        if let Some(found) = find(child, id) {
            return Some(found);
        }
    }
    None
}

/// Deepest cell under a root-space point.
pub fn hit(node: &Laid, x: f64, y: f64) -> Option<CellId> {
    if !node.contains(x, y) {
        return None;
    }
    for child in node.children.iter().rev() {
        if let Some(id) = hit(child, x, y) {
            return Some(id);
        }
    }
    Some(node.id)
}

/// Deepest open group under a point, ignoring `skip` and anything inside it.
pub fn drop_group(node: &Laid, x: f64, y: f64, skip: Option<CellId>, sheet: &Worksheet) -> Option<CellId> {
    fn walk(
        node: &Laid,
        x: f64,
        y: f64,
        skip: Option<CellId>,
        sheet: &Worksheet,
    ) -> Option<CellId> {
        if skip == Some(node.id) || !node.contains(x, y) {
            return None;
        }
        let mut found = None;
        if node.open && sheet.cell(node.id).is_some_and(|cell| cell.inside().is_group()) {
            found = Some(node.id);
        }
        for child in &node.children {
            if let Some(deeper) = walk(child, x, y, skip, sheet) {
                found = Some(deeper);
            }
        }
        found
    }
    walk(node, x, y, skip, sheet)
}

/// Where a drop sits in a stack: below each child whose middle is above the point.
pub fn stack_index(group: &Laid, world_y: f64) -> usize {
    let mut index = 0;
    for child in &group.children {
        if world_y > child.y + child.h / 2.0 {
            index += 1;
        }
    }
    index
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
    fn three_deep_layout_nests_and_keeps_the_leaf_inside_its_parent() {
        let mut sheet = Worksheet::empty();
        let root = sheet.add(None, group("Root")).unwrap();
        let a = sheet.add(Some(root), group("A")).unwrap();
        let b = sheet.add(Some(a), group("B")).unwrap();
        let leaf = sheet
            .add(
                Some(b),
                NewCell {
                    name: "L".into(),
                    inside: Inside::Number {
                        value: workspace_model::Fraction::from_i64(24),
                        unit: "ft".into(),
                    },
                    x: 0,
                    y: 0,
                    show_on_face: false,
                },
            )
            .unwrap();
        let laid = layout(&sheet, 1.0, 8).expect("root");
        let leaf_box = find(&laid, leaf).expect("leaf");
        let b_box = find(&laid, b).expect("b");
        assert!(b_box.contains(leaf_box.x + 1.0, leaf_box.y + 1.0));
        assert!(leaf_box.y > b_box.y);
    }
}
