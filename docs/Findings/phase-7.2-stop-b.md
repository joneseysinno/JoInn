# Phase 7.2 Stop B: end of chunk B (P72-03a to P72-10a)

Recorded by Cursor on 5 Oct 2026 from `D:\JoInn` on Windows (PowerShell).
Values below are copied from the terminal, not summarised.

`git rev-parse HEAD` after P72-10a (last commit of chunk B):

```
c2eeb303e100cc594da71d62a87820f86d1c383b
```

The stop-report commit that adds this file is `P72-stop-b` on `main` immediately after that.

Every commit's suite, scans and done-when commands were run on that commit's tree before it was committed. P72-10a was added at this stop after CI's `fmt` step refused 8a2acb1 (see Snags).

## Commit reports (in order)

```
Commit:     P72-03a ae2c64322930a5dd6069c64747c9093494dff359
Done-when:  cargo xtask grove → grove seed 7: 8 galaxies, 128 systems, 3072 bodies (calc 1181, units 982, bus 909), 137 links, 1182 members, admitted; hash 068428e1a3663079efc55bf76b097b0e28108713f6fbd361a8234d2402500ab2          MET
            cargo test -p xtask grove → test result: ok. 11 passed; 0 failed (seed_7_is_admitted_with_the_plan_s_counts passes)
            cargo xtask zoom → zoom: 15 views, cut counts printed; touches 264 and 1182 (the block byte for byte, by the xtask zoom test)
Suite:      cargo test --workspace --no-fail-fast → 380 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Zoom:       cargo xtask zoom → zoom: 15 views, cut counts printed; touches 264 and 1182
Snags:      none
```

```
Commit:     P72-06 366c49a0b8f253d89ecd9e0a83f7c977d35ac9af
Done-when:  cargo test -p joinn-visual font → test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 73 filtered out          MET
            (every glyph inside its 6x10 box; no two glyphs share a stroke set; 'S', '!', 'é' refused "text: '<c>' has no glyph; acceptance is a character from the stroke set"; the s of b0000's sum label spans sixteenths x 400..412, y 76..92 = x 25 .. 25 3/4, y 4 3/4 .. 5 3/4; value_text("12345") = "123…")
Suite:      cargo test --workspace --no-fail-fast → 391 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Zoom:       cargo xtask zoom → zoom: 15 views, cut counts printed; touches 264 and 1182
Snags:      none
```

```
Commit:     P72-07 b6c5abf2c48b88851610746e97bee4137b670f42
Done-when:  cargo test -p xtask zoom_script -- --nocapture → zoom script: 86 notches in and out; rebases [("focus s", 3209)]; 3209 charts · test result: ok. 1 passed; 0 failed          MET
            (every pan and zoom of the grove's zoom script leaves 0 rows pending; the frame -> s rebase writes exactly 3209 chart rows = the chart count and nothing else; the tables after the script equal a fresh grow at the final anchor byte for byte)
            cargo test -p joinn-visual universe_scene → test result: ok. 3 passed; 0 failed
            cargo test -p joinn-visual a_single_body_scene_has_one_chart_row → ok (one chart row: origin 0, parent 0, kind body, size 40)
            row_bytes tests untouched and passing; chart_frame_and_stroke_rows_are_little_endian_in_field_order added
Suite:      cargo test --workspace --no-fail-fast → 397 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Zoom:       cargo xtask zoom → zoom: 15 views, cut counts printed; touches 264 and 1182
Snags:      a single-body scene (Scene) holds its one chart row and no stroke rows. Label rows would enter Phase 6's replace delta (replace_to_drop_cli_b_writes_five_rows_and_stales_its_old_id asserts 5 rows), and value rows written by Scene::present would change Phase 7.1's regrow lines (rows and bytes per event, and the V121 bound), which P72-08 requires unchanged. Text (titles, labels, values) is drawn by the universe scene; UniverseScene::present writes the value strokes. The renderer skips chart, frame and stroke rows until P72-08 binds them.
```

```
Commit:     P72-08 7270ca3329731431eb8d3e67fbf8ba01263afbd1
Done-when:  cargo xtask pick; cargo xtask regrow; Compare-Object against the 366c49a captures → pick : base 101 lines, now 101 lines, differing 0 · regrow : base 22 lines, now 22 lines, differing 0          MET
            (both sides normalized the same way: cargo's own Compiling/Finished/Running lines dropped, the adapter separator U+00B7 folded to '?' because the two captures were written in different console encodings, and the three "frame median N ms (information, not a check)" timing lines dropped; every other line is byte-identical)
            cargo xtask gate 6 → 1 ok · 2 ok · 3 ok · phase 6: 3/3 (exit 0)
            cargo xtask gate 7 → 1 ok · 2 ok · 3 ok · phase 7: 3/3 (exit 0)
            cargo test -p joinn-gpu tick_bytes → the_anchor_origin_splits_into_whole_and_fraction_and_overflow_is_refused ok (origin 636.625 px -> 636 + 0xA0000000; a level-9 focus 2^22 units out is refused; a chart 2^22 units from the anchor at level 9 is refused)
Suite:      cargo test --workspace --no-fail-fast → 398 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      the vertex stage has room for eight storage buffers and the universe group now holds eight tables (body, cell, port, link, incidence, chart, frame, stroke), so the style table moved from a storage buffer to a 64-byte uniform (sixteen styles as four vec4<u32>); its row count and the upload byte counts are unchanged. Frames (open systems and galaxies, and lens nodes) switch between the frame and node styles at the summary threshold without a crossfade: only body elements fade, through the ghost pass. The FitCamera entry points (draw, frame, picture) are kept and convert through Camera::from_fit, so gates 6 and 7 draw through the converted camera without their callers changing.
```

```
Commit:     P72-09 d9cbba92b0275f6d2b47052956c203f186f54599
Done-when:  cargo xtask pick → exit 0; every grove line disagree 0 and owners n (cut allows n) with equal n, on all three adapters          MET
            (frame 3208 = 3208 · s level -4 136 = 136 · -3 3208 · -2 2342 · -1 904 · 0 939 · 1 452 · 2 141 · 3 99 · 4 44 · 5 19 · 6 10 · 7 4 · 8 4 · 9 3; beyond the 4096-pixel sample, every view's whole 1920x1080 image is also compared and has 0 disagreeing pixels, and the brute-force walk names every sampled pixel as the grid does)
            cargo test -p joinn-visual → 94 passed (the calculator grid/reference agreement and probe tests unchanged and passing at 2^-32 px; the_grid_and_the_reference_agree_on_every_pixel_from_the_galaxy_to_level_9 on the phase 5 universe; a 256-bit capsule comparison past 2^128)
            (the whole pick output is pasted in the commit message; its grove lines, identical on all three adapters:)
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
            pick: 3 adapter(s), 12 subject(s) agree; planted disagreement: refused as truth violation (ok)
Suite:      cargo test --workspace --no-fail-fast → 403 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      at 2^-32 px a wire's squared cross product reaches about 2^216, past i128, so the capsule's middle test compares c² against (w ± 2^28)²·L in 256 bits (mul_wide); circles and corners stay in i128. A radius under the edge band holds nothing inside (Phase 6 radii never were). cpu_pick and cpu_pick_reference keep their FitCamera signatures and convert through Camera::from_fit to the new cpu_pick_at and cpu_pick_reference_at; cpu_pick_sample is the brute-force walk at chosen pixels, which the grove's 4096-pixel reference check uses. The sample is SplitMix64 from seed 7, one draw per pixel: x from the low 32 bits mod 1920, y from the high 32 bits mod 1080. "owners" counts the GPU's owners of pixels the CPU doesn't call edge; "cut allows" counts the owners the cut's shapes give a pixel.
```

```
Commit:     P72-10 8a2acb1a06f6a4fe1fe170edfa880e05ff959ece
Done-when:  cargo xtask regrow → exit 0; 174 grove steps, 174 "tables equal regrow"; one rebase (step 1, focus s: rows 3209, universe -> b0000), color identical, ids identical on all three adapters          MET
            Shown then reverted: UniverseLayout::rebase rounding the new focus down to whole layout units ((focus - d·2^16).div_euclid(2^16)·2^16); cargo xtask regrow → exit 1, the lines that fail:
              rebase universe -> b0000 Microsoft Basic Render Driver · Dx12 · Cpu · vertex storage yes: color differ, ids differ
              rebase universe -> b0000 NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes: color differ, ids differ
              rebase universe -> b0000 NVIDIA GeForce RTX 2080 · Vulkan · DiscreteGpu · vertex storage yes: color differ, ids differ
              rebase universe -> b0000 Microsoft Basic Render Driver · Dx12 · Cpu · vertex storage yes: a rebase moved a pixel (V143)
              rebase universe -> b0000 NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes: a rebase moved a pixel (V143)
              rebase universe -> b0000 NVIDIA GeForce RTX 2080 · Vulkan · DiscreteGpu · vertex storage yes: a rebase moved a pixel (V143)
            The rebase check draws the tables before the rebase through the camera before it, then applies the rebase's delta (chart rows only) to the same renderer and draws through the rebased camera.
            (the whole regrow output is pasted in the commit message; its new lines:)
              grove step 0 frame: rows 0, universe, tables equal regrow
              grove step 1 focus s: rows 3209, b0000, tables equal regrow
              grove step 2 in: rows 0, b0000, tables equal regrow  … through step 87 in, then steps 88 … 173 out, each rows 0, b0000, tables equal regrow
              rebase universe -> b0000 Microsoft Basic Render Driver · Dx12 · Cpu · vertex storage yes: color identical, ids identical
              rebase universe -> b0000 NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes: color identical, ids identical
              rebase universe -> b0000 NVIDIA GeForce RTX 2080 · Vulkan · DiscreteGpu · vertex storage yes: color identical, ids identical
              regrow: 3 adapter(s); planted difference: refused (ok)
Suite:      cargo test --workspace --no-fail-fast → 403 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      none. zoom_script is no longer test-only: regrow uses it.
```

```
Commit:     P72-10a c2eeb303e100cc594da71d62a87820f86d1c383b
Done-when:  cargo fmt --all -- --check → exit 0 (before: "Diff in" for bind_groups.rs, shader_source.rs, tick_bytes.rs, capsule_class.rs, circle_class.rs, cpu_pick_at.rs, round_rect_class.rs, shapes_at.rs, grove_pass.rs); cargo clippy --workspace --all-targets -- -D warnings → exit 0          MET
Suite:      cargo test --workspace --no-fail-fast → 403 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Zoom:       cargo xtask zoom → zoom: 15 views, cut counts printed; touches 264 and 1182
Snags:      CI run 37366413288 (head 8a2acb1) failed at its fmt step on windows-latest and cancelled ubuntu-24.04, so clippy, test, pick, regrow and gate all did not run in CI on chunk B. The per-commit checks ran the suite and the scans but not cargo fmt --check; from this commit on, they also run fmt and clippy as CI does. Layout only: no token changed.
```

## Last line of each command (end of chunk B)

Run on 8a2acb1's tree (P72-10). P72-10a changes whitespace only; on c2eeb30 the suite printed `403 passed, 0 failed`, fmt and clippy exit 0, and the three scans as above.

- `cargo test --workspace --no-fail-fast`: summed over every `test result:` line, `403 passed, 0 failed` (exit 0)
- `cargo xtask gate all` (working tree; `git status --short -- joinn/gates.lock` printed nothing afterwards): `gate all wall milliseconds: 571431` (exit 0)
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
```

Every score equals Stop A's.

### Every `gate all` line containing `fail`

```
1 ok  references agree; a blind seal fails the item
```

### Every failing test name

None. The workspace has 0 failing tests (Stop A's `seed_7_is_admitted_with_the_plan_s_counts` passes since P72-03a), and the spike has 0 (54 passed).

## CI

Read with `Invoke-RestMethod` on `https://api.github.com/repos/joneseysinno/JoInn/actions/runs?branch=main&per_page=3` and each run's `jobs_url`:

```
run 37372041109 head_sha: c2eeb303e100cc594da71d62a87820f86d1c383b status: queued conclusion:  created: 10/05/2026 20:48:41
run 37366413288 head_sha: 8a2acb1a06f6a4fe1fe170edfa880e05ff959ece status: completed conclusion: failure created: 10/05/2026 19:54:30
run 37169295468 head_sha: 3c9ec3764a7dbdb9e9b44046aef34ee9959feed1 status: completed conclusion: failure created: 10/04/2026 01:52:00
```

- Run 37366413288 (8a2acb1, P72-10): `check (windows-latest): completed failure` at step 5 `fmt` (clippy through gate all skipped); `check (ubuntu-24.04): completed cancelled`. P72-10a fixes this.
- Run 37169295468 (3c9ec37, the Amendment 1 plan commit on top of P72-stop-a): both jobs failed at step 7 `test`. That is Stop A's known failing test, `seed_7_is_admitted_with_the_plan_s_counts`, which P72-03a fixed.
- Run 37372041109 (c2eeb30, P72-10a): `check (windows-latest): in_progress` (toolchain step), `check (ubuntu-24.04): queued` when this report was written. Its result is read at Stop C.

Newest run: https://github.com/joneseysinno/JoInn/actions/runs/37372041109

## Snags (all from the chunk)

- **P72-07**: a single-body scene (Scene) holds its one chart row and no stroke rows; text is drawn by the universe scene only, so Phase 6's replace delta and Phase 7.1's regrow lines stay unchanged.
- **P72-08**: the style table moved from a storage buffer to a 64-byte uniform (eight tables fill the vertex stage's storage slots); frames switch frame/node style at the summary threshold without a crossfade; the FitCamera entry points convert through `Camera::from_fit`.
- **P72-09**: the capsule's middle test needs 256-bit products at 2^-32 px (`mul_wide`); the 4096-pixel sample is SplitMix64 from seed 7 (x low 32 bits mod 1920, y high 32 bits mod 1080); "owners" excludes pixels the CPU calls edge.
- **P72-08, P72-09, P72-10**: the commit messages have no `Zoom:` line, which the run asks for from P72-05 on. Each commit's suite includes `fns::zoom` tests that assert §2.12's block byte for byte, and they passed; `cargo xtask zoom` at this stop prints `zoom: 15 views, cut counts printed; touches 264 and 1182`.
- **P72-10a**: CI's `fmt` step refused 8a2acb1 (run 37366413288), so CI ran nothing past fmt on chunk B. The per-commit checks did not include `cargo fmt --check`; they now run fmt and clippy as CI does.

Run:        chunk 1 of 11 · ledger line written · tripwires: none
Next:       7.2 C
