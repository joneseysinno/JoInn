//! One organelle's WGSL, with the constants it shares with joinn-visual.

use joinn_visual::{
    FILLED, LIVE, PORT_TAG, STYLE_MEMBRANE, STYLE_PORT_EMPTY, STYLE_PORT_FILLED, STYLE_WIRE,
    WIRE_TAG,
};

use super::{KIND_BODY, KIND_CELL, KIND_PORT, KIND_SHIFT};

const COMMON: &str = include_str!("../wgsl/common.wgsl");

/// The constants, then the common tables and helpers, then the organelle.
pub(super) fn shader_source(organelle: &str) -> String {
    let consts = [
        ("LIVE", LIVE),
        ("FILLED", FILLED),
        ("PORT_TAG", PORT_TAG),
        ("WIRE_TAG", WIRE_TAG),
        ("STYLE_MEMBRANE", STYLE_MEMBRANE),
        ("STYLE_PORT_EMPTY", STYLE_PORT_EMPTY),
        ("STYLE_PORT_FILLED", STYLE_PORT_FILLED),
        ("STYLE_WIRE", STYLE_WIRE),
        ("KIND_SHIFT", KIND_SHIFT),
        ("SLOT_MASK", (1u32 << KIND_SHIFT) - 1),
        ("KIND_BODY", KIND_BODY),
        ("KIND_CELL", KIND_CELL),
        ("KIND_PORT", KIND_PORT),
    ];
    let mut src: String = consts
        .iter()
        .map(|(name, v)| format!("const {name}: u32 = {v}u;\n"))
        .collect();
    src.push_str(COMMON);
    src.push_str(organelle);
    src
}
