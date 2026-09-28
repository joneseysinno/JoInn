//! S2 · Core-limits GPU spike. Throwaway: never promoted into `crates/`.
//!
//! Question: does Part II §7 (the GPU Body Model) fit WebGPU core limits, and
//! what happens where vertex-stage storage buffers are missing?
//!
//! For every adapter the machine offers (real GPUs and the software one), it:
//!   1. records the adapter's own limits and whether it can read storage
//!      buffers in the vertex stage (the compatibility-mode fork);
//!   2. asks for a device with exactly the WebGPU core limits, so a strong
//!      desktop is held to what the weakest core device must offer;
//!   3. binds §7.2's layout: 4 bind groups, 7 storage buffers visible to the
//!      vertex stage (body, cell, port, link, incidence, shape instances,
//!      style);
//!   4. draws N instanced SDF rounded rects two ways:
//!        path S: instance -> cell -> body walked in the vertex shader from
//!                storage buffers (Part II §7 as written);
//!        path V: the fallback, the same rects resolved on the CPU and fed as
//!                a per-instance vertex buffer (no storage in the vertex
//!                stage at all);
//!   5. writes a color target and an R32Uint ID target, reads the ID target
//!      back asynchronously, and checks a randomized sweep of points against
//!      a CPU pick of the same tables;
//!   6. checks that path S and path V produce the same ID image, pixel for
//!      pixel (two engines, one truth).
//!
//! Results go to the console and to RESULTS.md next to Cargo.toml.
//! Run with:  cargo run --release

use std::fmt::Write as _;
use std::sync::mpsc;
use std::time::Instant;

const W: u32 = 1920;
const H: u32 = 1080;
const SIZES: [u32; 2] = [10_000, 100_000];
const WARMUP: usize = 10;
const FRAMES: usize = 100;
const SWEEP: usize = 10_000;
const PICKS: usize = 50;
const INTERACTIVE_MS: f64 = 1000.0 / 60.0;

// ---------------------------------------------------------------------------
// Scene: tables shaped like Part II §7.1. Every coordinate is an integer and
// every scale a power of two, so f32 on the GPU and f64 on the CPU compute
// the same rectangles exactly.
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
struct BodyRow {
    chart: [f32; 4], // translate x, y; scale; pad
}

#[repr(C)]
#[derive(Clone, Copy)]
struct CellRow {
    local: [f32; 4], // offset x, y; half extent x, y (body units)
    body: u32,
    style: u32,
    generation: u32,
    flags: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ShapeRow {
    cell: u32,
    radius: f32, // body units
    _pad: [u32; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VertexRow {
    center: [f32; 2],
    half: [f32; 2],
    radius: f32,
    color: u32,
    id: u32,
}

struct Scene {
    bodies: Vec<BodyRow>,
    cells: Vec<CellRow>,
    shapes: Vec<ShapeRow>,
    ports: Vec<u32>,
    links: Vec<u32>,
    incidence: Vec<u32>,
    styles: Vec<u32>,
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

fn scene(n: u32) -> Scene {
    let mut rng = Rng(0x5eed_0000_0000_0001 ^ n as u64);
    let n_bodies = 100u32;
    let per_body = n / n_bodies;
    let mut bodies = Vec::new();
    for _ in 0..n_bodies {
        let scale = [0.5f32, 1.0, 2.0][rng.below(3) as usize];
        bodies.push(BodyRow {
            chart: [
                rng.below(W as u64) as f32,
                rng.below(H as u64) as f32,
                scale,
                0.0,
            ],
        });
    }
    let mut cells = Vec::new();
    let mut shapes = Vec::new();
    for b in 0..n_bodies {
        for _ in 0..per_body {
            let ox = rng.below(401) as f32 - 200.0;
            let oy = rng.below(401) as f32 - 200.0;
            let hx = 2.0 + rng.below(30) as f32;
            let hy = 2.0 + rng.below(30) as f32;
            let r = rng.below(hx.min(hy) as u64 + 1) as f32;
            cells.push(CellRow {
                local: [ox, oy, hx, hy],
                body: b,
                style: rng.below(16) as u32,
                generation: 1,
                flags: 0,
            });
            shapes.push(ShapeRow {
                cell: cells.len() as u32 - 1,
                radius: r,
                _pad: [0; 2],
            });
        }
    }
    let styles = (0..16u32)
        .map(|i| 0xff40_4040 | (i.wrapping_mul(0x000f_0b07) & 0x00ff_ffff))
        .collect();
    // Ports, links and incidence are bound (the layout must fit them) but not
    // drawn: wires are Phase 6's business, not this spike's.
    let ports = (0..cells.len() as u32 * 2).collect();
    let links = (0..cells.len() as u32).collect();
    let incidence = (0..cells.len() as u32 * 2).collect();
    Scene {
        bodies,
        cells,
        shapes,
        ports,
        links,
        incidence,
        styles,
    }
}

/// The rectangle shape i must become, with the same f32 arithmetic as the
/// storage-path vertex shader.
fn resolve(s: &Scene, i: usize) -> VertexRow {
    let sh = s.shapes[i];
    let c = s.cells[sh.cell as usize];
    let b = s.bodies[c.body as usize];
    let k = b.chart[2];
    VertexRow {
        center: [b.chart[0] + k * c.local[0], b.chart[1] + k * c.local[1]],
        half: [k * c.local[2], k * c.local[3]],
        radius: k * sh.radius,
        color: s.styles[c.style as usize],
        id: i as u32 + 1,
    }
}

fn sdf(v: &VertexRow, px: f64, py: f64) -> f64 {
    let r = v.radius as f64;
    let qx = (px - v.center[0] as f64).abs() - (v.half[0] as f64 - r);
    let qy = (py - v.center[1] as f64).abs() - (v.half[1] as f64 - r);
    let outside = (qx.max(0.0).powi(2) + qy.max(0.0).powi(2)).sqrt();
    outside + qx.max(qy).min(0.0) - r
}

enum CpuPick {
    Id(u32),
    Edge,
}

/// Last drawn wins. A pixel centre within 0.01 px of an edge is too close to
/// call either way in floating point and is counted separately.
fn cpu_pick(rects: &[VertexRow], x: u32, y: u32) -> CpuPick {
    let (px, py) = (x as f64 + 0.5, y as f64 + 0.5);
    for v in rects.iter().rev() {
        let d = sdf(v, px, py);
        if d.abs() < 0.01 {
            return CpuPick::Edge;
        }
        if d < 0.0 {
            return CpuPick::Id(v.id);
        }
    }
    CpuPick::Id(0)
}

// ---------------------------------------------------------------------------
// Shaders
// ---------------------------------------------------------------------------

const COMMON: &str = r#"
struct Out {
    @builtin(position) pos: vec4<f32>,
    @location(0) @interpolate(flat) center: vec2<f32>,
    @location(1) @interpolate(flat) half_size: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(3) @interpolate(flat) color: u32,
    @location(4) @interpolate(flat) id: u32,
};
struct Tick { viewport: vec4<f32> };
@group(0) @binding(0) var<uniform> tick: Tick;

fn corner(vi: u32) -> vec2<f32> {
    var c = array<vec2<f32>, 6>(vec2(-1.0, -1.0), vec2(1.0, -1.0), vec2(1.0, 1.0),
                                vec2(-1.0, -1.0), vec2(1.0, 1.0), vec2(-1.0, 1.0));
    return c[vi];
}

fn emit(vi: u32, center: vec2<f32>, half_size: vec2<f32>, radius: f32, color: u32, id: u32) -> Out {
    let p = center + corner(vi) * (half_size + vec2(1.0, 1.0));
    var o: Out;
    o.pos = vec4(p.x / tick.viewport.x * 2.0 - 1.0, 1.0 - p.y / tick.viewport.y * 2.0, 0.0, 1.0);
    o.center = center;
    o.half_size = half_size;
    o.radius = radius;
    o.color = color;
    o.id = id;
    return o;
}

struct Targets {
    @location(0) color: vec4<f32>,
    @location(1) id: u32,
};

@fragment
fn fs(i: Out) -> Targets {
    let q = abs(i.pos.xy - i.center) - (i.half_size - vec2(i.radius, i.radius));
    let d = length(max(q, vec2(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - i.radius;
    if (d >= 0.0) { discard; }
    var t: Targets;
    t.color = unpack4x8unorm(i.color);
    t.id = i.id;
    return t;
}
"#;

const STORAGE_VS: &str = r#"
struct Body { chart: vec4<f32> };
struct Cell { local: vec4<f32>, body: u32, style: u32, generation: u32, flags: u32 };
struct Shape { cell: u32, radius: f32, pad0: u32, pad1: u32 };

// §7.2: group 0 Tick, group 1 Universe, group 2 Genome, group 3 Pass.
@group(1) @binding(0) var<storage, read> bodies: array<Body>;
@group(1) @binding(1) var<storage, read> cells: array<Cell>;
@group(1) @binding(2) var<storage, read> ports: array<u32>;
@group(1) @binding(3) var<storage, read> links: array<u32>;
@group(1) @binding(4) var<storage, read> incidence: array<u32>;
@group(1) @binding(5) var<storage, read> shapes: array<Shape>;
@group(2) @binding(0) var<storage, read> styles: array<u32>;
struct Pass { flags: vec4<u32> };
@group(3) @binding(0) var<uniform> pass_data: Pass;

@vertex
fn vs(@builtin(vertex_index) vi: u32, @builtin(instance_index) ii: u32) -> Out {
    let s = shapes[ii];
    let c = cells[s.cell];
    let b = bodies[c.body];
    let k = b.chart.z;
    // Touch the undrawn tables so no backend strips their bindings.
    let keep = (ports[0] + links[0] + incidence[0] + pass_data.flags.x) & 0u;
    return emit(vi, b.chart.xy + k * c.local.xy, k * c.local.zw, k * s.radius,
                styles[c.style], ii + 1u + keep);
}
"#;

const VERTEX_VS: &str = r#"
@vertex
fn vs(@builtin(vertex_index) vi: u32,
      @location(0) center: vec2<f32>, @location(1) half_size: vec2<f32>,
      @location(2) radius: f32, @location(3) color: u32, @location(4) id: u32) -> Out {
    return emit(vi, center, half_size, radius, color, id);
}
"#;

// ---------------------------------------------------------------------------
// GPU plumbing
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
enum Path {
    Storage,
    Vertex,
}

struct Run {
    median_ms: f64,
    p95_ms: f64,
    ids: Vec<u32>,
}

fn bytes<T: Copy>(v: &[T]) -> &[u8] {
    // Plain-old-data rows with explicit padding; the spike needs no bytemuck.
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, std::mem::size_of_val(v)) }
}

fn buffer(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    data: &[u8],
    usage: wgpu::BufferUsages,
) -> wgpu::Buffer {
    let b = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: (data.len() as u64).max(16).next_multiple_of(4),
        usage: usage | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    queue.write_buffer(&b, 0, data);
    b
}

fn wait(device: &wgpu::Device) {
    let _ = device.poll(wgpu::PollType::wait_indefinitely());
}

fn read_buffer(device: &wgpu::Device, buf: &wgpu::Buffer) -> Vec<u8> {
    let (tx, rx) = mpsc::channel();
    buf.slice(..).map_async(wgpu::MapMode::Read, move |r| {
        let _ = tx.send(r);
    });
    wait(device);
    let ok = rx.recv().map(|r| r.is_ok()).unwrap_or(false);
    let out = if ok {
        buf.slice(..)
            .get_mapped_range()
            .map(|v| v.to_vec())
            .unwrap_or_default()
    } else {
        Vec::new()
    };
    buf.unmap();
    out
}

fn storage_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only: true },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn uniform_entry() -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn layout(
    device: &wgpu::Device,
    label: &str,
    entries: &[wgpu::BindGroupLayoutEntry],
) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some(label),
        entries,
    })
}

fn bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    bufs: &[&wgpu::Buffer],
) -> wgpu::BindGroup {
    let entries: Vec<_> = bufs
        .iter()
        .enumerate()
        .map(|(i, b)| wgpu::BindGroupEntry {
            binding: i as u32,
            resource: b.as_entire_binding(),
        })
        .collect();
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout,
        entries: &entries,
    })
}

fn id_texture(device: &wgpu::Device, format: wgpu::TextureFormat) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: wgpu::Extent3d {
            width: W,
            height: H,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    })
}

fn run(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    s: &Scene,
    rects: &[VertexRow],
    path: Path,
) -> Result<Run, String> {
    let scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
    let n = s.shapes.len() as u32;
    let tick_buf = buffer(
        device,
        queue,
        bytes(&[W as f32, H as f32, 0.0, 0.0]),
        wgpu::BufferUsages::UNIFORM,
    );
    let tick_layout = layout(device, "0 tick", &[uniform_entry()]);
    let mut groups = vec![bind_group(device, &tick_layout, &[&tick_buf])];
    let mut layouts = vec![tick_layout];
    let mut vbuf = None;

    let src = match path {
        Path::Storage => format!("{COMMON}{STORAGE_VS}"),
        Path::Vertex => format!("{COMMON}{VERTEX_VS}"),
    };
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(src.into()),
    });

    match path {
        Path::Storage => {
            let st = wgpu::BufferUsages::STORAGE;
            let universe = [
                buffer(device, queue, bytes(&s.bodies), st),
                buffer(device, queue, bytes(&s.cells), st),
                buffer(device, queue, bytes(&s.ports), st),
                buffer(device, queue, bytes(&s.links), st),
                buffer(device, queue, bytes(&s.incidence), st),
                buffer(device, queue, bytes(&s.shapes), st),
            ];
            let genome = buffer(device, queue, bytes(&s.styles), st);
            let pass = buffer(
                device,
                queue,
                bytes(&[0u32; 4]),
                wgpu::BufferUsages::UNIFORM,
            );
            let l1 = layout(
                device,
                "1 universe",
                &(0..6).map(storage_entry).collect::<Vec<_>>(),
            );
            let l2 = layout(device, "2 genome", &[storage_entry(0)]);
            let l3 = layout(device, "3 pass", &[uniform_entry()]);
            groups.push(bind_group(
                device,
                &l1,
                &universe.iter().collect::<Vec<_>>(),
            ));
            groups.push(bind_group(device, &l2, &[&genome]));
            groups.push(bind_group(device, &l3, &[&pass]));
            layouts.extend([l1, l2, l3]);
        }
        Path::Vertex => {
            vbuf = Some(buffer(
                device,
                queue,
                bytes(rects),
                wgpu::BufferUsages::VERTEX,
            ));
        }
    }
    let layout_refs: Vec<Option<&wgpu::BindGroupLayout>> = layouts.iter().map(Some).collect();
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &layout_refs,
        immediate_size: 0,
    });
    let attrs = wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32, 3 => Uint32, 4 => Uint32];
    let vlayout = [Some(wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<VertexRow>() as u64,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &attrs,
    })];
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &module,
            entry_point: Some("vs"),
            compilation_options: Default::default(),
            buffers: if path == Path::Vertex { &vlayout } else { &[] },
        },
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(wgpu::FragmentState {
            module: &module,
            entry_point: Some("fs"),
            compilation_options: Default::default(),
            targets: &[
                Some(wgpu::TextureFormat::Rgba8Unorm.into()),
                Some(wgpu::TextureFormat::R32Uint.into()),
            ],
        }),
        multiview_mask: None,
        cache: None,
    });

    let color = id_texture(device, wgpu::TextureFormat::Rgba8Unorm);
    let id_tex = id_texture(device, wgpu::TextureFormat::R32Uint);
    let color_view = color.create_view(&Default::default());
    let id_view = id_tex.create_view(&Default::default());

    if let Some(err) = pollster::block_on(scope.pop()) {
        return Err(format!("setup refused: {err}"));
    }

    let frame = || {
        let mut enc = device.create_command_encoder(&Default::default());
        {
            let attachment = |view, clear| {
                Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear),
                        store: wgpu::StoreOp::Store,
                    },
                })
            };
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[
                    attachment(&color_view, wgpu::Color::BLACK),
                    attachment(&id_view, wgpu::Color::TRANSPARENT),
                ],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_pipeline(&pipeline);
            for (i, g) in groups.iter().enumerate() {
                pass.set_bind_group(i as u32, g, &[]);
            }
            if let Some(v) = &vbuf {
                pass.set_vertex_buffer(0, v.slice(..));
            }
            pass.draw(0..6, 0..n);
        }
        queue.submit([enc.finish()]);
        wait(device);
    };

    for _ in 0..WARMUP {
        frame();
    }
    let mut times: Vec<f64> = (0..FRAMES)
        .map(|_| {
            let t = Instant::now();
            frame();
            t.elapsed().as_secs_f64() * 1000.0
        })
        .collect();
    times.sort_by(|a, b| a.total_cmp(b));

    // Full ID readback (row pitch 1920 * 4 = 7680, a multiple of 256).
    let out = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: (W * H * 4) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut enc = device.create_command_encoder(&Default::default());
    enc.copy_texture_to_buffer(
        id_tex.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &out,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(W * 4),
                rows_per_image: Some(H),
            },
        },
        wgpu::Extent3d {
            width: W,
            height: H,
            depth_or_array_layers: 1,
        },
    );
    queue.submit([enc.finish()]);
    let raw = read_buffer(device, &out);
    if raw.is_empty() {
        return Err("ID readback map failed".into());
    }
    let ids = raw
        .chunks_exact(4)
        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect();

    Ok(Run {
        median_ms: times[FRAMES / 2],
        p95_ms: times[FRAMES * 95 / 100],
        ids,
    })
}

/// The pick path by itself: copy one texel of an ID target, map it, wait.
/// Returns the median latency in ms.
fn pick_latency(device: &wgpu::Device, queue: &wgpu::Queue) -> Option<f64> {
    let tex = id_texture(device, wgpu::TextureFormat::R32Uint);
    let mut rng = Rng(42);
    let mut times = Vec::new();
    for _ in 0..PICKS {
        let buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 256,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let t = Instant::now();
        let mut enc = device.create_command_encoder(&Default::default());
        enc.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &tex,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: rng.below(W as u64) as u32,
                    y: rng.below(H as u64) as u32,
                    z: 0,
                },
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &buf,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(256),
                    rows_per_image: Some(1),
                },
            },
            wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
        );
        queue.submit([enc.finish()]);
        if read_buffer(device, &buf).is_empty() {
            return None;
        }
        times.push(t.elapsed().as_secs_f64() * 1000.0);
    }
    times.sort_by(|a, b| a.total_cmp(b));
    Some(times[PICKS / 2])
}

struct Sweep {
    agree: usize,
    disagree: usize,
    edge: usize,
    first_bad: Option<String>,
}

fn sweep(ids: &[u32], rects: &[VertexRow]) -> Sweep {
    let mut rng = Rng(0x0bad_cafe);
    let mut r = Sweep {
        agree: 0,
        disagree: 0,
        edge: 0,
        first_bad: None,
    };
    for _ in 0..SWEEP {
        let (x, y) = (rng.below(W as u64) as u32, rng.below(H as u64) as u32);
        let gpu = ids[(y * W + x) as usize];
        match cpu_pick(rects, x, y) {
            CpuPick::Edge => r.edge += 1,
            CpuPick::Id(cpu) if cpu == gpu => r.agree += 1,
            CpuPick::Id(cpu) => {
                r.disagree += 1;
                if r.first_bad.is_none() {
                    r.first_bad = Some(format!("({x}, {y}): cpu {cpu}, gpu {gpu}"));
                }
            }
        }
    }
    r
}

// ---------------------------------------------------------------------------
// Report
// ---------------------------------------------------------------------------

fn main() {
    let mut md = String::new();
    let mut problems: Vec<String> = Vec::new();
    let _ = writeln!(md, "# S2 run results\n");
    let _ = writeln!(
        md,
        "Written by `cargo run --release` in `joinn/spikes/s2-gpu` ({} on {}).\n",
        std::env::consts::OS,
        std::env::consts::ARCH
    );
    let _ = writeln!(
        md,
        "Target {W}×{H}, headless (no window, no vsync). Frame time = encode + submit + wait for the GPU: \
         median and 95th percentile of {FRAMES} frames after {WARMUP} warm-up frames. \
         Interactive means a median under {INTERACTIVE_MS:.1} ms.\n"
    );

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    let mut adapters = pollster::block_on(instance.enumerate_adapters(wgpu::Backends::PRIMARY));
    // Also ask for the software fallback explicitly, in case enumeration hid it.
    if let Ok(a) = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::LowPower,
        force_fallback_adapter: true,
        compatible_surface: None,
        apply_limit_buckets: false,
    })) {
        let info = a.get_info();
        if !adapters.iter().any(|b| {
            let bi = b.get_info();
            bi.name == info.name && bi.backend == info.backend
        }) {
            adapters.push(a);
        }
    }
    if adapters.is_empty() {
        let _ = writeln!(md, "**No adapter found.** Nothing could be measured.");
        println!("{md}");
        write_results(&md);
        return;
    }

    for adapter in &adapters {
        let info = adapter.get_info();
        let lim = adapter.limits();
        let down = adapter.get_downlevel_capabilities();
        let vertex_storage = down.flags.contains(wgpu::DownlevelFlags::VERTEX_STORAGE);
        let title = format!(
            "{} · {:?} · {:?}",
            info.name, info.backend, info.device_type
        );
        println!("== {title}");
        let _ = writeln!(md, "## {title}\n");
        let _ = writeln!(md, "Driver: {} {}\n", info.driver, info.driver_info);
        let _ = writeln!(
            md,
            "| Adapter reports | Value | Core minimum |\n|---|---|---|"
        );
        let _ = writeln!(md, "| `max_bind_groups` | {} | 4 |", lim.max_bind_groups);
        let _ = writeln!(
            md,
            "| `max_storage_buffers_per_shader_stage` | {} | 8 |",
            lim.max_storage_buffers_per_shader_stage
        );
        let _ = writeln!(
            md,
            "| `max_storage_buffer_binding_size` | {} | 134217728 |",
            lim.max_storage_buffer_binding_size
        );
        let _ = writeln!(
            md,
            "| `max_vertex_buffers` | {} | 8 |",
            lim.max_vertex_buffers
        );
        let _ = writeln!(
            md,
            "| Storage buffers in the vertex stage | {} | core: yes · compatibility: no |",
            if vertex_storage { "yes" } else { "**no**" }
        );
        let _ = writeln!(
            md,
            "| WebGPU compliant (wgpu downlevel check) | {} | |\n",
            down.is_webgpu_compliant()
        );

        let device = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::defaults(),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            memory_hints: wgpu::MemoryHints::Performance,
            trace: wgpu::Trace::Off,
        }));
        let (device, queue) = match device {
            Ok(d) => d,
            Err(e) => {
                let _ = writeln!(md, "**Device with WebGPU core limits refused:** {e}\n");
                problems.push(format!("{title}: core-limits device refused ({e})"));
                continue;
            }
        };
        let _ = writeln!(
            md,
            "Device requested with exactly the WebGPU core limits (`Limits::defaults()`): granted.\n"
        );
        let _ = writeln!(
            md,
            "| Instances | Path | Frame median | Frame p95 | Interactive | Pick sweep ({SWEEP} points): agree / disagree / on an edge | S and V ID images |"
        );
        let _ = writeln!(md, "|---|---|---|---|---|---|---|");

        for &n in &SIZES {
            let s = scene(n);
            let rects: Vec<VertexRow> = (0..s.shapes.len()).map(|i| resolve(&s, i)).collect();
            let mut storage_image: Option<Vec<u32>> = None;
            for path in [Path::Storage, Path::Vertex] {
                let label = match path {
                    Path::Storage => "S · storage, walked in the vertex shader",
                    Path::Vertex => "V · vertex-buffer fallback",
                };
                if path == Path::Storage && !vertex_storage {
                    let _ = writeln!(
                        md,
                        "| {n} | {label} | — | — | — | not run: no vertex-stage storage | |"
                    );
                    continue;
                }
                match run(&device, &queue, &s, &rects, path) {
                    Ok(r) => {
                        let sw = sweep(&r.ids, &rects);
                        let interactive = r.median_ms < INTERACTIVE_MS;
                        let cmp = match (&storage_image, path) {
                            (Some(img), Path::Vertex) => {
                                let diff = img.iter().zip(&r.ids).filter(|(a, b)| a != b).count();
                                if diff == 0 {
                                    "identical".to_string()
                                } else {
                                    format!("**{diff} pixels differ**")
                                }
                            }
                            _ => String::new(),
                        };
                        let bad = sw
                            .first_bad
                            .map(|b| format!(" (first: {b})"))
                            .unwrap_or_default();
                        let _ = writeln!(
                            md,
                            "| {n} | {label} | {:.2} ms | {:.2} ms | {} | {} / {} / {}{bad} | {cmp} |",
                            r.median_ms,
                            r.p95_ms,
                            if interactive { "yes" } else { "**no**" },
                            sw.agree,
                            sw.disagree,
                            sw.edge
                        );
                        println!(
                            "  {n:>6} {label}: median {:.2} ms, p95 {:.2} ms, sweep {} agree / {} disagree / {} edge {cmp}",
                            r.median_ms, r.p95_ms, sw.agree, sw.disagree, sw.edge
                        );
                        if sw.disagree > 0 {
                            problems.push(format!(
                                "{title}: {n}, {label}: {} pick disagreements",
                                sw.disagree
                            ));
                        }
                        if n == SIZES[0] && !interactive {
                            problems.push(format!(
                                "{title}: {n}, {label}: not interactive ({:.2} ms)",
                                r.median_ms
                            ));
                        }
                        if cmp.starts_with("**") {
                            problems.push(format!(
                                "{title}: {n}: storage and vertex paths drew different ID images"
                            ));
                        }
                        if path == Path::Storage {
                            storage_image = Some(r.ids);
                        }
                    }
                    Err(e) => {
                        let _ = writeln!(md, "| {n} | {label} | — | — | — | refused: {e} | |");
                        problems.push(format!("{title}: {n}, {label}: refused ({e})"));
                    }
                }
            }
        }
        match pick_latency(&device, &queue) {
            Some(ms) => {
                let _ = writeln!(
                    md,
                    "\nSingle-pixel ID readback (copy one texel, map, wait), median of {PICKS}: **{ms:.2} ms**\n"
                );
            }
            None => {
                let _ = writeln!(md, "\nSingle-pixel ID readback: **map failed**\n");
                problems.push(format!("{title}: single-pixel readback failed"));
            }
        }
        if !vertex_storage {
            problems.push(format!(
                "{title}: no vertex-stage storage; only the fallback path ran"
            ));
        }
    }

    let _ = writeln!(md, "## Summary\n");
    if problems.is_empty() {
        let _ = writeln!(
            md,
            "Every adapter granted a core-limits device, fit §7.2's four bind groups, drew {} rects at interactive rates on both paths, picked every non-edge point correctly, and drew identical ID images on both paths.",
            SIZES[0]
        );
        println!("\nNo problems.");
    } else {
        for p in &problems {
            let _ = writeln!(md, "- {p}");
            println!("{p}");
        }
    }
    write_results(&md);
}

fn write_results(md: &str) {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/RESULTS.md");
    match std::fs::write(path, md) {
        Ok(()) => println!("\nWrote {path}"),
        Err(e) => println!("\nCould not write {path}: {e}"),
    }
}
