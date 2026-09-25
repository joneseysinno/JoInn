//! `cargo xtask probe-refusal` — double delivery; probe label, then far-side lines.

use super::double_delivery::double_delivery;
use joinn_link::format_link_refusal;

/// Run the double-delivery scenario; print probe label, then each far-side line.
pub(crate) fn probe_refusal() -> Result<(), String> {
    let delivery = double_delivery()?;
    println!("{}", delivery.label);
    for refusal in &delivery.far_side {
        println!("{}", format_link_refusal(refusal));
    }
    Ok(())
}
