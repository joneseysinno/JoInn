// Shared by both organelles. The tables mirror joinn-visual's `row_bytes`:
// every field a u32 or i32, in field order, rows padded to 16 bytes.

struct Tick { k: i32, ox: i32, oy: i32, width: i32, height: i32, pad0: i32, pad1: i32, pad2: i32 };
struct Body { x: i32, y: i32, w: i32, h: i32, radius: u32, generation: u32, flags: u32, pad0: u32 };
struct Cell {
    body: u32, x: i32, y: i32, w: i32, h: i32, radius: u32, generation: u32, style: u32, flags: u32,
    pad0: u32, pad1: u32, pad2: u32,
};
struct Port { cell: u32, position: u32, direction: u32, x: i32, y: i32, radius: u32, generation: u32, flags: u32 };
struct Link { kind: u32, start: u32, count: u32, body: u32, half_width_quarters: u32, generation: u32, flags: u32, pad0: u32 };

// §7.2: group 0 tick, group 1 universe, group 2 genome, group 3 pass.
@group(0) @binding(0) var<uniform> tick: Tick;
@group(1) @binding(0) var<storage, read> bodies: array<Body>;
@group(1) @binding(1) var<storage, read> cells: array<Cell>;
@group(1) @binding(2) var<storage, read> ports: array<Port>;
@group(1) @binding(3) var<storage, read> links: array<Link>;
@group(1) @binding(4) var<storage, read> incidence: array<u32>;
@group(2) @binding(0) var<storage, read> styles: array<u32>;

struct Out {
    @builtin(position) pos: vec4<f32>,
    @location(0) @interpolate(flat) geom: vec4<f32>,
    @location(1) @interpolate(flat) radius: f32,
    @location(2) @interpolate(flat) color: u32,
    @location(3) @interpolate(flat) id: vec4<u32>,
};

struct Targets {
    @location(0) color: vec4<f32>,
    @location(1) id: vec4<u32>,
};

fn corner(vi: u32) -> vec2<f32> {
    var c = array<vec2<f32>, 6>(vec2(-1.0, -1.0), vec2(1.0, -1.0), vec2(1.0, 1.0),
                                vec2(-1.0, -1.0), vec2(1.0, 1.0), vec2(-1.0, 1.0));
    return c[vi];
}

// A layout coordinate u is pixel o + k·u; a layout length n is k·n pixels.
fn px(o: i32, u: i32) -> f32 {
    return f32(o) + f32(tick.k) * f32(u);
}

fn span(n: i32) -> f32 {
    return f32(tick.k) * f32(n);
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
