# JoInn Phase 5.1 Implementation Plan

**Something crosses · a working plan for Cursor**

*Phase 5's correction: a typed boundary, a real second body, a value that crosses a hyperedge, and controls that have to read their own artifacts*

Author: AJ · Draft 0.1 · September 22, 2026

> Status tags follow Parts I–IV: **DECIDED**, **PROPOSED**, **OPEN**. **PROPOSED · yours** marks a recommendation made while writing this plan. You can overrule it; Cursor implements whatever you settle on, not what is written here.

> **What this is.** Phase 5 gave the universe a second body, a file kind, lenses, Law 4 and a derived membrane. The phase review (`docs/Findings/phase-5-review.md`, F33–F43) found that no value ever crossed a link, that the second body is the calculator's parser under a new name, that links check neither frame nor direction, that universe body hashes are bound to nothing, and that three of gate 5's controls name a file they never read. This plan fixes that and nothing more. It is to Phase 5 what 2.1 and 2.2 were to Phase 2.

> **The one idea.** Phase 5's was *a forbidden connection is refused by a measurement, not by a predicate*. This one: **a control is only an artifact if damaging the artifact changes the control's answer.** Rule 24 said a control is a path, and the type made sure the path exists. It did not make sure the path is used, so `broken_chain.universe` became a file that exists only so something has a path to resolve. Phase 5.1 fixes the mechanism rather than the instance: the harness hands each control its artifact's bytes, runs it twice (once on the real bytes, once on a damaged copy), and refuses any control whose answer does not change.

> **The corollary that pays for it.** If ∂ carries direction and frame, a link has a contract after all. R49 asked whether a link is *admitted*, and the answer is yes: a link is admitted when its tails are out-ports, its heads are in-ports, and every member has the same frame. The frame check Part II §8 calls the cheapest opposition then covers the one place where two independently written blueprints meet, and the Phase 5 universe connecting ℤ to Text is refused.

> **Where Phase 5 left off, honestly.** `gates.lock` reads `phase 5: 10/10`. Nobody has watched it print, and no phase-5 run was written down. Five of the ten items would print `ok` whether or not the platform does what the item's name says (review F33, F37, F38, F40). The instruments that do work are touch-only on `transits.universe`, exclusivity, lenses, Law 4, and the derived membrane. **P51-00 is the witness run, and AJ types it.** That is the one commit in this plan that the builder cannot do (review F41).

---

## 0. How to Drive Cursor With This Plan

| | |
|---|---|
| **Where this file lives** | `D:\JoInn\docs\Plans\JoInn Phase 5.1 Implementation Plan.md` |
| **Where the code lives** | `D:\JoInn\joinn\`, no new crates |
| **What it closes** | F33–F43 of `docs/Findings/phase-5-review.md`. It moves R49 to PROPOSED. R50, R51 and R53 get partial answers |
| **Standing rules** | `AGENTS.md` as of Phase 5, **amended by Appendix A**. Rules 24 and 30 are rewritten; rules 36–40 are added |
| **Unit of work** | one numbered commit from §4, one Cursor session per commit. **Stop at the done-when. Do not start the next commit in the same session** |
| **How a commit ends** | its done-when command reports a refusal, a disagreement, a compile error, or a byte-identical replay, never a number the same command chose |
| **What Cursor may decide** | module layout (subject to rule 25), function bodies, error strings, how a `UniverseState` stores its bodies |
| **What Cursor may not decide** | anything in §2; the new `units.body` (§3.3); the delivery order (§2.8); the adversary (P51-16, which AJ specifies); any finding marked *typed by AJ* |

The prompt stays short:

> Implement commit **P51-07** from `docs/Plans/JoInn Phase 5.1 Implementation Plan.md`. Follow `AGENTS.md`. Stop when the done-when command passes and report what it printed. Do not write any file under `docs/Findings/` that the commit marks *typed by AJ*.

**The failure mode this phase must not have.** Each phase so far has had its hollow items in its newest construct. Phase 5.1's newest construct is the **universe runtime**. A runtime can easily be tested by checking its own bookkeeping: "the capability set no longer contains the pair" instead of "the body did not fire." Every runtime check in this plan is therefore stated in terms of **what a body did**: its description, its fire count, the value on its port. None is stated in terms of the runtime's internal state. If a done-when can be satisfied by reading a field of `UniverseState`, it has been written wrong.

---

## 1. Scope Fence

### In scope

- **The witness run** of the tree as Phase 5 left it, typed by AJ.
- **Artifact-sensitive controls**: the harness change that makes rule 24 impossible to satisfy on paper alone.
- **The lock as a postcondition** of `gate all`, replacing item 10's substring check.
- **An honest ∂**: primitive ports included, missing cells refused, then a **typed ∂** with direction and frame.
- **Universe bodies bound by hash.**
- **A well-typed link**: a direction rule and a frame rule.
- **A real second body** and a revised `universe.universe`.
- **A universe runtime** in `joinn-link`, plus both hosts running it.
- **Capability on delivery**, and **G6 with a real refusal**.
- **Writing down what ∂∂ does and does not do yet** (F42).
- **The adversary again**, specified by AJ.

### Out of scope — Cursor must refuse these even when they look small

| Not now | Where | Why it is tempting |
|---|---|---|
| Any 2-block, filling, H₀/H₁/H₂, invariance harness | Phase 4 | The runtime makes the complex feel "real enough." It isn't: no laws have been turned into fillings (§2.6) |
| The augmented complex (ε∘∂ = 0) as a check | R55 | It would make ∂∂ able to fail on universes today, and it would refuse the Phase 5 adversary. That is a design decision about fan-out, not a fix |
| Frame **embeddings** across a link (ℤ ↪ ℚ as a link adapter) | R56 | It is the obvious next step once frames are checked, and it is the first cross-membrane use of G3. Exact frame equality first (§2.3) |
| A scheduler, priorities, concurrency, or any clock in delivery | R51 | §2.8 fixes one order. Anything cleverer is a runtime design question |
| A third linked body in `universe.universe` | later | `bus.body` stays where it is, as the adversary's artifact |
| Renderer, pixels, drawing a hyperedge | 6+ | Unchanged |
| A new primitive, a line in `grandfather.txt`, a reblessed golden | never | Unchanged |
| Storing a membrane, typed or not | never | Rule 30. Typing ∂ does not change where it lives |

---

## 2. Decisions Assumed by This Plan

Eight decisions. §2.1 is the phase. §2.2 and §2.3 make the link a contract. The rest are what a runtime forces you to decide.

### 2.1 An artifact is a control only if damaging it changes the answer · **PROPOSED · yours · this is the phase**

> **`GateItem::control` becomes `fn(&Artifact) -> bool`, where `Artifact` carries the repo-relative path and the file's bytes. `run_gate_table` reads the file (`joinn-gate` still never opens one) and calls the control twice: once on the real bytes, then once on a damaged copy (the bytes with every byte after the first line removed; for a one-line file, the empty string). The item is admitted only if the two calls return different answers. Otherwise the whole run is refused, naming the item and saying "control does not read its artifact."**

For a well-formed control, the real artifact gives `false` (the opposed failure did not happen) and the damaged one gives `true`, because the artifact no longer says what the control needs it to say. A control that ignores its input gives the same answer both times and is caught before the item runs.

What this catches in today's tree:

| Item | What happens under §2.1 |
|---|---|
| gate 5 item 3 · `broken_chain.universe` | The control builds a `Complex` in code and ignores the bytes. **Refused.** |
| gate 5 item 2 · `calculator.body` | Adds a wire in memory, but does load the body; damage makes loading fail, so the answer flips. **Admitted.** It stays admitted only because it really does depend on the file |
| gate 5 item 8 · `inner_reason.txt` | Reads the file, but `far_side_refusal` ignores the inner refusal. Admitted by §2.1, and caught by P51-14's rewrite instead. **§2.1 is necessary but not sufficient**, and this plan says so rather than overclaiming |

*Why "damaged" and not "missing":* P5-01 already refuses missing files. The new question is whether the **content** matters. Truncating after the first line keeps any header (so a control that only checks the header still passes, correctly), and it destroys anything a body, universe or lock actually says.

**Cost to reverse:** controls go back to naming a path they don't read, and the pattern gets a sixth address.

### 2.2 ∂ is typed · **PROPOSED · yours**

> **`membrane(body, cells)` returns `BTreeSet<BoundaryPort>`, where `BoundaryPort { address: Address, direction: Direction, frame: FrameRef }` is taken from the instance's cell contract or primitive port table. Nothing about the membrane is stored anywhere, as before.**

`Address` is still the unit of identity. Two `BoundaryPort`s with the same address and different frames cannot occur, because a port has exactly one frame, and a test asserts that. `intent_set` becomes "the `In` members of ∂(body)" with no change to its result for any corpus body. `g5_membrane`'s by-value assertion is restated over addresses, so its golden does not move.

**Cost to reverse:** links go back to being unjudgeable (F35).

### 2.3 A link is admitted when its members agree · **PROPOSED · yours · moves R49 to PROPOSED**

> **For every link: each member marked `tail` is an `Out` port; each member marked `head` is an `In` port; unmarked members may be either; and every member's frame is equal to every other member's, exactly, including version. A violation is refused naming the link, the member, and what the member is next to what was required (rule 35).**

So R49's answer is that a link's contract is **its members' frame, plus the rule that flow goes out → in**. It has no laws and no witnesses of its own. This phase does not decide whether it *should* (R49 stays open for laws).

*Why exact equality rather than embeddings:* ℤ ↪ ℚ across a link would be the first time G3's embedding obligation does work between bodies. That is valuable, and it is exactly the kind of structure Phase 4's H¹ question is about. It belongs in R56, decided with Phase 4 in view.

**Cost to reverse:** `e0` in the Phase 5 universe ships as ℤ → Text.

### 2.4 A universe binds bodies by hash · **DECIDED** (Law 5)

> **When a universe is loaded against a set of bodies, each binding's declared hash must equal `hash(&body.coding)` of the body supplied for that alias. A mismatch is refused naming the alias, the declared hash (short form) and the supplied one. Loaders look bodies up by hash first; the alias is only a local name.**

This is Law 5, not a design choice. The Phase 5 tree drops it (F36).

### 2.5 The far side of a link gets a refusal with no free text · **PROPOSED · yours · G6**

> **`LinkRefusal { link: String, member: Address, body: String, kind: LinkRefusalKind }` is the only value that crosses a link when something goes wrong. `LinkRefusalKind` is a closed enum: `Refused`, `CapabilityNotHeld`, `NotDelivered`. It has no `String` field for a reason. The receiving body's own `Refusal` stays in that body's state, where a host `probe` can read it.**

The Phase 5 plan's §2.6 argued that *"a rule the type enforces never detected anything,"* and that argument was right for V15. G6 is different. It is a confidentiality property, and for confidentiality the goal is impossibility, not detection. The detection still exists at runtime: P51-14's check reads the far-side line a **host** actually printed, and its control is a **probe** that actually reaches the inner reason. What the type rules out is the Phase 5 shape, where a function *could* leak but happens not to.

`CapabilityNotHeld` is a partial answer to **R53**: a revoked capability is **the link's fact, not the near side's reason**. The far side learns *that* delivery stopped and not *why* the other side stopped it.

**Cost to reverse:** refusal locality goes back to depending on a function choosing not to read its input.

### 2.6 ∂∂ is vacuous on universes until Phase 4, and the plan says so · **DECIDED**

> **Every complex a `.universe` can express has blocks of dimension 0 and 1 only. On those, ∂∂ = 0 always holds. Touch-only is enforced by the well-formedness branch of `assemble` (an endpoint that is not a 0-block), and its teeth come from C₀ being derived from the body (Phase 5 §2.5), not from ∂∂ (Phase 5 §2.1). Gate 5 item 3 is renamed "The universe is well-formed." Its control becomes a universe that names a port that does not exist, which is refused as *no such port* and not as *interior*. `broken_chain.universe` is deleted. The `dim ≥ 2` closure branch stays covered by `joinn-assay`'s unit fixtures, and it becomes a gate item when Phase 4 derives fillings from laws.**

This changes no code path. It is on the decision list because Phase 4's plan will otherwise rebuild the Phase 5 argument, and it should start from the correct version: *touch-only is a membership test in C₀, and C₀ is what's derived.* The finding is `docs/Findings/boundary-depth.md` (P51-15).

### 2.7 Capability is named by the link it rides, and delivery is the attempt · **PROPOSED · yours**

> **A link's capability is named by the link's id. `grant(rt, universe, "e0", to: "units")` authorizes delivery on `e0` into `units`. The universe runtime checks it at every delivery. If it isn't held, the delivery does not happen, the head body does not fire, and the tail side sees `LinkRefusal { kind: CapabilityNotHeld }`. Only ordered links carry a capability. On an unordered link, delivery is always authorized.**

This replaces the Phase 5 shape, where the capability name (`"scale"`) was just a string checked against a set nothing else consulted. The check is now on the delivery path, so revoking has an effect a host can see: `units` stops firing.

*Why only ordered links:* Part II §11.1 says an ordered hyperedge *orders access: turn-taking, priority, delivery, passing a capability*. An unordered link is a relation, not a channel of authority. This keeps the Phase 5 rule (grant refuses on an unordered link) and makes it do something.

### 2.8 Delivery order is canonical, never arrival time · **PROPOSED · yours · R51, first half**

> **The universe runtime alternates two phases until nothing changes. (1) Run each body to quiescence, in canonical alias order. (2) For each link in canonical link-id order, for each tail member that emitted a value in phase 1, deliver that value to each head member: in declared sequence for an ordered link, in sorted order for an unordered one. A body receives at most one delivery per port per pass. The step budget is the sum of the bodies' budgets and is shared across passes.**

This extends rule 4 (*delivery order is a function of the grant list, never of arrival time*) to the universe. It does **not** answer R51's second half, which is how an ordered hyperedge orders runtime access when two tails compete. With one tail per link in this phase, that question can't come up. P51-16's adversary is chosen to make it come up.

---

## 3. Architecture

### 3.1 Crates

No crates are added. One dependency is added:

```
joinn-link  gains  joinn-prim (primitive port table, for ∂ over prim instances)
                   joinn-live (BodyState, for UniverseState)
```

Both are already to its left, so the dependency rule holds. `joinn-link` does no IO, and `vocab` already enforces that. `joinn-host` and `joinn-test-host` gain the universe entry points. `joinn-cli` gains `run universe`.

### 3.2 Types added or changed

```rust
// joinn-gate
pub struct Artifact<'a> { pub path: &'static str, pub bytes: &'a [u8] }
pub struct GateItem {
    pub name: &'static str,
    pub check: fn() -> bool,
    pub control: fn(&Artifact) -> bool,        // was fn() -> bool
    pub control_artifact: &'static str,
}

// joinn-link
pub enum Direction { In, Out }                 // reuse joinn-dna's if it exists; do not duplicate
pub struct BoundaryPort { pub address: Address, pub direction: Direction, pub frame: FrameRef }
pub fn membrane(body: &Body, cells: &BTreeMap<Hash, Cell>) -> Verdict<BTreeSet<BoundaryPort>>;

pub fn bind_bodies(u: &Universe, supplied: &BTreeMap<Hash, (Body, BTreeMap<Hash, Cell>)>)
    -> Verdict<BTreeMap<String, (Body, BTreeMap<Hash, Cell>)>>;   // alias → body, by hash
pub fn check_link_types(u: &Universe, bound: &BTreeMap<String, (Body, Cells)>) -> Verdict<()>;

pub struct LinkRefusal { pub link: String, pub body: String, pub member: Address, pub kind: LinkRefusalKind }
pub enum LinkRefusalKind { Refused, CapabilityNotHeld, NotDelivered }

pub struct UniverseState { /* BodyState per alias, LinkRuntime, pass counter, far-side log */ }
impl UniverseState {
    pub fn new(u: &Universe, bound: BTreeMap<String, (Body, Cells)>, natives: NativeRegistry) -> Verdict<Self>;
    pub fn inject(&mut self, body: &str, addr: &Address, v: Value, epoch: u64) -> Verdict<()>;
    pub fn run(&mut self) -> Verdict<Vec<UniverseReport>>;   // UniverseReport names body + fire, or a LinkRefusal
    pub fn body(&self, alias: &str) -> Option<&BodyState>;  // for describe / probe; read-only
}
```

`far_side_refusal` is **deleted**. There is no function that turns a `Refusal` into a `LinkRefusal`. The runtime builds a `LinkRefusal` from the link and the member, and the body's `Refusal` stays in its `BodyState`.

### 3.3 The second body, specified · **PROPOSED · yours**

```
body {
  codex 1
  genome {
    cell:<mul.cell hash> as scale
  }
  grants {
    stdin: scale
  }
  wires { }
  budget { steps 100000 }
  lineage none
}
---
regulatory {
  prompts { scale "factor: " }
  present { scale "{0} ft = {2} in" }
  names   { scale "Scale" }
  labels  { scale "Feet to inches" }
}
```

`mul.cell` (phase21) has the contract `0 in ℤ`, `1 in ℤ`, `2 out ℤ`, `join refuse`. So:

- `∂(units) = { scale@0 In ℤ, scale@1 In ℤ, scale@2 Out ℤ }`.
- **Standalone**, the host prompts for both in-ports: `7` and `12` print `7 ft = 84 in`.
- **Linked**, `scale@0` is fed by `e0`, so the host prompts only for `scale@1`.
- A second value into `scale@0` before `scale@1` arrives triggers `join refuse` inside `units`, which is the real refusal P51-14 needs.

A first-grader could build this, it is not the calculator's parser under another name, and every Phase 5 check involving `units` now tests something that could fail for other reasons (F34).

The Phase 5 `units.body` (a `cli_input` renamed `scale`) moves to `corpus/phase5/controls/echo.body`, unchanged byte for byte, so **its hash stays in `hashes.txt` under its new path**. It becomes the frame-mismatch control's body: a Text in-port that a ℤ link must be refused against.

### 3.4 The universe, revised

```
universe {
  codex 1
  bodies {
    body:b55fba1eff65… as calc
    body:<new units hash> as units
  }
  links {
    link e0 order ordered {
      calc.sum@2 tail
      units.scale@0 head
    }
  }
  lenses {
    lens deployment { galaxy app { system local { calc units } } }
    lens function   { galaxy app { system calculation { calc }  system measurement { units } } }
  }
}
---
regulatory {
  names  { e0 "result to inches" calculation "Calculation" }
  labels { e0 "the sum, in feet, converted to inches" }
}
```

`g0` is deleted: it linked two in-ports and existed only so `grant` had an ordered link to name. `e0` becomes ordered, so it can carry the capability (§2.7). `ordered.universe` is rewritten to the same typed shape, and `adversary.universe` is left alone, as a record of the Phase 5 attempt. Every universe hash moves, and P51-17 records each one. **No Phase 0–3 coding hash moves.**

### 3.5 The linked transcript

`corpus/transcripts/universe.txt`, input `two`, `2`, `3`, `12`:

```
a: two
refused at membrane: "two" is not in ℤ
a: 2
b: 3
2 + 3 = 5
factor: 12
5 ft = 60 in
```

**The first five lines are byte-identical to `corpus/transcripts/calculator.txt`**, indentation included. This is asserted by comparing byte ranges of the two files, not by retyping the lines. `calculator.txt` itself does not change, and `joinn run calculator` still prints it. The runtime asks for a host value only at in-ports on ∂ that no link head feeds. It asks in canonical alias order, then in the body's declaration order, so `factor:` comes after the calculator is done.

---

## 4. The Commit Plan

Nineteen commits. **Done-when** is a command, and the command must be able to refuse.

### Witness and harness

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P51-00** | **The tree as Phase 5 left it — typed by AJ** | AJ runs `gate all`, `power`, `agree`, `corpus verify`, `modules`, `vocab`, `perf` on the Windows machine and types `docs/Findings/phase-5-run.md` by hand, quoting each command's last line and gate 5's ten rows. **Cursor does not write this file** | The file exists, its mtime is later than every file under `crates/` and `xtask/`, and it quotes `phase 5:` exactly as printed. If gate 5 printed anything other than 10/10, that is recorded and the commit is red, which is its purpose |
| **P51-01** | **Controls receive their artifact** | `Artifact`, and `GateItem::control: fn(&Artifact) -> bool`; `run_gate_table` reads the bytes and passes them in; every control in gates 1–5 takes `&Artifact` (unused where it truly is unused, which P51-02 will catch) | `cargo test --workspace` and `cargo xtask gate all` behave exactly as before. `joinn-gate` still contains no `std::fs`, asserted by `vocab` |
| **P51-02** | **Damaging the artifact must flip the control (§2.1)** | `run_gate_table` runs each control on the real bytes and on the damaged bytes, and refuses the run naming any item whose answer does not change. Fixture `xtask/gate_fixtures/insensitive.rs`: an item whose control ignores its bytes | The fixture **fails the run naming the item** and the words "does not read its artifact". **Gate 5 item 3 fails too**, and its fix is P51-15; until then `gate all` is red and the commit says so. List every other item that fails in the commit message. No other failure is fixed in this commit |
| **P51-03** | **No shared check or control across all gates** | A test over every gate table, comparing function pointers, that no two items anywhere share a `check` or a `control`. `g5_artifacts` / `g5_artifacts_control` deleted (gate 5 item 9 removed; rule 24 is now a property of `run_gate_table`, not an item) | The test **fails** when `g5_artifacts` is restored, shown then reverted. Gate 5 prints nine rows |
| **P51-04** | **The lock records this run** | `gate_all` writes the lock, reads it back, parses it with `parse_lock_scores` extended to return `(phase, n, total)` rows, and requires those rows to equal the scores it just returned. Mismatch fails `gate all` naming the phase. `g5_path` / `g5_path_control` deleted (gate 5 item 10 removed) | `gate all` **fails naming the phase** when `write_lock` is temporarily made to write one score off by one, shown then reverted. No file under `xtask/src` contains the text `"phase 5"` except `gate_all.rs`'s label array, asserted by the existing literal-score scan extended to phase labels. Gate 5 prints eight rows |

### An honest, typed boundary

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P51-05** | **∂ covers primitives and refuses a missing cell** | `membrane()` reads primitive port positions from `joinn-prim`'s table; a genome entry whose cell is not in `cells` is refused naming the instance and the short hash | `membrane(int_mul_ref)` is non-empty and contains a port of a primitive instance, asserted by value; `membrane` of a body with one cell missing from `cells` is **refused naming the instance**. `membrane(calculator)` unchanged by value |
| **P51-06** | **∂ is typed (§2.2)** | `BoundaryPort`; `intent_set` filters `Direction::In` | `membrane(units_echo)` (the old body) is `{scale@0 In Text, scale@1 Out ℤ}` by value; `intent_set` is unchanged for every corpus body, asserted by value over the corpus; a test asserts no two `BoundaryPort`s in any corpus membrane share an address |
| **P51-07** | **R50, measured properly** | `perf` probe 7 splits into load+parse and `membrane()`, over **every** corpus body with every cell it names loaded (phase0, phase2, phase21, phase22) | `perf` prints both numbers and the count of bodies whose membrane was **refused**, which must be zero. AJ updates `r50-membrane-cost.md` by hand; Cursor does not |

### Binding and typing the link

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P51-08** | **Bodies bound by hash (§2.4)** | `bind_bodies`; `load_phase5_bodies` replaced by a loader that indexes supplied bodies by coding hash; `corpus/phase5/controls/wrong_hash.universe` binds the units hash under the alias `calc` | `wrong_hash.universe` is **refused naming `calc`** and both short hashes; every other phase5 universe binds; no function in `xtask` or `joinn-link` looks a body up by alias before binding, asserted by a scan for `bodies.get(&binding.alias)` |
| **P51-09** | **Direction (§2.3)** | `check_link_types`, direction half; `corpus/phase5/controls/wrong_direction.universe`: a link whose tail is `calc.cli_a@0` | Refused **naming `calc.cli_a@0`, "tail", "In" and "Out"**; `adversary.universe` is refused naming `units.scale@1` as a head that is an out-port. That is expected and recorded in P51-16 |
| **P51-10** | **Frame (§2.3)** | `check_link_types`, frame half; `corpus/phase5/controls/frame_mismatch.universe` binds `echo.body` and links `calc.sum@2 tail` to `echo.scale@0 head` | Refused **naming `echo.scale@0`, `Text 1` and `ℤ 1`**; the Phase 5 `universe.universe`, applied to `echo.body` by hash, is refused the same way. This is F35 reproduced as a refusal |

### The second body and the runtime

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P51-11** | **The second body, for real (§3.3)** | Old `units.body` moved byte-for-byte to `controls/echo.body`; new `units.body`; hash computed by hand and entered in `hashes.txt` | `gate all` admits it; the machine's hash equals AJ's hand-computed one; `joinn run units` with `7`, `12` prints `7 ft = 84 in`, and `joinn-test-host` with the same events captures `84` on `scale@2`; `echo.body`'s hash is unchanged in `hashes.txt` under its new path |
| **P51-12** | **The universe, revised (§3.4)** | `universe.universe` and `ordered.universe` rewritten; `g0` gone; `transits`, `two_systems`, `wrong_container` re-bound to the new units hash | Every phase5 universe binds, types and assembles, or is refused for exactly the reason its file is named for, asserted by a table test from file name to the refusal's key words |
| **P51-13** | **`UniverseState` (§2.8)** | The runtime in `joinn-link`: two-phase passes, canonical order, shared budget, `UniverseReport`s. No IO | Test: inject `2` at `calc.cli_a@0`, `3` at `calc.cli_b@0`, `12` at `units.scale@1`, run. **`units` fires exactly once and its `scale@2` holds 60**, read through `describe` on `body("units")` and not through any field of `UniverseState`. Control in the same test: the same events with `e0` removed from the universe, where `units` **does not fire** |
| **P51-14** | **Both hosts run a universe** | `joinn-test-host::run_universe`; `joinn run universe` in `joinn-cli`; `corpus/transcripts/universe.txt` | `echo "two\n2\n3\n12" \| joinn run universe` prints `universe.txt` **byte-identically**; bytes 0..len(`calculator.txt`) of `universe.txt` equal `calculator.txt`, asserted by comparing byte ranges; the test host's capture for `units` has `60` on `scale@2`; `joinn run calculator` still matches `calculator.txt` |
| **P51-15** | **Capability on delivery (§2.7)** | `grant`/`revoke` keyed by link id; the runtime checks on each delivery; `LinkRefusalKind::CapabilityNotHeld` | One test, in order: grant `e0` to `units`, run, **`units` fires** (control). Revoke, inject a fresh pair into `calc` and `12` into `units`, run: **`units` does not fire** (fire count unchanged, read via `describe`), and the report carries `LinkRefusal { link: "e0", kind: CapabilityNotHeld }`. Gate 5 item 7 rewritten to this |
| **P51-16** | **G6 with a real refusal (§2.5)** | `LinkRefusal` with no text field; `far_side_refusal` deleted; a trybuild compile-fail fixture that tries to put a `String` reason into a `LinkRefusal`; `inner_reason.txt` rewritten **by AJ** from what a probe printed | Scenario: run `calc` twice (two sums) before `12` arrives, so the second delivery into `scale@0` is refused by `join refuse` inside `units`. The CLI's far-side line and the test host's far-side capture **do not contain** `inner_reason.txt`'s text; `probe` on `units` **does contain** it, and that is the control, under §2.1's damage rule. The trybuild fixture fails to compile. Gate 5 item 8 rewritten to this |

### Honesty and the adversary

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P51-17** | **Item 3 honest (§2.6)** | Gate 5 item 3 renamed "The universe is well-formed"; `broken_chain.universe` deleted; `controls/no_such_port.universe` names `calc.sum@9`; `assemble_universe`'s refusal distinguishes *no such port* from *interior*; `docs/Findings/boundary-depth.md` **typed by AJ** from §2.6 | `no_such_port.universe` is refused naming `calc.sum@9` and "no such port", and the refusal does **not** contain "interior"; `transits.universe` still names "interior" and the wire. The item passes §2.1's damage rule |
| **P51-18** | **The adversary, specified by AJ** | AJ writes, before this commit starts, a one-paragraph spec for a third body that wants something neither container obviously offers. **PROPOSED:** *a `lookup` body that asks `units` "what factor did you last use?" and must pair each answer with the question that caused it (request and response with correlation)*. Cursor attempts it with the existing wire + hyperedge and reports; AJ writes `law-4-adversary.md` | The finding uses **fired** only if the body cannot be expressed. The Phase 5 entry is corrected to *"held — the attempt did not test Law 4"* and says why (F41). If correlation turns out to need an order across two links, R51's second half opens with the example attached |
| **P51-19** | **Re-freeze, docs, gate 5.1, lock** | New goldens and `phase-5.1-hashes.md`; `README.md` and `Guides/03` updated; gate 5 at eight items, rewritten per P51-13…P51-17; **gate 5.1** (§6); lock | `cargo xtask gate all` from a clean checkout, offline, prints phases 0, 1, 2, 2.1, 2.2, 3, 5 and 5.1, and every item passes the damage rule; `corpus verify` matches every golden; `phase-5.1-hashes.md` confirms **no Phase 0–3 coding hash moved**; the lock postcondition (P51-04) holds |

**Ordering notes.**

- **P51-00 comes first, and AJ does it.** If the witness run is red, the rest of the plan waits.
- **P51-01 and P51-02 are not cuttable.** Every later item's control is judged by them.
- P51-02 deliberately leaves `gate all` red until P51-17. That is intended: the commit message lists what failed, and nothing is fixed early to make it green. **If that is too uncomfortable, move P51-17 directly after P51-02.** Do not weaken the damage rule instead.
- P51-05 and P51-06 must land before P51-09 and P51-10, which need a typed ∂.
- P51-11 must land before P51-12. P51-13 must land before P51-14, P51-15 and P51-16.
- P51-18 runs before the re-freeze, so its finding can still change something.
- **If something has to be cut, cut P51-07 or P51-18. Never P51-00 through P51-04, and never P51-13.**

---

## 5. Test Strategy

### 5.1 New invariants

Continuing from V84.

| # | Invariant | Test | Commit |
|---|---|---|---|
| **V85** | Damaging a control's artifact changes the control's answer | `run_gate_table`, `insensitive.rs` fixture | P51-02 |
| **V86** | No two gate items anywhere share a check or a control | function-pointer test over all tables | P51-03 |
| **V87** | The lock `gate all` wrote equals the scores `gate all` returned | read-back comparison | P51-04 |
| **V88** | ∂ covers every instance, primitive or cell, and refuses a missing cell | `int_mul_ref`, missing-cell body | P51-05 |
| **V89** | ∂ is typed; one address has one direction and one frame | by value over the corpus | P51-06 |
| **V90** | A universe binds bodies by coding hash | `wrong_hash.universe` | P51-08 |
| **V91** | Tails are out-ports, heads are in-ports | `wrong_direction.universe` | P51-09 |
| **V92** | Every member of a link has the same frame | `frame_mismatch.universe` | P51-10 |
| **V93** | A value emitted at a tail arrives at the head, and the head fires | `UniverseState`, with the `e0`-removed control | P51-13 |
| **V94** | The calculator's transcript is a prefix of the universe's, byte for byte | byte-range comparison | P51-14 |
| **V95** | A revoked link delivers nothing; the head does not fire | before/after, by fire count | P51-15 |
| **V96** | A far-side refusal carries no text the refusing body wrote | CLI line, test-host capture, probe control, trybuild | P51-16 |
| **V97** | Touch-only distinguishes *no such port* from *interior* | two controls | P51-17 |

Carried forward and re-run on every commit: V18–V20, V24-embryo, FO1–FO10, the canonical-text properties, the 1 000-append hash-stability test, V33–V84, and Phase 1's four demos. **V16 is now tested three ways:** interior (`transits`), nonexistent (`no_such_port`), and wrongly typed (`wrong_direction`, `frame_mismatch`).

### 5.2 What a runtime test must never do

It must never assert on `UniverseState`'s own fields, on `LinkRuntime::held`, or on anything else the runtime uses to decide. It asserts on what a body **did**: `describe` output, fire counts, port values, the host's printed lines. A test that passes by reading the runtime's bookkeeping is checking the runtime against its own notes (§0).

### 5.3 What Phase 5.1 deliberately does not test

- **Competing tails.** Two tails on one ordered link racing to a head is R51's second half, and §2.8 cannot produce it.
- **Embeddings across links.** R56.
- **Whether ∂∂ is worth having on universes.** §2.6 says it isn't yet. R55 is where that could change.
- **Scale.** Two bodies, one link.

---

## 6. Exit Gate 5.1, As a Checklist

`cargo xtask gate 5.1`, run from a clean checkout, offline. Every item's control is a file that §2.1's damage rule has verified the control actually reads.

- [ ] **1 · Something crosses.** `2`, `3` and `12` go in, and `units.scale@2` holds `60`, read through `describe`. *Control:* the same universe with `e0` removed, where `units` does not fire (`corpus/phase51/controls/unlinked.universe`).
- [ ] **2 · Two hosts, one universe.** `joinn run universe` prints `universe.txt` byte-identically, and `joinn-test-host` captures `60`. *Control:* `universe.txt`'s first bytes equal `calculator.txt`, and a `universe.txt` edited to change the calculator's lines fails the prefix check.
- [ ] **3 · Bodies are bound by hash.** *Control:* `wrong_hash.universe`, refused naming `calc`.
- [ ] **4 · Links are typed.** *Controls:* `wrong_direction.universe` and `frame_mismatch.universe`, each refused naming the member and both halves of the mismatch.
- [ ] **5 · ∂ is total.** Every corpus body's membrane is computed, and a missing cell is refused. *Control:* `corpus/phase51/controls/missing_cell.body`.
- [ ] **6 · Revocation stops delivery.** *Control:* the pre-revoke run, where `units` fires, from `universe.universe`.
- [ ] **7 · A refusal stays home.** *Control:* `inner_reason.txt`, reached by `probe` on `units` and absent from both hosts' far-side output.
- [ ] **8 · Every control reads its artifact.** *Control:* `xtask/gate_fixtures/insensitive.rs` fails the run naming the item.
- [ ] **9 · The lock is this run.** *Control:* a `write_lock` that is off by one fails `gate all` naming the phase.

Gate 5 itself is rewritten in place to eight items (the old 9 and 10 are gone; items 1, 3, 7 and 8 are rewritten). The lock records `phase 5: 8/8` and `phase 5.1: 9/9`, each score returned by its gate.

### 6.1 The conditions for opening Phase 4

Phase 4 opens when all of the following hold. Its plan's P4-00 cites this list:

1. Gate 5.1 passes, and **AJ has typed the run** (`phase-5.1-run.md`).
2. At least one value has crossed at least one typed link, under both hosts, with a transcript golden.
3. `boundary-depth.md` exists, so Phase 4 starts from *touch-only is membership in a derived C₀* and not from *∂∂ refuses transits*.
4. The adversary has been run with an AJ-specified body and graded with `fired` used as the plan defines it.
5. R55 and R56 are written up in the backlog, each with the example that motivated it. They are the two places where Phase 4's homology and Phase 5's links meet: whether fan-out conserves, and whether an embedding may bridge a membrane. **The H¹ candidate Phase 4 is looking for is most likely to appear at R56**: two bodies whose laws each hold on their own frame, joined by an embedding whose round-trip holds on one side and not the other.

---

## 7. Risks Watched During This Phase

| Risk | Instrument | What to do when it fires |
|---|---|---|
| **The new construct is the runtime, and its tests read its own state** | §5.2; every done-when written in terms of `describe` or fire counts | Rewrite the test to observe a body. If it can't be observed from a body, the feature is bookkeeping and does not ship |
| **§2.1's damage rule is gamed** (a control that checks "the file is longer than one line") | Review. The rule makes the gaming visible in the control's source | Add a second damage mode (byte-reversed) only if gaming shows up. Don't pre-build it |
| **Typing ∂ moves a golden** | `corpus verify`; P51-06's by-value `intent_set` check | Stop. No Phase 0–3 hash may move. If one does, a coding type leaked into the membrane's hash path, and that is a bug |
| **`join refuse` doesn't fire the way §3.3 assumes** | P51-16's scenario | Record the actual behavior in a finding and choose a different real refusal (a Text value that doesn't parse, delivered over a Text link). Do not fake a refusal |
| **The builder writes the AJ-only findings anyway** | Each AJ-only file's mtime compared with the code it judges (P51-00's done-when) | Delete the file and leave the commit red. Rule 38 |
| **Pace** | One session per commit, stop at the done-when | If three commits land in one session, the review for 5.1 starts from that fact |
| **Carried · the two hosts share a bug** | Unchanged: both call `describe` | The universe transcript is a golden, and the test-host capture is checked against it by value, not against the CLI |

---

## 8. What This Plan Refuses To Do

| Refused | Because |
|---|---|
| Admit a control that doesn't read its artifact | §2.1 |
| Make ∂∂ do work on universes before Phase 4 has 2-blocks | §2.6. Claiming it does is how F42 happened |
| Let a link join frames that differ, even by an embedding | §2.3, R56 |
| Look a body up by alias before checking its hash | §2.4, Law 5 |
| Let a far-side refusal carry text | §2.5 |
| Test a runtime by reading its own bookkeeping | §0, §5.2 |
| Let Cursor write a finding marked *typed by AJ*, or choose the adversary | review F41; rule 38 |
| Delete or weaken the damage rule to get `gate all` green after P51-02 | §4's ordering note |
| Rebless a golden, add a primitive, add a line to `grandfather.txt` | unchanged |

---

## 9. Open Items This Plan Creates

Continuing from R54.

| ID | Topic | Question |
|---|---|---|
| **R55** | The augmented complex | With ε : C₀ → ℤ and ε∘∂ = 0 required on 1-blocks, a link's tails must balance its heads. `e0` passes; the Phase 5 `bus` link (one tail, two heads) is refused. Is fan-out a single 1-block that should conserve, a hub (one 0-block for the link, plus one 1-block per member), or a genuine k-block? Part II §11.1 draws the hub, and the roadmap says *hyperedges → k-blocks*. They disagree, and the assay will have to pick one |
| **R56** | Embeddings across a link | Can a link join ℤ and ℚ through the G3 embedding, with the gate checking the round-trip on witnesses from both bodies? If so, the link has laws, which answers R49's open half. It is also the most likely place for Phase 4's H¹ candidate |
| **R57** | A universe that never finishes | If `12` never arrives, `units` waits with `scale@0` filled and the universe goes quiet in a half-fed state. Is that a refusal, a valid state, or something the host must describe? It bears on R46 (*does a host owe a refusal a description?*) and on Phase 6's drawing of pending ports |
| **R58** | Correlation | From P51-18. If request and response need the reply paired with its request, is that an ordered link with two tails (R51), a value that carries its own correlation token (which is just a frame), or a third container method (which would mean Law 4 fired)? |

**Touchpoints.** **R49** moves to PROPOSED (§2.3) for its frame half; its law half becomes R56. **R50** is re-measured at P51-07. **R51** is half answered by §2.8, and the other half is R58. **R53** is partly answered by §2.5: revocation is the link's fact. **R7, R8, R11, R12, R14, R35, R39, R41, R43–R48** are unchanged.

---

## Appendix A · Amendments to `AGENTS.md`

Rewrite rules 24 and 30 as below, and append rules 36–40. Leave everything else as Phase 5 left it. Update the header paragraph to name Phase 5.1 and its one idea.

```markdown
24. A CONTROL IS AN ARTIFACT, NOT A PREDICATE — IT IS A PATH, AND IT IS READ.
    `GateItem` carries `control_artifact`; `run_gate_table` resolves it, reads
    its bytes, and passes them to `control(&Artifact)`. The control is run on the
    real bytes and on a damaged copy and must answer differently, or the run is
    refused naming the item. joinn-gate carries the bytes and never opens a file.
30. A MEMBRANE IS ∂(BODY) AND IS STORED NOWHERE. It is every port of every
    instance in the genome — cell or primitive — that no internal wire consumes,
    each with its direction and frame, computed on demand. A genome entry whose
    cell is not supplied is refused, never skipped. Nothing in a .body or
    .universe file declares it.
36. A LINK IS ADMITTED WHEN ITS MEMBERS AGREE. Tails are out-ports, heads are
    in-ports, and every member's frame is equal. A universe binds bodies by
    coding hash, never by alias.
37. A RUNTIME IS TESTED BY WHAT A BODY DID. Assert on descriptions, fire counts,
    port values and host output, never on the runtime's own bookkeeping.
38. SOME FINDINGS ARE TYPED BY AJ. A commit that marks a file "typed by AJ"
    (witness runs, the adversary's spec and verdict, perf conclusions) is not
    written by the agent. If such a file appears in the agent's commit, the
    commit is red.
39. NOTHING TEXTUAL CROSSES A MEMBRANE ON REFUSAL. The far side of a link
    receives a `LinkRefusal` — link, body, member, kind — and no reason string.
    The refusing body's reason is reachable by `probe` on that body only.
40. ∂∂ = 0 IS NOT CLAIMED WHERE IT CANNOT FAIL. A universe's complex has
    dimension ≤ 1 until fillings exist; touch-only is enforced by C₀ being
    derived. No check, doc comment or finding may attribute a universe refusal
    to ∂∂ ≠ 0 before Phase 4.
```

## Appendix B · Directory changes

```
docs\Findings\
    phase-5-review.md            ← F33–F43
    phase-5-run.md               ← new, P51-00, typed by AJ
    boundary-depth.md            ← new, P51-17, typed by AJ
    law-4-adversary.md           ← corrected and extended, P51-18, typed by AJ
    r50-membrane-cost.md         ← updated, P51-07, typed by AJ
    phase-5.1-hashes.md          ← new, P51-19
    phase-5.1-run.md             ← new, after P51-19, typed by AJ
joinn\corpus\
    phase5\units.body            ← rewritten (mul, feet → inches)
    phase5\universe.universe     ← e0 ordered and typed; g0 removed
    phase5\controls\echo.body    ← the old units.body, byte-identical
    phase5\controls\wrong_hash.universe  wrong_direction.universe
    phase5\controls\frame_mismatch.universe  no_such_port.universe
    phase5\controls\broken_chain.universe  ← deleted
    phase51\controls\unlinked.universe  missing_cell.body
    transcripts\universe.txt     ← new; calculator.txt is its prefix
joinn\xtask\gate_fixtures\insensitive.rs  ← new
joinn\crates\joinn-link\src\far_side_refusal.rs  ← deleted
joinn\crates\joinn-link\src\universe_state.rs (+ universe_state\)  ← new
```

---

*JoInn Phase 5.1 Implementation Plan (Draft 0.1). Closes F33–F43 of `docs/Findings/phase-5-review.md`. It is a correction phase: no crates are added and no primitive is touched, and after it a value crosses a typed hyperedge under two hosts. Everything marked PROPOSED · yours is a recommendation; Cursor implements the decisions AJ settles on.*
