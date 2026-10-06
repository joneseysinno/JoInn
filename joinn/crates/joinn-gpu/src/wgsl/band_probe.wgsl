// The band probe: one triangle over the whole target, every pixel `band(size)`
// from the common code every organelle ships with, the size carried in the
// tick's first pad word.

@vertex
fn vs(@builtin(vertex_index) vi: u32) -> @builtin(position) vec4<f32> {
    var p = array<vec2<f32>, 3>(vec2(-1.0, -1.0), vec2(3.0, -1.0), vec2(-1.0, 3.0));
    return vec4(p[vi], 0.0, 1.0);
}

@fragment
fn fs() -> @location(0) u32 {
    return band(tick.pad0);
}
