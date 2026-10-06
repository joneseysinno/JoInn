// The segment organelle: links (plan 7.3 §2.5). A segment draws only in the
// fold state the zoom shows, in its route's form, both chosen here from the
// tick and the route row in integers. A leg or stub is a capsule, a knot a
// disc, an arrowhead a triangle from its base centre to its tip, all in
// sixteenths of the segment's chart.

struct Route {
    link: u32, fold: u32, size: u32, ordered: u32, segment_first: u32, segment_count: u32,
    generation: u32, flags: u32,
};
struct Segment {
    chart: u32, x0: i32, y0: i32, x1: i32, y1: i32, route: u32, member: u32, kind: u32, legs: u32,
    style: u32, generation: u32, flags: u32,
};

@group(1) @binding(8) var<storage, read> segments: array<Segment>;
@group(1) @binding(9) var<storage, read> routes: array<Route>;

// Half-width in sixteenths: a stub 4 and an arrowhead 16 in every form; a
// region's leg or knot 48; a knot disc 16; a spine's leg 8, a hub's 4, a
// bundle's 4 + 2·(legs − 1) up to 48.
fn half16(kind: u32, form: u32, legs: u32) -> u32 {
    if kind == SEG_STUB {
        return 4u;
    }
    if kind == SEG_ARROW {
        return 16u;
    }
    if form == FORM_REGION {
        return 48u;
    }
    if kind == SEG_KNOT {
        return 16u;
    }
    if form == FORM_SPINE {
        return 8u;
    }
    if form == FORM_HUB {
        return 4u;
    }
    return min(4u + 2u * (max(legs, 1u) - 1u), 48u);
}

// A region's legs and knot take the region style; everything else its row's.
fn seg_style(s: Segment, form: u32) -> u32 {
    if form == FORM_REGION && (s.kind == SEG_LEG || s.kind == SEG_KNOT) {
        return STYLE_REGION;
    }
    return s.style;
}

@vertex
fn vs(@builtin(vertex_index) vi: u32, @builtin(instance_index) slot: u32) -> Out {
    let s = segments[slot];
    if (s.flags & LIVE) == 0u || s.route >= arrayLength(&routes) || !live_chart(s.chart) {
        return nothing();
    }
    let r = routes[s.route];
    if (r.flags & LIVE) == 0u || r.fold != fold_now() {
        return nothing();
    }
    // The owner pass draws the owner form; in a form window the ghost draws
    // the other form, colour only, each faded across the window. A piece both
    // forms draw alike is the owner's alone, at full opacity.
    var form = FORM_SPINE;
    var alpha = 1.0;
    if r.ordered == 0u {
        let owner = form_owner(r.size);
        form = owner;
        let w = form_window(r.size);
        if w == 0u {
            if GHOST {
                return nothing();
            }
        } else {
            let lo = w - 1u;
            let hi = w;
            let alike = half16(s.kind, lo, s.legs) == half16(s.kind, hi, s.legs)
                && seg_style(s, lo) == seg_style(s, hi);
            if alike {
                if GHOST {
                    return nothing();
                }
            } else {
                if GHOST {
                    form = select(hi, lo, owner == hi);
                }
                var ts = array<f32, 2>(f32(F0), f32(F1));
                let t = ts[w - 1u];
                let up = clamp((kf() * f32(r.size) - t) / (0.2 * t), 0.0, 1.0);
                alpha = select(1.0 - up, up, form == hi);
            }
        }
    } else if GHOST {
        return nothing();
    }
    let ch = charts[s.chart];
    let x = ax(ch.origin_x);
    let y = ay(ch.origin_y);
    let u = kf() * 0.0625;
    let a = vec2(place(x, u * f32(s.x0)), place(y, u * f32(s.y0)));
    let b = vec2(place(x, u * f32(s.x1)), place(y, u * f32(s.y1)));
    let w = u * f32(half16(s.kind, form, s.legs));
    var o: Out;
    o.color = style(seg_style(s, form));
    o.alpha = alpha;
    o.id = vec4(r.link + 1u, s.member, LINK_TAG | form, r.generation);
    o.pos = quad(vi, min(a, b) - vec2(w, w), max(a, b) + vec2(w, w));
    o.geom = vec4(a, b);
    // An arrowhead carries its half-width negated.
    o.radius = select(w, -w, s.kind == SEG_ARROW);
    return o;
}

// A capsule's distance, or an arrowhead's: inside when the point lies between
// base and tip and within the half-width narrowing to the tip. Drawn where
// d < 0.
@fragment
fn fs(i: Out) -> Targets {
    let a = i.geom.xy;
    let ba = i.geom.zw - a;
    let pa = i.pos.xy - a;
    var d: f32;
    if i.radius < 0.0 {
        let w = -i.radius;
        let l = length(ba);
        let e = ba / l;
        let t = dot(pa, e);
        let n = abs(e.x * pa.y - e.y * pa.x);
        d = max(max(-t, t - l), n - w * (1.0 - t / l));
    } else {
        let l = dot(ba, ba);
        var h = 0.0;
        if l > 0.0 {
            h = clamp(dot(pa, ba) / l, 0.0, 1.0);
        }
        d = length(pa - ba * h) - i.radius;
    }
    if d >= 0.0 {
        discard;
    }
    var t: Targets;
    t.color = shade(i.color, i.alpha);
    t.id = i.id;
    return t;
}
