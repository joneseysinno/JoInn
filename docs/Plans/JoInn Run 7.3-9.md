# JoInn Run 7.3–9

**One phase at a time, one stop per phase, checked by tier**

Author: AJ, with Claude · Draft 0.2 · October 5, 2026

> **This plan replaces `JoInn Run 7.2B-9.md` from its chunk 3 on.** Phase 7.2 is closed by Claude's review (`docs/Findings/phase-7.2-review.md`). Phases 7.3, 8 and 9 each run as **one unit**: every commit in order, one stop at the end, then Claude reviews before the next phase opens. Every decision is in the plans. Cursor never stops to ask; it follows each plan's Snags section.

---

## For AJ: this run in plain English

**What changed, and why.** Phase 7.2 cost most of its time in checking, not in building. Its gate run took **20.6 minutes** on your machine at the end of the phase (it was 9.5 at Stop B), and one commit, gate 7.2, took 72 minutes, mostly re-running the whole gate for each "show it fail" demo. Then each of three stops re-ran everything and waited for GitHub. This run keeps every check and changes when and how they run:

| Before | Now |
|---|---|
| Three stops per phase (A, B, C), each with a full `gate all`, every command, CI waited for | **One stop per phase.** The A/B/C headings stay as sections of the plan; they are not stops |
| Full suite and scans typed by hand after every commit | **`cargo xtask check`**: one command, prints the commit report's lines itself |
| Every earlier gate re-run only at stops, all of them | **Scoped gates**: a commit runs the gates its files can affect. Changing a foundation crate still runs `gate all` |
| A demo ("shown then reverted") re-runs the whole phase gate | **`gate <phase> --item <n>`**: one item |
| Unoptimized debug build | **An optimized debug build**: same overflow checks, same answers, faster: on Claude's measurement every gate together went from 724 s to 267 s, gate 7.2 from 223 s to 36 s, with identical output |
| `cargo test` and `cargo xtask` built the GPU stack twice | **One build**: both use the same features |
| Cursor waited for CI at every stop | Cursor reads CI once and **does not wait**; Claude reads the result at the review |

None of this weakens a test or a gate. Every gate still runs in full at the end of every phase, from a fresh clone, and CI still runs everything on Linux and Windows on every push.

**Before you start (two minutes).** Phase 7.2's chunk C is only on your machine. In PowerShell:

```
cd D:\JoInn
git push origin main
```

Then do the **7.2 window check** (four minutes, the 7.2 plan's §6.2) and tell Claude what you saw. Then give Cursor:

```
Do the run in docs/Plans/JoInn Run 7.3-9.md, starting at its §1 row 0. Follow AGENTS.md. AJ authorizes git push origin main at every stop in this run.
```

Cursor closes 7.2's paperwork, does all of Phase 7.3, stops, and prints `phase 7.3: stopped for review`. Tell Claude "7.3 stopped". After the review, the next prompt is:

```
Continue the run in docs/Plans/JoInn Run 7.3-9.md with the next phase in §1. Follow AGENTS.md. AJ authorizes git push origin main at every stop in this run.
```

If Cursor runs out of room in the middle of a phase, open a new chat with:

```
Continue the run in docs/Plans/JoInn Run 7.3-9.md from docs/Findings/run-ledger.md and git log. Follow AGENTS.md. AJ authorizes git push origin main at every stop in this run.
```

---

## 1. The order

| # | Unit | Plan | Ends with |
|---|---|---|---|
| 0 | **Close Phase 7.2** (no code) | this plan, §2 | `P72-stop-c` commit, ledger line 2, push |
| 1 | **Phase 7.3** | `JoInn Phase 7.3 Implementation Plan.md` (Draft 0.2) | `docs/Findings/phase-7.3-stop.md`; Claude reviews |
| 2 | **Phase 8** | `JoInn Phase 8 Implementation Plan.md`, read with §5 below | `docs/Findings/phase-8-stop.md`; Claude reviews |
| 3 | **Phase 9** | `JoInn Phase 9 Implementation Plan.md`, read with §5 below | `docs/Findings/phase-9-stop.md`; Claude reviews; AJ's first-grader test |

**Each unit stops.** After a phase's stop commit and push, Cursor prints `phase <n>: stopped for review` and does nothing else. The next phase opens only on AJ's prompt after Claude's review. (If AJ is away, AJ may write "chain the phases" in the prompt: then a phase stop with no tripwire continues into the next phase, exactly as Run 7.2B–9 did.)

---

## 2. Row 0: close Phase 7.2

Phase 7.2's last commit, `P72-15`, says `report pending`. Nothing else is owed in code. Do, in order:

1. `cargo xtask gate all > target/gate-all-7.2.txt 2>&1` on HEAD (`17e5160`). Keep the file: P73-S1 compares against it. Paste its last 15 lines into the stop report.
2. Write `docs/Findings/phase-7.2-stop-c.md` in the **short form** (§4): HEAD; one line per chunk C commit (P72-11 … P72-15) with MET / NOT MET; the last line of each command in the 7.2 plan's §0.2 item 3 (run once now); the chunk's snags as already written in the P72-11 … P72-14 commit messages; the CI read (§4 item 6).
3. Append ledger line 2 (`7.2 C`) to `docs/Findings/run-ledger.md` and change its title line to `# JoInn run ledger (Run 7.2B–9, then Run 7.3–9).`
4. Commit `P72-stop-c: stop report` with the two files, push.

Do **not** change any code in row 0. The 7.2 fixes are in Phase 7.3 (P73-F1, P73-F2).

---

## 3. Checks by tier

These replace "Suite" and "Scans" after every commit and the three full stops. Phase 7.3's §2.11 builds the commands; until they exist (P73-01 … P73-S3), Cursor runs the same steps by hand.

### 3.1 Tier 1: every commit

1. The commit's **done-when**, exactly as the plan writes it.
2. **`cargo xtask check`**: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace --no-fail-fast`, `vocab`, `modules`, `layers`, and the phase's own commands (7.3: `zoom`, and `links` from P73-04). It prints the commit report's `Check:` block.
3. **The scoped gates** for the files the commit changed (`git diff --name-only HEAD~1`), from this table. Run the widest row that applies.

| The commit changes | Also run |
|---|---|
| only files under `docs/` | nothing, not even `check` (write `Check: docs only`) |
| files it **adds** in `joinn-visual`, `joinn-gpu` or `joinn-shell-desktop`, plus only `mod` / `pub use` lines in existing files | nothing more |
| any **existing** file in `joinn-visual`, `joinn-gpu` (the shader included), `joinn-shell-desktop` or `joinn-test-host` | `gate 6`, `gate 7`, `gate 7.2`, and every later picture gate that exists (`7.3`, `8`, `9`) |
| a gate leaf or gate table of the current phase (`xtask/src/fns/g73_*`, `gate_seven_three*`, and so on) | that phase's gate |
| anything else under `joinn/`: `joinn-frame`, `-dna`, `-prim`, `-live`, `-gate`, `-link`, `-host`, `-assay`, `-cli`, `-creator`; xtask's shared harness (`mutate*`, `subject*`, `harness_fixtures*`, `run_gate_table`, `grade_opposed`, `gate_all`, `corpus_*`, `pick*`, `regrow*`, `grove*`); `Cargo.toml`, `Cargo.lock`, `.cargo/`, `rust-toolchain.toml`; anything under `corpus/` | `gate all` |

A plan's done-when that names a gate (for example P73-07's "gate 6, gate 7, gate 7.2 each full") always runs it, whatever the table says.

### 3.2 Demos

Every "shown then reverted" demo runs **only what it is about**: `cargo xtask gate <phase> --item <n>` for a gate item, the named command otherwise. Paste the failure, revert, and run the same command once more to show it passes again.

### 3.3 Tier 2: the phase stop

**`cargo xtask stop-check --fresh`**: a fresh clone of HEAD, then everything CI runs (the full list in `.github/workflows/ci.yml`, `gate all` included), each command's last line, every `fail` line, every failing test name, and each command's time. It is run once, at the phase stop, after the docs-and-freeze commit. Its output goes into the stop report whole.

### 3.4 What a tier never does

It never skips a done-when, never marks a test `#[ignore]`, never edits `gates.lock` by hand, never uses `--release` for a check (only for a plan's `--measure`), and never counts a gate as passed that did not run.

---

## 4. The stop report (short form)

At each phase stop, `docs/Findings/phase-<n>-stop.md`:

1. `git rev-parse HEAD`.
2. One line per commit: `P73-NN <short hash> <subject>: MET | NOT MET | skipped (depends on …)`. **The full commit report is in each commit message; do not copy it here.**
3. The whole output of `cargo xtask stop-check --fresh`.
4. Every predicted value the plan's findings commit marked `differs`, one line each.
5. **Snags**: every snag, tagged with its commit id, with its printed line.
6. **CI**: after pushing, read the newest run once (`Invoke-RestMethod "https://api.github.com/repos/joneseysinno/JoInn/actions/runs?branch=main&per_page=3"`, then that run's `jobs_url`). Paste `head_sha`, `status`, `conclusion`, each job's name and conclusion. If `status` is not `completed`, paste the run id and `in progress` and **do not wait**. Claude reads the result at the review.
7. The ledger line (§6) and `Next: review`.

Commit `P<n>-stop: stop report`, push, print `phase <n>: stopped for review`, stop.

---

## 5. Phases 8 and 9 under this run

Their plans were written for Run 7.2B–9. Read them with these changes, which override the plan's text:

- **§0, unit of work and end:** the phase, not a chunk; one stop at the end (§4 above). The `— Stop A` and `— Stop B` rows in §4 are **skipped**. The `— Stop C` row is the phase stop, written as `docs/Findings/phase-<n>-stop.md`.
- **§0.1, the commit report:** `Check:` and `Gates:` lines (Phase 7.3's §0.1) replace `Suite:` and `Scans:`. Phase 8 adds its commands to `check` when it builds them (`reader`, `visual`, `inverse`, `keys`); Phase 9 adds `create`.
- **§0.2:** this run's §4.
- **Gate commits** (P8-12, P9-13): the demos use `gate <n> --item <k>` (§3.2); the done-when's `gate all` runs once.
- **Freeze commits** (P8-13, P9-14): "`gate all` from a fresh clone" is `stop-check --fresh`, run once at the stop and pasted there; the commit itself runs tier 1.
- **AGENTS.md:** each phase's Appendix A header replaces "Work one CHUNK at a time … continue unless a tripwire fires" with "Work the phase's commits in order, one git commit per numbered step, check each by tier, and stop once at the phase's end."

Phase 8's and 9's own decisions, commits and gates do not change.

---

## 6. The ledger

`docs/Findings/run-ledger.md`, one line per stop, as before:

```
| # | Unit | HEAD | gate all | suite | snags | pushed | tripwire | gate all ms |
```

The new last column is `gate all`'s wall milliseconds from `stop-check`, so the speed of the gate is tracked phase by phase.

---

## 7. Tripwires

A tripwire stops the phase at once: Cursor writes the stop report as far as it got, adds the ledger line with the tripwire, commits, pushes, prints `run: stopped by T<n>`, and does nothing else.

| # | Fires when |
|---|---|
| **T1** | Any file under `joinn/corpus/` that existed before the phase changes (Phase 8's nine new witness files and appended `hashes.txt` lines are not changes) |
| **T2** | `gate all` (scoped or at the stop) does not exit 0, or prints any earlier phase's line with a different score than `gates.lock` |
| **T3** | At the stop, `cargo test --workspace --no-fail-fast` has any failure |
| **T4** | A phase's gate commit is `skipped: depends on …` |
| **T5** | The same snag text appears in three consecutive commits' reports |
| **T6** | A speed commit (P73-S1 … S4) changes any line of `gate all`'s output other than a line containing `milliseconds` or `(information` |

A tripwire is never avoided by weakening a test, editing `gates.lock`, reblessing, or rewording a plan. Rules 6, 7 and 42 hold.

---

*JoInn Run 7.3–9, Draft 0.2 (5 Oct 2026). AJ asked on 5 Oct, after Phase 7.2 finished: run each phase as one unit, with every speed improvement for Cursor's checks. Claude decided, open to AJ's veto: one stop per phase with a review between phases; tier-1 checks per commit with gates scoped by the files changed; the full gate once per phase from a fresh clone; demos by item; an optimized debug profile with overflow checks kept; one feature set for the whole workspace; CI read once and never waited for.*
