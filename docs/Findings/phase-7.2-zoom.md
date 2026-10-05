# Phase 7.2 — The universe zooms: findings

P72-13. A generated universe (the grove, seed 7: 8 galaxies, 128 systems, 3072
bodies, 137 links) is laid out on a fixed grid of charts, drawn through an exact
camera from level −4 to level 9, and picked exactly at any zoom. Bands follow
projected size, a CPU cut decides what is drawn, and the GPU draws every row and
decides per instance with integer tests; the two cuts name the same owners on
every adapter. A pan or zoom writes no table row; a rebase writes chart rows only
and moves no pixel. The shell opens a `.universe`, zooms about the cursor, pans,
resizes, and prints one tick line per change. Every prediction in §2.2, §2.4,
§2.5, §2.12 and §2.13 held; none differs.

Commits: P72-01 `744f4d0`, P72-02 `ca89756`, P72-03 `470f5f3`, P72-04 `ac1aa44`,
P72-05 `4d721f6`, P72-03a `ae2c643`, P72-06 `366c49a`, P72-07 `b6c5abf`, P72-08
`7270ca3`, P72-09 `d9cbba9`, P72-10 `8a2acb1`, P72-10a `c2eeb30`, P72-11
`24bef99`, P72-12 `0875fc4`. Machine: Windows 10.0.26300, adapters Microsoft
Basic Render Driver (Dx12, Cpu) and NVIDIA GeForce RTX 2080 (Dx12 and Vulkan).
The outputs in the appendix were re-run at `0875fc4` from `joinn/` with the
console in UTF-8 and copied from the terminal (cargo's own Compiling, Finished
and Running lines dropped). The window lines are P72-11's run.

## What was built, by commit

- **P72-02, the camera.** `k = 2^level·(256 + step)/256`, a focus in 2^-16 layout
  units of the anchor chart, a whole-pixel pin. Zoom about a pixel refocuses once
  (the only rounding) and then only changes `(level, step)`. Eight notches in and
  eight out about one pixel return the identical camera (V144).
- **P72-03, P72-03a, the grove.** SplitMix64 from seed 7 assigns slots 6 … 23.
  The `sys_` links of §2.4 were refused (`link sys_g0s00 member b0005.listen@0 is
  Text 1 beside ℤ 1`); Amendment 1 changed the links, not a body, and the grove
  is admitted.
- **P72-04, layout.** Galaxies, systems and bodies on the §2.5 grid; the anchor
  is the deepest chart under the centre pixel.
- **P72-05, bands and the cut.** Integer band tests (`10·s ≥ 11·T` to enter),
  lens nodes for folded systems, and touches: a link touches each cut node at
  most once (V146).
- **P72-06, the stroke font.** 48 glyphs on a 6 × 8 grid; titles, cell labels,
  out-port values.
- **P72-07, tables.** Chart, frame and stroke tables; chart rows hold origins
  relative to the anchor, so only a rebase writes them (V142).
- **P72-08, the shader.** A 48-byte exact tick uniform (whole pixel plus a
  2^-32 fraction); band, owner and fade decisions in `u32`/`i32`; owner and
  ghost pipelines. Gates 6 and 7 unchanged.
- **P72-09, exact pick.** The CPU pick in 2^-32 px with a 2^28 edge band; the
  capsule's middle test needs 256-bit products. On every adapter and every
  §2.12 view, disagree 0 and the GPU's owners equal the cut's (V145).
- **P72-10, the rebase on the GPU.** The zoom script's 174 steps all leave tables
  equal to a fresh grow; the one rebase is color- and ID-identical on all three
  adapters (V143). A plant that rounds the rebased focus moves pixels and is
  caught.
- **P72-11, the shell.** `.universe` opens its first lens in canonical order
  (`--lens` chooses), framed; wheel, drag (≥ 4 px), `+`/`-`, arrows, `F` and
  resize drive the camera; `tick: level <L> step <t> (k <f>), anchor <chart>,
  rows <n>` once per change, nothing when idle (V147).
- **P72-12, the measurement.** `cargo xtask zoom --measure`.

## Predictions

### §2.2 The camera

| Prediction | Plan | Printed | Mark |
|---|---|---|---|
| Frame of the grove at 1920 × 1080 | level −2, step 95, k 351/1024 (`k·5504 = 1886 5/8 ≤ 1888`; step 96 gives 1892) | `zoom 1920x1080 frame level -2 step 95 (k 351/1024), anchor universe: …` (first line of `cargo xtask zoom`); `the_grove_frames_two_levels_down_at_step_95_at_1920_by_1080` asserts `(351, 1024)`, `351·5504 ≤ 1888·1024` and step 96 `> 1888` | as predicted |
| Level outside −4 … 9 | `zoom: level <n> is outside −4 … 9; acceptance is a level from −4 to 9` | `refused: zoom: level 10 is outside −4 … 9; acceptance is a level from −4 to 9` (the shell, zooming past level 9) | as predicted |

### §2.4 The grove

| Prediction | Plan | Printed | Mark |
|---|---|---|---|
| First three SplitMix64 draws, seed 7 | 7191089600892374487, 309689372594955804, 16616101746815609346 | asserted by `splitmix64`'s test | as predicted |
| Kinds | calc 1181, units 982, bus 909 | `3072 bodies (calc 1181, units 982, bus 909)` | as predicted |
| Links and members | 137 links, 1182 members | `137 links, 1182 members` | as predicted |
| Admission | admitted (after Amendment 1) | `admitted; hash 068428e1a3663079efc55bf76b097b0e28108713f6fbd361a8234d2402500ab2` | as predicted (Amendment 1; §2.4 as first written was refused, see P72-03) |

### §2.5 Universe layout

| Prediction | Plan | Printed | Mark |
|---|---|---|---|
| Last line of `cargo xtask layout --universe grove` | `layout universe: 8 galaxies, 128 systems, 3072 bodies, 5504x1600` | `layout universe: 8 galaxies, 128 systems, 3072 bodies, 5504x1600` | as predicted |

### §2.12 The predicted views

All seventeen lines (fifteen views, two touches lines) printed byte for byte;
`fns::zoom` tests assert the block against the plan's text.

| Line | Mark |
|---|---|
| frame level −2 step 95: galaxies 8 open · systems 128 open · glyph 3072 | as predicted |
| at s level −4: galaxies 8 open · systems 0 open, 128 nodes · no bodies | as predicted |
| at s level −3: systems 128 open · dot 1891, glyph 1181 | as predicted |
| at s level −2: galaxies 6 · systems 96 · glyph 2240 | as predicted |
| at s level −1: galaxies 4 · systems 36 · glyph 864 | as predicted |
| at s level 0: systems 16 · glyph 153, summary 113 | as predicted |
| at s level 1: systems 4 · summary 80 | as predicted |
| at s level 2: systems 1 · summary 24 | as predicted |
| at s level 3: summary 4, full 8 | as predicted |
| at s level 4: full 4 | as predicted |
| at s level 5: full 2 | as predicted |
| at s levels 6, 7, 8, 9: full 1 | as predicted (four lines) |
| touches level −4 step 0: 137 links, 264 touches | as predicted |
| touches frame: 137 links, 1182 touches | as predicted |

### §2.13 Touch counts

| Prediction | Plan | Printed | Mark |
|---|---|---|---|
| Level −4 step 0 | 128 node owners, no body owner; touches 264 | `touches level -4 step 0: 137 links, 264 touches, each node at most once per link`; pick at s level −4: `owners 136 (cut allows 136)` = 128 system nodes + 8 galaxy frames, no body | as predicted |
| Frame | touches 1182 | `touches frame: 137 links, 1182 touches, each node at most once per link` | as predicted |

## The adversary: does the cut cost per tick?

**The CPU cut does not run per tick.** A tick in the shell runs
`UniverseScene::rebase` (which calls `UniverseLayout::anchor_of`, then
`chart_at`), `tick_bytes`, and `Renderer::draw_at`. `cut` runs only when the
user clicks (`shapes_at` for the CPU pick) and in the xtask checks. Measured in
release (`cargo xtask zoom --measure`, least of five runs): **19 to 66 µs** per
view, largest at the frame and level −3, where 3208 charts are in the cut, and
19 µs once only 3 or 4 entries remain.

What does grow with the universe on every tick:

- **The GPU draw.** Every tick issues **302352 instances** at every view:
  2 × (136 frames + 2 × 3072 bodies + 5434 cells + 2362 links + 13031 ports +
  124069 strokes). The shader discards what the cut excludes, so the cost is per
  row, not per visible thing. Strokes are 82% of the rows.
- **The rebase check.** `chart_at` walks every chart at each depth to find the
  child holding the centre point: up to 4 walks over 3209 charts per tick. It
  was not timed separately.

No Core limit was hit: eight storage tables fill the vertex stage's eight
storage-buffer slots, so the style table moved to a uniform (P72-08), and the
GPU-side cut needed nothing more.

## What 7.3 needs (observed)

- Drawing every stroke row every tick dominates the instance count; text needs a per-chart or per-band instance range, or a GPU-side cull, before more text is added.
- `chart_at` scans all charts per tick; a parent-to-children index would make the rebase check proportional to depth.
- Touches already give each link its cut nodes per view; hyperedges can be drawn from them.
- Frames switch frame/node style at the summary threshold without a crossfade; only body elements fade.
- A single-body `Scene` draws no text; titles, labels and values exist only in the universe scene.
- The window's idle (V147) could not be observed from Cursor: both window runs received input Cursor did not send. AJ's §6.2 check is the window evidence.
- A resize can rebase (the 1280 × 720 resize moved the centre into `g1s10` and wrote 3209 chart rows); 3209 rows per rebase is every chart, which a larger universe multiplies.

## Appendix: command outputs

### `cargo xtask grove`

```
grove seed 7: 8 galaxies, 128 systems, 3072 bodies (calc 1181, units 982, bus 909), 137 links, 1182 members, admitted; hash 068428e1a3663079efc55bf76b097b0e28108713f6fbd361a8234d2402500ab2
[exit 0]
```

### `cargo xtask layout --universe grove` (first 8 lines, last 3; 3209 lines in all)

```
galaxy g0 64 64 1296 704
system g0s00 16 32 304 152
body b0000 calc 8 16 40 24
body b0001 calc 56 16 40 24
body b0002 calc 104 16 40 24
body b0003 units 152 16 20 18
body b0004 units 200 16 20 18
body b0005 bus 248 16 20 14
...
body b3070 bus 200 112 20 14
body b3071 bus 248 112 20 14
layout universe: 8 galaxies, 128 systems, 3072 bodies, 5504x1600
[exit 0]
```

### `cargo xtask zoom`

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
zoom: 15 views, cut counts printed; touches 264 and 1182
[exit 0]
```

### `cargo run --release -p xtask -- zoom --measure`

```
measure frame level -2 step 95 (k 351/1024): cut cpu 60 µs (3208 entries), draw instances 302352
measure at s level -4 step 0 (k 1/16): cut cpu 22 µs (136 entries), draw instances 302352
measure at s level -3 step 0 (k 1/8): cut cpu 66 µs (3208 entries), draw instances 302352
measure at s level -2 step 0 (k 1/4): cut cpu 49 µs (2342 entries), draw instances 302352
measure at s level -1 step 0 (k 1/2): cut cpu 33 µs (904 entries), draw instances 302352
measure at s level 0 step 0 (k 1): cut cpu 32 µs (283 entries), draw instances 302352
measure at s level 1 step 0 (k 2): cut cpu 22 µs (85 entries), draw instances 302352
measure at s level 2 step 0 (k 4): cut cpu 21 µs (26 entries), draw instances 302352
measure at s level 3 step 0 (k 8): cut cpu 28 µs (14 entries), draw instances 302352
measure at s level 4 step 0 (k 16): cut cpu 20 µs (6 entries), draw instances 302352
measure at s level 5 step 0 (k 32): cut cpu 20 µs (4 entries), draw instances 302352
measure at s level 6 step 0 (k 64): cut cpu 20 µs (3 entries), draw instances 302352
measure at s level 7 step 0 (k 128): cut cpu 20 µs (3 entries), draw instances 302352
measure at s level 8 step 0 (k 256): cut cpu 20 µs (3 entries), draw instances 302352
measure at s level 9 step 0 (k 512): cut cpu 20 µs (3 entries), draw instances 302352
zoom --measure: 15 views; tables frame 136, body 3072, cell 5434, link 2362, port 13031, stroke 124069
[exit 0]
```

### `cargo test -p xtask shell_session -- --nocapture`

```

running 2 tests
tick: level -2 step 95 (k 351/1024), anchor universe, rows 0
tick: level -2 step 127 (k 383/1024), anchor universe, rows 0
tick: level -2 step 159 (k 415/1024), anchor universe, rows 0
tick: level -2 step 191 (k 447/1024), anchor universe, rows 0
tick: level -2 step 223 (k 479/1024), anchor universe, rows 0
tick: level -2 step 255 (k 511/1024), anchor universe, rows 0
tick: level -1 step 31 (k 287/512), anchor universe, rows 0
tick: level -1 step 63 (k 319/512), anchor universe, rows 0
tick: level -1 step 95 (k 351/512), anchor universe, rows 0
tick: level -1 step 95 (k 351/512), anchor universe, rows 0
tick: level -1 step 95 (k 351/512), anchor g1s10, rows 3209
test fns::shell_session::tests::the_grove_frames_zooms_pans_and_resizes_then_idles ... ok
test fns::shell_session::tests::a_click_names_its_owner_and_a_zoom_past_level_9_is_refused ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 90 filtered out; finished in 1.16s

[exit 0]
```

### `cargo xtask pick`

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
frame median 4.276 ms (calculator 1280x720; information, not a check)
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
grove frame: sample 4096 agree 4059, edge 37, disagree 0, owners 3208 (cut allows 3208)
grove s level -4 step 0: sample 4096 agree 4095, edge 1, disagree 0, owners 136 (cut allows 136)
grove s level -3 step 0: sample 4096 agree 4092, edge 4, disagree 0, owners 3208 (cut allows 3208)
grove s level -2 step 0: sample 4096 agree 4095, edge 1, disagree 0, owners 2342 (cut allows 2342)
grove s level -1 step 0: sample 4096 agree 4095, edge 1, disagree 0, owners 904 (cut allows 904)
grove s level 0 step 0: sample 4096 agree 4093, edge 3, disagree 0, owners 939 (cut allows 939)
grove s level 1 step 0: sample 4096 agree 4068, edge 28, disagree 0, owners 452 (cut allows 452)
grove s level 2 step 0: sample 4096 agree 4081, edge 15, disagree 0, owners 141 (cut allows 141)
grove s level 3 step 0: sample 4096 agree 4092, edge 4, disagree 0, owners 99 (cut allows 99)
grove s level 4 step 0: sample 4096 agree 4096, edge 0, disagree 0, owners 44 (cut allows 44)
grove s level 5 step 0: sample 4096 agree 4094, edge 2, disagree 0, owners 19 (cut allows 19)
grove s level 6 step 0: sample 4096 agree 4096, edge 0, disagree 0, owners 10 (cut allows 10)
grove s level 7 step 0: sample 4096 agree 4095, edge 1, disagree 0, owners 4 (cut allows 4)
grove s level 8 step 0: sample 4096 agree 4096, edge 0, disagree 0, owners 4 (cut allows 4)
grove s level 9 step 0: sample 4096 agree 4094, edge 2, disagree 0, owners 3 (cut allows 3)
adapter NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes
calculator 640x360: agree 230216, edge 184, disagree 0, owners 13/13
calculator 1000x777: agree 776768, edge 232, disagree 0, owners 13/13
calculator 1280x720: agree 921288, edge 312, disagree 0, owners 13/13
calculator 1920x1080: agree 2073144, edge 456, disagree 0, owners 13/13
calculator.contact 640x360: agree 230248, edge 152, disagree 0, owners 7/7, links 0
calculator.contact 1000x777: agree 776784, edge 216, disagree 0, owners 7/7, links 0
calculator.contact 1280x720: agree 921256, edge 344, disagree 0, owners 7/7, links 0
calculator.contact 1920x1080: agree 2073164, edge 436, disagree 0, owners 7/7, links 0
frame median 2.519 ms (calculator 1280x720; information, not a check)
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
grove frame: sample 4096 agree 4059, edge 37, disagree 0, owners 3208 (cut allows 3208)
grove s level -4 step 0: sample 4096 agree 4095, edge 1, disagree 0, owners 136 (cut allows 136)
grove s level -3 step 0: sample 4096 agree 4092, edge 4, disagree 0, owners 3208 (cut allows 3208)
grove s level -2 step 0: sample 4096 agree 4095, edge 1, disagree 0, owners 2342 (cut allows 2342)
grove s level -1 step 0: sample 4096 agree 4095, edge 1, disagree 0, owners 904 (cut allows 904)
grove s level 0 step 0: sample 4096 agree 4093, edge 3, disagree 0, owners 939 (cut allows 939)
grove s level 1 step 0: sample 4096 agree 4068, edge 28, disagree 0, owners 452 (cut allows 452)
grove s level 2 step 0: sample 4096 agree 4081, edge 15, disagree 0, owners 141 (cut allows 141)
grove s level 3 step 0: sample 4096 agree 4092, edge 4, disagree 0, owners 99 (cut allows 99)
grove s level 4 step 0: sample 4096 agree 4096, edge 0, disagree 0, owners 44 (cut allows 44)
grove s level 5 step 0: sample 4096 agree 4094, edge 2, disagree 0, owners 19 (cut allows 19)
grove s level 6 step 0: sample 4096 agree 4096, edge 0, disagree 0, owners 10 (cut allows 10)
grove s level 7 step 0: sample 4096 agree 4095, edge 1, disagree 0, owners 4 (cut allows 4)
grove s level 8 step 0: sample 4096 agree 4096, edge 0, disagree 0, owners 4 (cut allows 4)
grove s level 9 step 0: sample 4096 agree 4094, edge 2, disagree 0, owners 3 (cut allows 3)
adapter NVIDIA GeForce RTX 2080 · Vulkan · DiscreteGpu · vertex storage yes
calculator 640x360: agree 230216, edge 184, disagree 0, owners 13/13
calculator 1000x777: agree 776768, edge 232, disagree 0, owners 13/13
calculator 1280x720: agree 921288, edge 312, disagree 0, owners 13/13
calculator 1920x1080: agree 2073144, edge 456, disagree 0, owners 13/13
calculator.contact 640x360: agree 230248, edge 152, disagree 0, owners 7/7, links 0
calculator.contact 1000x777: agree 776784, edge 216, disagree 0, owners 7/7, links 0
calculator.contact 1280x720: agree 921256, edge 344, disagree 0, owners 7/7, links 0
calculator.contact 1920x1080: agree 2073164, edge 436, disagree 0, owners 7/7, links 0
frame median 0.663 ms (calculator 1280x720; information, not a check)
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
grove frame: sample 4096 agree 4059, edge 37, disagree 0, owners 3208 (cut allows 3208)
grove s level -4 step 0: sample 4096 agree 4095, edge 1, disagree 0, owners 136 (cut allows 136)
grove s level -3 step 0: sample 4096 agree 4092, edge 4, disagree 0, owners 3208 (cut allows 3208)
grove s level -2 step 0: sample 4096 agree 4095, edge 1, disagree 0, owners 2342 (cut allows 2342)
grove s level -1 step 0: sample 4096 agree 4095, edge 1, disagree 0, owners 904 (cut allows 904)
grove s level 0 step 0: sample 4096 agree 4093, edge 3, disagree 0, owners 939 (cut allows 939)
grove s level 1 step 0: sample 4096 agree 4068, edge 28, disagree 0, owners 452 (cut allows 452)
grove s level 2 step 0: sample 4096 agree 4081, edge 15, disagree 0, owners 141 (cut allows 141)
grove s level 3 step 0: sample 4096 agree 4092, edge 4, disagree 0, owners 99 (cut allows 99)
grove s level 4 step 0: sample 4096 agree 4096, edge 0, disagree 0, owners 44 (cut allows 44)
grove s level 5 step 0: sample 4096 agree 4094, edge 2, disagree 0, owners 19 (cut allows 19)
grove s level 6 step 0: sample 4096 agree 4096, edge 0, disagree 0, owners 10 (cut allows 10)
grove s level 7 step 0: sample 4096 agree 4095, edge 1, disagree 0, owners 4 (cut allows 4)
grove s level 8 step 0: sample 4096 agree 4096, edge 0, disagree 0, owners 4 (cut allows 4)
grove s level 9 step 0: sample 4096 agree 4094, edge 2, disagree 0, owners 3 (cut allows 3)
planted: port sum@1 moved one unit, cpu_pick against the GPU on Microsoft Basic Render Driver · Dx12 · Cpu · vertex storage yes: disagree 7336; refused as truth violation (ok)
pick: 3 adapter(s), 12 subject(s) agree; planted disagreement: refused as truth violation (ok)
[exit 0]
```

### `cargo xtask regrow`

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
grove step 0 frame: rows 0, universe, tables equal regrow
grove step 1 focus s: rows 3209, b0000, tables equal regrow
grove step 2 in: rows 0, b0000, tables equal regrow
grove step 3 in: rows 0, b0000, tables equal regrow
grove step 4 in: rows 0, b0000, tables equal regrow
grove step 5 in: rows 0, b0000, tables equal regrow
grove step 6 in: rows 0, b0000, tables equal regrow
grove step 7 in: rows 0, b0000, tables equal regrow
grove step 8 in: rows 0, b0000, tables equal regrow
grove step 9 in: rows 0, b0000, tables equal regrow
grove step 10 in: rows 0, b0000, tables equal regrow
grove step 11 in: rows 0, b0000, tables equal regrow
grove step 12 in: rows 0, b0000, tables equal regrow
grove step 13 in: rows 0, b0000, tables equal regrow
grove step 14 in: rows 0, b0000, tables equal regrow
grove step 15 in: rows 0, b0000, tables equal regrow
grove step 16 in: rows 0, b0000, tables equal regrow
grove step 17 in: rows 0, b0000, tables equal regrow
grove step 18 in: rows 0, b0000, tables equal regrow
grove step 19 in: rows 0, b0000, tables equal regrow
grove step 20 in: rows 0, b0000, tables equal regrow
grove step 21 in: rows 0, b0000, tables equal regrow
grove step 22 in: rows 0, b0000, tables equal regrow
grove step 23 in: rows 0, b0000, tables equal regrow
grove step 24 in: rows 0, b0000, tables equal regrow
grove step 25 in: rows 0, b0000, tables equal regrow
grove step 26 in: rows 0, b0000, tables equal regrow
grove step 27 in: rows 0, b0000, tables equal regrow
grove step 28 in: rows 0, b0000, tables equal regrow
grove step 29 in: rows 0, b0000, tables equal regrow
grove step 30 in: rows 0, b0000, tables equal regrow
grove step 31 in: rows 0, b0000, tables equal regrow
grove step 32 in: rows 0, b0000, tables equal regrow
grove step 33 in: rows 0, b0000, tables equal regrow
grove step 34 in: rows 0, b0000, tables equal regrow
grove step 35 in: rows 0, b0000, tables equal regrow
grove step 36 in: rows 0, b0000, tables equal regrow
grove step 37 in: rows 0, b0000, tables equal regrow
grove step 38 in: rows 0, b0000, tables equal regrow
grove step 39 in: rows 0, b0000, tables equal regrow
grove step 40 in: rows 0, b0000, tables equal regrow
grove step 41 in: rows 0, b0000, tables equal regrow
grove step 42 in: rows 0, b0000, tables equal regrow
grove step 43 in: rows 0, b0000, tables equal regrow
grove step 44 in: rows 0, b0000, tables equal regrow
grove step 45 in: rows 0, b0000, tables equal regrow
grove step 46 in: rows 0, b0000, tables equal regrow
grove step 47 in: rows 0, b0000, tables equal regrow
grove step 48 in: rows 0, b0000, tables equal regrow
grove step 49 in: rows 0, b0000, tables equal regrow
grove step 50 in: rows 0, b0000, tables equal regrow
grove step 51 in: rows 0, b0000, tables equal regrow
grove step 52 in: rows 0, b0000, tables equal regrow
grove step 53 in: rows 0, b0000, tables equal regrow
grove step 54 in: rows 0, b0000, tables equal regrow
grove step 55 in: rows 0, b0000, tables equal regrow
grove step 56 in: rows 0, b0000, tables equal regrow
grove step 57 in: rows 0, b0000, tables equal regrow
grove step 58 in: rows 0, b0000, tables equal regrow
grove step 59 in: rows 0, b0000, tables equal regrow
grove step 60 in: rows 0, b0000, tables equal regrow
grove step 61 in: rows 0, b0000, tables equal regrow
grove step 62 in: rows 0, b0000, tables equal regrow
grove step 63 in: rows 0, b0000, tables equal regrow
grove step 64 in: rows 0, b0000, tables equal regrow
grove step 65 in: rows 0, b0000, tables equal regrow
grove step 66 in: rows 0, b0000, tables equal regrow
grove step 67 in: rows 0, b0000, tables equal regrow
grove step 68 in: rows 0, b0000, tables equal regrow
grove step 69 in: rows 0, b0000, tables equal regrow
grove step 70 in: rows 0, b0000, tables equal regrow
grove step 71 in: rows 0, b0000, tables equal regrow
grove step 72 in: rows 0, b0000, tables equal regrow
grove step 73 in: rows 0, b0000, tables equal regrow
grove step 74 in: rows 0, b0000, tables equal regrow
grove step 75 in: rows 0, b0000, tables equal regrow
grove step 76 in: rows 0, b0000, tables equal regrow
grove step 77 in: rows 0, b0000, tables equal regrow
grove step 78 in: rows 0, b0000, tables equal regrow
grove step 79 in: rows 0, b0000, tables equal regrow
grove step 80 in: rows 0, b0000, tables equal regrow
grove step 81 in: rows 0, b0000, tables equal regrow
grove step 82 in: rows 0, b0000, tables equal regrow
grove step 83 in: rows 0, b0000, tables equal regrow
grove step 84 in: rows 0, b0000, tables equal regrow
grove step 85 in: rows 0, b0000, tables equal regrow
grove step 86 in: rows 0, b0000, tables equal regrow
grove step 87 in: rows 0, b0000, tables equal regrow
grove step 88 out: rows 0, b0000, tables equal regrow
grove step 89 out: rows 0, b0000, tables equal regrow
grove step 90 out: rows 0, b0000, tables equal regrow
grove step 91 out: rows 0, b0000, tables equal regrow
grove step 92 out: rows 0, b0000, tables equal regrow
grove step 93 out: rows 0, b0000, tables equal regrow
grove step 94 out: rows 0, b0000, tables equal regrow
grove step 95 out: rows 0, b0000, tables equal regrow
grove step 96 out: rows 0, b0000, tables equal regrow
grove step 97 out: rows 0, b0000, tables equal regrow
grove step 98 out: rows 0, b0000, tables equal regrow
grove step 99 out: rows 0, b0000, tables equal regrow
grove step 100 out: rows 0, b0000, tables equal regrow
grove step 101 out: rows 0, b0000, tables equal regrow
grove step 102 out: rows 0, b0000, tables equal regrow
grove step 103 out: rows 0, b0000, tables equal regrow
grove step 104 out: rows 0, b0000, tables equal regrow
grove step 105 out: rows 0, b0000, tables equal regrow
grove step 106 out: rows 0, b0000, tables equal regrow
grove step 107 out: rows 0, b0000, tables equal regrow
grove step 108 out: rows 0, b0000, tables equal regrow
grove step 109 out: rows 0, b0000, tables equal regrow
grove step 110 out: rows 0, b0000, tables equal regrow
grove step 111 out: rows 0, b0000, tables equal regrow
grove step 112 out: rows 0, b0000, tables equal regrow
grove step 113 out: rows 0, b0000, tables equal regrow
grove step 114 out: rows 0, b0000, tables equal regrow
grove step 115 out: rows 0, b0000, tables equal regrow
grove step 116 out: rows 0, b0000, tables equal regrow
grove step 117 out: rows 0, b0000, tables equal regrow
grove step 118 out: rows 0, b0000, tables equal regrow
grove step 119 out: rows 0, b0000, tables equal regrow
grove step 120 out: rows 0, b0000, tables equal regrow
grove step 121 out: rows 0, b0000, tables equal regrow
grove step 122 out: rows 0, b0000, tables equal regrow
grove step 123 out: rows 0, b0000, tables equal regrow
grove step 124 out: rows 0, b0000, tables equal regrow
grove step 125 out: rows 0, b0000, tables equal regrow
grove step 126 out: rows 0, b0000, tables equal regrow
grove step 127 out: rows 0, b0000, tables equal regrow
grove step 128 out: rows 0, b0000, tables equal regrow
grove step 129 out: rows 0, b0000, tables equal regrow
grove step 130 out: rows 0, b0000, tables equal regrow
grove step 131 out: rows 0, b0000, tables equal regrow
grove step 132 out: rows 0, b0000, tables equal regrow
grove step 133 out: rows 0, b0000, tables equal regrow
grove step 134 out: rows 0, b0000, tables equal regrow
grove step 135 out: rows 0, b0000, tables equal regrow
grove step 136 out: rows 0, b0000, tables equal regrow
grove step 137 out: rows 0, b0000, tables equal regrow
grove step 138 out: rows 0, b0000, tables equal regrow
grove step 139 out: rows 0, b0000, tables equal regrow
grove step 140 out: rows 0, b0000, tables equal regrow
grove step 141 out: rows 0, b0000, tables equal regrow
grove step 142 out: rows 0, b0000, tables equal regrow
grove step 143 out: rows 0, b0000, tables equal regrow
grove step 144 out: rows 0, b0000, tables equal regrow
grove step 145 out: rows 0, b0000, tables equal regrow
grove step 146 out: rows 0, b0000, tables equal regrow
grove step 147 out: rows 0, b0000, tables equal regrow
grove step 148 out: rows 0, b0000, tables equal regrow
grove step 149 out: rows 0, b0000, tables equal regrow
grove step 150 out: rows 0, b0000, tables equal regrow
grove step 151 out: rows 0, b0000, tables equal regrow
grove step 152 out: rows 0, b0000, tables equal regrow
grove step 153 out: rows 0, b0000, tables equal regrow
grove step 154 out: rows 0, b0000, tables equal regrow
grove step 155 out: rows 0, b0000, tables equal regrow
grove step 156 out: rows 0, b0000, tables equal regrow
grove step 157 out: rows 0, b0000, tables equal regrow
grove step 158 out: rows 0, b0000, tables equal regrow
grove step 159 out: rows 0, b0000, tables equal regrow
grove step 160 out: rows 0, b0000, tables equal regrow
grove step 161 out: rows 0, b0000, tables equal regrow
grove step 162 out: rows 0, b0000, tables equal regrow
grove step 163 out: rows 0, b0000, tables equal regrow
grove step 164 out: rows 0, b0000, tables equal regrow
grove step 165 out: rows 0, b0000, tables equal regrow
grove step 166 out: rows 0, b0000, tables equal regrow
grove step 167 out: rows 0, b0000, tables equal regrow
grove step 168 out: rows 0, b0000, tables equal regrow
grove step 169 out: rows 0, b0000, tables equal regrow
grove step 170 out: rows 0, b0000, tables equal regrow
grove step 171 out: rows 0, b0000, tables equal regrow
grove step 172 out: rows 0, b0000, tables equal regrow
grove step 173 out: rows 0, b0000, tables equal regrow
rebase universe -> b0000 Microsoft Basic Render Driver · Dx12 · Cpu · vertex storage yes: color identical, ids identical
rebase universe -> b0000 NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes: color identical, ids identical
rebase universe -> b0000 NVIDIA GeForce RTX 2080 · Vulkan · DiscreteGpu · vertex storage yes: color identical, ids identical
planted: regrown tables with cli_a@0's filled bit flipped differ from the delta-built tables; refused (ok)
regrow: 3 adapter(s); planted difference: refused (ok)
[exit 0]
```
