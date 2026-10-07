# JoInn Phase 7.4 Review

Claude · 7 Oct 2026 · against `docs/Plans/JoInn Phase 7.4 Implementation Plan.md` (Draft 0.1), `docs/Findings/phase-7.4-stop.md`, `docs/Findings/phase-7.4-growth.md`, and the commit messages of P74-01 … P74-15

**Method.** Cloned GitHub `main` at `16e2766` (P74-stop: ledger line 4). Ran `cargo xtask stop-check` in the clone, then `cargo xtask gate 7.4` on its own. Read the growth code, the witness, evolution, the lasso rules, gate 7.4's items and controls, and the change to gate 7.3 item 3. Then probed growth at large sizes.

What this review could not cover:

1. Linux, 2 cores, one adapter (llvmpipe), **rustc 1.97 stable**. 1.95 still can't be downloaded from this session. AJ's machine has three adapters.
2. CI couldn't be read from this session, so the run on the stop push is unread.
3. The window wasn't run. That is AJ's §6.2 check.

---

## Verdict

**Phase 7.4 is accepted.** A system grows by its DNA, every size gives the right count, counting checks every step, adding evolved from counting and keeps its witnesses, and the lasso is drawn, picked and agrees on every adapter. Every count the plan predicted came out exactly. The three 7.3 fixes landed. Every earlier phase still equals `gates.lock`.

The review found two real gaps and one finding that needs correcting. None is a wrong answer:

- The phase's own new rule (rule 93: a truth nobody can break gets a standing plant) is missed in one place: **evolution's "every old witness holds."**
- `gate all` writes `gates.lock` *before* comparing, so the comparison it does is against itself.
- Growth cost is **quadratic per step**, not "a little faster than linear." It doesn't matter at a child's pace for a while, but the findings understate it.

I propose fixing the first two as the opening commits of Phase 7.5 (P75-F1, F2), and correcting the findings' sentence there too.

---

## Reproduced here

| Command | Here | Cursor |
|---|---|---|
| `cargo test --workspace --no-fail-fast` | **486 passed, 0 failed** | 486 passed, 0 failed |
| fmt, clippy, vocab, modules, layers | all ok | the same |
| `corpus verify` | **48 hash(es) match** | the same |
| `agree`, `assay agree`, `forces`, `contact`, `power`, spike s8 | every last line identical | the same |
| `pick` / `regrow` | `1 adapter(s), 12 subject(s) agree; planted …: refused` / `planted difference: refused (ok)` | the same with 3 adapters |
| `zoom` step → `grow` | `grow: 2 systems, every size true, counting witnesses every step; evolution holds; planted fold, lasso, hash: refused (ok)` | the same |
| `gate all` | exit 0; `gates.lock` unchanged in the clone; 258 s | exit 0; 528 s |
| `gate 7.4` | **3/3**; items 1, 2 answered by check, item 3 by admission; `body pixels the force owns 0` | the same |
| `stop-check` | every step ok except `lavapipe` (apt here can't reach a mirror; llvmpipe was already installed) | ok |
| `git diff 45d868a..HEAD -- joinn/corpus` | the four phase74 files and `hashes.txt` only (T1 didn't fire) | the same |

---

## Snags settled

All 45 snags were read. Most are recording-level and accepted as written. The ones that matter:

| Commit | Snag | Settled |
|---|---|---|
| P74-01 | Part VII on disk lists G1–G16; G17–G19 were written from the plan's words | Accepted. **My error**: the plan named G17–G19 from my working draft of Part VII. Cursor's rows say where the words came from, which is right |
| P74-F3 (1) | `pick`'s grove lines changed; the plan predicted none | Accepted. **My prediction was wrong.** Where two links overlap, the one drawn last owns the pixel, and putting links in print order changes which one is last. That's F3 doing its job. But it exposes something for 7.5 (Found in review, 4) |
| P74-F3 (3) | Gate 7.3 item 3's check was changed after it failed | Accepted. The old check clicked a midpoint that another link also covered, so it passed only because of draw order. The new check picks a point on the leg that is clear of every other piece, which is what it meant to test. The lock was restored by `git checkout` (see Found in review, 2) |
| P74-03 | P74-02 was committed before its `gate all` finished | Accepted. Its run was redone on `fa6e7cc` and matched. A process lapse, recorded honestly |
| P74-06 (2) | The engine can't add an instance, so each step re-lowers and replays the whole body | Accepted as the plan allowed. **The cost is worse than the findings say** (Found in review, 3) |
| P74-07 (1) | `check_evolution` returns what it held and gained, not `()` | Accepted. Better than the plan |
| P74-14 (4) | Item 2's check makes "counting with nothing new" with the same mutation its control opposes | Accepted. The check proves the refusal on the real path; the control proves it on the gate's path. Same fact, two doors. Harmless |
| Everything else | Line forms, trybuild stderr, the camera key (`F`), Text refused by the body, stale worktree build | Accepted as written |

---

## Found in review

### 1. Evolution's "every old witness holds" can't fail, and has no standing plant

`check_evolution` has two halves: (1) on every parent transcript, the child gives the same count; (2) the child accepts something the parent refuses.

Half 2 is opposed for real: gate 7.4 item 2's mutant (adding made to accept `one`) is admitted, and the check refuses it. That's why item 2 says *answered by check*.

Half 1 **cannot fail** with anything admission accepts. Admission rule 7 makes the child grow the same cell as the parent, and rule 6 pins the force's response to the register's one combine for ℤ. Same cell, same force, same engine, so the counts are the same by construction. If the comparison inside `check_evolution` were broken (say it compared the parent to itself), nothing in the repo would notice.

That's exactly what rule 93 (new this phase) is for: *a truth no admitted artifact can break gets a standing plant in its command.* `grow` plants the fold, the lasso and the hash, but not evolution.

**Fix (P75-F1):** `grow` plants (d) on every run: a child whose response is off by one from n ≥ 2 (perturbed in the plant's code path, not in a file). `check_evolution` must refuse it with *acceptance is every old witness*. Last line gains `…, witness: refused (ok)`.

### 2. `gate all` writes the lock, then compares the lock to itself

`gate_all` runs every gate, **writes `gates.lock`**, reads it back, and checks that it matches what it just wrote. That comparison can't fail. The real guard has always been `git diff` on the lock (the tripwire), which is why P74-F3 (3)'s failing run wrote `phase 7.3: 2/3` into the lock and Cursor had to `git checkout` it.

This is older than 7.4 (it dates from the gate harness), and nothing bad happened. But a failed run left uncommitted is one `git commit -a` away from recording a lower score as the truth, and the file's own header says *never edit to make a check pass.*

**Fix (P75-F2):** `gate all` reads the committed lock **first**, compares every phase's outcome to it, and **writes nothing when any phase is lower** (it prints the difference and exits 1). It writes only when every phase is equal or a new phase is added. A test plants a lower outcome and asserts the file is byte-unchanged.

### 3. Growth costs n² per step, not "a little faster than linear"

The findings measured n = 0, 6, 24, 96 and called the curve "a little faster than linearly." I grew counting much larger (release, one `respond` at size n, which is one step):

| n | one step |
|---|---|
| 96 | 17 ms |
| 500 | 188 ms |
| 1 000 | 0.66 s |
| 2 000 | 2.9 s |
| 4 000 | 11 s |
| 8 000 | 44 s |

Doubling n costs **4×** per step: quadratic. So a whole session of n steps costs n³. Every answer was still right (8 000 at n = 8 000; the budget never bit), so this is speed, not truth.

What it means for her: counting to a few hundred is instant. Around 500 each Enter starts to lag; by 1 000 it's most of a second. The cause is the same one the findings named (re-lowering and replaying the whole body each step), multiplied by the engine's per-fire clone and re-hash (R108).

**Fix:** no code now (plan §7: a slow number is a finding). Correct the findings' sentence in P75-01 and add n = 500 and 1 000 to `grow --measure` so the number stays visible. Growth that doesn't re-lower (the findings' first item for 7.5) is the real cure; it becomes urgent when a universe grows many systems in a tick.

### 4. Links can lie on top of each other

P74-F3 (1) shows that where two links share a gutter, **one draws over the other**, and a click there names whichever was drawn last. Gate 7.3 item 3 had to move its click point to a spot only one link covers.

That's honest exactness (CPU and GPU agree), but it's a picture problem: two different connections drawn as one line, and the click picks by draw order. 7.5 is already planned to make *links show their ends*. It should also give each link its own lane where they share a gutter, so a click is never ambiguous. **Carried to the 7.5 plan as a requirement, not a fix.**

### 5. Two things to decide before evolution goes further (open items)

These aren't faults. They're where 7.4's shape will pinch next.

- **R118 · Where witnesses live.** The old witnesses are a table in code (`transcripts.rs`), chosen by the word `one` or `any`. A third evolution (say subtraction as a turn) means editing that table. In JoInn's own terms witnesses are *testimony* and should travel with the parent (a file with lineage), the way Phase 8's witness files will. This joins R116 (where her counts live): the counts she types *are* the witnesses for the next evolution.
- **R119 · Evolution that changes the frame.** Here evolution only widens what a body accepts. Counting → adding works because both live in ℤ. The next step on the path of truth (integers → fractions) changes the frame, and admission rules 5 and 7 forbid exactly that (same receptor frame, same grown cell). Before fractions, decide how a child may grow a *wider* cell than its parent and still keep every old witness.

Something worth noticing in a good way: in 7.4, adding *is* counting with the guard lifted. Same cell, same force, same engine; only what the body accepts changed. That's AJ's "all basic math is counting" made literal in the code.

---

## Proposed fixes: the first commits of Phase 7.5

| # | Fix | Why |
|---|---|---|
| **P75-F1 · Evolution's standing plant** | `grow` plants a child that drifts by one from n ≥ 2 on every run; `check_evolution` must refuse it on the witness half. `grow`'s last line names it | Gap 1 (rule 93) |
| **P75-F2 · The lock is compared before it is written** | `gate all` compares outcomes to the committed `gates.lock` first; any lower phase prints the difference, exits 1, and leaves the file untouched | Gap 2 |
| **P75-01 additions** | Correct the findings' cost sentence (quadratic per step); `grow --measure` adds n = 500, 1 000; backlog gains R118, R119; the 7.5 plan requires a lane per link where links share a gutter | Gaps 3, 4, 5 |

F1 and F2 change only checks. No gate answer changes, no corpus file changes, and `gates.lock` stays as it is.

---

## Exit conditions (§6.1)

1. `stop-check --fresh` exits 0 with `phase 7.4: 3/3` and no tripwire: **met** (AJ's machine; reproduced here).
2. `phase-7.4-growth.md` with every prediction marked and the adversary answered: **met**, with the cost sentence corrected in 7.5 (gap 3).
3. No open snag: **met.** The gaps above are carried into 7.5 by decision, and none blocks it.
4. AJ's window check (§6.2, about three minutes): **owed.** The CI run on the stop push is also unread.

**Phase 7.4 is closed** once AJ has done the window check and the CI run on `16e2766` is green.
