# JoInn Phase 2 Implementation Plan

**The floor and the live engine · a working plan for Cursor**

*Roadmap Part IV §5 Phase 2 turned into decisions, commits, tests and gates*

Author: AJ · Draft 0.1 · September 17, 2026

> Status tags follow Parts I–IV: **DECIDED**, **PROPOSED**, **OPEN**. Anything marked **PROPOSED · yours** is a recommendation made while writing this plan and is yours to overrule; Cursor implements whatever lands there, not whatever is written here.

> **What this is.** Roadmap Phase 2 — the minimal primitive set, seals, `Turn`, the live engine and the body bus — as an executable build plan. **Milestone 0 is the target:** `joinn run calculator` prints the transcript of the calculator note, refusal included, with no GPU in the process and the DNA typed by hand. Nothing beyond Phase 2 is in scope — no host protocol, no second body, no assay, no pixels, no compiler.

> **Where Phase 1 left off.** `gates.lock` reads `phase 0: pass` / `phase 1: 4/4`. Three crates exist and are green: `joinn-frame` (Text, ℤ, ℚ with the FO1–FO7 obligation set), `joinn-dna` (grammar, canonical text, hashing, the `Genotype` seal), `joinn-gate` (four checks, sampling, shrinking, testimony, twelve mutants refused). The corpus holds nine `.cell` files and their goldens. Phase 2 starts from that and adds nothing to it that moves a hash.

---

## 0. How to Drive Cursor With This Plan

| | |
|---|---|
| **Where this file lives** | `D:\JoInn\docs\Plans\JoInn Phase 2 Implementation Plan.md`, beside the Phase 0–1 plan |
| **Where the code lives** | `D:\JoInn\joinn\` — the existing repo, extended |
| **Standing rules** | Appendix A. It **replaces** the current `AGENTS.md` and `.cursor/rules/joinn.mdc`. Cursor re-reads rules every request; it does not re-read this plan |
| **Unit of work** | one numbered commit from §4. One Cursor session per commit, not one session for the phase |
| **How a commit ends** | its **done-when** line is checkable by a command. If the command does not exist yet, building it is part of the commit |
| **What Cursor may decide** | module layout inside a crate, function bodies, test names, error strings, the shape of the engine's internal queue |
| **What Cursor may not decide** | anything in §2, the minimal set (§3.2 — it is frozen, §3.2 of the Phase 0 exit), the body grammar (§3.4), crate boundaries (§3.1), whether a seal needs a reference allele, or whether an invariant is worth enforcing |

The prompt stays short:

> Implement commit **P2-11** from `docs/Plans/JoInn Phase 2 Implementation Plan.md`. Follow the rules in `AGENTS.md`. Stop when the done-when command passes and report what it printed.

**The Phase 1 failure mode has not gone away, and Phase 2 makes it worse.** Phase 1 was a library that could only refuse things; a wrong answer showed up as a passing gate. Phase 2 is a runtime that produces a *transcript*, and a transcript that looks right is the most convincing wrong artifact this project will ever produce. Every commit below is written so that the thing being checked is a refusal, a disagreement, or a byte-identical replay — never "it printed something plausible."

---

## 1. Scope Fence

### In scope

The minimal set as executable reference semantics; seals with their differential harness; `Turn`; the body form as data; the live engine and the body bus; a runner that is hard-coded to the calculator and is deleted in Phase 3.

### Out of scope — Cursor must refuse these even when they look small

| Not now | Phase | Why it is tempting |
|---|---|---|
| The host protocol, `present`, `probe` as a vocabulary, the environment body | 3 | The transcript needs *printing*, and printing looks like a host. It is not. §1.1 draws the line |
| A second host, a headless test host, witness capture | 3 | "We may as well make the runner generic" is how Phase 3's design gets made by accident in Phase 2 |
| A second body, hyperedges, lenses, systems, `grant` across a body boundary | 5 | The calculator is one body. A `grant` that crosses a membrane is Phase 5's question |
| The assay layer, ∂, homology, declarations | 4 | `declarations` stays empty and stays a refusal |
| The compiler, any specialization, any monomorphization | 10 | Risk #3 is measured in P2-18, not fixed |
| Anything wgpu, winit, or visual | 6+ | — |
| A new primitive outside the frozen set | never, without a fold petition | §3.2. If a task seems to need one, the task is wrong |
| A general constraint solver for `Turn` | not in this plan, maybe never | §2.5. A solver is how `Turn` becomes a research project |

If a Phase 2 task seems to require something in that table, the task is wrong, not the fence. Say so and stop.

### 1.1 The line between the runner and a host

This is the one fence Cursor will walk over without noticing, so it is stated as a rule rather than a paragraph:

> **The Phase 2 runner may name the calculator. A Phase 3 host may not.**

The runner knows there are two `CliInput` instances and one `Sum`; it knows the prompt literals live in the regulatory region and formats them itself; it knows stdout. It is a thousand lines of scaffolding that exists to prove the engine runs, and Appendix A forbids anything from depending on it. It is deleted at the start of Phase 3, exactly as `spikes/` was deleted at the start of Phase 1.

The thing that must **not** happen is the runner growing a `trait Host`. The moment it does, Phase 3's protocol has been designed by a scaffold, and the whole point of Phase 3 — that a host is proved to be a host by there being two — is gone.

---

## 2. Decisions Assumed by This Plan

Eleven calls. Three are carried forward from the roadmap's gaps; the rest were found while writing this plan, and each says what it costs to reverse.

### 2.1 Every frame owes an eliminator · **new, and it is what makes a reference allele writable at all**

Here is the contradiction Cursor hits in commit P2-11, and it is worth seeing in full because it is the sharpest thing in this phase.

V21 says every sealed primitive carries a reference allele *expressible in the minimal set*. The calculator note names the reference for `int.add`: repeated successor, bottoming out in `zero`, `succ`, `pred`, `eq`. Write it:

```
add(a, b) = if b = zero then a else add(succ a, pred b)
```

That terminates for `b ≥ 0` and runs away forever for `b < 0`. To recurse correctly over ℤ you must know *which constructor made this value* — and ℤ's minimal signature is `{zero, succ, pred, eq}`, which is four introduction forms and no elimination form. `eq(b, zero)` distinguishes exactly one value out of infinitely many. **The floor cannot be written with the floor as currently frozen.**

The wrong fixes are all available and all bad: add `lt` to ℤ's signature (a comparison smuggled in as arithmetic, and §2.4's degeneracy problem gets a new front door); hard-code `add@ℤ` with no reference allele (kills V21 and the entire seal story with it); write the reference in Rust (kills it more quietly).

The right fix is the one Law 1 was already asking for:

> **A frame that declares generators owes the matching eliminator. Constructors without a destructor is opposition missing at the bottom of the stack.**

```rust
// joinn-frame — added to the Frame trait
pub enum Case {
    Generator,                  // zero, "", …
    Built { op: OpName, parts: Vec<Value> },   // succ x, pred x, cons(c, rest), ratio(n, d)
}

fn case(&self, v: &Value) -> Case;
```

For ℤ: `zero` → `Generator`, positive → `Built { succ, [n−1] }`, negative → `Built { pred, [n+1] }`. For `Text`: empty → `Generator`, otherwise `Built { cons, [first_char, rest] }`. For ℚ: `Built { ratio, [numer, denom] }` — which is precisely what lets `add@ℚ`'s reference allele be cross-multiply-and-reduce over `add@ℤ` and `mul@ℤ`, as Part III §11 already says it should be.

Two new obligations, checked by the existing conformance harness:

| # | Obligation | Checked how |
|---|---|---|
| **FO8** | `case` inverts the constructors: for every signature op `f` and sampled args, `case(apply_op(f, args)) = Built { f, args }`; `case(v) = Generator` only for a declared generator | sampled, plus every shrunk value |
| **FO9** | `case` is well-founded: iterating `case` from any sampled value reaches a `Generator` in finitely many steps, and every part is strictly smaller under `shrink`'s order | bounded walk, same instrument as FO5 |

FO9 is the one that pays for itself: it is the difference between "the reference allele is slow" and "the reference allele does not terminate," and it is checkable in the frame rather than discovered in the engine.

**Cost to reverse:** `Frame` gains one method and the conformance harness two obligations. Every frame implementation is touched once. Nothing else moves — see §2.2. **PROPOSED · yours**

### 2.2 A frame's signature may grow without moving a cell hash · **new, and it must be tested, not assumed**

§2.1 grows ℤ's signature. Cells name `ℤ 1`. If growing the signature bumps ℤ to version 2, every golden in `corpus/hashes.txt` moves and the Phase 0 hand-computed evidence — the one place in this project where a human and a machine independently agreed on a number — is destroyed to add a destructor.

> **Adding an operation to a frame's signature is a conservative extension and does not bump the frame version. Removing an operation, or changing what one means, does.**

The justification is the same one the gate already uses for cells: accepts ⊇ old, guarantees ⊆ old. A law written against `ℤ 1` before the growth is still a law against `ℤ 1` after it, because nothing it could name was removed. The honest cost is stated rather than argued away: with one implementation there is no way for two machines to disagree about what `ℤ 1` means; with two there would be, and the answer would be a signature hash inside `FrameRef`. That is **R29**, and it is not Phase 2's problem.

The test is not optional. P2-02's done-when is that the signature grew **and `cargo xtask corpus verify` still matches all nine goldens**. If a hash moves, one of these two decisions is wrong and the phase stops until it is known which. **PROPOSED · yours · R29**

### 2.3 The minimal set is admitted by declaration, not by the gate · **new**

Cursor will reasonably try to run the minimal set through `Gate::admit_allele`, because everything else goes through the gate and that is the house style. It must not, and the reason is Part III §7.4, made operational:

> **The floor of the fractal is a declared axiom set. Everything above the floor is gated; the floor itself is declared, named, and visible.**

`succ` has no coding region to be judged against, because `succ` is part of what a coding region is *written in*. Judging it would mean judging it against laws stated in terms of itself. What the floor gets instead is the thing a gate cannot give it: **each minimal primitive ships with its stated opposition as an executable property test** — `pred(succ x) = x`, `split(pair(x, y)) = (x, y)`, `choose(eq(x, x), a, b) = a`, `unbind(bind(p, q))` restores the prior wiring. Those tests are Law 1 at the floor, and they are the only thing standing under it.

Say it plainly in the README, because a declared floor that nobody mentions is an undeclared floor: **this is where the trust is.** That is **R32**. **PROPOSED · yours**

### 2.4 The minimal set is three registers, not one enum · **new**

The frozen set mixes three kinds of thing (Part III §1), and a single `enum Primitive` with an `apply(&[Value])` gives four of the eleven no sensible body and invites a `panic!("bound is not callable")` — which is a Rust panic where a refusal belongs, on day one of the phase.

| Register | Members | What it is | Where it lives |
|---|---|---|---|
| **Matter** | `eq`, `zero`, `succ`, `pred`, `pair`, `split`, `choose` | evaluable, pure, frame-parametric | `joinn-prim::matter`, each an `Oracle` |
| **Space** | `bound`, `fill`, `bind`, `unbind` | body construction. **Not runtime operations** | the `.body` grammar (§3.4). Nothing calls them |
| **Physics** | `hash`, `grant`, `revoke`, `join`, `fan` | services the body bus offers | `joinn-live`, never callable from an allele |

A body file *is* `bound` and `bind` applied; that is what §3.4's grammar means and why there is no `fn bound(...)` anywhere. An allele that could call `grant` would be an allele that can widen its own capabilities, which is R12's whole problem arriving eight phases early. **PROPOSED · yours**

### 2.5 `Turn` is a declared direction with a witnessed turn allele — never a solver · **new, and it is the decision most likely to be argued with**

S5 did not fire its kill criterion: two hand annotations in seven configurations, both on genuinely underdetermined systems. `Turn` is in, and `sub` never exists. The question S5 did not answer is *how*, and there are two roads:

- **A solver.** Give the engine a propagator that inverts a law set. This is the version everyone imagines, and it is a research project with a paper attached. It also puts a search inside the thing that is supposed to be the simple, obviously-true engine.
- **A declaration plus a witnessed allele.** The one taken here.

> A coding region may declare `turn <out-position> from {<positions>}`. A turn is admitted only with a **turn allele** that computes the named position from the others, and the gate admits it by generating the **round-trip law** and checking it against the forward allele on samples.

For `Sum`, `turn 0 from {1, 2}` generates, with no creator involvement:

```
∀ b s : ℤ.  self@2(0: turn@0(1: b, 2: s), 1: b) = s
```

The creator writes no law. The platform writes it, the gate checks it, and a turn allele that disagrees with the forward allele is refused with a counter-example. `sub` is then not a cell, not a primitive, and not a name: it is `Sum` read at a declared turn whose agreement with addition is checked every time the gate runs.

**The honest adversary, stated here so it is not discovered later.** A hand-written turn allele looks a great deal like writing `sub` and filing the paperwork. Three things distinguish them, and all three are checkable: the turn shares the cell's identity and lineage rather than having its own; its law is generated rather than written; and it cannot disagree with the forward allele and survive. What would make the distinction *false* is if a creator must hand-write a turn allele for every direction of every cell — at which point `Turn` is `sub` with ceremony. **So P2-16 counts them**, the way S5 counted annotations, and writes the number into findings. That count is **R31**, and it is the number that decides whether `Turn` survives to Phase 5.

There is one free case worth taking: when the forward allele is a single frame op that has a declared inverse in the signature (`succ`/`pred`), the turn allele is derivable and the platform derives it. Derive where possible; hand-write otherwise; count the hand-written ones.

**Cost to reverse:** the `turn` block is optional in the canonical form (§2.6), so dropping `Turn` drops a block and rehashes only the cells that used it. **PROPOSED · yours**

### 2.6 The `turn` block is an optional coding-region member and `codex` stays `1` · **new**

`turn` is a promise about what the cell will answer, so it is identity and it is hashed. The naive consequence is `codex 2` and a corpus rehash. That is a real cost — the goldens are the only hand-computed numbers in the project — and it is avoidable:

> **An optional coding-region block prints only when present. A cell with no `turn` block prints byte-for-byte as it did under codex 1, and therefore hashes as it did.**

This is legal precisely because absence means the same thing in both formats: *this cell declares no turn*. §2.6 of the Phase 0–1 plan bans a format change that gives two different meanings the same hash; it does not ban a format change that leaves an unchanged meaning with an unchanged hash. The rule that keeps it honest, and that goes in the grammar document:

> An optional member may be added to the canonical form without a `codex` increment **only if** its absence in the new format means exactly what its absence in the old format meant. Any other change increments `codex` and rehashes the corpus.

`corpus verify` matching all nine goldens after the parser learns `turn` is the test. **PROPOSED · yours · R27 gains a row**

### 2.7 Adding a turn to `Sum` is an evolution event, and it is the first one · **new**

`sum.cell` today hashes `6b32…`. Give it a `turn` block and the hash moves — correctly, because the contract now promises more. This is not a problem to be engineered around; it is the first time in this project that the path of truth does what it was built to do, and it should be treated as the event it is:

- `corpus/phase0/sum.cell` stays, unchanged, hash `6b32…`, still verifying.
- `corpus/phase2/sum_turn.cell` is new, with `lineage 6b32…`, and its own golden.
- `Gate::admit_cell(proposed, parent)` judges it under check 4 and must **accept**: no in-port added, nothing weakened, founding witnesses replay.

The first real use of `lineage`, the first real use of check 4 on something other than a test fixture, and a genuinely good story: the add cell grew a direction and kept its family. **PROPOSED · yours**

### 2.8 Recursion is a self-wire bounded by a step budget · **new**

The reference allele for `int.add` recurses, and neither the formula language nor the minimal set has a fixpoint. Three roads; two are bad.

Adding a `rec` primitive breaks the freeze. Writing the reference in Rust makes V21 a comment. What is left is the one that costs nothing new:

> **A wire may run from a cell's out-port to one of its own in-ports. Recursion is a cycle in the place graph, not an operation. Every run carries a step budget; exceeding it is a refusal carrying the step count, the last message, and the seed.**

It fits the rest of the model without an exception: the place graph already permits cycles (the calculator's own complex has two, per S1), and a self-loop is a 1-cycle whose filling is the law `pred(succ x) = x`. It costs one relaxation in the body checker and one counter in the engine.

The thing to be careful about, and P2-10's real deliverable: **a budget refusal must be distinguishable from a correct refusal.** A runaway recursion and a legitimately refused message both come back as `Verdict::Refused`, and if they read the same, "the engine hit its budget" becomes indistinguishable from "the cell said no" — which is how a non-termination bug hides for six months. The budget refusal gets its own `CheckId`, prints the step count and the last message, and is asserted by value in a test. **PROPOSED · yours · R30**

### 2.9 The engine is a loop over a queue and never recurses in Rust · **new**

Small rule, large consequence. The reference allele for `int.add` on `(0, 3000)` is three thousand nested cell activations. If the engine's `step` calls itself, that is a stack overflow — a crash, not a refusal, at exactly the moment the system is supposed to demonstrate that it turns wrongness into values. An explicit worklist costs a morning and makes the step budget meaningful. `#![forbid(unsafe_code)]` does not save you from a stack that deep, and neither does `catch_unwind`. **PROPOSED · yours**

### 2.10 Delivery order is a deterministic function of the grant sequence · **G4, roadmap-recommended**

The body bus is a priority queue keyed by `(grant_epoch, wire_position, message_sequence)`. No timestamps, no arrival order, no `HashMap`. A run is a fold over `step`, so the whole execution is reproducible from `(body, initial messages, grant order)` and replayable from its own recorded trace — which is what makes the transcript a witness in §5.3 rather than a screenshot.

`probe` is not a feature added to the engine. It is the step report the engine already produces, printed. That is the cheapest possible version of Part I's observability claim and it should stay that cheap.

### 2.11 `join` reads its policy from the contract for the first time · **G5, roadmap-recommended**

The field has been hashed and unread since Phase 1. Phase 2 reads it: `Refuse` (default), `Latest`, `Queue`. Under `Refuse`, a second message arriving at a filled required in-port before the cell fires is a refusal naming the port and the policy — not a silent drop, which is the untrue option and therefore not the default. The calculator exercises `Refuse`; the other two get a test each, because a policy nothing exercises is a policy that is wrong.

---

## 3. Phase 2 — Architecture

### 3.1 Crates and the dependency rule

```
joinn-frame ──► joinn-dna ──► joinn-gate ──► joinn-prim ──► joinn-live ──► joinn-run
                                                                              (bin)
xtask ──► all of them  (gate, corpus, vocab, power, agree, perf)
```

| Crate | Phase | Owns | Must not know about |
|---|---|---|---|
| `joinn-frame` | 1, **extended** | values, frames, obligations FO1–**FO9**, conformance | everything above |
| `joinn-dna` | 1, **extended** | coding/regulatory regions, the `turn` block, **the body form**, canonical text, hashing | evaluation, the engine |
| `joinn-gate` | 1, **narrowed** | evaluation, sampling, the four checks, `Oracle`, the registry **mechanism** | the primitives themselves, the engine |
| `joinn-prim` | **2** | the minimal set, reference semantics, seals, fold/unfold, turn admission | the engine's scheduling, IO |
| `joinn-live` | **2** | live engine, body bus, message, join, grant, require/ensure, verdict, probe | hosts, stdin, stdout, the calculator |
| `joinn-run` | **2, temporary** | the hard-coded calculator runner. **Deleted at Phase 3** | — nothing may depend on it |

**Three boundary rules Cursor must not bend:**

1. `joinn-dna` is still **data**. It now parses bodies as well as cells. It still never evaluates anything and never holds a function pointer.
2. `joinn-gate` gets *smaller* in this phase, not larger. `add@ℤ`, `parse@Text` and the twelve mutants currently live in `joinn-gate::natives`; they are alleles, and alleles belong to `joinn-prim` and the corpus. What stays behind is `Oracle`, `NativeRegistry` and the checks. P2-01 exists because if this does not happen first, every subsequent commit adds one more arithmetic function to the gate and the crate boundary quietly dissolves.
3. `joinn-live` contains no `std::io`. Effects are at the membrane, which in Phase 2 means: in `joinn-run`.

### 3.2 The minimal set, frozen

`docs/Findings/minimal-primitives.md` is the freeze. It is not reopened in this phase. Eleven entries in three registers per §2.4; matter is evaluable, space is grammar, physics is engine service.

```rust
// joinn-prim — illustrative
pub trait Reference: Oracle {
    fn name(&self) -> &'static str;
    fn opposition(&self) -> Opposition;   // the stated inverse, or Declared::OneWay
}
```

`opposition()` is not documentation. P2-03's test walks the register, asks each primitive for its opposition, and checks it on samples. A primitive that cannot state one must declare itself one-way, and exactly one does: `hash`.

### 3.3 Seals

```rust
pub struct Seal {
    pub cell: Hash,               // its own coding region — it is a cell
    pub reference: BodyRef,       // expressible in the minimal set (V21)
    pub sealed: NativeId,         // the fast path
    pub agreements: Vec<Witness>, // recorded agreements between the two
}

pub enum AlleleBody { Native(NativeId), Dna(BodyRef) }   // Dna is new in Phase 2
```

`AlleleBody::Dna` is the variant Phase 1 deliberately left room for; adding it changes the allele payload encoding under `joinn.allele.v1` and **moves no cell hash**, because alleles have never been part of cell identity (V19). That is the Phase 1 design collecting its rent.

**The differential harness ships in the same commit as the first seal.** Not the next commit, not "before the phase ends." V32 exists because every system that has tried this wrote the cross-check after the fast path was already trusted, at which point it found nothing because nobody ran it. `cargo xtask agree` is written in P2-11 and run by CI from P2-11 onward.

Two folds, then two more:

| Fold | Reference allele | Closes |
|---|---|---|
| `int.add` | succ-recursion over `case` and `choose` (§2.1, §2.8) | the calculator note's §7 claim |
| `text.parse_int` | digit accumulation over `Text`'s `cons` eliminator | **R6**, in code |
| `int.format` | the turn of `parse_int`, with `"007"` as the declared hole | Part III §11.1 |
| `mul@ℤ`, then `add@ℚ` | cross-multiply-and-reduce over `add@ℤ` and `mul@ℤ` | Part III §11's own description of `add@ℚ` |

**Unfold** returns the reference cell, and V23 is one line: `hash(unfold(fold(c))) == hash(c)`.

### 3.4 The body form

New file kind, new domain tag, **no change to `.cell` and no change to any existing hash**. That is the whole reason it is a separate file: the Phase 0 corpus is the project's only independently-verified artifact and it does not get disturbed to add instances and wires.

```
body {
  codex 1
  genome {
    cell:6b32…  as sum
    cell:c4a0…  as cli_a, cli_b
  }
  grants {
    stdin: cli_a, cli_b        # ordering comes from this list, not from a clock
  }
  wires {
    cli_a@1 -> sum@0
    cli_b@1 -> sum@1
  }
  budget { steps 100000 }
  lineage none
}

---

regulatory {
  prompts { cli_a "a: "  cli_b "b: " }
  present { sum "{0} + {1} = {2}" }
}
```

| Rule | |
|---|---|
| Domain tag | `joinn.body.v1`, same length-prefixed BLAKE3-256 construction as `joinn.cell.v1` |
| Hashed | the `body` block only. The regulatory region and everything after `---` never |
| Cell references | full 64-hex, never a name. A genome entry names a hash and binds instance names to it |
| Instance names | regulatory? **No — coding.** A wire names an instance, so an instance name is structure. Display names for instances are a separate regulatory map, as ports already are |
| Self-wires | permitted (§2.8) |
| Ordering | genome by hash then instance name; grants by capability name; wires by (source instance, source position) |
| Grant order | the `grants` list is the delivery order, and it is hashed. This is G4 made syntactic |

The canonical form gets the same four-spelling treatment the cells got in P1-06: four `calculator.body` files differing in whitespace, ordering, comments and instance display names must produce byte-identical canonical output. That test is P2-04's done-when and it is not negotiable, because a body hash that drifts is a Phase 5 catastrophe with a Phase 2 cause.

### 3.5 The live engine

```rust
// joinn-live — illustrative
pub struct BodyState { /* instances, mailboxes, grants, step counter */ }

pub struct StepReport {
    pub step: u64,
    pub fired: Option<InstanceId>,
    pub direction: Option<Direction>,   // which turn, when a cell has more than one
    pub delivered: Vec<(WireId, Value)>,
    pub checks: Vec<CheckOutcome>,      // require/ensure, by port
}

pub fn step(state: &mut BodyState, natives: &NativeRegistry) -> Verdict<StepReport>;
```

A run is `step` until quiescent or budget-exceeded. Consequences worth stating because they are what makes the phase checkable:

- **Deterministic.** Same body, same inputs, same grants ⟹ the same `Vec<StepReport>`, byte for byte, on every machine and every run. P2-06 asserts it.
- **`probe` is free.** It is `StepReport`, printed.
- **The transcript is a witness.** Because the trace is deterministic, it can be recorded and replayed. §5.3.
- **Every membrane crossing is checked.** No value enters an in-port without `require`, no value leaves an out-port without `ensure`. The engine has exactly one delivery path so that this cannot be true in one place and false in another (V35).
- **Firing is direction-aware.** A cell fires when *any* declared direction's required set is filled (§2.5). Two unknowns means no direction fires and the cell simply waits — which is correct, and which the budget eventually turns into a refusal that says so.

The `require`/`ensure` evaluator is `joinn-gate`'s, with the reserved variable `port` bound. It is not a second evaluator. Two evaluators would need a V32 agreement obligation between them, and there is no reason to buy one.

---

## 4. Phase 2 — The Commit Plan

Twenty commits. Each is one Cursor session. **Done-when** is a command; if it does not exist, building it is part of the commit.

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P2-00** | Scaffold and the Phase 1 freeze | `joinn-prim`, `joinn-live`, `joinn-run` in the workspace; Appendix A into `AGENTS.md` and `.cursor/rules/joinn.mdc`; `spikes/` **deleted** (its findings already live in `docs/Findings/spikes/`); CI unchanged and green | `cargo xtask gate all` still prints `phase 1: 4/4` and `corpus verify: 9 hash(es) match`; `spikes/` no longer exists; the three new crates build empty |
| **P2-01** | Narrow the gate | `add@ℤ`, `add@ℚ`, `parse@Text`, `format@ℤ` move to `joinn-prim`; the twelve mutants move to a test-only register; `joinn-gate` keeps `Oracle`, `NativeRegistry`, the checks | `cargo xtask power` still prints `12/12`; `cargo xtask vocab` gains a rule that `joinn-gate` contains no `BigInt` and no `BigRational`, and passes |
| **P2-02** | Frame eliminators | `Frame::case`, `Case`; FO8 and FO9 in `conformance::check`; eliminators for `Text`, `ℤ`, `ℚ`; `BrokenFrame` extended with a lying `case` | The harness **refuses** `BrokenFrame` for FO8 and FO9 by name; all three frames pass at 10 000 samples; **`corpus verify` still matches all nine goldens** (§2.2) |
| **P2-03** | The matter primitives | `eq`, `zero`, `succ`, `pred`, `pair`, `split`, `choose` as `Reference` oracles; `opposition()`; the floor's README paragraph (§2.3) | A property test walks the register and checks each stated opposition on 10 000 samples; `hash` is the only primitive declaring itself one-way; a primitive with no opposition and no declaration fails the test |
| **P2-04** | The body form, as text | `.body` grammar, parser, printer, canonicalizer; tag `joinn.body.v1`; `calculator.body` hand-written **and its hash hand-computed first** | Four spellings of `calculator.body` canonicalize byte-identically; the computed body hash equals the one **you computed by hand**. If they differ, the plan stops here until it is known which is wrong |
| **P2-05** | The body model | `Genome`, `Instance`, `Wire`, `Grants`, `Budget` in `joinn-dna`; `Genotype`/`Sealed` extended to bodies; self-wires permitted; a wire to a nonexistent port refuses | `cargo test -p joinn-dna`; `hash(&body.regulatory)` is a compile error, proven by a new `trybuild` case; a body naming an unknown cell hash refuses with the hash in the reason |
| **P2-06** | The engine core | `BodyState`, `step`, the deterministic queue, `StepReport`; no IO, no recursion in Rust (§2.9) | A three-cell body produces a byte-identical `Vec<StepReport>` across 100 runs and across two different seeds; replaying the recorded trace reproduces it exactly |
| **P2-07** | `join` and the join policy | The contract's `join_policy` read for the first time; `Refuse`, `Latest`, `Queue`; `fan` | Under `Refuse`, a second message to a filled required in-port refuses **naming the port and the policy**; `Latest` replaces; `Queue` queues — one asserted test each |
| **P2-08** | The membrane | `require`/`ensure` at every crossing, via the gate's evaluator with `port` bound | `"two"` into `CliInput` refuses at the membrane with the reason, and **nothing crosses the out-port** — the transcript's second line as a unit test, before a CLI exists. A test asserts there is exactly one delivery path (V35) |
| **P2-09** | `grant` and `revoke` | Capability ordering from the body's `grants` list; a read without the grant refuses; a grant never returned stalls the body | `cli_a` reads before `cli_b` in every run regardless of queue state; an ungranted read refuses naming the capability; a stalled body refuses at its budget naming the holder (R9 foreshadowed, not solved) |
| **P2-10** | Recursion and the budget | Self-wires; the step budget; a dedicated `CheckId::Budget` | A self-wired counting body terminates and reports its step count; an unguarded one refuses with the step count, the last message and the seed, **asserted by value** and visibly distinct from a membrane refusal (§2.8) |
| **P2-11** | **The first fold: `int.add`** | Reference allele as a cell (case + choose + succ/pred) running on the live engine; sealed allele `add@ℤ`; `Seal`; `AlleleBody::Dna`; **`cargo xtask agree` in this same commit** | `cargo xtask agree` prints `int.add: reference ≡ sealed on N samples`; an injected disagreement prints a **truth violation with its counter-example**, not a test failure; all nine cell goldens unchanged |
| **P2-12** | Fold and unfold | `fold`, `unfold`; V23 | `hash(unfold(fold(c))) == hash(c)` as a property over the corpus; unfolding a sealed primitive in a test returns a cell the engine can actually run |
| **P2-13** | **The second fold: `text.parse_int`** and `int.format` | Digit accumulation over `Text`'s eliminator; `format` as its turn; `"007"` recorded as the declared hole | Both agree with their sealed alleles; `parse(format n) = n` holds on 10 000 samples; `format(parse s) = s` is **declared one-way** and a test asserts the declaration exists. **R6 is closed in code** |
| **P2-14** | `mul@ℤ` and `add@ℚ`'s reference | `mul@ℤ` folded; `add@ℚ` reference allele as cross-multiply-and-reduce built from `add@ℤ` and `mul@ℤ` | `agree` covers all four seals; **Phase 1 exit-gate demo 1 passes unchanged** — `add@ℚ` still admitted, `(2,3) → 5` still replays, `6b32…` still byte-identical |
| **P2-15** | **Turn I: declaration and generated law** | The optional `turn` block (§2.6); turn admission; the generated round-trip law; derivation for the free case | `corpus verify` matches all nine codex-1 goldens **after** the parser learns `turn`; `corpus/phase2/sum_turn.cell` is admitted against parent `6b32…` under check 4; a turn allele that disagrees with addition is refused with a counter-example |
| **P2-16** | **Turn II: direction in the engine** | Direction choice per instance; waiting on two unknowns; the hand-written-turn-allele count | `5 − 3 = 2` comes out of `Sum` turned; `cargo xtask vocab` bans `sub` and `subtract` as identifiers **and passes**; the count of hand-written turn alleles is printed and written to `docs/Findings/turn-annotations.md` (R31) |
| **P2-17** | **The runner · Milestone 0** | `joinn-run`; stdin/stdout at the membrane only; prompts and the present template read from the regulatory region | `joinn run calculator` prints the five-line transcript **exactly**, refusal included. The transcript is stored in `corpus/transcripts/calculator.txt` and `cargo xtask gate 2` replays it byte-for-byte |
| **P2-18** | **Live engine performance, honestly** | `cargo xtask perf`: the calculator, a 50-cell synthetic body, and the reference `add` on a large number | Three numbers exist in `docs/Findings/live-engine-performance.md`, with a stated extrapolation to a creator-sized body and an explicit yes/no on whether **Risk #3 has fired** |
| **P2-19** | Gate power II and the freeze | Live-engine mutants (§5.4); `cargo xtask power` extended; `gates.lock` records phase 2; README; Phase 2 findings | `cargo xtask power` prints `20/20` and fails below 100%; `cargo xtask gate all` runs phases 0, 1 and 2 from a clean checkout, offline |

**Ordering notes.** P2-00 through P2-03 are independent of the body form and can run in any order after P2-00. P2-04 is the first commit that needs the body grammar, and the hand-computed body hash in it is the same discipline P1-07 used — it is the only place a human checks the machine, and it is cheap exactly once. P2-11 is the substance of the phase: everything before it is the equipment that makes a seal checkable, and everything after it is consequence. P2-15 and P2-16 can slip to the end of the phase without blocking Milestone 0 — the calculator does not need `Turn` to print — which is worth knowing if the turn-allele count starts looking bad.

---

## 5. Test Strategy

### 5.1 New invariants, as property tests, from the commit that introduces them

Continuing Part IV's numbering from V32.

| # | Invariant | Test | Commit |
|---|---|---|---|
| **V33** | Every sealed primitive's reference allele is expressible in the minimal set, and `unfold(fold(c))` hashes to `c` | property over the corpus | P2-12 |
| **V34** | The engine's step sequence is a deterministic function of (body, inputs, grant order) | 100 runs, two seeds, byte-identical traces | P2-06 |
| **V35** | No value enters an in-port without `require`; none leaves an out-port without `ensure` | one delivery path, asserted structurally | P2-08 |
| **V36** | A declared turn never disagrees with the forward allele on samples; disagreement is a truth violation | generated round-trip law, in the gate | P2-15 |
| **V37** | Growing a frame's signature moves no cell hash | `corpus verify` after P2-02 and after P2-15 | P2-02 |
| **V38** | Nothing in `joinn-live` or `joinn-prim` names a host, stdin, stdout or the calculator | `xtask vocab` | P2-00 |
| **V39** | A budget refusal is distinguishable by value from every other refusal | asserted `CheckId` and fields | P2-10 |

Carried forward and re-run every commit: V18, V19, V20, V24-embryo, FO1–FO7, the three canonical-text properties, and the 1 000-append hash stability test. V28 is not a slogan here — `cargo xtask gate all` runs Phase 1's four demos on every commit in this phase, and a Phase 2 commit that breaks one is rejected rather than explained.

### 5.2 The agreement harness

`cargo xtask agree` is V22 and V32 in one command. For each seal, it samples the frame's generators, runs both alleles, and compares. It prints the sample count and the seed. Three rules:

1. It ships with the **first** seal, not after the fourth.
2. A disagreement is a **truth violation** — it prints the counter-example, the seed, and the words "reference and sealed alleles disagree," and it fails the build. It is never reported as a test failure, because a test failure invites a fix to the test.
3. It runs in CI. A cross-check nobody runs finds nothing, which is Urbit's scar and the reason V32 is worded the way it is.

### 5.3 The transcript is a witness, not a screenshot

Because the engine is deterministic (§2.10), the calculator's run is a recordable artifact:

```
corpus/transcripts/calculator.txt      the five lines
corpus/transcripts/calculator.trace    the StepReport sequence
```

`cargo xtask gate 2` replays both and compares byte-for-byte. The transcript is the roadmap's §7 promise made mechanical:

> **From this commit onward, `joinn run calculator` prints those five lines forever.** On three platforms, with a visual creator and a compiler and a registry. The day it stops, something true became untrue.

The trace is the stronger of the two, and it is the one that catches the interesting failure: a Phase 6 change that produces the right five lines by a different sequence of steps has changed the engine's meaning while keeping its output, and the transcript alone would not notice.

### 5.4 Gate power II

The twelve Phase 1 mutants stay and still must all be refused. Eight more, aimed at the things Phase 2 introduces — the gate's power is now partly the *engine's* power, and the same instrument measures it:

| # | Mutant | Should be caught by |
|---|---|---|
| 13 | A seal whose sealed allele is correct except beyond `i64` | `agree` (this is mutant 5 grown up: Phase 10's compiler risk, rehearsed eight phases early) |
| 14 | A seal whose reference allele is the sealed one in disguise | `agree` passes — **so this one is caught by a rule, not a test**: a reference allele that is not expressible in the minimal set fails V33's structural check |
| 15 | An engine that drops a second message under `Refuse` instead of refusing | P2-07's policy test |
| 16 | An engine that delivers by arrival order instead of grant order | V34's determinism test, and `cli_b` reading first |
| 17 | A cell that reads a capability it was never granted | P2-09 |
| 18 | A `require` skipped on the second delivery to the same port | V35 |
| 19 | A turn allele correct on positives, wrong on negatives | the generated round-trip law, with ℤ's generator doing its job |
| 20 | A budget refusal reported as a membrane refusal | V39 |

`cargo xtask power` prints `20/20` and fails below 100%. **A survivor is written up in `docs/Findings/surviving-mutants.md` before it is fixed**, because a surviving mutant names a law or a rule nobody wrote, and that pair is still the raw material for R23.

### 5.5 What Phase 2 measures, and what it deliberately does not

It measures exactly three numbers (P2-18) and draws one conclusion from them. It does not build a benchmark suite, does not tune anything, and does not optimize the engine — because Risk #3's question is "is the two-engine model sound," not "is this loop fast," and an optimized live engine answers the second question while hiding the first.

---

## 6. Dependencies and Forbidden Constructs

**No new dependencies.** `num-bigint`, `num-rational`, `blake3`, `unicode-normalization`, `proptest`, `trybuild`, `insta` are all that Phase 2 needs. If Cursor believes otherwise, the answer goes in `docs/Findings/dependencies.md` with a line saying what it replaces — and the likeliest candidate, an async runtime for the engine, is refused outright: the engine is a synchronous fold and nothing in Phase 2 waits on anything except stdin, which happens in `joinn-run`.

Everything forbidden in Phase 1 stays forbidden. New:

| Forbidden | Why |
|---|---|
| Rust recursion in the engine or in reference-allele evaluation | §2.9. A stack overflow is a crash where a refusal belongs |
| `std::io` anywhere but `joinn-run` | Effects at the membrane. `joinn-live` that can print is a host |
| A `trait Host`, or any generic surface, in `joinn-run` | §1.1. Phase 3's protocol is not designed by a scaffold |
| Any dependency on `joinn-run` | It is deleted in Phase 3 |
| The identifiers `sub`, `subtract`, `minus` | §2.5. `Turn` means the word does not exist. Enforced by `xtask vocab` |
| A new primitive outside `docs/Findings/minimal-primitives.md` | §3.2. The set is frozen; growth is a fold petition, and petitions are Phase 12 |
| A solver, a search, or backtracking in `Turn` | §2.5 |
| A second formula evaluator | §3.5. Two would need a V32 obligation between them for no gain |
| Reblessing a golden hash, still and always | §6.2 of the Phase 0–1 plan. This is the move that destroys a witness corpus |

---

## 7. Exit Gate 2, As a Checklist

Scripted and repeatable, from a clean checkout, offline. `cargo xtask gate 2` runs it and prints the transcript.

- [ ] **1 · Milestone 0.** `joinn run calculator` prints, exactly:

```
a: two
   refused at membrane: "two" is not in ℤ
a: 2
b: 3
2 + 3 = 5
```

  with no GPU in the process, no visual editor in existence, and the DNA typed by hand.

- [ ] **2 · Agreement.** Every seal's reference and sealed alleles agree on a sampled corpus. An injected disagreement is reported as a truth violation with a counter-example and fails the build.
- [ ] **3 · Determinism.** The recorded step trace replays byte-for-byte, on a different machine, from a clean checkout.
- [ ] **4 · The turn.** `5 − 3 = 2` is produced by `Sum` read at a declared turn. The identifier `sub` appears nowhere in the workspace.
- [ ] **5 · Evolution.** `sum_turn.cell` is admitted against parent `6b32…` under check 4, and `sum.cell` still hashes `6b32…`.

And, because a gate that only tests what it was built to pass is not a gate:

- [ ] **6 · Gate power.** `cargo xtask power` reports 20/20.
- [ ] **7 · The path of truth.** `cargo xtask gate all` runs Phase 0's corpus checks, Phase 1's four demos, and Phase 2's transcript — all of them, all still passing.
- [ ] **8 · The honest number.** `docs/Findings/live-engine-performance.md` exists, contains three measurements and an extrapolation, and says yes or no on Risk #3.

---

## 8. Risks Watched During This Phase

| Roadmap risk | Instrument in this plan | What to do when it fires |
|---|---|---|
| **3 · The live engine is too slow to build with** — the phase's stated adversary | P2-18's three numbers | Do not optimize. Write the finding, then decide whether the answer is incremental compilation of stable subgraphs — a **third** engine, with its own V32 obligation, which must be planned rather than improvised into existence |
| **5 · Jet mismatch** | `cargo xtask agree`, in CI from P2-11 | The disagreement is the finding. Do not fix the sealed allele until the counter-example is written down — it is the shape of every future mismatch |
| **1 · Laws underdetermine implementations** | Gate power II; S3's finding already recorded | Still collecting for R23. Mutant 14 is the new shape: a reference allele that is not really a reference |
| **New · `Turn` is `sub` with paperwork** | The hand-written-turn-allele count (P2-16, R31) | If the count is one per direction per cell, say so plainly and let Phase 5 decide whether `Turn` survives. S5 counted annotations; this counts alleles; both are numbers, not impressions |
| **New · The runner becomes the host** | §1.1's fence, V38, and the deletion at Phase 3 | Delete it and start Phase 3's protocol from the roadmap, not from the scaffold |
| **New · The budget masks non-termination** | V39, P2-10 | A budget refusal that reads like a membrane refusal is the bug. Fix the refusal, then look for the loop |
| **New · An optional block makes the canonical form ambiguous** | §2.6's rule, and `corpus verify` on every commit | Increment `codex`, rehash deliberately, write the finding. The rehash was always affordable; doing it by accident was not |
| **New · The floor is trusted without being named** | §2.3's README paragraph, R32 | The floor is where the trust is. Write it where a stranger will read it, not in a plan they will not |

---

## 9. What This Plan Refuses To Do

| Refused | Because |
|---|---|
| Define a host protocol, or let the runner grow one | §1.1. A host is proved to be a host by there being two, and there is one |
| Add a primitive to the minimal set | §3.2. The set was frozen at the Phase 0 exit; growth is a fold petition and petitions are Phase 12 |
| Write a solver for `Turn` | §2.5. A declared direction with a witnessed allele is checkable this phase; a solver is a research project with a paper attached |
| Ship a seal without a reference allele | V21. A seal without one is a native function with good manners |
| Ship the differential harness after the seal | V32. That is the exact sequence that made jet mismatch famous |
| Optimize the live engine | §5.5. Risk #3's question is soundness, not speed, and a tuned engine answers the wrong one |
| Let a stack overflow stand in for a refusal | §2.9 |
| Bump `codex` to add an optional block | §2.6, and only while its absence means what it always meant |
| Rebless a golden hash | Still §6.2. Still the move that destroys a witness corpus |
| Keep `joinn-run` past Phase 3 | §1.1. `spikes/` set the precedent and P2-00 enforces it retroactively |

---

## 10. Open Items This Plan Creates

Continuing from R28.

| ID | Topic | Question |
|---|---|---|
| **R29** | Frame signature versioning | Signature growth is conservative and does not bump the version (§2.2). Does that survive a second implementation, where two builds could disagree about what `ℤ 1` means? Is the answer a signature hash inside `FrameRef` — and if so, does adding one rehash the corpus, or is it outside the cell's canonical text? |
| **R30** | Recursion and the budget | Is a self-wire the right shape for recursion, or does the place graph want an explicit fixpoint after all (§2.8)? What is the budget's unit — steps, messages, or activations — and who sets it: the body, the runner, or the creator? What does the creator *see* when a budget fires? |
| **R31** | Is a declared turn really `Turn`? | The hand-written-turn-allele count from P2-16 (§2.5). If a creator must write one per direction per cell, `Turn` is a naming convention in a structural costume after all, and S5's clean result was a property of one very symmetric relation |
| **R32** | The floor's trust | The minimal set is admitted by declaration, not by the gate (§2.3). What witnesses the declaration? Are the stated oppositions enough, or does the floor owe something more — a second implementation, a proof, a published statement of what is being trusted? |

**Touchpoints with existing items.** R6 (is `text.parse_int` a primitive or a cell?) is **closed in code** at P2-13: it is a cell, sealed. R11 (time) becomes real at P2-09 — grant-based ordering is implemented and hashed. R18 (sealing mechanics) is implemented at P2-11 and P2-12; its petition and retraction half stays in Phase 12. R7 (lone cells) is forced by the calculator and answered in whatever way P2-05's body checker ends up requiring — record the answer, do not leave it implicit. R9 (supervision) is foreshadowed by P2-09's stalled-grant refusal and not solved. R20 and R26 (frame obligations) each gain a row: FO8 and FO9. R27 (canonical-form versioning) gains §2.6's optional-member rule.

---

## Appendix A · `AGENTS.md` and `.cursor/rules/joinn.mdc` for Phase 2

Replace both files with this. It is written to be read cold, on every request, by an agent that has not read the plan.

```markdown
# JoInn — standing rules

This repo is JoInn. Phase 1 (the truth core) is done and frozen: three crates,
nine corpus goldens, gates.lock at 4/4. Phase 2 adds the minimal primitive set,
seals, Turn, the live engine and a temporary runner. The target is Milestone 0:
`joinn run calculator` prints the calculator transcript, refusal included, with
no GPU. The build plan is docs/Plans/JoInn Phase 2 Implementation Plan.md. Work
one numbered commit at a time. Do not start the next one.

There is no host protocol, no second body, no assay, no compiler and no UI in
this phase.

## Hard rules

1. A refusal is a VALUE (`Verdict::Refused`), never a Rust `Err` and never a
   panic. `Result` is for host errors only: IO, malformed input, bugs.
2. Never `unwrap`, `expect`, or panic outside tests. `#![forbid(unsafe_code)]`.
3. No `f32`/`f64` anywhere. No `HashMap`/`HashSet` — `BTreeMap`/`BTreeSet` only.
4. No wall clock, no environment, no unseeded RNG. Sampling takes an explicit
   seed and every refusal reports the seed that produced it. Message delivery
   order is a function of the body's grant list, never of arrival time.
5. No `#[derive(Hash)]` and no `serde` derive on any DNA type. The canonical
   writer is hand-written and tested. Bodies follow the same rule as cells.
6. Never change a golden hash in corpus/ to make a test pass. If a hash moves,
   stop and report it. Reblessing requires an explicit flag and a written
   finding.
7. Never weaken, skip, `#[ignore]`, or delete a test to make a build green.
   Report the failure instead.
8. A primitive name must never appear in a coding region. Laws may name: the
   cell under definition (`self`), frame signature operations, and other coding
   regions BY HASH.
9. Display names, literals, prompts, styles and layout are REGULATORY. They are
   never hashed. If a change to one of them moves a hash, that is a bug in the
   canonicalizer.
10. THE ENGINE NEVER RECURSES IN RUST. It is a loop over an explicit queue. A
    reference allele can be three thousand activations deep; a stack overflow is
    a crash where a refusal belongs.
11. Every seal carries a reference allele expressible in the minimal set, and
    the agreement harness (`cargo xtask agree`) ships in the SAME COMMIT as the
    seal. A reference/sealed disagreement is a TRUTH VIOLATION, never a test
    failure.
12. The minimal primitive set is frozen in docs/Findings/minimal-primitives.md.
    Do not add to it. If a task seems to need a new primitive, the task is wrong.
13. `sub`, `subtract` and `minus` are not identifiers in this project.
    Subtraction is `Sum` read at a declared turn.
14. Nothing may depend on joinn-run. It is hard-coded to the calculator, it is
    deleted at the start of Phase 3, and it must never grow a `trait Host`.
15. `std::io` appears only in joinn-run. Effects are at the membrane.

## Crate boundaries

- joinn-frame: values, frames, obligations FO1–FO9, the conformance harness.
  Knows nothing about DNA.
- joinn-dna: coding region, regulatory region, laws, witnesses, the body form,
  canonical text, hashing. It is DATA — it never evaluates anything and never
  holds a function pointer.
- joinn-gate: evaluation, sampling, the four checks, Oracle, the registry
  mechanism. It holds no arithmetic and no alleles.
- joinn-prim: the minimal set, reference semantics, seals, fold/unfold, turn
  admission. No scheduling, no IO.
- joinn-live: live engine, body bus, message, join, grant, require/ensure,
  verdict, probe. No IO, no host, no knowledge of the calculator.
- joinn-run: the temporary runner. Nothing depends on it.

## Vocabulary

Use exactly these words in identifiers, comments and commit messages:
coding region, regulatory region, allele, reference allele, sealed allele,
frame, eliminator, witness, founding witness, testimony, verdict, refusal,
counter-example, contract, port, law, gate, assay, body, genome, instance,
wire, grant, seal, fold, unfold, turn, step, probe.

Do not use: schema, spec, metadata, config, impl, variant, backend, type (for
frame), error/failure/invalid (for refusal), test case/fixture (for witness),
sub/subtract/minus (for a turn), host (for the runner), node (for a cell).

## Definition of done

A commit is done when the plan's "done when" command passes and you have
reported what it printed. "It compiles" is not done. "The tests pass" is not
done if the test does not refuse anything. A transcript that looks right is not
done if it was not replayed byte-for-byte.
```

## Appendix B · Directory layout after Phase 2

```
D:\JoInn\
├── docs\
│   ├── Theory\                     (Parts I–III, calculator note)
│   ├── Plans\
│   │   ├── JoInn Build Roadmap.md
│   │   ├── JoInn Phase 0-1 Implementation Plan.md
│   │   ├── JoInn Phase 2 Implementation Plan.md      ← this file
│   │   ├── JoInn Coding Region Grammar.md            (+ the body form, + the turn block)
│   │   └── JoInn Phase 0 Exit.md
│   └── Findings\
│       ├── spikes\                 (S1–S6; the code is gone, the findings stay)
│       ├── minimal-primitives.md   (frozen)
│       ├── surviving-mutants.md
│       ├── turn-annotations.md     ← new, R31
│       ├── live-engine-performance.md  ← new, Risk #3
│       ├── canonical-form-changes.md
│       └── dependencies.md
└── joinn\
    ├── Cargo.toml                  (spikes gone; three new members)
    ├── AGENTS.md                   (Appendix A)
    ├── .cursor\rules\joinn.mdc     (Appendix A)
    ├── gates.lock                  (phase 0: pass · phase 1: 4/4 · phase 2: 8/8)
    ├── crates\
    │   ├── joinn-frame\            (+ case, FO8, FO9)
    │   ├── joinn-dna\              (+ the body form, + the turn block)
    │   ├── joinn-gate\             (narrowed: no alleles, no arithmetic)
    │   ├── joinn-prim\             ← new
    │   ├── joinn-live\             ← new
    │   └── joinn-run\              ← new, temporary
    ├── corpus\
    │   ├── phase0\                 (nine .cell files, untouched)
    │   ├── phase2\                 (sum_turn.cell, calculator.body ×4)
    │   ├── hashes.txt              (nine old + the new ones)
    │   ├── mutants\                (twelve + eight)
    │   ├── transcripts\            ← new: calculator.txt, calculator.trace
    │   └── testimony\
    └── xtask\                      (gate, corpus, vocab, power, agree, perf)
```

## Appendix C · Glossary delta for code

| Theory term | Rust identifier | Notes |
|---|---|---|
| eliminator | `Frame::case`, `Case` | §2.1. Every constructor's opposite |
| minimal set, matter register | `joinn_prim::matter`, `Reference`, `Opposition` | evaluable; each an `Oracle` |
| minimal set, space register | the `.body` grammar | `bound`/`bind` are not functions |
| minimal set, physics register | `BodyBus::{grant, revoke, join, fan}` | engine services; no allele may call them |
| seal | `Seal`, `AlleleBody::Dna`, `fold`, `unfold` | §3.3 |
| reference allele | `Seal::reference` (a `BodyRef`) | runs on the live engine |
| sealed allele | `Seal::sealed` (a `NativeId`) | the fast path |
| agreement | `xtask agree` | V22, V32. Disagreement is a truth violation |
| body | `Body`, `Genome`, `Instance`, `Wire`, `Grants` | hashed under `joinn.body.v1` |
| the live engine | `BodyState`, `step`, `StepReport` | a fold, never a recursion |
| probe | `StepReport`, printed | not a separate feature |
| turn | the `turn` block, `TurnDecl`, the generated round-trip law | §2.5. `sub` does not exist |
| step budget | `Budget::steps`, `CheckId::Budget` | §2.8. Distinguishable by value from every other refusal |
| the runner | `joinn-run` | temporary. Deleted at Phase 3 |

---

*JoInn Phase 2 Implementation Plan (Draft 0.1). Builds on the Build Roadmap (Part IV) §5 Phase 2, the Phase 0–1 Implementation Plan, and Parts I–III. Everything marked PROPOSED · yours is a recommendation made while writing this plan; Cursor implements what the roadmap, the grammar document and your decisions say, not what this plan prefers.*
