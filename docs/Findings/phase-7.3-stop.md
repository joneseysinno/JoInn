# Phase 7.3 stop (Run 7.3–9 §4)

## HEAD

`git rev-parse HEAD` at the stop check: `5c71b95e9eac00173f8c4c46eace0aefee126050` (P73-14).

## Commits

- P73-01 0480723 Update Phase 7.2 and 7.3 plans with amendments and performance improvements: NOT MET (committed without `AGENTS.md` and Appendix B; completed by P73-01a)
- P73-01a 5b9f0d5 AGENTS.md per Appendix A; Appendix B to the backlog, decisions.md and the roadmap; the ledger retitled: MET
- P73-S1 f082035 the dev profile: MET
- P73-S2 24d782e one build: MET
- P73-S3 bf489ad the gate harness (`--item`, ms per phase, pure work once): MET
- P73-S4 6691c9c `check` and `stop-check [--fresh]`: MET
- P73-F1 27da365 gate 7.2's controls fully admitted: MET
- P73-F2 d35ad03 the band boundary on the GPU: MET
- P73-02 688be90 the routing graph: MET
- P73-03 641b442 touch points and routes: MET
- P73-04 33ed885 V16, exactly; `cargo xtask links`: MET
- P73-05 6537378 forms; `links --forms`: MET
- P73-06 b587bc2 Route and Segment tables: MET
- P73-07 fa44434 the shader draws links: MET
- P73-08 62247c0 picking links: MET
- P73-09 252e9e0 regrow with links: MET
- P73-10 7d6b4a3 the shell draws links: MET
- P73-11 fb81be0 the measurement: MET
- P73-12 7c66518 findings: MET
- P73-13 d998122 gate 7.3: MET
- P73-14 5c71b95 docs and freeze: MET

Before P73-S1, `cargo xtask gate all > target/gate-all-7.2.txt` ran on 5b9f0d5 (the 7.2 code): exit 0, `gate all wall milliseconds: 1082128`. It is the baseline P73-S1 … S3 compared against.

## `cargo xtask stop-check --fresh`

```
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.49s
     Running `C:\Users\ajone\AppData\Local\Temp\cursor-sandbox-cache\836e5dfea01f3c32b597c40880b189c8\cargo-target\debug\xtask.exe stop-check --fresh`
stop-check: D:\JoInn\joinn\target\fresh\joinn at 5c71b95 (fresh clone)
lavapipe: sudo apt-get update && sudo apt-get install -y mesa-vulkan-drivers → skipped (Linux only)
fmt: cargo fmt --all -- --check → exit 0 (5862 ms, information)
clippy: cargo clippy --workspace --all-targets -- -D warnings →     Finished `dev` profile [optimized + debuginfo] target(s) in 59.28s (59820 ms, information)
test: cargo test --workspace --no-fail-fast → 442 passed, 0 failed (153978 ms, information)
vocab: cargo xtask vocab → vocab: ok (11638 ms, information)
modules: cargo xtask modules → modules: ok (enforced 14 crate(s)) (1431 ms, information)
layers: cargo xtask layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop) (746 ms, information)
corpus verify: cargo xtask corpus verify → corpus verify: 44 hash(es) match; cells admitted (777 ms, information)
agree: cargo xtask agree → injected disagreement: refused as truth violation (ok) (111901 ms, information)
assay agree: cargo xtask assay agree → assay agree: 38 subject(s) agree, 1 not measured; injected disagreements: 2 refused as truth violation (ok) (3690 ms, information)
pick: cargo xtask pick → pick: 3 adapter(s), 12 subject(s) agree; planted disagreement: refused as truth violation (ok) (139249 ms, information)
regrow: cargo xtask regrow → regrow: 3 adapter(s); planted difference: refused (ok) (65883 ms, information)
forces: cargo xtask forces → forces: 1 combine registered, 1 opposed; plants: difference refused, midpoint refused, max and plus1 order-blind but unregistered, unopposed refused (ok) (828 ms, information)
contact: cargo xtask contact → contact: 1 body(ies); two forms, one truth (1230 ms, information)
zoom: cargo xtask grove && cargo xtask zoom && cargo xtask links → links: 3 universes, crossings 0, each folded node touched once (7005 ms, information)
spike s8: cargo test --manifest-path spikes/s8-beam/Cargo.toml → 54 passed, 0 failed (6788 ms, information)
power: cargo xtask power → gate power: 20/20 (801 ms, information)
gate all: cargo xtask gate all → gate all wall milliseconds: 492425 (493313 ms, information)
says fail: gate all: 1 ok  references agree; a blind seal fails the item
stop-check: ok (1071383 ms, information)
```

The `says fail` line is a gate item's name, not a failure (P73-S4 snag 2). `gate all` in the same tree (P73-13, `target/gate-all-13.txt`) printed phase 0 … phase 7.2 as `gates.lock` and `phase 7.3: 3/3`.

## Predictions marked `differs`

None: every row of `phase-7.3-links.md`'s Predictions is `as predicted`.

## Snags

- **P73-01a**: P73-01 was committed without `AGENTS.md` and Appendix B and with a message that is not the plan's; P73-01a completes it instead of amending. `AGENTS.md` sits under `joinn/`; it was checked as docs only (AJ's instruction).
- **P73-S1**: with xtask at opt-level 1 the suite printed `test tests::no_two_gate_items_share_a_check_or_control ... FAILED` (`shared gate functions: ["check references agree; a blind seal fails the item / every seal can see", …]`): the linker merges p21_agree, p22_see, p22_true and p22_roundtrip, whose bodies are identical (`agree_cached().is_ok()`). xtask alone stays at opt-level 0. For Claude: those four legacy rows of gates 2.1 and 2.2 check one fact, which rule 41 would delete.
- **P73-S2**: after `cargo test --workspace --no-run`, `cargo xtask vocab` compiles xtask's own binary (`Compiling xtask v0.1.0`) and no dependency. joinn-cli's binary now compiles joinn-prim's mutant register, which it never calls.
- **P73-S3**: `phase 2.2` takes 14 ms because its agree items read the sweep phase 2.1 ran. The `gate` usage line ends `[--item <n>]`.
- **P73-S4**: stop-check prints summed test counts, `exit 0` for steps that print nothing, and cargo's `Finished` line as clippy's last line; "every line containing fail" catches `a blind seal fails the item`.
- **P73-F1**: item 1's control also admits through `g72_layouts`; under full admission `ShiftPort("path", "calc.sum@2", 9)` is refused as `link path member calc.sum@9 no such port; acceptance is a declared port`.
- **P73-F2**: the probe carries the size in the tick's first pad word; it also requires each boundary's band to be above the band a notch below.
- **P73-02**: P73-F1 and P73-F2 each ran gate 7.2 only, though Run §3.1 asked `gate all` for their files; P73-02's `gate all` covers both.
- **P73-03**: on a spine a folded node faces its neighbour in member order (no knot); a leg standing for several members is member 0; arrowheads are 2 units centred on the path's middle.
- **P73-04**: "meets" is read as positive-area overlap of the widened piece with the closed rectangle; a stub's start inside its own surface is excused.
- **P73-05**: `--forms` counts the routes of the view's fold state over the whole universe; "fading" counts either window; a region's knot takes the region half-width; the first check failed on vocab only (a test name), renamed.
- **P73-06**: style 11 held transparent and the style-table test asserts 15 entries; routes grow inside `layout_universe`; closed rectangles for "deepest chart"; region colour chosen by the shader.
- **P73-07**: the segment owner wrote colour only until P73-08; pick's `frame median … ; information` lines differ by timing only.
- **P73-08**: the first check failed on fmt (one line), `gate all` ran before `cargo fmt` (whitespace only); grove lines' agree/edge counts move with link outlines (`frame edge 37 → 70`); at the universe frame the ordered spine owns no pixel, so the ordered view frames its galaxy.
- **P73-09**: the routes field is appended to each grove step line; the first demo's redirect failed (`The process cannot access the file because it is being used by another process`) and the demo was rerun.
- **P73-10**: the pan writes 3209 chart rows (a rebase). The window said `member 4` where the in-memory replay says `member 3`: `parse(print(grow_grove(7)))` reorders links (gal_g0 first in the file). Rule 67: input posted only to the window Cursor started.
- **P73-11**: `--release` used for the measurement only; zoom --measure's draw instances now 311634 (segments counted).
- **P73-12**: PowerShell's `*>` redirect garbled µ and · in the saved measure files; the appendix restores them.
- **P73-13 (1)**: §6's item 3 opposes `DropMember("e0", "units.scale@0")`, refused by the harness: `gate item 3 (A folded system is touched once): mutation DropMember("e0", "units.scale@0") does not apply to this subject kind (universe)`. Per §6, the next unused mutation of `universe.universe` in catalogue order is used: `FlipMark("e0", "units.scale@0")`.
- **P73-13 (2)**: the test listing gate leaves that name the grove now lists the four g73 files that do, renamed `only_gate_seven_two_s_and_seven_three_s_items_name_the_grove`.
- **P73-13 (3)**: §6 gives no per-view form counts; the acceptance is the findings' `links --forms` block (`FORMS_BLOCK`).
- **P73-13 (4)**: item 1's "body or cell pixel" is any pixel the CPU, with links left out, gives an owner other than a frame or lens node.
- **P73-13 (5)**: one zoom per form threshold, the in-window route owning the most pixels: `form fade 240: grove s level 0 step 0, link sys_g0s09 (size 240) owner region: cpu 3606 pixels`, `form fade 1920: grove s level 1 step 0, link gal_g0 (size 1024) owner hub: cpu 1640 pixels`.
- **P73-13 (6)**: gate 7.3 adds 106565 ms to `gate all` (497566 ms).
- **Stop**: the first `stop-check --fresh` call returned at once with `The process cannot access the file 'D:\JoInn\joinn\target\stop-check-73.txt' because it is being used by another process`, while one stop-check (started 12:49:21) kept running; it was waited for and its output is above. No second run was started.

## CI

Read before this stop's push (`Invoke-RestMethod "https://api.github.com/repos/joneseysinno/JoInn/actions/runs?branch=main&per_page=3"`):

```
37392421907 0480723 completed success 10/06/2026 00:08:44
37391259727 a44e2d9 completed success 10/05/2026 23:55:53
37372648096 4845326 completed failure 10/05/2026 20:54:46
```

- Run 37392421907 (`head_sha` 0480723, P73-01): `check (windows-latest): completed success`, `check (ubuntu-24.04): completed success`.

P73-01a … P73-14 and this report are pushed at this stop; their run is read once after the push, without waiting.

## Tripwires

None fired. `git diff --stat 0480723..HEAD -- joinn/corpus` prints nothing; `gate all` exits 0 with every earlier phase as `gates.lock` and `phase 7.3: 3/3`; the suite passes (442, 0 failed); no `gate all` line other than timing lines changed from P73-11's run to P73-13's (the added lines are gate 7.3's).

AJ's window check (§6.2) waits for the review.

Ledger: `| 3 | 7.3 | 5c71b95e9eac00173f8c4c46eace0aefee126050 | phase 7.3: 3/3 | 442 passed, 0 failed | 25 | yes | none | 492425 |`

Next: review
