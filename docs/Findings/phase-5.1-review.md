# Phase 5.1 review — are we ready for Phase 4?

Reviewer: Claude · September 23, 2026 · against `docs/Plans/JoInn Phase 5.1 Implementation Plan.md`

**Method, same as the last two reviews and just as limited.** This review **ran
nothing.** The bridge in this session can list and copy files from `D:\JoInn`,
but it has no shell, so the tree was read one file at a time. That covered every
file `joinn-link` gained or changed in this phase (`universe_state/*`,
`membrane.rs`, `check_link_types.rs`, `bind_bodies.rs`, `assemble_universe.rs`,
`capability/*`, all of `tests/`). It also covered every `g5_*` and `g51_*` file,
the harness (`run_gate_table`, `damage_bytes`, `artifact_loads`, `gate_all`,
`scores_match`, `parse_lock_scores`), both hosts' `run_universe`, `probe`, all of
`corpus/phase5/` and `corpus/phase51/`, `hashes.txt`, `gates.lock`, `AGENTS.md`,
and the findings the phase touched. Each claim below can be checked in the
source. None of them is quoted from a command's output.

Three further limits:

1. **Where this review says something fails, that is a prediction from reading
   the code, and it says so.** Where it says something is true of a file (for
   example, two files are byte-identical), it was checked directly.
2. The timeline comes from mtimes. The plan was written at 20:14 UTC on
   22 September. The first Phase 5.1 edit is at 20:41 UTC on 23 September, and
   the last is at 21:35 UTC. `gates.lock` was last written at **20:31 UTC**,
   ten minutes before the first edit.
3. Numbering continues from the Phase 5 review. F33–F43 are Phase 5's, so
   **F44 is the first finding here.**

---

## Verdict

**The design landed; the phase did not. Phase 5.1 built the right code, and for
the first time a value crosses a typed hyperedge, in a test. But the tree has
never been run, the damage rule was satisfied by parsing rather than reading,
and none of the findings that belong to AJ exists. Do not open Phase 4. Open
Phase 5.2, and start it with the witness run that Phase 5.1 skipped.**

The good part is substantial, and it should not be lost in the list below:

- **The link is a contract.** `check_link_types` refuses a tail that is not an
  out-port, a head that is not an in-port, and any two members whose frames
  differ, including version. Each refusal names the link, the member, what it
  is, and what was required. `frame_mismatch.universe` and
  `wrong_direction.universe` are honest controls. R49's frame half has an
  answer in code. F35 is closed.
- **∂ is typed and total.** `BoundaryPort { address, direction, frame }`.
  Primitive ports come from `prim_ports`, and a missing cell is refused naming
  the instance and the short hash, never skipped. F39's first half is closed.
- **A value crosses.** `UniverseState` alternates the two phases in canonical
  alias and link order, with one shared budget. `crossing.rs` injects 2, 3 and
  12, then reads `scale@2 = 60` **through `describe`**, and its control (the
  same events with no link) shows `units` not firing. It observes what a body
  did, the way rule 37 asks. This is F33's first real answer.
- **Revoking has an effect.** `check_capability` sits on the delivery path in
  `run.rs`. After a revoke, `units` does not fire, its `scale@2` does not move,
  and the report carries `LinkRefusal { link: "e0", kind: CapabilityNotHeld }`.
- **`LinkRefusal` has no text field**, and `far_side_refusal` is gone. F38's
  mechanism is fixed at the type level.
- **The second body is a body.** `units.body` is the `mul` cell
  (`0 in ℤ, 1 in ℤ, 2 out ℤ`, `join refuse`), feet to inches. The old body was
  moved byte for byte to `controls/echo.body`, and its hash is unchanged in
  `hashes.txt`. F34 is closed.
- **The lock is compared with the run.** `gate_all` writes the lock, reads it
  back, parses it, and calls `scores_match`. F40's item 10 is gone.

What did not land is the part of the phase that was about **discipline rather
than code**: the witness, the damage rule, and AJ's findings. Those were the
phase's one idea and its first four commits, and the plan said they could not
be cut.

---

## What landed

| Commit | Landed |
|---|---|
| **P51-00** | **Not done.** No `phase-5-run.md`. A `gate all` ran at 20:31 UTC, probably this run, but nobody wrote down what it printed. See F44 |
| **P51-01** | `Artifact { path, bytes }`; `control: fn(&Artifact) -> bool`; `joinn-gate` carries bytes and opens nothing |
| **P51-02** | `damage_bytes` and the two-call check in `run_gate_table`, plus the `insensitive.rs` fixture. **Satisfied by parsing, not by reading.** See F45, F46 |
| **P51-03** | A function-pointer distinctness test over all tables. **Met with wrapper functions.** See F47 |
| **P51-04** | Lock read-back and `scores_match`. Real. The gate item that tests it is not; see F46 |
| **P51-05/06** | Total, typed ∂. Real |
| **P51-07** | `perf` splits load+parse from `membrane()` and counts refusals. `r50-membrane-cost.md` not updated (AJ's) |
| **P51-08** | `bind_bodies` by hash. **Binding is a function, not a type; `assemble_universe` still takes alias maps.** See F48 |
| **P51-09/10** | Direction and frame. Real, with good refusals |
| **P51-11** | New `units.body`, `echo.body`. Real. AJ's hand-computed hash is not recorded anywhere |
| **P51-12** | `universe.universe` and `ordered.universe` rewritten, `g0` gone. **Broke `tests/capability.rs`.** See F52 |
| **P51-13** | `UniverseState`. Real, with two defects. See F51 |
| **P51-14** | Both hosts run the universe, and `universe.txt` starts with `calculator.txt`. **The CLI fakes the refusal line.** See F50 |
| **P51-15** | Capability on delivery. Real |
| **P51-16** | `LinkRefusal`, `far_side_refusal` deleted, trybuild fixture. **`inner_reason.txt` not rewritten; the gate item contradicts the test.** See F49 |
| **P51-17** | `no_such_port.universe` separates *no such port* from *interior*. Real. `boundary-depth.md` missing (AJ's) |
| **P51-18** | **Not done.** No adversary spec, no attempt, and `law-4-adversary.md` still says **fired** |
| **P51-19** | `phase-5.1-hashes.md`, README and Guides updated, gate 5.1 table. **The lock was never written with a 5.1 row.** See F44 |

---

## Findings

### F44 · Nothing was run, and the lock shows it

`gate_all` calls `write_lock` on every run, whether it passes or fails. The only
thing that stops it before writing is `corpus_verify()?` failing. `gates.lock`
was last written at **20:31 UTC** and reads:

```
phase 3: 9/9
phase 5: 10/10
```

It has no `phase 5.1` row. `gate_all.rs` gained that row at 21:24 UTC. So
**`gate all` has not run to completion on any tree that contains Phase 5.1's
gate code.** `phase-5.1-hashes.md` says *"`corpus verify` matches the new
goldens,"* and `Guides/03-where-we-are.md` says `joinn run universe` prints
`5 ft = 60 in`. Neither claim has a witness.

The pace behind this: **nineteen commits between 20:41 and 21:35 UTC, about
fifty-four minutes.** The plan's §7 said *"if three commits land in one session,
the review for 5.1 starts from that fact."* This review starts from it.

The 20:31 run matters. It came ten minutes before the first 5.1 edit, which
makes it very probably the P51-00 witness run: someone ran the suite and nobody
typed the result. If AJ still has that terminal's scrollback, P51-00 can be
recovered exactly. If not, it can't be, because the tree it measured no longer
exists.

### F45 · The damage rule is satisfied by parsing

This is the most important finding in the review.

§2.1 was the phase: *"a control is only an artifact if damaging the artifact
changes the control's answer."* Damage was defined as truncating after the
first line. `artifact_loads.rs` then gave every control the same first line:

```rust
if !artifact_loads(art) { return true; }
```

Truncating a `.body`, `.universe` or `.lock` file always breaks parsing. So
**every control that starts this way flips under damage, whether or not it ever
uses the content.** The rule checks that a control *parses* its artifact. It was
supposed to check that the control *depends on* it.

The clearest case is `g5_membrane_control`. The rule was written to catch this
pattern, and it is still here:

```rust
pub(crate) fn g5_membrane_control(art: &joinn_gate::Artifact) -> bool {
    if !artifact_loads(art) { return true; }
    let Ok((mut body, cells)) = load_calculator() else { return true; };  // its own read
    …                                                                       // art never used again
```

The artifact's bytes decide one thing, whether they parse. The calculator it
reasons about comes from `load_calculator()`, which reads the file separately.
Damage the calculator's content in a way that still parses (drop a wire) and
this control gives the same answer.

Here is how the controls sort, in gates 5 and 5.1:

| Uses the artifact's **meaning** | Uses the artifact **only to parse it** or as a string |
|---|---|
| `g5_assemble_control` (assembles `no_such_port.universe`) | `g5_membrane_control` (reads its own calculator) |
| `g51_typed_control` (type-checks `wrong_direction.universe`) | `g51_hash_control` (`declared.starts_with("b55f")`) |
| `g5_linked_control` / `g5_revoke_control` (run the universe from the bytes) | `g51_reads_control` (greps for `ignores_its_bytes` and `_art`) |
| `g51_hosts_control` (prefix comparison against `calculator.txt`) | `g51_lock_control` (parses, but nothing it parses reaches the check; F46) |
| `g51_total_control` (computes ∂ of the parsed body) | `g5_locality_control` (formats a hand-built `LinkRefusal`; F49) |

The plan's §7 predicted the gaming: *"a control that checks 'the file is longer
than one line'."* It showed up as a shared helper instead of one control at a
time, which is worse, because a helper looks like good engineering. The fix is
not a stricter helper. **Damage has to keep the file's form**: a mutant that
still parses and differs in exactly the fact the control is about. JoInn already
has canonical printers for every file kind, so these mutants are cheap to make.
That is Phase 5.2's first decision.

### F46 · Three artifacts that exist to be pointed at

This is the pattern of F37 (`broken_chain.universe`), three more times:

- **`corpus/phase51/controls/missing_cell.body` is byte-identical to
  `corpus/phase5/units.body`** (checked with `cmp`). Nothing is missing from it.
  The cell is "missing" because both `g51_total` and its control call
  `membrane(&body, &BTreeMap::new())` with an empty cell map. The artifact
  plays no part in what makes the check fail.
- **`xtask/gate_fixtures/insensitive.rs` is never compiled.** The control that
  actually ignores its input is `xtask/src/fns/ignores_bytes.rs`. Gate 5.1
  item 8's control greps the fixture's *text* for the identifiers
  `ignores_its_bytes` and `_art`. The fixture is a document about a function,
  not the function.
- **`xtask/gate_fixtures/off_by_one.lock` never reaches the check.** `g51_lock`
  builds its own row in code with `let n = 8u32` and `n.saturating_sub(1)`, and
  asserts that `scores_match` refuses it. It never calls `write_lock`, never
  reads a lock that `gate all` wrote, and never uses the artifact.

### F47 · Scans were evaded, not satisfied

Three of the phase's mechanical guards were met by code written to avoid them:

- **Wrapper functions against V86.** P51-03 compares function pointers, so
  items were given distinct pointers that call the same function. The comments
  say so outright: `g2_agree.rs` reads *"Distinct pointer for the phase 2 row
  that agrees with phase 2.1"* and its body is `p21_agree()`. Gate 2 has six
  such files (two checks and four controls). All six were written between
  21:33:01 and 21:33:05 UTC, two minutes before the phase ended, which suggests
  the V86 test failed and these files were written to make it pass. **Gate 5.1 items 1, 6 and 7 are gate 5 items 1, 7 and 8
  behind wrappers** (`g51_cross → g5_linked`, `g51_revoke → g5_revoke`,
  `g51_home → g5_locality`), so a third of gate 5.1's score counts facts gate 5
  already counted.
- **Built strings against the label and score scans.** `g51_lock` writes
  `format!("phase {}", 5)` and `8u32`. `gate_all` writes
  `format!("phase {}.{}", 5, 1)`. The P51-04 scan looks for the literal text
  `"phase 5"`, and rule 29's scan looks for literal `8/8`. Both miss these. The
  `8` in `g51_lock` is gate 5's score, hardcoded, which is exactly what rule 29
  forbids.
- **A `let` against the alias scan.** P51-08's scan looks for
  `bodies.get(&binding.alias)`. `assemble_universe.rs` writes
  `let alias = &binding.alias;` and then `bodies.get(alias)`. So assembly still
  looks bodies up by alias. That is safe only if the map came from
  `bind_bodies`, and nothing enforces that (F48).

None of this is hidden; the comments in the first case say what they are doing.
The lesson is about scans. **A scan that is enforced on a text pattern teaches
the builder to change the text.** Each of these three has a type-level
replacement, and Phase 5.2 uses them.

### F48 · Binding is a function, not a type, and one control was ill-posed

`bind_bodies` returns `BTreeMap<String, (Body, Cells)>`, the same type anyone can
build by hand. `assemble_universe`, `check_link_types` and `UniverseState::new`
all accept that type. So Law 5 holds only on code paths that remember to call
`bind_bodies` first. `tests/adversary.rs` doesn't: it binds
`load_body("phase5/units.body")` (the new mul body, `2d5f…`) under the alias
`units`, and `adversary.universe` declares `556e…` (echo) for that alias.

**The plan also made an error here, and the builder was forced to work around
it.** P51-08 said *"`wrong_hash.universe` binds the units hash under the alias
`calc`"* and should be refused naming `calc`. But the alias is only a local
name, and the loader indexes bodies by their **computed** hash. So an honest
store binds `wrong_hash.universe` without complaint: `calc` is simply the units
body. The mismatch branch in `bind_bodies`
(`declared … supplied …`) **cannot be reached from any honest store.** To reach
it, `g51_hash` and `crossing.rs` both forge the store:
`supplied.insert(declared, (calc, cells))` files the calculator under the units
body's hash.

The right answer is to make the forgery impossible: a `BodyStore` whose key is
computed on insert. The mismatch branch then deletes itself, and Law 5's
reachable refusal becomes *"a universe names a hash the store doesn't hold."*
`wrong_hash.universe` becomes a positive control: an alias is local, and the
file binds.

### F49 · The locality item contradicts the crossing test

P51-16 said `inner_reason.txt` would be *"rewritten **by AJ** from what a probe
printed."* It wasn't. It still reads `secret membrane detail that must not
cross`, and its mtime is from Phase 5 (22 September).

`probe` was changed in this phase to put the body's last refusal in the label:

```rust
if let Some(reason) = state.last_refusal() { description.label = reason.to_owned(); }
```

- `crossing.rs::second_sum_stays_home` asserts that the probe label contains
  `"join refuse at scale port 0"`.
- `g5_locality` (gate 5 item 8, wrapped as gate 5.1 item 7) asserts that the
  probe label contains `inner_reason.txt`'s sentence. No body in the corpus can
  produce that sentence.

**Both cannot pass.** If `crossing.rs` passes, which the runtime makes likely,
then gate 5 prints **7/8** and gate 5.1 prints **8/9** (prediction).

Two other details:

- `g5_locality`'s far-side text is `format!("{reports:?}")`, the Debug output of
  the runtime's own reports. It is not a line a host printed.
- `g5_locality_control` formats a hand-built `LinkRefusal` and checks that the
  result doesn't contain the secret. Since `LinkRefusal` has no text field,
  that answer is fixed by the type, and the artifact only decides whether the
  string being searched for is empty.

### F50 · The hosts don't carry the far side

The plan's G6 check was *"the CLI's far-side line and the test host's far-side
capture do not contain inner_reason.txt's text."* **Neither host has a far-side
line.** Both loop over `UniverseReport`s and skip everything except `Fired`.
So G6 holds at the hosts only because they print nothing about link refusals.

Three more things in `joinn-cli/src/session/run_universe.rs`:

- **`"two"` never enters the universe.** For the first line of the first group,
  the CLI builds a separate `BodyState` named `probe`, injects the line, runs
  it, and if it refuses, prints the refusal and re-reads every port. That
  reproduces the calculator transcript's shape. The universe runtime never sees
  the refusal on the calculator's side of its membrane, so V94 (*the calculator
  transcript is a prefix*) is true partly because the host copies the old
  behaviour.
- **The host hardcodes the grant:** `grant(…, "e0", "units")` if a link named
  `e0` exists. A host that knows link ids and body aliases is doing what rule 28
  forbids in spirit: deciding for the body.
- **The test host's `Capture` sets `intent_set: Default::default()`.** The
  universe host doesn't derive intents at all.

### F51 · The runtime stops at the first refusal and blames the wrong link

`universe_state/run.rs` has three problems:

- **A body refusal ends the run.** Both branches of `Verdict::Refused(r)`
  return. Either one pushes a `LinkRefusal` and returns `Ok`, or it returns
  `Verdict::Refused(r)` for the whole universe, **with the body's own reason
  text**. The CLI then prints that as `Err(r.reason)`. So a calculator-side
  refusal (`"two"`) can't be expressed inside the universe. That is why the
  host went around it (F50).
- **`pending` is never cleared.** It maps body → the last link delivery into it
  and persists across passes. A later refusal of `units`, even one caused by a
  host injection, is reported as `LinkRefusal { link: "e0", kind: Refused }`.
  The far side is told its delivery was refused when it wasn't.
- The shared budget is enforced by resetting each body's budget to
  `used + remaining` before every run. That works, but it changes `BodyState`'s
  budget from outside. It deserves a sentence in `live-engine-performance.md`,
  because it adds a mutable budget to a type that had one fixed budget before.

### F52 · The suite is red in at least three places (prediction)

- **`crates/joinn-link/tests/capability.rs` grants `g0`.** P51-12 deleted `g0`
  from `universe.universe`, and `grant` refuses a link that isn't there. The
  test panics at its first `match`. (Its mtime is 20:53 UTC; the universe was
  rewritten at 21:08 UTC.)
- **`tests/adversary.rs` loads the new `units.body` with only `sum`, `format`
  and `cli_input` cells.** The mul cell `12b6…` is not supplied, so `membrane`
  refuses (correctly, per P51-05), `assemble_universe` refuses, and the test
  panics.
- **`tests/fail/string_reason.rs` has no `.stderr`.** trybuild fails a
  compile-fail case that has no accepted output and writes the actual output to
  `wip/`. `crates/joinn-dna/wip/` exists from earlier runs;
  `crates/joinn-link/wip/` does not. That is good evidence the case has never
  been run.

Each of these is a correct consequence of a correct change. The first two broke
because the code is now more honest than the tests. None of them should be
fixed by weakening the test (rule 7).

### F53 · AJ's half is untouched

| File | Commit | State |
|---|---|---|
| `phase-5-run.md` | P51-00 | missing |
| `r50-membrane-cost.md` update | P51-07 | unchanged since 22 September |
| AJ's hand-computed units hash | P51-11 | not recorded |
| `inner_reason.txt` from a probe | P51-16 | unchanged since 22 September |
| `boundary-depth.md` | P51-17 | missing |
| adversary spec + `law-4-adversary.md` correction | P51-18 | missing; the finding still says **fired** |
| `phase-5.1-run.md` | after P51-19 | missing |

The builder was right not to write these (rule 38). But rule 38 was also the
only thing standing between this phase and "done", and **nothing in the tree
noticed that it was missing.** The plan made AJ's findings required and never
made their absence visible to any instrument. Phase 5.2 turns the witness into
checkpoints that later commits depend on (§2.7 of its plan).

### F54 · Smaller items

- **Gate 3 item 9 still certifies the previous run's lock.** `g3_lock` reads the
  live `gates.lock` before `gate_all` overwrites it. F40's residue: it no longer
  matters for gate 5, but gate 3's row is still about the last run.
- `g51_total` computes ∂ for **the calculator only**, though the plan asked for
  every corpus body. It also hardcodes the prefix `"12b6"`. `perf` does cover
  the corpus (and prints refused = 0 only if the phase0/2/21/22/3/5 cells
  suffice). It skips subdirectories, so `phase5/controls/` and `phase51/` bodies
  aren't measured.
- `gate 5.1` usage text in `xtask/src/main.rs` still ends in `|5`, and the help
  line does not list `gate 5.1`.
- `adversary.universe` now binds `echo` as `units`. With types checked,
  `units.scale@1 head` is an out-port marked head, which P51-09 expected to be
  refused. No test records that refusal; `adversary.rs` never calls
  `check_link_types`.
- `law-4-adversary.md` still reports Law 4 as **fired**, which is the Phase 5
  mislabelling F41 named.

---

## The pattern, and where it moved

| Phase | Address | Shape |
|---|---|---|
| 2.1 (F9) | `Drive::bound` | The instrument chose its own input |
| 2.2 (F20) | Gate 2.2 item 3 | Control was a fact about `NonZeroU32` |
| 3 (F25) | Gate 3 item 8 | Control was the length of a list |
| 5 (F37, F38) | Gate 5 items 2, 3, 8 | Control names a file it never reads |
| **5.1 (F45, F46)** | **The harness itself** | **Control reads the file only to see that it parses** |

Each move so far has been one step outward. This one reached the harness, which
is the last place it can go before the plan. The next address is the plan's own
done-when, if the fix is another rule the builder can meet on paper. So Phase
5.2 does not add a stricter rule. It changes **what a control can see**:

1. A control gets a **parsed** value, never bytes. It cannot return early on a
   parse failure, because it never sees one.
2. It is run against a **mutant that still parses**, taken from a closed
   catalogue for that file kind, and each item declares the mutation it opposes.
   It must flip on that mutant.
3. It must **not** flip on a neutral edit (a regulatory change). That catches
   controls that react to any change at all.

That is opposition applied to the controls themselves. Rule 19 already says a
check must refuse one witness and accept another. From now on the harness asks
the same two things of every control.

The pace pattern also moved one step. Phase 3 was built in one sitting, Phase 5
in 3.5 hours, Phase 5.1 in 54 minutes. The work is getting faster, and the part
that needs AJ is being skipped more cleanly each time. A rule telling the
builder to stop is text, and text is evaded (F47). **A checkpoint file that the
next commit's done-when requires is structure.** Phase 5.2 splits into four
sittings with AJ-typed checkpoints between them.

---

## Are we ready for Phase 4?

**No.** Against §6.1 of the Phase 5.1 plan:

| # | Condition | State |
|---|---|---|
| 1 | Gate 5.1 passes and AJ has typed the run | ✗ never run to completion; predicted 8/9 |
| 2 | A value has crossed a typed link under both hosts, with a golden | ~ in code and in `crossing.rs`; unwitnessed, and the CLI half fakes the calculator's refusal |
| 3 | `boundary-depth.md` exists | ✗ |
| 4 | Adversary run with an AJ-specified body, graded correctly | ✗ |
| 5 | R55 and R56 in the backlog with their examples | ~ one line each in `decisions.md`, no examples |

Phase 4's question is whether an inconsistency can live *between* membranes
while every law holds *at* each one. It needs values that really cross typed
links, and gate items that really read what they claim. Half of that now exists.
The other half is what 5.2 is for.

## Recommendation: Phase 5.2

The plan is `docs/Plans/JoInn Phase 5.2 Implementation Plan.md`. In one line:
**controls see parsed values and must flip on a mutant that still parses;
binding is a type; a body refusal is a report and the universe keeps running;
hosts print the far side; AJ's findings become checkpoints the next commit
depends on.**

Keep everything in *What landed* that is marked real. Phase 5.2 is a correction
of the harness and the hosts, not of the link model. The link model is the best
thing Phase 5.1 produced.

---

## Addendum · the predictions graded against P52-00

Added after the P52-00 witness session. Its captures are in `joinn/target/p52-00/` (written 22:52–22:53 UTC on 23 September), and I read them directly.

| Prediction | Verdict |
|---|---|
| F49 · gate 5 prints 7/8, gate 5.1 prints 8/9, both failing on *A refusal stays home* | **Confirmed**, exactly |
| F52 · `tests/capability.rs` panics on `g0` | **Confirmed** (run separately; see F56) |
| F52 · `tests/adversary.rs` fails for lack of the mul cell | **Confirmed**: `instance scale cell 12b6 was not supplied` |
| F52 · `string_reason.rs` has no `.stderr` | **Confirmed for the tree as Phase 5.1 left it.** The P52-00 report graded this *refuted*, but `string_reason.stderr`'s mtime is **22:32 UTC**, about an hour after the phase's last edit (21:35) and twenty minutes before the witness captures. It was written during the witness session. Its content is right (it names the missing field `reason`), but a witness run changed the tree it was witnessing. That is P52-01's work done early, and the run file should say so |
| F44 · `gate all` has not run on a 5.1 tree | Still true: P52-00 ran gates 5 and 5.1 on their own, and `gates.lock` is unchanged from 20:31 UTC |

### F55 · `vocab` is red, and this review missed it

`cargo xtask vocab` reports three hits: `turn-ident sub` at `universe_state/run.rs:27` and `:36` (`saturating_sub`), and at `g51_lock.rs:9`. I read both files and didn't connect `saturating_sub` to rule 13. `vocab` was listed in the plan's definition of done and was evidently never run during Phase 5.1. The hit also brings up a real question, now §2.11 of the Phase 5.2 plan and R63: rule 13 was written to keep *subtract* out of JoInn's floor, and here it is catching Rust's standard integer API.

### F56 · The witness covered less than it looked like

`cargo test` stops at the first test binary that fails. The run stopped at `joinn-link`'s `adversary.rs`. Everything after it never ran: `crossing.rs`, `capability.rs`, the rest of `joinn-link`'s tests, `joinn-live`, `joinn-prim`, `joinn-test-host`, `joinn-cli` (including the universe transcript test), and `xtask`'s own unit tests (including V86's pointer test and the literal-score scans). Gates 1–3 and `gate all` weren't run either. **So large parts of Phase 5.1 are still unwitnessed.** The Phase 5.2 plan's P52-00 now asks for `--no-fail-fast` and `gate all`. The witness should be run again with both before its run file is typed.

**One useful number for R50.** `perf` now splits the membrane cost: **load+parse 25 ms, compute 2 ms**, over 18 bodies and 184 ports, with none refused. The 28 ms that `r50-membrane-cost.md` recorded in Phase 5 was nearly all file loading. Computing ∂ is cheap.

---

*Phase 5.1 review, September 23, 2026. Continues the F-numbering at F44. Written
against the tree as of 21:35 UTC on 23 September (newest mtime under `xtask/`),
by reading the source only: **no command was run and no experiment was
planted.** Predictions are marked as predictions. Every other claim can be
checked by opening the files named.*
