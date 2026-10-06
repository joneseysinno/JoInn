# JoInn Run 7.2B–9

**One run, four phases: finish the zoom, draw the links, make meaning witnessed, build the creator**

Author: AJ, with Claude · Draft 0.1 · October 3, 2026

> **Superseded on 5 Oct 2026 from chunk 3 on** by `docs/Plans/JoInn Run 7.3-9.md` (one stop per phase, checks by tier). Chunks 1 and 2 (Phase 7.2 B and C) were run under this plan; Claude's review is `docs/Findings/phase-7.2-review.md`.

> **Every decision in this run is final.** Nothing waits on AJ or on Claude. Cursor works the chunks below in order, writes every stop report, pushes, and keeps going. It stops only when a **tripwire** (§3) fires or the run is done.

---

## For AJ: this run in plain English

**What happens while you're away.** Cursor works straight through eleven chunks:

| # | Chunk | What it builds |
|---|---|---|
| 1 | 7.2 B | The fix for the grove's links, the stroke font, the new tables, the shader, exact picking at any zoom |
| 2 | 7.2 C | The window zooms, the measurement, gate 7.2 |
| 3 | 7.3 A | Hyperedges get routes: paths through the gaps between bodies, never through a body |
| 4 | 7.3 B | The routes are drawn as region, hub, bundle or spine, and you can click one |
| 5 | 7.3 C | The window shows links, gate 7.3 |
| 6 | 8 A | The screen-reader tree, built from the same cut as the picture |
| 7 | 8 B | Witnesses: what each body *means* on screen is recorded once and checked forever |
| 8 | 8 C | Keyboard-only use, the real screen-reader bridge, gate 8 |
| 9 | 9 A | The creator's brain: every gesture is an edit with an exact undo |
| 10 | 9 B | The creator's face: drawers, the bench, refusals drawn where they happen |
| 11 | 9 C | The creator in the window, unfold, gate 9 |

It will probably not finish all eleven in one day. That's fine: the ledger (§4) remembers where it stopped, and one prompt resumes it.

**What changed from how we usually work.** Normally Cursor stops after each chunk and waits for me to review. This time it doesn't wait. Instead it has **tripwires**: if a gate fails, a corpus hash moves, or a test is still failing at the end of a phase, it stops the whole run and writes why. When you're back, tell me "the run stopped" or "the run finished", and I review every stop report at once.

**I settled Stop A.** Chunk A is good. The one failure was my mistake in the plan (I joined a number port to a text port). The fix is written into the 7.2 plan as Amendment 1, and Cursor applies it first.

**Before you leave (two minutes).** In PowerShell:

```
cd D:\JoInn
git push origin main
```

That sends chunk A to GitHub (Cursor's own push was blocked). Then give Cursor this prompt:

```
Do the run in docs/Plans/JoInn Run 7.2B-9.md. Follow AGENTS.md. AJ authorizes git push origin main at every stop in this run.
```

If Cursor runs out of room, or you open a new chat, use:

```
Continue the run in docs/Plans/JoInn Run 7.2B-9.md from docs/Findings/run-ledger.md. Follow AGENTS.md. AJ authorizes git push origin main at every stop in this run.
```

**Choices I made for you while you're away (say if you disagree when you're back).**

- **Links travel in the gaps.** Every hyperedge is routed only through the empty lanes between bodies, which the grid layout always leaves. So a link can never cross a body: it's true by construction, and still checked exactly.
- **Region, hub and bundle are one path at three thicknesses.** Part II drew a region as a soft blob over everything, which would cross bodies. Here the region is the same path drawn wide and pale. Nothing crosses.
- **Snapshots wait.** Nothing so far says drawing is slow. They become a research item until a measurement asks for them.
- **What a body means is witnessed; how it looks is not.** Phase 8 records *who owns pixels*, *what the screen reader says* (roles and values, not names), and *what you can do*. Colours, names and exact pixels are style and may change. Golden pixel images are dropped as witnesses, the fallback the roadmap pre-agreed.
- **The creator builds bodies and universes, not new cells.** Writing new laws for new cells (R23, law authoring) is its own problem and gets its own phase after you've used the creator.
- **Unfold is a view.** Pressing U on a sealed cell shows the body it was folded from; Escape returns exactly where you were.

**What you do when you're back.** Tell me the run finished or stopped. I review. Then you do the window checks for each finished phase (about fifteen minutes in total, §6), and, after Phase 9, the **first-grader test** with a real person.

---

## 1. The Order

| # | Plan | Chunk | Starts with | Stop report |
|---|---|---|---|---|
| 1 | `JoInn Phase 7.2 Implementation Plan.md` | B | P72-03a (Amendment 1), then P72-06 … P72-10 | `phase-7.2-stop-b.md` |
| 2 | same | C | P72-11 … P72-15 | `phase-7.2-stop-c.md` |
| 3 | `JoInn Phase 7.3 Implementation Plan.md` | A | P73-01 … | `phase-7.3-stop-a.md` |
| 4 | same | B | | `phase-7.3-stop-b.md` |
| 5 | same | C | | `phase-7.3-stop-c.md` |
| 6 | `JoInn Phase 8 Implementation Plan.md` | A | P8-01 … | `phase-8-stop-a.md` |
| 7 | same | B | | `phase-8-stop-b.md` |
| 8 | same | C | | `phase-8-stop-c.md` |
| 9 | `JoInn Phase 9 Implementation Plan.md` | A | P9-01 … | `phase-9-stop-a.md` |
| 10 | same | B | | `phase-9-stop-b.md` |
| 11 | same | C | | `phase-9-stop-c.md` |

All plans live in `docs/Plans/`. Each phase plan's first commit (P73-01, P8-01, P9-01) commits that plan, the run plan if not yet committed, and the AGENTS.md header and rules from its Appendix A.

**The first commit of the run** is P72-03a. Before it, if `git status` shows this run plan or the amended 7.2 plan as changed or untracked, commit them as `RUN-01: the run plan, 7.2 Amendment 1, plans 7.3, 8 and 9` with the commit report block (done-when: `git show --stat HEAD` lists those five files).

---

## 2. How each chunk ends

Exactly as each phase plan's §0.2 says: the stop report, a commit, `git push origin main`. Then:

1. Append one line to `docs/Findings/run-ledger.md` (create it on the first stop, with the header below) and commit it in the same stop commit.
2. Check the tripwires (§3).
3. If none fired, start the next chunk in §1 **in the same session if there is room**, otherwise end with the line `run: continue from chunk <n>`.

**The ledger** (`docs/Findings/run-ledger.md`):

```
# JoInn run 7.2B–9 ledger. One line per stop. Written by the agent from command output.
Run start: <full hash of the P72-stop-a commit>
| # | Chunk | HEAD | gate all | suite | snags | pushed | tripwire |
|---|---|---|---|---|---|---|---|
| 1 | 7.2 B | <full hash> | <last phase line> | <N passed, M failed> | <count> | yes / blocked | none / T<n>: <line> |
```

`gate all`'s column is the last `phase …` line it printed. A resumed session reads the ledger's last line, checks `git log` agrees, and starts the next chunk.

**Pushing.** AJ's prompt authorizes `git push origin main`. If it is blocked anyway, write `blocked` in the ledger and the snag in the stop report, and continue. CI is then read as "not run" in the stop report.

---

## 3. Tripwires

A tripwire stops the run. Cursor writes the stop report as usual, adds the ledger line with the tripwire, commits, pushes, prints `run: stopped by T<n>`, and does nothing else.

| # | Fires when | Why it stops |
|---|---|---|
| **T1** | Any file under `joinn/corpus/` that existed before this run changes, other than lines appended to `hashes.txt` by Phase 8 (`git diff <run start>..HEAD -- joinn/corpus`, where *run start* is the P72-stop-a commit, written in the ledger's header) | The corpus is testimony. A moved hash needs Claude and AJ |
| **T2** | At a phase's **last** stop (C), `cargo xtask gate all` does not exit 0, or does not print that phase's line with a full score | The next phase would build on an unproven floor |
| **T3** | At a phase's last stop, `cargo test --workspace --no-fail-fast` has any failure | Same reason. Inside a phase a failing test is a snag; at its end it is a wall |
| **T4** | A phase's gate commit is `skipped: depends on …` | Same reason |
| **T5** | An earlier phase's gate line changes score (`phase 7: 3/3` becomes anything else) | Law 2: every earlier gate still passes |
| **T6** | The same snag text appears in three consecutive commits' reports | Something is stuck; more code would dig deeper |

A tripwire is never avoided by weakening a test, editing `gates.lock`, reblessing, or rewording a plan. Rules 6, 7 and 42 hold.

**Snags inside a chunk do not stop the run.** They follow each plan's §0.3: paste the line, leave the piece honest, continue.

---

## 4. What is shared across the four phases

- **Numbering.** AGENTS rules continue at 74; invariants at V148; research items at R105. Each plan states its own range.
- **Window checks by Cursor.** Every phase has one commit where Cursor runs the window itself and pastes a line, or writes `no display` (rule 67). AJ's checks are collected in §6.
- **Measurements never reach a decision** (rule 4). They go to findings.
- **New external dependencies.** Only Phase 8 adds any: `accesskit` and `accesskit_winit`, in `joinn-shell-desktop` only. Nothing else is added in any phase.
- **The corpus.** Only Phase 8 adds to it: nine witness files in `corpus/phase8/visual/` and their lines in `hashes.txt`. Those are new files and appended lines, not changes; T1 ignores them. Phase 9's gesture scripts are fixtures in `xtask/gate_fixtures/creator/`, not corpus.

---

## 5. The stop report, one addition

Every stop report in this run adds, after the CI section:

```
Run:        chunk <n> of 11 · ledger line written · tripwires: none | T<n>
Next:       <chunk> | stopped
```

---

## 6. AJ's checks at the end (about fifteen minutes, plus the first-grader test)

Do each only for a phase whose Stop C is in the ledger.

1. **7.2, the zoom** (four minutes): the 7.2 plan's §6.2.
2. **7.3, the links** (three minutes): the 7.3 plan's §6.2.
3. **8, keyboard and screen reader** (four minutes): the 8 plan's §6.2.
4. **9, the creator** (five minutes): the 9 plan's §6.2.
5. **The first-grader test** (thirty minutes, another day): the 9 plan's §6.3. A person who can't program, building the add body, unaided. You coach for a living: watch, don't rescue.

Tell Claude which steps worked and which looked wrong.

---

*JoInn Run 7.2B–9, Draft 0.1 (3 Oct 2026). Written after the Phase 7.2 Stop A review while AJ is away for a day. AJ decided earlier the same day: the order 7.2, 8, 9; Visual Host II split into 7.2 and 7.3; exact all the way; the stroke font; the grove. AJ asked on 3 Oct for one plan that lets Cursor finish phases 7, 8 and 9. Claude decided the rest, open to AJ's veto on return: non-blocking stops with tripwires; gutter routing; one path at three widths; snapshots deferred; witnessed meaning without golden pixels; the creator edits bodies and universes; unfold as a view.*
