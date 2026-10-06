//! The downward wrap: adapter, device, organelles, ID target and readback.

#![forbid(unsafe_code)]

mod adapter;
#[cfg(test)]
mod fixtures;
mod gpu;
mod read_texel;
mod refuse;
mod render_offscreen;
mod renderer;

pub use adapter::{GpuAdapter, adapters};
pub use gpu::{Gpu, open, wait};
pub use read_texel::read_texel;
pub use render_offscreen::render_offscreen;
pub use renderer::{
    ID_FORMAT, OFFSCREEN_FORMAT, Picture, Renderer, Upload, band_probe, link_probe,
};
