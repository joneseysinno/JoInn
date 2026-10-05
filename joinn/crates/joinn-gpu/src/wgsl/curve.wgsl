// The curve organelle: wires and strokes as capsules. A wire's ends are read
// through link -> incidence -> port, so moving a port moves its wires without
// a write. A stroke is in sixteenths of its chart.

@vertex
fn vs(@builtin(vertex_index) vi: u32, @builtin(instance_index) ii: u32) -> Out {
    let kind = ii >> KIND_SHIFT;
    let slot = ii & SLOT_MASK;
    let k = kf();
    var o: Out;
    var a: vec2<f32>;
    var b: vec2<f32>;
    var w: f32;
    if kind == KIND_LINK {
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
        let alpha = body_alpha(l.body, MASK_FULL);
        if alpha <= 0.0 {
            return nothing();
        }
        let ch = charts[l.body];
        let x = ax(ch.origin_x);
        let y = ay(ch.origin_y);
        a = vec2(place(x, k * f32(ports[sa].x)), place(y, k * f32(ports[sa].y)));
        b = vec2(place(x, k * f32(ports[sb].x)), place(y, k * f32(ports[sb].y)));
        w = k * f32(l.half_width_quarters) * 0.25;
        o.color = style(STYLE_WIRE);
        o.alpha = alpha;
        o.id = vec4(l.body + 1u, 0u, WIRE_TAG | slot, l.generation);
    } else {
        let s = strokes[slot];
        if (s.flags & LIVE) == 0u || s.chart >= arrayLength(&charts) {
            return nothing();
        }
        var alpha = 0.0;
        if charts[s.chart].kind == CHART_BODY {
            alpha = body_alpha(s.chart, MASK_FULL);
        } else if !GHOST && shown(s.chart) {
            alpha = 1.0;
        }
        if alpha <= 0.0 {
            return nothing();
        }
        let ch = charts[s.chart];
        let x = ax(ch.origin_x);
        let y = ay(ch.origin_y);
        let u = k * 0.0625;
        a = vec2(place(x, u * f32(s.x0)), place(y, u * f32(s.y0)));
        b = vec2(place(x, u * f32(s.x1)), place(y, u * f32(s.y1)));
        w = u * f32(s.half_width);
        o.color = style(s.style);
        o.alpha = alpha;
        o.id = vec4(s.r, s.g, s.b, s.generation);
    }
    o.pos = quad(vi, min(a, b) - vec2(w, w), max(a, b) + vec2(w, w));
    o.geom = vec4(a, b);
    o.radius = w;
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
    t.color = shade(i.color, i.alpha);
    t.id = i.id;
    return t;
}
