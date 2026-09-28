# JoInn Phase 6 Implementation Plan

**Visual Host I: draw and pick · every pixel knows who owns it · a working plan for Cursor**

Author: AJ, with Claude · Draft 0.1 · September 28, 2026

> **Every decision in this plan is final.** Nothing here waits on AJ. Cursor never stops to ask AJ anything, and no file in this phase is typed by AJ. If a step can't be done the way the plan says, Cursor follows §0.3 (Snags) and keeps going.

---

## For AJ: this plan in plain English

**What Phase 6 is.** This is the first time JoInn draws itself. The calculator body appears in a window as a membrane holding three cells, with ports and wires. Clicking anything prints exactly what you clicked (`body.sum@2`, `body wire cli_a@1 -> sum@0`). Typing a number into an in-port runs the body, and the picture changes because the engine changed a row in a table, not because anything redrew from scratch.

**Your decisions are built in:**

1. **Floats only below the renderer boundary.** Decimal numbers are allowed in the two new crates that touch the GPU and the window (`joinn-gpu`, `joinn-shell-desktop`) and nowhere else. A scan fails the build if one shows up anywhere else.
2. **GitHub checks GPU code with software GPUs.** lavapipe on Linux, WARP on Windows. S2 showed both give exact picks, only slower. Every GPU check runs on every adapter it finds, and fails loudly if it finds none. It never skips.
3. **R64 and R69 carry forward.** Neither is touched in this phase.
4. **S2's result is committed first,** and its trade-off is built in. The GPU reads the tables directly, as §7 designs it. Every table write is counted, and a camera change writes zero table rows.

**Choices Claude made while writing this. Say if you disagree before Cursor starts:**

- **No text on screen yet.** Drawing text means borrowing a text-shaping crate and building its inverse (point → character), which is its own gate item. It belongs with Phase 7's zoom bands (dot, glyph, summary, full). In Phase 6 the terminal is the text face: every click and every run prints there. The window shows structure. Port values show as a port that fills in.
- **Layout is computed, not authored.** Cells are placed by a fixed rule on a whole-number grid: columns follow the wires, rows go in name order. No new file grammar, so no hash can move. Authored layout waits for the creator (Phase 9) and is logged as R71.
- **One body at a time.** Drawing two linked bodies needs lens layout and hyperedge drawing, which is Phase 7.
- **The picker's reference is exact arithmetic.** The CPU works out who owns each pixel with whole numbers only, no floats, the same way the truth core works. The GPU is checked against it on every pixel of the image, not a sample. Pixels within 1/16 of a pixel of an edge are counted and left unjudged (S2 had 0.2% of these).
- **The fallback path isn't built.** S2 found no adapter without vertex-stage storage. `joinn-gpu` refuses such an adapter and names what's missing. That opens R72.

**The honest adversary.** The roadmap's risk for this phase is that keeping the tables in sync with a live universe turns into rebuilding everything. Phase 6 measures it directly. Every run prints how many rows it wrote, and the rule is at most one cell row plus that cell's port rows for each instance the run touched. A resize writes zero rows. Tables built step by step must equal tables regrown from scratch, byte for byte.

**What you do at the end (Stop C), about two minutes.** Run the window and click around. The exact steps are in §6.2. You report "it worked" or what looked wrong. You never copy output.

**Your prompts to Cursor** (copy exactly, one per chunk):

- Chunk A: `Do chunk A of docs/Plans/JoInn Phase 6 Implementation Plan.md. Follow AGENTS.md.`
- Chunk B: `Do chunk B of docs/Plans/JoInn Phase 6 Implementation Plan.md. Follow AGENTS.md.`
- Chunk C: `Do chunk C of docs/Plans/JoInn Phase 6 Implementation Plan.md. Follow AGENTS.md.`

If Cursor runs out of room partway: `Continue chunk A of docs/Plans/JoInn Phase 6 Implementation Plan.md from the first commit not in git log. Follow AGENTS.md.` (Use the right letter.)

After each stop, tell Claude "Cursor finished chunk A" (or B, C).

---

## 0. How Cursor Works This Plan

| | |
|---|---|
| **Plan file** | `D:\JoInn\docs\Plans\JoInn Phase 6 Implementation Plan.md` |
| **Code** | `D:\JoInn\joinn\`. Three new crates (§2.1). New dependencies only as listed in §2.2 |
| **Standing rules** | `AGENTS.md` as updated by P6-01 (Appendix A) |
| **Unit of work** | a **chunk** (A, B or C). Inside a chunk, do the commits in the order listed. Each commit is its own git commit, and its message starts with its id (`P6-05: …`) |
| **End of a chunk** | write the stop report (§0.2), commit it, `git push`, and stop. Do not start the next chunk |
| **Who decides** | every decision is in §2. Cursor decides only module layout (rule 25), function bodies, error strings, WGSL function bodies, and Rust representation of private types |
| **Who checks** | Claude, at each stop, from a fresh clone on Linux (with lavapipe). GitHub CI runs on Windows and Linux on every push. AJ runs the window at Stop C |

### 0.1 The commit report

Every commit ends with this block, in the commit message body **and** in the chunk's stop report. Every value is copied from the terminal, never summarised.

```
Commit:     P6-NN (hash in git log)
Done-when:  <the command> → <the line it printed>          MET | NOT MET
Suite:      cargo test --workspace --no-fail-fast → <N passed, M failed>
Scans:      vocab → <last line> · modules → <last line> · layers → <last line> (from P6-03)
Snags:      none | <each thing that went differently from the plan, with the printed line>
```

### 0.2 The stop report

At the end of each chunk Cursor writes `docs/Findings/phase-6-stop-<letter>.md` with:

1. `git rev-parse HEAD` (full hash).
2. Every commit report block from the chunk, in order, with real hashes.
3. The last line of each of: `cargo test --workspace --no-fail-fast`, `cargo xtask gate all`, `cargo xtask corpus verify`, `cargo xtask vocab`, `cargo xtask modules`, `cargo xtask layers`, `cargo xtask assay agree`, and from P6-08 on `cargo xtask adapters`, `cargo xtask pick`, `cargo xtask regrow`. Also every line of `gate all` that contains `fail`, and every failing test name.
4. **CI**: for the newest run after the push, read with PowerShell `Invoke-RestMethod "https://api.github.com/repos/joneseysinno/JoInn/actions/runs?branch=main&per_page=3"` and that run's `jobs_url`. Paste `head_sha`, `status`, `conclusion`, and each job's `name` and `conclusion`. If the run hasn't finished, wait and read again. A failed job is a snag, not a reason to change CI.
5. **Snags**: every snag from the chunk in one list, each tagged with its commit id. A snag says what the plan actually says.

Then commit (`P6-stop-<letter>: stop report`), push, and stop.

### 0.3 Snags

A snag is any point where a done-when can't be met as written, or a prediction in this plan turns out wrong.

- **Never** fake it, work around a scan, weaken a test, rebless a golden, or change the plan's meaning to make it pass.
- **A prediction that differs is not a failure.** Paste what the machine printed next to what the plan predicted, and continue. Claude settles it at the stop.
- Leave that piece in its honest state and **continue with the next commit**.
- If a later commit depends on the snagged one (§4), skip it and write `skipped: depends on P6-NN`.

---

## 1. Scope Fence

### In scope

- **`joinn-visual`** (the upward wrap): layout, camera, the visual vocabulary Phase 6 needs (Shape, Port, Link as a wire, Chart as the camera), the tables of Part II §7.1 with generational slots, the delta protocol, exact CPU pick, and address resolution.
- **`joinn-gpu`** (the downward wrap): adapter and device at WebGPU core limits, §7.2's bind groups, two organelles (SDF shape and curve), the ID target, table upload, delta upload, offscreen render, and readback.
- **`joinn-shell-desktop`** (L0): a winit window that renders on demand, click to pick, type into an in-port, regrow on device or surface loss. Binary `joinn-desktop`.
- **xtask**: `layers`, `layout`, `adapters`, `pick`, `regrow`, and gate 6.
- **CI**: `ubuntu-24.04` pinned, `actions/checkout@v5`, lavapipe installed on Linux, and the new steps.
- **S2 committed**: the spike, its results and its finding.

### Out of scope (Cursor refuses these even when they look small)

| Not now | Why it is tempting |
|---|---|
| Text, glyphs, a font, a glyph atlas | Values are the obvious thing to draw. Phase 7 (R73) |
| Images, snapshots, bands, zoom, pan, the chart chain | Phase 7 |
| Two or more bodies in one picture, lenses, hyperedge forms | Phase 7 |
| Accessibility tree, AccessKit, golden pixels | Phase 8 |
| A `layout` section in any file kind | R71; new grammar, and the creator's job |
| The vertex-buffer fallback path | R72; no adapter needed it (S2) |
| Antialiasing, multisampling, blending on the ID target | An integer ID target cannot be blended (§12.3) |
| Mobile, web, wasm | Phase 11 |
| Hover effects, animation, a tick loop that runs while idle | V12: an idle universe draws nothing |
| Any change to a corpus file, a hash, `grandfather.txt`, a golden | Never |
| Upgrading gates 1–3 (R64); describe-at-firing-time (R69) | Carried forward (AJ, 28 Sep) |

---

## 2. Decisions

All **DECIDED**.

### 2.1 The three crates and the boundary

```
joinn-frame · joinn-dna · joinn-gate · joinn-prim · joinn-live · joinn-assay
joinn-link · joinn-host · joinn-test-host · joinn-cli · joinn-visual        ← above the boundary: exact, no float, no wgpu
═══════════════════════════ renderer boundary ═══════════════════════════
joinn-gpu                                                                   ← downward wrap: wgpu, f32
joinn-shell-desktop                                                         ← L0: winit, wgpu surface, f64 from the OS
```

| Crate | Depends on (workspace) | Depends on (external) | IO | Floats |
|---|---|---|---|---|
| `joinn-visual` | frame, dna, link, host, live | none | no | **no** |
| `joinn-gpu` | frame, visual | `wgpu`, `bytemuck`, `pollster` | no | yes |
| `joinn-shell-desktop` | frame, dna, gate, prim, live, link, host, visual, gpu | `wgpu`, `winit`, `pollster` | yes (loads corpus files) | yes |
| `xtask` | adds visual, gpu | none new | yes | no |

- **V1 is a compile error.** `joinn-visual` has no `wgpu` dependency, so naming a wgpu type there does not compile. `cargo xtask layers` (§2.9) makes the table above mechanical: every edge that isn't in it is refused.
- **joinn-visual is exact.** Every coordinate it holds is an integer in layout units. Every pick it computes uses integer arithmetic (§2.5). It is the reference allele of drawing, and the GPU is checked against it.
- **joinn-link gains `instance_ports`** (P6-04): `pub fn instance_ports(body: &Body, cells: &BTreeMap<Hash, Cell>) -> Verdict<BTreeMap<String, Vec<Port>>>`. Every instance maps to its ports in position order, from the cell's contract or `prim_ports`, with the same refusals `membrane` gives today. `membrane` is rewritten to call it. One derivation of "what ports does this instance have", and no output of `membrane` changes.

### 2.2 Dependencies

Added to `docs/Findings/dependencies.md` in P6-01, each with the crate that may use it:

| Crate | Version | Used by | Provides |
|---|---|---|---|
| `wgpu` | `30` | joinn-gpu, joinn-shell-desktop | The GPU API (validated by S2 at 30.0.1) |
| `winit` | `0.30` | joinn-shell-desktop | Window, event loop, input |
| `pollster` | `0.4` | joinn-gpu, joinn-shell-desktop | Blocking on wgpu's async setup |
| `bytemuck` | `1` (feature `derive`) | joinn-gpu | Casting GPU uniform structs to bytes. Not on any DNA type |

Nothing else. joinn-visual encodes its own rows to bytes by hand (§2.6), so it needs no `bytemuck`.

### 2.3 Layout (joinn-visual)

A body's layout is a **pure function of its coding region and its cells' contracts**. Labels, names, prompts, alleles and lenses cannot reach it (V119). All numbers are integers in **layout units**.

| Constant | Value |
|---|---|
| `MARGIN` (membrane to cells) | 4 |
| `CELL_W` | 12 |
| `GAP_X` (between columns) | 8 |
| `GAP_Y` (between cells in a column) | 4 |
| `PITCH` (between ports on one side) | 4 |
| Cell height | `4 × max(in-ports, out-ports, 1) + 2` |
| Port centre y | `top + 3 + 4 × k`, for the k-th port on that side in position order |
| In-ports | on the left edge (`x = left`); out-ports on the right edge (`x = left + CELL_W`) |
| Radii | port 1 · cell 2 · membrane 3 |
| Wire half-width | 1/4 unit |

1. **Column** of an instance = the length of the longest wire path that ends at it (instances with no incoming wire are column 0). Wires go `src → dst`. A wire cycle inside a body is refused: `layout: wire cycle through <instances in name order>; acceptance is a body whose wires form no cycle`.
2. **Rows**: within a column, instances in name order, stacked from `top = MARGIN` with `GAP_Y` between them.
3. **Column x**: `left = MARGIN + column × (CELL_W + GAP_X)`.
4. **Membrane**: the rectangle `(0, 0)` to `(W, H)` with `W = 2·MARGIN + columns·CELL_W + (columns − 1)·GAP_X` and `H = 2·MARGIN + the tallest column's stack height`.
5. **Wires**: straight, from the source out-port's centre to the destination in-port's centre. The geometry is not stored. It is read from the two port rows through the incidence table (§2.6), so moving a port moves its wires without writing them.

`print_layout` prints the canonical form. **Claude's prediction** for `phase2/calculator.body`, computed by hand from the rules above (P6-05 asserts it byte for byte):

```
layout body
membrane 0 0 40 24 r 3
cell cli_a 4 4 12 6 r 2
cell cli_b 4 14 12 6 r 2
cell sum 24 4 12 10 r 2
port cli_a@0 in 4 7 r 1
port cli_a@1 out 16 7 r 1
port cli_b@0 in 4 17 r 1
port cli_b@1 out 16 17 r 1
port sum@0 in 24 7 r 1
port sum@1 in 24 11 r 1
port sum@2 out 36 7 r 1
wire cli_a@1 -> sum@0 16 7 24 7 w 1/4
wire cli_b@1 -> sum@1 16 17 24 11 w 1/4
```

A rectangle prints `x y width height`. Cells print in name order, ports by instance name then position, and wires in the order `print_body` prints them.

### 2.4 The camera (Chart, Phase 6 form)

A camera is `k` (pixels per layout unit), an integer origin `(ox, oy)` in pixels, and the viewport size. A layout point `(u, v)` is pixel point `(ox + k·u, oy + k·v)`.

- **Fit rule**: `k` is the largest multiple of 4 with `W·k ≤ width − 32` and `H·k ≤ height − 32`, and at least 4. `ox = ⌊(width − W·k) / 2⌋`, `oy = ⌊(height − H·k) / 2⌋`.
- `k` is a multiple of 4, so the wire half-width `k/4` is a whole number of pixels, and every shape edge is on integer pixel coordinates.
- **A camera change writes the tick uniform and no table row** (V121). Tables hold layout units, never pixels.
- There is no zoom or pan in Phase 6. The camera changes only when the window is resized.

**Claude's prediction** for the calculator (`W 40`, `H 24`), printed by `cargo xtask layout phase2/calculator.body`:

```
camera 640x360: k 12, origin 80 36
camera 1000x777: k 24, origin 20 100
camera 1280x720: k 28, origin 80 24
camera 1920x1080: k 40, origin 160 60
```

These four are the **standard viewports**. Every calculator check runs at all four.

### 2.5 Exact CPU pick (the reference)

A pixel `(x, y)` is judged at its centre `(x + ½, y + ½)`. Every coordinate is multiplied by 16, so the centre is `(16x + 8, 16y + 8)`, and every shape parameter in pixels is multiplied by 16 as well. Everything is then an integer (`i128`, checked arithmetic, where overflow is a refusal naming the shape). The **edge band** is 1 unit = 1/16 pixel.

For a shape with signed distance `d` (negative inside), a pixel is:

- **inside** when `d ≤ −1`
- **outside** when `d ≥ 1`
- **edge** otherwise

These are decided without square roots:

- **Rounded rectangle** (centre `c`, half-extent `h`, radius `r`): `q = |p − c| − (h − r)`. If `qx ≤ 0` and `qy ≤ 0`, then `d = max(qx, qy) − r`, which is an integer. Otherwise let `s = max(qx,0)² + max(qy,0)²`. It is inside when `s ≤ (r − 1)²`, outside when `s ≥ (r + 1)²`, and edge otherwise.
- **Circle** (port): `s = |p − c|²` against `(r − 1)²` and `(r + 1)²`.
- **Capsule** (wire from `A` to `B`, half-width `w`): let `t = (p − A)·(B − A)` and `L = |B − A|²`. If `t ≤ 0`, then `s = |p − A|²`. If `t ≥ L`, then `s = |p − B|²`. Both are compared as for a circle. Otherwise let `c = (p − A) × (B − A)`, compare `c²` with `(w − 1)²·L` and `(w + 1)²·L`.

**Draw order**, and therefore **owner order**:

1. membranes (body slots, ascending)
2. cells (cell slots, ascending)
3. wires (link slots, ascending)
4. ports (port slots, ascending)

A free slot draws nothing. The owner of a pixel is found by walking shapes in **reverse** draw order:

- the first that is **inside** owns it;
- the first that is **edge** makes the pixel an **edge pixel** (counted, never judged);
- if all are **outside**, the pixel is **background**.

There are two CPU pickers:

- `cpu_pick_reference`: brute force over every shape.
- `cpu_pick`: a uniform grid of 32-pixel buckets. Each shape is entered into every bucket its bounding box (grown by 1 pixel) touches, and candidates are walked in reverse draw order.

They must agree on every pixel of every standard viewport (V120). The grid is the one the shell uses. The reference exists to be agreed with.

### 2.6 The tables (Part II §7.1, Phase 6 form)

Every row field is a `u32` or `i32`, encoded little-endian in field order, with rows padded to a multiple of 16 bytes. `joinn-visual` owns the encoding: `pub fn table_bytes(&Tables) -> TableBytes`. That is what the GPU uploads and what regrow compares. The WGSL structs in `joinn-gpu` mirror it, and a test in `joinn-gpu` creates the pipelines with `min_binding_size` equal to each row size.

| Table | Row fields | Group · binding |
|---|---|---|
| **body** | `x y w h` (membrane rectangle), `radius`, `generation`, `flags` (bit 0 live) | 1 · 0 |
| **cell** | `body`, `x y w h`, `radius`, `generation`, `style`, `flags` (bit 0 live, bit 1 refused) | 1 · 1 |
| **port** | `cell`, `position`, `direction` (0 in, 1 out), `x y` (centre), `radius`, `generation`, `flags` (bit 0 live, bit 1 filled) | 1 · 2 |
| **link** | `kind` (1 wire), `start`, `count`, `body`, `half_width_quarters`, `generation`, `flags` (bit 0 live) | 1 · 3 |
| **incidence** | one `u32` per entry: a port slot. Wire *s* owns entries `2s` (source) and `2s + 1` (destination) | 1 · 4 |
| **style** | one `u32` RGBA8 per style id (§2.8) | 2 · 0 |

Group 0 is the tick uniform (camera: `k`, `ox`, `oy`, `width`, `height`). Group 3 is the pass uniform (Phase 6 draws one pass; it holds zeros). That is §7.2 exactly: 4 bind groups, 6 storage buffers visible to the vertex stage (S2 validated 7). A table with no rows is bound as one zeroed row.

- **Generational slots** (§7.3). A slot's generation starts at 1. Freeing a slot writes the row with `live` cleared and the generation incremented, and puts the slot on a free list. Reuse takes the lowest free slot. `resolve` refuses an ID whose generation isn't the row's current one: `stale pick: <table> slot <n> generation <g>, now <g'>; acceptance is a pick taken from the current picture`.
- **Growth order** is canonical, so two builds of the same body give the same bytes: cells in name order, then each cell's ports by position, then wires in `print_body` order.

### 2.7 The Scene and the delta protocol

```rust
// joinn-visual
pub struct Scene { /* alias, layout, tables, name ↔ slot maps, pending delta */ }
impl Scene {
    pub fn grow(alias: &str, body: &Body, cells: &BTreeMap<Hash, Cell>) -> Verdict<Scene>;
    pub fn regrow(alias: &str, body: &Body, cells: &BTreeMap<Hash, Cell>, state: &BodyState) -> Verdict<Scene>;
    pub fn present(&mut self, d: &Description) -> Verdict<Delta>;
    pub fn apply_run(&mut self, reports: &[StepReport], state: &BodyState) -> Verdict<Delta>;
    pub fn replace(&mut self, body: &Body, cells: &BTreeMap<Hash, Cell>) -> Verdict<Delta>;
    pub fn take_pending(&mut self) -> Option<Delta>;          // None: nothing to draw (V12)
    pub fn tables(&self) -> &Tables;
    pub fn resolve(&self, id: [u32; 4]) -> Verdict<Owner>;     // decode the ID target
    pub fn print_owner(&self, o: &Owner) -> String;
}
pub struct Delta { pub rows: Vec<RowWrite> }                  // (table, slot), each changed row once
```

- **`present(d)`** sets the instance's port `filled` bits from `d.ports[..].value.is_some()`. Its `refused` bit is set when `d.role == Role::Refusal`. A non-refusal description **clears every** `refused` bit, which matches `BodyState`: `last_refusal` clears when anything fires. Only rows whose bytes changed enter the delta.
- **`apply_run(reports, state)`** presents `describe(state, i)` once for each instance a run touched, in first-appearance order. "Touched" means named in a report's `fired`, or the destination of a `delivered` entry (the text before the last `@`). Destinations count because a delivery fills an in-port slot before its cell fires. Without them, the picture would miss a value the engine holds.
- **A refused run** presents `describe_refusal(state, instance, reason)` for the intent's instance, the way the test host does.
- **`regrow`** builds the tables fresh:
  - `grow`, then presents `describe(state, i)` for every instance in name order;
  - then sets `refused` on instance *i* if `state.last_refusal()` is `Some` and `state.refusal_site()` names *i*.
- **The delta bound (V121).** One `apply_run` or refused-run presentation writes at most `Σ (1 + ports(i))` rows, summed over the instances it touched. A camera change writes 0 rows.
- **Regrow is the test of the delta (V2, rule 58).**
  - After every scripted event, the scene built by deltas and the scene regrown from the same `BodyState` must have **identical `table_bytes`**.
  - A `replace` is not compared this way, because slots are reused. After `replace`, every pixel's printed owner must equal that of a fresh `grow` of the new body, at every standard viewport.
- **`replace`**:
  - An instance that is still there keeps its slot and generation, and its rows are rewritten only if they changed.
  - An instance that is gone has its cell, port and link slots freed.
  - A new instance takes free slots.
  - The body row is rewritten if the membrane changed.
- **Claude's prediction** for `replace` from `calculator.body` to its `DropGenome(cli_b)` form writes **5 rows**:
  - the body row (the membrane goes from `H 24` to `H 18`)
  - `cli_b`'s cell row
  - `cli_b@0` and `cli_b@1`
  - link slot 1

  `cli_a` and `sum` do not move.

### 2.8 The GPU (joinn-gpu)

- **Adapters.** `adapters()` returns every adapter `enumerate_adapters(Backends::PRIMARY)` finds, plus the one `request_adapter` returns with `force_fallback_adapter: true` and `apply_limit_buckets: false` when it isn't already listed. It is sorted by (API name, adapter name) and printed by `cargo xtask adapters` as `<name> · <API> · <device type> · vertex storage <yes|no>`. **An empty list is a refusal**: `no GPU adapter; acceptance is at least one adapter (on Linux, install mesa-vulkan-drivers for lavapipe)`.
- **Device.** `open(adapter)` requests `Limits::defaults()` (WebGPU core) with no features. An adapter without `DownlevelFlags::VERTEX_STORAGE` is refused naming the flag and R72.
- **Organelles.** Two pipelines, both instanced, both reading the tables in the vertex shader (S2's path S):
  - **shape**: rounded rectangles and circles (membranes, cells, ports). Instances are table slots. A non-live row emits a degenerate triangle.
  - **curve**: wires as capsules. Endpoints are read through `link → incidence → port`.

  One render pass draws membranes, cells, wires, ports, in that order (§2.5). There is no blending and no multisampling. The fragment shader computes the same distance as §2.5 in f32 and discards where `d ≥ 0`.
- **Targets.** Color `Rgba8Unorm` offscreen, or the surface format in the shell. **ID target `Rgba32Uint`** (§13.1):

  | Channel | Holds |
  |---|---|
  | R | body slot + 1 (0 = background) |
  | G | cell slot + 1, or 0 for a membrane or a wire |
  | B | `0` for a membrane or a cell face; `0x1000_0000 \| position` for a port; `0x2000_0000 \| link slot` for a wire |
  | A | the owning row's generation (the cell's for cells and ports, the body's for membranes, the link's for wires) |

  Cleared to all zeros.
- **Upload.** `Renderer::upload_all(&Tables)` writes every table. `Renderer::apply(&Tables, &Delta)` writes only the rows in the delta, with adjacent slots merged into one range. It returns `Upload { rows, bytes }`.
- **Offscreen.** `render_offscreen(gpu, tables, camera) -> Verdict<Picture { color: Vec<u8>, ids: Vec<[u32; 4]> }>` renders at the camera's viewport and reads back both targets. `read_texel(…)` reads one ID texel, the pick path.
- **Style table** (decision, not truth: Phase 8 treats colors as decision):

  | id | Used for | RGBA8 |
  |---|---|---|
  | 0 | background (clear color) | `#15171C` |
  | 1 | membrane | `#22262E` |
  | 2 | cell | `#2F5D8A` |
  | 3 | cell, refused | `#B03A2E` |
  | 4 | port, empty | `#C9CED6` |
  | 5 | port, filled | `#F2B134` |
  | 6 | wire | `#8A94A3` |

### 2.9 The float fence and the layers check

- **Floats (rule 54).**
  - `vocab`'s `\bf32\b` and `\bf64\b` bans skip `crates/joinn-gpu/` and `crates/joinn-shell-desktop/` and nothing else.
  - Those two crates have their own `[lints]` table: the workspace table copied exactly, minus `float_arithmetic`. Every other crate keeps `[lints] workspace = true`.
  - A vocab fixture with `f32` in a file under a `joinn-visual` fixture path must still be flagged.
- **`std::io`/`std::fs`.** `vocab` adds `/joinn-shell-desktop/` to the IO-allowed paths. joinn-visual and joinn-gpu do no IO.
- **`cargo xtask layers`** reads every workspace member's `Cargo.toml` (the `[dependencies]` table, textually: one `name = …` per line) and checks it against §2.1's table:
  - no crate above the boundary reaches `joinn-gpu` or `joinn-shell-desktop` through workspace edges;
  - `wgpu` appears only in joinn-gpu and joinn-shell-desktop;
  - `winit` appears only in joinn-shell-desktop;
  - every crate except those two has `[lints] workspace = true`.

  It runs its fixture first: `xtask/layers_fixtures/illegal/`, a fake `joinn-visual/Cargo.toml` depending on `joinn-gpu`. It must print `layers fixture: joinn-visual -> joinn-gpu refused (ok)`. It ends `layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)`. CI gains a `layers` step after `modules`.
- **Words.** JoInn calls wgpu's Vulkan/DX12/Metal choices **graphics APIs**. `vocab` bans the lowercase word for them in `crates/` (it already does). wgpu's own identifiers are fine.

### 2.10 The desktop shell (joinn-shell-desktop, binary `joinn-desktop`)

- **Run.** `cargo run -p joinn-shell-desktop -- <path to a .body>` loads the body and its cells from the corpus the way `joinn-cli`'s `load` does (its own leaf, not a dependency on joinn-cli). It picks the first adapter from `adapters()` that is not a CPU device, or the first adapter otherwise. It prints `adapter: <line as in cargo xtask adapters>`.
- **Render on demand (V12).** `ControlFlow::Wait`. A redraw is requested only by a resize, a click, a key that ran the body, or a lost surface. Each redraw prints `tick <n>: rows <r>, bytes <b>`. Moving the mouse prints nothing.
- **Click.**
  - The cursor position (f64 physical pixels) is floored to an integer pixel.
  - The CPU pick prints immediately: `pick <x>,<y>: <owner> (cpu)`.
  - The shell then reads that texel from the ID target on the next tick and prints `        <owner> (gpu) · agree`.
  - In a debug build, a disagreement prints `TRUTH VIOLATION at <x>,<y>: cpu <owner>, gpu <owner>` (V14).
  - Clicking an edge pixel prints `pick <x>,<y>: edge (cpu)` and does no GPU confirmation.
- **Typing.** Clicking an in-port that is in the body's intent set selects it: `selected body.cli_a@0`. Typed characters echo as `  typing: 2`. Enter makes a text term and calls `Host::intend`, then `check_intent`, then inject, then run. The shell prints `intent cli_a@0 "2"`, then `fired <instances>` or `refused: <reason from probe>`. Then `apply_run` feeds the delta to the renderer. Escape clears the selection. Clicking an in-port that isn't in the intent set prints `not an intent address: <owner>`.
- **The shell is a host.** It implements `Host`:
  - `present` → `Scene::present`;
  - `intend` → typed text on the selected address, checked by `check_intent`;
  - `signals()` → empty, like the test host.
- **Loss (V2, R9).**
  - `SurfaceError::Lost`/`Outdated`: reconfigure the surface.
  - A lost device: drop the renderer, open the adapter again, and `upload_all` from the scene's tables. It prints `regrow: device lost, tables re-uploaded`.
- **No window in CI.** The shell's non-window logic (key → intent, click → owner string) lives in leaves that take plain values and are unit-tested. CI builds the binary but never opens a window.

### 2.11 Gate 6

Three items, in `xtask/src/fns/gate_six_items.rs`, built like `gate_four_items.rs`. `"phase 6"` goes into `PHASE_LABELS` after `"phase 5.2"`, and `gates.lock` gains `phase 6: 3/3` at the end, written by the lock writer.

**Every GPU part of every check runs on every adapter `adapters()` returns.** An empty list fails the item with the adapter refusal line. Controls render on the first adapter in `adapters()` order.

| # | Item | `control_artifact` | `opposes` | Control answers `true` when |
|---|---|---|---|---|
| 1 | **Every pixel has one owner, and both pickers name it** | `corpus/phase2/calculator.body` | `DropWire("cli_b@1", "sum@1")` | the rendered ID image (1280×720) holds exactly one distinct wire owner |
| 2 | **What the engine computes is a row the picture shows** | `corpus/phase2/calculator.body` | `DropGenome("sum")` | after the script (§2.12), no live port row `sum@2` has `filled` set |
| 3 | **A click names an address; a stale click is refused** | `corpus/phase2/calculator.body` | `DropGenome("cli_b")` | no pixel of the rendered ID image (1280×720) resolves to instance `cli_b` or one of its ports |

**Checks:**

1. On every adapter, at every standard viewport:
   - Every non-edge pixel's GPU ID resolves to the same owner as `cpu_pick`.
   - `cpu_pick` equals `cpu_pick_reference` everywhere.
   - All **13** owners (1 membrane, 3 cells, 7 ports, 2 wires) own at least one non-edge pixel.
2. The script (§2.12), run through `apply_run`:
   - After every event, the delta-built tables equal `regrow`'s, byte for byte.
   - Every delta stays within the V121 bound.
   - After the script, `sum@2` is filled and no cell is refused.
   - A camera change produces an empty delta, and `take_pending()` is then `None` (V12).
   - On every adapter, the final scene rendered, then the renderer and device dropped and reopened and `upload_all` from `regrow`, gives identical color bytes and identical ID bytes (V2).
3. On every adapter, at 1280×720, the probe pixels (§2.13) print exactly the owners in §2.13. Then:
   - `replace` to the `DropGenome(cli_b)` form writes 5 rows (§2.7);
   - resolving the old `cli_b` ID is refused as a stale pick naming generation 1 and now 2;
   - resolving the old `cli_a` ID still prints `body.cli_a`.

Rule 41 holds: no two non-legacy items across gates 4, 5, 5.1, 5.2 and 6 share `(control_artifact, opposes)`. The uniqueness check extends to gate 6.

### 2.12 The script

The calculator transcript's inputs, as `RawEvent`s on `calculator.body`, one run per event:

1. `cli_a@0` ← `"two"` (refused at the membrane: `cli_a` is refused afterwards)
2. `cli_a@0` ← `"2"`
3. `cli_b@0` ← `"3"` (`sum` fires; `sum@2` holds 5)

Each event goes through `check_intent`, inject, `run`, then `apply_run` (or the refusal presentation), exactly as §2.7 says. Epochs count from 0. This is the same script as gate 3's hosts item. The shell drives the same path when AJ types.

### 2.13 Probe pixels at 1280×720 (`k 28`, origin `80 24`)

A layout point `(u, v)` probes pixel `(80 + 28u, 24 + 28v)`. **Claude's prediction** (gate 6 item 3 asserts every line):

| Layout point | Pixel | Owner printed |
|---|---|---|
| (2, 2) | 136, 80 | `body membrane` |
| (10, 7) | 360, 220 | `body.cli_a` |
| (10, 17) | 360, 500 | `body.cli_b` |
| (30, 9) | 920, 276 | `body.sum` |
| (4, 7) | 192, 220 | `body.cli_a@0` |
| (16, 7) | 528, 220 | `body.cli_a@1` |
| (4, 17) | 192, 500 | `body.cli_b@0` |
| (16, 17) | 528, 500 | `body.cli_b@1` |
| (24, 7) | 752, 220 | `body.sum@0` |
| (24, 11) | 752, 332 | `body.sum@1` |
| (36, 7) | 1088, 220 | `body.sum@2` |
| (20, 7) | 640, 220 | `body wire cli_a@1 -> sum@0` |
| (20, 14) | 640, 416 | `body wire cli_b@1 -> sum@1` |
| — | 0, 0 | `background` |

Checked by hand. The pixel centre (640.5, 416.5) is 0.7 px from the diagonal wire's centre line, inside its 7 px half-width. The membrane probe is 38.9 px from the membrane's corner-circle centre (radius 84), so it is inside.

---

## 3. What xtask prints

| Command | Prints | Exit 1 when |
|---|---|---|
| `cargo xtask layers` | the fixture line, then `layers: ok (…)` | any edge outside §2.1, or the fixture isn't refused |
| `cargo xtask layout <path>` | `print_layout`, then the four camera lines (§2.4) | the layout is refused |
| `cargo xtask layout --all` | one block per corpus `.body` that binds; `<rel>: not measured: <reason>` for the rest | never on a refusal (it reports); only on its own failure |
| `cargo xtask adapters` | one line per adapter (§2.8), then `adapters: <n>` | the list is empty |
| `cargo xtask pick` | per adapter: per standard viewport for the calculator, `calculator <w>x<h>: agree <a>, edge <e>, disagree 0, owners 13/13`; then per corpus body that binds (at 1280×720), `<rel>: agree <a>, edge <e>, disagree 0`; then the planted line; last line `pick: <n> adapter(s), <m> subject(s) agree; planted disagreement: refused as truth violation (ok)` | any disagreement, any unowned owner, or the plant not refused |
| `cargo xtask regrow` | per script event: `event <n> <address> <term>: touched <instances>, rows <r> (bound <b>), bytes <y>, tables equal regrow`; then `camera change: rows 0`; then `idle tick: nothing to draw`; per adapter: `regrow <adapter>: color identical, ids identical`; then the planted line; last line `regrow: <n> adapter(s); planted difference: refused (ok)` | any inequality, bound broken, or plant not refused |

**The plants** (rule 19):

- **pick**: `cpu_pick` run on a copy of the calculator's tables with port `sum@1`'s centre moved by one unit. It must disagree with the GPU on at least one pixel, and the command prints the count.
- **regrow**: a regrown scene with port `cli_a@0`'s `filled` bit flipped. It must be refused as unequal.

Frame times are printed by `pick` as information only (`frame median <ms>`). No check reads them (rule 4).

---

## 4. The Commits

Done-when is a command, and the command must be able to fail.

### Chunk A: the ground, no GPU

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P6-01** | **Plan, rules, S2** | Commit this plan. Update `AGENTS.md` per Appendix A. Commit the S2 files already on disk, unchanged: `docs/Findings/spikes/s2-gpu.md`, `joinn/spikes/s2-gpu/Cargo.toml`, `Cargo.lock`, `.gitignore`, `src/main.rs`, `RESULTS.md`. `dependencies.md`: the four rows of §2.2. `decisions.md`: rows R70–R73 `open` (§8). Backlog: R70–R73 with §8's text; R64 and R69 unchanged | `git show --stat HEAD` lists the plan, `AGENTS.md`, the six S2 files, `dependencies.md`, `decisions.md` and the backlog, and nothing under `joinn/crates` |
| **P6-02** | **CI** | `ci.yml`: matrix `[ubuntu-24.04, windows-latest]`; `actions/checkout@v5`; on Linux only, a step before `fmt`: `sudo apt-get update && sudo apt-get install -y mesa-vulkan-drivers` | Push, then read CI as in §0.2 item 4. Both jobs `success`. Paste the read |
| **P6-03** | **Three crates, the fence, `layers`** | Create `joinn-visual`, `joinn-gpu`, `joinn-shell-desktop` (facade `lib.rs`; `main.rs` for the shell holding `main` only, which prints `joinn-desktop: not built yet` until P6-12); workspace members; dependencies per §2.1–§2.2 (wgpu, winit, pollster, bytemuck declared now); the two own-`[lints]` tables; the vocab float and IO changes and their fixture; `cargo xtask layers` with its fixture; CI step `layers` after `modules` | `cargo xtask layers` prints the fixture line and `layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)`. `cargo xtask modules` → `modules: ok (enforced 14 crate(s))`. `cargo xtask vocab` → `vocab: ok`. Shown then reverted: add `joinn-gpu` to joinn-visual's dependencies; paste the refusal line |
| **P6-04** | **`instance_ports`** | §2.1's function in joinn-link; `membrane` rewritten to use it | `cargo test --workspace` passes with no test changed. `cargo xtask gate all` exits 0 with every phase line unchanged. `cargo xtask assay agree` unchanged. Paste the three last lines |
| **P6-05** | **Layout** | `layout`, `print_layout`, `fit` (§2.3–§2.4) in joinn-visual; `cargo xtask layout <path>` and `--all`. The invariance test (V119): for every corpus body that binds, `print_layout` is identical under the neutral edit and under stripping every allele from its cells | `cargo xtask layout phase2/calculator.body` prints §2.3's block and §2.4's four camera lines byte for byte (a test asserts it). `cargo xtask layout --all`: paste the whole output. A body with a wire cycle (built in a test) is refused naming its instances |
| **P6-06** | **Exact pick** | Shape classification (§2.5), `cpu_pick_reference`, grid `cpu_pick`, owner resolution and printing | A test: at each standard viewport, `cpu_pick` equals `cpu_pick_reference` on every pixel of the calculator, and the edge-pixel count is printed. A test asserts §2.13's table on the CPU alone. Unit tests for each shape classifier include one pixel exactly on an edge (edge), one 1/16 px inside a straight edge (edge), and one 2/16 px inside (inside) |
| **P6-07** | **Tables, deltas, regrow, replace** | §2.6–§2.7 in joinn-visual: `Tables`, `table_bytes`, generational slots, `Scene` with every method in §2.7 | Tests: the script (§2.12) with tables equal to `regrow` after every event and each delta within its bound (print the rows per event); `sum@2` filled at the end; camera change empty; `take_pending` `None` when idle; `replace` to `DropGenome(cli_b)` writes 5 rows; the stale `cli_b` ID refused naming generations 1 and 2; after `replace`, every pixel's owner at every standard viewport equals a fresh `grow` of the new body |
| — | **Stop A** | `phase-6-stop-a.md` (§0.2), push, stop | — |

Dependencies: in order. P6-05 needs P6-03 and P6-04. P6-06 and P6-07 need P6-05.

### Chunk B: draw and pick

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P6-08** | **The GPU** | joinn-gpu per §2.8: `adapters`, `open`, `Renderer` (`new`, `upload_all`, `apply`, `draw`), the two organelles in WGSL, the ID target, `render_offscreen`, `read_texel`; `cargo xtask adapters` | `cargo xtask adapters` lists at least one adapter (paste the whole output). A joinn-gpu test renders the calculator at 640×360 on every adapter and asserts the ID at each §2.13 probe (scaled to that viewport) decodes to the expected owner. The `min_binding_size` test passes. CI green on both OSes (paste the read) |
| **P6-09** | **Both pickers name it** | `cargo xtask pick` (§3), with the plant; CI step `pick` after `assay agree` | Whole output pasted. Every line `disagree 0`, calculator lines `owners 13/13`, last line as in §3 |
| **P6-10** | **Regrow** | `cargo xtask regrow` (§3), with the plant; CI step `regrow` after `pick` | Whole output pasted. Every event line ends `tables equal regrow`; `camera change: rows 0`; `idle tick: nothing to draw`; every adapter `color identical, ids identical`; last line as in §3 |
| **P6-11** | **Findings** | `docs/Findings/phase-6-pick.md`: the commands, their full output from P6-09 and P6-10, the adapters, and a *Predictions* section marking each of §2.3, §2.4, §2.7's 5 rows and §2.13 `as predicted` or `differs` with both values | Every prediction is marked. The frame-time lines are quoted and labelled *information, not a check* |
| — | **Stop B** | `phase-6-stop-b.md`, push, stop | — |

Dependencies: P6-08 first. P6-09 and P6-10 need P6-08. P6-11 needs both.

### Chunk C: the window, gate 6, freeze

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P6-12** | **The shell** | joinn-shell-desktop per §2.10, with unit tests for the non-window leaves | `cargo build -p joinn-shell-desktop` succeeds on both OSes (CI). Unit tests: a click at each §2.13 probe maps to its owner string; typing `2` then Enter on the selected `cli_a@0` yields `intent cli_a@0 "2"`; an in-port outside the intent set yields `not an intent address: …`. Cursor also runs the window once on the Windows machine, clicks the `sum` cell, and pastes the two printed pick lines (only if a display is available; otherwise writes `no display`) |
| **P6-13** | **Gate 6** | §2.11's three items; uniqueness across gates 4–6; `phase 6` label; lock row | `cargo xtask gate all` exits 0 and prints `phase 6: 3/3` after `phase 5.2: 3/3`. Shown then reverted: give item 3 item 2's `opposes`; paste the uniqueness refusal. Shown then reverted: make `adapters()` return an empty list; paste item 1's printed failure (it must fail, not skip) |
| **P6-14** | **Docs and freeze** | README and `Guides/03-where-we-are.md` (JoInn draws one body; what the picture means; what it doesn't draw yet); `Guides/05-glossary.md` gains *layout unit, camera, tick, organelle, ID target, owner, edge pixel, delta, regrow*; `decisions.md` rows VH1, VH2, VH5, VH11, VH12, VH14 (Part II's V1, V2, V5, V11, V12, V14) and V119–V124 `holds`; the roadmap's Phase 6 section gains a dated note pointing at `phase-6-pick.md` and naming what Phase 6 deferred (glyph, image and snapshot organelles → Phase 7; fallback path → R72) | `cargo xtask gate all` from a fresh clone prints the fixtures, then phases 0, 1, 2, 2.1, 2.2, 3 (legacy), 4, 5, 5.1, 5.2 and 6, and exits 0. `corpus verify` 43. `git diff --stat <P6-01>..HEAD -- joinn/corpus` prints nothing |
| — | **Stop C** | `phase-6-stop-c.md` with the CI read, push, stop | — |

If something has to be cut for time, cut `replace` and gate 6 item 3's stale-pick half together (the probe table stays), and carry them forward with a line in `decisions.md`. Never cut P6-06, P6-07's regrow equality, P6-09 or P6-10.

---

## 5. Test Strategy

| # | Invariant | Commit |
|---|---|---|
| **VH1** | No crate above the boundary can name a wgpu or winit type (`layers`) | P6-03 |
| **V119** | A layout is blind to the regulatory region and to alleles | P6-05 |
| **V120** | The grid picker and the brute-force picker agree on every pixel | P6-06 |
| **V121** | One run writes at most Σ(1 + ports) rows over the instances it touched; a camera change writes none | P6-07 |
| **V122** | Tables built by deltas equal tables regrown from DNA and live state, byte for byte | P6-07 |
| **VH2** | GPU state discarded and regrown draws identical color and ID bytes | P6-10 |
| **VH5 · VH14** | Every non-edge pixel has one owner, and the GPU and CPU pickers name the same one, on every adapter | P6-09 |
| **VH11** | The device is opened at WebGPU core limits and §7.2's layout fits | P6-08 |
| **VH12** | An idle scene has nothing to draw; the shell redraws only on change | P6-07, P6-12 |
| **V123** | A stale pick is refused naming both generations | P6-07 |
| **V124** | Floats exist only in joinn-gpu and joinn-shell-desktop | P6-03 |

Carried forward and re-run on every commit: every earlier invariant. No corpus file changes in this phase.

**A test must never** assert only that a picture rendered or that two things differ. It asserts the owner, the count, or the bytes.

---

## 6. Exit Gate 6

`cargo xtask gate 6`, from a fresh clone, after the harness fixtures pass. It needs at least one GPU adapter; lavapipe or WARP counts.

- [ ] **1 · Every pixel has one owner, and both pickers name it.** *Opposes* `DropWire(cli_b@1, sum@1)`.
- [ ] **2 · What the engine computes is a row the picture shows.** *Opposes* `DropGenome(sum)`.
- [ ] **3 · A click names an address; a stale click is refused.** *Opposes* `DropGenome(cli_b)`.

### 6.1 Conditions for opening Phase 7

1. Gate 6 passes; `gate all` exits 0; CI is green on Windows and Linux (read, not assumed).
2. `phase-6-pick.md` exists with every prediction marked.
3. AJ has run the window (§6.2) and said it worked, or each thing that looked wrong is a snag Claude has settled.
4. Claude's stop-C review lists no open snag, or AJ has chosen in conversation to carry each one forward.

### 6.2 AJ's window check (Stop C, about two minutes)

In PowerShell:

```
cd D:\JoInn\joinn
cargo run -p joinn-shell-desktop -- corpus/phase2/calculator.body
```

1. A window opens: a dark membrane with two cells on the left, one on the right, and two wires.
2. Click the middle of the right-hand cell. The terminal prints a `pick …: body.sum (cpu)` line, then a `body.sum (gpu) · agree` line.
3. Move the mouse around without clicking. No new `tick` lines appear.
4. Click the top-left cell's **left** port. The terminal prints `selected body.cli_a@0`. Type `two` and press Enter. The top-left cell turns red.
5. Type `2` and press Enter. The red goes away and that cell's right port fills in.
6. Click the bottom-left cell's left port, type `3`, and press Enter. The right-hand cell's right port fills in.
7. Resize the window. The picture re-centres, and the tick line says `rows 0`.
8. Close the window.

Tell Claude "window worked", or which step looked wrong.

---

## 7. Risks

| Risk | What to do |
|---|---|
| **The delta protocol degenerates into a rebuild** (roadmap risk #7, the phase's adversary) | V121 bounds it and V122 tests it every event. A bound that breaks is a finding, not a tuning job. It goes in `phase-6-pick.md` with the counts |
| The GPU and the exact picker disagree off the edge band | That is a truth violation and the phase stops there. Report the pixel, both owners, and the adapter. Never widen the band to make it pass |
| lavapipe or WARP disagrees where a hardware GPU doesn't | Same rule: it is a disagreement. Software adapters are witnesses too |
| Edge pixels are a large share of an image | Report the share. S2 saw 0.2%. The band is 1/16 px and is not changed in this phase |
| `apply_run`'s "touched" set misses an instance whose description changed | V122 catches it (regrow shows the value, deltas don't). That is a snag with the event and the instance named |
| A body in the corpus has a wire cycle or a primitive `instance_ports` can't read | `layout --all` prints it `not measured` with the reason. It's recorded, not fixed |
| CI time grows with wgpu | Accepted. Never split or skip a GPU step to save time |
| winit can't open a window on the machine Cursor uses | P6-12 says `no display`; AJ's check at Stop C covers it |

---

## 8. Open Items

| ID | Topic | Question |
|---|---|---|
| **R70** | A real weak device | S2 and Phase 6 ran on an RTX 2080 and on software adapters. V31 asks for a real weak device in the test set. The first one AJ gets reruns `spikes/s2-gpu` and `cargo xtask pick` unchanged |
| **R71** | Authored layout | Layout is computed from the coding region. When a creator places a cell by hand, where does that live? A regulatory `layout` section (new grammar, so R60 applies), or the creator's own store? |
| **R72** | The vertex-buffer fallback | `joinn-gpu` refuses an adapter without vertex-stage storage. S2 showed the fallback draws the same IDs, but it rewrites every instance row when a chart moves. When a compatibility-only target matters, is the fallback worth that cost? |
| **R73** | Text on screen | The glyph organelle, a borrowed shaping crate, and text's inverse (point → character index). Planned with Phase 7's bands |
| R64 | Legacy gates | Carried forward (AJ, 28 Sep) |
| R69 | Describe at firing time | Carried forward (AJ, 28 Sep). Phase 6 describes after the run, the same as the test host |

---

## Appendix A · `AGENTS.md` changes

Replace the header paragraph with:

```markdown
# JoInn — standing rules

This repo is JoInn. Phase 6 is Visual Host I: JoInn draws one body, and every
pixel knows who owns it. Its one idea is THE PICTURE IS THE TABLES, AND TWO
PICKERS AGREE. The build plan is docs/Plans/JoInn Phase 6 Implementation
Plan.md. Work one CHUNK at a time (A, B or C), one git commit per numbered step,
and stop at the chunk's stop report. Every decision is in the plan. Never stop to
ask; follow the plan's Snags section instead.

There is no text on screen, no zoom, no second body in a picture, no
accessibility tree and no compiler in this phase. Phase 7 runs after it.
```

Rule 3 becomes:

```markdown
3. No `f32`/`f64` above the renderer boundary: only crates/joinn-gpu and
   crates/joinn-shell-desktop may hold them (rule 54). No `HashMap`/`HashSet` —
   `BTreeMap`/`BTreeSet` only.
```

In rule 15, add: `joinn-shell-desktop is a host crate. joinn-visual and joinn-gpu do none.`

Keep every other rule exactly as it is. Append:

```markdown
54. FLOATS LIVE BELOW THE RENDERER BOUNDARY. f32 and f64 appear only in
    joinn-gpu and joinn-shell-desktop, and only those two crates have their own
    [lints] table (the workspace table minus float_arithmetic). joinn-visual is
    exact: integer layout units, integer picking.
55. NO CELL NAMES THE GPU (V1). `cargo xtask layers` enforces the crate table of
    the plan's §2.1. wgpu appears only in joinn-gpu and joinn-shell-desktop;
    winit only in joinn-shell-desktop. JoInn calls Vulkan, DX12 and Metal
    "graphics APIs".
56. THE PICTURE IS THE TABLES. Every pixel is drawn from joinn-visual's tables
    through the vertex shader. Nothing is drawn from a list rebuilt per tick.
    Tables hold layout units; a camera change writes the tick uniform and no row.
57. EVERY PIXEL HAS ONE OWNER, AND TWO PICKERS NAME IT. The exact CPU pick is the
    reference. The GPU ID target must agree on every non-edge pixel. An edge
    pixel (within 1/16 px) is counted, never judged. A disagreement is a truth
    violation, never a test failure, and the band is never widened.
58. REGROW IS THE TEST OF THE DELTA. Tables built by deltas equal tables regrown
    from DNA and live state, byte for byte, after every run. GPU state dropped
    and regrown draws identical color and ID bytes.
59. AN IDLE UNIVERSE DRAWS NOTHING (V12). No tick runs without a resize, an
    input, or a pending delta.
60. A GPU CHECK RUNS ON EVERY ADAPTER IT FINDS, AND FAILS WHEN IT FINDS NONE. It
    never skips. Software adapters (lavapipe, WARP) are witnesses like any other.
```

---

*JoInn Phase 6 Implementation Plan, Draft 0.1 (28 Sep 2026). Opens after Phase 4's stop-C review and the S2 run on AJ's desktop. AJ decided: floats only below the renderer boundary; software adapters in CI; R64 and R69 carried forward. Claude decided, open to AJ's veto before chunk A: no text until Phase 7; computed layout; one body per picture; exact integer picking as the reference; no fallback path. Every other decision is made here. Cursor executes. Claude verifies at each stop. AJ runs the window at Stop C.*
