# Phase 6 · Pick and regrow findings

P6-11. The GPU picker (joinn-gpu's ID target) and the exact CPU picker
(`joinn_visual::cpu_pick`) name the same owner for every non-edge pixel of the
calculator at the four standard viewports and of every measured corpus body at
1280×720, on every adapter this machine has. Tables built by deltas equal a
regrow byte for byte after every scripted event, and GPU state that is
discarded and regrown draws identical color and ID bytes on every adapter.
Both plants are refused.

Commits: P6-09 `9167909`, P6-10 `087c7cb`. Machine: Windows 10.0.26200.
All commands run from `joinn/`.

## Adapters

`cargo xtask adapters`:

```
Microsoft Basic Render Driver ┬╖ Dx12 ┬╖ Cpu ┬╖ vertex storage yes
NVIDIA GeForce RTX 2080 ┬╖ Dx12 ┬╖ DiscreteGpu ┬╖ vertex storage yes
NVIDIA GeForce RTX 2080 ┬╖ Vulkan ┬╖ DiscreteGpu ┬╖ vertex storage yes
adapters: 3
```

## `cargo xtask pick` (P6-09)

For each adapter, the calculator is drawn at each standard viewport and read
back, and each pixel is compared with `cpu_pick` of the same scene and camera.
`agree` counts non-edge pixels where both pickers name the same owner, `edge`
counts pixels in `cpu_pick`'s 1/16-pixel edge band (counted, never judged), and
`disagree` counts everything else. A test asserts that the three add up to
width × height (Amendment A2). `owners` is the number of distinct owners the GPU
named, out of the scene's 13. Each corpus body that grows a scene is then
compared at 1280×720; the others print why they are not measured (Amendment
A3). The plant moves port `sum@1` down one layout unit in the CPU's copy of the
tables only, and the comparison must fail.

```
adapter Microsoft Basic Render Driver ┬╖ Dx12 ┬╖ Cpu ┬╖ vertex storage yes
calculator 640x360: agree 230216, edge 184, disagree 0, owners 13/13
calculator 1000x777: agree 776768, edge 232, disagree 0, owners 13/13
calculator 1280x720: agree 921288, edge 312, disagree 0, owners 13/13
calculator 1920x1080: agree 2073144, edge 456, disagree 0, owners 13/13
frame median 3.462 ms (calculator 1280x720; information, not a check)
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
adapter NVIDIA GeForce RTX 2080 ┬╖ Dx12 ┬╖ DiscreteGpu ┬╖ vertex storage yes
calculator 640x360: agree 230216, edge 184, disagree 0, owners 13/13
calculator 1000x777: agree 776768, edge 232, disagree 0, owners 13/13
calculator 1280x720: agree 921288, edge 312, disagree 0, owners 13/13
calculator 1920x1080: agree 2073144, edge 456, disagree 0, owners 13/13
frame median 1.741 ms (calculator 1280x720; information, not a check)
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
adapter NVIDIA GeForce RTX 2080 ┬╖ Vulkan ┬╖ DiscreteGpu ┬╖ vertex storage yes
calculator 640x360: agree 230216, edge 184, disagree 0, owners 13/13
calculator 1000x777: agree 776768, edge 232, disagree 0, owners 13/13
calculator 1280x720: agree 921288, edge 312, disagree 0, owners 13/13
calculator 1920x1080: agree 2073144, edge 456, disagree 0, owners 13/13
frame median 0.505 ms (calculator 1280x720; information, not a check)
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
planted: port sum@1 moved one unit, cpu_pick against the GPU on Microsoft Basic Render Driver ┬╖ Dx12 ┬╖ Cpu ┬╖ vertex storage yes: disagree 7336; refused as truth violation (ok)
pick: 3 adapter(s), 12 subject(s) agree; planted disagreement: refused as truth violation (ok)
```

## `cargo xtask regrow` (P6-10)

Events 1–3 are §2.12's script. Events 4–5 are Amendment A4's, run on a fresh
calculator. After each event, the pending delta is applied to a joinn-gpu
`Renderer` (so only its rows are written) and drawn. The scene is then regrown
from the same `BodyState`, and the two `table_bytes` are compared. `bound` is
V121's Σ (1 + ports(i)) over the touched instances, `+ 1` when the run cleared
a refused bit on an instance it did not touch. A camera change (frames at
640×360 and 1920×1080) leaves no pending rows and the tables unchanged. Then,
on every adapter: the delta-built picture at 1280×720 is taken, the renderer
and device are dropped, the adapter is opened again, the scene is regrown and
uploaded whole, and the picture is taken again (VH2). The plant flips
`cli_a@0`'s filled bit in the regrown tables, which must then differ from the
delta-built tables.

```
event 1 cli_a@0 "two": touched cli_a, rows 2 (bound 3), bytes 80, tables equal regrow
event 2 cli_a@0 "2": touched cli_a, sum, rows 3 (bound 7), bytes 112, tables equal regrow
event 3 cli_b@0 "3": touched cli_b, sum, rows 4 (bound 7), bytes 128, tables equal regrow
fresh calculator (Amendment A4):
event 4 cli_b@0 "x": touched cli_b, rows 2 (bound 3), bytes 80, tables equal regrow
event 5 cli_a@0 "2": touched cli_a, sum, rows 4 (bound 7 + 1), bytes 144, tables equal regrow
camera change: rows 0
idle tick: nothing to draw
regrow Microsoft Basic Render Driver ┬╖ Dx12 ┬╖ Cpu ┬╖ vertex storage yes: color identical, ids identical
regrow NVIDIA GeForce RTX 2080 ┬╖ Dx12 ┬╖ DiscreteGpu ┬╖ vertex storage yes: color identical, ids identical
regrow NVIDIA GeForce RTX 2080 ┬╖ Vulkan ┬╖ DiscreteGpu ┬╖ vertex storage yes: color identical, ids identical
planted: regrown tables with cli_a@0's filled bit flipped differ from the delta-built tables; refused (ok)
regrow: 3 adapter(s); planted difference: refused (ok)
```

## Predictions

Each of Claude's predictions in the plan, against what was run.

| Prediction | Predicted | Ran | Mark |
|---|---|---|---|
| §2.3 layout block for `phase2/calculator.body` | the 14-line block in §2.3 | `cargo xtask layout phase2/calculator.body`, below: byte-identical (P6-05 asserts it) | as predicted |
| §2.4 camera 640x360 | `k 12, origin 80 36` | `camera 640x360: k 12, origin 80 36` | as predicted |
| §2.4 camera 1000x777 | `k 24, origin 20 100` | `camera 1000x777: k 24, origin 20 100` | as predicted |
| §2.4 camera 1280x720 | `k 28, origin 80 24` | `camera 1280x720: k 28, origin 80 24` | as predicted |
| §2.4 camera 1920x1080 | `k 40, origin 160 60` | `camera 1920x1080: k 40, origin 160 60` | as predicted |
| §2.7 replace to `DropGenome(cli_b)`: row 1 | the body row | `Body 0` | as predicted |
| §2.7 row 2 | `cli_b`'s cell row | `Cell 1` | as predicted |
| §2.7 row 3 | `cli_b@0` | `Port 2` | as predicted |
| §2.7 row 4 | `cli_b@1` | `Port 3` | as predicted |
| §2.7 row 5 | link slot 1 | `Link 1` | as predicted |
| §2.13 probes at 1280×720 (14 lines) | the owners in §2.13's table | CPU: `the_probe_pixels_name_the_plan_owners_on_the_cpu ... ok` asserts all 14. GPU: `calculator 1280x720: ... disagree 0, owners 13/13` on all 3 adapters, so the GPU names the same owner at every probe (no probe is an edge pixel, since the CPU test names an owner at each) | as predicted |
| A2: calculator 1280×720 pick line | `agree 921288, edge 312, disagree 0, owners 13/13` | `calculator 1280x720: agree 921288, edge 312, disagree 0, owners 13/13` on all 3 adapters | as predicted |
| A3: bodies not measured, and the count measured | `columns_reader` and `environment` print `not measured`; 12 bodies measured | both print `not measured: scene: … names an in-port and an out-port …`; `pick: 3 adapter(s), 12 subject(s) agree` | as predicted |

The §2.3 and §2.4 lines, from `cargo xtask layout phase2/calculator.body`:

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
camera 640x360: k 12, origin 80 36
camera 1000x777: k 24, origin 20 100
camera 1280x720: k 28, origin 80 24
camera 1920x1080: k 40, origin 160 60
```

The §2.7 rows, from `cargo test -p joinn-visual replace_to_drop_cli_b -- --nocapture`:

```
replace to DropGenome(cli_b): 5 row(s): Body 0, Cell 1, Port 2, Port 3, Link 1
test scene::replace::tests::replace_to_drop_cli_b_writes_five_rows_and_stales_its_old_id ... ok
```

The §2.13 table, on the CPU (`cargo test -p joinn-visual the_probe_pixels`) and,
scaled to 640×360, on the GPU (`cargo test -p joinn-gpu render_offscreen -- --nocapture`):

```
test pick::owner_of_layout::tests::the_probe_pixels_name_the_plan_owners_on_the_cpu ... ok
Microsoft Basic Render Driver · Dx12 · Cpu · vertex storage yes: 14 probes at 640x360 name their owners
NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes: 14 probes at 640x360 name their owners
NVIDIA GeForce RTX 2080 · Vulkan · DiscreteGpu · vertex storage yes: 14 probes at 640x360 name their owners
test render_offscreen::tests::the_calculator_at_640x360_names_every_probe_owner_on_every_adapter ... ok
```

## Frame time

*Information, not a check.* The median of 31 frames (after 5 warm-up frames)
of the calculator at 1280×720, each drawn and waited for, from the P6-09 run:

```
adapter Microsoft Basic Render Driver ┬╖ Dx12 ┬╖ Cpu ┬╖ vertex storage yes
frame median 3.462 ms (calculator 1280x720; information, not a check)
adapter NVIDIA GeForce RTX 2080 ┬╖ Dx12 ┬╖ DiscreteGpu ┬╖ vertex storage yes
frame median 1.741 ms (calculator 1280x720; information, not a check)
adapter NVIDIA GeForce RTX 2080 ┬╖ Vulkan ┬╖ DiscreteGpu ┬╖ vertex storage yes
frame median 0.505 ms (calculator 1280x720; information, not a check)
```

## Notes

- The per-body pick counts (`agree`/`edge`) at 1280×720 are the same on all three
  adapters, and so are the calculator's at every viewport: the fragment stage
  in f32 and the exact integer picker agree on every pixel outside the 1/16-pixel
  edge band.
- The plant's `disagree 7336` is the pixels that change owner when `sum@1` moves
  one unit (28 px) on the CPU side only.
- The regrow plant is a table comparison, not a GPU image comparison. VH2's GPU
  check is the per-adapter `color identical, ids identical` lines.
