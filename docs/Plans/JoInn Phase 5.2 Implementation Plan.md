# JoInn Phase 5.2 Implementation Plan

**Damage that keeps the form · a working plan for Cursor**

Author: AJ, with Claude · Draft 0.3 · September 24, 2026

> **Every decision in this plan is final.** Nothing here waits on AJ. Cursor never stops to ask AJ anything, and no file in this phase is typed by AJ. If a step can't be done the way the plan says, Cursor follows §0.3 (Snags) and keeps going.

---

## For AJ: this plan in plain English

**What 5.2 fixes.** Phase 5.1 built links between bodies. The review found that several of the "controls" (the checks that are supposed to prove a test can fail) could pass without really looking at anything. 5.2 fixes that. Every control now has to catch a small, specific, realistic change to its file, like one link removed or one hash changed, and it has to ignore a harmless change, like a renamed label.

**What changed in this draft.**

1. **You don't type anything.** The checkpoint forms and the "Accepted: AJ" line are gone. Claude checks Cursor's work by downloading the code from GitHub and running it on a separate computer.
2. **Three stops.** Cursor works through a chunk, writes a short stop report, pushes, and stops. You tell Claude "Cursor finished chunk B". Claude checks the work and tells you in plain English how it went. Then you give Cursor the next prompt.
3. **Snags don't block.** If a step doesn't work, Cursor writes down exactly what happened, doesn't fake or dodge it, and moves on.
4. **Your three decisions from today are built in:**
   - The forbidden-words scan checks JoInn's own language only.
   - One body has one set of screen labels.
   - The older checks (phases 1–3) are labeled "legacy" and get upgraded in a later cleanup phase.
5. **Three problems Claude found by running your code on Linux are fixed first:**
   - The calculator's screen labels depended on file order.
   - Empty folders on your disk were hiding a rule violation from the modules scan.
   - The GitHub checks were never actually running, because they sat in the wrong folder.

**Your prompts to Cursor** (copy exactly, one per chunk):

- Chunk B: `Do chunk B of docs/Plans/JoInn Phase 5.2 Implementation Plan.md. Follow AGENTS.md.`
- Chunk C: `Do chunk C of docs/Plans/JoInn Phase 5.2 Implementation Plan.md. Follow AGENTS.md.`
- Chunk D: `Do chunk D of docs/Plans/JoInn Phase 5.2 Implementation Plan.md. Follow AGENTS.md.`

If Cursor runs out of room partway through a chunk, start a new Cursor chat with: `Continue chunk B of docs/Plans/JoInn Phase 5.2 Implementation Plan.md from the first commit not in git log. Follow AGENTS.md.` (Use the right letter.)

---

## 0. How Cursor Works This Plan

| | |
|---|---|
| **Plan file** | `D:\JoInn\docs\Plans\JoInn Phase 5.2 Implementation Plan.md` |
| **Code** | `D:\JoInn\joinn\`, no new crates |
| **Standing rules** | `AGENTS.md` as rewritten by P52-02b (Appendix A). `.cursor/rules/joinn.mdc` becomes a pointer to `AGENTS.md` and holds no rules of its own |
| **Unit of work** | a **chunk** (B, C or D). Inside a chunk, do the commits in the order listed. Each commit is its own git commit, and its message starts with its id (`P52-05: …`) |
| **End of a chunk** | write the stop report (§0.2), commit it, `git push`, and stop. Do not start the next chunk |
| **Who decides** | every decision is in §2. Cursor decides only module layout (rule 25), function bodies, error strings, and how the mutation catalogue is represented in Rust |
| **Who checks** | Claude, at each stop, from a fresh clone on Linux. GitHub CI (P52-02b) runs the same commands on Windows and Linux on every push |

### 0.1 The commit report

Every commit ends with this block. It goes in the commit message body **and** in the chunk's stop report. Every value is copied from the terminal, never summarised.

```
Commit:     P52-NN <git short hash>
Done-when:  <the command> → <the line it printed>          MET | NOT MET
Suite:      cargo test --workspace --no-fail-fast → <N passed, M failed>
Scans:      vocab → <last line> · modules → <last line>
Snags:      none | <each thing that went differently from the plan, with the printed line>
```

`vocab` and `modules` run on every commit.

### 0.2 The stop report

At the end of each chunk, Cursor writes `docs/Findings/phase-5.2-stop-<letter>.md` with:

1. `git rev-parse HEAD` (the full hash).
2. Every commit report block from the chunk, in order.
3. The last line of each of: `cargo test --workspace --no-fail-fast`, `cargo xtask gate all`, `cargo xtask corpus verify`, `cargo xtask vocab`, `cargo xtask modules`. Also every line of `gate all` that contains `fail`, and every failing test name.
4. **Snags**: every snag from the chunk in one list, each tagged with its commit id.

Then commit (`P52-stop-<letter>: stop report`), push, and stop. There is no acceptance line. Claude reviews the pushed tree.

### 0.3 Snags

A snag is any point where the plan's done-when can't be met as written, or the plan's prediction turns out wrong.

- **Never** fake it, work around a scan, weaken a test, rebless a golden, or change the plan's meaning to make it pass.
- Write the snag in the commit report with the exact printed line, leave that piece in its honest (possibly failing) state, and **continue with the next commit**.
- If a later commit depends on the snagged one (dependencies are listed in §4), skip that later commit too, and write it down as a snag: `skipped: depends on P52-NN`.
- A failing gate item or a red test left on purpose is not a reason to stop.

---

## 1. Scope Fence

### In scope

- **Workflow repair (P52-02b):** new rules, CI that actually runs, the order-dependence and empty-folder bugs, one face per body.
- **Parsed subjects, the mutation catalogue, neutral edits.** Controls see values, and damage keeps the form. This applies to gates 5, 5.1 and 5.2.
- **Legacy gates:** gates 1, 2, 2.1, 2.2 and 3 run their checks and are labeled `legacy` in the lock.
- **Harness fixtures that run before any table.**
- **Distinct opposition.** Wrappers and duplicate rows are deleted (the list is in §2.4).
- **Binding as a type:** `BodyStore`, then `Bound`.
- **A body refusal is a report.**
- **Declared grants.**
- **The far side at both hosts.**
- **Findings written by Cursor from command output:** the inner reason, the adversary, boundary depth, the R50 update, R55 and R56.

### Out of scope (Cursor refuses these even when they look small)

| Not now | Why it is tempting |
|---|---|
| Any change to how links are typed, delivered or ordered | `run.rs` is open for §2.6 |
| Embeddings across a link (R56) | Unchanged |
| A new file kind (`.events`, `.script`, `.lens`) | P52-13's scenario is written as a test |
| Random or generated mutants | The catalogue is closed |
| Mutating `.cell`, `.desc`, `.trace` or `.rs` artifacts | That is the legacy upgrade, a later phase (R64) |
| Upgrading any legacy gate's controls | Later phase (R64) |
| A third linked body in `universe.universe` | The adversary lives in its own file |
| Any 2-block, filling or homology | Phase 4 |
| A new primitive, a line in `grandfather.txt`, a reblessed golden | Never |

---

## 2. Decisions

All are **DECIDED**. §2.1–§2.8 are unchanged in meaning from Draft 0.2. §2.9–§2.16 are new or replaced.

### 2.1 A control sees a parsed value, never bytes

The harness parses each control artifact by its file kind: `.body`, `.universe`, `.lock`, a transcript (`corpus/transcripts/*.txt`, as a list of lines), or text (any other `.txt`, as one trimmed string). It hands the control the parsed value (`Subject::Body(Body)`, `Subject::Universe(Universe)`, and so on). If the real artifact does not parse, the run is refused, naming the item and the parser's refusal. `artifact_loads` is deleted. `joinn-gate` names no DNA, link or lock type: `GateItem` becomes generic over the subject, or it moves to xtask. Cursor chooses which.

**Applies to gates 5, 5.1 and 5.2.** Legacy gates are covered by §2.13.

### 2.2 Damage keeps the form

Each non-legacy gate item declares `opposes: Mutation` from a closed catalogue per file kind. The harness applies that mutation to the parsed subject, reprints it with the kind's canonical printer, and re-parses it. For bodies and universes, the mutant's coding hash must differ from the original's.

The control must answer:

- `false` on the real subject;
- `true` on the mutant;
- `false` on the kind's *neutral edit* (one regulatory label changed), for kinds that have one.

Any violation refuses the run, naming the item and which condition failed. A mutation whose target doesn't exist in the subject is refused, naming the target.

**The catalogue** (closed):

| Kind | Mutations |
|---|---|
| `.universe` | `DropLink(id)` · `FlipMark(link, member)` · `ShiftPort(link, member, to)` · `SwapBinding(alias, to_hash)` · `CorruptHash(alias)` (last hex digit +1 mod 16) · `CopyMember(lens, alias, into_system)` · `DropLens(name)` · `WireAcross(link)` · `DropGrant(link)` · `RenameLink(from, to)` · `RenameAlias(from, to)` |
| `.body` | `DropWire(src, dst)` · `DropGenome(instance)` · `SwapCell(instance, to_hash)` |
| `.lock` | `SetScore(phase, n, total)` · neutral: add a comment line |
| transcript | `DropLine(i)` · `SwapLines(i, j)` · no neutral edit |
| text | `Replace(s)` · no neutral edit |

**What each item opposes:**

| Gate · item | Subject | Opposes | The control answers `true` when |
|---|---|---|---|
| 5 · 1 Something crosses | `phase5/universe.universe` | `DropLink(e0)` | `units` does not fire |
| 5 · 2 The membrane is measured | `phase2/calculator.body` | `DropWire(cli_a@1, sum@0)` | ∂ ≠ `{cli_a@0, cli_b@0, sum@2}` |
| 5 · 3 The universe is well-formed | `phase5/universe.universe` | `ShiftPort(e0, calc.sum@2, 9)` | assembly refuses naming *no such port* |
| 5 · 4 Exclusivity holds | `phase5/universe.universe` | `CopyMember(function, units, calculation)` | exclusivity refuses naming `units` |
| 5 · 5 Two lenses, one body | `phase5/universe.universe` | `DropLens(deployment)` | fewer than two lenses place `units` |
| 5 · 6 Law 4 is a check | `phase5/universe.universe` | `WireAcross(e0)` | Law 4 refuses naming the container |
| 5 · 7 A capability can be revoked | `phase5/universe.universe` | `DropGrant(e0)` (after P52-11; before it, `phase5/ordered.universe` with `DropLink(path)`) | `units` does not fire before any revoke |
| 5 · 8 A refusal stays home | `phase5/controls/inner_reason.txt` | `Replace("e0")` | the hosts' far-side output contains the text |
| 5.1 · Two hosts, one universe | `transcripts/universe.txt` | `SwapLines(2, 3)` | the transcript does not start with `calculator.txt` |
| 5.1 · Tails are out, heads are in | `phase5/universe.universe` | `FlipMark(e0, calc.sum@2)` | typing refuses naming `calc.sum@2` |
| 5.1 · Members share a frame | `phase5/universe.universe` | `SwapBinding(units, <echo.body hash>)` | typing refuses naming `Text 1` and `ℤ 1` |
| 5.1 · The boundary is total | `phase52/controls/missing_cell.body` | `DropGenome(orphan)` | ∂ is computed with no refusal |
| 5.2 · Binding is by store | `phase5/universe.universe` | `CorruptHash(calc)` | binding refuses naming `calc` |
| 5.2 · A refusal is a report | `transcripts/universe.txt` | `DropLine(1)` (line 1 counted from 0 is the refusal line) | the CLI, run on the transcript's inputs, prints a line the subject lacks |
| 5.2 · The host knows no ids | `phase5/ordered.universe` | `DropGrant(path)` | `units` does not fire under the CLI |

Rows for gate 5.2 are added when that item is created (P52-09, P52-12, P52-16), not before.

**If a row turns out wrong** (the mutation can't reach the fact, or no honest control can see it), that is a snag. Keep that item's old hand-written control artifact, write down which condition failed, and continue. Claude settles it at the stop.

### 2.3 The harness tests itself before any table runs

`run_gate_table` runs four fixture items before the first table:

1. a control that ignores its subject must be refused;
2. a control that answers `true` on any change, including the neutral edit, must be refused;
3. a control whose declared mutation targets something missing must be refused, naming the target;
4. an honest control must be admitted.

If any fixture comes out wrong, `gate all` stops before any gate runs.

The lock gets a round-trip fixture: `write_lock` takes a path; a two-item fixture table's score is written under `target/`, read back and compared; an off-by-one writer is refused naming the phase.

Gate 5.1 items *Every control reads its artifact* and *The lock is this run*, and gate 3 item 9 *The path of truth*, are deleted as gate items. Deleted with them: `insensitive.rs`, `off_by_one.lock`, `ignores_bytes.rs`, `g51_reads*`, `g51_lock*`, `g3_lock*`.

### 2.4 A gate row opposes its own mutation, and the duplicate list

Across gates 5, 5.1 and 5.2, no two items share `(control_artifact, opposes)`. The harness checks this once, before the fixtures, and refuses naming both items. V86 (the pointer-distinctness test) stays.

**Deleted rows** (every wrapper whose body is one call to another gate function, plus the row it served). This list is final:

| Gate | Row deleted | Why |
|---|---|---|
| 2 | the corpus verifies | wrapper (`corpus_verify` / `p22_corpus_control`); phase 0 and gate 2.2 cover it |
| 2 | references agree | wrapper of gate 2.1's row |
| 2 | the transcript matches | control is a wrapper of gate 2.1's row |
| 2 | the trace replays | wrapper of gate 2.1's row |
| 5.1 | Something crosses | wrapper of gate 5 item 1 |
| 5.1 | Revocation stops delivery | wrapper of gate 5 item 7 |
| 5.1 | A refusal stays home | wrapper of gate 5 item 8 |

Gate 5.1's *Bodies are bound by hash* moves to gate 5.2 as *Binding is by store* (P52-09). Gate 5.1's *Links are typed* splits into *Tails are out, heads are in* and *Members share a frame*. After this phase, gate 2 has four items, gate 3 has eight, gate 5 has eight, gate 5.1 has four and gate 5.2 has three.

### 2.5 Binding is a type

`BodyStore` holds bodies keyed by their computed coding hash. Its only way to add a body is `insert(body, cells, source_path)`, which computes the key. It **refuses a second body with the same coding hash and a different regulatory region, naming both source paths** (§2.10).

`bind(&Universe, &BodyStore) -> Verdict<Bound>` is the only constructor of `Bound`. `assemble_universe`, `check_link_types` and `UniverseState::new` take `&Bound`. A trybuild fixture that builds a `Bound` from a `BTreeMap` fails to compile. `bind_bodies` is deleted. Law 5's refusal is *"alias `calc` declared `abcd1234` and the store holds no such body."* The forgeries in `g51_hash` and `crossing.rs::wrong_hash_names_both_shorts` are deleted. `wrong_hash.universe` is renamed `alias_is_local.universe` and becomes a positive control: it binds.

### 2.6 A body refusal is a report, and the universe keeps running

When a body refuses during `run`, the runtime records `UniverseReport::Refused { body, instance }` (no text) and carries on. `LinkRefusal { kind: Refused }` is added only when the refused activation consumed a value that a link delivered **in the same pass**; the delivery record is cleared at the start of every pass. `run` returns `Verdict::Refused` only for its own failures (budget exhausted, unknown alias). The host, as the near side of `calc`, may call `describe_refusal` on `calc` and print the reason.

### 2.7 Grants are declared by the universe

The universe's coding region gains `grants { e0: units }`, one line per capability. It is parsed, printed canonically and hashed. `UniverseState::new` starts with exactly the declared grants. `revoke` still works at runtime. Hosts never call `grant`. A grant on an unordered link, or to a body that isn't a member of the link, is refused at parse time, naming both. Universe hashes move and are recorded; no body or cell hash may move.

### 2.8 The far side is a host line, built from the type

The CLI prints every `UniverseReport::Link` through a presenter `fn(&LinkRefusal) -> String`. The test host's `Capture` gains `far_side: Vec<LinkRefusal>` and a derived `intent_set`. The CLI injects every line into the universe, and the side `BodyState` is deleted. `universe.txt` does not change. The CLI's output must be unchanged, apart from the renamed names, under `RenameLink(e0, e1)` and `RenameAlias(units, meters)`. Double delivery can't happen under the interactive CLI (R59).

### 2.9 How the work is checked (replaces Draft 0.2 §2.9)

- **No file is typed by AJ.** Rules 38 and 46 are deleted. Every finding this phase needs is written by Cursor, either from command output or from text given verbatim in this plan.
- **Stops, not checkpoints.** A chunk ends with the stop report (§0.2), a commit and a push.
- **Claude verifies** at each stop, from a fresh clone on Linux. It runs `cargo test --workspace --no-fail-fast`, `gate all`, `corpus verify`, `vocab` and `modules`, reads the diffs of the chunk's commits against this plan, and reports to AJ in plain English. Anything wrong comes back as a short plan amendment.
- **CI** runs the same commands on Windows and Linux on every push (§2.12).
- `cargo xtask witness` stays as a tool, and `requires_acceptance` is deleted. No done-when depends on `witness`.
- **Checkpoint A** is closed. Its skeleton box is replaced with a closing note (P52-02b), and `phase-5.2-stop-a.md`, the baseline Claude recorded, is committed beside it.

### 2.10 One body, one face

A body's coding hash does not include its regulatory region (labels, prompts, presentation). Today `calculator.body`, `calculator_b.body`, `calculator_c.body` and `calculator_d.body` all have coding hash `b55fba1e…`, but they have different labels. The CLI picks whichever file the directory listing returns first. That happens to be right on Windows and wrong on Linux, where the calculator prints `ignored-in-hash`.

**Rule:** one coding hash has one regulatory region in any set of bodies a host loads. Loading a second body with the same coding hash and a different regulatory region is refused, naming both files.

- `calculator_b.body`, `calculator_c.body` and `calculator_d.body` move to `corpus/phase2/variants/`. `corpus/hashes.txt` keys them by name, so no entry changes. Update every path that names them.
- Hosts skip any folder named `variants` or `controls` when they gather bodies.
- Before P52-09 the refusal lives in the CLI's and test host's body loaders. From P52-09 on, it lives in `BodyStore::insert`.

### 2.11 Nothing depends on folder order or empty folders

- **Every directory walk sorts its entries by file name** before using them: loaders, scans, `corpus verify`, perf. Walks that collect results into a `BTreeMap` still sort, so the first file wins the same way everywhere.
- **The empty-folder loophole:** the `modules` scan treats `foo.rs` as a capsule root (exempt from the one-function rule) whenever a folder `foo/` exists, **even an empty one**. AJ's disk has empty folders `xtask/src/modules/scan/`, `xtask/src/modules/count/`, `crates/joinn-prim/src/floor/check_oppositions/` and `crates/joinn-prim/src/seals/check_seal_dag/`. Git doesn't store empty folders, so a fresh clone reports `xtask/src/modules/scan.rs: leaf has 8 production fn(s)`, while AJ's disk reports `modules: ok`.
  - Fix: a capsule root requires its folder to contain at least one `.rs` file. Add a fourth `modules` fixture: a two-function leaf beside an empty folder must be reported.
  - Split `scan.rs` (and anything else the fixed scan reports) by rule 25. Don't silence it and don't grandfather it.
  - Cursor deletes the four empty folders on AJ's disk.

### 2.12 CI that runs

`joinn/.github/workflows/ci.yml` is in the wrong place: GitHub only reads `.github/` at the repository root (`D:\JoInn\.github\`), so the checks have never run. Move it to the root.

- `working-directory: joinn`
- a matrix of `ubuntu-latest` and `windows-latest`
- steps: fmt, clippy `-D warnings`, `cargo test --workspace --no-fail-fast`, `vocab`, `modules`, `corpus verify`, `agree`, `power`, `gate all`
- until P52-16 the `gate all` step has `continue-on-error: true`, because gates are expected to be red mid-phase. P52-16 removes that line.

### 2.13 Legacy gates

Gates 1, 2, 2.1, 2.2 and 3 are **legacy** for this phase:

- The harness runs each item's `check` and requires it to pass. Their controls are not run, and the Phase 5.1 byte-damage rule is deleted for everyone.
- The lock marks them: `phase 2: 4/4 legacy`.
- `gate all` passes only if every legacy check passes and every non-legacy gate passes in full.
- No legacy control file is deleted or edited, except the rows §2.4 deletes.
- Upgrading them to the catalogue is R64, a later cleanup phase.

This ends the `0/0` rows.

### 2.14 Rule 13 and the forbidden words apply to JoInn's language

The concept-word bans (rule 13's `sub`/`subtract`/`minus`, and `schema`, `metadata`, `backend`, `genotype`, and the floor words) keep those concepts out of **JoInn's language**. `vocab` enforces them on:

- corpus files (`.cell`, `.body`, `.universe`, `.desc`);
- names this project declares in Rust (`fn`, `struct`, `enum`, `const`, `static`, `mod`, `trait`, `type`, fields, enum variants and `let` bindings).

A match that is a call into Rust's standard library is exempt: immediately after `.` (method call), or after a `std::`, `core::`, `fs::`, `io::` or `u32::`/`u64::`/`usize::`/`i64::` path segment.

- The Rust-engineering bans (`HashMap`, `HashSet`, `f32`, `f64`, `thread_rng`) still scan all Rust, unchanged.
- Delete every `allow(vocab)` silencer that exists only because of a standard-library call. That includes the three `saturating_sub` ones in `joinn-live/src/state/`, the two in `universe_state/run.rs`, the one in `g51_lock.rs` and the two `fs::metadata` ones in `witness/against_mtime.rs`.
- Add a `vocab` fixture pair: `fn sub_total()` must be reported, and `x.saturating_sub(1)` must not be.
- R63 is closed as *decided: JoInn's language only*.

### 2.15 The inner reason is what the probe printed

`cargo xtask probe-refusal` runs the double-delivery scenario. Its **first line** is the probe label on `units`, and each following line is one far-side presenter line. Cursor writes `inner_reason.txt` with a shell redirect of that first line (`cargo xtask probe-refusal | Select-Object -First 1 > …` on PowerShell), never by retyping it. The check requires that the probe label contains the file's text and that no far-side line does. `Replace("e0")` must make the far-side check fire.

### 2.16 The adversary (specified here, attempted by Cursor)

**Spec**, which Cursor copies verbatim into `docs/Findings/law-4-adversary-spec.md`:

> *A `lookup` body asks `units`, "what factor did you last use?", and must pair each answer with the question that caused it. It is built only from what exists: wires inside bodies, hyperedges between them, and declared grants. It lives in `corpus/phase52/adversary/` with its own universe file. The attempt succeeds if the universe assembles and a run shows two questions each paired with its own answer. If Law 4 or typing refuses it, the refusal line is the result.*

Cursor writes `docs/Findings/law-4-adversary.md` from what happened, quoting every printed line. It uses **fired** only if the body cannot be expressed under Law 4 (the law refused something the spec needs). It corrects the Phase 5 entry to *"held — the attempt did not test Law 4"*. If pairing needs an order across two links, R58 in the research backlog gains that example. Claude grades the finding at stop D.

---

## 3. Architecture

No crates and no dependencies are added. `joinn-gate` stays free of DNA, link and lock types. `joinn-link` gains `BodyStore` and `Bound`. The universe grammar gains `grants`. `xtask` gains `mutate/`, `probe-refusal`, the harness fixtures, and the legacy marking.

```rust
pub enum Subject { Body(Body), Universe(Universe), Lock(Vec<LockRow>), Transcript(Vec<String>), Text(String) }
pub enum Mutation { DropLink(..), FlipMark(..), ShiftPort(..), SwapBinding(..), CorruptHash(..), CopyMember(..),
                    DropLens(..), WireAcross(..), DropGrant(..), RenameLink(..), RenameAlias(..),
                    DropWire(..), DropGenome(..), SwapCell(..), SetScore(..), DropLine(usize), SwapLines(usize, usize),
                    Replace(&'static str) }
fn mutate(s: &Subject, m: &Mutation) -> Verdict<Subject>;   // reprint + reparse inside
fn neutral(s: &Subject) -> Option<Subject>;                  // None for transcript and text

pub struct BodyStore { /* private */ }
impl BodyStore { pub fn insert(&mut self, body: Body, cells: Cells, source: &str) -> Verdict<Hash>; }
pub struct Bound { /* private */ }
pub fn bind(u: &Universe, store: &BodyStore) -> Verdict<Bound>;
pub enum UniverseReport { Fired { body, instance }, Refused { body, instance }, Link(LinkRefusal) }
```

**`corpus/phase52/controls/missing_cell.body`** is `units.body` with a second genome entry `cell:<false_law.cell hash> as orphan` and no wires to it. The store only holds admitted cells, so ∂ is refused naming `orphan`. `DropGenome(orphan)` gives back `units.body`, whose ∂ is `{scale@0 In ℤ, scale@1 In ℤ, scale@2 Out ℤ}`. The old `phase51/controls/missing_cell.body` is deleted.

**Hand-written controls that become mutants:** after P52-04, for each of `unlinked.universe`, `no_such_port.universe`, `wrong_direction.universe`, `frame_mismatch.universe`, `two_systems.universe` and `wrong_container.universe`, run the catalogue mutation that imitates it and compare canonical text. **Delete the file only if some mutant reproduces it byte for byte.** Otherwise keep it, and say in the commit which mutation came closest. `transits.universe` stays.

---

## 4. The Commits

Done-when is a command, and the command must be able to fail.

**Already done (Sitting A):** P52-00, P52-01, P52-02 and P52-02a (P52-02a is partly reverted by P52-02b).

### Chunk B: repair, then controls that can be wrong

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P52-02b** | **Workflow repair** | Replace `AGENTS.md` with Appendix A. Make `.cursor/rules/joinn.mdc` a pointer to `AGENTS.md` (Appendix B). Commit this plan file. Add `phase-5.2-stop-a.md` (Appendix C) and close checkpoint A (Appendix C). Delete `requires_acceptance`. §2.10 (move variants, hosts skip `variants`/`controls`, duplicate-face refusal in both hosts' loaders). §2.11 (sorted walks, capsule-root fix and fixture, split `scan.rs`, delete the 4 empty folders). §2.12 (CI at the root). §2.14 (vocab scope, fixtures, silencers removed) | On a fresh clone (`git clone` into a temp folder, then `cargo test --workspace --no-fail-fast` there): `universe_transcript_matches_golden` passes. `cargo xtask modules` reports the planted empty-folder fixture and then `modules: ok`. `cargo xtask vocab` → `vocab: ok`, and `git grep -n "allow(vocab)" -- crates xtask` shows no line containing `saturating_sub` or `fs::metadata` (paste its output in the report). A test that loads `calculator.body` and `variants/calculator_c.body` into one set is refused naming both paths |
| **P52-03** | **Parsed subjects (§2.1)** | `Subject`; the harness parses by kind for gates 5 and 5.1; every non-legacy control takes `&Subject`; `artifact_loads` and `damage_bytes` deleted; legacy marking (§2.13) | A fixture artifact that doesn't parse refuses the run naming the item and the parser's reason. `gate all` prints `phase 2: <n>/<n> legacy` (no more `0/0`). No file under `xtask/src` defines or calls `artifact_loads` |
| **P52-04** | **The catalogue (§2.2)** | `mutate` and `neutral` for every row | One test per mutation on a real corpus file: the mutant re-parses, its hash differs (bodies, universes), a missing target is refused naming it, **and one downstream effect is asserted** (e.g. `DropLink(e0)` → `units` does not fire). No test asserts only that the mutant differs |
| **P52-05** | **Opposition declared and enforced** | `opposes` on every non-legacy item per §2.2 | `gate all` is **refused**, listing every item that fails a condition. Predicted: at least 5·2, 5·8, 5.1's hash item, 5.1's total item. The list goes in the commit report. **Nothing is fixed in this commit** |
| **P52-06** | **The harness tests itself (§2.3)** | four fixtures, lock round-trip, `write_lock(path)`; the three items and files in §2.3 deleted | Breaking fixture 1 makes `gate all` stop before any gate prints a row, shown then reverted (both outputs in the report). Gate 3 prints eight rows |
| **P52-07** | **Distinct opposition and the duplicate list (§2.4)** | uniqueness check; the seven rows in §2.4 and their wrapper files deleted; phase labels from one `const` in `gate_all.rs` | Giving 5.1's *Tails are out* item the same `(artifact, opposes)` as 5·3 refuses naming both, shown then reverted. Gate 2 prints four rows. Gate 5.1 prints only non-wrapper rows |
| **P52-08** | **Every control made honest** | each control from P52-05's list rewritten to decide from its subject; new `missing_cell.body`; §3's control-file deletions | `gate all` passes the harness (fixtures, mutant, neutral, uniqueness). Items may still fail as checks: 5·8 is expected to until P52-13. `cmp` of each deleted file against its mutant is quoted |
| — | **Stop B** | `phase-5.2-stop-b.md` (§0.2), push, stop | — |

Dependencies: P52-04 needs P52-03. P52-05 needs P52-04. P52-06 and P52-07 need P52-05. P52-08 needs P52-07.

### Amendment B (after Stop B, 25 Sep 2026): do these first in chunk C

Claude re-ran the tree at `618d3d7` from a fresh clone on Linux. Every number in `phase-5.2-stop-b.md` reproduced exactly (168 passed, `gate all` 7/8 on phase 5 with only 5·8 failing, `vocab: ok`, `modules: ok`, 33 hashes). The findings below are fixed by three commits that come **before P52-09**. They are decided; nothing waits on AJ.

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P52-08a** | **CI can go green** | `cargo fmt --all` (one mechanical commit, no other change in it). Fix every `cargo clippy --workspace --all-targets -- -D warnings` error, starting with `collapsible_match` at `crates/joinn-live/src/slot/apply.rs:25`. Fix the code, never `allow` a lint. No coding hash may move | `cargo fmt --all -- --check` exits 0. `cargo clippy --workspace --all-targets -- -D warnings` exits 0. `corpus verify` unchanged. Paste the last line of each |
| **P52-08b** | **A control never fails open** | In `g5_assemble_control`, `g51_tails_control` and `g51_frame_control`, every early exit before the control's named refusal (wrong subject kind, loader error, **binding refused**) returns `false`, not `true`. The control returns `true` only when it sees its own named refusal. `g5_law4_control` returns `true` only when the Law 4 refusal names **both** bodies (`calc` and `units`) and contains `hyperedge`; this replaces §2.2's "naming the container", which does not match what Law 4 prints. Leave `g5_linked_control`, `g5_revoke_control` (their fact is "units does not fire", which a refusal also shows) and `g5_locality_control` (P52-13) as they are | A test runs each of those four controls on the `CorruptHash(calc)` mutant of `universe.universe` and asserts `false` for all four. `gate all` still passes the harness with phase 5 at 7/8 and phase 5.1 at 5/5 |
| **P52-08c** | **Catalogue tests assert what the mutant does** | Rewrite these P52-04 tests so each asserts one effect outside the mutated value itself (§5): `Replace("e0")` on `inner_reason.txt` → `g5_locality_control` answers `true`; `SetScore` → `scores_match` against the real rows is refused naming the phase; `DropLine(1)` on `universe.txt` → `g51_hosts_control` still answers `false` and the mutant has no line equal to the original line 1; `SwapCell` → `membrane` is refused, or ∂ differs from `{cli_a@0, cli_b@0, sum@2}` (one plain assertion, no `\|\|` escape); `RenameAlias(units, meters)` → the renamed universe still binds and assembles (delete the assertion that `units` stops firing; it tests the helper's hard-coded alias, and §2.8 wants the opposite). No test may accept `Err(_)` as a pass. In `set_score.rs`, use `PHASE_LABELS` and delete the letter-by-letter phase string and its "banned literal" comments (rule 42: there is no such scan). Rename `xtask/vocab_fixtures/accept_saturating_sub.rs` to `accept_std_call.rs` and delete the `allow(vocab)` that its path needed | `git grep -n "allow(vocab)" -- crates xtask` shows no line containing `saturating_sub` or `fs::metadata` (paste the output). `git grep -n "banned literal" -- xtask` prints nothing. The suite passes |

**Changes to later commits:**

- **P52-09:** the binding refusal prints the alias and the **full 64-hex declared hash**, not a 4-character prefix. Today it prints `alias calc declared b55f and no body was supplied`, which is the same prefix for the real and the corrupted hash, because `CorruptHash` changes the last digit. The done-when's "naming `calc` and the short hash" becomes "naming `calc` and the full declared hash".
- **Every commit message:** the `Commit:` line carries the hash the commit actually has in `git log` (P52-02b's message says `30ba982`; the commit is `8a7229a`). When the hash is only known after committing, write `Commit: P52-NN (hash in git log)` in the message and the real hash in the stop report.
- **"Shown then reverted" done-whens:** paste both printed lines in full. Do not shorten them with `(...)`.
- **P52-08's control-file comparison** carries forward into the stop-C report: for each of the five kept files, name the catalogue mutation that came closest and paste the first differing line.

### Chunk C: binding, runtime, hosts

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P52-09** | **Binding is a type (§2.5)** | `BodyStore` (with the one-face refusal moved in from the loaders), `Bound`, `bind`; consumers take `&Bound`; trybuild fixture; `alias_is_local.universe`; forgeries deleted; gate 5.2 created with *Binding is by store* | The trybuild fixture fails to compile. `CorruptHash(calc)` is refused naming `calc` and the short hash. `alias_is_local.universe` binds. `bind_bodies` no longer exists. `BodyStore::insert` of `calculator.body` then `variants/calculator_c.body` is refused naming both |
| **P52-10** | **A body refusal is a report (§2.6)** | `UniverseReport::Refused`; delivery record cleared per pass | Test: inject `"two"` at `calc.cli_a@0`, run, then `2`, `3`, `12`, run. The reports contain `Refused { calc, cli_a }`, then `units` fires holding `60` read via `describe`. Same test, after one full crossing: inject a second `12` into `units.scale@1` from the host. `units` refuses, and the reports contain **no** `LinkRefusal` |
| **P52-11** | **Declared grants (§2.7)** | parser, printer, hash; `grants { e0: units }` in `universe.universe`, `grants { path: units }` in `ordered.universe`; hosts stop calling `grant` | `DropGrant(e0)` → `units` does not fire. A grant on an unordered link is refused at parse time naming the link. `git grep -n "grant(" crates/joinn-cli crates/joinn-test-host` finds no call. The moved universe hashes are listed |
| **P52-12** | **Hosts carry the far side (§2.8)** | CLI injects every line; side `BodyState` deleted; presenter; `Capture.far_side` and `intent_set`; gate 5.2 items 2 and 3 | `"two\n2\n3\n12"` piped to `joinn run universe` prints `universe.txt` byte-identically, and `joinn run calculator` prints `calculator.txt`. On `RenameLink(e0, e1)` and `RenameAlias(units, meters)` the output is unchanged apart from the names. The test host's `intent_set` for `units` is `{scale@1}` |
| **P52-13** | **G6 at the hosts (§2.15)** | `cargo xtask probe-refusal`; `inner_reason.txt` rewritten by redirect; 5·8 reads `far_side`, the presenter lines and the probe | The probe label contains `inner_reason.txt`. No far-side line does. `Replace("e0")` makes the far-side check fire. `crossing.rs::second_sum_stays_home` and gate 5·8 both pass |
| — | **Stop C** | `phase-5.2-stop-c.md`, push, stop | — |

Dependencies: P52-08a, P52-08b and P52-08c come first, in that order. P52-09 needs P52-08b. P52-11 needs P52-09. P52-12 needs P52-10 and P52-11. P52-13 needs P52-12.

### Chunk D: findings and the freeze

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P52-14** | **The adversary (§2.16)** | spec file, the attempt in `corpus/phase52/adversary/`, `law-4-adversary.md` | The finding quotes the run or refusal lines and uses **fired** only as §2.16 defines. The Phase 5 entry is corrected |
| **P52-15** | **The other findings** | `boundary-depth.md` with the text in Appendix D; `r50-membrane-cost.md` gains the latest `cargo xtask perf` line, pasted; R55, R56, R59–R62 and R64 added to `docs/Theory/JoInn Research Backlog.md` from Appendix D; R63 marked decided | The perf line in the finding matches a fresh `cargo xtask perf` run except for the timing numbers |
| **P52-16** | **Re-freeze, docs, gate 5.2, lock** | new goldens; `phase-5.2-hashes.md`; README, `Guides/03`, `decisions.md` rows V98–V111 and R59–R64; CI's `continue-on-error` removed | `cargo xtask gate all` from a fresh clone prints the fixtures first, then phases 0, 1, 2, 2.1, 2.2, 3 (legacy), 5, 5.1 and 5.2, with every non-legacy item passing all three conditions, and exits 0. `corpus verify` matches. `phase-5.2-hashes.md` confirms no Phase 0–3 coding hash moved |
| — | **Stop D** | `phase-5.2-stop-d.md`, push, stop | — |

If something has to be cut, cut P52-14 and carry the adversary forward with a line in `decisions.md`. Never cut P52-02b, P52-03 through P52-06, or P52-10.

---

## 5. Test Strategy

| # | Invariant | Commit |
|---|---|---|
| **V98** | A control never sees bytes; an artifact that doesn't parse refuses the run | P52-03 |
| **V99** | Every catalogue mutant parses, moves the hash, and has a named downstream effect | P52-04 |
| **V100** | Each non-legacy control flips on its declared mutant | P52-05 |
| **V101** | Each non-legacy control ignores its neutral edit | P52-05 |
| **V102** | The harness fixtures come out right before any gate runs | P52-06 |
| **V103** | No two non-legacy items share `(artifact, opposes)` | P52-07 |
| **V104** | A `Bound` can only come from a store | P52-09 |
| **V105** | A hash the store doesn't hold is refused naming the alias | P52-09 |
| **V106** | A body refusal is a report, and the universe keeps running | P52-10 |
| **V107** | `LinkRefusal::Refused` only for a delivery in the same pass | P52-10 |
| **V108** | Grants are declared; no host calls `grant` | P52-11 |
| **V109** | A host's output doesn't depend on link ids or aliases | P52-12 |
| **V110** | The far side is a kind at both hosts; the probe has the words | P52-13 |
| **V111** | One coding hash, one face; results don't depend on folder order or empty folders | P52-02b, P52-09 |

Carried forward and re-run on every commit: every earlier invariant, and V86 next to V103.

**A catalogue test must never** assert only that a mutant differs or that `mutate` returned `Ok`. It asserts what the mutant **does**.

---

## 6. Exit Gate 5.2

`cargo xtask gate 5.2`, from a fresh clone, after the harness fixtures pass:

- [ ] **1 · Binding is by store.** `universe.universe` and `alias_is_local.universe` bind. *Opposes* `CorruptHash(calc)`.
- [ ] **2 · A refusal is a report.** The CLI on `two, 2, 3, 12` prints `universe.txt`, with `"two"` refused inside the universe. *Opposes* `DropLine(1)` on `universe.txt`.
- [ ] **3 · The host knows no ids.** The CLI's output is unchanged under `RenameLink(e0, e1)` and `RenameAlias(units, meters)`. *Opposes* `DropGrant(path)` on `ordered.universe`.

### 6.1 Conditions for opening Phase 4

1. Gate 5.2 passes, `gate all` exits 0, and CI is green on Windows and Linux.
2. A value has crossed a typed link under both hosts, with the calculator's refusal happening inside the universe.
3. `boundary-depth.md`, the corrected `law-4-adversary.md` and the R50 update exist.
4. Claude's stop-D review lists no open snag, or AJ has chosen in conversation to carry each remaining one forward.
5. R55, R56 and R60 are in the backlog. Whether Phase 4 ships its own mutation catalogue (R60) is decided with AJ before Phase 4's plan is written.

---

## 7. Risks

| Risk | What to do |
|---|---|
| The catalogue is tested against itself | Each mutation test names a downstream effect; if none can be named, drop the mutation (a snag) |
| Controls get written to the mutation rather than the fact | Claude checks at stops; a second mutation is added only if it shows up |
| A scan is evaded | Rule 42: never write code to avoid a scan. Report it as a snag |
| A report summarises instead of quoting | Claude re-runs the command at the stop and compares |
| `grants` moves a hash that shouldn't move | Only universe hashes may move. A body hash moving is a snag and the commit is not continued past |
| Something passes on one OS and fails on the other | CI matrix; Claude's Linux run |

---

## 8. Open Items

| ID | Topic | Question |
|---|---|---|
| **R59** | Interactive hosts and races | In canonical prompt order, the CLI can't deliver two values into one port before its partner arrives. Is that a property or a limitation? |
| **R60** | Every file kind ships its mutants | Should a new file kind be required to come with its mutation catalogue before any control may point at it? |
| **R61** | Whose budget is it | Is a body's budget its own (declared) or borrowed from its container? |
| **R62** | One host, both sides | When a host presents several bodies, does it show a link-caused refusal's reason (near side) or only its kind (far side)? |
| **R63** | Vocabulary vs the host language | **Decided 24 Sep:** bans apply to JoInn's language only (§2.14) |
| **R64** | Legacy gates | Upgrade gates 1–3 to parsed subjects and a catalogue that covers `.cell`, `.desc`, `.trace` and `.rs`. That will be a short cleanup phase |

---

## Appendix A · `AGENTS.md` (full replacement for the header and rules 24, 38, 41–48)

Replace the header paragraph with:

```markdown
# JoInn — standing rules

This repo is JoInn. Phase 5.2 is a correction phase. Its one idea is DAMAGE MUST
KEEP THE FORM: a control sees a parsed value and must flip on a mutant that still
parses. The build plan is docs/Plans/JoInn Phase 5.2 Implementation Plan.md.
Work one CHUNK at a time (B, C or D), one git commit per numbered step, and stop
at the chunk's stop report. Every decision is in the plan. Never stop to ask;
follow the plan's Snags section instead.

There is no renderer, no homology, no third linked body and no compiler in this
phase. Phase 4 runs after it.
```

Keep rules 1–23 (rule 13 gains the scope sentence below), 25–37, 39 and 40 exactly as they are. Replace rule 13, rule 24 and rule 38. Delete rule 46. Append rules 41–45 and 47–48:

```markdown
13. `sub`, `subtract` and `minus` are not names in JoInn's language: corpus files
    and names this project declares in Rust. Calls into Rust's standard library
    (`x.saturating_sub(1)`, `fs::metadata`) are not JoInn's language and are not
    scanned by the concept-word bans.
24. A CONTROL IS AN ARTIFACT, PARSED, AND OPPOSED BY A MUTATION THAT KEEPS ITS
    FORM (gates 5 and later). The harness parses the artifact by kind and passes
    the parsed subject; a control never sees bytes. The control must answer
    false on the subject, true on the declared catalogue mutant, and false on the
    kind's neutral edit, or the run is refused naming the item. The catalogue is
    closed. Gates 1–3 are LEGACY: their checks must pass, their controls are not
    graded, and the lock says `legacy`. joinn-gate names no DNA, link or lock type.
38. NOTHING IS TYPED BY AJ. AJ decides in conversation with Claude; the plan
    carries every decision. Findings are written by the agent from command
    output or from text given verbatim in the plan. Claude verifies each chunk
    from a fresh clone.
41. A GATE ROW OPPOSES ITS OWN MUTATION. No two non-legacy items share
    (control_artifact, opposes). A row that restates another gate's fact is
    deleted, not wrapped.
42. NEVER WRITE CODE TO AVOID A SCAN. If a scan blocks a correct change, write it
    as a snag and continue with the next step.
43. BINDING IS A TYPE. A `Bound` comes only from `bind(&Universe, &BodyStore)`,
    and a `BodyStore` computes every key it holds. One coding hash has one
    regulatory region in a store; a second, different one is refused naming both
    files.
44. A BODY REFUSAL IS A REPORT. The universe records `Refused { body, instance }`
    and keeps running. A link refusal is reported only for a delivery made in the
    same pass. `run` never returns a body's reason.
45. A HOST KNOWS NO LINK IDS OR ALIASES. Grants are declared in the universe's
    coding region; hosts never call `grant`. The far side is presented from a
    `LinkRefusal` alone.
47. EVERY COMMIT ENDS WITH THE REPORT BLOCK (plan §0.1), and every chunk ends
    with the stop report (plan §0.2), a commit and a push. Values are quoted from
    output. `vocab` and `modules` run on every commit. A commit message's id
    names the work the commit did.
48. NOTHING DEPENDS ON FOLDER ORDER OR EMPTY FOLDERS. Every directory walk sorts
    its entries by name. A capsule root needs a folder that contains `.rs` files.
    Results must be the same on Windows and Linux, and CI runs both.
```

## Appendix B · `.cursor/rules/joinn.mdc` (full replacement)

```markdown
---
description: JoInn standing rules
alwaysApply: true
---
Read and follow `AGENTS.md` in this folder. It is the only list of standing rules.
Do not keep rules here. The current plan is named in AGENTS.md's header.
```

`.cursor/rules/modules.mdc` stays as it is.

## Appendix C · Closing checkpoint A

Replace the whole skeleton box at the top of `docs/Findings/phase-5.2-checkpoint-a.md` (the `> **SKELETON …` paragraph) with:

```markdown
> **Closed 24 Sep 2026 under plan Draft 0.3.** The evidence blanks below were
> never filled. Checkpoints were replaced by stop reports (plan §0.2, §2.9). The
> tree at `7093897` was re-run by Claude from a fresh clone on Linux; see
> `phase-5.2-stop-a.md`.
```

Leave the rest of the file as it is.

`docs/Findings/phase-5.2-stop-a.md` is added by P52-02b with the text Claude supplies in the same folder (it is already in the working tree when P52-02b starts; commit it unchanged).

## Appendix D · Text for P52-15

**`docs/Findings/boundary-depth.md`:**

```markdown
# Boundary depth

A universe's complex has dimension at most 1 until Phase 4 adds fillings. So
∂∂ = 0 holds on every universe trivially and cannot refuse anything (rule 40).

"Touch-only" (a link may only touch ports on a body's membrane) is therefore NOT
enforced by ∂∂. It is enforced by C₀ being derived: the only 0-cells a link may
name are the ports of ∂(body), computed on demand (rule 30). A member naming an
interior port is refused because that port is not in C₀, and the refusal names
the port and the wire that consumes it (rule 31, `transits.universe`).

Phase 4 starts from this: touch-only is membership in a derived C₀. The first
place ∂∂ can do real work on a universe is a 2-block (a filling) whose boundary
is a cycle of links. Until one exists, no check, doc or finding may credit ∂∂
with a universe refusal.
```

**Research backlog entries** (append to `docs/Theory/JoInn Research Backlog.md`). Use the text of §8 for R59–R62 and R64. Add:

- **R55 · An augmented complex on a link.** When one tail fans out to several heads, is the value conserved (one delivery split, like a sum over heads) or copied (each head gets it all)? Example: `calc.sum@2` feeding both `units.scale@1` and a logger body. If augmentation ε counts deliveries, copying breaks ε∘∂ = 0.
- **R56 · Embeddings across a link.** May a link carry a value from one frame into another by an embedding (ℤ into ℚ)? Example: two bodies whose laws each hold on their own frame, joined by an embedding whose round-trip holds on one side and not the other. This is the most likely place for Phase 4's first H¹ candidate.

---

*JoInn Phase 5.2 Implementation Plan, Draft 0.3 with Amendment B (25 Sep 2026). It closes F44–F54 of the Phase 5.1 review and the three problems found on 24 Sep by running the tree on Linux. Every decision is made. Cursor executes. Claude verifies at each stop.*
