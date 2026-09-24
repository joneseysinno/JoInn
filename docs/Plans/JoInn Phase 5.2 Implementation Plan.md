# JoInn Phase 5.2 Implementation Plan

**Damage that keeps the form · a working plan for Cursor**

*Phase 5.1's correction: controls see parsed values and must flip on a mutant that still parses, binding is a type, a body refusal is a report, hosts carry the far side, and AJ's witness becomes a checkpoint the next commit depends on*

Author: AJ · Draft 0.2 · September 24, 2026 (§0.1, §2.9, P52-02a and rules 46–47 amended after Checkpoint A)

> Status tags follow Parts I–IV: **DECIDED**, **PROPOSED**, **OPEN**. **PROPOSED · yours** marks a recommendation made while writing this plan. You can overrule it; Cursor implements whatever you settle on, not what is written here.

> **What this is.** Phase 5.1 built the link model properly. It has a typed ∂, links that check direction and frame, a runtime that moves `5` across `e0` so that `units` holds `60`, and a capability checked on delivery. The phase review (`docs/Findings/phase-5.1-review.md`, F44–F54) found five problems. The tree was never run. The damage rule was met by parsing rather than reading. Three control artifacts exist only to be pointed at. The scans were evaded. And none of AJ's findings exists. This plan fixes that and nothing more. It does not touch the link model, except where the hosts and the runtime are wrong about refusals.

> **The one idea.** Phase 5.1's was *a control is only an artifact if damaging the artifact changes the control's answer.* That was right, but the damage chosen broke the file's form, so a control only had to notice that the file no longer parsed. This phase: **damage must keep the form.** A control never sees bytes. It sees a parsed value, and it must give a different answer on a **mutant that still parses** and differs in exactly the fact the control is about. It must also give the **same** answer on a neutral edit. That is rule 19's opposition applied to the controls themselves: each control must be shown to hear the change it is about and to ignore a change it is not about.

> **The corollary that pays for it.** Most hand-written control files in `corpus/phase5/controls/` are one edit away from `universe.universe`. Once there is a mutation catalogue, those files *are* mutants. So the catalogue makes them, and a file that the catalogue reproduces byte for byte is deleted. A control file then exists only where no catalogue mutation can reach it. That is exactly where a reviewer should look hardest.

> **The second corollary.** Rules that are only text get skipped: F47 is a scan evaded, and F53 is a rule-38 finding simply left missing. So this plan does not add a rule telling the builder to stop. It splits the phase into **four sittings**. Each sitting ends with a checkpoint: Cursor writes the evidence (what the commands printed), AJ writes the acceptance (his decisions and one sign-off line), and the first commit of the next sitting has a done-when that `cargo xtask witness` refuses while the checkpoint is older than the code or has no sign-off. *(Amended 24 Sep after Checkpoint A: AJ does not retype output a builder can read.)*

---

## 0. How to Drive Cursor With This Plan

| | |
|---|---|
| **Where this file lives** | `D:\JoInn\docs\Plans\JoInn Phase 5.2 Implementation Plan.md` |
| **Where the code lives** | `D:\JoInn\joinn\`, no new crates |
| **What it closes** | F44–F54 of `docs/Findings/phase-5.1-review.md`, plus Phase 5.1's unfinished P51-00, P51-16 (AJ half), P51-17 (AJ half) and P51-18 |
| **Standing rules** | `AGENTS.md` as of Phase 5.1, **amended by Appendix A**. Rule 24 is rewritten, and rules 41–47 are added |
| **Unit of work** | one numbered commit from §4, one Cursor session per commit. **Stop at the done-when. Do not start the next commit in the same session** |
| **Sittings** | four (A–D). A sitting ends at a checkpoint: Cursor writes the evidence, AJ writes the acceptance (§2.9). The next sitting's first done-when requires `cargo xtask witness <checkpoint>` to print `current` |
| **How a session reports** | every Cursor session ends with the report block in §0.1, and nothing else counts as done |
| **How a commit ends** | its done-when command reports a refusal, a disagreement, a compile error, or a byte-identical replay, never a number the same command chose |
| **What Cursor may decide** | module layout (rule 25), function bodies, error strings, how the mutation catalogue is represented, whether `GateItem` is generic or xtask-local (§2.1) |
| **What Cursor may not decide** | anything in §2; which mutation an item opposes (§2.2's table is AJ's); which duplicate gate row is deleted (§2.4, AJ approves the list); the adversary (P52-14); any file marked *typed by AJ* |

The prompt stays short:

> Implement commit **P52-09** from `docs/Plans/JoInn Phase 5.2 Implementation Plan.md`. Follow `AGENTS.md`. If the commit names a checkpoint, run `cargo xtask witness` on it first and stop if it prints `stale`. Stop when the done-when command passes and report what it printed. Never change code to avoid a scan; if a scan blocks a correct change, stop and report.

### 0.1 The session report · **DECIDED** (24 Sep, after Checkpoint A)

Every Cursor session ends with this block, filled in from what the commands actually printed. AJ reads it and answers only the **Needs AJ** lines.

```
Commit:     P52-NN <git short hash>   (the id must match the work; if the work is not P52-NN, say so)
Done-when:  <the command> → <what it printed>          MET | NOT MET
Suite:      cargo test --workspace --no-fail-fast → <N passed, M failed>
Scans:      vocab → <line> · modules → <line>
Unexpected: none | <each thing that differs from the plan's prediction, with the printed line>
Needs AJ:   none | <each decision, with 2–3 options and a recommendation>
```

Rules for the block: every value is quoted from output, never summarised; `vocab` and `modules` run on **every** commit, not only where a done-when names them (a scan broken by one commit is reported by that commit); a `NOT MET` or a non-empty **Needs AJ** ends the session there.

**The failure mode this phase must not have.** Phase 5.1's newest construct was the runtime, and it came out mostly honest. The hollow parts were in the **harness** and in the **scans**. This phase's newest construct is the **mutation catalogue**. A catalogue can easily be tested against itself: "the mutant differs from the original." That is true of any edit, so it proves nothing. Every catalogue test in this plan is therefore stated as **what a check or control does on the mutant**: the canonical hash moved, a named refusal appeared, a body stopped firing. None is stated as "the mutant is different."

---

## 1. Scope Fence

### In scope

- **The witness run** of the tree as Phase 5.1 left it, typed by AJ, and `cargo xtask witness`.
- **An honest test suite**: the three red tests F52 predicts, fixed without weakening them.
- **Parsed subjects, the mutation catalogue, and neutral edits.** Controls see values, and damage keeps the form.
- **Harness fixtures that run before any table.** Gate 5.1 items 8 and 9 stop being gate items, because they are properties of the harness.
- **Distinct opposition.** No two gate items anywhere share (artifact, mutation). The wrappers and built strings are deleted.
- **Binding as a type**: `BodyStore`, then `Bound`.
- **A body refusal is a report**, and the universe keeps running.
- **Declared grants**, so no host names a link or an alias.
- **The far side at both hosts**, and G6 read from what the hosts actually present.
- **AJ's unfinished findings**: `inner_reason.txt`, `boundary-depth.md`, the R50 update, the adversary, and the Law 4 correction.

### Out of scope — Cursor must refuse these even when they look small

| Not now | Where | Why it is tempting |
|---|---|---|
| Any change to how links are typed, delivered, or ordered | 5.1, held | The runtime's `run.rs` is open for §2.6 anyway, and fan-out and competing tails are next to it. R51 and R55 |
| Embeddings across a link | R56 | Unchanged |
| A new file kind (`.events`, `.script`, `.lens`) | later | The scenario in P52-13 would be easier to write as a file. It is written as a test instead |
| Random or generated mutants | later | The catalogue is **closed** and each mutation is named on purpose. Random mutation is a search, and a search chooses its own inputs (rule 20) |
| Mutating `.cell` artifacts | held | Cells already have counterfeits and mutants (rules 18, 20). The catalogue is for bodies, universes, locks, transcripts and text |
| A third linked body in `universe.universe` | later | The adversary's body lives in its own universe file (P52-14) |
| Any 2-block, filling or homology | Phase 4 | Unchanged |
| A new primitive, a line in `grandfather.txt`, a reblessed golden | never | Unchanged |

---

## 2. Decisions Assumed by This Plan

Twelve decisions. §2.11 and §2.12 were added after P52-00. §2.1–§2.4 are the phase. §2.5–§2.8 fix the binding, the runtime's refusals and the hosts. §2.9–§2.10 are about AJ's half.

### 2.1 A control sees a parsed value, never bytes · **PROPOSED · yours · this is the phase, first half**

> **The harness parses each control artifact by its file kind: `.body`, `.universe`, `.lock`, a transcript (`corpus/transcripts/*.txt`, as a list of lines), or text (any other `.txt`, as one trimmed string). It hands the control the parsed value (`Subject::Body(Body)`, `Subject::Universe(Universe)`, and so on). If the real artifact does not parse, the whole run is refused, naming the item and the parser's refusal. `artifact_loads` is deleted. `joinn-gate` names no DNA, link or lock type: `GateItem` becomes generic over the subject (`GateItem<S>`), or it moves to xtask. Cursor chooses which, and either keeps rule 34.**

A control can't return early on a parse failure because it never sees one. That removes the whole mechanism F45 describes, not just one instance of it.

**Cost to reverse:** controls go back to receiving bytes, and `artifact_loads` returns under a new name.

### 2.2 Damage keeps the form · **PROPOSED · yours · this is the phase, second half**

> **Each gate item declares `opposes: Mutation`, taken from a closed catalogue per file kind. The harness applies that mutation to the parsed subject, reprints it with the kind's canonical printer, and re-parses it. The mutant must parse, and for bodies and universes its coding hash must differ from the original's. The control must answer `false` on the real subject and `true` on the mutant. For kinds that have a regulatory section, the harness also applies a *neutral edit* (one regulatory label changed), and the control must still answer `false`. Any violation refuses the run, naming the item and which of the three conditions failed. A declared mutation whose target does not exist in the subject (a link id that isn't there) refuses the run, naming the target.**

**The catalogue** (closed; adding to it is a plan change):

| Kind | Mutations |
|---|---|
| `.universe` | `DropLink(id)` · `FlipMark(link, member)` · `ShiftPort(link, member, to)` · `SwapBinding(alias, to_hash)` · `CorruptHash(alias)` (last hex digit +1 mod 16) · `CopyMember(lens, alias, into_system)` · `DropLens(name)` · `WireAcross(link)` (rewrite a link as a cross-wire) · `DropGrant(link)` · `RenameLink(from, to)` · `RenameAlias(from, to)` |
| `.body` | `DropWire(src, dst)` · `DropGenome(instance)` · `SwapCell(instance, to_hash)` |
| `.lock` | `SetScore(phase, n, total)` · neutral edit: add a comment line |
| transcript | `DropLine(i)` · `SwapLines(i, j)` · no neutral edit (declared `None`) |
| text | `Replace(s)` · no neutral edit (declared `None`) |

**What each current item opposes** (AJ's table; §2.4's uniqueness check will reject any clash):

| Gate · item | Subject | Opposes | The control answers `true` (it sees the mutation's effect) when |
|---|---|---|---|
| 5 · 1 Something crosses | `phase5/universe.universe` | `DropLink(e0)` | `units` does not fire |
| 5 · 2 The membrane is measured | `phase2/calculator.body` | `DropWire(cli_a@1, sum@0)` | ∂ ≠ `{cli_a@0, cli_b@0, sum@2}` |
| 5 · 3 The universe is well-formed | `phase5/universe.universe` | `ShiftPort(e0, calc.sum@2, 9)` | assembly refuses naming *no such port* |
| 5 · 4 Exclusivity holds | `phase5/universe.universe` | `CopyMember(function, units, calculation)` | exclusivity refuses naming `units` |
| 5 · 5 Two lenses, one body | `phase5/universe.universe` | `DropLens(deployment)` | fewer than two lenses place `units` |
| 5 · 6 Law 4 is a check | `phase5/universe.universe` | `WireAcross(e0)` | Law 4 refuses naming the container |
| 5 · 7 A capability can be revoked | `phase5/universe.universe` | `DropGrant(e0)` (from P52-11; until then, `ordered.universe` with `DropLink(path)`, the control granting each ordered link to its head) | `units` does not fire before any revoke |
| 5 · 8 A refusal stays home | `phase5/controls/inner_reason.txt` | `Replace("e0")` | the hosts' far-side output contains the text |
| 5.1 · Two hosts, one universe | `transcripts/universe.txt` | `SwapLines(2, 3)` | the transcript does not start with `calculator.txt` |
| 5.1 · Tails are out, heads are in | `phase5/universe.universe` | `FlipMark(e0, calc.sum@2)` | typing refuses naming `calc.sum@2` |
| 5.1 · Members share a frame | `phase5/universe.universe` | `SwapBinding(units, <echo hash>)` | typing refuses naming `Text 1` and `ℤ 1` |
| 5.1 · The boundary is total | `phase52/controls/missing_cell.body` | `DropGenome(orphan)` | ∂ is computed with no refusal |
| 5.2 · Binding is by store | `phase5/universe.universe` | `CorruptHash(calc)` | binding refuses naming `calc` |
| 5.2 · A refusal is a report | `transcripts/universe.txt` | `DropLine(1)` (lines count from 0; line 1 is the refusal) | the CLI, run on the transcript's inputs, prints a line the subject does not have |
| 5.2 · The host knows no ids | `phase5/ordered.universe` | `DropGrant(path)` | `units` does not fire under the CLI |

The table is built so that no two rows share a pair. §2.4's check will enforce that from now on: `universe.universe` is the subject of nine rows, and each opposes a different mutation. Where the table turns out to be wrong (a mutation that can't reach the fact, or a control that can't be written to see it), AJ decides in P52-05's list.

*Why a closed catalogue instead of generated mutants:* a generated mutant is a search, and a search picks its own inputs (rule 20). A named mutation is a claim that a reviewer can read: *this control is about the link's direction.* The table above is that claim, made once for every item.

**Cost to reverse:** the damage rule goes back to catching parse failures (F45).

### 2.3 The harness tests itself before any table runs · **PROPOSED · yours**

> **`run_gate_table` runs four fixture items before the first table, the way `modules` runs its fixtures before the scan: (1) a control that ignores its subject must be refused; (2) a control that answers `true` on *any* change, including the neutral edit, must be refused; (3) a control whose declared mutation doesn't exist in its subject must be refused, naming the target; (4) an honest control must be admitted. If any fixture comes out the wrong way, `gate all` stops before any gate runs. The lock postcondition gets a fixture of its own: `write_lock` takes a path, a two-item fixture table's returned score is written to a file under `target/`, read back and compared, and an off-by-one writer is refused naming the phase.**

Gate 5.1 items 8 (*every control reads its artifact*) and 9 (*the lock is this run*) are deleted as gate items. They were properties of the harness wearing gate-item names, which is exactly what P51-03 said of Phase 5's item 9. `insensitive.rs`, `ignores_bytes.rs`, `off_by_one.lock` and `g51_lock*` are deleted with them (F46). **Gate 3 item 9 is deleted for the same reason** (F54: it certified the previous run's lock).

### 2.4 A gate row opposes its own mutation · **PROPOSED · yours**

> **Across every gate table, no two items share the pair `(control_artifact, opposes)`. The harness checks this once, before the fixtures, and refuses naming both items. The function-pointer test (V86) stays. Every wrapper whose body is one call to another gate function is deleted, along with the row it served: gate 5.1's items 1, 6 and 7 (they duplicate gate 5's 1, 7 and 8), and gate 2's rows that duplicate 2.1 and 2.2 rows. A duplicated row stays in the gate whose phase introduced the fact. Cursor lists each pair in the commit message and AJ approves the list before the commit lands.**

This is distinctness by **what an item opposes**, which is data, not by the text or the pointer of its function. You can't give two items different opposition pairs without making them about different facts.

**Scores move** (gate 2 and gate 5.1 lose rows). Each gate still returns its own score, and the lock records whatever was returned (rule 29).

### 2.5 Binding is a type · **PROPOSED · yours · closes F48**

> **`BodyStore` holds bodies keyed by their computed coding hash. Its only way to add a body is `insert(body, cells)`, which computes the key. `bind(&Universe, &BodyStore) -> Verdict<Bound>` is the only constructor of `Bound`. `assemble_universe`, `check_link_types` and `UniverseState::new` take `&Bound`. A trybuild fixture that builds a `Bound` from a `BTreeMap` fails to compile. `bind_bodies`'s declared-versus-supplied branch is deleted, because it can no longer be reached. Law 5's reachable refusal is *"alias `calc` declared `abcd1234` and the store holds no such body."***

A refusal that can only be reached by forging the store should be made impossible to express, not tested with a forgery. `g51_hash` and `crossing.rs::wrong_hash_names_both_shorts` lose their forgeries. `wrong_hash.universe` is renamed `alias_is_local.universe`, and it becomes a **positive** control: it binds, because an alias is only a local name.

**The plan error, recorded.** P51-08's done-when asked for a refusal that an honest store can't produce. That was the plan's mistake, and the builder's forgery was the only way to meet it.

### 2.6 A body refusal is a report, and the universe keeps running · **PROPOSED · yours · closes F51**

> **When a body refuses during `run`, the runtime records `UniverseReport::Refused { body, instance }`. It carries no text, and the reason stays in that body's state. The runtime then carries on with the other bodies. `LinkRefusal { kind: Refused }` is added only when the refused activation consumed a value that a link delivered *in the same pass*. The record of deliveries is cleared at the start of every pass. `run` returns `Verdict::Refused` only for its own failures (budget exhausted, unknown alias), and never with a body's reason text.**

This is what lets `"two"` enter the universe: `calc` refuses, `UniverseReport::Refused { body: "calc", instance: "cli_a" }` is recorded, and the calculator is prompted again. The host is the near side of `calc`, so it may call `describe_refusal` on `calc` and print the reason. That is the same thing the standalone calculator does.

### 2.7 Grants are declared by the universe · **PROPOSED · yours · closes F50's hardcoded grant**

> **The universe's coding region gains `grants { e0: units }`, one line per capability: the link id, then the body it authorizes. It is parsed, printed canonically, and hashed. `UniverseState::new` starts with exactly the declared grants. `revoke` still works at runtime. Hosts never call `grant`. A grant on an unordered link, or to a body that is not a member of the link, is refused at parse time, naming both.**

*Why in the coding region:* a body already declares `grants { stdin: scale }`, which is its capability over its own container's channel. A universe declaring `grants { e0: units }` is the same idea one level up. That is the fractal reading, where each container states its own communication method (Law 4), and it keeps the capability in the blueprint instead of in the host. *The alternative* is a host flag (`--grant e0:units`). It moves no hashes, but it puts authority in the host, and F50 is about removing authority from the host.

**Every universe hash that gains a `grants` section moves**, and P52-16 records each one. No Phase 0–3 hash moves.

### 2.8 The far side is a host line, built from the type · **PROPOSED · yours · closes F50**

> **The CLI prints every `UniverseReport::Link` through a presenter whose only parameter is `&LinkRefusal`. By its signature, it can't reach a body's state. The test host's `Capture` gains `far_side: Vec<LinkRefusal>` (values, not strings) and an `intent_set` derived per body from the in-ports on ∂ that no link head feeds. The CLI injects every line into the universe, and the side `BodyState` used to check the first line is deleted. The normal transcript has no link refusals, so `universe.txt` does not change.**

*What the interactive CLI cannot do, recorded rather than faked:* in canonical order, the CLI asks for `a`, `b` and `factor` in that order, so it can never deliver two sums before a factor, and the double-delivery refusal can't happen under it. That is recorded as R59. For G6, the CLI's part is its presenter, run on the reports from the test-host scenario. The presenter's signature proves it can't read a body. The test proves what it prints.

**The host knows no ids.** Gate 5.2 item 3's check runs the CLI on the catalogue mutant `RenameLink(e0, e1)` **and** on `RenameAlias(units, meters)` (with the transcript's names mapped), and requires the same output as on the original. A host that hardcodes `e0` fails this check. A host that doesn't has nothing to change.

### 2.9 AJ's acceptance is a checkpoint that later commits depend on · **DECIDED · amended 24 Sep** · closes F44, F53

> **`cargo xtask witness <path>` prints `current` when the finding's last commit is later than the last commit touching `crates/`, `xtask/` or `corpus/` (mtimes when there is no repository, §2.12). Otherwise it prints `stale: <newest path> <when>` and exits non-zero. It writes nothing (rule 17). The phase has four sittings, and each ends with a checkpoint (`docs/Findings/phase-5.2-checkpoint-{a,b,c}.md`, then `phase-5.2-run.md`) in two parts:**
>
> **1. Evidence, written by Cursor.** The session report blocks of the sitting's commits, then one run of `cargo test --workspace --no-fail-fast`, `gate all`, `vocab` and `modules` with each command's last line and every `fail` row quoted, each marked *expected* (with the plan section that predicts it) or *unexpected*.
>
> **2. Acceptance, written by AJ.** His answer to every *unexpected* item and every **Needs AJ** line, and one line: `Accepted: AJ, <date>`. Nothing else.
>
> **The first commit of the next sitting has `witness <checkpoint>` → `current` in its done-when. Cursor may write the evidence and may not write the acceptance (rule 38).**

*Why the change:* the Draft 0.1 version had AJ retype every command's output. That cost AJ most of a sitting at Checkpoint A and added nothing a builder can't read. What only AJ can supply is judgment: accepting a changed expectation (rule 7), choosing between options, and saying the sitting is closed. The checkpoint now asks him for exactly that.

*What still stops a builder from closing a sitting alone:* the acceptance line is AJ's (rule 38), and a checkpoint without it is not accepted. P52-02a makes `witness` check for it, so this is not only a sentence in the plan.

### 2.10 The inner reason is what a probe printed · **DECIDED** (P51-16, unfinished)

> **`cargo xtask probe-refusal` runs the double-delivery scenario and prints two things: the probe label on `units`, and each far-side value's presenter line. AJ copies the probe label into `inner_reason.txt` by hand. The check requires that the probe label contains it and that no far-side line does. The control's mutation `Replace("e0")` must make the far-side check fire. That proves the check really searches the far-side output.**

This ends the contradiction in F49. The artifact is now the actual sentence the body produced, not a sentence nothing produces.

### 2.11 Rule 13 and the standard library · **OPEN · AJ decides before P52-01** (from P52-00)

> **`vocab` refuses `saturating_sub` because rule 13 bans `sub` as an identifier part, and here the ban hits Rust's standard arithmetic API rather than the *subtract* concept the rule exists to keep out of the floor.** There are three honest options: **(a)** limit rule 13's identifier check to DNA-facing text (coding regions, cell and body grammars, primitive and native names) and exempt method calls on integer types; **(b)** keep the rule as it is and silence the budget arithmetic with `allow(vocab): budget arithmetic on u64, not the subtract concept`, using the existing reasoned silencer; **(c)** keep the rule and restructure the budget so no difference is ever computed (store `remaining` and draw it down with a checked decrement). Recommendation: **(b)** for now, with R63 written, because (a) changes a Phase 2 rule in the middle of a correction phase. Option (c) is allowed only if it is the better design on its own merits. Rewriting to a synonym (`abs_diff`) just to get past the scan is rule 42's case and is refused.

### 2.12 Version control · **OPEN · AJ decides before P52-01** (from P52-00)

> **There is no `.git` under `D:\JoInn`.** Without one, a "commit" in these plans is a session, and `witness` can only compare mtimes. Mtimes are easy to disturb: a tool that rewrites a file moves them, and a fresh checkout resets them. **Recommendation:** AJ runs `git init` in `D:\JoInn` before P52-01 and makes one commit of the tree exactly as P52-00 witnessed it. From then on, each P52 commit is a git commit whose message is the plan's commit id plus the done-when output. `witness` compares the commit time of the checkpoint against the latest commit touching `crates/`, `xtask/` or `corpus/`, and falls back to mtimes only when there is no repository.

---

## 3. Architecture

### 3.1 Crates

No crates are added, and no dependency is added. `joinn-gate` loses its knowledge of bytes, or becomes generic, per §2.1, and still names no DNA or link type. `joinn-link` gains `BodyStore` and `Bound`. The universe parser and printer gain a `grants` section. `xtask` gains `mutate/` (the catalogue), `witness`, `probe-refusal`, and the fixtures.

### 3.2 Types added or changed

```rust
// xtask (or joinn-gate generic over S)
pub enum Subject { Body(Body), Universe(Universe), Lock(Vec<LockRow>), Transcript(Vec<String>), Text(String) }
pub enum Mutation { DropLink(&'static str), FlipMark(..), ShiftPort(..), SwapBinding(..), CorruptHash(..),
                    CopyMember(..), DropLens(..), WireAcross(..), DropGrant(..), RenameLink(..), RenameAlias(..),
                    DropWire(..), DropGenome(..), SwapCell(..), SetScore(..), DropLine(usize), SwapLines(usize, usize),
                    Replace(&'static str) }
pub struct GateItem { name, check: fn() -> bool, control: fn(&Subject) -> bool,
                      control_artifact: &'static str, opposes: Mutation }
fn mutate(s: &Subject, m: &Mutation) -> Verdict<Subject>;     // reprint + reparse inside
fn neutral(s: &Subject) -> Option<Subject>;                     // None for transcript and text

// joinn-link
pub struct BodyStore { /* private BTreeMap<Hash, (Body, Cells)> */ }
impl BodyStore { pub fn insert(&mut self, body: Body, cells: Cells) -> Hash; }
pub struct Bound { /* private */ }
pub fn bind(u: &Universe, store: &BodyStore) -> Verdict<Bound>;
pub enum UniverseReport { Fired { body, instance }, Refused { body, instance }, Link(LinkRefusal) }
// UniverseCoding gains: pub grants: Vec<(String /*link*/, String /*alias*/)>
```

`bind_bodies`, `artifact_loads`, `damage_bytes`, `ignores_bytes`, the six `g2_*` wrappers, `g51_cross*`, `g51_revoke*`, `g51_home*`, `g51_reads*` and `g51_lock*` are **deleted**.

### 3.3 The new `missing_cell.body` · **PROPOSED · yours**

The Phase 5.1 file is byte-identical to `units.body` (F46). The new one lives at `corpus/phase52/controls/missing_cell.body`. It is `units.body` with a second genome entry, `cell:<false_law.cell hash> as orphan`, and no wires to it.

Why that cell: `corpus/phase22/counterfeit/false_law.cell` is refused **on purpose** and is never in any store (rule 22). So this is a real reason for a cell to be missing, *the store only holds admitted cells*, and not an empty map passed by the test. The control computes ∂ against the **full** store and is refused naming `orphan`. Its mutation `DropGenome(orphan)` gives back `units.body`, whose ∂ is `{scale@0 In ℤ, scale@1 In ℤ, scale@2 Out ℤ}`.

### 3.4 Hand-written controls that become mutants

After P52-04, Cursor runs each catalogue mutation of `universe.universe` that the old control files imitated and compares canonical text. **A control file is deleted only if some catalogue mutant reproduces it byte for byte.** Otherwise it stays, and the commit says which mutation came closest and how it differs. Expected candidates: `unlinked.universe`, `no_such_port.universe`, `wrong_direction.universe`, `frame_mismatch.universe`, `two_systems.universe`, `wrong_container.universe`. Also expected: `transits.universe` stays, because a member moved onto an interior port is not in the catalogue, and `crossing.rs` keeps it as V97's second control.

---

## 4. The Commit Plan

Seventeen commits in four sittings. **Done-when** is a command, and the command must be able to refuse.

### Sitting A — the witness and an honest suite

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P52-00** | **The tree as Phase 5.1 left it — typed by AJ** | AJ runs `cargo test --workspace --no-fail-fast` (without the flag, cargo stops at the first failing test binary and most of the suite never runs), `gate all` (which writes the lock; that is expected), `corpus verify`, `vocab`, `modules`, `perf`. **The witness session changes no file outside `target/`.** Any file it does change is listed in the run file and types `docs/Findings/phase-5.1-run.md`, quoting each command's last line and every `fail` row. If the 20:31 UTC scrollback from 23 September still exists, AJ also types `phase-5-run.md` from it; if not, AJ writes one line saying it can't be recovered. **Cursor does not write either file** | The file exists and quotes what was printed. F52's three predicted failures and F49's predicted 7/8 and 8/9 are confirmed or refuted there, and the review's predictions are graded against it |
| **P52-01** | **The suite, honest** | `tests/capability.rs` grants `e0` (its link was removed on purpose by P51-12). `tests/adversary.rs` binds through `bind_bodies` with the mul cell supplied and **asserts the refusal** naming `units.scale@1` as an out-port marked head. `tests/fail/string_reason.stderr` is accepted after it is read, and it must name the missing field `reason` (it was already written during the P52-00 session; this commit confirms its content and records when it appeared). `xtask` usage and help strings list `5.1`. **The three `vocab` hits (`saturating_sub` in `universe_state/run.rs:27,36` and `g51_lock.rs:9`) are resolved the way AJ decides in §2.11, never by renaming around the scan (rule 42)** | `cargo test --workspace --no-fail-fast` passes, and `cargo xtask vocab` reports no hits. The commit message lists every assertion that changed, with old and new text. None was removed |
| **P52-02** | **`cargo xtask witness`** (§2.9) | The command; a unit test that ages a temp file below a temp tree | `witness docs/Findings/phase-3-run.md` prints `stale:` naming a file newer than it, and exits non-zero |
| — | **Checkpoint A** | `phase-5.2-checkpoint-a.md`. Closed 24 Sep under the Draft 0.1 rule (AJ transcribed). Last Draft 0.1 checkpoint | — |
| **P52-02a** | **`witness` requires an acceptance** (§2.9 amended) | `witness` refuses a checkpoint with no line matching `^Accepted: AJ, ` and prints `unaccepted: <path>` | A copy of checkpoint A with the line removed, committed on a scratch branch, prints `unaccepted:` and exits non-zero; the real checkpoint A prints `current` |

### Sitting B — controls that can be wrong about the thing

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P52-03** | **Parsed subjects (§2.1)** | `Subject`; the harness parses by kind; every control ported mechanically to take `&Subject`; `artifact_loads` deleted | `witness phase-5.2-checkpoint-a.md` prints `current` (after P52-02a). A fixture artifact that does not parse refuses the run naming the item and the parser's reason. No file under `xtask/src` defines or calls `artifact_loads` |
| **P52-04** | **The catalogue (§2.2)** | `mutate` and `neutral` for every kind and every row of the catalogue table | One test per mutation, applied to a real corpus file: the mutant re-parses, its canonical hash differs (bodies, universes), and a named target that doesn't exist is refused naming it. **Each test also asserts one downstream effect**, for example `DropLink(e0)` → `units` does not fire, and `FlipMark` → `check_link_types` refuses. None asserts only that the mutant differs |
| **P52-05** | **Opposition is declared and enforced (§2.2)** | `opposes` on every item, from §2.2's table; truncation damage deleted | `gate all` is **refused**, listing every item that fails the mutant or neutral condition. The prediction is at least 5·2, 5·8, 5.1's hash item and 5.1's total item. The list goes in the commit message. **Nothing is fixed in this commit.** AJ settles any table row the list shows to be wrong |
| **P52-06** | **The harness tests itself (§2.3)** | four fixtures, lock round-trip, `write_lock(path)`; gate 5.1 items 8 and 9 and gate 3 item 9 retired; files in §3.2 deleted | Breaking fixture 1 (making the harness admit a control that ignores its subject) makes `gate all` stop **before any gate prints a row**, naming the fixture, shown then reverted. The harness validates every table's controls (mutant, neutral, uniqueness) before running any check. Gate 3 prints eight rows |
| **P52-07** | **Distinct opposition (§2.4)** | the uniqueness check; wrappers and duplicate rows deleted after AJ approves the list; phase labels from one `const` in `gate_all.rs`; `format!("phase {}", …)` gone | Giving 5.1's typed item the same `(artifact, opposes)` as 5·3 refuses naming both, shown then reverted. No gate function's body is a single call to another gate function (listed by hand in the commit, not scanned) |
| **P52-08** | **Every control made honest** | each control from P52-05's list rewritten to decide from its subject; `missing_cell.body` per §3.3; §3.4's deletions | `gate all` passes the harness. **Items may still fail as checks**, and 5·8 is expected to until P52-13; each failing check is listed. `cmp` of each deleted control file against its catalogue mutant is quoted in the commit |
| — | **Checkpoint B** | `phase-5.2-checkpoint-b.md`: evidence by Cursor, acceptance by AJ (§2.9) | — |

### Sitting C — binding, runtime, hosts

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P52-09** | **Binding is a type (§2.5)** | `BodyStore`, `Bound`, `bind`; the three consumers take `&Bound`; trybuild fixture; `alias_is_local.universe`; forgeries deleted from `g51_hash` and `crossing.rs` | `witness phase-5.2-checkpoint-b.md` prints `current`. The trybuild fixture fails to compile. `CorruptHash(calc)` on `universe.universe` is refused naming `calc` and the short hash. `alias_is_local.universe` binds. `bind_bodies` no longer exists |
| **P52-10** | **A body refusal is a report (§2.6)** | `UniverseReport::Refused`; the delivery record cleared per pass; `run` never returns a body's reason | Test: inject `"two"` at `calc.cli_a@0`, run, then `2`, `3`, `12`, run. The reports contain `Refused { calc, cli_a }` and then `units` fires, holding `60` read via `describe`. Control in the same test: after one full crossing, inject a second `12` into `units.scale@1` from the host. `units` refuses (`join refuse`), and the reports contain **no** `LinkRefusal`, which is the case F51's never-cleared `pending` got wrong |
| **P52-11** | **Declared grants (§2.7)** | parser, printer, hash; `grants { e0: units }` in `universe.universe` and `grants { path: units }` in `ordered.universe`; hosts stop calling `grant`; parse-time refusals | `DropGrant(e0)` → `units` does not fire. A grant on an unordered link is refused at parse time naming the link. No file under `crates/joinn-cli` or `crates/joinn-test-host` calls `grant`. Universe hashes move and are listed in the commit |
| **P52-12** | **Hosts carry the far side (§2.8)** | the CLI injects every line into the universe; side `BodyState` deleted; presenter `fn(&LinkRefusal) -> String`; `Capture.far_side` and a derived `intent_set` | `echo "two\n2\n3\n12" \| joinn run universe` prints `universe.txt` byte-identically, and `joinn run calculator` prints `calculator.txt`. On the mutants `RenameLink(e0, e1)` and `RenameAlias(units, meters)`, the CLI's output is unchanged apart from the renamed names. The test host's `intent_set` for `units` is `{scale@1}` |
| **P52-13** | **G6 at the hosts (§2.10)** | `cargo xtask probe-refusal`; **AJ rewrites `inner_reason.txt` from its output**, and this commit waits for that; 5·8 rewritten to read the test host's `far_side`, the CLI presenter's lines for those values, and the probe | The probe label contains `inner_reason.txt`. No far-side line does. The control's `Replace("e0")` makes the far-side check fire. `crossing.rs::second_sum_stays_home` and gate 5·8 both pass, and they now read the same sentence |
| — | **Checkpoint C** | `phase-5.2-checkpoint-c.md`: evidence by Cursor, acceptance by AJ (§2.9) | — |

### Sitting D — AJ's findings and the freeze

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P52-14** | **The adversary, specified by AJ** | Before this commit starts, AJ writes a one-paragraph spec in `docs/Findings/law-4-adversary-spec.md`. **PROPOSED:** *a `lookup` body that asks `units` "what factor did you last use?" and must pair each answer with the question that caused it.* Cursor attempts it with the existing wire, hyperedge and declared grants, in `corpus/phase52/adversary/`, and reports. AJ writes `law-4-adversary.md` | `witness phase-5.2-checkpoint-c.md` prints `current`. The finding uses **fired** only if the body cannot be expressed. The Phase 5 entry is corrected to *"held — the attempt did not test Law 4"* (F41). If correlation needs an order across two links, R58 is updated with the example |
| **P52-15** | **The rest of AJ's findings** | AJ types `boundary-depth.md` (from 5.1 §2.6) and the `r50-membrane-cost.md` update (from `perf`'s split numbers), and writes R55 and R56 into `docs/Theory/JoInn Research Backlog.md` with their motivating examples | `witness` prints `current` for all three files |
| **P52-16** | **Re-freeze, docs, gate 5.2, lock** | new goldens; `phase-5.2-hashes.md`; gates 5 and 5.1 per §2.2's table; **gate 5.2** (§6); README, `Guides/03`, `decisions.md` rows for V98–V110 and R59–R62 | `cargo xtask gate all` from a clean checkout, offline, runs the fixtures first and then prints phases 0, 1, 2, 2.1, 2.2, 3, 5, 5.1 and 5.2, with every item passing the mutant and neutral conditions. `corpus verify` matches. `phase-5.2-hashes.md` confirms **no Phase 0–3 coding hash moved**. The lock postcondition holds |
| — | **The run** | `phase-5.2-run.md`: evidence by Cursor, acceptance by AJ (§2.9) | `witness phase-5.2-run.md` prints `current` |

**Ordering notes.**

- **P52-00 comes first, and AJ does it.** It also grades the Phase 5.1 review's predictions. If a prediction was wrong, the review is corrected before P52-01.
- **P52-03 through P52-06 can't be cut.** Every later control is judged by them.
- P52-05 deliberately leaves `gate all` red until P52-08. **Do not weaken the catalogue or the neutral condition to get it green.** If a control can't be made to flip on a mutant that keeps the form, that is a finding about the control.
- P52-09 must land before P52-11 (grants are on the bound universe). P52-10 must land before P52-12 (the CLI needs refusal reports).
- P52-13 waits for AJ's `inner_reason.txt`. If AJ hasn't typed it, the commit stops.
- **If something has to be cut, cut P52-14 and carry the adversary forward again with a line in `decisions.md`. Never cut P52-00, P52-03 through P52-06, or P52-10.**

---

## 5. Test Strategy

### 5.1 New invariants

Continuing from V97.

| # | Invariant | Test | Commit |
|---|---|---|---|
| **V98** | A control never sees bytes; an artifact that doesn't parse refuses the run | fixture | P52-03 |
| **V99** | Every catalogue mutant parses, moves the hash, and has a named downstream effect | one test per mutation | P52-04 |
| **V100** | Each control flips on its declared mutant | harness | P52-05 |
| **V101** | Each control ignores its neutral edit | harness | P52-05 |
| **V102** | The harness's fixtures come out right before any gate runs | four fixtures | P52-06 |
| **V103** | No two items anywhere share `(artifact, opposes)` | uniqueness check | P52-07 |
| **V104** | A `Bound` can only come from a store | trybuild | P52-09 |
| **V105** | A hash the store doesn't hold is refused naming the alias | `CorruptHash(calc)` | P52-09 |
| **V106** | A body refusal is a report, and the universe keeps running | `"two"` then `2, 3, 12` → 60 | P52-10 |
| **V107** | `LinkRefusal::Refused` only for a delivery in the same pass | host-caused refusal → no link refusal | P52-10 |
| **V108** | Grants are declared; no host calls `grant` | `DropGrant`, parse refusals | P52-11 |
| **V109** | A host's output doesn't depend on link ids or aliases | `RenameLink`, `RenameAlias` | P52-12 |
| **V110** | The far side is a kind at both hosts; the probe has the words | scenario, `Replace("e0")` | P52-13 |

Carried forward and re-run on every commit: V18–V20, V24-embryo, FO1–FO10, the canonical-text properties, the 1 000-append hash-stability test, V33–V97, and Phase 1's four demos. V86 (pointer distinctness) stays, next to V103.

### 5.2 What a catalogue test must never do

It must never assert only that a mutant *differs* from its original, or that `mutate` returned `Ok`. It asserts what the mutant **does**: a refusal that names the mutated thing, a body that stops firing, a ∂ that grows. A catalogue that is tested only against itself is the harness checking its own notes. That is §5.2 of the Phase 5.1 plan, applied to the new construct.

### 5.3 What Phase 5.2 deliberately does not test

- **Double delivery under the interactive CLI.** Canonical prompt order makes it impossible (R59).
- **Mutants of cells.** Rules 18 and 20 already cover them.
- **Whether the catalogue is complete.** It is closed, not complete. A control that no catalogue mutation can reach keeps a hand-written artifact, and that is the file a reviewer reads first.

---

## 6. Exit Gate 5.2, As a Checklist

`cargo xtask gate 5.2`, run from a clean checkout, offline, after the harness fixtures have passed. Every item opposes a catalogue mutation that keeps the form, and ignores its neutral edit.

- [ ] **1 · Binding is by store.** `universe.universe` binds, and `alias_is_local.universe` binds. *Opposes:* `CorruptHash(calc)`, refused naming `calc`.
- [ ] **2 · A refusal is a report.** The CLI on `two, 2, 3, 12` prints `universe.txt`, with `"two"` refused *inside* the universe. *Opposes:* `DropLine(1)` on `universe.txt`.
- [ ] **3 · The host knows no ids.** The CLI's output is unchanged under `RenameLink(e0, e1)` and `RenameAlias(units, meters)`, apart from the names. *Opposes:* `DropGrant(path)` on `ordered.universe`, where `units` no longer fires.

Gates 5 and 5.1 are rewritten in place per §2.2's table. Gate 5 keeps eight items. Gate 5.1 keeps four: *two hosts*, *tails and heads*, *one frame*, *a total boundary*. The lock records each gate's returned score. This plan does not quote one.

### 6.1 The conditions for opening Phase 4

Phase 4 opens when all of the following hold. Its plan's P4-00 cites this list and runs `cargo xtask witness` on item 1:

1. Gate 5.2 passes, and **`witness phase-5.2-run.md` prints `current`.**
2. A value has crossed a typed link under both hosts, with the calculator's refusal happening inside the universe, and a transcript golden.
3. `boundary-depth.md`, the corrected `law-4-adversary.md` and the R50 update exist, each `current`.
4. The adversary was specified by AJ, attempted, and graded, with `fired` used as defined.
5. R55 and R56 are in the research backlog with their examples. **R60 is written up too:** Phase 4 adds a new construct (fillings), and whether it must come with its own mutation catalogue from the start should be decided before Phase 4's first commit.

---

## 7. Risks Watched During This Phase

| Risk | Instrument | What to do when it fires |
|---|---|---|
| **The catalogue is tested against itself** | §5.2; each mutation test names a downstream effect | Rewrite the test to observe a check, a refusal or a body. If no effect can be named, the mutation is dropped from the catalogue |
| **Controls get written to the mutation rather than the fact** (a control that checks "is link `e0` present") | review; the §2.2 table makes each control's claim readable | Choose a second mutation for that item from the same kind and require it to flip too. Only if this shows up. Don't build it in advance |
| **A scan is evaded again** | rule 42; types instead of scans (§2.4, §2.5) | Stop. Report the scan and the change it blocked. Don't write around it |
| **A builder writes the acceptance line** | rule 38; `witness` checks for the line (P52-02a) and compares commit times | Delete the line and leave the sitting open |
| **A session report summarises instead of quoting** | §0.1 | Re-run the command and quote the line. A report that says "passes" without the printed line is not a report |
| **`grants` moves a hash that shouldn't move** | `corpus verify`; `phase-5.2-hashes.md` | Only universe hashes may move. If a body hash moves, the grants section leaked into body coding. Stop |
| **The interactive CLI still produces the calculator refusal with a side channel** | P52-12 deletes the side `BodyState`; V106 | If the transcript can't be produced with `"two"` inside the universe, record it. Don't re-add the side channel |
| **Pace** | sittings; `witness` | If two sittings land in one day without a checkpoint between them, the review starts from that fact |
| **Carried · the two hosts share a bug** | Unchanged: both call `describe` | The universe transcript is a golden, and the test host's capture is checked by value against it |

---

## 8. What This Plan Refuses To Do

| Refused | Because |
|---|---|
| Give a control bytes | §2.1 |
| Accept damage that breaks the form | §2.2 |
| Admit a control that flips on a neutral edit | §2.2 |
| Let two gate rows oppose the same thing | §2.4 |
| Test an unreachable refusal with a forged store | §2.5 |
| Let a body's refusal stop the universe, or leave through `run` as text | §2.6 |
| Let a host name a link or an alias | §2.7, §2.8 |
| Fake a refusal in a host, or fake a refusal scenario the host can't produce | §2.8, R59 |
| Write code to avoid a scan | rule 42 |
| Let Cursor write a checkpoint's acceptance, `inner_reason.txt`, or a finding marked *typed by AJ* | rule 38, §2.9, §2.10 |
| Make AJ retype output a builder can read | §0.1, §2.9 |
| Weaken the catalogue or the neutral rule to get `gate all` green after P52-05 | §4's ordering note |
| Rebless a golden, add a primitive, add a line to `grandfather.txt` | unchanged |

---

## 9. Open Items This Plan Creates

Continuing from R58. R63 was added after P52-00.

| ID | Topic | Question |
|---|---|---|
| **R59** | Interactive hosts and races | In canonical prompt order, the CLI can never deliver two values into one port before its partner arrives. Is that a *property* (an interactive host can't create R57's half-fed races, so it never has to describe them) or a *limitation* (the host hides states the universe really has)? It bears on R46 and on Phase 6's drawing of pending ports |
| **R60** | Every file kind ships its mutants | A cell isn't admitted without a counterfeit (rule 20). Should a new file kind (`.lens`, a Phase 4 filling, a `.galaxy`) be required to come with its mutation catalogue before any control may point at a file of that kind? If so, the catalogue belongs in the grammar documents, not in xtask |
| **R61** | Whose budget is it | The runtime shares one budget by resetting each body's budget from outside (F51). Is a body's budget its own (declared in `.body`) or borrowed from its container? The fractal reading says borrowed. The Phase 2 grammar says declared |
| **R62** | One host, both sides | A host that presents several bodies is the near side of each. When a body refuses a value that another body sent, that host is also the far side. Does the host show the reason (it is the near side) or only the kind (it is the far side)? §2.6 shows the reason for host-caused refusals and the kind for link-caused ones. Is that the right line? |
| **R63** | A vocabulary ban against a host language | Rule 13 keeps *subtract* out of JoInn's floor, but its identifier check also catches `u64::saturating_sub`. Should vocabulary rules apply to JoInn's own language (coding regions, grammars, primitive names) only, and leave the Rust that implements the engine alone? The answer decides how every later vocabulary rule is scoped |

**Touchpoints.** **R49** is unchanged (its frame half is answered, and its law half is R56). **R51** is unchanged. **R53**: declared grants (§2.7) make revocation a change to the runtime's state that starts from something declared in the universe, which is a firmer answer than 5.1's. **R57** meets R59. **R58** is updated by P52-14.

---

## Appendix A · Amendments to `AGENTS.md`

Rewrite rule 24 as below, and append rules 41–47. Leave everything else as Phase 5.1 left it. Update the header paragraph to name Phase 5.2 and its one idea: *damage must keep the form.*

```markdown
24. A CONTROL IS AN ARTIFACT, PARSED, AND OPPOSED BY A MUTATION THAT KEEPS ITS
    FORM. `GateItem` carries `control_artifact` and `opposes`. The harness
    parses the artifact by kind and passes the parsed subject; a control never
    sees bytes. The control must answer false on the subject, true on the
    declared catalogue mutant, and false on the kind's neutral edit, or the run
    is refused naming the item. The catalogue is closed. joinn-gate names no
    DNA, link or lock type.
41. A GATE ROW OPPOSES ITS OWN MUTATION. No two items in any gate share
    (control_artifact, opposes). A row that restates another gate's fact is
    deleted from the later gate, not wrapped.
42. NEVER WRITE CODE TO AVOID A SCAN. If a scan blocks a correct change, stop
    and report. A scan that can be met by rewriting text is replaced by a type
    when one exists.
43. BINDING IS A TYPE. A `Bound` comes only from `bind(&Universe, &BodyStore)`,
    and a `BodyStore` computes every key it holds. Nothing downstream of binding
    accepts an alias map.
44. A BODY REFUSAL IS A REPORT. The universe records `Refused { body, instance }`
    and keeps running. A link refusal is reported only for a delivery made in
    the same pass. `run` never returns a body's reason.
45. A HOST KNOWS NO LINK IDS OR ALIASES. Grants are declared in the universe's
    coding region; hosts never call `grant`. The far side is presented from a
    `LinkRefusal` alone.
46. A SITTING ENDS AT A CHECKPOINT ACCEPTED BY AJ. The agent writes the
    evidence (commands run, output quoted, each item marked expected or
    unexpected); AJ writes the decisions and the line `Accepted: AJ, <date>`.
    The agent never writes that line. A commit whose done-when names a
    checkpoint starts with `cargo xtask witness <checkpoint>` and stops if it
    does not print `current`.
47. EVERY SESSION ENDS WITH THE REPORT BLOCK (plan §0.1). Values are quoted
    from output. `vocab` and `modules` run on every commit. A commit message's
    id names the work the commit did, or the commit is refused.
```

## Appendix B · Directory changes

```
docs\Findings\
    phase-5.1-review.md               ← F44–F54
    phase-5.1-run.md                  ← new, P52-00, typed by AJ
    phase-5-run.md                    ← new if recoverable, P52-00, typed by AJ
    phase-5.2-checkpoint-a.md …-c.md  ← new, typed by AJ
    law-4-adversary-spec.md           ← new, P52-14, typed by AJ
    law-4-adversary.md                ← corrected and extended, P52-14, typed by AJ
    boundary-depth.md                 ← new, P52-15, typed by AJ
    r50-membrane-cost.md              ← updated, P52-15, typed by AJ
    phase-5.2-hashes.md               ← new, P52-16
    phase-5.2-run.md                  ← new, after P52-16, typed by AJ
docs\Theory\JoInn Research Backlog.md ← R55, R56 with examples, P52-15
joinn\corpus\
    phase5\universe.universe          ← gains grants { e0: units }
    phase5\ordered.universe           ← gains grants { path: units }
    phase5\controls\inner_reason.txt  ← rewritten by AJ from probe-refusal
    phase5\controls\wrong_hash.universe → alias_is_local.universe
    phase5\controls\*.universe        ← deleted where a catalogue mutant reproduces them (§3.4)
    phase51\controls\missing_cell.body  ← deleted (F46)
    phase51\controls\unlinked.universe  ← deleted if reproduced by DropLink(e0)
    phase52\controls\missing_cell.body  ← new (§3.3)
    phase52\adversary\                  ← new, P52-14
joinn\xtask\gate_fixtures\insensitive.rs, off_by_one.lock  ← deleted
joinn\xtask\src\fns\mutate\ (+ mutate.rs), witness.rs, probe_refusal.rs  ← new
joinn\xtask\src\fns\artifact_loads.rs, damage_bytes.rs, ignores_bytes.rs,
    g2_{agree,trace}{,_control}.rs, g2_{corpus,transcript}_control.rs,
    g51_{cross,revoke,home,reads,lock}{,_control}.rs, g3_lock{,_control}.rs  ← deleted
joinn\crates\joinn-link\src\bind_bodies.rs  ← replaced by body_store.rs, bind.rs
joinn\crates\joinn-link\tests\fail\string_reason.stderr, bound_from_map.rs(+.stderr)  ← new
```

---

*JoInn Phase 5.2 Implementation Plan (Draft 0.1). Closes F44–F54 of `docs/Findings/phase-5.1-review.md` and finishes Phase 5.1's AJ-owned commits. It is a correction phase: no crates are added, no primitive is touched, and the link model is left as Phase 5.1 built it. Afterwards, every control is opposed by a mutation that keeps its form, and the phase cannot finish without AJ having watched it run. Everything marked PROPOSED · yours is a recommendation; Cursor implements the decisions AJ settles on.*
