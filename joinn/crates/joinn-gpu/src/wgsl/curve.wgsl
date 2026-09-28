// The curve organelle: wires as capsules. A wire's ends are read through
// link -> incidence -> port, so moving a port moves its wires without a write.

@vertex
fn vs(@builtin(vertex_index) vi: u32, @builtin(instance_index) slot: u32) -> Out {
    let l = links[slot];
    if (l.flags & LIVE) == 0u {
        return nothing();
    }
    let entries = arrayLength(&incidence);
    if l.start >= entries || l.start + 1u >= entries {
        return nothing();
    }
    let sa = incidence[l.start];
    let sb = incidence[l.start + 1u];
    let slots = arrayLength(&ports);
    if sa >= slots || sb >= slots {
        return nothing();
    }
    let a = vec2(px(tick.ox, ports[sa].x), px(tick.oy, ports[sa].y));
    let b = vec2(px(tick.ox, ports[sb].x), px(tick.oy, ports[sb].y));
    let w = f32(tick.k) * f32(l.half_width_quarters) * 0.25;
    var o: Out;
    o.pos = quad(vi, min(a, b) - vec2(w, w), max(a, b) + vec2(w, w));
    o.geom = vec4(a, b);
    o.radius = w;
    o.color = styles[STYLE_WIRE];
    o.id = vec4(l.body + 1u, 0u, WIRE_TAG | slot, l.generation);
    return o;
}

// §2.5's capsule distance, in pixels; drawn only where d < 0.
@fragment
fn fs(i: Out) -> Targets {
    let a = i.geom.xy;
    let ba = i.geom.zw - a;
    let pa = i.pos.xy - a;
    let l = dot(ba, ba);
    var h = 0.0;
    if l > 0.0 {
        h = clamp(dot(pa, ba) / l, 0.0, 1.0);
    }
    let d = length(pa - ba * h) - i.radius;
    if d >= 0.0 {
        discard;
    }
    var t: Targets;
    t.color = unpack4x8unorm(i.color);
    t.id = i.id;
    return t;
}
