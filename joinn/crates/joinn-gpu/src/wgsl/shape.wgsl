// The shape organelle: surfaces, cells and ports as rounded rectangles (a
// port is a rectangle whose radius is its half-extent, which is a circle).
// The instance index carries the table in its top bits and the slot below.

@vertex
fn vs(@builtin(vertex_index) vi: u32, @builtin(instance_index) ii: u32) -> Out {
    let kind = ii >> KIND_SHIFT;
    let slot = ii & SLOT_MASK;
    var o: Out;
    var lo: vec2<f32>;
    var hi: vec2<f32>;
    var r: f32;
    if kind == KIND_BODY {
        let m = bodies[slot];
        if (m.flags & LIVE) == 0u {
            return nothing();
        }
        lo = vec2(px(tick.ox, m.x), px(tick.oy, m.y));
        hi = lo + vec2(span(m.w), span(m.h));
        r = span(i32(m.radius));
        o.color = styles[STYLE_SURFACE];
        o.id = vec4(slot + 1u, 0u, 0u, m.generation);
    } else if kind == KIND_CELL {
        let c = cells[slot];
        if (c.flags & LIVE) == 0u {
            return nothing();
        }
        lo = vec2(px(tick.ox, c.x), px(tick.oy, c.y));
        hi = lo + vec2(span(c.w), span(c.h));
        r = span(i32(c.radius));
        o.color = styles[c.style];
        o.id = vec4(c.body + 1u, slot + 1u, 0u, c.generation);
    } else {
        let p = ports[slot];
        if (p.flags & LIVE) == 0u || p.cell >= arrayLength(&cells) {
            return nothing();
        }
        let c = cells[p.cell];
        let centre = vec2(px(tick.ox, p.x), px(tick.oy, p.y));
        r = span(i32(p.radius));
        lo = centre - vec2(r, r);
        hi = centre + vec2(r, r);
        o.color = styles[select(STYLE_PORT_EMPTY, STYLE_PORT_FILLED, (p.flags & FILLED) != 0u)];
        o.id = vec4(c.body + 1u, p.cell + 1u, PORT_TAG | p.position, c.generation);
    }
    o.pos = quad(vi, lo, hi);
    o.geom = vec4((lo + hi) * 0.5, (hi - lo) * 0.5);
    o.radius = r;
    return o;
}

// §2.5's rounded-rectangle distance, in pixels; drawn only where d < 0.
@fragment
fn fs(i: Out) -> Targets {
    let q = abs(i.pos.xy - i.geom.xy) - (i.geom.zw - vec2(i.radius, i.radius));
    let d = length(max(q, vec2(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - i.radius;
    if d >= 0.0 {
        discard;
    }
    var t: Targets;
    t.color = unpack4x8unorm(i.color);
    t.id = i.id;
    return t;
}
