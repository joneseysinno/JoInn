# JoInn Phase 7.2 Implementation Plan

**Visual Host II, part 1: the zoom · exact charts · bands · the cut · JoInn's own lettering**

Author: AJ, with Claude · Draft 0.1 · October 3, 2026

> **Every decision in this plan is final.** Nothing here waits on AJ. Cursor never stops to ask AJ anything, and no file in this phase is typed by AJ. If a step can't be done the way the plan says, Cursor follows §0.3 (Snags) and keeps going.

---

## For AJ: this plan in plain English

**What Phase 7.2 is.** Until now JoInn could draw one body, at one fixed size. Phase 7.2 makes the fractal work: a whole universe on the screen, and you zoom from all of it down to a single letter inside a cell, smoothly, with no jitter. As you zoom, things change how they're drawn: far away a body is a dot, then a block, then you see its cells, then its ports, wires and words. Zoom far enough out and a whole system folds into one tile.

**What you decided (3 Oct).**
- The beam's stage 2 waits. Bodies like the beams get built *inside* JoInn once it can be used. Roadmap order stays: 7.2, then 8, then the creator (9).
- Visual Host II is split. **7.2 is the zoom.** **7.3 is the links**: hyperedges drawn as regions, hubs, bundles and spines, rerouted at collapsed systems, plus snapshots.
- **Exact all the way.** No float decides anything. Floats appear only at the very last step, when a position is handed to the GPU.
- **JoInn's own stroke font.** Letters and digits are a few straight strokes each, sharp at any zoom, exact to click.
- **A generated universe.** 8 galaxies × 16 systems × 24 bodies = 3,072 bodies, grown from the bodies already in the corpus with a fixed seed.

**How the zoom stays exact.** Zoom is a whole number of doublings plus a fine step, 1/256 of the way to the next doubling. One wheel notch is 32 fine steps, so 8 notches double the size. The camera pins one exact point of the universe to one whole pixel of the screen. Zooming in and back out at the same spot returns *exactly* the same picture, byte for byte. When you zoom deep into one body, the camera re-anchors to that body's own chart, so the numbers stay small. A re-anchor moves nothing on screen, and that is tested byte for byte.

**What the zoom does to things** (by size on screen):

| Band | Size on screen | What you see |
|---|---|---|
| Dot | under 4 px | a small dot |
| Glyph | 4 to 32 px | the body's outline, filled |
| Summary | 32 to 240 px | its cells and the ports on its surface |
| Full | over 240 px | every port, the wires, cell names and values |

A system or galaxy smaller than 32 px folds into one tile with its name. Moving between bands fades, and the fade is driven by the zoom itself, not by a clock. So an idle screen still draws nothing, and every run replays exactly.

**What Claude chose while writing this. Say if you disagree before Cursor starts:**
- **The cut is decided in two places, from the same integers.** The roadmap had the CPU decide each frame what to draw. Here the GPU decides it per shape, and the CPU decides it only when it's needed (a click, a printout). Both use the same whole-number rule, and the gate checks they agree: two cuts, one truth. This may simply dissolve the phase's adversary (a CPU cost proportional to the universe every frame). Cursor measures it.
- **Crossfade instead of hysteresis.** The roadmap wanted separate thresholds for zooming in and out, so the picture wouldn't flicker. A fade driven by size is a smooth function of the zoom, so it can't flicker, and zooming out retraces zooming in exactly (Law 1). For clicks, a fading body belongs to its larger band once it is past the middle of the fade.
- **Snapshots move to 7.3.** They exist to save drawing cost. 7.2 measures the cost first, so 7.3 builds snapshots from numbers instead of guesses.
- **Hyperedges are counted in 7.2, not drawn.** A straight line between bodies would cross other bodies' membranes, which the touch-only law forbids. Drawing them properly means routing, which is 7.3. 7.2 computes, for every link, which tiles it touches when systems fold, and checks each tile is touched once.
- **Values show beside out-ports only.** A cell's name sits small at its top-left. A filled out-port's value sits beside it, up to 4 characters. In-port values show where they came from.

**What you do at the end (Stop C), about four minutes.** Open the generated universe, zoom into one body until the "s" of "sum" fills the window, zoom back out, resize the window, and leave it idle. Steps are in §6.2.

**Your prompts to Cursor** (copy exactly, one per chunk):

- Chunk A: `Do chunk A of docs/Plans/JoInn Phase 7.2 Implementation Plan.md. Follow AGENTS.md.`
- Chunk B: `Do chunk B of docs/Plans/JoInn Phase 7.2 Implementation Plan.md. Follow AGENTS.md.`
- Chunk C: `Do chunk C of docs/Plans/JoInn Phase 7.2 Implementation Plan.md. Follow AGENTS.md.`

If Cursor runs out of room partway: `Continue chunk A of docs/Plans/JoInn Phase 7.2 Implementation Plan.md from the first commit not in git log. Follow AGENTS.md.` (Use the right letter.)

After each stop, tell Claude "Cursor finished chunk A" (or B, C).

---

## 0. How Cursor Works This Plan

| | |
|---|---|
| **Plan file** | `D:\JoInn\docs\Plans\JoInn Phase 7.2 Implementation Plan.md` |
| **Code** | `D:\JoInn\joinn\`. No new crate. No new external dependency in the workspace |
| **Standing rules** | `AGENTS.md` as updated by P72-01 (Appendix A) |
| **Unit of work** | a **chunk** (A, B or C). Inside a chunk, do the commits in the order listed. Each commit is its own git commit, and its message starts with its id (`P72-05: …`) |
| **End of a chunk** | write the stop report (§0.2), commit it, `git push`, and stop. Do not start the next chunk |
| **Who decides** | every decision is in §2. Cursor decides only module layout, function bodies, private type representation, glyph stroke shapes inside §2.8's grid, table indexing where §2.9 says so, and error-string wording where §2 gives none |
| **Who checks** | Claude, at each stop, from a fresh clone on Linux. GitHub CI runs on Windows and Linux on every push. AJ runs the window at Stop C |
| **Input** | **Never move the system mouse, click, or type outside a window Cursor itself started** (rule 67). A window check posts messages to that window, or writes `no display` |

### 0.1 The commit report

Every commit ends with this block, in the commit message body **and** in the chunk's stop report. Every value is copied from the terminal, never summarised.

```
Commit:     P72-NN (hash in git log)
Done-when:  <the command> → <the line it printed>          MET | NOT MET
Suite:      cargo test --workspace --no-fail-fast → <N passed, M failed>
Scans:      vocab → <last line> · modules → <last line> · layers → <last line>
Snags:      none | <each thing that went differently from the plan, with the printed line>
```

From P72-05 on, add one line: `Zoom:       cargo xtask zoom → <last line>`.

### 0.2 The stop report

At the end of each chunk Cursor writes `docs/Findings/phase-7.2-stop-<letter>.md` with:

1. `git rev-parse HEAD` (full hash).
2. Every commit report block from the chunk, in order, with real hashes.
3. The last line of each of: `cargo test --workspace --no-fail-fast`, `cargo xtask gate all`, `cargo xtask corpus verify`, `cargo xtask vocab`, `cargo xtask modules`, `cargo xtask layers`, `cargo xtask floor`, `cargo xtask forces`, `cargo xtask contact`, `cargo xtask pick`, `cargo xtask regrow`, `cargo test --manifest-path spikes/s8-beam/Cargo.toml`; from P72-03 on `cargo xtask grove`; from P72-05 on `cargo xtask zoom`. Also every line of `gate all` that contains `fail`, and every failing test name.
4. **CI**: for the newest run after the push, read with PowerShell `Invoke-RestMethod "https://api.github.com/repos/joneseysinno/JoInn/actions/runs?branch=main&per_page=3"` and that run's `jobs_url`. Paste `head_sha`, `status`, `conclusion`, and each job's `name` and `conclusion`. If the run hasn't finished, wait and read again. A failed job is a snag, not a reason to change CI.
5. **Snags**: every snag from the chunk in one list, each tagged with its commit id.

Then commit (`P72-stop-<letter>: stop report`), push, and stop.

### 0.3 Snags

A snag is any point where a done-when can't be met as written, or a prediction in this plan turns out wrong.

- **Never** fake it, work around a scan, weaken a test, rebless a golden, or change the plan's meaning to make it pass.
- **A prediction that differs is not a failure.** Paste what the machine printed next to what the plan predicted, and continue. Claude settles it at the stop.
- Leave that piece in its honest state and **continue with the next commit**.
- If a later commit depends on the snagged one (§4), skip it and write `skipped: depends on P72-NN`.

---

## 1. Scope Fence

### In scope

- **The camera** as an exact value: zoom level and step, focus, pan, zoom about a pixel, frame-to-fit, refusals (§2.2).
- **The chart chain** and the **rebase** (§2.3).
- **The grove**: a generated universe of 3,072 bodies (§2.4), and **universe layout** (§2.5).
- **Bands**, the **crossfade**, the **owner rule** (§2.6), and **the cut** with **lens nodes** and **touch counts** (§2.7).
- **The stroke font**: cell labels, out-port values, system and galaxy titles (§2.8).
- **Tables and tick uniform** for all of it (§2.9), **exact picking at any zoom** (§2.10).
- **The shell** opens a `.universe`: wheel, drag, keys, smooth resize, render on demand (§2.11).
- **Gate 7.2** and the freeze; findings `docs/Findings/phase-7.2-zoom.md`.

### Out of scope (Cursor refuses these even when they look small)

| Not now | Why it is tempting |
|---|---|
| Drawing a hyperedge (region, hub, bundle, spine), routing around membranes | Phase 7.3. A straight line would cross membranes (touch-only law) |
| Snapshots, atlas pages, a re-render budget | Phase 7.3, sized from this phase's measurements |
| A GPU compute pass for the cut | Only if 7.2's measurement says so; never in 7.2 |
| A real font, a font file, a font crate, MSDF | AJ chose the stroke font. A real font is presentation for later |
| Running the grove (injecting values, delivering links) | Not needed to zoom. Values are shown on the calculator scene in the shell |
| Clicking a system to collapse or expand it | Zoom collapses. Collapse by intent is the creator's (Phase 9) |
| Accessibility tree, screen reader | Phase 8 |
| Any change to `.cell`, `.body`, `.contact` or `.universe` grammar | No new grammar in this phase |
| Writing the grove into `corpus/` | The grove is generated, never stored. The corpus does not change |
| Changing any corpus file, hash, golden, `.desc`, `grandfather.txt`, or an earlier `gates.lock` row | Never |
| The beam's stage 2 (tags in grammar, ℚ's turn, multiply) | AJ, 3 Oct: built in JoInn itself after the UX phases |
| Rewriting history: Phase 7 and 7.1 plans, reviews and findings keep their words | They record what was true when written |

---

## 2. Decisions

All **DECIDED**.

### 2.1 Where things live

| Crate | Gains |
|---|---|
| `joinn-visual` (exact, no float, no GPU) | `Zoom`, the new `Camera` fields and its operations (§2.2); charts and rebase (§2.3); `layout_universe` (§2.5); bands and the owner rule (§2.6); the cut, lens nodes, touches (§2.7); the stroke font and text layout (§2.8); the new tables (§2.9); exact pick at any zoom (§2.10); the universe scene |
| `joinn-gpu` | the new tick uniform, the shader's integer band and cut decisions, the new pipelines for frames and strokes (§2.9) |
| `joinn-shell-desktop` | opening a `.universe`; wheel, drag, keys, resize (§2.11) |
| `xtask` | `grove`, `zoom`, `layout --universe`, the extended `pick` and `regrow`, gate 7.2 |

No new workspace edge. `cargo xtask layers` passes unchanged. Rule 54 holds: `joinn-visual` has no float.

### 2.2 The camera (an exact value)

```rust
pub struct Zoom { pub level: i32, pub step: u32 }     // step in 0..256
pub struct Camera {
    pub zoom: Zoom,
    pub anchor: ChartId,          // §2.3
    pub focus: (i64, i64),        // a point of the anchor chart, in 2^-16 layout units
    pub pin: (i64, i64),          // the whole pixel the focus sits on
    pub width: u32, pub height: u32,
}
```

- **Pixels per layout unit:** `k = 2^level · (256 + step) / 256`, step 0 … 255. `level` runs from **−4 to 9** inclusive; anything else is refused: `zoom: level <n> is outside −4 … 9; acceptance is a level from −4 to 9`.
- **Where a point lands:** the pixel of anchor-chart point `p` is `pin + k·(p − focus)`. Every quantity is a dyadic rational, so every position is exact.
- **Zoom in one notch:** `step + 32`, carrying into `level` at 256. **Out:** the reverse. Eight notches double `k`. Zoom never changes `focus` or `pin`.
- **Zoom about a pixel `q`:** if `pin ≠ q`, first **refocus**: `focus ← floor₂₋₁₆(focus + (q − pin)/k)` (floor to a multiple of 2^-16 layout units), `pin ← q`. Then zoom. Refocus is the only rounding in the camera, it happens only when the pixel changes, and it moves the picture by less than `k·2^-16` px.
- **Pan by `(dx, dy)` whole pixels:** `pin ← pin + (dx, dy)`.
- **Frame a rectangle** (`frame`): the largest zoom with `k·w ≤ width − 32` and `k·h ≤ height − 32`; focus = the rectangle's centre; pin = `(width div 2, height div 2)`. If even level −4 step 0 doesn't fit, the camera is −4 step 0.
- **Resize:** width and height change; zoom, focus and pin do not. Nothing jumps (Phase 7's stepped resize is gone). Key `F` frames again.
- Phase 6's `fit` and its integer `k` stay exactly as they are, for gates 6 and 7 and the four standard viewports. A Phase 6 camera `(k, ox, oy)` converts exactly: every whole `k` from 1 to 511 is some `(level, step)`, with focus `0` and pin `(ox, oy)`. Gates 6 and 7 draw through the converted camera.

**Predicted:** `frame` of the grove (5504 × 1600) at 1920 × 1080 is **level −2, step 95, k 351/1024**: `k·5504 = 1886 5/8 ≤ 1888`, and step 96 gives `1892`, which does not fit.

### 2.3 Charts and the rebase

A **chart** is a coordinate system with an integer origin in its parent chart, in layout units. The chain, from the lens (view) down to the place graph (data):

```
universe chart ─ galaxy chart ─ system chart ─ body chart
```

Cells, ports, wires and strokes live in their body's chart. Frames (§2.7) live in their own chart.

- **The anchor** is the deepest chart whose rectangle contains the world point under the viewport's centre pixel `(width div 2, height div 2)`. Rectangles are half-open: `[x, x + w) × [y, y + h)`. When no chart below the root contains it, the root is the anchor: the universe chart, or in a single-body scene the body's chart, which is then its only chart.
- Each chart row holds its **origin relative to the anchor's origin** (§2.9). This is the only thing that changes when the anchor changes.
- **Rebase** to a new anchor `A′` whose origin is `d` from the old anchor's: `focus ← focus − d·2^16`, chart rows rewritten. `pin`, `zoom` and every pixel stay exactly the same (`k·d` is exact).
- The CPU checks for a rebase after every camera change. **A pan or a zoom writes the tick uniform and no row. A rebase writes chart rows only, at most one per chart.** (V142)

### 2.4 The grove (a generated universe)

`cargo xtask grove [--seed N] [--out PATH]`. Default seed **7**. With `--out`, it writes the canonical text, as bytes with `\n` line ends, to PATH (a path outside `corpus/` and `docs/`; refused otherwise). It always prints one line (§3).

**Bodies.** Three corpus bodies, bound by coding hash:

| Kind | File | Hash | Surface |
|---|---|---|---|
| calc | `phase2/calculator.body` | `b55fba1eff65942099f6daf84b8bc47d605be05f637d805d260d3fcfd8c3ebde` | 40 × 24 |
| units | `phase5/units.body` | `2d5fcc96689b26df93fa86ace7525b4820d1bd7f68b46e9ce75ee77d7e2c9fda` | 20 × 18 |
| bus | `phase5/bus.body` | `fa812abd9ab0b1b6a3aef72b3c8ca1e2641c6f307350a8cc86ebea56187b8123` | 20 × 14 |

**Shape.** Galaxies `g0` … `g7`; in each, systems `g<G>s00` … `g<G>s15`; in each, 24 bodies, slots 0 … 23. Body aliases `b0000` … `b3071` in (galaxy, system, slot) order. In every system, slots 0, 1, 2 are calc, slots 3, 4 are units, slot 5 is bus. Slots 6 … 23 take their kind from **SplitMix64** seeded with the seed, one draw per slot in (galaxy, system, slot) order: draw mod 3 → 0 calc, 1 units, 2 bus.

SplitMix64 (no other generator): `s ← s + 0x9E3779B97F4A7C15; z ← s; z ← (z ⊕ (z ≫ 30))·0xBF58476D1CE4E5B9; z ← (z ⊕ (z ≫ 27))·0x94D049BB133111EB; return z ⊕ (z ≫ 31)`, all wrapping in u64. With seed 7 the first three draws are `7191089600892374487`, `309689372594955804`, `16616101746815609346`, and the first six kinds are `calc calc calc calc units calc`.

**Predicted counts (seed 7):** calc **1181**, units **982**, bus **909**.

**Links** (all `order none`; no grants; every port in at most one link):

| Link | Tail | Heads |
|---|---|---|
| `sys_<system>`, one per system (128) | `sum@2` of the system's slot 0 | `listen@0` of every bus in the system |
| `gal_<galaxy>`, one per galaxy (8) | `sum@2` of slot 1 of the galaxy's `s00` | `scale@0` of slot 3 of each of its 16 systems |
| `uni` (1) | `sum@2` of slot 2 of `g0s00` | `scale@0` of slot 4 of `s00` in each of the 8 galaxies |

Members are listed tail first, then heads in alias order. **Predicted:** 137 links, **1182 members** (128 + 909 for `sys_`, 8 × 17 for `gal_`, 9 for `uni`).

**Lens.** One lens, `function`: `galaxy g<G> { system g<G>s<SS> { b… } }`, in order. No regulatory region.

The grove must **bind and be admitted** exactly as any universe (`bind`, `check_lenses`, the link checks). Its hash is printed, not predicted; a test asserts two runs with seed 7 print the same hash and text, and seed 8 a different hash.

### 2.5 Universe layout

`layout_universe(universe, store, lens) → Verdict<UniverseLayout>`. Every number is an integer in layout units. A lens the universe doesn't declare is refused: `layout: lens <name> not found; acceptance is a lens the universe declares`.

| Level | Placed in its parent chart at | Size |
|---|---|---|
| body | slot `i` of its system: `(8 + 48·(i mod 6), 16 + 32·(i div 6))` | its own surface (`layout` / `layout_contact`) |
| system | index `j` in its galaxy: `(16 + 320·(j mod 4), 32 + 168·(j div 4))` | 304 × 152 |
| galaxy | index `g`: `(64 + 1360·(g mod 4), 64 + 768·(g div 4))` | 1296 × 704 |
| universe | origin | 5504 × 1600 |

For a lens with other shapes (`universe.universe`, `adversary.universe`) the same rule applies with these sizes: a system holds up to 24 bodies (6 across), a galaxy up to 16 systems (4 across), the universe up to 8 galaxies (4 across). More than that is refused naming the container and the count. A body whose surface exceeds 40 × 24 is refused naming it (`layout: body <alias> is <w>×<h>; acceptance is a surface within 40×24`).

`cargo xtask layout --universe <path or grove> [--lens NAME]` prints one line per chart: `galaxy g0 64 64 1296 704`, `system g0s00 16 32 304 152`, `body b0000 calc 8 16 40 24`, then `layout universe: <g> galaxies, <s> systems, <b> bodies, 5504x1600`. For the grove the last line is predicted: **`layout universe: 8 galaxies, 128 systems, 3072 bodies, 5504x1600`**.

### 2.6 Bands, the crossfade, and the owner rule

The **projected size** of a thing is `s = k · max(w, h)` in pixels, from its surface (body) or frame (system, galaxy). Thresholds `T = 4, 32, 240`.

| Band | Owner band when |
|---|---|
| Dot | `s < 4.4` |
| Glyph | `4.4 ≤ s < 35.2` |
| Summary | `35.2 ≤ s < 264` |
| Full | `264 ≤ s` |

The owner band is the higher one exactly when `10·s ≥ 11·T`: the middle of the fade window. Written as integers: with `k = 2^L·(256+t)/256`, the test is `10·(256+t)·size·2^L ≥ 11·T·256`, moved to whichever side keeps both sides whole. **No float decides a band** (V141). The CPU and the shader make the same integer test.

**Crossfade.** In the window `T ≤ s < 1.2·T`, the lower band fades out and the higher fades in, linearly in `s`. Opacity is colour only, so it may be float in the shader. The ID target always gets the owner band's ID. Outside a window, one band draws at full opacity.

**What each band draws** (for a body):

| Band | Drawn |
|---|---|
| Dot | a disc of radius 2 px at the surface's centre, the surface's style |
| Glyph | the surface |
| Summary | the surface, the cells, the surface ports (∂(body), rule 30) |
| Full | everything Phase 6 and 7 draw, plus §2.8's text |

A system or galaxy whose owner band is below Summary is a **lens node** (§2.7). Otherwise it is an **open frame**: its rectangle in the frame style, its title (§2.8), and its children.

### 2.7 The cut, lens nodes, and touches

**The cut** walks the lens: each galaxy, then if open each system, then if open each body. A thing is **visible** when its pixel rectangle meets the viewport: `x0 < width`, `x0 + k·w > 0`, and the same for y. Only visible things count.

The cut is decided **in two places from the same integers**: the shader per instance (each instance reads its ancestors' chart rows, §2.9, and discards itself when an ancestor is a lens node or it isn't visible), and the CPU (`cut(&scene, &camera)`, used by pick, by xtask and by the gate). The gate requires they agree: the set of IDs in the GPU image equals the set the CPU cut says can own a pixel (V145).

**A lens node** is the system's (or galaxy's) rectangle, drawn in the node style, with its name (§2.8) and nothing inside it. It owns its pixels: ID `[0, 0, SYSTEM_TAG | index, generation]` (galaxies `GALAXY_TAG`).

**Touches** (data, not drawn). For each link, its **touched nodes** are the distinct cut nodes its members map to: a member's body if the body is drawn, otherwise the lens node that contains it. `cargo xtask zoom` prints, at two views, the number of links and the total touches. **A link touches each cut node at most once** (V146), however many of its members are inside.

**Predicted touches:** at level −4 step 0 every system is a lens node: **264** (128 `sys_` links × 1, 8 `gal_` × 16, `uni` × 8). At `frame` every body is drawn: **1182** (= the member count).

### 2.8 The stroke font

Every glyph is a set of straight strokes on a grid **6 wide, 8 tall** (baseline at 8, descender to 10), each stroke from one grid point to another, drawn as a capsule. Advance is 8 grid units. Shapes are Cursor's (presentation), within these rules:

- The set: `0`–`9`, `a`–`z`, `_ . , - + = × ÷ ( ) …` and space. A character outside it is refused naming it: `text: '<c>' has no glyph; acceptance is a character from the stroke set`.
- Every stroke lies inside the 6 × 10 box. Two different characters never have the same stroke set. Tests assert both over the whole set.
- Round letters are polygons. No arc, no curve.

**Where text goes** (positions in sixteenths of a layout unit inside the body chart; `u` is the grid unit):

| Text | Grid unit `u` | Stroke half-width | Placed |
|---|---|---|---|
| Cell label (the instance name) | 2 sixteenths (cap height 1 layout unit) | 1 sixteenth | top-left inside the cell: grid origin at `(cell.x + 1, cell.y + 3/4)` |
| Out-port value | 4 sixteenths (cap height 2) | 2 sixteenths | right-aligned inside the port's cell, ending 2 units left of the port centre, cap centred on the port's y |
| System title (lens name) | 12 sixteenths (cap 6) | 6 sixteenths | system chart `(8, 4)` |
| Galaxy title | 32 sixteenths (cap 16) | 16 sixteenths | galaxy chart `(16, 8)` |

- A value is drawn for each **filled out-port that has a port row**, decimal, `-` for negative. At most 4 glyphs; longer prints 3 and `…`. A value changes by a delta: `present` rewrites that port's stroke rows, and regrow must equal (rule 58).
- Text draws in the Full band only (titles with their frame or node, at Summary and above, and on a lens node at any band).
- A stroke owns pixels for its owner: a label for its cell, a value for its port, a title for its system or galaxy.

**The zoom target.** In body `b0000` (calc, at universe `(88, 112)`), the cell `sum` is at `(24, 4)`; its label's first glyph `s` occupies body-chart `x 25 … 25 3/4`, `y 4 3/4 … 5 3/4`. Its centre is universe point **`(113 3/8, 117 1/4)`**. At level 9 step 0 (`k 512`) the `s` is 384 × 512 px.

### 2.9 Tables and the tick uniform

The five Phase 6 tables keep their row layouts **byte for byte**. New tables (all fields `u32`/`i32`, little-endian, rows padded to 16):

| Table | One row per | Fields |
|---|---|---|
| **Chart** | chart | `origin_x, origin_y` (relative to the anchor, layout units), `parent` (chart slot, or itself for the universe), `kind` (0 universe, 1 galaxy, 2 system, 3 body), `size` (`max(w, h)`, layout units), `generation`, `flags` |
| **Frame** | system or galaxy | `chart`, `w`, `h`, `kind`, `index`, `generation`, `flags` |
| **Stroke** | stroke | `chart`, `x0, y0, x1, y1` (sixteenths, in that chart), `half_width` (sixteenths), `owner` (`[R, G, B]` of the owner's ID), `style`, `generation`, `flags` |

How a body row finds its chart (a field on a new side table, or the chart slot equal to the body slot) is Cursor's. A single-body scene has exactly one chart row, the body's, at origin 0.

**New style ids:** 8 frame, 9 lens node, 10 text. Colours are decision; Cursor picks them so that 8, 9, 10 differ from each other and from 0 … 7.

**The tick uniform**, 48 bytes:

```
level i32 · step u32 · pin_x i32 · pin_y i32 · origin_px_x i32 · origin_px_y i32 ·
origin_frac_x u32 · origin_frac_y u32 · width u32 · height u32 · 0 · 0
```

`origin` is the pixel position of the anchor's origin, `pin − k·focus`, exact: whole pixels plus a fraction in units of 2^-32 px (for level ≥ −4 the fraction is always a whole number of those units). The shader places a point `p` (sixteenths, anchor-relative) at `origin + k·p/16`, computed as `f32`. That is the only float, and it only places. Every discard, band and owner test in the shader is in `i32`/`u32` (V141). An overflow in those tests is a refusal from `tick_bytes` before drawing, with its own test.

**IDs.** A universe body's ID red channel is its body slot (1 … 3072). `SYSTEM_TAG = 0x3000_0000`, `GALAXY_TAG = 0x4000_0000` in blue. A dot or glyph band body writes its surface ID. Owners print as Phase 6 (`b0000.sum`, `b0000.sum@2`), plus `system g0s00` and `galaxy g0`.

**Gates 6 and 7 do not move.** Their pictures are drawn through the converted camera (§2.2). `cargo xtask pick` and `cargo xtask regrow` print every Phase 7.1 line unchanged.

### 2.10 Exact pick at any zoom

The CPU pick takes coordinates in units of **2^-32 px** in `i128` (instead of Phase 6's sixteenths). The edge band stays **1/16 px** (2^28 units): inside when `d ≤ −2^28`, outside when `d ≥ 2^28`, edge otherwise. A dot is a circle of radius 2 px in pixel space. `cpu_pick` and `cpu_pick_reference` agree on every pixel of the calculator at the four standard viewports exactly as before, and on 4096 seeded pixels (SplitMix64, seed 7) of each §2.12 view of the grove. Pixels in a crossfade window are judged by the owner band.

### 2.11 The shell

`cargo run -p joinn-shell-desktop -- <path>` accepts `.universe` (and still `.body` and `.contact`). A universe binds its bodies from the corpus by hash; a body the corpus doesn't hold is refused naming its alias and hash. It opens the **first lens in canonical order** (`--lens NAME` to choose) and frames the universe.

| Input | Does |
|---|---|
| Wheel up / down | zoom one notch in / out about the cursor pixel |
| Left drag (moved ≥ 4 px before release) | pan by whole pixels |
| Left click (moved < 4 px) | pick, as Phase 6: a `pick …: <owner> (cpu)` line, then the GPU confirm line |
| `+` / `-` | zoom one notch about the centre pixel |
| Arrow keys | pan 64 px |
| `F` | frame the universe (or the body) again |
| Resize | zoom, focus and pin unchanged; nothing jumps |

For a `.body` or `.contact`, the same inputs work; typing into a selected port works as before.

Every tick prints one line: `tick: level <L> step <t> (k <fraction>), anchor <chart>, rows <n>`, where `rows` counts table rows written that tick. A pan or zoom prints `rows 0`; a rebase prints the chart rows it wrote. **An idle window prints nothing** (V12, rule 59): no clock, no animation, the fade is a function of zoom.

### 2.12 The predicted views (1920 × 1080)

Claude computed these from §2.4–§2.7 with an independent script in exact fractions. `cargo xtask zoom` prints exactly these lines (format in §3), then the touches lines, then its last line.

```
zoom 1920x1080 frame level -2 step 95 (k 351/1024), anchor universe: galaxies 8 open, 0 nodes · systems 128 open, 0 nodes · bodies dot 0, glyph 3072, summary 0, full 0 · fading 0
zoom 1920x1080 at s level -4 step 0 (k 1/16), anchor b0000: galaxies 8 open, 0 nodes · systems 0 open, 128 nodes · bodies dot 0, glyph 0, summary 0, full 0 · fading 0
zoom 1920x1080 at s level -3 step 0 (k 1/8), anchor b0000: galaxies 8 open, 0 nodes · systems 128 open, 0 nodes · bodies dot 1891, glyph 1181, summary 0, full 0 · fading 0
zoom 1920x1080 at s level -2 step 0 (k 1/4), anchor b0000: galaxies 6 open, 0 nodes · systems 96 open, 0 nodes · bodies dot 0, glyph 2240, summary 0, full 0 · fading 0
zoom 1920x1080 at s level -1 step 0 (k 1/2), anchor b0000: galaxies 4 open, 0 nodes · systems 36 open, 0 nodes · bodies dot 0, glyph 864, summary 0, full 0 · fading 0
zoom 1920x1080 at s level 0 step 0 (k 1), anchor b0000: galaxies 1 open, 0 nodes · systems 16 open, 0 nodes · bodies dot 0, glyph 153, summary 113, full 0 · fading 0
zoom 1920x1080 at s level 1 step 0 (k 2), anchor b0000: galaxies 1 open, 0 nodes · systems 4 open, 0 nodes · bodies dot 0, glyph 0, summary 80, full 0 · fading 0
zoom 1920x1080 at s level 2 step 0 (k 4), anchor b0000: galaxies 1 open, 0 nodes · systems 1 open, 0 nodes · bodies dot 0, glyph 0, summary 24, full 0 · fading 0
zoom 1920x1080 at s level 3 step 0 (k 8), anchor b0000: galaxies 1 open, 0 nodes · systems 1 open, 0 nodes · bodies dot 0, glyph 0, summary 4, full 8 · fading 0
zoom 1920x1080 at s level 4 step 0 (k 16), anchor b0000: galaxies 1 open, 0 nodes · systems 1 open, 0 nodes · bodies dot 0, glyph 0, summary 0, full 4 · fading 0
zoom 1920x1080 at s level 5 step 0 (k 32), anchor b0000: galaxies 1 open, 0 nodes · systems 1 open, 0 nodes · bodies dot 0, glyph 0, summary 0, full 2 · fading 0
zoom 1920x1080 at s level 6 step 0 (k 64), anchor b0000: galaxies 1 open, 0 nodes · systems 1 open, 0 nodes · bodies dot 0, glyph 0, summary 0, full 1 · fading 0
zoom 1920x1080 at s level 7 step 0 (k 128), anchor b0000: galaxies 1 open, 0 nodes · systems 1 open, 0 nodes · bodies dot 0, glyph 0, summary 0, full 1 · fading 0
zoom 1920x1080 at s level 8 step 0 (k 256), anchor b0000: galaxies 1 open, 0 nodes · systems 1 open, 0 nodes · bodies dot 0, glyph 0, summary 0, full 1 · fading 0
zoom 1920x1080 at s level 9 step 0 (k 512), anchor b0000: galaxies 1 open, 0 nodes · systems 1 open, 0 nodes · bodies dot 0, glyph 0, summary 0, full 1 · fading 0
touches level -4 step 0: 137 links, 264 touches, each node at most once per link
touches frame: 137 links, 1182 touches, each node at most once per link
```

"at s" views: focus = the `s` centre of §2.8, pin = `(960, 540)`. The frame view: §2.2's frame of the universe. A body counts if visible and its system and galaxy are open; a system or galaxy counts once, as open or as a node, if visible. At level −4 every system is a node and its bodies are not walked. No view in this block sits in a fade window.

### 2.13 Gate 7.2

Three items in `xtask/src/fns/gate_seven_two_items.rs`, built like `gate_seven_items.rs`. `"phase 7.2"` goes into `PHASE_LABELS` after `"phase 7"`, and `gates.lock` gains `phase 7.2: 3/3`, written by the lock writer. GPU parts run on every adapter and fail on none (rule 60). The grove is generated inside the gate (seed 7); the controls are corpus artifacts (rule 24).

| # | Item | `control_artifact` | `opposes` | Control answers `true` when |
|---|---|---|---|---|
| 1 | **The zoom is exact, and a rebase moves nothing** | `corpus/phase5/universe.universe` | `DropLens("function")` | `layout_universe(subject, "function")` refuses |
| 2 | **Bands follow size, and two pickers name every pixel** | `corpus/phase5/adversary.universe` | `CopyMember("function", "units", "calculation")` | the subject is refused at admission, or some body would be drawn twice in one lens |
| 3 | **A folded system is one node, and a link touches it once** | `corpus/phase5/adversary.universe` | `ShiftPort("bus", "calc.sum@2", 9)` | the subject is refused at admission, or some link's touch count differs from the number of distinct cut nodes its members map to |

**Checks:**

1. **The zoom is exact, and a rebase moves nothing.**
   - Every §2.2 refusal with its wording; level −5 and 10 refused.
   - The zoom script on the grove (frame → focus on the `s` with pin `(960, 540)` → one notch at a time about that pin until the level is 9 → the same number of notches out): every pan and zoom writes **0 rows**; every rebase writes chart rows only; the whole script ends with tables equal to a fresh grow at the final anchor (rule 58).
   - At every rebase in the script, on every adapter, color and ID bytes before and after are **identical**.
   - 8 notches in then 8 out about one pixel returns the identical camera and identical bytes (V144).
   - The calculator's Phase 6 cameras convert exactly, and gate 6's and gate 7's pictures are unchanged (their items pass).
2. **Bands follow size, and two pickers name every pixel.** On every adapter, at every §2.12 view:
   - `cpu_pick` equals `cpu_pick_reference` on the 4096 seeded pixels; every non-edge pixel's GPU owner equals `cpu_pick`;
   - the set of owners in the GPU image equals the set the CPU cut allows (V145);
   - the CPU cut's counts equal §2.12's lines;
   - at a body in a fade window (Cursor picks one zoom per threshold, printed), its pixels' owner is the owner band's.
3. **A folded system is one node, and a link touches it once.**
   - At level −4 step 0: 128 node owners, no body owner; touches 264.
   - At the frame: touches 1182.
   - On `adversary.universe` framed at level −4: each of its three systems is one node, and link `bus` touches 3 nodes.

Rule 41 holds: the three `(control_artifact, opposes)` pairs are new. A test asserts only gate 7.2's rows name the grove's generator.

### 2.14 Invariants

| # | Invariant | Commit |
|---|---|---|
| **V141** | No float decides anything: bands, owners, the cut and the rebase are integer decisions, the same on CPU and GPU | P72-05, P72-08 |
| **V142** | A pan or zoom writes the tick uniform and no row; a rebase writes chart rows only | P72-07 |
| **V143** | A rebase moves nothing: color and ID bytes identical before and after | P72-10 |
| **V144** | Zoom about one pixel is reversible: n notches in and n out return the identical camera and bytes | P72-02, P72-10 |
| **V145** | Two cuts, one truth: the owners in the GPU image are exactly those the CPU cut allows | P72-09 |
| **V146** | A link touches each cut node at most once | P72-05 |
| **V147** | An idle window draws nothing, at any zoom (V12 extended) | P72-11 |

---

## 3. What runs

| Command | Prints | Exit 1 when |
|---|---|---|
| `cargo xtask grove [--seed N] [--out PATH]` | `grove seed 7: 8 galaxies, 128 systems, 3072 bodies (calc 1181, units 982, bus 909), 137 links, 1182 members, admitted; hash <hex>` | not admitted, or `--out` under `corpus/` or `docs/` |
| `cargo xtask layout --universe <path\|grove> [--lens NAME]` | §2.5's lines | layout refuses |
| `cargo xtask zoom` | §2.12's block; last line `zoom: 15 views, cut counts printed; touches 264 and 1182` | any refusal |
| `cargo xtask zoom --measure` | per view, `cut cpu <µs> µs, draw instances <n>` (release build advised); wall time never reaches a hash or a refusal (rule 4) | — |
| `cargo xtask pick` | the Phase 7.1 lines unchanged, then per adapter and §2.12 view `grove <view>: sample 4096 agree <a>, edge <e>, disagree 0, owners <n> (cut allows <n>)` | as Phase 6, or owners ≠ cut |
| `cargo xtask regrow` | the Phase 7.1 lines unchanged, then the zoom script: `grove step <i> <action>: rows <r>, <anchor>, tables equal regrow`, and per adapter `rebase <from> -> <to> <adapter>: color identical, ids identical` | as Phase 6 |
| CI | new step `zoom` after `contact` on both jobs: `cargo xtask grove` then `cargo xtask zoom` | the step fails |

---

## 4. The Commits

Done-when is a command, and the command must be able to fail.

### Chunk A: the camera, the grove, layout, the cut (CPU only)

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P72-01** | **Plan, rules, docs** | Commit this plan. `AGENTS.md` per Appendix A. Commit the doc edits already on disk unchanged, **then** apply Appendix B. No code | `git show --stat HEAD` lists this plan, `AGENTS.md`, the backlog, `decisions.md` and the roadmap (paste `git status` first). Nothing under `joinn/` except `AGENTS.md` |
| **P72-02** | **The camera** | §2.2: `Zoom`, `Camera`, notches, zoom about a pixel, refocus, pan, `frame`, the Phase 6 conversion, refusals | Tests: every refusal's wording; `k` for every (level, step) from −4/0 to 9/255 equals `2^L(256+t)/256`; every whole `k` 1 … 511 converts and back; 8 in then 8 out about one pixel returns the same camera; refocus moves the focus by less than 2^-16; `frame` of 5504 × 1600 at 1920 × 1080 is level −2 step 95 |
| **P72-03** | **The grove** | §2.4; `cargo xtask grove` | Its line printed whole (paste it); counts as §2.4; the SplitMix64 test (first three draws); same hash twice with seed 7, a different one with 8; `--out corpus/x.universe` refused |
| **P72-04** | **Universe layout and charts** | §2.5, §2.3's chart chain and anchor rule (no tables yet); `cargo xtask layout --universe` | `cargo xtask layout --universe grove` last line as §2.5 (paste it, and the lines for `g0`, `g0s00`, `b0000`). The lens-not-found and too-big refusals each tested. Anchor tests: the `s` centre is in `b0000`; the frame centre `(2752, 800)` is in the universe only |
| **P72-05** | **Bands and the cut** | §2.6's owner rule in integers, §2.7's CPU cut, lens nodes, touches; `cargo xtask zoom` | `cargo xtask zoom` prints §2.12's block byte for byte (paste it whole). Tests: the owner rule at `10s = 11T` exactly (higher band) and one step below (lower band); touches 264 and 1182 |
| — | **Stop A** | `phase-7.2-stop-a.md` (§0.2), push, stop | — |

### Chunk B: text, tables, the shader, exact pick, the rebase on the GPU

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P72-06** | **The stroke font** | §2.8's glyph set and text layout | Tests: every glyph inside its box; no two glyphs share a stroke set; an unknown character refused with §2.8's wording; the `s` of `b0000`'s `sum` label occupies §2.8's box exactly; `12345` prints `123…` |
| **P72-07** | **Tables and the universe scene** | §2.9's Chart, Frame and Stroke tables; grow and regrow for a universe; `present` writes stroke rows for values; rebase rewrites chart rows only | Tests: a pan and a zoom write 0 rows; the frame → `s` rebase writes exactly the chart rows (count printed, ≤ chart count); tables after the zoom script equal regrow; a single-body scene has one chart row; the five Phase 6 row layouts unchanged (`row_bytes` tests untouched and passing) |
| **P72-08** | **The tick uniform and shader** | §2.9's 48-byte tick, the shader's integer decisions, frame and stroke pipelines, the dot | `cargo xtask pick` and `cargo xtask regrow` print every Phase 7.1 line unchanged (paste a `Compare-Object` of those lines that prints nothing). `cargo xtask gate 6` and `gate 7` each 3/3. The tick overflow refusal tested |
| **P72-09** | **Exact pick at any zoom** | §2.10; `cargo xtask pick` gains the grove views | Whole `pick` output pasted. Every grove line `disagree 0` and `owners n (cut allows n)` with equal n |
| **P72-10** | **The rebase on the GPU** | `cargo xtask regrow` gains §3's zoom script lines | Whole `regrow` output pasted: every grove step `tables equal regrow`, every rebase `color identical, ids identical`, on every adapter. Shown then reverted: round the focus to whole units during rebase; paste the line that fails |
| — | **Stop B** | `phase-7.2-stop-b.md`, push, stop | — |

### Chunk C: the shell, the measurement, gate 7.2, freeze

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P72-11** | **The shell zooms** | §2.11 | A test drives the session (no window): open the grove, frame, 8 notches in about `(960, 540)`, a drag of `(10, 0)`, a resize to 1280 × 720; the tick lines are pasted, every pan/zoom `rows 0`. V147: after the last input, 0 ticks. Cursor runs the window once on Windows with `target/grove.universe` and pastes the first tick line and one pick line (or writes `no display`) |
| **P72-12** | **The measurement** | `cargo xtask zoom --measure` in release | Output pasted. No threshold: the numbers go to the findings |
| **P72-13** | **Findings** | `docs/Findings/phase-7.2-zoom.md`: every command's output from P72-03 on; a *Predictions* section marking §2.2's frame, §2.4's counts, §2.5's last line, §2.12's 17 lines, and §2.13's touch counts `as predicted` or `differs`; **The adversary**: whether the CPU cut grows with the universe per tick (it should not run per tick at all) and the measured µs; **What 7.3 needs**: observed, one line each | Every prediction marked |
| **P72-14** | **Gate 7.2** | §2.13; uniqueness through gate 7.2; `phase 7.2` label; lock row | `cargo xtask gate all` exits 0 and prints `phase 7.2: 3/3` after `phase 7: 3/3`. Shown then reverted, pasting each printed failure: (a) give item 2 item 3's `opposes` (uniqueness refusal); (b) make the shader's owner test use `>` instead of `≥` (item 2 must fail at the fade-window body); (c) write chart rows on a plain zoom (item 1 must fail on rows) |
| **P72-15** | **Docs and freeze** | README and `Guides/03-where-we-are.md` (the universe zooms; what isn't built: hyperedges drawn, snapshots); `Guides/05-glossary.md` gains *zoom level, step, focus, pin, chart, anchor, rebase, band, crossfade, owner band, the cut, lens node, touch, stroke font, the grove*; `decisions.md` V141–V147 with status; the roadmap's 7.2 note points at the findings | `cargo xtask gate all` from a fresh clone prints phases 0 … 7.2 and exits 0. `corpus verify` 44. `git diff --stat <P72-01>..HEAD -- joinn/corpus` prints nothing |
| — | **Stop C** | `phase-7.2-stop-c.md` with the CI read, push, stop | — |

Dependencies: in order. P72-08 depends on P72-07; P72-09 and P72-10 on P72-08; P72-14 on P72-09 and P72-10.

**If something has to be cut for time:** cut galaxy titles first, then the dot band (draw dots as glyphs and say so), then `--measure`. Never cut V141, V142, V143, V145, or gates 6 and 7 staying unchanged.

---

## 5. Test Strategy

Every earlier invariant is re-run on every commit; gates 0 … 7 still pass, and no score moves. **A test must never** assert only that something ran. It asserts the exact count, the rows written, the bytes, the owner, or the refusal's words. Camera tests assert exact fractions, never decimals. Picture tests compare bytes. Tests that need the grove generate it (seed 7); none reads it from disk.

---

## 6. Exit Gate 7.2

`cargo xtask gate 7.2`, from a fresh clone. It needs at least one GPU adapter.

- [ ] **1 · The zoom is exact, and a rebase moves nothing.** *Opposes* `DropLens(function)`.
- [ ] **2 · Bands follow size, and two pickers name every pixel.** *Opposes* `CopyMember(function, units, calculation)`.
- [ ] **3 · A folded system is one node, and a link touches it once.** *Opposes* `ShiftPort(bus, calc.sum@2, 9)`.

### 6.1 Conditions for opening Phase 7.3

1. Gate 7.2 passes; `gate all` exits 0; CI green on both OSes, the `zoom` step included (read, not assumed).
2. `phase-7.2-zoom.md` exists with every prediction marked and the adversary answered.
3. AJ has run the window (§6.2) and said it worked, or each thing that looked wrong is a snag Claude has settled.
4. Claude's Stop C review lists no open snag, or AJ has chosen to carry each one forward.

**Phase 7.3** (planned after this): hyperedges drawn as region, hub, bundle and spine; routing around membranes; rerouting to lens nodes; picking a hyperedge (link, member); snapshots if 7.2's measurement asks for them.

### 6.2 AJ's window check (Stop C, about four minutes)

In PowerShell:

```
cd D:\JoInn\joinn
cargo xtask grove --out target/grove.universe
cargo run --release -p joinn-shell-desktop -- target/grove.universe
```

1. A window shows **8 big tiles** (galaxies) in two rows of four, each holding 16 smaller tiles (systems), each full of small blocks (bodies).
2. Put the cursor on the **top-left** small block of the top-left system of the top-left galaxy, and scroll in slowly. The blocks grow; they show their cells; then ports, wires and small names appear. Nothing jitters or jumps.
3. Keep going until one letter fills the window (the **s** of **sum**). The terminal's tick lines say `rows 0` while zooming, and once say `anchor b0000` with some rows.
4. Scroll back out all the way. Systems fold into single tiles with names.
5. Drag with the mouse to pan. Press `F`. The whole universe comes back.
6. Resize the window by dragging its corner. The picture stays put and grows or shrinks smoothly, no jumps.
7. Take your hands off the mouse. No more tick lines print.
8. Close the window.

Tell Claude "window worked", or which step looked wrong.

---

## 7. Risks

| Risk | What to do |
|---|---|
| **The cut costs per tick** (the phase's adversary) | Measured in P72-12. If the GPU-side cut needs something Core limits don't give, that is the finding, written in `phase-7.2-zoom.md`; never a workaround |
| The shader's integer band test overflows `u32` at some zoom | `tick_bytes` refuses before drawing; a snag with the zoom and the size. Never move a decision to float |
| Gates 6 or 7 change by a pixel after the tick change | A truth problem in the conversion. Paste the pixel line. Never rebless |
| A count differs from §2.12 | A prediction that differs. Paste both. Exact fractions decide |
| The grove isn't admitted (a link's members disagree, a port in two links) | A snag with the refusal. Never change a corpus body to make it fit |
| The stroke font looks bad | Presentation; not a snag. AJ judges at Stop C |
| `CopyMember`, `DropLens` or `ShiftPort` doesn't apply to its artifact as §2.13 says | A snag with the printed line; Claude settles the row at the stop |

---

## 8. Open Items

| ID | Topic | Question |
|---|---|---|
| **R101** | Where the cut lives | Decided twice (GPU per instance, CPU on demand) from the same integers. Is "two cuts, one truth" the general rule for every derived view (accessibility in Phase 8)? |
| **R102** | Hysteresis or fade | A size-driven fade replaces separate up/down thresholds. Does any band change need hysteresis once bands carry live interiors (editing in Phase 9)? |
| **R103** | Text as cells | Labels and values are strokes owned by a cell or port. When a label is edited in the creator, is a label a cell of its own, or presentation of a name? |
| **R104** | Layout of a lens | The grove's grid is fixed (6 / 4 / 4). Is a lens's layout a body of its own (a layout system), so other lenses can lay out differently? |
| R4a | The chart in 3D | Still open; the crane mat question. 7.2's charts are 2D |
| R87–R100 | From Part VI and 7.1 | Stay open |

---

## Appendix A · `AGENTS.md` changes

Replace the header paragraph with:

```markdown
# JoInn — standing rules

This repo is JoInn. Phase 7.2 is Visual Host II, part 1: the zoom. A universe
on screen, zoomed exactly from all of it to one letter, with charts, bands, the
cut and JoInn's own stroke font. Its one idea is NO FLOAT DECIDES ANYTHING. The
build plan is docs/Plans/JoInn Phase 7.2 Implementation Plan.md. Work one CHUNK
at a time (A, B or C), one git commit per numbered step, and stop at the chunk's
stop report. Every decision is in the plan. Never stop to ask; follow the plan's
Snags section instead.

No new grammar in this phase. Hyperedges are counted, not drawn (Phase 7.3).
```

Keep every rule exactly as it is. Append:

```markdown
70. NO FLOAT DECIDES ANYTHING. Bands, owners, the cut and the rebase are integer
    decisions, made the same way on the CPU and in the shader. A float only
    places a pixel, after every decision is made.
71. THE CAMERA IS EXACT. Zoom is a level and a step; the focus is a dyadic
    point of the anchor chart; the pin is a whole pixel. A pan or zoom writes
    the tick uniform and no row. A rebase writes chart rows only and moves
    nothing.
72. TWO CUTS, ONE TRUTH. What the GPU draws and what the CPU says can be drawn
    are the same set of owners. A difference is a truth violation.
73. GENERATED IS NOT STORED. The grove is grown from its seed wherever it is
    needed and never written under corpus/ or docs/.
```

---

## Appendix B · Documents (P72-01 applies these after committing the on-disk edits)

### B1 · The research backlog (`docs/Theory/JoInn Research Backlog.md`)

Append, after R100:

```markdown
> **3 Oct 2026 · Phase 7.2.** R101–R104 come from the zoom plan.

## R101 — Where the cut lives  ·  status: open
## R102 — Hysteresis or fade  ·  status: open
## R103 — Text as cells  ·  status: open
## R104 — Layout of a lens  ·  status: open
```

Each heading is followed by its question, copied from this plan's §8.

### B2 · `docs/Findings/decisions.md`

Add rows R101–R104 with phase `7.2`, status `open`. Add rows V141–V147 with status `open` (P72-15 sets them).

### B3 · The roadmap (`docs/Plans/JoInn Build Roadmap.md`)

- After the 2 Oct note, add:

```markdown
> **3 Oct 2026 · The UX first; Visual Host II in two.** AJ: the beam's stage 2
> waits. Bodies like the beams will be built in JoInn itself, once the creator
> exists; the order stays 7.2, 8, 9. Visual Host II is split: **Phase 7.2 is the
> zoom** (exact charts, bands, the cut, lens nodes, JoInn's own stroke font,
> `Plans/JoInn Phase 7.2 Implementation Plan.md`), and **Phase 7.3 is the links**
> (hyperedges drawn as region, hub, bundle and spine, rerouting, snapshots).
> Every later phase keeps its number.
```

- In the 2 Oct note, change `Stage 2 (the tag in JoInn's grammar) is planned from its *What stage 2 needs*.` to `Stage 2 (the tag in JoInn's grammar) waits for the creator (3 Oct note).`
- Retitle `#### Phase 7.2 · Visual Host II: charts, zoom, bands, links` to `#### Phase 7.2 · Visual Host II: charts, zoom, bands`, move the **Hyperedge forms** and **Snapshots** deliverables into a new `#### Phase 7.3 · Visual Host II: links` section after it, with the exit-gate sentence about collapse and rerouting, and add to 7.2's deliverables: *the stroke font* and *two cuts, one truth*.
- In the diagram, `P7["Phase 7.2<br/>visual host II"]` becomes `P7["Phase 7.2<br/>zoom"]`, followed by a new node `P73["Phase 7.3<br/>links"]` between it and Phase 8.

---

*JoInn Phase 7.2 Implementation Plan, Draft 0.1 (3 Oct 2026). Opens after Phase 7.1's Stop C review. AJ decided on 3 Oct: the beam's stage 2 waits for the creator; roadmap order; Visual Host II split into 7.2 (zoom) and 7.3 (links); exact all the way; JoInn's own stroke font; a generated, seeded universe. Claude decided, open to AJ's veto before chunk A: two cuts, one truth; a size-driven fade instead of hysteresis; snapshots to 7.3; hyperedges counted, not drawn; values beside out-ports only. Cursor executes. Claude verifies at each stop. AJ runs the window at Stop C.*
