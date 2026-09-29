//! RGBA8 of the style the probe's owning row names.

use joinn_frame::Verdict;
use joinn_visual::{
    FILLED, PORT_TAG, Pick, STYLE_BACKGROUND, STYLE_PORT_EMPTY, STYLE_PORT_FILLED, STYLE_SURFACE,
    STYLE_TABLE, STYLE_WIRE, Scene, TAG_MASK, WIRE_TAG,
};

/// Surface is 1, a cell uses its row's `style`, a port is 4 or 5 by `filled`,
/// a wire is 6, and the background is 0. The bytes are the style table's RGBA8.
pub(crate) fn g6_style_rgba(scene: &Scene, pick: &Pick) -> Result<[u8; 4], String> {
    let id = match pick {
        Pick::Background => [0; 4],
        Pick::Edge => {
            return Err(
                "the probe is an edge pixel; acceptance is an owner from the probe table".into(),
            );
        }
        Pick::Owned(id) => *id,
    };
    if let Verdict::Refused(r) = scene.resolve(id) {
        return Err(r.reason);
    }
    let style = if id == [0; 4] {
        STYLE_BACKGROUND
    } else {
        let [_, g, b, _] = id;
        if g == 0 && b == 0 {
            STYLE_SURFACE
        } else if (b & TAG_MASK) == WIRE_TAG {
            STYLE_WIRE
        } else if (b & TAG_MASK) == PORT_TAG {
            let cell = g.wrapping_sub(1);
            let position = b & !TAG_MASK;
            let Some(row) = scene
                .tables()
                .port
                .iter()
                .find(|p| p.cell == cell && p.position == position)
            else {
                return Err(format!(
                    "port ID {id:?} has no row; acceptance is a port in the tables"
                ));
            };
            if (row.flags & FILLED) != 0 {
                STYLE_PORT_FILLED
            } else {
                STYLE_PORT_EMPTY
            }
        } else if b == 0 {
            let slot = g.wrapping_sub(1);
            let Some(row) = scene.tables().cell.get(slot as usize) else {
                return Err(format!(
                    "cell ID {id:?} has no row; acceptance is a cell in the tables"
                ));
            };
            row.style
        } else {
            return Err(format!(
                "ID {id:?} is not a drawn owner; acceptance is surface, cell, port, wire, or background"
            ));
        }
    };
    let Some(word) = STYLE_TABLE.get(style as usize) else {
        return Err(format!(
            "style {style} is outside the style table; acceptance is a style id the table names"
        ));
    };
    Ok(word.to_le_bytes())
}
