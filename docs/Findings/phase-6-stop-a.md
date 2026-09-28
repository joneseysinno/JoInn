# Phase 6 Stop A: end of chunk A (P6-01 to P6-07)

Recorded by Cursor on 28 Sep 2026 from `D:\JoInn` on Windows (PowerShell).
Values below are copied from the terminal, not summarised.

`git rev-parse HEAD` after P6-07 (last commit of chunk A):

```
bd638b6544308c585837f84ab9f2718ba4b7dc3a
```

The stop-report commit that adds this file is `P6-stop-a` on `main` immediately after that.

## Commit reports (in order)

```
Commit:     P6-01 0ec763de1bfce21b098468cfdcd1eca43a654a40
Done-when:  git show --stat HEAD → 11 files changed, 3051 insertions(+), 27 deletions(-)          MET
            docs/Findings/decisions.md                      |    4 +
            docs/Findings/dependencies.md                   |   11 +
            docs/Findings/spikes/s2-gpu.md                  |   62 +-
            docs/Plans/JoInn Phase 6 Implementation Plan.md |  663 ++++++++++++
            docs/Theory/JoInn Research Backlog.md           |   16 +
            joinn/AGENTS.md                                 |   37 +-
            joinn/spikes/s2-gpu/.gitignore                  |    1 +
            joinn/spikes/s2-gpu/Cargo.lock                  | 1233 +++++++++++++++++++++++
            joinn/spikes/s2-gpu/Cargo.toml                  |   13 +
            joinn/spikes/s2-gpu/RESULTS.md                  |   81 ++
            joinn/spikes/s2-gpu/src/main.rs                 |  957 ++++++++++++++++++
            (nothing under joinn/crates)
Suite:      cargo test --workspace --no-fail-fast → 207 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s)) · layers → (from P6-03)
Snags:      none
```

```
Commit:     P6-02 7cf7d0a6404cd83733fa3876de911548dd8aa6ff
Done-when:  push, then read CI as in §0.2 item 4 → read after the push and pasted in docs/Findings/phase-6-stop-a.md (a commit cannot carry the CI read of its own push)
            read after the push: run 36484809177, head_sha 7cf7d0a6404cd83733fa3876de911548dd8aa6ff, status completed, conclusion success; check (ubuntu-24.04): success; check (windows-latest): success   MET
Suite:      cargo test --workspace --no-fail-fast → 207 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s)) · layers → (from P6-03)
Snags:      none
```

```
Commit:     P6-03 0bfbdbb8386755b122b9b88d54c6efa0484cc65e
Done-when:  cargo xtask layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)          MET
            layers fixture: joinn-visual -> joinn-gpu refused (ok)
            layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
            cargo xtask modules → modules: ok (enforced 14 crate(s))
            cargo xtask vocab → vocab: ok
            Shown then reverted (joinn-gpu = { path = "../joinn-gpu" } added to crates/joinn-visual/Cargo.toml):
            cargo xtask layers → error: cyclic package dependency: package `joinn-gpu v0.1.0 (D:\JoInn\joinn\crates\joinn-gpu)` depends on itself. Cycle: (exit 101; Cargo refuses before xtask runs)
            the xtask binary built before the edit, run directly with `layers`:
            layers fixture: joinn-visual -> joinn-gpu refused (ok)
            layers: 2 hit(s)
            layers: joinn-visual -> joinn-gpu is not an edge of the plan's §2.1 table; acceptance is joinn-visual depending on joinn-frame, joinn-dna, joinn-link, joinn-host, joinn-live
            layers: joinn-visual -> joinn-gpu: a crate above the renderer boundary reaches joinn-gpu; acceptance is no path from above the boundary to joinn-gpu or joinn-shell-desktop
            (exit 1)
            After the revert: cargo xtask layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop) (exit 0)
Suite:      cargo test --workspace --no-fail-fast → 210 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      P6-03: the shown-then-reverted edit makes a Cargo dependency cycle (joinn-gpu already depends on joinn-visual), so `cargo xtask layers` cannot build and Cargo prints `error: cyclic package dependency` (exit 101). The layers refusal was shown by running the xtask binary built before the edit.
```

```
Commit:     P6-04 e0e8a8a3e415083cbf125c3933bbf3e0b1e736b7
Done-when:  cargo test --workspace --no-fail-fast → 211 passed, 0 failed (no existing test changed)   MET
            cargo xtask gate all → exit 0; phase 0: pass … phase 5.2: 3/3, every line unchanged; last line "gate all wall milliseconds: 343091"   MET
            cargo xtask assay agree → "assay agree: 38 subject(s) agree, 1 not measured; injected disagreements: 2 refused as truth violation (ok)"; diff against the pre-change output: 0 lines   MET
Suite:      cargo test --workspace --no-fail-fast → 211 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      §2.1 names the element type Port; joinn-link's type is PortDecl, so the map holds Vec<PortDecl>.
```

```
Commit:     P6-05 6bdafa684e57067eeef838dfa5f01a9933345c64
Done-when:  cargo xtask layout phase2/calculator.body → §2.3's block and §2.4's four camera lines byte for byte; test layout_text::the_calculator_prints_the_plan_block_and_four_cameras passes   MET
            cargo xtask layout --all → 200 lines, 14 bodies laid out, 10 not measured (pasted in the commit message)   MET
            wire cycle built in a test → "layout: wire cycle through cli_a, sum; acceptance is a body whose wires form no cycle"   MET
Suite:      cargo test --workspace --no-fail-fast → 220 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      P6-05: 9 phase21/phase22 reference bodies print not measured. Their wires run from a port that is an in-port in the contract (a declared turn: cell 6fcb… as differ, c45) or into the self instance's out-port (the nested boundary). §2.3 draws a wire from an out-port to an in-port. The plan's risk table says such a body is printed not measured and recorded, not fixed.
            P6-05: floor primitives declare an in-port and an out-port at one position (eq: in 0, in 1, out 0). The first --all run refused every such wire because a port was looked up by position alone; layout now takes a wire's source among out-ports and its destination among in-ports. §2.8's ID names a port by position, so P6-07's scene refuses a body with two ports at one position instead of merging them.
```

`cargo xtask layout phase2/calculator.body`:

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

The `not measured` lines of `cargo xtask layout --all` (the whole output is in P6-05's commit message):

```
phase21/int_add_ref.body: not measured: layout: wire pickz@0 -> self@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/int_format_ref.body: not measured: layout: wire mkchr@0 -> self@1 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/int_mul_ref.body: not measured: layout: wire differ@0 -> picks@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/rat_add_ref.body: not measured: layout: wire self@0 -> self@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/text_parse_ref.body: not measured: layout: wire c@1 -> self@1 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/int_format_ref.body: not measured: layout: wire a48@2 -> d45@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/int_mul_ref.body: not measured: layout: wire differ@0 -> picks@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/rat_add_ref.body: not measured: layout: wire pick@0 -> self@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/text_parse_ref.body: not measured: layout: wire c45@0 -> isdash@1 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase52/controls/missing_cell.body: not measured: instance orphan cell 4a37 was not supplied; acceptance is that cell in the cell map
```

```
Commit:     P6-06 cbfc7de800612feb6dfd8f92b5df1662470425e7
Done-when:  cpu_pick = cpu_pick_reference on every pixel, edge counts printed → the four lines below   MET
            §2.13's table on the CPU → pick::owner_of_layout::tests::the_probe_pixels_name_the_plan_owners_on_the_cpu ok   MET
            classifier unit tests: exactly on an edge → edge (MET); 2/16 px inside → inside (MET); 1/16 px inside a straight edge → plan says edge, machine says inside   NOT MET as worded (see snag)
Suite:      cargo test --workspace --no-fail-fast → 237 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      P6-06: the done-when wants a pixel 1/16 px inside a straight edge judged edge. §2.5 defines inside as d ≤ −1 in sixteenths, and 1/16 px inside is d = −1, so §2.5 makes it inside. The tests assert §2.5's rule. Plan: edge; machine: inside (round_rect_class((319,0),(0,0),(320,160),64) = Inside; circle_class((63,0),(0,0),64) = Inside; capsule_class at 1/16 inside = Inside). The edge band holds only d = 0.
            P6-06: vocab flagged three test names that used a banned word; they were renamed to say what they assert (…_is_inside).
```

`cargo test -p joinn-visual -- --nocapture` at P6-06:

```
calculator 640x360: agree 230400, edge 184, disagree 0, owners 13/13
calculator 1000x777: agree 777000, edge 232, disagree 0, owners 13/13
calculator 1280x720: agree 921600, edge 312, disagree 0, owners 13/13
calculator 1920x1080: agree 2073600, edge 456, disagree 0, owners 13/13
test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```
Commit:     P6-07 bd638b6544308c585837f84ab9f2718ba4b7dc3a
Done-when:  the script: after every event table_bytes equal regrow's, each delta within Σ(1 + ports) → the three event lines below   MET
            sum@2 filled and no cell refused at the end → asserted   MET
            camera change → table_bytes unchanged, take_pending None   MET
            take_pending None when idle → asserted after grow and after every draw   MET
            replace to DropGenome(cli_b) → 5 row(s) (§2.7 predicts 5: body, cli_b's cell, cli_b@0, cli_b@1, link 1)   MET
            stale cli_b ID [1,2,0,1] → "stale pick: cell slot 1 generation 1, now 2; acceptance is a pick taken from the current picture"; old cli_a ID → body.cli_a   MET
            after replace, every pixel's owner = a fresh grow's at every standard viewport → 0 differ at all four   MET
Suite:      cargo test --workspace --no-fail-fast → 243 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      P6-07: a body whose cell declares an in-port and an out-port at one position (every floor primitive) is refused by the scene: "scene: a@0 names an in-port and an out-port; acceptance is a body whose cells give each position one port, since an ID names a port by position". §2.8 encodes a port as PORT_TAG | position and names it by Address, so two such ports cannot be told apart. The calculator is unaffected. A test holds the refusal.
```

`cargo test -p joinn-visual -- --nocapture` at P6-07:

```
event 0 cli_a@0 <- "two": touched cli_a; 2 row(s), bound 3: Cell 0, Port 0
event 1 cli_a@0 <- "2": touched cli_a, sum; 3 row(s), bound 7: Cell 0, Port 1, Port 4
event 2 cli_b@0 <- "3": touched cli_b, sum; 4 row(s), bound 7: Port 2, Port 3, Port 5, Port 6
replace to DropGenome(cli_b): 5 row(s): Body 0, Cell 1, Port 2, Port 3, Link 1
after replace 640x360: 230400 pixels, 0 owner(s) differ from a fresh grow
after replace 1000x777: 777000 pixels, 0 owner(s) differ from a fresh grow
after replace 1280x720: 921600 pixels, 0 owner(s) differ from a fresh grow
after replace 1920x1080: 2073600 pixels, 0 owner(s) differ from a fresh grow
test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Last line of each command (end of chunk A, on bd638b6)

- `cargo test --workspace --no-fail-fast` — `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s` (the last doc-test binary); summed over every `test result:` line: `243 passed, 0 failed` (exit 0)
- `cargo xtask gate all` — `gate all wall milliseconds: 356983` (exit 0)
- `cargo xtask corpus verify` — `corpus verify: 43 hash(es) match; cells admitted` (exit 0)
- `cargo xtask vocab` — `vocab: ok` (exit 0)
- `cargo xtask modules` — `modules: ok (enforced 14 crate(s))` (exit 0)
- `cargo xtask layers` — `layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)` (exit 0)
- `cargo xtask assay agree` — `assay agree: 38 subject(s) agree, 1 not measured; injected disagreements: 2 refused as truth violation (ok)` (exit 0)

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

none (suite on bd638b6: 243 passed, 0 failed)

## CI

Newest run after the push of `bd638b6` (P6-03 to P6-07), read with `Invoke-RestMethod` on `https://api.github.com/repos/joneseysinno/JoInn/actions/runs?branch=main&per_page=3` and that run's `jobs_url`.

```
head_sha: bd638b6544308c585837f84ab9f2718ba4b7dc3a
status: completed
conclusion: success
check (ubuntu-24.04): success
check (windows-latest): success
```

Run: https://github.com/joneseysinno/JoInn/actions/runs/36489023743

The run for P6-02's own push (`7cf7d0a`), read the same way:

```
head_sha: 7cf7d0a6404cd83733fa3876de911548dd8aa6ff
status: completed
conclusion: success
check (ubuntu-24.04): success
check (windows-latest): success
```

Run: https://github.com/joneseysinno/JoInn/actions/runs/36484809177

## Snags (all from the chunk)

- **P6-03**: the plan asks to show `layers` refusing an added `joinn-visual → joinn-gpu` edge, then revert it. That edge makes a Cargo dependency cycle (joinn-gpu already depends on joinn-visual), so `cargo xtask layers` cannot build: `error: cyclic package dependency: package `joinn-gpu v0.1.0 (D:\JoInn\joinn\crates\joinn-gpu)` depends on itself.` (exit 101). The refusal was shown with the xtask binary built before the edit: `layers: 2 hit(s)`, both lines beginning `layers: joinn-visual -> joinn-gpu`. The in-tree fixture `layers fixture: joinn-visual -> joinn-gpu refused (ok)` runs on every `cargo xtask layers`.
- **P6-04**: §2.1 names the element type `Port`; joinn-link's type is `PortDecl`, so `instance_ports` returns `BTreeMap<String, Vec<PortDecl>>`.
- **P6-05**: 9 phase21/phase22 reference bodies print `not measured` in `layout --all` (listed above). Their wires run from a port that is an in-port in the contract (a declared turn: cell `6fcb…` as `differ`, `c45`) or into the `self` instance's out-port (the nested boundary). §2.3 draws a wire from an out-port to an in-port. The plan's risk table says such a body is printed `not measured` and recorded, not fixed.
- **P6-05**: floor primitives declare an in-port and an out-port at one position (`eq`: in 0, in 1, out 0). The first `--all` run refused every such wire because a port was looked up by position alone. Layout now takes a wire's source among out-ports and its destination among in-ports (test `a_wire_joins_the_out_port_and_in_port_that_share_a_position`).
- **P6-06**: the done-when wants a pixel 1/16 px inside a straight edge judged `edge`. §2.5 defines inside as d ≤ −1 in sixteenths, and 1/16 px inside is d = −1, so §2.5 makes it `inside`. The tests assert §2.5's rule. Plan: edge; machine: inside. The edge band holds only d = 0.
- **P6-06**: vocab flagged three test names that used a banned word; they were renamed to say what they assert (`…_is_inside`).
- **P6-07**: the scene refuses a body whose cell declares an in-port and an out-port at one position (every floor primitive): `scene: a@0 names an in-port and an out-port; acceptance is a body whose cells give each position one port, since an ID names a port by position`. §2.8 encodes a port as `PORT_TAG | position` and names it by `Address`, so two such ports cannot be told apart. The calculator is unaffected. A test holds the refusal.
