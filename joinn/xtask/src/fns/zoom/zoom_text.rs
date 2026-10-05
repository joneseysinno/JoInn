//! Everything `cargo xtask zoom` prints.

use joinn_visual::{LEVEL_MIN, cut, touches};
use std::collections::BTreeSet;
use std::fmt::Write;

use super::{cut_line, zoom_views};
use crate::fns::grove::grove_layout;

/// One cut line per §2.12 view, the touches at level −4 step 0 and at the
/// frame, then `zoom: <n> views, cut counts printed; touches <a> and <b>`. A
/// link that touches one node twice is an error naming it (V146).
pub(crate) fn zoom_text() -> Result<String, String> {
    let (universe, layout) = grove_layout()?;
    let views = zoom_views(&layout);
    let mut out = String::new();
    for (label, camera) in &views {
        let c = cut(&layout, camera);
        let _ = writeln!(out, "{}", cut_line(label, &layout, camera, &c));
    }
    let mut totals = Vec::new();
    let picked = [
        views
            .iter()
            .find(|(l, c)| l == "at s" && c.zoom.level == LEVEL_MIN && c.zoom.step == 0),
        views.iter().find(|(l, _)| l == "frame"),
    ];
    for (label, camera) in picked.into_iter().flatten() {
        let c = cut(&layout, camera);
        let per_link = touches(&layout, &c, &universe.coding.links);
        for (link, nodes) in &per_link {
            let distinct: BTreeSet<_> = nodes.iter().collect();
            if distinct.len() != nodes.len() {
                return Err(format!(
                    "zoom: link {link} touches a node twice; acceptance is each node at most once per link (V146)"
                ));
            }
        }
        let total: usize = per_link.iter().map(|(_, n)| n.len()).sum();
        let name = if label == "frame" {
            "frame".to_owned()
        } else {
            format!("level {} step {}", camera.zoom.level, camera.zoom.step)
        };
        let _ = writeln!(
            out,
            "touches {name}: {} links, {total} touches, each node at most once per link",
            per_link.len()
        );
        totals.push(total.to_string());
    }
    let _ = writeln!(
        out,
        "zoom: {} views, cut counts printed; touches {}",
        views.len(),
        totals.join(" and ")
    );
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::zoom_text;
    use crate::fns::zoom::ZOOM_BLOCK;

    #[test]
    fn the_grove_prints_the_plan_s_block_byte_for_byte() {
        let got = match zoom_text() {
            Ok(t) => t,
            Err(e) => panic!("{e}"),
        };
        assert_eq!(got, ZOOM_BLOCK);
    }
}
