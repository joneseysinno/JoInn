# JoInn Phase 2.1 Implementation Plan

**The floor writes the floor · a working plan for Cursor**

*The correction phase: F1–F4 of the Phase 2 review turned into decisions, commits, tests and gates*

Author: AJ · Draft 0.1 · September 18, 2026

> Status tags follow Parts I–IV: **DECIDED**, **PROPOSED**, **OPEN**. Anything marked **PROPOSED · yours** is a recommendation made while writing this plan and is yours to overrule; Cursor implements whatever lands there, not whatever is written here.

> **What this is.** Phase 2 shipped a runtime that runs, a floor whose oppositions are checked, and a gate that refuses twelve mutants. It also shipped four checks that do not check. `docs/Findings/phase-2-review.md` names them F1–F4. This plan closes all four. **The target is that a reference allele is a body, running on the live engine, and that a machine can refuse one that isn't.**

> **Why this is a phase and not a patch.** F1 is not a bug. Phase 2's floor could not express its own reference alleles, so they were written in Rust, and the instrument that would have said so was made vacuous. Fixing that means the floor is restated as a rule and gains the pair it was missing, the engine learns to run DNA, the body form learns to name the floor, and the agreement harness is rebuilt around what a reference can actually compute. That is a phase.

> **Where Phase 2 left off.** Six crates, green. `corpus/hashes.txt` holds nine Phase 0 cell goldens and five Phase 2 body/cell goldens, all matching. `gates.lock` reads `phase 2: 8/8`, which is **wrong** and P21-00 corrects it.

---

## 0. How to Drive Cursor With This Plan

| | |
|---|---|
| **Where this file lives** | `D:\JoInn\docs\Plans\JoInn Phase 2.1 Implementation Plan.md`, beside the Phase 2 plan |
| **Where the code lives** | `D:\JoInn\joinn\` — the existing repo, extended |
| **Standing rules** | Appendix A. It **replaces** the current `AGENTS.md` and `.cursor/rules/joinn.mdc` |
| **Unit of work** | one numbered commit from §4. One Cursor session per commit |
| **How a commit ends** | its **done-when** line is checkable by a command, and that command reports a refusal, a disagreement or a byte-identical replay — never a number the command also chose |
| **What Cursor may decide** | module layout inside a crate, function bodies, test names, error strings, the internal representation of the activation stack |
| **What Cursor may not decide** | anything in §2, the twelfth primitive's shape (§2.1), the constructor-order rule (§2.2), the activation model (§2.4), crate boundaries, or whether a check is worth having teeth |

The prompt stays short:

> Implement commit **P21-07** from `docs/Plans/JoInn Phase 2.1 Implementation Plan.md`. Follow the rules in `AGENTS.md`. Stop when the done-when command passes and report what it printed.

**The Phase 2 failure mode, named precisely, because this phase exists because of it.** Phase 2's commits all ended in a command that printed a number. Four of those numbers were chosen by the same code that printed them: `power`'s 20/20 contained a tautology, R31's count was a `const`, Risk #3's verdict was template text, and `agree`'s sample range made its own blind spot. None of that is dishonest code — it is code written to satisfy a done-when, by an agent that is very good at satisfying done-whens. **The correction is not more vigilance. It is that every done-when in this plan is a thing that can refuse.**

---

## 1. Scope Fence

### In scope

F1 through F4 of `docs/Findings/phase-2-review.md`:

- **F1** — reference alleles become bodies that run on the live engine. `AlleleBody::Dna` fires.
- **F2** — V33 becomes a structural check with teeth; mutant 14 can fail.
- **F3** — `agree` draws from the frame's generators, and a bounded-drive rule makes that affordable.
- **F4** — turn admission moves into the gate and stops being hard-coded to `Sum`'s shape.

Plus the smaller findings F5–F8, which are cheap and are folded into the commits they touch.

### Out of scope — Cursor must refuse these even when they look small

| Not now | Phase | Why it is tempting |
|---|---|---|
| The host protocol, `present`/`probe` as a vocabulary | 3 | P21-16 gives the runner real stdin. Real IO looks like a host. It is not. §1.1 of the Phase 2 plan still governs, and `joinn-run` is still deleted at Phase 3 |
| A second body, hyperedges, lenses, `grant` across a membrane | 5 | Nested activations (§2.4) will make this look reachable. A nested activation is **not** a second body — see §2.5 |
| The assay layer, ∂, declarations | 4 | `declarations` stays empty and stays a refusal |
| Another primitive, beyond `build`/`case` and `resolve` | never, without an amendment carrying a refuted reduction | §2.1 amends the floor **once** in this phase. A second amendment means the first was wrong. And a candidate arrives with the reference body that failed to compute it, or it does not arrive |
| An unopposed primitive | never | §2.1. A member with no opposite makes the count odd, and the count is checked |
| Optimizing the reference alleles | 10 | A reference allele is slow by construction. That is what the sealed path is for |
| A grammar-level `case` block in the body form | maybe Phase 6 | Option C of the eliminator decision. It stays available as sugar over the primitive; it does not arrive alongside it (§2.1) |
| Anything wgpu, winit, or visual | 6+ | — |

---

## 2. Decisions Assumed by This Plan

Eight calls. Three answer the review's open questions; five were found while writing this plan, and each says what it costs to reverse.

### 2.1 The floor rule, and `build`/`case` as the pair it admits · **DECIDED, AJ, Sept 18**

**First the rule, because it decides the membership and the wording both.** The floor is not sized. It is tested:

> **A primitive is admitted only if it is irreducible and opposed.** Irreducible: no body over the rest of the floor computes it. Opposed: it names an opposite that is also in the floor, and that opposite names it back.

The floor's size is then **even by arithmetic**, and an odd count is a proof that an opposition is missing — a lint, not an aesthetic. And irreducibility is mechanically refutable in the direction that matters: **to petition for a primitive, first try to write it as a reference body over the existing floor; if `agree` succeeds, the petition is refused and you have just written the cell that replaces it.** That is the agreement harness pointed at the floor, and it answers R33.

The full statement, the eight pairs and the amendment record live in `docs/Findings/the-floor.md`, which replaces `minimal-primitives.md`. This section states only what Phase 2.1 must implement.

The floor could not be written with the floor. ℤ's signature is four constructors and an equality; nothing takes a value apart, so `add`'s recursion cannot ask which shape `b` has, and `eq(b, zero)` distinguishes one value out of infinitely many. §2.1 of the Phase 2 plan saw this and answered it with `Frame::case` — a **Rust method**. Rust can call it. A body cannot, because a body names cells and wires ports. That is F1 in one sentence.

**`build`/`case` joins the matter register, and `zero`/`succ`/`pred` leave it:**

```
build : (frame, tag, parts) → Value      the frame's tag-th declared constructor
case  : Value → pair(tag, parts)         tag 0 = generator; k ≥ 1 = k-th constructor
```

`zero`, `succ` and `pred` are `build(ℤ,0,[])`, `build(ℤ,1,[x])` and `build(ℤ,2,[x])`. A thing constructible from `build` fails the irreducibility clause, so they are **not floor primitives** — they are ℤ's first three constructors, they stay in ℤ's signature, and they stay nameable in a law as frame vocabulary under §2.3 of the Phase 0–1 plan. The floor stopped naming them; the frame did not. That is what makes the floor frame-parametric: a floor with `succ` in it privileges ℤ and has nothing to say about `Text` or `ℚ`.

Two more corrections fall out of the pairing half of the rule, and both are code changes:

- **`eq` and `choose` oppose each other**, not themselves. The register today declares `Opposition::Inverse("eq")` for `eq` and `Inverse("choose")` for `choose` — two self-oppositions, which is a pair that is not a pair. A distinction *made* is opposed by a distinction *spent*.
- **`hash` is opposed by `resolve`**, partial. `hash` is not one-way; its opposite is a lookup that refuses when the store is empty. R32's hole restated: `hash` is total, `resolve` is partial, **and the gap between them is the store**. `Opposition::OneWay` leaves the codebase, and with it `check_oppositions`' rule that "hash is the only one-way primitive."

The shape of `case` matters as much as the decision. Returning a tag-and-list would have forced a new value kind into a register whose members are all value-to-value. Returning `pair(tag, parts)` uses two primitives that were already frozen, so `case` fits the register it joins, and the branch is written in vocabulary that already exists:

```
split(case b) → (tag, b')
choose(eq(tag, zero), a, <recurse with succ a, b'>)
```

with the recursion a self-wire (§2.8 of the Phase 2 plan). No new shapes, no widened signature, one new member.

**The count is not stated in prose, here or anywhere.** The old wording — "the minimal set," "eleven," "the twelve" — is what let the code and the document drift apart in the first place: §2.4 of the Phase 2 plan says eleven while `minimal-primitives.md` documents sixteen names, and neither is checkable by reading. Under the rule the count is derived, not declared: `xtask floor` computes it, asserts it is even, and asserts every member's opposite names it back. A document that names its own size invites the reader to check the number instead of the rule. P21-02 adds "minimal set" and every prose count to the banned-wording list; `docs/Findings/floor-vocabulary.md` records what the completed plans' wording is superseded by, without editing them.

**The amendment is loud.** `docs/Findings/the-floor.md` carries a dated amendment record, and this change is its first entry: what was admitted, what was removed, **and the refuted reduction for each**. `zero`/`succ`/`pred` were reduced — each is `build` at a fixed tag — which is why they left. `build`, `case` and `resolve` were attempted as bodies over the remaining floor and could not be written, which is why they were admitted. A change with no refuted reduction is not an amendment; it is a convenience.

**Cost to reverse:** every reference body is rewritten and the-floor.md carries a second dated entry. Cheap until reference bodies are published, which is Phase 9.

### 2.2 A frame's constructor order is part of its identity · **new, and it is the trap in §2.1**

`case` returns tag `k` meaning "the k-th declared constructor," and `build` takes one. A reference body will contain both `eq(tag, 1)` and `build(ℤ, 1, …)` — the latter is what `succ` became. If ℤ's constructor list is ever reordered, that body silently computes something else, with no hash moving anywhere. That is an identity bug wearing a refactor's clothes, and §2.1 made it sharper by pushing `succ` out of the floor: the ordinal is now load-bearing on the construction side too, not just on destruction.

> **`Frame::constructors() -> &[OpName]` is ordered, and the order is part of the frame's identity. Appending a constructor is conservative and does not bump the frame version. Reordering or removing one does.**

This is deliberately *narrower* than §2.2 of the Phase 2 plan, which made all signature growth conservative. Adding an **operation** stays conservative; adding a **constructor** is conservative only when appended, because a prepended constructor shifts every tag above it. FO10 (§3.2) checks that `case`'s tags agree with the declared order on samples.

**Cost to reverse:** if constructor order leaves frame identity, every reference body needs a tag that is stable some other way — the obvious candidate being the constructor's name as a `Text`, which makes a *name* load-bearing in a body and cuts against G1's whole position that names are regulatory and positions are structure. Decide it here.

**PROPOSED · yours · R34**

### 2.3 The floor is named in a genome by `prim:`, not by a cell hash · **new**

A reference body must wire `build`, `case`, `choose` and friends into a place graph, so the floor has to be nameable in a genome. Two ways, and the wrong one is tempting: give each primitive a `.cell` file. That would mean a lawless, witness-less cell for `build` — and §2.3 of the Phase 2 plan is explicit that a floor primitive *has no coding region to be judged against, because it is part of what a coding region is written in*. Manufacturing an empty coding region for it would make the floor look gated when it is declared, which is the one thing the floor's honesty depends on.

So the genome grows a second entry kind:

```
genome {
  prim:build   as mk
  prim:case    as c
  prim:choose  as pick
  cell:6b32…   as sum
}
```

`prim:<name>` resolves against the floor register and nothing else. It is visibly not a `cell:`, which is the point: reading a reference body, you can see where the gated part stops and the declared part starts.

**No golden moves.** A body with no `prim:` entries prints exactly as it did, so `joinn.body.v1` stands and the five Phase 2 body goldens must still verify. That is P21-05's done-when, and if one moves, this decision is wrong.

**Cost to reverse:** the body grammar loses an entry kind and reference bodies are rewritten. Confined to `joinn-dna::body`.

**PROPOSED · yours**

### 2.4 Nesting is an activation stack, never a Rust call · **new, and it is the engineering of this phase**

When an instance's chosen allele is `AlleleBody::Dna(h)`, the engine must run body `h` with the instance's in-port values and deliver its out-port values as the fire's result. The obvious implementation — construct a nested `BodyState` and call `run()` inside `fire()` — is Rust recursion, and rule 10 of `AGENTS.md` forbids it for a reason that applies exactly here: `rat.add` references `mul`, which references `add`, and each is a fire inside a fire.

> **`BodyState` holds a stack of activations. `step` operates on the top activation. When an activation quiesces, it is popped and its out-port values are delivered to the parent's waiting port. `run` is still one loop.**

Four consequences, and they are the commit's real content:

1. **The budget is global.** Steps are charged to one counter across the whole stack. A per-activation budget lets recursion escape by descending, which is the non-termination bug this phase would otherwise ship.
2. **A budget refusal names the activation path.** `add ▸ int.add.ref ▸ int.add.ref` tells a creator where the loop is. A budget refusal that names only the outer instance is worth very little.
3. **`StepReport` gains a depth, printed only when non-zero.** §2.6 of the Phase 2 plan's optional-member rule applies to the trace format for the same reason it applied to the canonical form: absence means what it always meant. The calculator's own run has no DNA alleles, so `calculator.trace` must stay **byte-identical** through this commit. That is P21-06's done-when.
4. **Depth is bounded and checked.** The seal dependency graph is a DAG — a reference body may name sealed cells, but a cycle among seals is a refusal at admission, not a stack overflow at runtime. P21-09 checks it.

**Cost to reverse:** none worth stating. There is no other shape that honors rule 10.

**PROPOSED · yours**

### 2.5 A nested activation has no grants and no membrane of its own · **new**

The moment nesting exists, Cursor will ask whether a nested body inherits the parent's capabilities. The answer is no, and it is not a simplification:

> **A nested activation is a computation, not a body. It receives values, it produces values, and it can reach nothing else. It holds no grants, it cannot `grant` or `revoke`, and its instances cannot read a capability.**

A reference allele that could read `stdin` would be a reference allele whose agreement with the sealed version depends on the world. That is not a reference; it is a second program. And a nested activation that could hold a grant would be answering Phase 5's membrane question — what a capability means across a boundary — inside a Phase 2.1 commit, by accident.

`require`/`ensure` still run on the nested body's boundary ports, because V35 has exactly one delivery path and nesting does not get a second one.

**Cost to reverse:** Phase 5 may decide a nested body *is* a body with a membrane. Nothing here blocks that; it adds a grant table to the activation record.

**PROPOSED · yours · R36**

### 2.6 Agreement samples the full generator on every port except the one that drives the loop · **new, and it is what rescues F3**

F3 says `agree` must draw from `Frame::generate` rather than from a hand-rolled ±32. Point it there naively and the harness never finishes: the reference `add` is linear in `|b|`, and ℤ's generator is required to produce values well beyond `i64::MAX`. A reference allele cannot be run on a number the size of the generator's output. Ever. That is not a defect to fix; it is what "honest reference" means.

The resolution is that **only one port drives the loop**:

```
seal int.add {
  reference  body:9f41…
  sealed     add@ℤ
  drives     1 bound 512
}
```

`agree` draws port 1 from a bounded sampler and **every other port from the frame's own generator**. For `add`, that means `a` is sampled at `i64::MAX`, well past it, deeply negative, at zero — while `b` stays small enough for the reference to finish. And that is precisely enough to catch mutant 13: `a = i64::MAX, b = 1` makes the reference do one step and get it right while the wrapping sealed allele gets it wrong.

`drives` is declared per seal because it differs: `int.mul` loops `|b|` times doing `|a|` work, so both are bounded; `text.parse_int` is driven by the string's length. A seal that declares no `drives` runs its reference on full-generator values and will hang — so **a missing `drives` is a refusal at seal registration**, not a slow test.

**Cost to reverse:** if `drives` turns out to be derivable from the reference body's structure — a termination measure, FO9 lifted from values to bodies — the declaration becomes a check instead of an input. That would be better and it is **R35**.

**PROPOSED · yours · R35**

### 2.7 The turn's round-trip law is generated by the gate, for any `turn k from S` · **corrects F4**

Turn admission is currently in `joinn-prim`, called from `xtask`, and refuses anything but `turn 0 from {1 2}`. The gate admits a cell carrying a `turn` block without asking whether a turn allele exists. So a cell can promise a direction it cannot answer.

> **`check::turn` is the gate's fifth check. `Gate::admit_cell` refuses a cell with a non-empty `turn` block unless a turn allele is registered for it and the generated round-trip law holds on samples.**

Generalized: for `turn k from S`, where `S` holds exactly one out-position `o` and the rest in-positions `I`,

```
∀ (I ∪ {o}) drawn from their port frames.
   self@o( k: turn@k(S), …I ) = the value drawn at o
```

Sampling follows §2.6: full generator on every port unless the forward allele is a DNA reference, in which case the drive bound applies. A disagreement is a truth violation with a counter-example and a seed, as it is today.

**Cost to reverse:** the check moves back out of the gate and `admit_cell` stops refusing unwitnessed turns. One commit.

**PROPOSED · yours**

### 2.8 No instrument writes its own finding · **new, and it is the rule this whole phase is downstream of**

F5 and F6 are the same failure in two places: `xtask gate 2` authors the prose of `turn-annotations.md`, and `xtask perf` authors the sentence "Risk #3 has fired: no." Both are printed whatever the numbers are.

> **`xtask` prints numbers to stdout. A human writes what they mean into `docs/Findings/`. No `xtask` subcommand writes a file under `docs/Findings/` at all.**

It is one line in `AGENTS.md` and it removes an entire class of convincing-wrong-artifact. A findings file whose conclusion was typed by the thing being measured is not evidence, however accurate it happens to be today.

**Cost to reverse:** none. This is a subtraction.

**PROPOSED · yours · R37**

---

## 3. Phase 2.1 — Architecture

### 3.1 Crates

No new crates. The dependency rule is unchanged.

```
joinn-frame ──► joinn-dna ──► joinn-gate ──► joinn-prim ──► joinn-live ──► joinn-run
```

| Crate | What changes |
|---|---|
| `joinn-frame` | `Frame::constructors()`; FO10; `case` stays a trait method (the primitive wraps it) |
| `joinn-dna` | `prim:` genome entries; `Seal`'s `reference` becomes a body hash; the `drives` declaration |
| `joinn-gate` | gains `check::turn` — the fifth check; gains the V33 structural walk |
| `joinn-prim` | `case` in the matter register; reference **bodies** replace the reference Rust oracles; `agree` rebuilt |
| `joinn-live` | the activation stack; `AlleleBody::Dna` fires; `prim:` instances resolve to the floor |
| `joinn-run` | reads stdin; loads `sealed_natives()`, never the mutant register |

**One boundary rule restated because this phase strains it:** `joinn-live` runs a reference body by looking up a body it was handed. It does not know that the body is a reference, does not know what a seal is, and does not call `agree`. Seals are `joinn-prim`'s; the engine only fires what it is given.

### 3.2 The eliminator, end to end

```rust
// joinn-frame
fn constructors(&self) -> &[OpName];   // ordered; part of identity (§2.2)
fn case(&self, v: &Value) -> Case;     // unchanged from Phase 2
```

| # | Obligation | Checked how |
|---|---|---|
| **FO10** | `case`'s reported constructor is the `constructors()` entry at the tag the primitive computes, for every sampled value; and `constructors()` is non-empty for any frame with a `Built` case | sampled, plus every shrunk value. `BrokenFrame` gains a mis-ordered variant that the harness refuses **by name** |

```rust
// joinn-prim::floor
pub struct BuildPrim;   // build(frame, tag, parts) → Value
pub struct CasePrim;    // case(v) → pair(tag, parts)
// Opposition::Inverse("case") and Inverse("build") — each names the other
```

Their opposition is checked the way the others are: for sampled `v`, `split(case v)` gives `(tag, parts)`, and `build(frame, tag, parts)` returns `v`; for `tag = 0`, `v` is a declared generator. That check is FO8 seen from the register's side, which is a feature — the frame and the floor now have a V32-style agreement obligation between them, and it is cheap because both are already written.

**The pairing check** is the rule of §2.1 made mechanical, and it is the smallest useful thing in this phase:

```rust
// xtask — the floor's own Law 1
for p in floor::register() {
    let o = p.opposition();                     // no Opposition::OneWay variant exists
    assert!(floor::contains(o));                // the opposite is in the floor
    assert_eq!(floor::get(o).opposition(), p.name());   // and names it back
}
assert!(floor::register().len() % 2 == 0);      // therefore even
```

An odd count means some member's opposite is missing, and the loop above names it before the parity assertion ever fires. `Opposition::OneWay` is deleted rather than left unused, so a future primitive cannot quietly declare itself exempt.

### 3.3 Reference bodies

```
body {
  codex 1
  genome {
    prim:case    as c
    prim:split   as sp
    prim:eq      as iseq
    prim:choose  as pick
    prim:build   as mk            # mk(ℤ, 1, [a]) is what `succ a` used to be
    cell:6b32…   as self          # the cell under definition, by hash
  }
  grants { }                       # always empty (§2.5)
  wires { … ; self@2 -> self@0 }   # the self-wire is the recursion
  budget { steps 100000 }
  lineage none
}
```

Stored in `corpus/phase21/`, one per seal, each with a hand-computed hash checked against the machine's — the P1-07 and P2-04 discipline, for the third and last time. It is cheap exactly once per new file kind and it is the only place a human checks the machine.

### 3.4 The seal, corrected

```rust
pub struct Seal {
    pub cell: Hash,
    pub reference: BodyRef,        // a real body. No placeholder.
    pub sealed: NativeId,
    pub drives: Option<Drive>,     // §2.6 — None is a refusal at registration
    pub agreements: Vec<Witness>,
}
```

**V33, with teeth.** `check::v33(seal)` walks the reference body's genome transitively:

1. Every entry is `prim:` (a floor member) or `cell:` whose own seal passes this check.
2. `seal.sealed`'s `NativeId` appears nowhere in the reference body or in anything it reaches.
3. The transitive seal graph is acyclic.
4. The reference body holds no grants.

A violation is a refusal naming the offending genome entry. **That is mutant 14**: a seal whose reference is the sealed allele in disguise names `add@ℤ` in its genome and is refused for a reason a person can read. Compare the Phase 2 version, which compared two Rust type names and could not fail.

### 3.5 The activation stack

```rust
struct Activation {
    body: BodyRef,
    instances: BTreeMap<String, Instance>,
    queue: BTreeSet<Key>,
    mail: BTreeMap<Key, Mail>,
    return_to: Option<(InstanceId, u32)>,   // parent instance and port; None at the root
}

pub struct BodyState { stack: Vec<Activation>, step: u64, /* one global budget */ }
```

`step` reads the top of the stack. Firing a `Dna` allele pushes; quiescence pops and delivers. `run` is unchanged in shape — still a loop, still a fold, still no Rust recursion.

---

## 4. Phase 2.1 — The Commit Plan

Twenty-two commits. **Done-when** is a command, and the command must be able to refuse.

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P21-00** | The honest lock | `gates.lock` drops to the gates actually met; `write_lock` stops emitting a hardcoded string and records per-gate outcomes; Appendix A into `AGENTS.md` and `.cursor/rules/joinn.mdc`; §2.8's rule added | `cargo xtask gate all` writes a `gates.lock` that names each Phase 2 gate and its outcome, and `phase 2` is **not** 8/8; a test asserts `write_lock` contains no literal gate score |
| **P21-01** | Delete the tautologies | `v33_reference_not_sealed`, `format_parse_is_one_way`, `v23_holds` removed; mutant 14 registered as **unchecked**; the survivor written into `surviving-mutants.md` **before** any fix | `cargo xtask power` prints `19/20`, names `mutant.sealed_as_reference SURVIVED`, and **fails the build**. A commit whose done-when is a red build is the point |
| **P21-02** | **The floor rule** | `docs/Findings/the-floor.md` replaces `minimal-primitives.md`, with the rule, the eight pairs and the dated amendment; `eq`↔`choose` and `hash`↔`resolve` replace the two self-oppositions and the one-way declaration; `Opposition::OneWay` **deleted**; `hash` moves to the physics register (F8); `cargo xtask floor` written | `cargo xtask floor` prints the pairs and **fails** when an opposite does not name back or the count is odd — demonstrated by deleting `resolve`, which must fail naming `hash`, then restoring it. `vocab` bans the wording "minimal set" and every prose count |
| **P21-03** | Constructor order | `Frame::constructors()`; FO10; ordering in `FrameRef` identity per §2.2; `BrokenFrame` gains a mis-ordering variant | The harness **refuses** the mis-ordered `BrokenFrame` naming FO10; all three frames pass at 10 000 samples; **`corpus verify` still matches all fourteen goldens** |
| **P21-04** | **`build`/`case`, and the constructors leave** | `BuildPrim` and `CasePrim` in the matter register; `zero`, `succ`, `pred` **removed from the floor** and kept in ℤ's signature as frame vocabulary; `resolve` implemented against the corpus store | `check_oppositions` covers `build`/`case` and `hash`/`resolve` at 10 000 samples drawn from **the frame's generators**; a `CasePrim` returning a tag one off the declared order is refused, asserted by value; `resolve` on an absent hash **refuses** rather than returning a default; a genome naming `prim:succ` is refused naming it — the proof that they left |
| **P21-05** | `prim:` in a genome | Body grammar and canonicaliser; floor instances resolve in `joinn-live`; a `prim:` naming something outside the floor register refuses | Four spellings of a `prim:`-bearing body canonicalise byte-identically; **all four `calculator.body` goldens and `sum_turn.cell` still verify** (§2.3); `prim:nonesuch` refused with the name in the reason |
| **P21-06** | The activation stack | `Activation`, the stack, global budget, depth in `StepReport` printed only when non-zero; budget refusal names the activation path | `calculator.trace` replays **byte-for-byte** (no depth printed at depth 0); a nested budget overrun refuses naming the path `a ▸ b ▸ c`, asserted by value; determinism holds at 100 runs × 2 seeds with nesting |
| **P21-07** | Fire a DNA allele | `AlleleBody::Dna` fires; a trivial identity reference body runs end to end; nested activations hold no grants (§2.5); **V35 restated structurally** — `deliver` is the only function that writes a slot, asserted by a test that greps the crate, not by counting deliveries against steps (F8) | An instance whose allele is `Dna` produces the same out-port value as its native twin; a nested body attempting a capability read refuses naming the capability; the V35 test fails if a second write path is introduced into a nested activation, demonstrated by adding one and reverting it |
| **P21-08** | **Reference `int.add` as a body** | `corpus/phase21/int_add_ref.body`, hand-computed hash first; `case` + `split` + `eq` + `choose` + `build`, with `build(ℤ,1,·)` and `build(ℤ,2,·)` where `succ`/`pred` used to be; self-wire recursion | The computed hash equals the one **you computed by hand**; `add@ℤ` and the reference body agree on 256 samples with `a` drawn from the full ℤ generator; `add(0, 3000)` terminates and reports its step count |
| **P21-09** | **V33 with teeth** | `check::v33`; the transitive genome walk; the seal DAG acyclicity check | `cargo xtask power` prints `20/20` again — and a purpose-built seal whose reference genome names `add@ℤ` is **refused naming that entry**; the `surviving-mutants.md` entry is closed with a date and the reason |
| **P21-10** | References for `parse_int` and `format` | Both as bodies over `Text`'s `cons` eliminator; `"007"` still the declared hole | Both agree with their sealed alleles on generator-drawn input; `parse(format n) = n` on 10 000 samples; the one-way declaration is a **data** declaration on the seal that a test reads, not a function returning `true` |
| **P21-11** | References for `mul@ℤ` and `add@ℚ` | `mul` over repeated `add`; `add@ℚ` as cross-multiply-and-reduce over both, per Part III §11 | `agree` covers all five seals with every reference executing on the live engine; **Phase 1 exit-gate demo 1 still passes unchanged** |
| **P21-12** | `Seal::reference` is a body | The placeholder deleted; `fold`/`unfold` round-trip through real bodies | `unfold` returns a cell whose allele the engine **runs**, asserted by running it; V23 is tested as `fold` then `unfold` then *execute*, not as a hash comparison that holds vacuously |
| **P21-13** | **Agree, rebuilt** | `drives` on the seal; non-driving ports from `Frame::generate`; a seal with no `drives` refused at registration | `cargo xtask agree` catches **mutant 13 with no help** — `wrapping_disagrees_beyond_i64` is deleted and `power` still prints 20/20; a seal registered without `drives` is refused naming the seal |
| **P21-14** | **Turn in the gate** | `check::turn` as the fifth check; general `turn k from S`; the hard-coded `{1 2}` restriction removed | A cell with a `turn` block and no registered turn allele is **refused by `admit_cell`**; `sum_turn.cell` still admits against `6b32…`; a `turn 1 from {0 2}` on `Sum` admits with its own allele |
| **P21-15** | The turn count, derived | `HANDWRITTEN_TURN_ALLELES` computed from the turn register; `xtask` stops writing `turn-annotations.md` | `cargo xtask power` gains a case: registering a second hand-written turn allele moves the printed count to 2, asserted by value. The findings file is edited by hand |
| **P21-16** | **The runner reads stdin** | Real IO at the membrane; the scripted session becomes a test fixture, not the product | `echo "two\n2\n3" \| joinn run calculator` prints the five-line transcript; `cargo xtask gate 2` drives it through a pipe and compares byte-for-byte |
| **P21-17** | Fence the mutants | `natives_with_mutants` behind `#[cfg(any(test, feature = "mutants"))]`; runner and `perf` use `sealed_natives()` | The release binary does not link the mutant register — asserted by a test that looks up `mutant.impostor` in `sealed_natives()` and expects absence; `vocab` bans `natives_with_mutants` outside test and xtask paths |
| **P21-18** | Perf, honestly | §2.9's clock ban scoped to `crates/`; `xtask perf` reports **seconds** as well as steps; the reference-on-engine measured | `cargo xtask perf` prints wall time for three probes and **writes no file**; the conclusion on Risk #3 is typed by a human into `live-engine-performance.md` |
| **P21-19** | Harden the lint | `line_has` matches substrings for banned words; `allow(vocab)` requires a trailing reason string; V38 extended | `cargo xtask vocab` **fails** on the current tree (it will catch `calculator_body` in `joinn-live`), then passes once renamed; a bare `allow(vocab)` with no reason is a lint failure |
| **P21-20** | Re-freeze the witnesses | Transcript indent corrected to the plan's exact text; the duplicate `step` number fixed; both re-frozen deliberately with a written finding | `cargo xtask gate 2` replays both; `docs/Findings/` records the re-freeze, its date and its reason, typed by hand |
| **P21-21** | Exit gate 2.1 and restore the lock | `cargo xtask gate 2.1`; `gates.lock` restored | `cargo xtask gate all` runs phases 0, 1, 2 and 2.1 from a clean checkout, offline, and writes `phase 2: 8/8 · phase 2.1: 9/9` from **per-gate outcomes** |

**Ordering notes.** P21-00 and P21-01 come first and both make the build worse — the lock drops and `power` goes red. That is deliberate: the repo should be honest before it is fixed, so that nothing in the middle of this phase can be mistaken for a pass. P21-02 through P21-05 are the floor and the grammar, and are independent of the engine work. P21-06 through P21-09 are the substance; P21-08's hand-computed hash and P21-09's teeth are the two commits that decide whether this phase was worth running. P21-13 through P21-20 are the smaller findings and can be reordered freely.

---

## 5. Test Strategy

### 5.1 New invariants

Continuing from V39.

| # | Invariant | Test | Commit |
|---|---|---|---|
| **V40** | Every seal's reference is a body whose genome reaches only the floor and other valid seals, and never its own sealed allele | `check::v33`, over the seal register | P21-09 |
| **V41** | Every reference allele executes on the live engine; no `Oracle` in `joinn-prim` is registered as a reference | structural: the seal register holds `BodyRef`s, and the type system has no other option | P21-12 |
| **V42** | The seal graph is acyclic; nesting depth is statically bounded | walk at registration; a cyclic pair refuses | P21-09 |
| **V43** | A nested activation holds no grants and can reach no capability | a nested read refuses, asserted by value | P21-07 |
| **V44** | `case`'s tag agrees with the frame's declared constructor order | FO10 | P21-03 |
| **V45** | A cell declaring a turn is refused unless a turn allele agrees | `check::turn` in `admit_cell` | P21-14 |
| **V46** | No `xtask` subcommand writes a file under `docs/Findings/` | a test walks `xtask` for `findings_dir` writes and fails on any | P21-00 |
| **V35**, restated | Exactly one function writes a port slot, at every depth | structural, not a delivery count (F8) | P21-07 |
| **V47** | Every floor member's opposite is in the floor and names it back; the floor's size is therefore even | `cargo xtask floor`, in CI. Deleting one half of any pair fails it **naming the widowed member**, not the parity | P21-02 |
| **V48** | No floor member is computable by a body over the rest of the floor | refutation, not proof: the petition procedure of §2.1. Each amendment records the reduction that failed | P21-02 |

Carried forward and re-run every commit: V18, V19, V20, V24-embryo, FO1–FO9, the three canonical-text properties, the 1 000-append hash-stability test, V33–V39, and Phase 1's four demos.

### 5.2 The one number that matters

`cargo xtask power` is still the instrument, and this phase changes what it measures. Two rules, both new, both in Appendix A:

1. **Every mutant's refusal must name the check that caught it.** `power` prints `refused by check::v33` rather than `refused`. A mutant refused by the wrong check is a survivor wearing a pass — mutant 13 "caught" by a bespoke one-point helper was exactly that.
2. **A mutant that cannot fail is a build failure.** P21-01 demonstrates it on mutant 14. If a future check degenerates into a tautology, `power` should go red, not green. The mechanism: each mutant registers a **negative control** — a variant of itself the check must *accept* — and `power` fails if a mutant's check refuses both. A check that refuses everything is as broken as one that refuses nothing, and neither shows up in a count.

Rule 2 is the general form of the lesson from F2, and it is the most valuable thing in this plan.

### 5.3 What Phase 2.1 deliberately does not test

The reference alleles' speed beyond the three probes of P21-18. A reference is slow by construction; the number exists so that P21-18's `drives` bounds are chosen from evidence, not so anything gets tuned.

---

## 6. Dependencies and Forbidden Constructs

**No new dependencies.** Everything forbidden in Phases 1 and 2 stays forbidden, with two changes and three additions.

| Change | |
|---|---|
| `SystemTime` / `Instant` | Still forbidden in `crates/`. **Permitted in `xtask`**, which is a measuring instrument and cannot reach a hash, a canonical form, a sample or a refusal (§2.9's actual scope). P21-18 |
| The floor | No longer a frozen count. Membership is the rule of §2.1 — irreducible and opposed — and it changes only through a dated amendment in `the-floor.md` carrying a refuted reduction. A candidate that a body over the floor can compute is refused |

| Newly forbidden | Why |
|---|---|
| Any `xtask` write under `docs/Findings/` | §2.8. Instruments print; humans conclude |
| A reference allele implemented as a Rust `Oracle` | F1. `Seal::reference` is a `BodyRef` and there is no other constructor |
| Rust recursion in `fire`, or a per-activation budget | §2.4. Both are how recursion escapes |
| A grant table on an activation record | §2.5. That is Phase 5's question |
| A bare `allow(vocab)` with no reason | P21-19. A silencer with no record is how a lint dies |

---

## 7. Exit Gate 2.1, As a Checklist

Scripted and repeatable, from a clean checkout, offline. `cargo xtask gate 2.1` runs it.

- [x] **1 · The references are bodies.** Every seal's reference is a `BodyRef`; `agree` executes each one on the live engine; no reference `Oracle` exists in `joinn-prim`.
- [x] **2 · V33 has teeth.** A purpose-built seal whose reference genome names its own sealed allele is refused, naming the genome entry. Mutant 14 is refused **by `check::v33`**, printed.
- [x] **3 · The floor is paired, on the record.** `cargo xtask floor` prints every pair, each naming the other, and the count is even. `the-floor.md` holds the dated amendment with its refuted reductions. Deleting `resolve` fails the check naming `hash`; deleting `succ` from ℤ's *signature* fails the frames, and deleting it from the floor changes nothing, because it is not there.
- [x] **4 · Agreement sees past `i64`.** `agree` catches mutant 13 with no bespoke helper; `a` is drawn from ℤ's full generator on every seal that declares a drive.
- [x] **5 · Turn is gate business.** A cell with a `turn` block and no agreeing turn allele is refused by `admit_cell`. A turn other than `0 from {1 2}` admits.
- [x] **6 · Nesting is deterministic.** A trace including nested activations replays byte-for-byte on a different machine; `calculator.trace` at depth 0 is unchanged from Phase 2.
- [x] **7 · Milestone 0 is a session.** `joinn run calculator` reads three lines from stdin and prints the five-line transcript, exactly, indent included.
- [x] **8 · No instrument wrote its own finding.** No `xtask` subcommand writes under `docs/Findings/`; every conclusion line in that directory was typed by a person.
- [x] **9 · The path of truth.** `cargo xtask gate all` runs Phase 0's corpus checks, Phase 1's four demos, Phase 2's transcript and Phase 2.1's nine — all passing — and writes `gates.lock` from per-gate outcomes.

---

## 8. Risks Watched During This Phase

| Risk | Instrument | What to do when it fires |
|---|---|---|
| **The stated adversary · a correction phase that corrects the appearance** | §5.2's negative controls, and every done-when being a refusal | Reject the commit. This phase exists because four checks printed passes; a fix that prints a pass is the same failure with a newer date |
| **New · the reference-on-engine is unusably slow** | P21-18's seconds, and the `drives` bounds | Lower the bound and say so in the findings. Do **not** shorten the sample count silently — a harness that runs fast because it stopped looking is F3 again |
| **New · the floor grows a habit** | The petition procedure of §2.1 — an attempted reduction that must fail — and the amendment record | A candidate that survives because nobody tried the reduction is not admitted, it is unexamined. Write the failed reference body into the amendment, or refuse the petition |
| **New · the activation stack reintroduces recursion** | Rule 10; a depth-4 seal chain test | An overflow is a crash where a refusal belongs. Fix the stack, not the budget |
| **New · constructor order drifts** | FO10; §2.2's version rule | A reordered constructor list with an unmoved frame version silently changes every reference body. Treat it as an identity bug, which it is |
| **3 · The live engine is too slow to build with** | P21-18, now in seconds | Unchanged from Phase 2: write the finding, then decide. Do not optimize inside this phase |
| **5 · Jet mismatch** | `agree` with generator-drawn non-driving ports | The disagreement is the finding. Write the counter-example before touching the sealed allele |

---

## 9. What This Plan Refuses To Do

| Refused | Because |
|---|---|
| Leave `gates.lock` at 8/8 while the phase runs | P21-00. A lock that records intent is not evidence, and a red build is the honest state of a repo mid-correction |
| Add a grammar-level `case` block alongside the primitive | §2.1. Two ways to branch would need a V32 agreement obligation between them for no gain. It stays available as sugar in Phase 6 |
| Give a floor primitive a coding region | §2.3. The floor is declared; a lawless cell would make it look gated |
| Let a nested activation hold a grant | §2.5. That is Phase 5's membrane question and it does not get answered by accident |
| Run a reference allele on a full-generator value at the driving port | §2.6. It would not finish, and a harness that is quietly bounded is worse than one that is loudly bounded |
| Let `xtask` write a findings file | §2.8 |
| Design the Phase 3 host protocol while giving the runner stdin | §1. `joinn-run` is still deleted at Phase 3 and still must not grow a `trait Host` |
| Rebless a golden hash | Still §6.2 of the Phase 0–1 plan. Still the move that destroys a witness corpus |

---

## 10. Open Items This Plan Creates

Continuing from R32.

| ID | Topic | Question |
|---|---|---|
| **R33** | The petition — **answered in part** | §2.1 gives the procedure: attempt the candidate as a reference body over the existing floor, and admit it only if that fails. What stays open is the other half. A *failed* reduction is evidence, not proof — nobody found a body, which is not the same as there being none. Does an admission owe more: a second author's independent attempt, a published statement of what is being trusted, a bound on the search? R32 asked what witnesses the floor's declaration; this now asks what a failed search is worth |
| **R38** | The frameless product | §2.1 keeps `pair`/`split` on the grounds that **a pair is the only value that belongs to no frame** — which is the argument that saves it from being `build` at a product frame. But `Value` is frame-tagged in `joinn-frame` today. Does the value model grow a frameless product, or does a "product frame" exist whose constructor *is* `pair` — and if the latter, has `pair` just been reduced, and does it leave the floor? |
| **R34** | Constructor order as identity | §2.2 makes appending conservative and reordering identity-moving. Does that survive a frame whose constructors are naturally unordered? Is the tag the right encoding at all, or does a stable constructor *hash* beat an ordinal — and if so, does that drag frame identity into every reference body's bytes? |
| **R35** | Is `drives` derivable? | §2.6 declares which port drives the loop. Is that recoverable from the reference body's structure — a termination measure, FO9 lifted from values to place graphs? If it is, `drives` becomes a check rather than an input, and a reference body that cannot state its measure is one that might not terminate |
| **R36** | Nesting and the membrane | §2.5 says a nested activation is a computation, not a body. Phase 5 may disagree. If a nested body can hold a grant, what does `require`/`ensure` mean at a boundary that is inside another boundary — and is that the same question as a lens? |
| **R37** | Negative controls as a standing practice | §5.2 makes every mutant register a variant its check must accept. Does that generalize past the mutant corpus — should every *check* in the gate carry a negative control, so that a check which refuses everything fails the build? R28 asked whether the mutant corpus measures the gate's strength; this asks whether the corpus can measure its own honesty |

**Touchpoints with existing items.** R6 stays closed. R31's count becomes derived at P21-15 and the finding becomes human-authored. R20 and R26 gain FO10. R27 gains §2.2's narrower rule: appending a constructor is conservative, reordering is not. R29 (frame signature versioning across two implementations) gets sharper, because constructor order is now load-bearing in published reference bodies. R30 (recursion and the budget) is answered in part at P21-06 — the budget's unit is the step, it is global across the activation stack, and the creator sees the activation path.

---

## Appendix A · `AGENTS.md` and `.cursor/rules/joinn.mdc` for Phase 2.1

Replace both files with this.

```markdown
# JoInn — standing rules

This repo is JoInn. Phase 2 shipped the floor, seals, Turn, the live engine and a
temporary runner — and four checks that did not check. Phase 2.1 is the
correction: reference alleles become bodies that run on the live engine, V33
becomes a structural check that can fail, agreement samples the frame's
generators, and turn admission moves into the gate. The build plan is
docs/Plans/JoInn Phase 2.1 Implementation Plan.md. Work one numbered commit at a
time. Do not start the next one.

There is no host protocol, no second body, no assay and no compiler in this phase.

## Hard rules

1. A refusal is a VALUE (`Verdict::Refused`), never a Rust `Err` and never a
   panic. `Result` is for host errors only: IO, malformed input, bugs.
2. Never `unwrap`, `expect`, or panic outside tests. `#![forbid(unsafe_code)]`.
3. No `f32`/`f64` anywhere. No `HashMap`/`HashSet` — `BTreeMap`/`BTreeSet` only.
4. No wall clock and no unseeded RNG in crates/. `xtask` MAY measure wall time;
   it cannot reach a hash, a canonical form, a sample or a refusal. Message
   delivery order is a function of the body's grant list, never of arrival time.
5. No `#[derive(Hash)]` and no `serde` derive on any DNA type. The canonical
   writer is hand-written and tested. Bodies follow the same rule as cells.
6. Never change a golden hash in corpus/ to make a test pass. If a hash moves,
   stop and report it.
7. Never weaken, skip, `#[ignore]`, or delete a test to make a build green.
   Report the failure instead.
8. A primitive name must never appear in a coding region. Laws may name: the
   cell under definition (`self`), frame signature operations, and other coding
   regions BY HASH. A BODY may name primitives — that is what `prim:` is for.
9. Display names, literals, prompts, styles and layout are REGULATORY.
10. THE ENGINE NEVER RECURSES IN RUST. Nesting is an explicit activation stack
    with ONE global step budget. A per-activation budget lets recursion escape.
11. A REFERENCE ALLELE IS A BODY. `Seal::reference` is a `BodyRef`. A reference
    implemented as a Rust `Oracle` is the Phase 2 bug and is rejected on sight.
    The agreement harness ships in the same commit as the seal.
12. THE FLOOR IS IRREDUCIBLE AND OPPOSED, and it is defined in
    docs/Findings/the-floor.md. A primitive is admitted only if (a) no body over
    the rest of the floor computes it and (b) it names an opposite that is in
    the floor and names it back. The size is therefore even; an odd count means
    an opposition is missing. Never add to the floor. If a task seems to need a
    new primitive, first write it as a reference body over the existing floor —
    if that succeeds, it was never a primitive, and the body is the answer.
    NEVER write "the minimal set", "the minimal primitive set", or the floor's
    size as a number, in code, comments, commit messages or docs. The count is
    derived by `cargo xtask floor`, never declared.
    `zero`, `succ` and `pred` are NOT floor primitives. They are ℤ's first three
    constructors — `build(ℤ,0,[])`, `build(ℤ,1,[x])`, `build(ℤ,2,[x])` — and
    they live in ℤ's signature as frame vocabulary.
13. `sub`, `subtract` and `minus` are not identifiers in this project.
14. Nothing may depend on joinn-run. It is deleted at the start of Phase 3 and
    must never grow a `trait Host`.
15. `std::io` appears only in joinn-run.
16. A nested activation holds NO grants and can reach no capability.
17. NO INSTRUMENT WRITES ITS OWN FINDING. `xtask` prints numbers to stdout. No
    xtask subcommand writes a file under docs/Findings/. A human writes what the
    numbers mean.
18. EVERY MUTANT NAMES THE CHECK THAT CAUGHT IT, and registers a negative
    control the check must ACCEPT. A check that refuses everything is as broken
    as one that refuses nothing, and a count shows neither.

## Definition of done

A commit is done when the plan's "done when" command passes and you have
reported what it printed. "It compiles" is not done. "The tests pass" is not
done if the test does not refuse anything. A number is not done if the code that
printed it also chose it.
```

## Appendix B · Directory layout after Phase 2.1

```
D:\JoInn\
├── docs\
│   ├── Plans\
│   │   ├── JoInn Phase 2 Implementation Plan.md
│   │   └── JoInn Phase 2.1 Implementation Plan.md    ← this file
│   └── Findings\
│       ├── the-floor.md                 ← replaces minimal-primitives.md
│       ├── floor-vocabulary.md          ← new: what "minimal" is replaced by
│       ├── phase-2-review.md            (F1–F8)
│       ├── surviving-mutants.md         (mutant 14, opened P21-01, closed P21-09)
│       ├── turn-annotations.md          ← hand-written from P21-15
│       ├── live-engine-performance.md   ← hand-written from P21-18
│       └── witness-refreeze.md          ← new, P21-20
└── joinn\
    ├── corpus\
    │   ├── phase0\   phase2\
    │   ├── phase21\                     ← reference bodies, hashes hand-computed
    │   └── transcripts\                 (re-frozen at P21-20)
    └── crates\                          (six; joinn-gate gains check::turn, check::v33)
```

## Appendix C · Glossary delta for code

| Theory term | Rust identifier | Notes |
|---|---|---|
| the floor rule | `xtask floor`, `Opposition` | irreducible **and** opposed; `Opposition::OneWay` no longer exists |
| construction / destruction | `floor::BuildPrim`, `floor::CasePrim` | `build(frame, tag, parts)` ↔ `case(v) → pair(tag, parts)` |
| content address / lookup | `floor::HashPrim`, `floor::ResolvePrim` | `resolve` is partial and returns a `Verdict`; the gap is the store |
| constructor order | `Frame::constructors()` | ordered; part of frame identity (§2.2); `build`'s tag indexes it |
| floor entry in a genome | `prim:<name>` | resolves against the floor register; never a cell hash |
| reference body | `Seal::reference: BodyRef` | a real body. There is no placeholder variant |
| drive port | `Seal::drives: Option<Drive>` | which port bounds the reference's loop (§2.6). `None` refuses |
| activation | `Activation`, `BodyState::stack` | nesting without Rust recursion (§2.4) |
| the fifth check | `check::turn` | turn admission, in the gate (§2.7) |
| the structural check | `check::v33` | the transitive genome walk. This is mutant 14's teeth |
| negative control | `Mutant::control` | the variant the check must accept (§5.2) |

---

*JoInn Phase 2.1 Implementation Plan (Draft 0.1). Corrects F1–F4 of `docs/Findings/phase-2-review.md`. Builds on the Phase 2 Implementation Plan and Parts I–III. Everything marked PROPOSED · yours is a recommendation made while writing this plan; Cursor implements what the roadmap, the grammar document and your decisions say, not what this plan prefers.*
