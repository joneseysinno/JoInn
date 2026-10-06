// The probe: one triangle over the whole target, every pixel one decision
// from the common code every organelle ships with, the size carried in the
// tick's first pad word and the decision in its second: 0 `band(size)`,
// 1 `form_owner(size)`, 2 `fold_now()`.

@vertex
fn vs(@builtin(vertex_index) vi: u32) -> @builtin(position) vec4<f32> {
    var p = array<vec2<f32>, 3>(vec2(-1.0, -1.0), vec2(3.0, -1.0), vec2(-1.0, 3.0));
    return vec4(p[vi], 0.0, 1.0);
}

@fragment
fn fs() -> @location(0) u32 {
    if tick.pad1 == 1u {
        return form_owner(tick.pad0);
    }
    if tick.pad1 == 2u {
        return fold_now();
    }
    return band(tick.pad0);
}
