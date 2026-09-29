//! The surface format is not sRGB (Amendment B4).

/// `Bgra8Unorm`, then `Rgba8Unorm`, then the surface's first format.
pub(super) fn choose_format(formats: &[wgpu::TextureFormat]) -> Option<wgpu::TextureFormat> {
    for want in [
        wgpu::TextureFormat::Bgra8Unorm,
        wgpu::TextureFormat::Rgba8Unorm,
    ] {
        if formats.contains(&want) {
            return Some(want);
        }
    }
    formats.first().copied()
}
