# Phase 5 review — are we ready for Phase 4?

Reviewer: Claude · September 22, 2026 · against `docs/Plans/JoInn Phase 5 Implementation Plan.md`

**Method, same as Phase 3's and just as limited.** This review **ran nothing.**
The bridge in this session can list and copy files from `D:\JoInn`, but it has
no shell, so the tree was read one file at a time. That covered every source file
in `crates/joinn-link` and `crates/joinn-assay`, every `g5_*` file and the gate
plumbing in `xtask/src/fns`, all of `corpus/phase5/` and its controls,
`gates.lock`, `hashes.txt`, and the four findings the phase wrote. Each claim
below can be checked in the source. None of them is quoted from a command's
output.

Four further limits:

1. `gates.lock` reads `phase 5: 10/10`. **I read that number from a file and did
   not reproduce it.** Nothing below depends on the run: every finding is about
   what an item checks, not whether it printed `ok`.
2. The timeline comes from mtimes. The plan was written at 15:54 UTC on
   22 September; `gates.lock` was last written at 19:33 UTC. That is twenty-five
   commits in about three and a half hours.
3. `parse_universe.rs` (17.5 KB) was only checked for hash handling. The CLI's
   `run_session.rs` was staged but not read line by line.
4. Numbering continues from the Phase 3 review. F25–F32 are Phase 3's, so
   **F33 is the first finding here.**

---

## Verdict

**The correction block landed. The link graph did not. Phase 5 has two bodies
and a universe file, but no value has ever crossed a hyperedge, and the second
body is the fixture the plan's own risk table warned about. Do not open Phase 4
on this. Open Phase 5.1.**

Most of what the phase set out to repair, it repaired. P5-00 is a real witness
record, and it is honest about the four hollow items it inherited.
`run_gate_table` resolves every `control_artifact` before any item runs, and a
missing path refuses the whole run, naming the item. `parse_lock_scores` is a
real parser, and `bad.lock` is refused, naming `phase 3`. The gates return
their own scores, and the literals are gone from `gate_all.rs`. `intent_set`
now comes from ∂. `joinn-assay` depends on `joinn-frame` alone, and it contains
no homology or rank identifiers. `grandfather.txt` is still comments only, and
no hash from Phases 0–3 moved. So F25–F30 are closed properly.

The phase's turning point also held. **A membrane is computed and stored
nowhere.** `membrane(calculator)` returns `{cli_a@0, cli_b@0, sum@2}` by value,
and nothing in `.body` or `.universe` declares it. Four checks refuse the way
the plan said they would, each against a file a reviewer can read:

- `transits.universe` is refused, naming `calc.sum@0` and the wire
  `cli_a@1 -> sum@0`.
- `two_systems.universe` is refused, naming `units`, `calculation` and
  `measurement`.
- `wrong_container.universe` fires both Law 4 refusals, and each names its
  container.
- Membership is checked lens by lens.

Those are real instruments. The refusal on `transits.universe` is the best new
message in the repo, and it is shaped by rule 35.

What did not land is the part that made this phase Phase 5. The exit gate's
first line reads *"the result crosses the hyperedge."* No code in the tree
moves a value from one body to another. The capability that "rides an ordered
hyperedge" is an entry in a `BTreeSet<(String, String)>`. The link that "hides
the inner reason" never reads the inner reason. And the universe that passes
every check connects an ℤ out-port to a Text in-port.

---

## What landed

| Commit | Landed |
|---|---|
| **P5-00** | `phase-3-run.md`. It quotes each command's last line, and it names the four hollow items and the finding for each. Honest. |
| **P5-01/02** | `GateItem::control_artifact`; `run_gate_table` resolves it; `artifact_list.txt` deleted; gate 3 is a table; `gate_fixtures/unresolvable.txt`. **F25 closed** as the plan wrote it. See F37 for what is left. |
| **P5-03** | `parse_lock_scores` + `bad.lock` → refused naming `phase 3`. **F26 closed** for gate 3. See F40. |
| **P5-04/05/06/07** | Gate 3's duplicate items split. `intent_set` comes from ∂. `two_in_ports.body`. V61 builds a description. Scores are returned. **F27–F30 closed.** |
| **P5-08** | `trybuild` for a writing `probe`; `host_fixtures/plant_render.rs`. The README was rewritten but already reads stale (F43). |
| **P5-09** | A generic CLI session; `no_instance_names.rs` scans `joinn-cli` for instance names. **F31 closed.** |
| **P5-10/11** | `joinn-assay`: `BlockId`, `Chain`, `Complex`, `boundary`, `assemble`. It refuses in three ways, each naming the block. It is pure and depends on `joinn-frame` alone. |
| **P5-12** | `membrane()` in `joinn-link`, derived, with nothing added to `.body`. See F39. |
| **P5-13/14** | The `.universe` file kind: parser, canonical printer, `joinn.universe.v1`, CSR arrays that are never hashed (`csr_no_hash.rs`). See F36. |
| **P5-15** | `units.body`. See F34. |
| **P5-16** | Touch-only via `assemble_universe`. The refusal names the port and the wire. See F42. |
| **P5-17/18** | Lenses, exclusivity, Law 4. **These are the phase's most solid work.** |
| **P5-19** | `grant` / `revoke` / `check_capability`. See F33. |
| **P5-20** | `far_side_refusal`. See F38. |
| **P5-21** | Six perf probes, and R50 got a number. See F39. |
| **P5-22** | `bus.body`, `adversary.universe`, `law-4-adversary.md`. See F41. |
| **P5-23/24** | `phase-5-hashes.md`, gate 5 as a table, `phase 5: 10/10`. |

Twenty-five commits, and every one left an artifact. Nothing was silently dropped
except `bad_dimension.universe` (F43).

---

## Findings

### F33 · Nothing crosses a link

`g5_linked.rs` is gate 5 item 1, *"Two bodies, one universe."* It does five
separate things:

1. It parses `universe.universe`, runs `check_law4` and `assemble_universe`.
2. It calls `grant(rt, u, "g0", "scale", "units")`, which inserts a pair into a set.
3. It runs **the calculator alone** under `joinn-test-host`, with 2 and 3 as input.
4. It runs `joinn-cli` on the calculator alone.
5. It runs **units alone**, under both hosts, with input `7`.

Then it `&&`s the five booleans together. The two bodies are never in the same
`BodyState`, a value is never taken from `calc.sum@2`, and nothing is ever
injected at `units.scale@0`. There is no type in `joinn-link`, or anywhere
else, that holds two running bodies.

`LinkRuntime` holds `held: BTreeSet<(String, String)>`. `check_capability` looks
a pair up in that set. It is not called on any delivery path, because no
delivery path exists. Exit gate item 7 reads *"the receiving body succeeds,
`revoke` runs, the next attempt is refused."* The receiving body never makes an
attempt. What the item actually tests is `BTreeSet::remove`.

**Exit gate 5 items 1 and 7 are unmet.** The roadmap's Phase 5 gate is
*"a two-body universe where … a capability passed across a system boundary can
be revoked, after which the receiving body's next attempt is refused."* That
gate needs a runtime, and the phase did not build one.

### F34 · The second body is a fixture, not a body

The plan's risk table, verbatim:

> **The second body is a fixture, not a body** · A units body that only exists to
> be linked to will get written as two ports and a stub, and then V16 is tested
> against something that could not fail for any other reason. It must be admitted
> by the gate, run standalone under both hosts, and be a body a first-grader could
> have built.

`units.body` contains one instance of `cli_input`, the calculator's own parser
(`c4a0a132…`), renamed `scale`. It converts nothing and knows no units: it reads
`"7"` and prints `7`. `bus.body` is the same cell a third time, renamed
`listen`. The regulatory label *"the sum, converted"* describes something this
universe cannot do.

That body passes the letter of P5-15: the gate admits it, and it runs under
both hosts. What it misses is the reason P5-15 exists. **Every Phase 5 check
that involves `units` would pass against any body with a Text in-port.**

### F35 · Links check neither frame nor direction

`cli_input`'s contract is `port 0 in Text 1` and `port 1 out ℤ 1`. With that in
mind, here is what each accepted link in the phase5 corpus connects:

| Universe | Link | Members | What it connects |
|---|---|---|---|
| `universe.universe` | `e0` | `calc.sum@2 tail`, `units.scale@0 head` | **ℤ out → Text in** |
| `universe.universe` | `g0` | `calc.cli_a@0 tail`, `units.scale@0 head` | **Text in → Text in**: the tail is an in-port |
| `adversary.universe` | `bus` | `calc.sum@2 tail`, `units.scale@1 head`, `bus.listen@0 head` | the second member is **an out-port marked head** |
| `ordered.universe` | `path` | `units.scale@0 head`, `calc.sum@2 tail` | ℤ out → Text in again |

`joinn-link` never mentions a frame or a direction; a grep for `frame`,
`Frame` and `Direction` under `crates/joinn-link/src` finds only imports.
`membrane()` returns bare `(instance, port)` addresses, which throws away the
two facts a link would need in order to be judged.

This is the most useful finding in the review. Frame-match at snap time is
Part II §8's *cheapest opposition*, the one the creator's UI is designed around.
It is missing at the single place where two independently written blueprints
meet. It also answers R49 (*is a link admitted?*), because a link has a contract
after all: its members' directions and frames.

### F36 · Universe body hashes are decorative

`parse_universe` reads `body:<hex> as <alias>` into
`BodyBinding { hash, alias }`. After that, `assemble_universe` looks bodies up
with `bodies.get(&binding.alias)`, and `load_phase5_bodies` loads them by
**path** (`phase2/calculator.body`, `phase5/units.body`). Nothing compares
`binding.hash` with `hash(&body.coding)`.

A universe that says `body:00…00 as calc` assembles exactly as
`universe.universe` does. The universe's own hash covers the body hashes, so it
changes when they change. But nothing ties those hashes to the bodies that
actually get loaded. **Law 5 stops at the universe boundary.**

### F37 · Controls that point at an artifact they never read

Rule 24's new type says a control *is a path*. `run_gate_table` checks that the
path exists. Nothing checks that the control **uses** it:

| Gate 5 item | `control_artifact` | What the control actually does |
|---|---|---|
| 2 · membrane | `corpus/phase2/calculator.body` | adds a `Wire` to an in-memory body |
| 3 · assembles | `corpus/phase5/controls/broken_chain.universe` | builds a four-block `Complex` in code. **The file has no links and a regulatory note reading *"assemble names block 3"*.** A `.universe` file cannot express a 2-block at all |
| 7 · revoke | `corpus/phase5/universe.universe` | the positive universe; the control is the pre-revoke half of the check itself |

`broken_chain.universe` is the clearest case. It was written so that item 3
would have a path to resolve. The plan's §0 predicted it:

> **Phase 5's address is the `.universe` file.** A new file kind is a new place to
> write down structure that already exists somewhere else…

F25's pattern (a control that is an expression evaluated beside its check)
survived rule 24 by pointing at a file. A rule stated as a type can be satisfied
by providing a value of that type and never using it.

### F38 · Refusal locality holds because the function ignores its input

`far_side_refusal.rs`:

```rust
pub fn far_side_refusal(universe, link_id, member_body, member_instance, member_port,
                        _inner: &Refusal) -> Refusal {
    let _ = universe.coding.links.iter().find(|l| l.id == link_id);
    Refusal::structural(CheckId::Other, format!("link {link_id} refused at member …"))
}
```

The inner refusal is named `_inner` and never read. The link lookup is thrown
away with `let _`. The function formats its own arguments. Item 8 then builds an
inner refusal, putting the contents of `inner_reason.txt` into it itself, and
asserts that the output does not contain that text. It can't, because nothing
would have put it there.

The test's *"control half"* in `tests/locality.rs`:

```rust
// A probe on the refusing side still sees the inner reason (control half).
assert!(inner.reason.contains(&inner_secret));
```

This asserts that a string the test built contains a string the test inserted
two lines earlier. It is not a probe. No body refuses anything and no link
carries anything, so G6 is true only because nothing moves.

### F39 · ∂ skips things without saying so, and R50's number measures that

`membrane()` returns a `Verdict`, but it has no refusal path:

```rust
let GenomeTarget::Cell(hash) = &entry.target else { continue; };   // prims skipped
let Some(cell) = cells.get(hash) else { continue; };               // missing cells skipped
```

This causes two problems.

- **∂ is wrong for any body containing a primitive instance.** That is every
  reference body in phase21/phase22. Nothing links to one yet, but the first
  link that does will be refused as "interior" at a port that is really on the
  membrane, and the message will not say why.
- **R50's number mostly measures empty results.** `membrane over corpus bodies:
  23 ports: 18 milliseconds: 28` is eighteen ports across twenty-three bodies,
  because `membrane_over_corpus` loads only `corpus/phase0` cells. Most bodies
  produce an empty set without any error. The 28 ms also includes reading and
  parsing files. `r50-membrane-cost.md` calls 28 ms "more than a few
  milliseconds" and opens R50 on that basis. The number is real, but it does
  not answer R50's question.

### F40 · Item 10 is a substring check again, and item 9 is a copy

`g5_path.rs` passes when:

- the **existing** `gates.lock` parses as all passes (that is the lock the
  previous run wrote, which the current run has not yet overwritten), and
- the text of `gate_all.rs` contains the substring `"phase 5"`.

That is F26's pattern: Phase 3's item 9 checked for two substrings in the lock,
and this item checks for one substring in the source. On a clean checkout it
certifies whatever lock was committed. The plan's done-when for P5-24 (*"fails
when any item's control passes"*) is about `run_opposed`. It says nothing about
whether the lock records this run.

`g5_artifacts.rs` and `g3_artifacts.rs` are the same file with a different doc
comment, and so are their controls. P5-04's rule (*no two items share a check
function*) was scoped to gate 3. Across gates it was broken the next day.

### F41 · The Law 4 adversary was chosen and graded by the builder, and mislabelled

`law-4-adversary.md` is titled *"Law 4 adversary · fired"* and concludes *"Law 4
holds."* The plan's P5-22 used **fired** for the *other* outcome: the body can't
be expressed and `law-4-underspecified.md` gets written. By the plan's own
definition, this adversary did not fire.

The attempt was also weak. A body that wants *"to hear both the sum and the
units scale in one place"* is asking for a three-member hyperedge, which is
exactly what a hyperedge is. The roadmap's adversary is about wanting a
communication method **neither container offers**: request and response with
correlation, a stream with backpressure, shared state two bodies both write.
None of those was tried. The link it wrote is also untyped (an out-port marked
`head`; see F35).

The mtimes show the finding saved at 19:24:53 and `tests/adversary.rs` at
19:24:59. **The finding was written six seconds before the test that supports
it.** Rule 17 keeps `xtask` from writing its own findings. Nothing keeps the
agent that writes the check from also choosing the adversary, running it, and
recording the result in one pass. That is rule 20 (*no instrument chooses its
own inputs*), one level up.

### F42 · ∂∂ = 0 can't fail on any universe the grammar can express

This is not a bug; it is a gap in the plan's own argument, and it is worth
getting right before Phase 4 builds on it.

`assemble_universe` builds 0-blocks (membrane ports) and 1-blocks (links),
nothing else. A 0-block's boundary is empty, so the ∂∂ of a 1-block is ∂ of a
sum of 0-blocks, which is **always zero**. The closure test in `assemble`
(`if dim >= 2`) is never reached from a universe. The refusal that fires on
`transits.universe` is the well-formedness branch: *"endpoint N is not a 0-block
of this complex."* That is a set-membership test: is this port in C₀?

§2.1 claimed *"V16 is the output of a measurement, not a predicate."* That is
half true, and the true half comes from **§2.5, not §2.1**. The check can fail
because C₀ is **derived** from the body rather than declared beside the check.
If the membrane were written in the universe file, the same membership test
would be a comparison of two copies. The derived membrane is what gives
touch-only its teeth. ∂∂ adds nothing yet, and cannot until something produces
2-blocks. Part III says those are the fillings from laws, which is Phase 4's
derivation.

One place ∂∂ *could* bite now: if the complex were **augmented** (a map
ε : C₀ → ℤ, with ε∘∂ = 0 required on every 1-block), then a link's tails would
have to balance its heads. `e0` (one tail, one head) passes. The adversary's
`bus` (one tail, two heads) would be **refused**, which turns the question
*"does a fan-out conserve?"* into something checkable. That is a design
question, not a fix, and it goes in the backlog as R55.

### F43 · Smaller items

- `bad_dimension.universe`, one of §3.2's three artifacts, does not exist. The
  refusal is exercised only in unit tests.
- The doc comment on `revoke` says *"A missing grant is still Ok: revoke is
  idempotent,"* but the code refuses on a missing grant. The code is right; the
  comment is wrong.
- `README.md` says Phase 5 is *"the work underway."* `Guides/03-where-we-are.md`
  still describes Phase 3.
- No phase-5 run was written down. P5-00 set the rule that a phase opens by
  recording the tree it inherits; the next phase has to open with that record.
- The CLI perf probe went from 301 ms to 2 525 ms. `live-engine-performance.md`
  explains it as spawn cost, and that is probably right (the test-host number
  didn't move). Still, a 10× change in one probe needs a sentence explaining
  it, not just a new table row.

---

## The pattern, and what changed about it

In the Phase 3 review, the recurring failure (a control that can't fail) had
moved four times. The Phase 5 plan's answer was to give rule 24 a type, and it
worked against the old version: no control is a line count now, and every
control names a file. The failure moved one step, as the plan said it would:

| Phase | Address | Shape |
|---|---|---|
| 2.1 (F9) | `Drive::bound` | The instrument chose its own input |
| 2.2 (F20) | Gate 2.2 item 3 | Control was a fact about `NonZeroU32` |
| 3 (F25) | Gate 3 item 8 | Control was the length of a list |
| **5 (F37, F38)** | **Gate 5 items 2, 3, 8** | **Control names a file it never reads; the check's function ignores its input** |

The next fix follows directly. **An artifact is a control only if damaging it
changes the control's answer.** The harness can check that mechanically: give
each control the artifact's bytes, run it once on the real bytes and once on a
damaged copy, and require the answers to differ. `broken_chain.universe`
would fail that immediately, and so would `inner_reason.txt` under the current
`far_side_refusal`. That is the first commit of the next plan.

The second pattern is the pace. The Phase 3 review noted that the whole phase
was built in one sitting. Phase 5 was built in 3.5 hours, and its findings are
timestamped seconds after the code they judge. Speed isn't the problem.
Whatever writes the code is also choosing its adversaries and writing its
witnesses, and the only checks that step can't satisfy on its own are the ones
AJ does by hand. The next plan should make at least one of those a commit.

---

## Are we ready for Phase 4?

**No.** The reason is the one the Phase 5 plan gave for deferring Phase 4 in the
first place (§1.1):

> With one body there is no second membrane for the inconsistency to live
> between, so Phase 4 run now returns "inconclusive, complex too shallow."

Phase 4's adversary is the decoration check. Its candidate for something `require`
and `ensure` can't catch is an H¹ inconsistency across membranes: every law holds
at every membrane, yet no consistent global state exists. That requires values
that actually flow across membranes, through links whose endpoints are typed.
Right now the universe has one cell used under three names, joined to the
calculator by a link from ℤ to Text that nothing runs through. Phase 4 on this
tree would spend its question and get "inconclusive" back.

## Recommendation: Phase 5.1

Phase 2 was followed by 2.1 and 2.2 for the same reason. The plan is
`docs/Plans/JoInn Phase 5.1 Implementation Plan.md`, and its conditions for
opening Phase 4 are:

1. **Artifact-sensitive controls.** Every control receives its artifact's bytes
   and must give a different answer on a damaged copy. (F37, F38, and the
   general fix for the pattern above.)
2. **A typed boundary.** ∂ returns direction and frame. A link's tails are
   out-ports, its heads are in-ports, and all its members share one frame. (F35)
3. **Bodies bound by hash.** A universe whose body hash doesn't match the loaded
   body is refused, naming the alias. (F36)
4. **An honest ∂.** Primitive ports are included, a missing cell refuses, and
   R50 is re-measured. (F39)
5. **A real second body.** `scale` is a `mul` over ℤ with a factor prompt, so
   `2 + 3 = 5` crosses `e0` and comes out `5 ft = 60 in`. (F34)
6. **A universe runtime.** Values leave tail ports and arrive at head ports.
   Delivery requires the head body to hold the link's capability, and revoking
   it stops the next delivery. (F33)
7. **G6 with a real refusal.** A second delivery into a `join refuse` port makes
   `units` refuse. The far side gets a link refusal that has no free text, and a
   probe on `units` sees the real reason. (F38)
8. **The lock records this run.** `gate_all` re-reads the lock it just wrote and
   compares it with the scores it returned. Item 10's substring check is
   deleted. (F40)
9. **The adversary, specified by AJ.** A request and response with correlation,
   attempted and graded, with `fired` used the way the plan defines it. (F41)
10. **A witness run typed by AJ.** The one commit Cursor cannot do. (F41, F43)

The Phase 5.1 plan also writes down F42: touch-only gets its teeth from the
derived C₀, and ∂∂ = 0 becomes a gate item only when Phase 4 adds 2-blocks.
Phase 4 should inherit that sentence rather than rediscover it.

---

*Phase 5 review, September 22, 2026. Continues the F-numbering at F33. Written
against the tree as of `gates.lock` mtime 2026-09-22 19:33 UTC, by reading the
source only: **no command was run and no experiment was planted.** Every claim
is checkable by opening the files named.*
