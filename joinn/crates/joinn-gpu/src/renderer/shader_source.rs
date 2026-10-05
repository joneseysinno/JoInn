//! One organelle's WGSL, with the constants it shares with joinn-visual.

use joinn_visual::{
    CHART_BODY, CHART_GALAXY, CHART_SYSTEM, CHART_UNIVERSE, FILLED, GALAXY_SIZE, GALAXY_TAG,
    LATENT, LIVE, PORT_TAG, REFUSED, STYLE_CELL_REFUSED, STYLE_FRAME, STYLE_NODE, STYLE_PORT_EMPTY,
    STYLE_PORT_FILLED, STYLE_RESPONSE_LATENT, STYLE_SURFACE, STYLE_WIRE, SURFACE_PORT, SYSTEM_SIZE,
    SYSTEM_TAG, THRESHOLDS, UNIVERSE_SIZE, WIRE_TAG,
};

use super::{KIND_BODY, KIND_CELL, KIND_DOT, KIND_FRAME, KIND_LINK, KIND_PORT, KIND_SHIFT};

const COMMON: &str = include_str!("../wgsl/common.wgsl");

/// Band masks: bit b is band b (dot 0, glyph 1, summary 2, full 3).
const MASK_DOT: u32 = 1;
const MASK_SURFACE: u32 = 14;
const MASK_CELL: u32 = 12;
const MASK_FULL: u32 = 8;

/// The constants, then the common tables and helpers, then the organelle. A
/// ghost organelle draws only what is fading and not its owner band's.
pub(super) fn shader_source(organelle: &str, ghost: bool) -> String {
    let words = [
        ("LIVE", LIVE),
        ("FILLED", FILLED),
        ("REFUSED", REFUSED),
        ("LATENT", LATENT),
        ("SURFACE_PORT", SURFACE_PORT),
        ("PORT_TAG", PORT_TAG),
        ("WIRE_TAG", WIRE_TAG),
        ("SYSTEM_TAG", SYSTEM_TAG),
        ("GALAXY_TAG", GALAXY_TAG),
        ("STYLE_SURFACE", STYLE_SURFACE),
        ("STYLE_PORT_EMPTY", STYLE_PORT_EMPTY),
        ("STYLE_PORT_FILLED", STYLE_PORT_FILLED),
        ("STYLE_WIRE", STYLE_WIRE),
        ("STYLE_CELL_REFUSED", STYLE_CELL_REFUSED),
        ("STYLE_RESPONSE_LATENT", STYLE_RESPONSE_LATENT),
        ("STYLE_FRAME", STYLE_FRAME),
        ("STYLE_NODE", STYLE_NODE),
        ("CHART_UNIVERSE", CHART_UNIVERSE),
        ("CHART_GALAXY", CHART_GALAXY),
        ("CHART_SYSTEM", CHART_SYSTEM),
        ("CHART_BODY", CHART_BODY),
        ("KIND_SHIFT", KIND_SHIFT),
        ("SLOT_MASK", (1u32 << KIND_SHIFT) - 1),
        ("KIND_BODY", KIND_BODY),
        ("KIND_CELL", KIND_CELL),
        ("KIND_PORT", KIND_PORT),
        ("KIND_FRAME", KIND_FRAME),
        ("KIND_DOT", KIND_DOT),
        ("KIND_LINK", KIND_LINK),
        ("MASK_DOT", MASK_DOT),
        ("MASK_SURFACE", MASK_SURFACE),
        ("MASK_CELL", MASK_CELL),
        ("MASK_FULL", MASK_FULL),
    ];
    let sizes = [
        ("T0", THRESHOLDS[0]),
        ("T1", THRESHOLDS[1]),
        ("T2", THRESHOLDS[2]),
    ];
    let ints = [
        ("SYSTEM_W", SYSTEM_SIZE.0),
        ("SYSTEM_H", SYSTEM_SIZE.1),
        ("GALAXY_W", GALAXY_SIZE.0),
        ("GALAXY_H", GALAXY_SIZE.1),
        ("UNIVERSE_W", UNIVERSE_SIZE.0),
        ("UNIVERSE_H", UNIVERSE_SIZE.1),
    ];
    let mut src = format!("const GHOST: bool = {ghost};\n");
    for (name, v) in words {
        src.push_str(&format!("const {name}: u32 = {v}u;\n"));
    }
    for (name, v) in sizes {
        src.push_str(&format!("const {name}: u32 = {v}u;\n"));
    }
    for (name, v) in ints {
        src.push_str(&format!("const {name}: i32 = {v};\n"));
    }
    src.push_str(COMMON);
    src.push_str(organelle);
    src
}
