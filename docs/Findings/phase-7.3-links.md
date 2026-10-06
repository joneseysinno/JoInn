# Phase 7.3 — Links: findings

P73-12. Every link of a universe is routed once, at grow, along the gutters of
the grid layout: stubs from ports to their gutters, a knot at the 1-median of
the touch points on the streets, one leg per touch point, a spine through the
members when the link's order is declared. A route exists for each fold state
the zoom range reaches (for the grove: *open* and *systems folded*), and the
shader picks the fold state and the form (region, hub, bundle, spine) from the
tick uniform and the Route row, in integers; a zoom writes no row. No route
crosses a cell, a non-member body or a folded node (V148: `crossings 0` on
every universe and fold state). The CPU pick names every link pixel as the GPU
does, on every adapter and every Phase 7.2 view (V152). The shell draws links
and names them on a click. Every prediction of §2.2, §2.3, §2.4 and §3 held;
none differs.

Commits: P73-01 `0480723`, P73-01a `5b9f0d5`, P73-S1 `f082035`, P73-S2
`24d782e`, P73-S3 `bf489ad`, P73-S4 `6691c9c`, P73-F1 `27da365`, P73-F2
`d35ad03`, P73-02 `688be90`, P73-03 `641b442`, P73-04 `33ed885`, P73-05
`6537378`, P73-06 `b587bc2`, P73-07 `fa44434`, P73-08 `62247c0`, P73-09
`252e9e0`, P73-10 `7d6b4a3`, P73-11 `fb81be0`. Machine: Windows 10.0.26300,
adapters Microsoft Basic Render Driver (Dx12, Cpu) and NVIDIA GeForce RTX 2080
(Dx12 and Vulkan). The appendix's outputs are from `joinn/` with the console
in UTF-8 (cargo's Compiling, Finished and Running lines dropped): `links` and
`links --forms` re-run at `fb81be0`; `pick` at P73-08, `regrow` at P73-09 and
the session test at P73-10, none of whose code changed after; the measurements
at P73-11.

## What was built, by commit

- **P73-S1 … S4, speed.** The dev profile (opt-level 1, overflow checks and
  debug assertions kept, dependencies at 3), one feature resolution for the
  workspace, xtask and joinn-cli, pure gate work once per process with
  per-phase ms, `gate <phase> --item <n>`, `check` and `stop-check [--fresh]`.
  `gate all` went from **1 082 128 ms** (P73-01a, before S1) to about
  **390 000 ms** on this machine with every phase line unchanged; gate 7.2 is
  about 155 s of it now that pick and regrow carry links.
- **P73-F1, F2, carried from 7.2.** Gate 7.2's controls admit through
  `admit_universe` on `ordered.universe`; the shipped `band()` is probed on
  every adapter at the 240 px boundary (`band boundary … cpu agrees`).
- **P73-02, the routing graph.** Gutter lines of each level and their
  extensions to the parent's lines, split at every crossing and end:
  `graph grove: 6725 nodes, 13028 segments`.
- **P73-03, touch points and routes.** Stubs, one touch per folded node (V151),
  the knot by 1-median with `(y, x)` ties, legs and trunk, spines and
  arrowheads, size; every fold state at once.
- **P73-04, V16 exactly.** §2.6's four checks on every route in every fold
  state; `cargo xtask links` prints `crossings 0` (V148). A gutter moved four
  units into the bodies gave 1305 crossings.
- **P73-05, forms.** The owner rule over a route's size (`10·s ≥ 11·T`, T =
  240 and 1920), spine only when order is declared (V150); `links --forms`.
- **P73-06, tables.** Route (32 bytes) and Segment (48 bytes) rows grown with
  the scene; a segment lives in the deepest chart holding both its ends, so a
  rebase moves it with its chart row (V149).
- **P73-07, the shader.** The segment pipeline (capsules, knot discs,
  arrowheads), its own group 1 of three storage tables (chart, segment,
  route); fold state and form chosen in integers; frames drawn in two passes so
  links lie under nodes and bodies.
- **P73-08, picking links.** `Geom::Arrow` with an exact integer classifier;
  segments in `shapes_at` in draw order with IDs `[link + 1, member, LINK_TAG |
  form, generation]`; the segment owner writes the ID target; `pick` prints
  `link owners n (cut allows n)` (V152).
- **P73-09, regrow with links.** `routes equal regrow` on every grove step;
  recomputing routes on a zoom wrote 4779 rows per zoom and was caught.
- **P73-10, the shell.** No new code: the session test picks a leg of
  `sys_g0s00` and prints `link sys_g0s00 member 3`; the window, once, printed
  `link sys_g0s00 member 4` and the GPU agreed.
- **P73-11, the measurement.** `links --measure`; `zoom --measure` counts
  segments and both frame passes.

## Predictions

### §2.2 The routing graph

| Prediction | Plan | Printed | Mark |
|---|---|---|---|
| Crossings per chart | system 7 × 5 = 35, galaxy 25, universe 15 | `a_system_has_35_crossings_a_galaxy_25_the_universe_15 ... ok` | as predicted |
| Every line misses every slot; every extension ends on a parent line | a test asserts it for every slot | `every_grid_line_misses_every_slot_of_its_level ... ok`, `every_extension_ends_on_a_parent_line ... ok` | as predicted |
| Grove graph | not predicted beyond 35/25/15 | `graph grove: 6725 nodes, 13028 segments` | (printed) |

### §2.3 Touch points

| Prediction | Plan | Printed | Mark |
|---|---|---|---|
| Stubs | for every body in the corpus and the grove the stub is one horizontal segment | open fold: grove 1182 stubs in 1182 stub pieces; `universe.universe` and `ordered.universe` 2 in 2 (counted by a scratch test at P73-12, not committed) | as predicted |
| One touch per folded node (V151) | members in one folded node share one touch point and one leg | `three_members_in_one_folded_system_have_one_touch_point_and_one_leg ... ok`; `links: 3 universes, crossings 0, each folded node touched once` | as predicted |

### §2.4 Routes

| Prediction | Plan | Printed | Mark |
|---|---|---|---|
| Fold states of the grove at levels −4 … 9 | only *open* and *systems folded* | `links grove fold open: …`, `links grove fold systems folded: …`, no third line; `the_phase_5_link_has_a_route_per_fold_state_and_a_segment_per_piece` asserts `[Open, Systems]` | as predicted |
| Knot ties | smallest `y`, then smallest `x` | `three_members_on_a_ring_tie_and_the_smallest_y_then_x_wins ... ok` | as predicted |
| Routes at grow, never at a zoom | — | `regrow`: every grove step `rows 0` at one anchor, `routes equal regrow`; routes recomputed on a zoom: `a pan or zoom at b0000 wrote 4779 row(s)` (P73-09, reverted) | as predicted |

### §3 Counts

| Prediction | Plan | Printed | Mark |
|---|---|---|---|
| Grove, open | 137 links, 1182 legs, 1182 stubs, 0 spines, knots 137 | `links grove fold open: 137 links, 1182 legs, 1182 stubs, 0 spines, knots 137, crossings 0` | as predicted |
| Grove, systems folded | 137 links, 136 legs, 0 stubs, 0 spines, knots 9; touches 264 | `links grove fold systems folded: 137 links, 136 legs, 0 stubs, 0 spines, knots 9, crossings 0`; `every_grove_link_folded_touches_each_node_once` (264 touches) | as predicted |
| `universe.universe` and `ordered.universe`, open | 1 link, 0 legs, 2 stubs, 1 spine, knots 0 | `links universe.universe fold open: 1 links, 0 legs, 2 stubs, 1 spines, knots 0, crossings 0`, and the same for `ordered.universe` | as predicted |
| The same two, systems folded | not predicted | `1 links, 0 legs, 0 stubs, 1 spines, knots 0, crossings 0` each | (printed) |

### §2.5, §2.8, §2.9 (stated, not numbered predictions)

- **Forms.** At exactly `10·s = 11·T` the higher form owns and a notch below
  the lower, for T = 240 and 1920, on the CPU (`form_of` tests) and on every
  adapter (`link_probe`: size 33 hub at level 3 step 0, region at level 2 step
  255; bundle at level 6 step 0, hub at level 5 step 255). No grove link is a
  spine or carries a mid-path arrowhead at any view (V150).
- **Links under bodies.** `pick` on every adapter and view: disagree 0 with the
  CPU drawing bodies after segments; every grove view's non-link owners are
  P73-07's, unchanged.
- **The shell.** A click on a leg prints `link <id> member <i>` (session test,
  and the window with the GPU's `agree`).

## The adversary: routing cost

**Routing runs once per grow, never per tick.** `routes` is called inside
`layout_universe`; the scene writes Route and Segment rows at grow and never
again. A zoom writes 0 rows (every grove step of `regrow`, every zoom of the
session test), and P73-09's demo shows what a per-zoom recompute would cost: all
4779 route and segment rows rewritten on every notch.

Measured with `links --measure` (least of five runs of `routes`: every fold
state's grid, graph, knots, legs, stubs and arrowheads):

| Universe | Links | Routes | Pieces | Dev profile | Release |
|---|---|---|---|---|---|
| `universe.universe` | 1 | 2 | 9 | 278 µs | 178 µs |
| `ordered.universe` | 1 | 2 | 9 | 265 µs | 179 µs |
| grove | 137 | 274 | 4505 | 1 042 556 µs | **633 063 µs** |

So the grove's 137 links cost **0.63 s in release, about 4.6 ms per link**, at
grow. The 1-median is a shortest-path search from every touch point over a
6725-node graph (more with stub ends), and the grove's `gal_` and `uni` links
reach across galaxies; the cost per link grows with the graph, not with the
zoom. It is paid when a universe opens (the shell, every `UniverseScene::grow`
in the checks; `grove_layout` memoizes it per xtask process). It is a finding,
not a defect: nothing per tick depends on it.

What a tick costs did grow, by the rows it draws: **311 634 instances** at every
view (`zoom --measure`), up from 7.2's 302 352: the 4505 segments and a second
pass over the 136 frame rows, each through owner and ghost (2 × 4641 = 9282).
Segments are 2.9 % of a tick's instances; strokes are still 80 %. The CPU cut is
unchanged (22–68 µs in release).

## What snapshots would need (R105)

Nothing measured in this phase asks for snapshots yet: the slow thing (routing,
0.63 s) is grow-time, and a snapshot of pixels would not remove it. What would
make them necessary, and what they would have to carry, as observed:

- **A per-tick budget that the instance count breaks.** Every tick draws every
  row of every table (311 634 instances for 3072 bodies) and discards in the
  shader. A snapshot pays off only when a measured frame time at some view
  exceeds a budget; `frame median` (pick) is the existing timer, and it is
  measured on the calculator, not the grove. Measuring the grove's frame time
  per view is the first step.
- **A key that covers every integer decision the shader makes.** A frame
  depends on the anchor, the zoom's level and step, the fold state, every
  body's band and fade window, and now every link's form and form window. A
  cached picture is valid only while all of those are unchanged, i.e. for a
  pan at one zoom, never across a notch; within a fade window every step
  changes alpha.
- **The ID target with the colour.** Two pickers must name the same owner at
  every pixel (V145, V152). A snapshot of colour alone would leave the GPU pick
  without IDs; it would have to cache the ID image too, or the confirm line
  would have nothing to agree with.
- **Invalidation by row writes.** Any table write (a live value, a rebase's
  3209 chart rows) would invalidate every snapshot drawn from that row; the
  rebase writes every chart, so a rebase invalidates everything.
- **Routes are already a snapshot of the right kind:** grown once per fold
  state and stored in rows, chosen per tick by integers. If text (80 % of the
  rows) were treated the same way, per band instead of per fold, the instance
  count would fall without caching a pixel.

## What the next phase needs (observed)

- Printing and parsing the grove does not give back the generated universe: the
  first difference is link order (`sys_g0s00` first in memory, `gal_g0` first in
  the file), so a leg's member number follows the file (P73-10: the session
  test, from memory, `member 3`; the window, from `target/grove.universe`,
  `member 4` at its own pixel, equal to a replay from that file).
- At the universe frame `ordered.universe`'s spine is 0.17 px half-width and
  owns no pixel centre; `pick`'s ordered line frames its galaxy instead
  (3 link owners: the spine and two member stubs).
- The window received only the input Cursor posted this time (36 ticks for 36
  notches), with the system cursor outside the window and never moved.
- The 1-median per link is the largest cost at grow; a universe ten times the
  grove's links would take seconds to open. Routing per link is independent,
  so it can be measured per link before anything is changed.

## Appendix: command outputs

### `cargo xtask links` (at `fb81be0`)

```
links universe.universe fold open: 1 links, 0 legs, 2 stubs, 1 spines, knots 0, crossings 0
links universe.universe fold systems folded: 1 links, 0 legs, 0 stubs, 1 spines, knots 0, crossings 0
links ordered.universe fold open: 1 links, 0 legs, 2 stubs, 1 spines, knots 0, crossings 0
links ordered.universe fold systems folded: 1 links, 0 legs, 0 stubs, 1 spines, knots 0, crossings 0
links grove fold open: 137 links, 1182 legs, 1182 stubs, 0 spines, knots 137, crossings 0
links grove fold systems folded: 137 links, 136 legs, 0 stubs, 0 spines, knots 9, crossings 0
graph grove: 6725 nodes, 13028 segments
links: 3 universes, crossings 0, each folded node touched once
```

### `cargo xtask links --forms` (at `fb81be0`)

```
forms frame level -2 step 95 (k 351/1024) (open): region 128, hub 9, bundle 0, spine 0, fading 0
forms at s level -4 step 0 (k 1/16) (systems folded): region 9, hub 0, bundle 0, spine 0, fading 0
forms at s level -3 step 0 (k 1/8) (open): region 136, hub 1, bundle 0, spine 0, fading 0
forms at s level -2 step 0 (k 1/4) (open): region 136, hub 1, bundle 0, spine 0, fading 8
forms at s level -1 step 0 (k 1/2) (open): region 128, hub 9, bundle 0, spine 0, fading 1
forms at s level 0 step 0 (k 1) (open): region 127, hub 9, bundle 1, spine 0, fading 89
forms at s level 1 step 0 (k 2) (open): region 2, hub 134, bundle 1, spine 0, fading 8
forms at s level 2 step 0 (k 4) (open): region 0, hub 128, bundle 9, spine 0, fading 0
forms at s level 3 step 0 (k 8) (open): region 0, hub 127, bundle 10, spine 0, fading 89
forms at s level 4 step 0 (k 16) (open): region 0, hub 2, bundle 135, spine 0, fading 0
forms at s level 5 step 0 (k 32) (open): region 0, hub 0, bundle 137, spine 0, fading 0
forms at s level 6 step 0 (k 64) (open): region 0, hub 0, bundle 137, spine 0, fading 0
forms at s level 7 step 0 (k 128) (open): region 0, hub 0, bundle 137, spine 0, fading 0
forms at s level 8 step 0 (k 256) (open): region 0, hub 0, bundle 137, spine 0, fading 0
forms at s level 9 step 0 (k 512) (open): region 0, hub 0, bundle 137, spine 0, fading 0
```

### `cargo xtask links --measure` (dev profile, P73-11)

```
measure universe.universe: route µs 278 (1 links, 2 routes, 9 pieces; per link 278 µs)
measure ordered.universe: route µs 265 (1 links, 2 routes, 9 pieces; per link 265 µs)
measure grove: route µs 1042556 (137 links, 274 routes, 4505 pieces; per link 7609 µs)
links --measure: 3 universes, least of 5 runs (information, not a check)
```

### `cargo run --release -p xtask -- links --measure` (P73-11)

```
measure universe.universe: route µs 178 (1 links, 2 routes, 9 pieces; per link 178 µs)
measure ordered.universe: route µs 179 (1 links, 2 routes, 9 pieces; per link 179 µs)
measure grove: route µs 633063 (137 links, 274 routes, 4505 pieces; per link 4620 µs)
links --measure: 3 universes, least of 5 runs (information, not a check)
```

### `cargo xtask zoom --measure` (dev profile, P73-11)

```
measure frame level -2 step 95 (k 351/1024): cut cpu 126 µs (3208 entries), draw instances 311634
measure at s level -4 step 0 (k 1/16): cut cpu 70 µs (136 entries), draw instances 311634
measure at s level -3 step 0 (k 1/8): cut cpu 130 µs (3208 entries), draw instances 311634
measure at s level -2 step 0 (k 1/4): cut cpu 113 µs (2342 entries), draw instances 311634
measure at s level -1 step 0 (k 1/2): cut cpu 89 µs (904 entries), draw instances 311634
measure at s level 0 step 0 (k 1): cut cpu 86 µs (283 entries), draw instances 311634
measure at s level 1 step 0 (k 2): cut cpu 76 µs (85 entries), draw instances 311634
measure at s level 2 step 0 (k 4): cut cpu 74 µs (26 entries), draw instances 311634
measure at s level 3 step 0 (k 8): cut cpu 81 µs (14 entries), draw instances 311634
measure at s level 4 step 0 (k 16): cut cpu 75 µs (6 entries), draw instances 311634
measure at s level 5 step 0 (k 32): cut cpu 75 µs (4 entries), draw instances 311634
measure at s level 6 step 0 (k 64): cut cpu 75 µs (3 entries), draw instances 311634
measure at s level 7 step 0 (k 128): cut cpu 75 µs (3 entries), draw instances 311634
measure at s level 8 step 0 (k 256): cut cpu 74 µs (3 entries), draw instances 311634
measure at s level 9 step 0 (k 512): cut cpu 75 µs (3 entries), draw instances 311634
zoom --measure: 15 views; tables frame 136, body 3072, cell 5434, link 2362, port 13031, stroke 124069, route 274, segment 4505
```

### `cargo run --release -p xtask -- zoom --measure` (P73-11)

```
measure frame level -2 step 95 (k 351/1024): cut cpu 62 µs (3208 entries), draw instances 311634
measure at s level -4 step 0 (k 1/16): cut cpu 24 µs (136 entries), draw instances 311634
measure at s level -3 step 0 (k 1/8): cut cpu 68 µs (3208 entries), draw instances 311634
measure at s level -2 step 0 (k 1/4): cut cpu 52 µs (2342 entries), draw instances 311634
measure at s level -1 step 0 (k 1/2): cut cpu 34 µs (904 entries), draw instances 311634
measure at s level 0 step 0 (k 1): cut cpu 33 µs (283 entries), draw instances 311634
measure at s level 1 step 0 (k 2): cut cpu 24 µs (85 entries), draw instances 311634
measure at s level 2 step 0 (k 4): cut cpu 22 µs (26 entries), draw instances 311634
measure at s level 3 step 0 (k 8): cut cpu 29 µs (14 entries), draw instances 311634
measure at s level 4 step 0 (k 16): cut cpu 22 µs (6 entries), draw instances 311634
measure at s level 5 step 0 (k 32): cut cpu 22 µs (4 entries), draw instances 311634
measure at s level 6 step 0 (k 64): cut cpu 22 µs (3 entries), draw instances 311634
measure at s level 7 step 0 (k 128): cut cpu 22 µs (3 entries), draw instances 311634
measure at s level 8 step 0 (k 256): cut cpu 22 µs (3 entries), draw instances 311634
measure at s level 9 step 0 (k 512): cut cpu 22 µs (3 entries), draw instances 311634
zoom --measure: 15 views; tables frame 136, body 3072, cell 5434, link 2362, port 13031, stroke 124069, route 274, segment 4505
```

### `cargo test -p xtask a_click_on_a_leg -- --nocapture` (P73-10)

```
tick: level -2 step 95 (k 351/1024), anchor universe, rows 0
tick: level -2 step 95 (k 351/1024), anchor g0s00, rows 3209
tick: level -2 step 127 (k 383/1024), anchor g0s00, rows 0
tick: level -2 step 159 (k 415/1024), anchor g0s00, rows 0
tick: level -2 step 191 (k 447/1024), anchor g0s00, rows 0
tick: level -2 step 223 (k 479/1024), anchor g0s00, rows 0
tick: level -2 step 255 (k 511/1024), anchor g0s00, rows 0
tick: level -1 step 31 (k 287/512), anchor g0s00, rows 0
tick: level -1 step 63 (k 319/512), anchor g0s00, rows 0
tick: level -1 step 95 (k 351/512), anchor g0s00, rows 0
tick: level -1 step 127 (k 383/512), anchor g0s00, rows 0
tick: level -1 step 159 (k 415/512), anchor g0s00, rows 0
tick: level -1 step 191 (k 447/512), anchor g0s00, rows 0
tick: level -1 step 223 (k 479/512), anchor g0s00, rows 0
tick: level -1 step 255 (k 511/512), anchor g0s00, rows 0
tick: level 0 step 31 (k 287/256), anchor g0s00, rows 0
tick: level 0 step 63 (k 319/256), anchor g0s00, rows 0
tick: level 0 step 95 (k 351/256), anchor g0s00, rows 0
tick: level 0 step 127 (k 383/256), anchor g0s00, rows 0
tick: level 0 step 159 (k 415/256), anchor g0s00, rows 0
tick: level 0 step 191 (k 447/256), anchor g0s00, rows 0
tick: level 0 step 223 (k 479/256), anchor g0s00, rows 0
tick: level 0 step 255 (k 511/256), anchor g0s00, rows 0
tick: level 1 step 31 (k 287/128), anchor g0s00, rows 0
tick: level 1 step 63 (k 319/128), anchor g0s00, rows 0
tick: level 1 step 95 (k 351/128), anchor g0s00, rows 0
tick: level 1 step 127 (k 383/128), anchor g0s00, rows 0
tick: level 1 step 159 (k 415/128), anchor g0s00, rows 0
tick: level 1 step 191 (k 447/128), anchor g0s00, rows 0
tick: level 1 step 223 (k 479/128), anchor g0s00, rows 0
tick: level 1 step 255 (k 511/128), anchor g0s00, rows 0
tick: level 2 step 31 (k 287/64), anchor g0s00, rows 0
tick: level 2 step 63 (k 319/64), anchor g0s00, rows 0
tick: level 2 step 95 (k 351/64), anchor g0s00, rows 0
tick: level 2 step 127 (k 383/64), anchor g0s00, rows 0
tick: level 2 step 159 (k 415/64), anchor g0s00, rows 0
tick: level 2 step 191 (k 447/64), anchor g0s00, rows 0
tick: level 2 step 223 (k 479/64), anchor g0s00, rows 0
tick: level 2 step 223 (k 479/64), anchor g0s00, rows 0
pick 976,556: link sys_g0s00 member 3 (cpu)
```

### The window, once (P73-10): `joinn-desktop.exe target/grove.universe`, input posted to it

```
adapter: NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes
surface: Bgra8Unorm
tick: level -3 step 208 (k 29/128), anchor universe, rows 160831
tick: level -3 step 240 (k 31/128), anchor g1, rows 3209
tick: level -2 step 16 (k 17/64), anchor g1, rows 0
tick: level -2 step 48 (k 19/64), anchor g1s14, rows 3209
tick: level -2 step 80 (k 21/64), anchor g1s13, rows 3209
tick: level -2 step 112 (k 23/64), anchor g1s09, rows 3209
tick: level -2 step 144 (k 25/64), anchor g1s08, rows 3209
tick: level -2 step 176 (k 27/64), anchor g1s08, rows 0
tick: level -2 step 208 (k 29/64), anchor g1s08, rows 0
tick: level -2 step 240 (k 31/64), anchor b0576, rows 3209
tick: level -1 step 16 (k 17/32), anchor g0, rows 3209
tick: level -1 step 48 (k 19/32), anchor g0s07, rows 3209
tick: level -1 step 80 (k 21/32), anchor b0188, rows 3209
tick: level -1 step 112 (k 23/32), anchor g0s07, rows 3209
tick: level -1 step 144 (k 25/32), anchor g0, rows 3209
tick: level -1 step 176 (k 27/32), anchor g0s06, rows 3209
tick: level -1 step 208 (k 29/32), anchor g0s06, rows 0
tick: level -1 step 240 (k 31/32), anchor b0153, rows 3209
tick: level 0 step 16 (k 17/16), anchor b0146, rows 3209
tick: level 0 step 48 (k 19/16), anchor b0145, rows 3209
tick: level 0 step 80 (k 21/16), anchor g0s06, rows 3209
tick: level 0 step 112 (k 23/16), anchor g0s05, rows 3209
tick: level 0 step 144 (k 25/16), anchor g0, rows 3209
tick: level 0 step 176 (k 27/16), anchor g0, rows 0
tick: level 0 step 208 (k 29/16), anchor g0s01, rows 3209
tick: level 0 step 240 (k 31/16), anchor g0s01, rows 0
tick: level 1 step 16 (k 17/8), anchor g0s01, rows 0
tick: level 1 step 48 (k 19/8), anchor b0045, rows 3209
tick: level 1 step 80 (k 21/8), anchor b0044, rows 3209
tick: level 1 step 112 (k 23/8), anchor b0044, rows 0
tick: level 1 step 144 (k 25/8), anchor g0s01, rows 3209
tick: level 1 step 176 (k 27/8), anchor g0s01, rows 0
tick: level 1 step 208 (k 29/8), anchor b0043, rows 3209
tick: level 1 step 240 (k 31/8), anchor g0s01, rows 3209
tick: level 2 step 16 (k 17/4), anchor g0s01, rows 0
tick: level 2 step 48 (k 19/4), anchor g0s01, rows 0
tick: level 2 step 80 (k 21/4), anchor g0s01, rows 0
pick 110,233: link sys_g0s00 member 4 (cpu)
tick: level 2 step 80 (k 21/4), anchor g0s01, rows 0
        link sys_g0s00 member 4 (gpu) · agree
```

### `cargo xtask pick` (P73-08)

```
adapter Microsoft Basic Render Driver · Dx12 · Cpu · vertex storage yes
calculator 640x360: agree 230216, edge 184, disagree 0, owners 13/13
calculator 1000x777: agree 776768, edge 232, disagree 0, owners 13/13
calculator 1280x720: agree 921288, edge 312, disagree 0, owners 13/13
calculator 1920x1080: agree 2073144, edge 456, disagree 0, owners 13/13
calculator.contact 640x360: agree 230248, edge 152, disagree 0, owners 7/7, links 0
calculator.contact 1000x777: agree 776784, edge 216, disagree 0, owners 7/7, links 0
calculator.contact 1280x720: agree 921256, edge 344, disagree 0, owners 7/7, links 0
calculator.contact 1920x1080: agree 2073164, edge 436, disagree 0, owners 7/7, links 0
frame median 2.697 ms (calculator 1280x720; information, not a check)
phase2/calculator.body: agree 921288, edge 312, disagree 0
phase2/variants/calculator_b.body: agree 921288, edge 312, disagree 0
phase2/variants/calculator_c.body: agree 921288, edge 312, disagree 0
phase2/variants/calculator_d.body: agree 921288, edge 312, disagree 0
phase21/int_add_ref.body: not measured: layout: wire pickz@0 -> self@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/int_format_ref.body: not measured: layout: wire mkchr@0 -> self@1 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/int_mul_ref.body: not measured: layout: wire differ@0 -> picks@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/rat_add_ref.body: not measured: layout: wire self@0 -> self@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/text_parse_ref.body: not measured: layout: wire c@1 -> self@1 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/int_format_ref.body: not measured: layout: wire a48@2 -> d45@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/int_mul_ref.body: not measured: layout: wire differ@0 -> picks@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/rat_add_ref.body: not measured: layout: wire pick@0 -> self@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/text_parse_ref.body: not measured: layout: wire c45@0 -> isdash@1 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase3/columns_reader.body: not measured: scene: reader@0 names an in-port and an out-port; acceptance is a body whose cells give each position one port, since an ID names a port by position
phase3/environment.body: not measured: scene: columns@0 names an in-port and an out-port; acceptance is a body whose cells give each position one port, since an ID names a port by position
phase4/fmt.body: agree 921440, edge 160, disagree 0
phase4/fmt_twin.body: agree 921440, edge 160, disagree 0
phase5/bus.body: agree 921440, edge 160, disagree 0
phase5/controls/echo.body: agree 921440, edge 160, disagree 0
phase5/two_in_ports.body: agree 921412, edge 188, disagree 0
phase5/units.body: agree 921412, edge 188, disagree 0
phase52/adversary/asker.body: agree 921432, edge 168, disagree 0
phase52/adversary/lookup.body: agree 921432, edge 168, disagree 0
phase52/controls/missing_cell.body: not measured: instance orphan cell 4a37 was not supplied; acceptance is that cell in the cell map
grove frame: sample 4096 agree 4026, edge 70, disagree 0, owners 3208 (cut allows 3208), link owners 1047 (cut allows 1047)
grove s level -4 step 0: sample 4096 agree 4093, edge 3, disagree 0, owners 136 (cut allows 136), link owners 60 (cut allows 60)
grove s level -3 step 0: sample 4096 agree 4083, edge 13, disagree 0, owners 3208 (cut allows 3208), link owners 668 (cut allows 668)
grove s level -2 step 0: sample 4096 agree 4084, edge 12, disagree 0, owners 2342 (cut allows 2342), link owners 705 (cut allows 705)
grove s level -1 step 0: sample 4096 agree 4084, edge 12, disagree 0, owners 904 (cut allows 904), link owners 287 (cut allows 287)
grove s level 0 step 0: sample 4096 agree 4091, edge 5, disagree 0, owners 939 (cut allows 939), link owners 103 (cut allows 103)
grove s level 1 step 0: sample 4096 agree 4068, edge 28, disagree 0, owners 452 (cut allows 452), link owners 36 (cut allows 36)
grove s level 2 step 0: sample 4096 agree 4078, edge 18, disagree 0, owners 141 (cut allows 141), link owners 11 (cut allows 11)
grove s level 3 step 0: sample 4096 agree 4092, edge 4, disagree 0, owners 99 (cut allows 99), link owners 6 (cut allows 6)
grove s level 4 step 0: sample 4096 agree 4096, edge 0, disagree 0, owners 44 (cut allows 44), link owners 0 (cut allows 0)
grove s level 5 step 0: sample 4096 agree 4094, edge 2, disagree 0, owners 19 (cut allows 19), link owners 0 (cut allows 0)
grove s level 6 step 0: sample 4096 agree 4096, edge 0, disagree 0, owners 10 (cut allows 10), link owners 0 (cut allows 0)
grove s level 7 step 0: sample 4096 agree 4095, edge 1, disagree 0, owners 4 (cut allows 4), link owners 0 (cut allows 0)
grove s level 8 step 0: sample 4096 agree 4096, edge 0, disagree 0, owners 4 (cut allows 4), link owners 0 (cut allows 0)
grove s level 9 step 0: sample 4096 agree 4094, edge 2, disagree 0, owners 3 (cut allows 3), link owners 0 (cut allows 0)
grove fade 4: b0003 level -3 step 195: sample 4096 agree 4065, edge 31, disagree 0, owners 2712 (cut allows 2712), link owners 815 (cut allows 815)
grove fade 32: b0000 level -1 step 195: sample 4096 agree 4073, edge 23, disagree 0, owners 1239 (cut allows 1239), link owners 136 (cut allows 136)
grove fade 240: b0000 level 2 step 167: sample 4096 agree 4094, edge 2, disagree 0, owners 120 (cut allows 120), link owners 7 (cut allows 7)
ordered galaxy app: sample 4096 agree 4086, edge 10, disagree 0, owners 11 (cut allows 11), link owners 3 (cut allows 3)
band boundary Microsoft Basic Render Driver · Dx12 · Cpu · vertex storage yes: 48 at level 2 step 96 full, step 95 summary; 33 at level 3 step 0 full, level 2 step 255 summary; cpu agrees
adapter NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes
calculator 640x360: agree 230216, edge 184, disagree 0, owners 13/13
calculator 1000x777: agree 776768, edge 232, disagree 0, owners 13/13
calculator 1280x720: agree 921288, edge 312, disagree 0, owners 13/13
calculator 1920x1080: agree 2073144, edge 456, disagree 0, owners 13/13
calculator.contact 640x360: agree 230248, edge 152, disagree 0, owners 7/7, links 0
calculator.contact 1000x777: agree 776784, edge 216, disagree 0, owners 7/7, links 0
calculator.contact 1280x720: agree 921256, edge 344, disagree 0, owners 7/7, links 0
calculator.contact 1920x1080: agree 2073164, edge 436, disagree 0, owners 7/7, links 0
frame median 1.824 ms (calculator 1280x720; information, not a check)
phase2/calculator.body: agree 921288, edge 312, disagree 0
phase2/variants/calculator_b.body: agree 921288, edge 312, disagree 0
phase2/variants/calculator_c.body: agree 921288, edge 312, disagree 0
phase2/variants/calculator_d.body: agree 921288, edge 312, disagree 0
phase21/int_add_ref.body: not measured: layout: wire pickz@0 -> self@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/int_format_ref.body: not measured: layout: wire mkchr@0 -> self@1 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/int_mul_ref.body: not measured: layout: wire differ@0 -> picks@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/rat_add_ref.body: not measured: layout: wire self@0 -> self@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/text_parse_ref.body: not measured: layout: wire c@1 -> self@1 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/int_format_ref.body: not measured: layout: wire a48@2 -> d45@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/int_mul_ref.body: not measured: layout: wire differ@0 -> picks@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/rat_add_ref.body: not measured: layout: wire pick@0 -> self@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/text_parse_ref.body: not measured: layout: wire c45@0 -> isdash@1 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase3/columns_reader.body: not measured: scene: reader@0 names an in-port and an out-port; acceptance is a body whose cells give each position one port, since an ID names a port by position
phase3/environment.body: not measured: scene: columns@0 names an in-port and an out-port; acceptance is a body whose cells give each position one port, since an ID names a port by position
phase4/fmt.body: agree 921440, edge 160, disagree 0
phase4/fmt_twin.body: agree 921440, edge 160, disagree 0
phase5/bus.body: agree 921440, edge 160, disagree 0
phase5/controls/echo.body: agree 921440, edge 160, disagree 0
phase5/two_in_ports.body: agree 921412, edge 188, disagree 0
phase5/units.body: agree 921412, edge 188, disagree 0
phase52/adversary/asker.body: agree 921432, edge 168, disagree 0
phase52/adversary/lookup.body: agree 921432, edge 168, disagree 0
phase52/controls/missing_cell.body: not measured: instance orphan cell 4a37 was not supplied; acceptance is that cell in the cell map
grove frame: sample 4096 agree 4026, edge 70, disagree 0, owners 3208 (cut allows 3208), link owners 1047 (cut allows 1047)
grove s level -4 step 0: sample 4096 agree 4093, edge 3, disagree 0, owners 136 (cut allows 136), link owners 60 (cut allows 60)
grove s level -3 step 0: sample 4096 agree 4083, edge 13, disagree 0, owners 3208 (cut allows 3208), link owners 668 (cut allows 668)
grove s level -2 step 0: sample 4096 agree 4084, edge 12, disagree 0, owners 2342 (cut allows 2342), link owners 705 (cut allows 705)
grove s level -1 step 0: sample 4096 agree 4084, edge 12, disagree 0, owners 904 (cut allows 904), link owners 287 (cut allows 287)
grove s level 0 step 0: sample 4096 agree 4091, edge 5, disagree 0, owners 939 (cut allows 939), link owners 103 (cut allows 103)
grove s level 1 step 0: sample 4096 agree 4068, edge 28, disagree 0, owners 452 (cut allows 452), link owners 36 (cut allows 36)
grove s level 2 step 0: sample 4096 agree 4078, edge 18, disagree 0, owners 141 (cut allows 141), link owners 11 (cut allows 11)
grove s level 3 step 0: sample 4096 agree 4092, edge 4, disagree 0, owners 99 (cut allows 99), link owners 6 (cut allows 6)
grove s level 4 step 0: sample 4096 agree 4096, edge 0, disagree 0, owners 44 (cut allows 44), link owners 0 (cut allows 0)
grove s level 5 step 0: sample 4096 agree 4094, edge 2, disagree 0, owners 19 (cut allows 19), link owners 0 (cut allows 0)
grove s level 6 step 0: sample 4096 agree 4096, edge 0, disagree 0, owners 10 (cut allows 10), link owners 0 (cut allows 0)
grove s level 7 step 0: sample 4096 agree 4095, edge 1, disagree 0, owners 4 (cut allows 4), link owners 0 (cut allows 0)
grove s level 8 step 0: sample 4096 agree 4096, edge 0, disagree 0, owners 4 (cut allows 4), link owners 0 (cut allows 0)
grove s level 9 step 0: sample 4096 agree 4094, edge 2, disagree 0, owners 3 (cut allows 3), link owners 0 (cut allows 0)
grove fade 4: b0003 level -3 step 195: sample 4096 agree 4065, edge 31, disagree 0, owners 2712 (cut allows 2712), link owners 815 (cut allows 815)
grove fade 32: b0000 level -1 step 195: sample 4096 agree 4073, edge 23, disagree 0, owners 1239 (cut allows 1239), link owners 136 (cut allows 136)
grove fade 240: b0000 level 2 step 167: sample 4096 agree 4094, edge 2, disagree 0, owners 120 (cut allows 120), link owners 7 (cut allows 7)
ordered galaxy app: sample 4096 agree 4086, edge 10, disagree 0, owners 11 (cut allows 11), link owners 3 (cut allows 3)
band boundary NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes: 48 at level 2 step 96 full, step 95 summary; 33 at level 3 step 0 full, level 2 step 255 summary; cpu agrees
adapter NVIDIA GeForce RTX 2080 · Vulkan · DiscreteGpu · vertex storage yes
calculator 640x360: agree 230216, edge 184, disagree 0, owners 13/13
calculator 1000x777: agree 776768, edge 232, disagree 0, owners 13/13
calculator 1280x720: agree 921288, edge 312, disagree 0, owners 13/13
calculator 1920x1080: agree 2073144, edge 456, disagree 0, owners 13/13
calculator.contact 640x360: agree 230248, edge 152, disagree 0, owners 7/7, links 0
calculator.contact 1000x777: agree 776784, edge 216, disagree 0, owners 7/7, links 0
calculator.contact 1280x720: agree 921256, edge 344, disagree 0, owners 7/7, links 0
calculator.contact 1920x1080: agree 2073164, edge 436, disagree 0, owners 7/7, links 0
frame median 0.683 ms (calculator 1280x720; information, not a check)
phase2/calculator.body: agree 921288, edge 312, disagree 0
phase2/variants/calculator_b.body: agree 921288, edge 312, disagree 0
phase2/variants/calculator_c.body: agree 921288, edge 312, disagree 0
phase2/variants/calculator_d.body: agree 921288, edge 312, disagree 0
phase21/int_add_ref.body: not measured: layout: wire pickz@0 -> self@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/int_format_ref.body: not measured: layout: wire mkchr@0 -> self@1 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/int_mul_ref.body: not measured: layout: wire differ@0 -> picks@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/rat_add_ref.body: not measured: layout: wire self@0 -> self@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/text_parse_ref.body: not measured: layout: wire c@1 -> self@1 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/int_format_ref.body: not measured: layout: wire a48@2 -> d45@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/int_mul_ref.body: not measured: layout: wire differ@0 -> picks@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/rat_add_ref.body: not measured: layout: wire pick@0 -> self@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/text_parse_ref.body: not measured: layout: wire c45@0 -> isdash@1 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase3/columns_reader.body: not measured: scene: reader@0 names an in-port and an out-port; acceptance is a body whose cells give each position one port, since an ID names a port by position
phase3/environment.body: not measured: scene: columns@0 names an in-port and an out-port; acceptance is a body whose cells give each position one port, since an ID names a port by position
phase4/fmt.body: agree 921440, edge 160, disagree 0
phase4/fmt_twin.body: agree 921440, edge 160, disagree 0
phase5/bus.body: agree 921440, edge 160, disagree 0
phase5/controls/echo.body: agree 921440, edge 160, disagree 0
phase5/two_in_ports.body: agree 921412, edge 188, disagree 0
phase5/units.body: agree 921412, edge 188, disagree 0
phase52/adversary/asker.body: agree 921432, edge 168, disagree 0
phase52/adversary/lookup.body: agree 921432, edge 168, disagree 0
phase52/controls/missing_cell.body: not measured: instance orphan cell 4a37 was not supplied; acceptance is that cell in the cell map
grove frame: sample 4096 agree 4026, edge 70, disagree 0, owners 3208 (cut allows 3208), link owners 1047 (cut allows 1047)
grove s level -4 step 0: sample 4096 agree 4093, edge 3, disagree 0, owners 136 (cut allows 136), link owners 60 (cut allows 60)
grove s level -3 step 0: sample 4096 agree 4083, edge 13, disagree 0, owners 3208 (cut allows 3208), link owners 668 (cut allows 668)
grove s level -2 step 0: sample 4096 agree 4084, edge 12, disagree 0, owners 2342 (cut allows 2342), link owners 705 (cut allows 705)
grove s level -1 step 0: sample 4096 agree 4084, edge 12, disagree 0, owners 904 (cut allows 904), link owners 287 (cut allows 287)
grove s level 0 step 0: sample 4096 agree 4091, edge 5, disagree 0, owners 939 (cut allows 939), link owners 103 (cut allows 103)
grove s level 1 step 0: sample 4096 agree 4068, edge 28, disagree 0, owners 452 (cut allows 452), link owners 36 (cut allows 36)
grove s level 2 step 0: sample 4096 agree 4078, edge 18, disagree 0, owners 141 (cut allows 141), link owners 11 (cut allows 11)
grove s level 3 step 0: sample 4096 agree 4092, edge 4, disagree 0, owners 99 (cut allows 99), link owners 6 (cut allows 6)
grove s level 4 step 0: sample 4096 agree 4096, edge 0, disagree 0, owners 44 (cut allows 44), link owners 0 (cut allows 0)
grove s level 5 step 0: sample 4096 agree 4094, edge 2, disagree 0, owners 19 (cut allows 19), link owners 0 (cut allows 0)
grove s level 6 step 0: sample 4096 agree 4096, edge 0, disagree 0, owners 10 (cut allows 10), link owners 0 (cut allows 0)
grove s level 7 step 0: sample 4096 agree 4095, edge 1, disagree 0, owners 4 (cut allows 4), link owners 0 (cut allows 0)
grove s level 8 step 0: sample 4096 agree 4096, edge 0, disagree 0, owners 4 (cut allows 4), link owners 0 (cut allows 0)
grove s level 9 step 0: sample 4096 agree 4094, edge 2, disagree 0, owners 3 (cut allows 3), link owners 0 (cut allows 0)
grove fade 4: b0003 level -3 step 195: sample 4096 agree 4065, edge 31, disagree 0, owners 2712 (cut allows 2712), link owners 815 (cut allows 815)
grove fade 32: b0000 level -1 step 195: sample 4096 agree 4073, edge 23, disagree 0, owners 1239 (cut allows 1239), link owners 136 (cut allows 136)
grove fade 240: b0000 level 2 step 167: sample 4096 agree 4094, edge 2, disagree 0, owners 120 (cut allows 120), link owners 7 (cut allows 7)
ordered galaxy app: sample 4096 agree 4086, edge 10, disagree 0, owners 11 (cut allows 11), link owners 3 (cut allows 3)
band boundary NVIDIA GeForce RTX 2080 · Vulkan · DiscreteGpu · vertex storage yes: 48 at level 2 step 96 full, step 95 summary; 33 at level 3 step 0 full, level 2 step 255 summary; cpu agrees
planted: port sum@1 moved one unit, cpu_pick against the GPU on Microsoft Basic Render Driver · Dx12 · Cpu · vertex storage yes: disagree 7336; refused as truth violation (ok)
pick: 3 adapter(s), 12 subject(s) agree; planted disagreement: refused as truth violation (ok)
```

### `cargo xtask regrow` (P73-09)

```
event 1 cli_a@0 "two": touched cli_a, rows 2 (bound 3), bytes 80, tables equal regrow
event 2 cli_a@0 "2": touched cli_a, sum, rows 3 (bound 7), bytes 112, tables equal regrow
event 3 cli_b@0 "3": touched cli_b, sum, rows 4 (bound 7), bytes 128, tables equal regrow
fresh calculator (Amendment A4):
event 4 cli_b@0 "x": touched cli_b, rows 2 (bound 3), bytes 80, tables equal regrow
event 5 cli_a@0 "2": touched cli_a, sum, rows 4 (bound 7 + 1), bytes 144, tables equal regrow
camera change: rows 0
idle tick: nothing to draw
regrow Microsoft Basic Render Driver · Dx12 · Cpu · vertex storage yes: color identical, ids identical
contact event 1 cli_a@0 "two": touched cli_a, rows 2 (bound 2), bytes 80, tables equal regrow
contact event 2 cli_a@0 "2": touched cli_a, sum, rows 1 (bound 4), bytes 48, tables equal regrow
contact event 3 cli_b@0 "3": touched cli_b, sum, rows 3 (bound 4), bytes 112, tables equal regrow
fresh contact calculator (Amendment A4):
contact event 4 cli_b@0 "x": touched cli_b, rows 2 (bound 2), bytes 80, tables equal regrow
contact event 5 cli_a@0 "2": touched cli_a, sum, rows 2 (bound 4 + 1), bytes 80, tables equal regrow
regrow contact Microsoft Basic Render Driver · Dx12 · Cpu · vertex storage yes: color identical, ids identical
regrow NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes: color identical, ids identical
regrow contact NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes: color identical, ids identical
regrow NVIDIA GeForce RTX 2080 · Vulkan · DiscreteGpu · vertex storage yes: color identical, ids identical
regrow contact NVIDIA GeForce RTX 2080 · Vulkan · DiscreteGpu · vertex storage yes: color identical, ids identical
grove step 0 frame: rows 0, universe, tables equal regrow, routes equal regrow
grove step 1 focus s: rows 3209, b0000, tables equal regrow, routes equal regrow
grove step 2 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 3 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 4 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 5 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 6 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 7 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 8 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 9 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 10 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 11 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 12 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 13 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 14 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 15 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 16 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 17 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 18 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 19 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 20 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 21 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 22 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 23 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 24 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 25 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 26 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 27 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 28 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 29 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 30 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 31 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 32 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 33 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 34 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 35 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 36 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 37 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 38 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 39 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 40 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 41 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 42 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 43 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 44 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 45 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 46 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 47 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 48 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 49 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 50 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 51 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 52 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 53 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 54 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 55 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 56 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 57 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 58 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 59 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 60 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 61 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 62 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 63 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 64 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 65 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 66 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 67 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 68 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 69 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 70 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 71 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 72 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 73 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 74 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 75 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 76 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 77 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 78 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 79 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 80 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 81 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 82 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 83 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 84 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 85 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 86 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 87 in: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 88 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 89 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 90 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 91 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 92 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 93 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 94 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 95 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 96 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 97 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 98 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 99 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 100 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 101 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 102 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 103 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 104 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 105 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 106 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 107 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 108 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 109 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 110 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 111 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 112 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 113 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 114 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 115 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 116 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 117 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 118 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 119 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 120 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 121 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 122 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 123 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 124 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 125 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 126 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 127 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 128 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 129 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 130 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 131 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 132 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 133 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 134 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 135 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 136 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 137 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 138 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 139 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 140 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 141 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 142 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 143 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 144 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 145 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 146 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 147 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 148 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 149 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 150 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 151 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 152 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 153 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 154 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 155 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 156 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 157 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 158 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 159 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 160 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 161 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 162 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 163 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 164 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 165 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 166 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 167 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 168 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 169 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 170 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 171 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 172 out: rows 0, b0000, tables equal regrow, routes equal regrow
grove step 173 out: rows 0, b0000, tables equal regrow, routes equal regrow
rebase universe -> b0000 Microsoft Basic Render Driver · Dx12 · Cpu · vertex storage yes: color identical, ids identical
rebase universe -> b0000 NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes: color identical, ids identical
rebase universe -> b0000 NVIDIA GeForce RTX 2080 · Vulkan · DiscreteGpu · vertex storage yes: color identical, ids identical
planted: regrown tables with cli_a@0's filled bit flipped differ from the delta-built tables; refused (ok)
regrow: 3 adapter(s); planted difference: refused (ok)
```
