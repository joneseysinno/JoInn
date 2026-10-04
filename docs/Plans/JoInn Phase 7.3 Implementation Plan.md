# JoInn Phase 7.3 Implementation Plan

**Visual Host II, part 2: the links · routes in the gutters · region, hub, bundle, spine · a folded system touched once**

Author: AJ, with Claude · Draft 0.1 · October 3, 2026

> **Every decision in this plan is final.** Nothing here waits on AJ. Cursor never stops to ask AJ anything, and no file in this phase is typed by AJ. If a step can't be done the way the plan says, Cursor follows §0.3 (Snags) and keeps going. This phase runs inside `docs/Plans/JoInn Run 7.2B-9.md`; its tripwires apply.

---

## For AJ: this plan in plain English

**What Phase 7.3 is.** Phase 7.2 counted the links between bodies but didn't draw them. Phase 7.3 draws them, the JoInn way: a hyperedge is one thing touching several bodies, not a tangle of separate wires, and it **never passes through a body**.

**How a link finds its way.** The universe is laid out on a grid, so there are always empty lanes between bodies, between systems and between galaxies, like streets between buildings. Every link travels only on those streets. A short **stub** goes from the street to the port on the body's edge, and stops there. Because the streets never go through a building, no link can cross a body. That is true by construction, and the gate still checks every piece exactly.

**What you see, by zoom.**

| The link on screen | Drawn as |
|---|---|
| Small (under 240 px across) | **Region**: its streets drawn wide and pale |
| Medium (240 to 1920 px) | **Hub**: thin lines meeting at a knot |
| Large (over 1920 px) | **Bundle**: the shared stretch of street drawn thicker the more legs share it |
| An ordered link, any size | **Spine**: one path visiting its members in order, with arrowheads |

Tails (senders) have filled ports with an arrowhead leaving them; heads are hollow. A link with no order never shows arrowheads along its length.

**When you zoom out and a system folds into a tile**, its links stop at the tile's edge, and each link touches that tile **once**, however many of its members are inside. Since all systems are the same size, they all fold at the same zoom; so every link has just two routes (open and folded), both computed when the universe is loaded. Zooming still writes nothing but the camera.

**Clicking a link** names it: `link sys_g0s00`, or `link sys_g0s00 member 3` on a leg that serves one member.

**What I decided (say if you disagree):** streets instead of free routing; region/hub/bundle as one path at three thicknesses; links drawn under bodies so a body always owns its own pixels; snapshots wait for a measurement that asks for them.

**What you do at the end** (about three minutes, §6.2): open the grove, zoom into one system and watch its link become a hub, then fold it by zooming out.

---

## 0. How Cursor Works This Plan

| | |
|---|---|
| **Plan file** | `D:\JoInn\docs\Plans\JoInn Phase 7.3 Implementation Plan.md` |
| **Code** | `D:\JoInn\joinn\`. No new crate. No new external dependency |
| **Standing rules** | `AGENTS.md` as updated by P73-01 (Appendix A) |
| **Unit of work** | a **chunk** (A, B or C), commits in order, each its own git commit whose message starts with its id (`P73-04: …`) |
| **End of a chunk** | the stop report (§0.2), commit, push, ledger line, tripwires, then continue per the run plan |
| **Who decides** | every decision is in §2. Cursor decides only module layout, function bodies, private representations, the routing graph's internal storage, tie-break implementation (not the rule), and error wording where §2 gives none |
| **Who checks** | Claude, after the run, from a fresh clone. CI on every push. AJ runs the window at the end of the run |
| **Input** | Never move the system mouse, click, or type outside a window Cursor started (rule 67) |

### 0.1 The commit report

As Phase 7.2's §0.1, plus from P73-04 on: `Links:      cargo xtask links → <last line>`.

### 0.2 The stop report

As Phase 7.2's §0.2, at `docs/Findings/phase-7.3-stop-<letter>.md`, with these additions to item 3: `cargo xtask grove`, `cargo xtask zoom`, and from P73-04 on `cargo xtask links`. Then the run plan's §5 lines.

### 0.3 Snags

As Phase 7.2's §0.3. Predictions that differ are pasted beside the plan's value and work continues.

---

## 1. Scope Fence

### In scope

- **The routing graph**: gutter lines at every level of the grid layout, joined by extension segments (§2.2).
- **Touch points** for members: stubs to ports, or a lens node's side when folded (§2.3).
- **Routes**: the knot, legs, spines, both fold states, computed at grow (§2.4).
- **Forms**: region, hub, bundle, spine by the owner rule over the route's size (§2.5).
- **V16 checked exactly** in layout units (§2.6).
- **Tables**: Route and Segment; the shader draws them; the ID target names links and members (§2.7, §2.8).
- **The shell** draws links; picking a link prints its owner.
- **Gate 7.3**, findings `docs/Findings/phase-7.3-links.md`, the freeze.

### Out of scope

| Not now | Why it is tempting |
|---|---|
| Snapshots, atlas pages, a re-render budget | No measurement asks for them yet (R105) |
| Free (non-gutter) routing, curves, splines | Gutters make crossing impossible; curves are presentation for later |
| Changing the grid layout of §2.5 of Phase 7.2 | The gutters are a property of that grid |
| Animated probes or message pulses on links | Running the grove is not in scope |
| Editing a link (adding, removing members) | The creator, Phase 9 |
| Accessibility of links | Phase 8 |
| Any grammar change, any corpus change | No new grammar. The corpus does not change |
| A GPU compute pass | Routes are computed once per grow on the CPU |
| Changing earlier row layouts (the five Phase 6 tables, Chart, Frame, Stroke) | Byte for byte, as 7.2 left them |

---

## 2. Decisions

All **DECIDED**.

### 2.1 Where things live

| Crate | Gains |
|---|---|
| `joinn-visual` | the routing graph, touch points, routes, forms, the V16 check, Route and Segment tables, link owners in the CPU pick and cut |
| `joinn-gpu` | the segment pipeline (capsules, knots, arrowheads) and the shader's integer form and fold tests |
| `joinn-shell-desktop` | nothing new but drawing; pick prints link owners |
| `xtask` | `links`, extended `pick`, `regrow`, `zoom --measure`, gate 7.3 |

No new workspace edge.

### 2.2 The routing graph (the gutters)

All coordinates are integers in layout units, in the chart named. Each grid line spans its container from its first to its last line in the other direction.

| Level | Vertical lines at x = | Horizontal lines at y = |
|---|---|---|
| **System chart** (304 × 152) | `4 + 48c`, c = 0 … 6 | `12 + 32r`, r = 0 … 4 |
| **Galaxy chart** (1296 × 704) | `8 + 320c`, c = 0 … 4 | `24 + 168r`, r = 0 … 4 |
| **Universe chart** (5504 × 1600) | `32 + 1360c`, c = 0 … 4 | `32 + 768r`, r = 0 … 2 |

Why these are free: a body slot spans at most `[8 + 48c, 48 + 48c) × [16 + 32r, 40 + 32r)`, a system `[16 + 320c, 320 + 320c) × [32 + 168r, 184 + 168r)`, a galaxy `[64 + 1360c, 1360 + 1360c) × [64 + 768r, 768 + 768r)`. Each line sits in the gap between them. A test asserts it for every slot.

**Extension segments** join a level to its parent's lines:

- Every horizontal line of a system grid extends left from `x = 4` to `x = −8` and right from `x = 292` to `x = 312` (system chart); every vertical line extends up from `y = 12` to `y = −8` and down from `y = 140` to `y = 160`. These end exactly on the galaxy's lines.
- Every horizontal line of a galaxy grid extends from `x = 8` to `x = −32` and from `x = 1288` to `x = 1328`; every vertical line from `y = 24` to `y = −32` and from `y = 696` to `y = 736`. These end on the universe's lines.

(Check: system at galaxy `x = 16 + 320c` puts system `x = −8` on galaxy line `8 + 320c`, and `x = 312` on `8 + 320(c + 1)`. Galaxy at universe `x = 64 + 1360c` puts galaxy `x = −32` on `32 + 1360c` and `1328` on `32 + 1360(c + 1)`. Same for y: `−8` and `160` for systems; `−32` and `736` for galaxies.)

The **routing graph** is every grid line and extension, split at every crossing and every end, as nodes in universe coordinates and segments between neighbours. Its weight is length (Manhattan, integer). A universe whose lens is not the grid of Phase 7.2 §2.5 is refused by layout already; there is no other case.

**Predicted:** per system 7 × 5 = 35 crossings, per galaxy 25, the universe 15. Cursor prints the grove's node and segment counts (§3); they are not predicted beyond these.

### 2.3 Touch points

A **member** of a link maps to one touch point.

- **Body drawn** (its system open): the touch point is the port's centre. Its **stub** is the shortest path from the port centre to the routing graph that (a) leaves the port horizontally toward its side (in-ports left, out-ports right, Phase 6 layout), (b) runs only through the member body's own margins (`MARGIN / 2` from the surface edge) and column gaps (`GAP_X / 2` from cell edges), and (c) meets the routing graph at the nearest point of the gutter line on that side of the body's slot (`4 + 48c` left, `4 + 48(c + 1)` right). For every body in the corpus and the grove this stub is one horizontal segment; a body that needs a bend gets one, by the same rule. The stub's end on the gutter line becomes a node of the graph.
- **System folded** (a lens node): the touch point is the midpoint of the node's side facing the knot (§2.4): left `(0, 76)`, right `(304, 76)`, top `(148, 0)`, bottom `(148, 152)` in system chart (each lies on a grid line, so its extension continues it). Which side: if knot `x < 0` left; else if knot `x ≥ 304` right; else if knot `y < 0` top; else bottom. Galaxy nodes the same, at `(0, 360)`, `(1296, 360)`, `(648, 0)`, `(648, 704)`.
- **One touch per node** (V146, now drawn): all members inside one folded node share one touch point and one leg.

When a system is folded, its interior grid lines and stubs are removed from the graph; legs reach it only at its touch point.

### 2.4 Routes

For each link and each **fold state**:

- **Fold states.** Every system is 304 × 152 and every galaxy 1296 × 704, so systems fold at one zoom and galaxies at another (Phase 7.2's owner rule, Summary threshold). A universe has at most three fold states: *open*, *systems folded*, *galaxies folded*. For the grove at levels −4 … 9 only *open* and *systems folded* occur (a galaxy is at least 81 px). Every route for every state is computed **at grow**, never at a zoom.
- **The knot** (unordered links): the routing-graph node that minimizes the sum of shortest-path lengths to every touch point. Ties: smallest `y`, then smallest `x` (universe coordinates). A link whose touch points are all one point has no legs and no knot (drawn as nothing but its stub).
- **Legs**: the shortest path from the knot to each touch point. Ties between equal-length paths: at each node, prefer the neighbour with the smallest `(y, x)`. A **leg serves one member**; where several legs share a segment, that segment is **trunk**.
- **Spine** (ordered links): the concatenation of shortest paths from touch point `i` to `i + 1`, in incidence order. No knot.
- **Arrowheads**: on a spine, one at the midpoint of every path `i → i + 1`, pointing along it; on any link, one on the stub of every tail member, pointing away from the port. An unordered link has no arrowhead except on tail stubs.

### 2.5 Forms

A link's **size** in a fold state is `max(w, h)` of the bounding box of its touch points and knot, in layout units. Its projected size is `k · size`. Phase 7.2's owner rule (integers, `10·s ≥ 11·T`), with its 20 % crossfade window, chooses the form:

| Form | Owner when | Half-width (layout units) | Style |
|---|---|---|---|
| **Region** | `s < 264` (T = 240) | 3 | 12 region |
| **Hub** | `264 ≤ s < 2112` (T = 1920) | 1/4, knot disc radius 1 | 13 hub |
| **Bundle** | `2112 ≤ s` | `1/4 + 1/8 · (legs on the segment − 1)`, capped at 3; knot radius 1 | 13 hub |
| **Spine** | ordered, any `s` | 1/2, arrowheads 2 long, 2 wide | 14 spine |

Stubs are always drawn at hub half-width (1/4). Every half-width is a whole number of sixteenths. Every form's half-width fits its gutter (3 < 4, the narrowest half-gap).

**Drawing order:** frames → link segments → lens nodes → bodies, cells, ports → text. Links lie **under** bodies, so a body owns every pixel of its surface.

### 2.6 V16, checked exactly

For every route, in every fold state, in layout units (sixteenths):

1. No segment's capsule (with its largest half-width, 3 for region) meets any **cell rectangle** of any body, except a stub's end at its own port.
2. No segment meets a **non-member body's surface rectangle**.
3. No segment enters a **folded node's rectangle** except a leg ending at its touch point.
4. Every stub ends exactly on its port's centre.

`cargo xtask links` prints `crossings 0` per universe and fold state when all four hold. A violation prints the link, the segment and the rectangle, and exits 1.

### 2.7 Tables

Phase 6's Link and Incidence rows keep their layouts byte for byte. New tables (`u32`/`i32`, little-endian, padded to 16):

| Table | One row per | Fields |
|---|---|---|
| **Route** | link × fold state | `link`, `fold` (0 open, 1 systems folded, 2 galaxies folded), `size` (layout units), `ordered`, `segment_first`, `segment_count`, `generation`, `flags` |
| **Segment** | piece | `chart`, `x0, y0, x1, y1` (sixteenths, in that chart), `route`, `member` (index + 1, or 0 for trunk, knot, spine), `kind` (0 leg, 1 stub, 2 knot, 3 arrow), `legs` (sharing count), `style`, `generation`, `flags` |

A segment lives in the deepest chart that contains both its ends (system, else galaxy, else universe), so a rebase moves it exactly as 7.2's chart rows do. **A zoom writes no row** (V142 still holds): the shader chooses the fold state and the form from the tick uniform and the Route row, in integers.

**IDs.** `LINK_TAG = 0x5000_0000` in blue. A link pixel's ID is `[route's link slot + 1, member, LINK_TAG | form, generation]`. Owners print `link <id>` (trunk, knot, spine, region) or `link <id> member <i>` (a leg or stub serving one member, `i` from 0 in incidence order).

### 2.8 Picking

`cpu_pick` and `cpu_pick_reference` gain segments (capsules), knots (discs) and arrowheads (triangles), at Phase 7.2's precision (2^-32 px, edge band 1/16 px). Bodies and their contents win over links where both cover a pixel (drawing order). In a form crossfade window, the owner form's segments own the pixel.

### 2.9 What the shell shows

Links appear in the shell for any `.universe`. No new input. A pick on a link prints `pick …: link sys_g0s00 member 3 (cpu)` then the GPU confirm line. The tick line is unchanged.

### 2.10 Invariants

| # | Invariant | Commit |
|---|---|---|
| **V148** | A hyperedge touches; it never crosses. §2.6's four checks hold for every route in every fold state | P73-04 |
| **V149** | Routes live in gutters and are grown once per fold state; a zoom writes no row | P73-06 |
| **V150** | Order is drawn only when declared: no spine and no arrowhead along an unordered link | P73-05 |
| **V151** | A folded node is touched once per link: one touch point, one leg | P73-03 |
| **V152** | Two pickers name every link pixel: CPU and GPU agree on link owners at every view | P73-08 |

---

## 3. What runs

| Command | Prints | Exit 1 when |
|---|---|---|
| `cargo xtask links [--universe <path\|grove>]` | per universe (default: `universe.universe`, `ordered.universe`, the grove) and fold state: `links <name> fold <state>: <n> links, <legs> legs, <stubs> stubs, <spines> spines, knots <k>, crossings 0`; then the grove's graph line `graph grove: <nodes> nodes, <segments> segments`; last line `links: <u> universes, crossings 0, each folded node touched once` | a crossing, or a node touched twice |
| `cargo xtask links --forms` | per Phase 7.2 §2.12 view: `forms <view>: region <a>, hub <b>, bundle <c>, spine <d>, fading <f>` | — |
| `cargo xtask links --measure` | per universe: `route µs <t>` (release advised; never reaches a decision) | — |
| `cargo xtask pick` | every earlier line unchanged; per adapter, grove view: `… link owners <n> (cut allows <n>)` added to the grove lines; plus `ordered <view>: …` lines for `ordered.universe` at its frame | as before, or link owners ≠ cut |
| `cargo xtask regrow` | every earlier line unchanged; plus `routes equal regrow` on every grove step | as before |
| CI | the `zoom` step gains `cargo xtask links` | the step fails |

**Predicted** (from Phase 7.2's counts and §2.4's rules):

- Grove, fold *open*: **137 links, 1182 legs, 1182 stubs, 0 spines, knots 137**. Every member is its own touch point, and every link has at least two.
- Grove, fold *systems folded*: **137 links, 136 legs, 0 stubs, 0 spines, knots 9**. Each `sys_` link lies wholly inside one folded system: one touch point, so no leg and no knot. Each `gal_` link touches 16 nodes (16 legs); `uni` touches 8 (8 legs). Touches stay **264** (128 + 128 + 8).
- `universe.universe` and `ordered.universe`, fold *open* (their one link is ordered): **1 link, 0 legs, 2 stubs, 1 spine, knots 0**.

---

## 4. The Commits

### Chunk A: the routes (CPU only)

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P73-01** | **Plan, rules, docs** | Commit this plan. `AGENTS.md` per Appendix A. Appendix B. No code | `git show --stat HEAD` lists this plan, `AGENTS.md`, the backlog, `decisions.md`, the roadmap |
| **P73-02** | **The routing graph** | §2.2 | Tests: every grid line misses every slot of its level (all slots, all levels); every extension ends on a parent line; node counts per system 35, galaxy 25, universe 15; the grove's graph line printed (paste it) |
| **P73-03** | **Touch points and routes** | §2.3, §2.4; V151 | Tests: the knot rule on a hand-made 3-member example with a tie (smallest `y` then `x`); the side rule for each of the four sides; a link with three members in one folded system has one touch point and one leg; every grove link in fold *systems folded* touches each node once |
| **P73-04** | **V16, exactly** | §2.6; `cargo xtask links` | `cargo xtask links` printed whole (paste it); `crossings 0` everywhere. **Shown then reverted:** move one system gutter line 4 units into its bodies; paste the crossing it prints |
| **P73-05** | **Forms** | §2.5; V150; `links --forms` | Tests: the owner rule at `10s = 11T` for both thresholds; an unordered link never yields a spine or a mid-path arrowhead; an ordered link is a spine at every view. `links --forms` printed whole (paste it) |
| — | **Stop A** | `phase-7.3-stop-a.md`, push, ledger, continue | — |

### Chunk B: tables, shader, picking

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P73-06** | **Route and Segment tables** | §2.7; grow, regrow, rebase for segments; V149 | Tests: a pan and a zoom write 0 rows with links present; the zoom script's rebase writes chart rows only; tables after the script equal regrow; Phase 6, 7.2 row layouts unchanged |
| **P73-07** | **The shader draws links** | the segment pipeline: capsules, knot discs, arrowheads; integer fold and form tests | `cargo xtask gate 6`, `gate 7`, `gate 7.2` each full. `cargo xtask pick` and `regrow` print every earlier line unchanged (paste a `Compare-Object` that prints nothing) |
| **P73-08** | **Picking links** | §2.8; V152; `pick` gains link owners | Whole `pick` output pasted; every line `disagree 0`, and `link owners n (cut allows n)` equal |
| **P73-09** | **Regrow with links** | `regrow` gains `routes equal regrow` | Whole `regrow` output pasted. **Shown then reverted:** recompute routes on a zoom (instead of at grow); paste the `rows` line that fails |
| — | **Stop B** | `phase-7.3-stop-b.md`, push, ledger, continue | — |

### Chunk C: the shell, measurement, gate 7.3, freeze

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P73-10** | **The shell draws links** | §2.9 | The session test (no window) of P72-11 extended: a pick at a known leg pixel prints `link sys_g0s00 member <i>`; every pan/zoom `rows 0`. Cursor runs the window once and pastes one link pick line, or writes `no display` |
| **P73-11** | **The measurement** | `links --measure`, `zoom --measure` | Output pasted; numbers go to the findings |
| **P73-12** | **Findings** | `docs/Findings/phase-7.3-links.md`: every command's output from P73-02 on; *Predictions* marked `as predicted` / `differs`; **The adversary** (§7) answered with the measured µs; **What snapshots would need** (R105) | Every prediction marked |
| **P73-13** | **Gate 7.3** | §6 items; `phase 7.3` after `phase 7.2` in `PHASE_LABELS`; lock row | `cargo xtask gate all` exits 0, prints `phase 7.3: 3/3`. **Shown then reverted**, pasting each failure: (a) give item 2 item 3's `opposes`; (b) draw links over bodies (item 1 must fail on a body pixel owned by a link); (c) draw an arrowhead mid-path on an unordered link (item 2 must fail) |
| **P73-14** | **Docs and freeze** | README, `Guides/03-where-we-are.md` (links drawn; snapshots not built); glossary: *gutter, routing graph, touch point, stub, knot, leg, trunk, fold state, region, hub, bundle, spine, arrowhead*; `decisions.md` V148–V152 with status; roadmap 7.3 note points at the findings | `gate all` from a fresh clone prints phases 0 … 7.3 and exits 0; `corpus verify` 44; `git diff --stat <P73-01>..HEAD -- joinn/corpus` prints nothing |
| — | **Stop C** | `phase-7.3-stop-c.md` with CI, push, ledger, tripwires, continue | — |

Dependencies: in order. P73-06 depends on P73-03; P73-07 on P73-06; P73-08 and P73-09 on P73-07; P73-13 on P73-04, P73-08, P73-09.

**If something has to be cut for time:** cut bundle first (draw it as hub, say so), then `--measure`. Never cut V148, V149, V150, V152.

---

## 5. Test Strategy

As Phase 7.2's §5. Route tests assert exact coordinates of knots and segment counts, never "a route exists". Picture tests compare bytes.

---

## 6. Exit Gate 7.3

Three items in `xtask/src/fns/gate_seven_three_items.rs`, built like gate 7.2's. Every GPU part runs on every adapter (rule 60).

| # | Item | `control_artifact` | `opposes` | Control answers `true` when |
|---|---|---|---|---|
| 1 | **A hyperedge touches; it never crosses** | `corpus/phase5/ordered.universe` | `WireAcross("path")` | the subject is refused at admission, or some route of the subject has a crossing (§2.6) |
| 2 | **Order is drawn only when declared; the form follows size** | `corpus/phase5/ordered.universe` | `FlipMark("path", "calc.sum@2")` | the subject is refused at admission, or an arrowhead is drawn against its declared marks |
| 3 | **A folded system is touched once** | `corpus/phase5/universe.universe` | `DropMember("e0", "units.scale@0")` | the subject is refused at admission, or some link touches a folded node more than once or touches no node of a member |

**Checks:**

1. On `universe.universe`, `ordered.universe` and the grove (seed 7), every fold state: `crossings 0`. On every adapter at every Phase 7.2 §2.12 view: no body or cell pixel's GPU owner is a link.
2. Every grove link is unordered and draws no spine and no mid-path arrowhead at any view; `ordered.universe`'s `path` is a spine at its frame and at level −4; form counts at each view equal `links --forms`; a link in a form fade window is owned by its owner form (Cursor picks one zoom per threshold, printed).
3. At level −4: 264 touches and 136 legs, each folded node touched once per link; at the frame, 1182 stubs; a pick on a leg pixel returns `link <id> member <i>` on every adapter, equal to `cpu_pick`.

Rule 41 holds: the three pairs are new. If the uniqueness test refuses one, Cursor uses the next unused mutation of the same artifact from the closed catalogue, prints it, and records a snag.

### 6.1 Conditions for opening Phase 8

The run plan's tripwires. Claude's review comes after the run.

### 6.2 AJ's window check (end of run, about three minutes)

```
cd D:\JoInn\joinn
cargo xtask grove --out target/grove.universe
cargo run --release -p joinn-shell-desktop -- target/grove.universe
```

1. The whole universe shows. Inside each system, pale wide lanes join some of the blocks: those are links as **regions**.
2. Scroll in on one system. The lanes turn into **thin lines meeting at a knot** (hubs). None goes through a block.
3. Keep scrolling. Where several legs share a lane, it gets thicker (**bundle**).
4. Click a thin line near a block. The terminal prints `link … member …`.
5. Scroll all the way out. Systems fold into tiles; lines between galaxies stop at tile edges, one per tile.
6. Close the window. Tell Claude what worked.

---

## 7. Risks

| Risk | What to do |
|---|---|
| **The adversary: routing cost.** A knot by 1-median is a shortest-path search per member; on large universes it could be slow at grow | Measured in P73-11. Grow-time only, never per tick. A slow number is a finding, not a workaround |
| A body's port needs a stub with a bend and the bend meets a cell | V16 prints it; a snag with the body named. Never move a cell |
| Gate 7.2 changes after the shader change | A truth problem. Paste the pixel line; never rebless |
| Thin hub lines vanish at small zoom | Presentation: the region form covers small sizes. AJ judges at the end |
| A predicted count differs | Paste both; exact counts decide at the review |

---

## 8. Open Items

| ID | Topic | Question |
|---|---|---|
| **R105** | Snapshots | What measurement would make them necessary? Draw-time and instance counts are in `phase-7.3-links.md`. Until something is slow, they wait |
| **R106** | Streets as structure | The gutters come from the grid. If a lens's layout becomes a body of its own (R104), are its gutters part of its contract? |
| **R107** | The knot as a place | A hub's knot is a 1-median on the streets. Does it mean anything (a junction, a bus, a shear tab), or is it presentation only? |

---

## Appendix A · `AGENTS.md` changes

Replace the header paragraph with:

```markdown
# JoInn — standing rules

This repo is JoInn. Phase 7.3 is Visual Host II, part 2: the links. Hyperedges
are routed through the gutters of the grid, drawn as region, hub, bundle or
spine, and touch a folded system once. Its one idea is A LINK TOUCHES; IT NEVER
CROSSES. The build plan is docs/Plans/JoInn Phase 7.3 Implementation Plan.md,
run inside docs/Plans/JoInn Run 7.2B-9.md. Work one CHUNK at a time, one git
commit per numbered step, write each stop report, and continue unless a
tripwire fires. Never stop to ask; follow the plan's Snags section instead.

No new grammar in this phase. Snapshots are not built (R105).
```

Keep every rule. Append:

```markdown
74. A LINK TOUCHES; IT NEVER CROSSES. Route geometry is checked in layout units
    against every cell rectangle and every non-member surface. A crossing is a
    truth violation, never a style problem.
75. ROUTES LIVE IN GUTTERS AND ARE GROWN ONCE. Every fold state's routes are
    computed at grow. A zoom chooses among them in the shader and writes no row.
76. ORDER IS DRAWN ONLY WHEN DECLARED. A spine or a mid-path arrowhead appears
    only on an ordered link; a tail's stub arrow only on a tail.
```

## Appendix B · Documents

- **Backlog** (`docs/Theory/JoInn Research Backlog.md`): after R104, `> **3 Oct 2026 · Phase 7.3.** R105–R107 come from the links plan.` then R105–R107 with status `open` and their questions from §8.
- **`docs/Findings/decisions.md`**: rows R105–R107 (phase `7.3`, `open`) and V148–V152 (`open`; P73-14 sets them).
- **Roadmap**: under `#### Phase 7.3 · Visual Host II: links`, add: `> **3 Oct 2026.** Planned in Plans/JoInn Phase 7.3 Implementation Plan.md: routes in the gutters, one path at three widths, two fold states grown once. Snapshots wait (R105).`

---

*JoInn Phase 7.3 Implementation Plan, Draft 0.1 (3 Oct 2026). Written with the run plan while AJ was away. AJ decided earlier: Visual Host II split into 7.2 and 7.3; exact all the way. Claude decided, open to AJ's veto at the run review: gutter routing; region, hub and bundle as one path at three widths; links under bodies; fold states grown once; snapshots deferred.*
