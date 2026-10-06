# JoInn Phase 7.2 Review (Stops B and C)

Claude · 5 Oct 2026 · against `docs/Plans/JoInn Phase 7.2 Implementation Plan.md`, `docs/Findings/phase-7.2-stop-b.md`, `docs/Findings/phase-7.2-zoom.md`, and the commit messages of P72-11 … P72-15

**Method.** `origin/main` ends at `4845326` (P72-stop-b): chunk C was never pushed. So this review cloned GitHub at `4845326`, then copied in every file changed on `D:\JoInn` after that commit (41 crate files, 27 xtask files, `Cargo.lock`, `ci.yml`, docs) and applied the two deletions (`session/fit_viewport.rs`, `window/on_click.rs`). The commit messages of P72-11 … P72-15 were read from the repo's object files. Nothing under `joinn/corpus/` changed after the run started.

Three limits, stated so they are not mistaken for coverage:

1. Linux, 2 cores, one adapter (lavapipe), **rustc 1.97 stable**: 1.95 could not be downloaded from this session. AJ's machine has three adapters.
2. CI could not be read from this session, and chunk C has never run in CI.
3. The window was not run. `pick`, `regrow` and `zoom` were not run on their own; gate 7.2's checks include their substance.

---

## Verdict

**Phase 7.2 is accepted.** The zoom is exact, and it was shown exact the hard way: every prediction held, both cuts name the same owners at every view on all three of AJ's adapters, and the two plants that matter (a rounded rebase, a plain zoom that writes rows) were each caught with the exact line.

Two of P72-14's snags were **my errors in the plan**, not Cursor's, and both are fixed as the first code of Phase 7.3 (Amendment 2 of the 7.2 plan; P73-F1, P73-F2). What is still owed is paperwork and evidence: the Stop C report, the push, a CI run on chunk C, and AJ's four-minute window check. Row 0 of `JoInn Run 7.3-9.md` does the first three.

---

## Reproduced here

| Command | Here | Cursor |
|---|---|---|
| `cargo test --workspace --no-fail-fast` | **412 passed, 0 failed** (80 s) | 412 passed, 0 failed (P72-14) |
| `cargo xtask gate 1` … `gate 7.2`, one at a time | every phase line equals `gates.lock`; `phase 7.2: 3/3`, items 1, 2 and 3 `ok` | the same |
| The same thirteen gates with the optimized dev profile (§2.11 (a) of the 7.3 plan) | **every line identical** apart from cargo's own lines; suite 412 / 0 | — |
| `git diff` of `joinn/corpus` since the run started | empty (T1 did not fire) | — |

---

## Snags settled

| Commit | Snag | Settled |
|---|---|---|
| P72-07 | A single-body `Scene` draws no text; text exists only in the universe scene | Accepted. It kept Phase 6's replace delta and Phase 7.1's regrow lines byte-identical, which the plan required |
| P72-08 | Eight storage tables fill the vertex stage; the style table became a 64-byte uniform | Accepted. **It matters for 7.3**, which adds Route and Segment tables: the 7.3 plan (Draft 0.2, §2.7) now binds them only in the segment pipeline |
| P72-09 | The capsule's middle test needs 256-bit products at 2^-32 px | Accepted. Exactness was kept rather than the edge band widened |
| P72-10a | CI's `fmt` refused chunk B; per-commit checks had not run `fmt` | Accepted. `fmt` and `clippy` are part of `cargo xtask check` from now on |
| P72-11 | The window received input Cursor did not send, twice; V147 rests on the headless session test | Accepted. Cursor correctly did not chase it (rule 67). **AJ's §6.2 check, step 7, is the window evidence still owed** |
| P72-11 | The shell's session test lives in xtask; `joinn-shell-desktop` is an xtask dev-dependency, which `layers` does not read | Accepted for the harness. Noted: `layers` checks `[dependencies]` only |
| P72-14 (1) | `adversary.universe` is refused by full admission, so items 2 and 3 were graded through a second admission path without `check_link_types` | **Claude's error in §2.13.** P51-09 recorded that this file is refused. Fix: Amendment 2, P73-F1. Both controls move to `ordered.universe` and every check admits through `admit_universe` |
| P72-14 (2) | Demo (b), `>` for `≥` in the shader, is not caught | **Claude's prediction was wrong; Cursor's arithmetic is right.** `≥` and `>` differ only where `10·(256 + step)·size·2^level = 11·T·256`. For T = 4 and T = 32, `11·T·256` has no factor 5, so equality is impossible for any integers: the two operators are the same function there. For T = 240 it needs sizes such as 33 or 48, and the grove draws 40, 20, 304 and 1296. Fix: a GPU probe at exactly that boundary (Amendment 2, P73-F2) |
| P72-14 (3) | The first try at demo (c) wrote nothing, because `write_charts` writes only changed rows | Accepted. The plant was wrong and the gate was right; the second plant was caught on every plain step |
| P72-15 | `report pending` | Run 7.3–9, row 0 |

---

## Found in review

1. **The checking cost more than the building.** `gate all` took 571 431 ms at Stop B and **1 237 857 ms** at P72-14: gate 7.2 alone doubled it. Chunk B's six commits took 80 minutes; its stop took another 100. P72-14 took 72 minutes, because each demo re-ran the whole of gate 7.2. This is what AJ asked about, and it is the reason for `JoInn Run 7.3-9.md` and Part S of the 7.3 plan.
2. **An unoptimized build.** Measured here, with nothing changed but the dev profile (opt-level 1 for the workspace, 3 for dependencies, overflow-checks and debug-assertions kept): the suite went from 80 s to 46 s, and the thirteen gates from **724 s to 267 s** (gate 7.2 from 223 s to 36 s, gates 6 and 7 from 39 s to 10 s, gate 4 from 20 s to 4.5 s). Output identical.
3. **Two builds of the GPU stack.** After `cargo build --workspace --all-targets`, the first `cargo xtask` recompiled 15 crates (`wgpu`, `naga`, `wgpu-hal`, `wgpu-core` and their small dependencies, `joinn-gpu`, `xtask`): 38.6 s unoptimized, **102 s** optimized. `cargo run -p xtask` resolves features without `winit`, so it and `cargo test --workspace` never share those builds, and every edit below `joinn-gpu` compiles both. P73-S2.
4. **`agree` is the slowest thing in the repo**, and it runs twice inside `gate all` (gates 2.1 and 2.2) and a third time as CI's own step: about 100 s each even optimized. Sampled with a debugger, the time is in the live engine cloning cells on every delivery and re-hashing a cell's coding region on every fire. P73-S3 runs it once per process. The engine itself is a truth-core change and becomes **R108**, not a speed commit.
5. **Gate 7.2 item 1 re-runs gates 6 and 7's picture checks**, so `gate all` draws those pictures twice. P73-S3 runs them once.
6. **302 352 instances every tick, at every view.** The cut is CPU-cheap (19–66 µs, never per tick), but the GPU draws every row and discards in the shader; strokes are 82 % of it. Phase 7.3 adds segments on top. Not a defect: the measurement P73-11 asks for, and R105's (snapshots) evidence.
7. **A resize can rebase and rewrite all 3209 chart rows** (P72-11's 1280 × 720 resize). It is not a pan or a zoom, so V142 holds as written, but every rebase writes every chart: a cost that grows with the universe. Observed for R105.

---

## Exit conditions (§6.1)

1. Gate 7.2 passes and `gate all` exits 0 (AJ's machine, P72-14; reproduced here gate by gate): **met.** CI green on both OSes with the `zoom` step: **not yet**, because chunk C was never pushed. Row 0 pushes it.
2. `phase-7.2-zoom.md` with every prediction marked and the adversary answered: **met.** 17 of 17 zoom lines, the frame, the grove's counts, layout's last line and both touch counts are `as predicted`; the cut does not run per tick.
3. AJ's window check: **owed** (the plan's §6.2, four minutes).
4. No open snag: **met.** Two are carried into 7.3 by decision (P73-F1, P73-F2), and none blocks it.

**Phase 7.2 is closed** once row 0's push shows CI green. Phase 7.3 opens under `JoInn Run 7.3-9.md`, with the plan at Draft 0.2.
