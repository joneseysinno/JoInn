// The shape organelle: frames, dots, surfaces, cells and ports as rounded
// rectangles (a port or a dot is a rectangle whose radius is its half-extent,
// which is a circle). The instance index carries the table in its top bits and
// the slot below.

@vertex
fn vs(@builtin(vertex_index) vi: u32, @builtin(instance_index) ii: u32) -> Out {
    let kind = ii >> KIND_SHIFT;
    let slot = ii & SLOT_MASK;
    let k = kf();
    var o: Out;
    var lo: vec2<f32>;
    var hi: vec2<f32>;
    var r: f32;
    if kind == KIND_FRAME {
        let f = frames[slot];
        if GHOST || (f.flags & LIVE) == 0u || !shown(f.chart) {
            return nothing();
        }
        let ch = charts[f.chart];
        lo = vec2(place(ax(ch.origin_x), 0.0), place(ay(ch.origin_y), 0.0));
        hi = lo + vec2(k * f32(f.w), k * f32(f.h));
        r = 0.0;
        o.color = style(select(STYLE_NODE, STYLE_FRAME, open(f.chart)));
        o.alpha = 1.0;
        o.id = vec4(0u, 0u, select(SYSTEM_TAG, GALAXY_TAG, f.kind == CHART_GALAXY) | f.index, f.generation);
    } else if kind == KIND_BODY || kind == KIND_DOT {
        let m = bodies[slot];
        if (m.flags & LIVE) == 0u {
            return nothing();
        }
        let a = body_alpha(slot, select(MASK_SURFACE, MASK_DOT, kind == KIND_DOT));
        if a <= 0.0 {
            return nothing();
        }
        let ch = charts[slot];
        let x = ax(ch.origin_x);
        let y = ay(ch.origin_y);
        if kind == KIND_DOT {
            let c = vec2(place(x, k * f32(m.x) + k * f32(m.w) * 0.5),
                         place(y, k * f32(m.y) + k * f32(m.h) * 0.5));
            r = 2.0;
            lo = c - vec2(r, r);
            hi = c + vec2(r, r);
        } else {
            lo = vec2(place(x, k * f32(m.x)), place(y, k * f32(m.y)));
            hi = lo + vec2(k * f32(m.w), k * f32(m.h));
            r = k * f32(m.radius);
        }
        o.color = style(STYLE_SURFACE);
        o.alpha = a;
        o.id = vec4(slot + 1u, 0u, 0u, m.generation);
    } else if kind == KIND_CELL {
        let c = cells[slot];
        if (c.flags & LIVE) == 0u {
            return nothing();
        }
        let a = body_alpha(c.body, MASK_CELL);
        if a <= 0.0 {
            return nothing();
        }
        let ch = charts[c.body];
        lo = vec2(place(ax(ch.origin_x), k * f32(c.x)), place(ay(ch.origin_y), k * f32(c.y)));
        hi = lo + vec2(k * f32(c.w), k * f32(c.h));
        r = k * f32(c.radius);
        // Refused, else latent, else the row's style.
        let latent = select(c.style, STYLE_RESPONSE_LATENT, (c.flags & LATENT) != 0u);
        o.color = style(select(latent, STYLE_CELL_REFUSED, (c.flags & REFUSED) != 0u));
        o.alpha = a;
        o.id = vec4(c.body + 1u, slot + 1u, 0u, c.generation);
    } else {
        let p = ports[slot];
        if (p.flags & LIVE) == 0u || p.cell >= arrayLength(&cells) {
            return nothing();
        }
        let c = cells[p.cell];
        let a = body_alpha(c.body, select(MASK_FULL, MASK_CELL, (p.flags & SURFACE_PORT) != 0u));
        if a <= 0.0 {
            return nothing();
        }
        let ch = charts[c.body];
        let centre = vec2(place(ax(ch.origin_x), k * f32(p.x)), place(ay(ch.origin_y), k * f32(p.y)));
        r = k * f32(p.radius);
        lo = centre - vec2(r, r);
        hi = centre + vec2(r, r);
        o.color = style(select(STYLE_PORT_EMPTY, STYLE_PORT_FILLED, (p.flags & FILLED) != 0u));
        o.alpha = a;
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
    t.color = shade(i.color, i.alpha);
    t.id = i.id;
    return t;
}
