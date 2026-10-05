# Phase 7.2 Stop C: end of chunk C (P72-11 to P72-15)

Recorded by Cursor on 5 Oct 2026 from `D:\JoInn` on Windows (PowerShell).
Values below are copied from the terminal, not summarised.

`git rev-parse HEAD` after P72-15 (last commit of chunk C):

```
ac150415eba9dbd50a2c8500d2060788c24b4b32
```

The stop-report commit that adds this file is `P72-stop-c` on `main` immediately after that.

Every commit's suite, scans and done-when commands were run on that commit's tree before it was committed. P72-15 was first committed as `17e5160` with a placeholder message; its done-when commands and the stop commands below ran on that tree, and the commit was then amended to `ac15041` with the report block only (`git diff --stat 17e5160 ac15041` prints nothing).

## Commit reports (in order)

```
Commit:     P72-11 24bef998dccbce3a7dd5966a36ce6df30476f420
Done-when:  cargo test -p xtask shell_session -- --nocapture → test result: ok. 2 passed; 0 failed          MET
            the_grove_frames_zooms_pans_and_resizes_then_idles (open the grove at 1920x1080, frame, 8 notches in about (960, 540), a drag of (10, 0), a resize to 1280x720), its tick lines:
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
            every pan and zoom rows 0; after the last input take_tick is None twice (V147); a 3 px move is not yet a drag
            a_click_names_its_owner_and_a_zoom_past_level_9_is_refused → "pick 0,0: background (cpu)", then "refused: zoom: level 10 is outside −4 … 9; acceptance is a level from −4 to 9"
            cargo test -p joinn-shell-desktop → 12 passed (the calculator's probe clicks and V125 run path unchanged on the exact camera; phase5/universe.universe opens lens deployment by default, function by --lens, "layout: lens nowhere not found; acceptance is a lens the universe declares"; an empty store: "alias calc declared b55fba1e…c3ebde and the store holds no such body")
            cargo test -p joinn-visual print_id → 1 passed (background, galaxy app, system measurement, calc surface, calc.sum, calc.sum@2, calc wire cli_a@1 -> sum@0, an unknown ID refused)
            Window, once on Windows: cargo xtask grove --out target/grove.universe; target\release\joinn-desktop.exe target/grove.universe, a click posted to the JoInn window at client (640, 360) (PostMessage WM_MOUSEMOVE, WM_LBUTTONDOWN, WM_LBUTTONUP; then WM_CLOSE). First lines:
              adapter: NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes
              surface: Bgra8Unorm
              pick 640,360: background (cpu)
              tick: level -3 step 208 (k 29/128), anchor universe, rows 156048
              background (gpu) · agree
Suite:      cargo test --workspace --no-fail-fast → 408 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Zoom:       cargo xtask zoom → zoom: 15 views, cut counts printed; touches 264 and 1182
Snags:      the window's idle could not be read. In both runs the window received input Cursor did not send: run 1 printed 28 more ticks in the 5 s after the click (wheel notches from level -3 step 208 to level 0 step 208, and same-zoom ticks); run 2, with the window moved away from the system cursor (read with GetCursorPos, never moved), printed 66 ticks of drags and notches and no pick line (winit captures the mouse on a press, so the real mouse's moves became a drag). V147 rests on the no-window test. A resize can rebase (the 1280x720 resize moved the centre pixel into g1s10 and wrote 3209 chart rows); it is not a pan or zoom. The shell's test lives in xtask, which has the grove's generator: joinn-shell-desktop is an xtask [dev-dependencies] entry (layers reads [dependencies] only). The tick line no longer counts ticks ("tick <n>: rows, bytes" became §2.11's line). A press now picks on release (moved < 4 px), at the press pixel. With a port selected, + - F type into it.
```

```
Commit:     P72-12 0875fc4bdc72c5c04019d01d334752c4f904f077
Done-when:  cargo run --release -p xtask -- zoom --measure → output below (no threshold; the numbers go to the findings)          MET
              measure frame level -2 step 95 (k 351/1024): cut cpu 60 µs (3208 entries), draw instances 302352
              measure at s level -4 step 0 (k 1/16): cut cpu 22 µs (136 entries), draw instances 302352
              measure at s level -3 step 0 (k 1/8): cut cpu 66 µs (3208 entries), draw instances 302352
              measure at s level -2 step 0 (k 1/4): cut cpu 50 µs (2342 entries), draw instances 302352
              measure at s level -1 step 0 (k 1/2): cut cpu 33 µs (904 entries), draw instances 302352
              measure at s level 0 step 0 (k 1): cut cpu 32 µs (283 entries), draw instances 302352
              measure at s level 1 step 0 (k 2): cut cpu 22 µs (85 entries), draw instances 302352
              measure at s level 2 step 0 (k 4): cut cpu 21 µs (26 entries), draw instances 302352
              measure at s level 3 step 0 (k 8): cut cpu 27 µs (14 entries), draw instances 302352
              measure at s level 4 step 0 (k 16): cut cpu 20 µs (6 entries), draw instances 302352
              measure at s level 5 step 0 (k 32): cut cpu 19 µs (4 entries), draw instances 302352
              measure at s level 6 step 0 (k 64): cut cpu 19 µs (3 entries), draw instances 302352
              measure at s level 7 step 0 (k 128): cut cpu 19 µs (3 entries), draw instances 302352
              measure at s level 8 step 0 (k 256): cut cpu 19 µs (3 entries), draw instances 302352
              measure at s level 9 step 0 (k 512): cut cpu 19 µs (3 entries), draw instances 302352
              zoom --measure: 15 views; tables frame 136, body 3072, cell 5434, link 2362, port 13031, stroke 124069
            cargo xtask zoom --bogus → zoom: arguments ["--bogus"]; acceptance is none, or --measure (exit 1)
            cargo xtask zoom (no flag) → unchanged, last line zoom: 15 views, cut counts printed; touches 264 and 1182
Suite:      cargo test --workspace --no-fail-fast → 408 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Zoom:       cargo xtask zoom → zoom: 15 views, cut counts printed; touches 264 and 1182
Snags:      "cut cpu" is the least of five runs of joinn_visual::cut per view (Windows, release). "draw instances" counts every row of every drawn kind through the owner and ghost pipelines (2 × (frames + 2 × bodies for dot and surface + cells + links + ports + strokes)); the GPU draws them all and the shader decides per instance, so the number is the same at every view. The per-view cut entries are printed beside the time. The measurement uses Instant with #[allow(clippy::disallowed_methods)] and rule 4's note, as frame_median and gate_all do.
```

```
Commit:     P72-13 2f90aa3a5a1ef775b5e356893f44ba59e86a81b6
Done-when:  every prediction marked          MET
              §2.2 frame (level -2 step 95, k 351/1024): as predicted
              §2.4 counts (8 galaxies, 128 systems, 3072 bodies: calc 1181, units 982, bus 909; 137 links, 1182 members): as predicted
              §2.5 last line (layout universe: 8 galaxies, 128 systems, 3072 bodies, 5504x1600): as predicted
              §2.12 17 lines: as predicted, every line
              §2.13 touches (264 at level -4 step 0, 1182 at the frame): as predicted
              The adversary: the CPU cut does not run per tick; it runs per view in 19-66 µs (release), and the GPU draws 302352 instances per tick at every view.
              What 7.3 needs: 6 lines, observed.
              Appendix: grove, layout (head and tail), zoom, zoom --measure, the shell_session test, pick, regrow, re-run at 0875fc4.
Suite:      cargo test --workspace --no-fail-fast → 408 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Zoom:       cargo xtask zoom → zoom: 15 views, cut counts printed; touches 264 and 1182
Snags:      the window lines in the findings are P72-11's run; no window was run for P72-13 (rule 67: both earlier window runs received input Cursor did not send). fmt and clippy exit 0.
```

```
Commit:     P72-14 277e104251c24a314e853ded3ae2250687d92e5f
Done-when:  cargo xtask gate all exits 0 and prints phase 7.2: 3/3 after phase 7: 3/3          MET
              gate all exit 0 · ... phase 6: 3/3 · phase 7: 3/3 · phase 7.2: 3/3 · gate all wall milliseconds: 1237857
              gates.lock gains "phase 7.2: 3/3", written by the lock writer (git diff: one added line)
              cargo xtask gate 7.2 → 1 ok  The zoom is exact, and a rebase moves nothing · 2 ok  Bands follow size, and two pickers name every pixel · 3 ok  A folded system is one node, and a link touches it once · phase 7.2: 3/3
            Shown then reverted:
              (a) item 2 given item 3's opposes → cargo xtask gate 7.2 exit 1:
                  distinct opposition: Bands follow size, and two pickers name every pixel and A folded system is one node, and a link touches it once share (corpus/phase5/adversary.universe, ShiftPort("bus", "calc.sum@2", 9))
              (b) the shader's owner test (common.wgsl past()) using > instead of >= → cargo xtask gate 7.2 exit 0: 1 ok · 2 ok · 3 ok · phase 7.2: 3/3. NOT caught (see Snags).
              (c) chart rows written on a plain zoom (UniverseScene::rebase returning every chart row when the anchor stays) → cargo xtask gate 7.2 exit 1:
                  grove step 0 frame: a pan or zoom at universe wrote 3209 row(s); acceptance is 0 (V142)
                  grove step 2 in: a pan or zoom at b0000 wrote 3209 row(s); acceptance is 0 (V142)
                  ... one line per plain step, through ...
                  grove step 173 out: a pan or zoom at b0000 wrote 3209 row(s); acceptance is 0 (V142)
                  1 fail  The zoom is exact, and a rebase moves nothing · 2 ok · 3 ok · phase 7.2: 2/3
            Items: xtask/src/fns/gate_seven_two_items.rs; checks g72_zoom (g72_refusals: the 7 camera refusals word for word; grove_pass: the zoom script on every adapter, now also failing a pan or zoom that writes a row; g72_reversible: 8 in and 8 out about (700, 300), camera and bytes on every adapter; the calculator's Phase 6 cameras convert to their whole k; g6_owners, g6_picture, g7_picture pass), g72_bands (zoom_text equals the §2.12 block; one fade-window view per threshold, drawn in its owner band and fading: fade 4: b0003 level -3 step 195, fade 32: b0000 level -1 step 195, fade 240: b0000 level 2 step 167; cpu_pick = reference on 4096 seeded pixels, GPU = cpu_pick off edges, owners = cut, on every adapter), g72_folded (level -4: 128 system nodes, no body drawn, 264 touches; frame 1182; adversary.universe framed at level -4: three systems each one node, link bus touches 3).
            Uniqueness through gate 7.2: distinct_opposition, the shared-check test in main.rs, g3_artifacts and the contact-artifact test include gate 7.2's table. A test asserts only g72_* gate leaves name the grove. cargo xtask gate 7.2 added. CI: new step zoom after contact on both jobs: cargo xtask grove && cargo xtask zoom.
            pick and regrow: grove_lines and grove_pass return their lines (the commands print them unchanged); pick also prints the three fade views per adapter.
Suite:      cargo test --workspace --no-fail-fast → 412 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Zoom:       cargo xtask zoom → zoom: 15 views, cut counts printed; touches 264 and 1182
Snags:      (1) corpus/phase5/adversary.universe is refused by full admission: "link bus member units.scale@1 is head but direction is Out; acceptance is In" (check_link_types). Read literally, §2.13's controls 2 and 3 ("refused at admission, or ...") answer true on the real subject and the gate cannot grade them (gate 7.2 printed "control answered true on real subject" for items 2 and 3). Gate 7.2's controls and item 3's check admit as the picture needs: bind, check_law4, check_lenses, assemble_universe and the layout, without check_link_types (no picture reads a member's frame). A test pins it: the real adversary passes those, CopyMember is refused by check_lenses ("body units belongs to systems calculation and measurement in lens function; ..."), ShiftPort by assembly ("no such port calc.sum@9; ..."). The rows' control_artifact and opposes are as §2.13. For Claude to settle at the stop. (2) Demo (b) is not caught: >= and > differ only when 10·s = 11·T exactly, which needs size·(256+step)·2^L = 11·T·256/10; only T = 240 allows it (2^11·33), and grove bodies are 40 and 20 wide (a factor 5), so no grove body at any zoom sits there; nor does any frame (304, 1296). Sizes 8, 12, 16, 24, 32, 33 or 48 would. For Claude to settle at the stop. (3) A first try at (c) made a plain zoom call write_charts, which returns only rows whose bytes changed, so it wrote nothing and the gate passed; the plant shown above returns every chart row.
```

```
Commit:     P72-15 ac150415eba9dbd50a2c8500d2060788c24b4b32
Done-when:  cargo xtask gate all from a fresh clone prints phases 0 ... 7.2 and exits 0. corpus verify 44. git diff --stat 744f4d0..HEAD -- joinn/corpus prints nothing          MET
              fresh clone of 17e5160 (git clone D:\JoInn into %TEMP%\joinn_fresh, its own CARGO_TARGET_DIR): cargo xtask gate all exit 0 · phase 0: pass · phase 1: 4/4 legacy · phase 2: 4/4 legacy · phase 2.1: 8/8 legacy · phase 2.2: 9/9 legacy · phase 3: 8/8 legacy · phase 4: 4/4 · phase 5: 8/8 · phase 5.1: 4/4 · phase 5.2: 3/3 · phase 6: 3/3 · phase 7: 3/3 · phase 7.2: 3/3 · gate all wall milliseconds: 1314237
              same clone: cargo xtask corpus verify → corpus verify: refused false_law.cell by name · corpus verify: 44 hash(es) match; cells admitted (exit 0)
              git diff --stat 744f4d0..HEAD -- joinn/corpus → (nothing)
            Docs: Guides/05-glossary.md gains Anchor, Band, Chart, Crossfade, Cut (the), Focus, Grove (the), Lens node, Owner band, Pin, Rebase, Step, Stroke font, Touch, Zoom level (Camera, Organelle, Owner updated); Guides/03-where-we-are.md gains "The universe zooms (Phase 7.2)" (what isn't built: links drawn as hyperedges, rerouting, snapshots); README gains the universe try-it block, gate 7.2 and the grove, layout --universe, zoom, pick, regrow rows; decisions.md V141-V147 holds, each with its evidence; the roadmap's 7.2 note links docs/Findings/phase-7.2-zoom.md.
Suite:      cargo test --workspace --no-fail-fast → 412 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Zoom:       cargo xtask zoom → zoom: 15 views, cut counts printed; touches 264 and 1182
Snags:      V141 holds with the note that gate 7.2 does not catch the shader's > for >= (P72-14 snag 2). V147 holds on the no-window test; the window's idle was not observed (rule 67, P72-11 snag). fmt exit 0; clippy --workspace --all-targets -D warnings exit 0 (Finished `dev` profile). This message was amended at Stop C with these results; the tree is the one committed.
```

## Last line of each command (end of chunk C)

Run on 17e5160's tree (the same tree as ac15041), in `D:\JoInn\joinn`.

- `cargo test --workspace --no-fail-fast`: summed over every `test result:` line, `412 passed, 0 failed` (exit 0)
- `cargo xtask gate all` (working tree; `git status --short` printed nothing afterwards, so `gates.lock` is unchanged): `gate all wall milliseconds: 1242360` (exit 0)
- `cargo xtask corpus verify`: `corpus verify: 44 hash(es) match; cells admitted` (exit 0)
- `cargo xtask vocab`: `vocab: ok` (exit 0)
- `cargo xtask modules`: `modules: ok (enforced 14 crate(s))` (exit 0)
- `cargo xtask layers`: `layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)` (exit 0)
- `cargo xtask floor`: `floor: 16 members, paired` (exit 0)
- `cargo xtask forces`: `forces: 1 combine registered, 1 opposed; plants: difference refused, midpoint refused, max and plus1 order-blind but unregistered, unopposed refused (ok)` (exit 0)
- `cargo xtask contact`: `contact: 1 body(ies); two forms, one truth` (exit 0)
- `cargo xtask pick`: `pick: 3 adapter(s), 12 subject(s) agree; planted disagreement: refused as truth violation (ok)` (exit 0)
- `cargo xtask regrow`: `regrow: 3 adapter(s); planted difference: refused (ok)` (exit 0)
- `cargo test --manifest-path spikes/s8-beam/Cargo.toml`: `test result: ok. 54 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s` (exit 0)
- `cargo xtask grove`: `grove seed 7: 8 galaxies, 128 systems, 3072 bodies (calc 1181, units 982, bus 909), 137 links, 1182 members, admitted; hash 068428e1a3663079efc55bf76b097b0e28108713f6fbd361a8234d2402500ab2` (exit 0)
- `cargo xtask zoom`: `zoom: 15 views, cut counts printed; touches 264 and 1182` (exit 0)
- `cargo fmt --all -- --check`: no output (exit 0)
- `cargo clippy --workspace --all-targets -- -D warnings`: `Finished \`dev\` profile [unoptimized + debuginfo] target(s) in 0.82s` (exit 0)
- `git diff --stat 415ea22..HEAD -- joinn/corpus`: no output

Phase lines from `gate all` (the block it prints last, after the lock header):

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
phase 6: 3/3
phase 7: 3/3
phase 7.2: 3/3
```

Every earlier score equals Stop B's; phase 7.2 is new at 3/3. The fresh clone printed the same block.

### Every `gate all` line containing `fail`

```
1 ok  references agree; a blind seal fails the item
```

(The same single line in the working tree and in the fresh clone.)

### Every failing test name

None. The workspace has 0 failing tests, and the spike has 0 (54 passed).

## CI

Read with `Invoke-RestMethod` on `https://api.github.com/repos/joneseysinno/JoInn/actions/runs?branch=main&per_page=3` and each run's `jobs_url`, before this stop's push (chunk C's commits had not been pushed):

```
37372648096 4845326 completed failure 10/5/2026 8:54:46 PM
37372041109 c2eeb30 completed success 10/5/2026 8:48:41 PM
37366413288 8a2acb1 completed failure 10/5/2026 7:54:30 PM
```

- Run 37372041109 (c2eeb30, P72-10a): `check (ubuntu-24.04): completed success`, `check (windows-latest): completed success`. Chunk B is green in CI on both jobs; Stop B's fmt failure is fixed.
- Run 37372648096 (4845326, P72-stop-b): `check (windows-latest): completed success`, every step from fmt through gate all `success` (lavapipe `skipped`, as on Windows always). `check (ubuntu-24.04): completed cancelled`, started 8:54:46 PM and ended 9:09:48 PM with no steps listed: the job never ran a step. 4845326 adds only the stop report and the ledger to c2eeb30, whose Ubuntu job passed. See Snags.
- Run 37366413288 (8a2acb1, P72-10): Stop B's fmt failure, fixed by P72-10a.

Chunk C's commits (24bef99 … ac15041) and this report are pushed at this stop; their run, with the new `zoom` step, is read at Stop 7.3 A.

Newest run: https://github.com/joneseysinno/JoInn/actions/runs/37372648096

## Snags (all from the chunk)

- **P72-11**: the window's idle could not be read; the window received input Cursor did not send in both runs (rule 67), so V147 rests on the no-window test. A resize can rebase (3209 chart rows); it is not a pan or zoom. The shell's test lives in xtask (joinn-shell-desktop as an xtask dev-dependency). A press picks on release.
- **P72-12**: "cut cpu" is the least of five runs per view; "draw instances" is the same at every view because the shader decides per instance. `Instant` is used with `#[allow(clippy::disallowed_methods)]`, as frame_median and gate_all do.
- **P72-13**: no window was run for the findings (rule 67); the window lines are P72-11's.
- **P72-14 (1)**: `corpus/phase5/adversary.universe` is refused by full admission (`check_link_types`: "link bus member units.scale@1 is head but direction is Out; acceptance is In"), so gate 7.2's controls 2 and 3 and item 3's check admit without the link-type check. Pinned by a test. For Claude to settle.
- **P72-14 (2)**: the shader's `>` for `>=` plant is not caught: no grove body or frame sits at exactly 10·s = 11·T. Sizes 8, 12, 16, 24, 32, 33 or 48 would. For Claude to settle.
- **P72-14 (3)**: the first plant for (c) wrote nothing, because `write_charts` returns only changed rows; the plant shown returns every chart row.
- **P72-15**: V141 holds with the uncaught-plant note; V147 holds without the window observed.
- **Stop C**: CI run 37372648096 (4845326) cancelled its Ubuntu job before any step ran; Windows passed every step on the same commit, and c2eeb30 passed on both.

AJ's window check (§6.2) is moved to the end of the run, as the run plan says.

Tripwires: T1 `git diff 415ea22..HEAD -- joinn/corpus` prints nothing. T2 `gate all` exits 0 with phase 7.2: 3/3. T3 the suite passes (412, 0 failed). T4 gate 7.2 is committed (P72-14). T5 every earlier score equals Stop B's. T6 no snag text repeats in three consecutive commits.

Run:        chunk 2 of 11 · ledger line written · tripwires: none
Next:       7.3 A
