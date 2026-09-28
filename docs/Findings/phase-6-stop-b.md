# Phase 6 Stop B: end of chunk B (P6-07a to P6-11)

Recorded by Cursor on 28 Sep 2026 from `D:\JoInn` on Windows (PowerShell).
Values below are copied from the terminal, not summarised.

`git rev-parse HEAD` after P6-11 (last commit of chunk B):

```
08024b9e21d09ccc279ba89fc7c87c9f9cf3862b
```

The stop-report commit that adds this file is `P6-stop-b` on `main` immediately after that.

## Commit reports (in order)

```
Commit:     P6-07a 79a6f4cf572ad56a1d4f0cea8440bd6becd609da
Done-when:  git show --stat HEAD → 3 files changed, 25 insertions(+), 2 deletions(-); the plan, decisions.md and the backlog, nothing under joinn/crates or joinn/xtask          MET
             docs/Findings/decisions.md                      |  1 +
             docs/Plans/JoInn Phase 6 Implementation Plan.md | 22 ++++++++++++++++++++--
             docs/Theory/JoInn Research Backlog.md           |  4 ++++
Suite:      cargo test --workspace --no-fail-fast → 243 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      none
```

```
Commit:     P6-08 7c77f34b29e52eb6718a6620f89de2a860964945
Done-when:  cargo xtask adapters → adapters: 3 (whole output below)          MET
            joinn-gpu test at 640x360, every adapter, every §2.13 probe scaled → test render_offscreen::tests::the_calculator_at_640x360_names_every_probe_owner_on_every_adapter ... ok          MET
            min_binding_size test → test renderer::layouts::tests::the_pipelines_take_each_row_size_as_min_binding_size_and_refuse_less ... ok          MET
            CI green on both OSes → read after the push: run 36495169068, head_sha 7c77f34b29e52eb6718a6620f89de2a860964945, status completed, conclusion success; check (ubuntu-24.04): success; check (windows-latest): success          MET
Suite:      cargo test --workspace --no-fail-fast → 246 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      P6-08: the min_binding_size test shows a shortened minimum refused for body, cell, port, link and the tick uniform. Incidence and style rows are one u32, so one field short is 0, which wgpu reads as "no minimum"; the refusal cannot be shown for those two.
```

`cargo xtask adapters`:

```
Microsoft Basic Render Driver · Dx12 · Cpu · vertex storage yes
NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes
NVIDIA GeForce RTX 2080 · Vulkan · DiscreteGpu · vertex storage yes
adapters: 3
```

`cargo test -p joinn-gpu -- --nocapture` at P6-08:

```
Microsoft Basic Render Driver · Dx12 · Cpu · vertex storage yes: 14 probes at 640x360 name their owners
NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes: 14 probes at 640x360 name their owners
NVIDIA GeForce RTX 2080 · Vulkan · DiscreteGpu · vertex storage yes: 14 probes at 640x360 name their owners
test renderer::layouts::tests::the_pipelines_take_each_row_size_as_min_binding_size_and_refuse_less ... ok
test render_offscreen::tests::the_calculator_at_640x360_names_every_probe_owner_on_every_adapter ... ok
test read_texel::tests::one_texel_names_the_same_owner_as_the_full_readback ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```
Commit:     P6-09 9167909f5b59178e7d03c089ffa3b2d8ef17cee4
Done-when:  whole output pasted (in the commit message and in docs/Findings/phase-6-pick.md)          MET
            every line disagree 0 → 3 adapters x (4 calculator lines + 12 body lines), each "disagree 0"          MET
            calculator lines owners 13/13 → 12 of 12 calculator lines "owners 13/13"          MET
            last line as in §3 → pick: 3 adapter(s), 12 subject(s) agree; planted disagreement: refused as truth violation (ok)          MET
Suite:      cargo test --workspace --no-fail-fast → 248 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      P6-09: §3 lists output only for bodies that bind; pick also prints one "not measured" line with the refusal for each of the 12 corpus bodies that do not grow a scene (9 phase21/22 references, columns_reader, environment, missing_cell). 12 measured matches Amendment A3.
```

The calculator lines and the plant from `cargo xtask pick` at P6-09 (identical on all three adapters; the whole output is in `phase-6-pick.md`):

```
calculator 640x360: agree 230216, edge 184, disagree 0, owners 13/13
calculator 1000x777: agree 776768, edge 232, disagree 0, owners 13/13
calculator 1280x720: agree 921288, edge 312, disagree 0, owners 13/13
calculator 1920x1080: agree 2073144, edge 456, disagree 0, owners 13/13
planted: port sum@1 moved one unit, cpu_pick against the GPU on Microsoft Basic Render Driver · Dx12 · Cpu · vertex storage yes: disagree 7336; refused as truth violation (ok)
pick: 3 adapter(s), 12 subject(s) agree; planted disagreement: refused as truth violation (ok)
```

```
Commit:     P6-10 087c7cbe306c40c4c78349c53b88d3dd484a1834
Done-when:  whole output pasted (below, in the commit message and in docs/Findings/phase-6-pick.md)          MET
            every event line ends tables equal regrow → events 1-5 each end "tables equal regrow"          MET
            camera change: rows 0 → camera change: rows 0          MET
            idle tick: nothing to draw → idle tick: nothing to draw          MET
            every adapter color identical, ids identical → 3 of 3 adapter lines "color identical, ids identical"          MET
            A4: second event shows the +1 and ends tables equal regrow → event 5 cli_a@0 "2": touched cli_a, sum, rows 4 (bound 7 + 1), bytes 144, tables equal regrow          MET
            last line as in §3 → regrow: 3 adapter(s); planted difference: refused (ok)          MET
Suite:      cargo test --workspace --no-fail-fast → 248 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      P6-10: the camera-change and idle-tick checks run once, on the first adapter (§3 prints each once); the script and VH2 run on every adapter. A4's header line "fresh calculator (Amendment A4):" is extra to §3's format.
```

`cargo xtask regrow` at P6-10:

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
regrow NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes: color identical, ids identical
regrow NVIDIA GeForce RTX 2080 · Vulkan · DiscreteGpu · vertex storage yes: color identical, ids identical
planted: regrown tables with cli_a@0's filled bit flipped differ from the delta-built tables; refused (ok)
regrow: 3 adapter(s); planted difference: refused (ok)
```

```
Commit:     P6-11 08024b9e21d09ccc279ba89fc7c87c9f9cf3862b
Done-when:  every prediction is marked → 13 rows in the Predictions table (§2.3; §2.4 x4; §2.7 x5; §2.13; A2; A3), each "as predicted"          MET
            frame-time lines quoted and labelled information, not a check → "## Frame time" with "*Information, not a check.*" and the three "frame median ... (calculator 1280x720; information, not a check)" lines          MET
Suite:      cargo test --workspace --no-fail-fast → 248 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      P6-11: §2.13 has no 1280x720 GPU probe check of its own yet (gate 6 item 3 is later); the finding marks it from the CPU probe test plus pick's 1280x720 disagree 0 on every adapter, and the GPU probe test at 640x360.
```

## Last line of each command (end of chunk B, on 08024b9)

- `cargo test --workspace --no-fail-fast` — `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s` (the last doc-test binary); summed over every `test result:` line: `248 passed, 0 failed` (exit 0)
- `cargo xtask gate all` — `gate all wall milliseconds: 354667` (exit 0)
- `cargo xtask corpus verify` — `corpus verify: 43 hash(es) match; cells admitted` (exit 0)
- `cargo xtask vocab` — `vocab: ok` (exit 0)
- `cargo xtask modules` — `modules: ok (enforced 14 crate(s))` (exit 0)
- `cargo xtask layers` — `layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)` (exit 0)
- `cargo xtask assay agree` — `assay agree: 38 subject(s) agree, 1 not measured; injected disagreements: 2 refused as truth violation (ok)` (exit 0)
- `cargo xtask adapters` — `adapters: 3` (exit 0)
- `cargo xtask pick` — `pick: 3 adapter(s), 12 subject(s) agree; planted disagreement: refused as truth violation (ok)` (exit 0)
- `cargo xtask regrow` — `regrow: 3 adapter(s); planted difference: refused (ok)` (exit 0)

Phase lines from that `gate all`:

```
phase 0: pass
phase 1: 4/4 legacy
phase 2: 4/4 legacy
phase 2.1: 8/8 legacy
phase 2.2: 9/9 legacy
phase 3: 8/8 legacy
phase 4: 4/4
phase 5: 8/8
phase 5.1: 4/4
phase 5.2: 3/3
```

### Every `gate all` line containing `fail`

```
1 ok  references agree; a blind seal fails the item
```

### Every failing test name

none (suite on 08024b9: 248 passed, 0 failed)

## CI

Newest run after the push of `08024b9` (P6-09 to P6-11), read with `Invoke-RestMethod` on `https://api.github.com/repos/joneseysinno/JoInn/actions/runs?branch=main&per_page=3` and that run's `jobs_url`. Its steps include `pick` and `regrow`.

```
head_sha: 08024b9e21d09ccc279ba89fc7c87c9f9cf3862b
status: completed
conclusion: success
check (ubuntu-24.04): success
check (windows-latest): success
```

Run: https://github.com/joneseysinno/JoInn/actions/runs/36498408594

The run for P6-08's own push (`7c77f34`), read the same way:

```
head_sha: 7c77f34b29e52eb6718a6620f89de2a860964945
status: completed
conclusion: success
check (ubuntu-24.04): success
check (windows-latest): success
```

Run: https://github.com/joneseysinno/JoInn/actions/runs/36495169068

## Snags (all from the chunk)

- **P6-08**: the min_binding_size test shows a shortened minimum refused for body, cell, port, link and the tick uniform. Incidence and style rows are one `u32`, so a minimum one field short is 0, which wgpu reads as "no minimum"; the refusal cannot be shown for those two tables.
- **P6-09**: §3 lists `cargo xtask pick` output only for corpus bodies that bind. Pick also prints `<rel>: not measured: <reason>` for each of the 12 corpus bodies that do not grow a scene: the 9 phase21/phase22 reference bodies (layout refuses their wires, as at P6-05), `phase3/columns_reader.body` and `phase3/environment.body` (the scene refuses, R74, as Amendment A3 predicts), and `phase52/controls/missing_cell.body` (its cell is not supplied). 12 bodies are measured, as A3 predicts.
- **P6-10**: the camera-change and idle-tick checks run once, on the first adapter, because §3 prints each line once; §2.12's script and VH2 run on every adapter. The line `fresh calculator (Amendment A4):` before events 4 and 5 is not in §3's format.
- **P6-11**: §2.13's probe table has no 1280×720 GPU check of its own in chunk B (gate 6 item 3 comes later). The finding marks it `as predicted` from the CPU probe test (`the_probe_pixels_name_the_plan_owners_on_the_cpu`), pick's `calculator 1280x720: … disagree 0` on every adapter, and joinn-gpu's probe test at 640×360.
