// Shared by every organelle. The tables mirror joinn-visual's `row_bytes`:
// every field a u32 or i32, in field order, rows padded to 16 bytes. Every
// decision (live, the cut, the band, the owner) is an integer test made the
// same way as joinn-visual's CPU cut; a float only places a pixel and fades a
// colour, after every decision is made (V141).

struct Tick {
    level: i32, step: u32, pin_x: i32, pin_y: i32, ox: i32, oy: i32, fx: u32, fy: u32,
    width: u32, height: u32, pad0: u32, pad1: u32,
};
struct Body { x: i32, y: i32, w: i32, h: i32, radius: u32, generation: u32, flags: u32, pad0: u32 };
struct Cell {
    body: u32, x: i32, y: i32, w: i32, h: i32, radius: u32, generation: u32, style: u32, flags: u32,
    pad0: u32, pad1: u32, pad2: u32,
};
struct Port { cell: u32, position: u32, direction: u32, x: i32, y: i32, radius: u32, generation: u32, flags: u32 };
struct Link { kind: u32, start: u32, count: u32, body: u32, half_width_quarters: u32, generation: u32, flags: u32, pad0: u32 };
struct Chart { origin_x: i32, origin_y: i32, parent: u32, kind: u32, size: u32, generation: u32, flags: u32, pad0: u32 };
struct Frame { chart: u32, w: u32, h: u32, kind: u32, index: u32, generation: u32, flags: u32, pad0: u32 };
struct Stroke {
    chart: u32, x0: i32, y0: i32, x1: i32, y1: i32, half_width: u32, r: u32, g: u32, b: u32,
    style: u32, generation: u32, flags: u32,
};

// §7.2: group 0 tick, group 1 universe, group 2 genome, group 3 pass.
@group(0) @binding(0) var<uniform> tick: Tick;
@group(1) @binding(0) var<storage, read> bodies: array<Body>;
@group(1) @binding(1) var<storage, read> cells: array<Cell>;
@group(1) @binding(2) var<storage, read> ports: array<Port>;
@group(1) @binding(3) var<storage, read> links: array<Link>;
@group(1) @binding(4) var<storage, read> incidence: array<u32>;
@group(1) @binding(5) var<storage, read> charts: array<Chart>;
@group(1) @binding(6) var<storage, read> frames: array<Frame>;
@group(1) @binding(7) var<storage, read> strokes: array<Stroke>;
@group(2) @binding(0) var<uniform> styles: array<vec4<u32>, 4>;

struct Out {
    @builtin(position) pos: vec4<f32>,
    @location(0) @interpolate(flat) geom: vec4<f32>,
    @location(1) @interpolate(flat) radius: f32,
    @location(2) @interpolate(flat) color: u32,
    @location(3) @interpolate(flat) id: vec4<u32>,
    @location(4) @interpolate(flat) alpha: f32,
};

struct Targets {
    @location(0) color: vec4<f32>,
    @location(1) id: vec4<u32>,
};

fn style(i: u32) -> u32 {
    return styles[i / 4u][i % 4u];
}

fn corner(vi: u32) -> vec2<f32> {
    var c = array<vec2<f32>, 6>(vec2(-1.0, -1.0), vec2(1.0, -1.0), vec2(1.0, 1.0),
                                vec2(-1.0, -1.0), vec2(1.0, 1.0), vec2(-1.0, 1.0));
    return c[vi];
}

// Pixels per layout unit, 2^level·(256 + step)/256: exact in f32.
fn kf() -> f32 {
    return ldexp(f32(256u + tick.step), tick.level - 8);
}

// An exact pixel coordinate: n + f/2^32.
struct Px { n: i32, f: u32 };

// The pixel of anchor-relative layout coordinate u on the axis whose origin is
// o + fr/2^32: o + fr/2^32 + u·(256 + step)·2^(level − 8), in integers.
fn at(o: i32, fr: u32, u: i32) -> Px {
    let a = u * i32(256u + tick.step);
    if tick.level >= 8 {
        return Px(o + (a << u32(tick.level - 8)), fr);
    }
    let s = u32(8 - tick.level);
    let q = a >> s;
    let r = u32(a & ((1 << s) - 1));
    let t = r << (32u - s);
    let f = t + fr;
    return Px(o + q + select(0, 1, f < t), f);
}

fn ax(u: i32) -> Px {
    return at(tick.ox, tick.fx, u);
}

fn ay(u: i32) -> Px {
    return at(tick.oy, tick.fy, u);
}

fn below(p: Px, r: i32) -> bool {
    return p.n < r;
}

fn above0(p: Px) -> bool {
    return p.n > 0 || (p.n == 0 && p.f > 0u);
}

// The only float: a decided pixel plus a distance d in pixels.
fn place(p: Px, d: f32) -> f32 {
    return f32(p.n) + (d + ldexp(f32(p.f), -32));
}

fn live_chart(c: u32) -> bool {
    return c < arrayLength(&charts) && (charts[c].flags & LIVE) != 0u;
}

fn extent(c: u32) -> vec2<i32> {
    let kind = charts[c].kind;
    if kind == CHART_BODY {
        let b = bodies[c];
        return vec2(b.w, b.h);
    }
    if kind == CHART_SYSTEM {
        return vec2(SYSTEM_W, SYSTEM_H);
    }
    if kind == CHART_GALAXY {
        return vec2(GALAXY_W, GALAXY_H);
    }
    return vec2(UNIVERSE_W, UNIVERSE_H);
}

// The chart's pixel rectangle meets the viewport.
fn visible(c: u32) -> bool {
    let ch = charts[c];
    let e = extent(c);
    return below(ax(ch.origin_x), i32(tick.width)) && above0(ax(ch.origin_x + e.x))
        && below(ay(ch.origin_y), i32(tick.height)) && above0(ay(ch.origin_y + e.y));
}

// 10·s ≥ 11·T, with s = (256 + step)·size·2^level/256. 11·T·256 is a multiple
// of 2^9 for every threshold, so the shift right is exact.
fn past(size: u32, t: u32) -> bool {
    let a = 10u * (256u + tick.step) * size;
    if tick.level >= 0 {
        return a >= (11u * t * 256u) >> u32(tick.level);
    }
    return a >= (11u * t * 256u) << u32(-tick.level);
}

// 0 dot, 1 glyph, 2 summary, 3 full.
fn band(size: u32) -> u32 {
    if past(size, T2) {
        return 3u;
    }
    if past(size, T1) {
        return 2u;
    }
    if past(size, T0) {
        return 1u;
    }
    return 0u;
}

// T ≤ s < 1.2·T: 1 + the threshold's index, or 0 outside every window.
fn window(size: u32) -> u32 {
    let a = (256u + tick.step) * size;
    var ts = array<u32, 3>(T0, T1, T2);
    for (var i = 0u; i < 3u; i++) {
        let t = ts[i];
        var lo: u32;
        var hi: u32;
        if tick.level >= 0 {
            lo = (t * 256u) >> u32(tick.level);
            hi = (6u * t * 256u) >> u32(tick.level);
        } else {
            lo = (t * 256u) << u32(-tick.level);
            hi = (6u * t * 256u) << u32(-tick.level);
        }
        if a >= lo && 5u * a < hi {
            return i + 1u;
        }
    }
    return 0u;
}

fn open(c: u32) -> bool {
    return band(charts[c].size) >= 2u;
}

// Visible, and every system and galaxy above it visible and open.
fn shown(c: u32) -> bool {
    if !live_chart(c) || !visible(c) {
        return false;
    }
    var here = c;
    for (var i = 0u; i < 4u; i++) {
        let p = charts[here].parent;
        if p == here || !live_chart(p) || charts[p].kind == CHART_UNIVERSE {
            return true;
        }
        if !visible(p) || !open(p) {
            return false;
        }
        here = p;
    }
    return true;
}

// The opacity a body's element draws with in this pass, 0 when it draws
// nothing. `mask` holds the bands the element belongs to (bit b for band b).
// The owner pass draws an element of the owner band; the ghost pass draws one
// that is fading in or out but is not the owner band's, colour only.
fn body_alpha(b: u32, mask: u32) -> f32 {
    if !shown(b) {
        return 0.0;
    }
    let size = charts[b].size;
    let owned = ((mask >> band(size)) & 1u) != 0u;
    if owned == GHOST {
        return 0.0;
    }
    let w = window(size);
    if w == 0u {
        return select(0.0, 1.0, owned);
    }
    let lo = ((mask >> (w - 1u)) & 1u) != 0u;
    let hi = ((mask >> w) & 1u) != 0u;
    if lo && hi {
        return 1.0;
    }
    var ts = array<f32, 3>(f32(T0), f32(T1), f32(T2));
    let t = ts[w - 1u];
    let up = clamp((kf() * f32(size) - t) / (0.2 * t), 0.0, 1.0);
    return select(0.0, 1.0 - up, lo) + select(0.0, up, hi);
}

fn clip(p: vec2<f32>) -> vec4<f32> {
    return vec4(p.x / f32(tick.width) * 2.0 - 1.0, 1.0 - p.y / f32(tick.height) * 2.0, 0.0, 1.0);
}

// A free slot draws nothing: every corner lands on one point.
fn nothing() -> Out {
    var o: Out;
    o.pos = vec4(0.0, 0.0, 0.0, 1.0);
    return o;
}

// A quad over the box lo..hi, grown by one pixel so every centre inside is covered.
fn quad(vi: u32, lo: vec2<f32>, hi: vec2<f32>) -> vec4<f32> {
    let c = (lo + hi) * 0.5;
    let h = (hi - lo) * 0.5 + vec2(1.0, 1.0);
    return clip(c + corner(vi) * h);
}

fn shade(color: u32, alpha: f32) -> vec4<f32> {
    let c = unpack4x8unorm(color);
    return vec4(c.rgb, c.a * alpha);
}
