# JoInn Phase 5 Implementation Plan

**More than one · a working plan for Cursor**

*Bootstrap 0's last phase: the second body, the link graph, and the first rule in this repo that is measured instead of stated*

Author: AJ · Draft 0.1 · September 22, 2026

> Status tags follow Parts I–IV: **DECIDED**, **PROPOSED**, **OPEN**. Anything marked **PROPOSED · yours** is a recommendation made while writing this plan and is yours to overrule; Cursor implements whatever lands there, not whatever is written here.

> **What this is.** Phase 3 gave the universe a face and proved it by building two of them. It also shipped a gate whose item 8 — the one that enforces rule 24 across every gate in the repo — counts lines in a list. This phase opens with that debt and then does the roadmap's Phase 5: a second body, hyperedges, lenses, and capability flow across a system boundary. **Phase 4 is deferred until after this phase**, for reasons in §1 and in `docs/Findings/phase-3-review.md`; one piece of it, ∂ and the ∂∂ assembly, comes forward because Phase 5's central law is unenforceable without it.

> **The one idea.** Phase 2.1's was *a reference allele is a body*. Phase 2.2's: *opposition applies to the instruments*. Phase 3's: *a description is a value, not text*. This one: **a forbidden connection is refused by a measurement, not by a predicate.** The touch-only law has been a sentence in Part I since September and a hope ever since. Part III §9.4 already says what it is: `∂(hyperedge) ⊆ ∪ ∂(cell)` — *boundaries meet only at boundaries* — and says why keeping the assay separate matters: **merged, ∂∂ = 0 would hold by construction and therefore detect nothing.** Derived, the assembly can fail, and V16 stops being a rule one hopes was followed and becomes a refusal naming the port.

> **The corollary that pays for it.** If ∂ is real, then a body's membrane does not need to be declared: **a membrane is ∂(body)** — the ports of its cells that no internal wire consumes. Nothing is added to the `.body` grammar, no golden hash moves, and the question *"is this port reachable from outside?"* has one answer computed from DNA rather than a second copy of the structure that can drift from it. §2.5 is where the phase actually turns.

> **Why this is a phase and not a patch.** The roadmap's Phase 5 names one crate and five deliverables, and four of the five are data models that would be bookkeeping on their own. The decision with teeth in it is §2.5 — and everything else, including the second body, falls out of having somewhere true to attach it. A phase that only added a hyperedge struct would be a refactor; a phase in which the membrane becomes a computed thing is the last brick in Bootstrap 0.

> **Where Phase 3 left off, honestly.** `gates.lock` reads `phase 3: 9/9`. **Nobody has watched it print.** The whole phase was built between 16:15 and 20:12 UTC on 19 September and has not been run since; the Phase 3 review read the source and did not execute a command. Four of the nine items would print `ok` whether the code worked or not: item 8 counts list lines (F25), item 9 checks that the lock contains two substrings (F26), item 1 is item 6 with a different `Signals` (F26), and item 2 compares `describe` with itself under a variable named `cli_d` that never touches `joinn-cli` (F27). **P5-00 is the run, and it is the first commit for the same reason P21-00, P22-00 and P3-00 were: the repo is made honest before it is extended.** What landed is not in doubt — `Description` is a value, `vocab` enforces IO and construction by path across a tree, `joinn-run` is gone and the transcript moved first. The protocol is sound. The instrument that judges it is not.

---

## 0. How to Drive Cursor With This Plan

| | |
|---|---|
| **Where this file lives** | `D:\JoInn\docs\Plans\JoInn Phase 5 Implementation Plan.md`, beside the Phase 3 plan |
| **Where the code lives** | `D:\JoInn\joinn\` — the existing repo, extended by two crates |
| **What it closes** | The roadmap's Phase 5; F25–F32 of `docs/Findings/phase-3-review.md`; R7 in part, R8's decided half, R11 and R12 begin |
| **Standing rules** | Appendix A. It **replaces** the current `AGENTS.md` and `.cursor/rules/joinn.mdc` |
| **Unit of work** | one numbered commit from §4. One Cursor session per commit |
| **How a commit ends** | its **done-when** line is checkable by a command, and that command reports a refusal, a disagreement, a compile error, or a byte-identical replay — never a number the command also chose |
| **What Cursor may decide** | module layout inside a crate (subject to rule 25), function bodies, test names, error strings, the in-memory representation of a complex or an incidence table |
| **What Cursor may not decide** | anything in §2, the `.universe` grammar (§3.4), whether a membrane is declared or derived (§2.5), crate boundaries and their order (§3.1), the ordering of P5-01 and P5-02 before P5-24, or whether a check is worth having an artifact for an opposite |

The prompt stays short:

> Implement commit **P5-13** from `docs/Plans/JoInn Phase 5 Implementation Plan.md`. Follow the rules in `AGENTS.md`. Stop when the done-when command passes and report what it printed.

**The failure mode this phase must not have.** Phase 2 shipped checks that could not fail. Phase 2.1 shipped checks with empty domains. Phase 2.2 shipped a control that was a fact about the standard library. Phase 3 shipped a control that was the length of a list. The species is the same each time and it changes address every phase, and this phase's correction block is the first attempt to fix the *mechanism* rather than the instance. **Phase 5's address is the `.universe` file.** A new file kind is a new place to write down structure that already exists somewhere else, and the moment the membrane is written in the universe file as well as derivable from the body, the link checks become a comparison of one copy against another. §2.5 forbids that. Watch that one line.

**The correction block is not optional and is not last.** P5-01 and P5-02 give rule 24 a type. Every artifact-opposed item added later in this phase depends on them, and exit gate item 9 is a scan over gate tables that do not yet exist as tables. If the link work is going well and something has to be cut, cut §4's P5-21 or P5-22 — never P5-00 through P5-08.

---

## 1. Scope Fence

### In scope

- **The correction block** — F25–F32 of the Phase 3 review, eight commits, at the head.
- **A generic CLI session** — the host stops naming `cli_a`, `cli_b` and `sum`. Required by the exit gate, not debt repayment.
- **`joinn-assay`, in its ∂ form only** — a complex, a boundary operator, and the ∂∂ assembly. No homology, no H₀/H₁/H₂, no invariance harness, no declarations.
- **`joinn-link`** — hyperedge incidence in compressed sparse row form, order and tail/head marks, lenses as data, exclusive membership, capability flow across a system boundary.
- **The `.universe` file kind** — a sixth file kind, hashed, with hand-computed goldens.
- **A second body** — a small units body the calculator links to.
- **Law 4 and G6 as checks**, not as sentences.

### Out of scope — Cursor must refuse these even when they look small

| Not now | Phase | Why it is tempting |
|---|---|---|
| H₀, H₁, H₂, Betti numbers, the invariance harness, declarations | 4 | The complex will exist and the homology is fifty lines away. Part III §9.6's argument for the assay is a **cross-membrane** bug class; the calculator plus one units body is not deep enough to settle it, and settling it badly is worse than not settling it. See §1.1 |
| Any renderer, any pixel, `wgpu`, `winit`, `lyon`, any hyperedge **drawing** | 6+ | Part II §11.1 describes region, hub, bundle and spine in detail. That is the drawing of a thing this phase only has to get *true*. Order and tail/head marks are data here; the spine is Phase 6 |
| A third body, a galaxy with more than one system, the universe level | later | Two bodies prove a link the way two hosts proved a protocol. One galaxy, two systems, two lenses is the whole test matrix |
| Bodies nesting inside bodies | never | Part I §5.1. If a link seems to need it, it is two links and a body between them (§2.5) |
| A testimony corpus | 12 | `corpus/testimony/` stays empty. §2.8 of the Phase 2.2 plan still governs |
| Another primitive | never, without an amendment carrying a refuted reduction | `grant` is already on the floor (R11 asks whether it should be; that is a question, not a licence) |
| Adding to `grandfather.txt` | never in this phase | It has been empty for two phases. A file that cannot obey rule 25 is evidence about rule 25 |
| Storing a membrane in DNA | never | §2.5. This is the one that will be argued with, because deriving it twice looks wasteful |

### 1.1 Why Phase 4 comes after this phase

`docs/Findings/spikes/s1-assay.md` already did Phase 4's exit gate by hand, on the same complex, and wrote down its own objection:

> The complex is small: two independent 3-cycles sharing the return edge `sum → host`. That is exactly the "shallow complexes" cost in Part III §14. The finding that decides Phase 4 is real at this scale; it does not prove the assay will earn its keep on deeper bodies.

Phase 4's adversary is the decoration check — *does the assay catch anything `require` and `ensure` cannot?* — and Part III §9.6 names the candidate precisely: H¹ ≠ 0, a system where every law holds at every membrane and no consistent global state exists. **That is definitionally cross-membrane.** With one body there is no second membrane for the inconsistency to live between, so Phase 4 run now returns "inconclusive, complex too shallow" — which the spike said in a weekend.

The roadmap's own dependency graph has `P3 --> P5` as a hard edge and `P4 -.optional.-> P5`. Phase 5 needs nothing from Phase 4. D7 says Phase 4 is cut without ceremony; a phase that can be cut can be moved. **After this phase there will be two bodies, a link between them, and a membrane with a computed boundary — which is the smallest universe in which the decoration check can fire.**

---

## 2. Decisions Assumed by This Plan

Ten calls. The first two are the phase; the rest follow from them, resolve the Phase 3 review, or write down what a second body forces.

### 2.1 A forbidden connection is refused by ∂∂, not by a predicate · **PROPOSED · yours · this is the phase**

> **The touch-only law is checked by assembling the universe's complex and requiring ∂∂ = 0 on every block. A link whose member port is not on a membrane makes `∂(hyperedge) ⊆ ∪ ∂(body)` false, the assembly refuses, and the refusal names the port. V16 is the output of a measurement, not a rule written beside its own check.**

Part III §9.4 is the whole argument and it was made a year before there was anything to check:

> Merged, ∂∂ = 0 would hold by construction and therefore detect nothing. Derived, **the assembly can fail**, and failure is a finding — a link that is a second opening, a membrane that is not closed, a hyperedge that transits. V16 becomes a checkable output instead of a rule one hopes was followed.

Three things this buys.

1. **It is the first control in this repo that is neither a file nor a predicate but a computation over one.** F25's species — a control that is the same assertion written twice — cannot occur here: the check is `assemble(universe)` and the control is a `.universe` file that fails it. A reviewer reads the file; the machine does the arithmetic.
2. **It collects a price already paid.** Part III's table maps four decisions made for unrelated reasons onto one equation: only cells-in-body nests (∂ terminates), a fragment tests at most two membranes (bounded ∂-depth), touch-only (boundaries meet at boundaries), a transiting connection is two edges (a chain with a well-defined composite boundary). A GPU-clipping argument and a security argument landing on the same identity is the evidence worth having, and this is where it is collected.
3. **Phase 4 does not renegotiate.** The complex, ∂, and the assembly exist; Phase 4 adds H₀/H₁/H₂ over the same structure and a derivation from DNA. It inherits a boundary operator rather than inventing one under deadline.

**Cost to reverse:** V16 becomes a hand-written predicate over the incidence list, and F25's species finds a fifth address in the phase whose correction block exists to close it. Confined to `joinn-link`.

### 2.2 `joinn-assay` is ∂ over an abstract complex and knows nothing about DNA · **PROPOSED · yours**

The obvious shape is an assay crate that reads bodies and builds the complex itself. That shape puts `joinn-link` to the *left* of `joinn-assay` and then forces `joinn-assay` to depend on `joinn-link` to see hyperedges — a cycle.

> **`joinn-assay` defines `Block`, `Complex`, `Chain`, `boundary` and `assemble` over an incidence table of integers and signs. It depends on `joinn-frame` for `Verdict` and on nothing else. It does not know what a cell, a wire, a body or a hyperedge is. `joinn-link` builds a `Complex` from a universe and asks `assemble` the question.**

The derivation *from DNA* — ports → 0-blocks, wires → 1-blocks, laws → fillings — is Phase 4's deliverable and it goes in a second module of the same crate when Phase 4 opens. This phase writes only the part that has a caller.

**Cost to reverse:** merge the two and the crate order inverts. Cheap now; expensive once Phase 4's derivation exists.

### 2.3 The gate never depends on the assay · **PROPOSED · yours · decided now, used in Phase 4**

Part III §9.7 says a creator may write `assert H₁ = 0` into a coding region and *"the gate then checks the declaration against the assay result, exactly as it checks any other law."* Read against §3.1's crate order, that sentence inverts the whole workspace: `joinn-gate` sits fifth from the left and `joinn-assay` seventh.

> **An assay is injected into the gate the way natives already are. `joinn-gate` names no assay type, ever. Phase 4 adds an `AssayRegistry` injection point mirroring `NativeRegistry`; this phase adds nothing and only proves the shape is available.**

The precedent is already in the tree: `Gate::new(budget, natives)` takes its alleles from `joinn-prim`, which is to its right, and `register_cell_oracle` takes an `Arc<dyn Oracle>`. Nothing about the gate's design resists this. Deciding it now costs a paragraph; discovering it in Phase 4 costs a crate reshuffle under deadline, which is how `joinn-run` happened.

**Cost to reverse:** none. This is a subtraction of a future emergency.

### 2.4 One new file kind, `.universe` · **PROPOSED · yours**

Phase 2 put bodies in their own `.body` file kind so the `.cell` grammar and every existing cell hash stayed untouched. The same reasoning applies one level up, and the same reasoning forbids stuffing links into `.body`.

> **`corpus/**/*.universe` is a sixth file kind: a coding region holding `bodies`, `links` and `lenses`, a `---` separator, and a regulatory region holding names, labels and styles. It is hashed with tag `joinn.universe.v1` by the same hand-written canonical writer, with the hand-computed hash checked against the machine's. No `.body` or `.cell` file changes and no golden coding hash moves.**

A universe is a blueprint, so Law 5 applies to it: identity follows content, and two identical universes share a hash. Lens ordering and link ordering are therefore canonical-form questions, which is R54.

**Cost to reverse:** links move into `.body` and every body hash in the repo moves with the first link. Do not.

### 2.5 A membrane is ∂(body), and nothing declares it · **PROPOSED · yours · this is where the phase turns**

A link must name a port. Part I §5.3 says the body "owns its boundary." Nothing in the `.body` grammar says which ports are *at* that boundary, and the obvious fix — a `membrane { ... }` region — moves `calculator.body`'s golden hash the moment the calculator is linked to anything.

There is a better answer, and Part III §9.4 already wrote it: *∂(cell) is its ports — so a membrane **is** ∂ applied to a cell.*

> **A body's membrane is `∂(body)`: every port of every instance in its genome that no internal wire consumes. It is computed, never written. A port that an internal wire consumes is interior. A link member naming an interior port is reaching inside, and is refused naming the port.**

For the calculator, whose wires are `cli_a@1 -> sum@0` and `cli_b@1 -> sum@1`:

| Port | Consumed by an internal wire? | On the membrane |
|---|---|---|
| `cli_a@0` in | no | **yes** — this is where the host injects |
| `cli_a@1` out | yes, by `sum@0` | no |
| `cli_b@0` in | no | **yes** |
| `cli_b@1` out | yes, by `sum@1` | no |
| `sum@0` in | yes | no |
| `sum@1` in | yes | no |
| `sum@2` out | no | **yes** — this is where the result leaves |

`∂(calculator) = { cli_a@0, cli_b@0, sum@2 }`, derived, asserted by value in the gate, and **identical to the set of addresses the host has been injecting at since Phase 2 plus the one port the transcript prints.** The structure was already there; nothing had asked it the question.

Four consequences, all of them things the plan would otherwise have to invent:

- `intent_set` (F28) is *the in-ports of ∂(body)*, which is §2.2 of the Phase 3 plan stated correctly for the first time, and it stops being derived from `grants`.
- The touch-only control artifact writes itself: a universe naming `calc.sum@0` is reaching past the membrane into the wire between `cli_a` and `sum`.
- R7 — *is a lone cell an implicit body of one?* — becomes answerable: ∂ of a one-cell body is that cell's ports, and nothing special happens.
- No golden hash moves in this phase from the membrane at all.

**The temptation to refuse.** Deriving ∂(body) on every link check looks wasteful and the fix looks like caching it in the universe file. **That cache is the phase's failure mode** (§0): once the membrane exists in two places, the link check compares one copy with the other and refuses nothing. If ∂ is slow, R50 is where that goes.

**Cost to reverse:** a `membrane` region in `.body`, every existing body hash moves, and the check becomes a comparison of two copies of the same fact.

### 2.6 A body does not know its system; a lens does · **PROPOSED · yours · R8's decided half**

Part I §5.2 and Part II §10.4 are already **DECIDED**: within one lens every body belongs to exactly one system, and *"used by" is a link fact, not a membership fact.* The implementation question is where membership is written, and the wrong answer is a `system:` field on the body — which would make a body un-reusable across lenses and re-introduce the alias glyph Part II withdrew.

> **Membership lives in the `.universe` file's `lenses` region, one tree per lens. A body carries no system. Exclusivity is checked per lens: a body in two systems of one lens is refused naming the body and both systems. The same body in two systems across two lenses is accepted, and that acceptance is the item's control.**

**Cost to reverse:** membership moves into `.body`, every body hash moves, and V15 becomes unfalsifiable because the type forbids the violation. A rule the type enforces is a rule that never detected anything.

### 2.7 Inside a body is a wire; between bodies is a hyperedge · **PROPOSED · yours · Law 4 with teeth**

Law 4 — each container type defines one communication method, consistent across the platform — has been a sentence since Part I §6.1. It is checkable the moment there are two container types with two methods.

> **A wire whose endpoints are in different bodies is refused. A hyperedge all of whose members are in one body is refused. Both refusals name the container and the method it requires.**

This is the smallest true statement of Law 4 and it is the one the second body makes testable. It also closes a gap that would otherwise appear immediately: a `.universe` file is a natural place to write a convenient shortcut wire between two bodies, and there must be a refusal waiting for the first person who tries.

**Cost to reverse:** Law 4 goes back to being a sentence, and the first cross-body wire ships as a feature.

### 2.8 A refusal does not cross a membrane; a link reports its own · **PROPOSED · yours · G6**

G6 has been decided since the Phase 0 exit gate — *a `Refused` verdict does not cross a body boundary* — with the note "No Phase 1 consequence; the type is not designed to be serialized across a membrane." This is the phase with the consequence.

> **When a body refuses a message that arrived over a link, the far side receives the *link's* refusal — naming the link and the member port — and never the refusing body's own reason text. The inner reason is available to a host `probe` on that body and nowhere else.**

The check is an artifact comparison: the far-side refusal text must not contain a string that only the inner refusal produces. `corpus/phase5/controls/inner_reason.txt` holds that string, which makes the control a file a reviewer can read.

This also bears on R46 (*does a host owe a refusal a description?*): a link refusal is describable — it names a link and a port — while the inner reason is not available to describe. That is evidence for R46 rather than an answer to it.

**Cost to reverse:** refusals leak across membranes, G6 becomes advisory, and every body's internal vocabulary becomes part of its public contract.

### 2.9 The CLI becomes generic before the second body exists · **DECIDED**

Not a design decision — an ordering one, and the same shape as Phase 3's §2.5.

`joinn-cli::calculator_session` reads exactly three lines and names `cli_a`, `cli_b` and `sum` as string literals (F31). The Phase 5 exit gate requires the calculator *and* the units body to run under it. A generic session — load the universe, derive each body's ∂, prompt for each in-port in turn, present every fire — is the work, and it must land **before** the second body, with the five-line transcript byte-identical across the change, or the transcript's failure and the new body's failure are indistinguishable.

> **P5-09 makes the session generic and proves the transcript. P5-15 adds the units body. In that order, in separate commits, with `gate all` green between them.**

### 2.10 A control is a path on the item · **DECIDED, AJ, Sept 22 · F25**

Rule 24 said a control is an artifact the item points at. It was written into `AGENTS.md` and not into anything the build reads, and gate 3 item 8 — the item meant to enforce it — counts non-comment lines in a file the same commit wrote.

> **`GateItem` carries `control_artifact: &'static str`, a repo-relative path. `run_gate_table` resolves it against the workspace root and refuses the whole run, naming the item, if it does not exist. `joinn-gate` never opens the file — it carries the string; `xtask` opens it — so rule 15 holds unchanged.**

This is the third time the answer has been *make it a value*: `Drive::bound` became a `NonZeroU32`, `Seal::reference` became a `BodyRef`, `present` became a `Description`. A rule in prose is a habit; a rule in a struct field is a type.

**Cost to reverse:** rule 24 goes back to prose and the species finds a fifth address.

---

## 3. Phase 5 — Architecture

### 3.1 Crates

Two added, none deleted. The dependency rule is unchanged: each crate depends only on those to its left.

```
joinn-frame ─► joinn-dna ─► joinn-gate ─► joinn-prim ─► joinn-live ─► joinn-assay ─► joinn-link ─► joinn-host ─┬─► joinn-cli
                                                                                                                └─► joinn-test-host
```

| Crate | Owns | IO |
|---|---|---|
| `joinn-assay` | `Block`, `Complex`, `Chain`, `boundary`, `assemble`. Pure combinatorics over an incidence table. Depends on `joinn-frame` alone | **none** |
| `joinn-link` | The `.universe` file kind, hyperedge incidence in CSR, order and tail/head marks, lenses, exclusive membership, `∂(body)`, cross-boundary `grant`/`revoke`, Law 4, G6 | **none** |
| `joinn-host` | *moves right of `joinn-link`.* A host now presents a universe, not a body; `describe` gains a link-aware address | **none** |

**`joinn-host` moving is a real change and it has a price.** It gains two dependencies and recompiles; nothing in its source moves. The alternative is composing bodies and links in `xtask`, which puts the multi-body logic in the instrument rather than in the platform — and Phase 6 draws hyperedges, so the host learns about links either now with two bodies or later with a renderer attached. **PROPOSED · yours.**

`joinn-assay` sits where it does for readability, not for need: it depends only on `joinn-frame` and could sit second. Do not move it.

### 3.2 The complex, and what ∂ does this phase

```rust
/// A block of some dimension in an abstract complex.
pub struct BlockId(pub u32);

/// A signed formal sum of blocks of one dimension.
pub struct Chain { /* BTreeMap<BlockId, i32>, zero coefficients pruned */ }

pub struct Complex { /* blocks by dimension; per block, its boundary chain */ }

impl Complex {
    /// The boundary of one block, as a chain one dimension down.
    pub fn boundary(&self, b: BlockId) -> Verdict<Chain>;

    /// ∂∂ = 0 on every block, plus well-formedness. Refuses naming the block.
    pub fn assemble(&self) -> Verdict<()>;
}
```

`assemble` refuses in exactly three situations this phase, and each has an artifact:

| Refusal | Means | Artifact |
|---|---|---|
| a 1-block's endpoint is not a 0-block of this complex | a link names a port that is not on any membrane — *a boundary that is not a boundary* | `corpus/phase5/controls/transits.universe` |
| a 2-block's boundary chain is not closed (`∂∂ ≠ 0`) | a filling that does not fill | `corpus/phase5/controls/broken_chain.universe` |
| a block's boundary names a block of the wrong dimension | a malformed incidence table | `corpus/phase5/controls/bad_dimension.universe` |

**What ∂ does not do this phase.** No H₀, no H₁, no H₂, no ranks, no generators, no cycle reporting. `assemble` returns `Verdict<()>`. The moment it returns a number, Phase 4 has started.

### 3.3 The membrane, derived

```rust
/// Every port of every instance in the body's genome that no internal wire consumes.
pub fn membrane(body: &Body, cells: &BTreeMap<Hash, Cell>) -> Verdict<BTreeSet<PortAddress>>;
```

`PortAddress` is `(instance, position)` — the same shape as `joinn_host::Address`, and `Address` should be reused rather than duplicated. A port is consumed when it appears as either end of a `Wire` in `body.coding.wires`.

The gate asserts `membrane(calculator) == { cli_a@0, cli_b@0, sum@2 }` **by value**, and the control is the same body with one wire added, whose membrane is strictly smaller — also asserted by value, so the check is not satisfied by a function that returns everything.

### 3.4 The `.universe` grammar

```
universe {
  codex 1
  bodies {
    body:b55fba1eff65… as calc
    body:<units hash> as units
  }
  links {
    link e0 order none {
      calc.sum@2 tail
      units.scale@0 head
    }
    link g0 order ordered {
      calc.cli_a@0 tail
      units.scale@0 head
    }
  }
  lenses {
    lens function {
      galaxy app {
        system calculation { calc }
        system measurement { units }
      }
    }
    lens deployment {
      galaxy app {
        system local { calc units }
      }
    }
  }
}
---
regulatory {
  names { e0 "result to inches" calculation "Calculation" }
  labels { e0 "the sum, converted" }
}
```

Rules the parser enforces, each with a refusal that names what it found:

- A link member is `body.instance@position`, and the body alias must be declared in `bodies`.
- **A member port must be in `∂(that body)`.** Interior ports are refused naming the port (§2.5).
- `order` is `none` or `ordered`, declared, never inferred. An `ordered` link's members are a sequence; a `none` link's are a set and are printed sorted (Part II §11.1: *"style may not add an order; drawing a spine for an unordered edge states something false"* — the data half of V17).
- Each member is marked `tail`, `head`, or neither. Zero tails is a pure relation and is legal.
- A link with all members in one body is refused naming the body (Law 4, §2.7).
- Every body alias appears in exactly one system per lens (§2.6).
- Names, labels and styles are regulatory and live after the `---`.

**Canonical form.** Bodies sorted by alias, links sorted by id, members of an unordered link sorted, members of an ordered link **not** sorted, lenses sorted by name, systems sorted by name. Hashed as `joinn.universe.v1` with the length-prefixed tag rule of P0-03, no `serde`, no `#[derive(Hash)]`. Whether an ordered link's member order belongs in the hash is not in doubt — it does, because it is meaning — but whether *lens* order does is R54.

### 3.5 Incidence in compressed sparse row form

Part II §11.1 asks for CSR and it is the right shape for the renderer that arrives in Phase 6. This phase stores it and does not draw it:

```
link_offsets : Vec<u32>      // one entry per link, plus a terminator
members      : Vec<Member>   // Member { body: u16, instance: u16, port: u32, mark: Mark }
```

The CSR arrays are **derived from the parsed universe and never hashed** — the canonical text is the identity, the arrays are a representation. A test asserts that building CSR from a universe and printing it back yields the same canonical text, which is the same round-trip discipline `.cell` and `.body` already carry.

### 3.6 What the two containers must and must not share

| | **Body** (place graph) | **System** (link graph) |
|---|---|---|
| Communication | wire, out → in, both ends inside | hyperedge, any number of ports, touched never crossed |
| Declared in | `.body` `wires` region | `.universe` `links` region |
| May name an interior port | yes — that is what interior means | **no** (§2.5) |
| Refusal crosses it | yes, within the body | **no** (§2.8, G6) |
| Membership | a cell is in exactly one body | a body is in exactly one system **per lens** (§2.6) |
| Capability | `grants` in the body's coding region | rides an ordered hyperedge; revocable (§4, P5-19) |

---

## 4. Phase 5 — The Commit Plan

Twenty-five commits. **Done-when** is a command, and the command must be able to refuse.

### The correction block — F25 to F32

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P5-00** | **The tree as it stands, with a witness** | `gate all`, `power`, `agree`, `corpus verify`, `modules`, `vocab` run on the Windows machine; `docs/Findings/phase-3-run.md` records what each printed, typed by hand; `gates.lock` rewritten from that run | `phase-3-run.md` exists and quotes each command's last line. If any gate dropped, the lock records the drop and the commit is red — **that is the commit**. If `phase 3: 9/9` reproduces, the file says so and says which four items §0 expects to be hollow |
| **P5-01** | **A control is a path** | `GateItem` gains `control_artifact: &'static str`; `run_gate_table` resolves it against the workspace root; every existing item in gates 1, 2, 2.1, 2.2 names its artifact; `artifact_list.txt` deleted | `cargo xtask gate all` **fails naming the item** when one item's `control_artifact` is pointed at a path that does not exist, demonstrated then reverted; `joinn-gate` still names no `std::fs`, asserted by `vocab` |
| **P5-02** | **Gate 3 becomes a table, item 8 becomes a scan** | `gate_three()` rewritten as `&[GateItem]` through `run_gate_table`; item 8 walks every phase's table and resolves every `control_artifact`; its own control is a fixture table in `xtask/gate_fixtures/` with one unresolvable artifact | `cargo xtask gate 3` prints nine rows through `run_gate_table`; item 8 **fails naming the fixture item**; adding a tenth item with an unresolvable artifact to any gate fails the run |
| **P5-03** | **Item 9 reads the lock it is named for** | One score parser over `gates.lock`; item 9 runs `bad.lock` through it and requires a refusal | `gate 3` item 9 **fails** when `bad.lock` is substituted for the live lock, demonstrated then reverted; the item names the phase whose score is wrong |
| **P5-04** | **Items 1, 3 and 6 stop duplicating each other** | Item 1 runs the calculator under both hosts and compares results, **or** is deleted and the gate renumbered; item 3's control moves into `crates/joinn-host/host_fixtures/`, checked before the scan, and gains `vocab`'s IO half; item 6 keeps the signal check alone | No two items in `gate 3` share a check function, asserted by a test over the table; item 3 **fails** on the planted fixture and on a planted `std::fs`, each demonstrated then reverted; if item 1 was deleted, `docs/Findings/` records why |
| **P5-05** | **`intent_set` is the in-ports of ∂(body)** | `intent_set` computed from `membrane(body)` filtered to `Direction::In`; `corpus/phase5/two_in_ports.body` as the artifact | `intent_set(two_in_ports)` contains **both** positions, asserted by value; the old grants-derived implementation fails that test, demonstrated then reverted; `intent_set(calculator)` is unchanged at `{cli_a@0, cli_b@0}` |
| **P5-06** | **V61 builds a description** | `regulatory_label_moves_description_not_cell_hash` runs the body, prints a description, edits a regulatory label, prints again | The two printed descriptions **differ**, asserted by value on the differing field via `first_desc_diff`; the cell hash is equal; the test fails when the label edit is removed |
| **P5-07** | **A gate's score is returned by the gate** | `gate_one`/`gate_two`/`gate_two_one`/`gate_two_two`/`gate_three` return `(u32, u32)`; `gate_all` passes them to `write_lock`; the literals leave `gate_all.rs`; the `fns/write_lock.rs` exemption leaves `source_checks_scan_a_tree` | No file under `xtask/src` contains the literal `8/8`, `9/9` or `4/4`, asserted by a scan over the **tree**; `gate all` with one gate item forced to fail writes `n < total` for that phase and not a zero |
| **P5-08** | **The rest of F32** | `README.md` rewritten for phases 2.2, 3 and 5 and the current binary; `probe` reads `addr.port` or the port field is removed; a `trybuild` fixture for a writing `probe`; `environment.body` either wired to `Signals` or deleted with a dated finding saying which | `cargo test --workspace` includes the `trybuild` compile-fail for a writing `probe`; `README.md` names the current gates and no stale score; `corpus verify` still matches every golden |

### The generic host

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P5-09** | **The CLI stops naming the calculator** | `calculator_session` replaced by a generic session: load a body, derive `∂`, prompt for each in-port in declaration order, present every fire, present a refusal; `cli_a`, `cli_b`, `sum` appear nowhere in `joinn-cli` | `echo "two\n2\n3" \| joinn run calculator` prints the five lines **byte-identical** to `corpus/transcripts/calculator.txt`, indent included; a scan asserts `joinn-cli` contains no instance name from any corpus body; `gate 2`, `gate 2.1` and `gate 3` all pass |

### The assay, in its ∂ form

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P5-10** | **`joinn-assay`: the complex** | The crate; `BlockId`, `Chain`, `Complex`, `boundary`; `enforced.txt` gains `joinn-assay` | `joinn-assay`'s `Cargo.toml` names `joinn-frame` and nothing else, asserted by a test; `boundary` of a 1-block built by hand returns `head − tail`, asserted by value; the crate contains no `H`, `homology`, `betti` or `rank` identifier, asserted by a scan |
| **P5-11** | **∂∂ = 0, and it can fail** | `assemble`; three refusals per §3.2, each naming the block; three hand-built complexes as unit fixtures | `assemble` **refuses** each of the three malformed complexes naming the offending block, and accepts the well-formed one; the accepting case is the control and is asserted, not assumed |

### The link graph

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P5-12** | **The membrane is ∂(body)** | `membrane(body, cells)` in `joinn-link`; nothing added to `.body` | `membrane(calculator)` is exactly `{cli_a@0, cli_b@0, sum@2}`, asserted by value; the same body with one extra wire has a strictly smaller membrane, asserted by value; **`corpus/hashes.txt` is byte-identical to its previous contents** |
| **P5-13** | **The `.universe` file kind** | Parser, canonical printer, `joinn.universe.v1` hash; `corpus/phase5/universe.universe`; round-trip test | Parse → print → parse is byte-stable over three universes; **the hash matches the one you computed by hand**; a `.universe` naming an undeclared body alias is refused naming the alias |
| **P5-14** | **Hyperedges, order and marks** | `Link`, `Member`, `Mark`, CSR arrays derived from the parsed universe; `enforced.txt` gains `joinn-link` | CSR → canonical text round-trips byte-identically; an `ordered` link's member order survives the round-trip and an unordered link's members come back sorted; no CSR array is hashed, asserted by a scan for `hash(` in the CSR module |
| **P5-15** | **The second body** | `corpus/phase5/units.body` — a small units body over existing cells and frames, admitted by the gate like any other body; its hash in `hashes.txt`, hand-computed | `cargo xtask gate all` admits `units.body`; its hash matches the hand-computed one; it runs standalone under `joinn-cli` **and** under `joinn-test-host` with no code change |
| **P5-16** | **Touch-only, as ∂∂** | `joinn-link` builds a `Complex` from a universe and calls `assemble`; `corpus/phase5/controls/transits.universe` names `calc.sum@0` | `assemble` on `transits.universe` **refuses naming `calc.sum@0`**; `universe.universe` assembles; the refusal says the port is interior and names the wire that consumes it |
| **P5-17** | **Lenses and exclusivity** | Lens trees parsed; V15 checked per lens; `corpus/phase5/controls/two_systems.universe` | `two_systems.universe` is **refused naming the body and both systems**; `universe.universe`'s two lenses each place `units` in one system, and `units` appears in two different systems across the two lenses, accepted and asserted by value |
| **P5-18** | **Law 4** | A cross-body wire and a body-internal hyperedge both refused, each naming the container and the method it requires; `corpus/phase5/controls/wrong_container.universe` | Both refusals fire on the one control file and each names its container; the well-formed universe still assembles |
| **P5-19** | **Grant and revoke across a system boundary** | A capability rides an ordered hyperedge; `revoke` removes it; the receiving body's next attempt is refused naming the capability | The receiving body succeeds before `revoke` and is **refused naming the capability** after, in one test, in that order; the pre-revoke success is the control and is asserted |
| **P5-20** | **Refusal locality** | A body's own refusal reason does not cross the link; the far side gets the link's refusal naming the link and the member; `corpus/phase5/controls/inner_reason.txt` | The far-side refusal text **does not contain** the string in `inner_reason.txt`, asserted by value; a `probe` on the refusing body does reach the inner reason, asserted by value |

### Close

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P5-21** | **Perf, two bodies** | `xtask perf` gains a fifth and sixth probe: the two-body universe end to end, and `membrane()` over the corpus | `cargo xtask perf` prints six probes and **writes no file**; the conclusion is typed by a human into `live-engine-performance.md`; if `membrane()` is more than a few milliseconds over the corpus, R50 is opened with a number |
| **P5-22** | **The adversary, on purpose** | The roadmap's Phase 5 adversary: write a third small body that wants a communication method neither container offers, and see whether Law 4 bends | Either the body is expressible with a wire and a hyperedge (Law 4 holds, and that is the finding), or it is not, and `docs/Findings/law-4-underspecified.md` records what the body wanted, hand-typed, with the adversary marked **fired** |
| **P5-23** | **Re-freeze** | New goldens for `units.body`, the universes, `two_in_ports.body`, and anything P5-09 moved; a dated finding per deliberate move | `cargo xtask corpus verify` matches every golden; `docs/Findings/phase-5-hashes.md` records each new hash and confirms **no Phase 0–3 coding hash moved** |
| **P5-24** | **Exit gate 5 and the lock** | `cargo xtask gate 5` as a table of artifact-opposed items; `gates.lock` restored | `cargo xtask gate all` runs phases 0, 1, 2, 2.1, 2.2, 3 and 5 from a clean checkout, offline, writes the lock from **returned** per-gate scores, **and fails when any item's control passes** |

**Ordering notes.** P5-00 may well be red and that is information, not a setback. **P5-01 and P5-02 are not negotiable and are not cuttable** — every artifact-opposed item added after them depends on the field they add, and exit gate item 9 is a scan over tables that P5-02 creates. P5-05 depends on P5-12's `membrane`, so either P5-12 moves before P5-05 or P5-05 computes the in-port set inline and P5-12 refactors it; **prefer moving P5-12 earlier** if Cursor asks. P5-09 before P5-15 is §2.9. P5-10 and P5-11 must both land before P5-16, because a touch-only check written before `assemble` exists will be written as a predicate and then never replaced. P5-22 is scheduled before the re-freeze so a finding can still change the shape.

**The gate number.** There is no gate 4. `gates.lock` will read `phase 3` then `phase 5`, and Phase 4 inserts between them when it runs. The lock is a record of gates that passed, not a sequence, and `write_lock` already takes its rows from its caller.

---

## 5. Test Strategy

### 5.1 New invariants

Continuing from V69.

| # | Invariant | Test | Commit |
|---|---|---|---|
| **V70** | Every gate item's control resolves to a file on disk | `run_gate_table`, with the fixture table | P5-01, P5-02 |
| **V71** | A gate's score is returned by the gate, never written beside it | a tree scan for literal scores | P5-07 |
| **V72** | `intent_set` is one address per in-port of `∂(body)` | `two_in_ports.body`, by value | P5-05 |
| **V73** | A regulatory edit moves a description and never a cell hash | both halves, by value | P5-06 |
| **V74** | A body's membrane is `∂(body)` and is stored nowhere | by value, plus a scan for a `membrane` field in `.body` | P5-12 |
| **V75** | A universe assembles: `∂∂ = 0` on every block | `assemble`, with three malformed fixtures | P5-11 |
| **V76** | A hyperedge touches ports on membranes only; an interior member is refused naming the port | `transits.universe` | P5-16 |
| **V77** | Within one lens every body has one owning system and every system one galaxy | `two_systems.universe` | P5-17 |
| **V78** | One body appears in two systems across two lenses | `universe.universe`, by value | P5-17 |
| **V79** | Inside a body is a wire; between bodies is a hyperedge; each other form is refused | `wrong_container.universe` | P5-18 |
| **V80** | An order is declared, never inferred; an unordered link round-trips sorted | the CSR round-trip | P5-14 |
| **V81** | A capability crossing a system boundary can be revoked, and the next attempt is refused naming it | before-and-after, one test | P5-19 |
| **V82** | A refusal does not cross a membrane | `inner_reason.txt` | P5-20 |
| **V83** | `joinn-gate` names no assay type | a scan | P5-10 |
| **V84** | `joinn-assay` names no DNA type and computes no homology | `Cargo.toml` and an identifier scan | P5-10 |

Carried forward and re-run every commit: V18, V19, V20, V24-embryo, FO1–FO10, the three canonical-text properties, the 1 000-append hash-stability test, V33–V69, and Phase 1's four demos.

### 5.2 The numbers that matter

Four now, and the fourth is this phase's contribution.

1. **`cargo xtask power`** — unchanged. Every mutant names the check that caught it.
2. **`cargo xtask agree`** — unchanged, plus P3-03's standing obligation: a separating sample of 0 or 1 for every seal is a finding, not a tuning problem.
3. **The description diff** — unchanged, and now reused by V73.
4. **The assembly refusal.** `assemble` names **the block**, and `joinn-link` translates that block back into a port address before the refusal reaches the user. A refusal that says `calc.sum@0 is interior: consumed by wire cli_a@1 -> sum@0` is a bug report. A refusal that says `∂∂ ≠ 0` is a shrug, and it is the same lesson as `BLIND SEAL` and `first_desc_diff` arriving at a third instrument. **This is R43 becoming a habit rather than an accident, and it is worth a rule** (Appendix A, rule 35).

### 5.3 What Phase 5 deliberately does not test

**How a link looks.** No region, hub, bundle or spine. No geometry, no routing, no crossing minimisation. Part II §11.1 describes all four forms in detail and none of them is drawn until Phase 6. Order and tail/head marks are stored and checked; nothing turns them into a picture.

**Whether the assay is worth having.** That is Phase 4's adversary and it needs a deeper complex than this phase builds. ∂∂ earns its place here on the touch-only law alone — it is on the never-cut list for that reason — and nothing in this phase argues for or against homology.

**Runtime ordering across bodies.** An ordered hyperedge *declares* an order; R11 asks how the owning container enforces it at runtime, and this phase does not answer. The data is checked; the scheduler is not built.

---

## 6. Dependencies and Forbidden Constructs

**No new third-party dependencies.** Everything forbidden in Phases 1 through 3 stays forbidden, with seven additions.

| Newly forbidden | Why |
|---|---|
| A membrane stored anywhere | §2.5. Two copies of a fact make the check a comparison |
| A `system` or `lens` field on a body | §2.6. A rule the type enforces never detected anything |
| `joinn-gate` naming an assay type | §2.3 |
| `joinn-assay` naming a DNA type, or any homology identifier | §2.2, §1. `H`, `homology`, `betti`, `rank` are Phase 4 words |
| A wire between bodies, or a hyperedge inside one | §2.7 |
| A refusal reason crossing a membrane | §2.8, G6 |
| A gate item without a `control_artifact` | §2.10. This one is a compile error, which is the point |

And the standing ones worth repeating because this phase will want them: no new line in `grandfather.txt`; no `mod.rs`; no check scoped to one file that is not its subject; no golden reblessed; no primitive added.

---

## 7. Exit Gate 5, As a Checklist

Scripted and repeatable, from a clean checkout, offline. `cargo xtask gate 5` runs it as a table of artifact-opposed items — each line names both halves, and each control is a file that `run_gate_table` resolves before the item runs.

- [ ] **1 · Two bodies, one universe.** The calculator and the units body run linked, unchanged and un-recompiled, under `joinn-cli` and under `joinn-test-host`, and the result crosses the hyperedge. *Control:* `corpus/phase5/controls/transits.universe` is refused, naming `calc.sum@0` and the wire that consumes it.
- [ ] **2 · The membrane is measured.** `∂(calculator) = {cli_a@0, cli_b@0, sum@2}`, by value, and nothing in `.body` declares it. *Control:* the same body with one added wire has a strictly smaller membrane, by value.
- [ ] **3 · The universe assembles.** `∂∂ = 0` on every `.universe` in the corpus. *Control:* `broken_chain.universe` fails, and the failure **names the block**, not the file.
- [ ] **4 · Exclusivity holds.** A body in two systems of one lens is refused, naming the body and both systems. *Control:* `two_systems.universe`.
- [ ] **5 · Two lenses, one body.** `units` sits in two different systems across two lenses and is placed once per lens. *Control:* the same pair of systems inside one lens is refused.
- [ ] **6 · Law 4 is a check.** A cross-body wire and a body-internal hyperedge are both refused, each naming the container and the method it requires. *Control:* `wrong_container.universe`.
- [ ] **7 · A capability can be revoked.** The receiving body succeeds, `revoke` runs, the next attempt is refused naming the capability. *Control:* the pre-revoke success, asserted in the same test.
- [ ] **8 · A refusal stays home.** The far side of a link receives the link's refusal and not the refusing body's reason. *Control:* `inner_reason.txt` — the string that must not appear.
- [ ] **9 · Every control is an artifact.** Across gates 1, 2, 2.1, 2.2, 3 and 5, every item's `control_artifact` resolves to a file. *Control:* `xtask/gate_fixtures/` — a table whose item names a path that does not exist, which fails the run naming the item.
- [ ] **10 · The path of truth.** `cargo xtask gate all` runs phases 0, 1, 2, 2.1, 2.2, 3 and 5 and writes `gates.lock` from **returned** per-gate scores. *Control:* `bad.lock` through the same score parser makes the item fail, naming the phase whose score is wrong.

**End of Bootstrap 0.** When gate 5 passes, JoInn is a correct, verifiable, host-independent, multi-body platform with a text editor for a face. The Phase 3 plan claimed this line for itself; the roadmap puts it here, and the roadmap is right — one body is not a universe.

---

## 8. Risks Watched During This Phase

| Risk | Instrument | What to do when it fires |
|---|---|---|
| **The stated adversary · Law 4 is under-specified** | P5-22, run on purpose, before the re-freeze | The roadmap's own Phase 5 adversary: *"the container communication methods turn out to want to be different per app."* If the third body wants a bus the platform does not offer, that is a finding about Law 4, not about the app. Write `law-4-underspecified.md` and stop — renegotiating a law is unbudgeted and belongs in a plan |
| **New · the membrane gets cached** | §0's one line; V74's scan; item 2's control | The moment `∂(body)` is stored in the universe file "for speed", the link check compares one copy with another and refuses nothing. Delete the field; do not make it optional. If ∂ is genuinely slow, that is R50 and it gets a number first |
| **New · the `.universe` file becomes a drawer** | The parser's refusals; §3.4's list | A new file kind attracts everything that has no home: styles, layout hints, runtime config, a cached membrane. Each one that lands is a thing the body should have owned. Refuse by grammar, not by convention |
| **New · the correction block gets cut** | P5-01 and P5-02's position in §4 | It will look like debt while the link work is interesting. Exit gate item 9 cannot exist without it, and F25's species is four phases old. Cut P5-21 or P5-22 instead |
| **New · the second body is a fixture, not a body** | P5-15's done-when | A units body that only exists to be linked to will get written as two ports and a stub, and then V16 is tested against something that could not fail for any other reason. It must be admitted by the gate, run standalone under both hosts, and be a body a first-grader could have built |
| **New · ∂ grows a homology while nobody is looking** | P5-10's identifier scan | The complex will exist and H₁ is fifty lines away. Phase 4's whole value is that the decoration check gets asked against a deep enough body; asking it early and getting "inconclusive" spends the question |
| **Carried · the two hosts share a bug** | Nothing, and that is still the danger | Both hosts call one `describe`, so V60 is true by construction (F27). The second body does not change that. The mitigation remains ordering: the generic session (P5-09) lands before the body that needs it |
| **Carried · a control chosen weak** | §2.10's type; P5-01; R39 | An artifact control is readable, which is most of the defence. Now it is also resolvable, which is the rest |
| **Carried · the floor grows a habit** | The petition procedure; the amendment record | Unchanged. A hyperedge is not a reason to add a primitive, and a commit that tries has misread §1 |
| **3 · The live engine is too slow to build with** | `perf`, now six probes | Unchanged. The finding is written by a human, then the decision is made |

---

## 9. What This Plan Refuses To Do

| Refused | Because |
|---|---|
| Let a membrane be declared | §2.5. It is the phase |
| Compute a homology group | §1. Phase 4, after this one, against a body deep enough to answer the decoration check |
| Draw a hyperedge | §1. Phase 6, and Part II §11.1 has already described what will be drawn |
| Let the gate depend on the assay | §2.3. Deciding it late costs a crate reshuffle under deadline |
| Put a link in a `.body` file | §2.4. Every body hash in the repo moves with the first link |
| Put a system on a body | §2.6. A rule the type enforces never detected anything |
| Add the second body before the CLI is generic | §2.9. Two failures that cannot be told apart |
| Ship gate 5 before rule 24 has a type | §2.10, and §4's ordering note. Item 9 is a scan over tables that P5-02 creates |
| Nest a body inside a body | Part I §5.1. It is two links and a body between them |
| Add a line to `grandfather.txt` | §1. Two phases empty |
| Amend the floor | §1. The allowance was spent in 2.1 and has not been renewed |
| Rebless a golden hash | §6.2 of the Phase 0–1 plan. P5-23's moves are deliberate additions, recorded and dated — and **no Phase 0–3 coding hash moves at all** |

---

## 10. Open Items This Plan Creates

Continuing from R48.

| ID | Topic | Question |
|---|---|---|
| **R49** | Is a link admitted? | A body is admitted by the gate; a cell is admitted against its frame. A `.universe` is parsed, assembled and hashed — but is it *admitted*? If a link has a contract, what are its laws, and what witnesses would it carry? If it is only assembled, the platform has a hashed artifact that no gate judges, which is a new category and R32's question about what a declaration witnesses applies to it |
| **R50** | What does ∂ cost at scale? | §2.5 derives the membrane on every link check and refuses to cache it. At two bodies that is free. At Phase 7's cut, probing thousands of bodies per tick, it is not obviously free, and R10's snapshot budget and R48's probe cost are the same question wearing a third hat. Measured at P5-21; a number, then a decision |
| **R51** | Does an ordered hyperedge order runtime? | Part II §11.1: *an ordered hyperedge orders **access** — turn-taking, priority, delivery, passing a capability.* This phase stores the order and checks it is declared. R11 asks how the owning container enforces it when messages actually flow, and the answer interacts with rule 4 (delivery order is a function of the grant list, never of arrival time) |
| **R52** | Who owns a truly shared body? | R8's open half, now with data behind it. Lenses are files; a body's owning system is a line in a lens. When the choice is genuinely arbitrary — a units body everything uses — is a "shared kernel" system the answer, or is that the name for having no answer? |
| **R53** | Does a revoked capability leave a trace? | §2.8 says a refusal does not cross a membrane. §4 P5-19 says a revoked capability produces a refusal on the far side. Those are in tension: the far side must learn *that* it was refused without learning *why* the near side refused. Is "the capability named X is no longer granted" the near side's reason or the link's fact? R12's first real question |
| **R54** | Is a universe hash stable under lens reordering? | §3.4 sorts lenses by name in canonical form, which makes two universes with the same lenses in different written order share a hash. That is right if a lens set is a set. If lens *order* ever means something — a default view, a priority — the canonical form has already thrown it away, and unthrowing it moves every universe hash |

**Touchpoints with existing items.** **R7** is answered in part by §2.5: ∂ of a one-cell body is that cell's ports, so a lone cell is a body of one with no special case. **R8**'s decided half is implemented at P5-17; its open half becomes R52. **R11 begins** at P5-14 with declared order and becomes R51. **R12 begins** at P5-19 and becomes R53. **R14** is unchanged — `columns` is still the only signal. **R39** is unchanged. **R41** is unchanged and `corpus/testimony/` stays empty. **R43** is advanced again by §5.2's fourth number and is promoted to a rule in Appendix A. **R44–R48** are carried untouched; R48 merges into R50 if P5-21's number is large. **R35** remains the one open item that could retroactively unmake a Phase 2.2 decision, and it is not chased here either.

---

## Appendix A · `AGENTS.md` and `.cursor/rules/joinn.mdc` for Phase 5

Replace both files with this.

```markdown
# JoInn — standing rules

This repo is JoInn. Phases 2–3 built a universe that is true, and gave it a face
proved by there being two hosts. Phase 5 gives it a second body. Its one idea is
that A FORBIDDEN CONNECTION IS REFUSED BY A MEASUREMENT, NOT BY A PREDICATE, and
its turning point is that A MEMBRANE IS ∂(BODY). The build plan is
docs/Plans/JoInn Phase 5 Implementation Plan.md. Work one numbered commit at a
time. Do not start the next one.

There is no renderer, no homology, no third body and no compiler in this phase.
Phase 4 (the assay layer) runs AFTER this phase, against a universe deep enough
to answer its own decoration check.

## Hard rules

1. A refusal is a VALUE (`Verdict::Refused`), never a Rust `Err` and never a
   panic. `Result` is for host errors only: IO, malformed input, bugs.
2. Never `unwrap`, `expect`, or panic outside tests. `#![forbid(unsafe_code)]`.
3. No `f32`/`f64` anywhere. No `HashMap`/`HashSet` — `BTreeMap`/`BTreeSet` only.
4. No wall clock and no unseeded RNG in crates/. `xtask` MAY measure wall time;
   it cannot reach a hash, a canonical form, a sample or a refusal. Message
   delivery order is a function of the body's grant list, never of arrival time.
5. No `#[derive(Hash)]` and no `serde` derive on any DNA type, and none on a
   Description or a Universe. The canonical writer is hand-written and tested.
6. Never change a golden hash in corpus/ to make a test pass. If a hash moves,
   stop and report it. A hash that moves because an artifact was CORRECTED on
   purpose is recorded with a dated finding — that is not reblessing.
7. Never weaken, skip, `#[ignore]`, or delete a test to make a build green.
   Report the failure instead.
8. A primitive name must never appear in a coding region. Laws may name: the
   cell under definition (`self`), frame signature operations, and other coding
   regions BY HASH. A BODY may name primitives — that is what `prim:` is for.
9. Display names, literals, prompts, styles, labels, roles and layout are
   REGULATORY.
10. THE ENGINE NEVER RECURSES IN RUST. Nesting is an explicit activation stack
    with ONE global step budget.
11. A REFERENCE ALLELE IS A BODY. `Seal::reference` is a `BodyRef`. A reference
    implemented as a Rust `Oracle` is rejected on sight.
12. THE FLOOR IS IRREDUCIBLE AND OPPOSED, and it is defined in
    docs/Findings/the-floor.md. Never add to the floor. If a task seems to need a
    new primitive, first write it as a reference body over the existing floor.
    NEVER write "the minimal set" or the floor's size as a number. The count is
    derived by `cargo xtask floor`.
    `zero`, `succ` and `pred` are NOT floor primitives. They are ℤ's first three
    constructors and live in ℤ's signature as frame vocabulary.
13. `sub`, `subtract` and `minus` are not identifiers in this project.
15. `std::io` AND `std::fs` APPEAR ONLY IN HOST CRATES AND xtask. joinn-host is
    protocol and does none. joinn-test-host, joinn-assay and joinn-link do none.
    joinn-frame, joinn-dna, joinn-gate, joinn-prim and joinn-live do none.
    `cargo xtask vocab` enforces this by path.
16. A nested activation holds NO grants and can reach no capability.
17. NO INSTRUMENT WRITES ITS OWN FINDING. `xtask` prints numbers to stdout. No
    xtask subcommand writes a file under docs/Findings/.
18. EVERY MUTANT NAMES THE CHECK THAT CAUGHT IT, and registers a negative
    control the check must ACCEPT.
19. EVERY CHECK IS OPPOSED. A check declares a witness it must refuse and a
    witness it must accept, and fails the build if it cannot do both.
20. NO INSTRUMENT CHOOSES ITS OWN INPUTS. `Drive::bound` is a `NonZeroU32`.
    Every seal carries a COUNTERFEIT body that `agree` must separate from the
    reference at the declared bound; a counterfeit that is not separated is a
    BLIND SEAL and fails the build.
21. A SEAL NAMES THE CELL THAT DECLARES ITS SEALED ALLELE, found by lookup;
    zero matches and two matches both refuse.
22. A CORPUS CELL IS ONE THE GATE ADMITS, and an allele it carries is one the
    gate admits against it. The counterfeit corpus under
    corpus/phase22/counterfeit/ is refused ON PURPOSE and is never a golden.
23. AN ALLELE MAY NAME ONLY MATTER. `hash` and `resolve` are physics and are not
    in the allele-visible native set.
24. A CONTROL IS AN ARTIFACT, NOT A PREDICATE — AND IT IS A PATH. `GateItem`
    carries `control_artifact`, a repo-relative path that `run_gate_table`
    resolves before the item runs. An item whose control is an expression
    evaluated beside its check is not opposed; it is the same assertion written
    twice. joinn-gate carries the string and never opens the file.
25. MODULE.RS BESIDE MODULE/, NEVER MOD.RS. A leaf file holds at most one named
    production function. A capsule root (`foo.rs` with `foo/` beside it) holds
    types, `mod` declarations, `pub use` and getters. `lib.rs` is a facade with
    zero production functions; `main.rs` holds `main` only. `#[cfg(test)]` does
    not count. One `impl Trait for T`'s required methods may share a leaf.
    `allow(modules): <reason>` silences one file and a bare silencer fails.
    Crate-level `tests/` is exempt from the one-function rule but not from the
    mod.rs ban. `cargo xtask modules` enforces this, running its three fixtures
    BEFORE the scan. NEVER ADD A LINE TO grandfather.txt — a file that cannot
    obey the rule is evidence about the rule, and it goes in docs/Findings/.
26. NO CHECK IS SCOPED TO ONE FILE unless that file is its subject. A check that
    scans "the source" scans a directory tree.
27. A DESCRIPTION IS A VALUE, NOT TEXT. `Description` is constructed only in
    joinn-host. It carries no template, width, alignment or pre-rendered field.
    There is no `String`-returning render function in joinn-host.
28. A HOST NEVER INVENTS AN INTENT. The intent set is derived from the body —
    one address per in-port of ∂(body). A host emitting an intent outside it is
    refused naming the address.
29. A GATE'S SCORE IS RETURNED BY THE GATE. No literal gate score appears
    anywhere under xtask/src. `write_lock` receives counts; it does not choose
    them.
30. A MEMBRANE IS ∂(BODY) AND IS STORED NOWHERE. It is every port of every
    instance in the genome that no internal wire consumes, computed on demand.
    Nothing in a .body or .universe file declares it. If it is slow, that is a
    finding with a number in it, not a cache.
31. A LINK TOUCHES PORTS ON MEMBRANES ONLY. A member naming an interior port is
    refused naming the port and the wire that consumes it. This is checked by
    assembling the complex and requiring ∂∂ = 0, never by a predicate written
    beside the check.
32. INSIDE A BODY IS A WIRE; BETWEEN BODIES IS A HYPEREDGE. Each other form is
    refused naming the container and the method it requires.
33. A BODY DOES NOT KNOW ITS SYSTEM. Membership lives in a lens. Within one lens
    a body has exactly one owning system; across lenses it may differ.
34. THE GATE NEVER DEPENDS ON THE ASSAY. An assay is injected the way natives
    are. joinn-gate names no assay type. joinn-assay names no DNA type and
    computes no homology in this phase.
35. A REFUSAL NAMES WHAT ITS ACCEPTANCE WOULD HAVE LOOKED LIKE. `BLIND SEAL`
    names the bound and the size of the domain it admits; `first_desc_diff`
    names the differing field; an assembly refusal names the block and the port.
    A refusal that only says what failed is half a refusal.

## Definition of done

A commit is done when the plan's "done when" command passes and you have
reported what it printed. "It compiles" is not done. "The tests pass" is not
done if the test does not refuse anything. A number is not done if the code that
printed it also chose it. A refusal is not done if nothing was ever shown to it.
A control is not done if a reviewer cannot read it — OR IF THE BUILD CANNOT
RESOLVE IT. And a rule is not done while it lives only in this file: if it can
be a type, make it a type.
```

## Appendix B · Directory layout after Phase 5

```
D:\JoInn\
├── docs\
│   ├── Plans\
│   │   ├── JoInn Phase 2 Implementation Plan.md
│   │   ├── JoInn Phase 2.1 Implementation Plan.md
│   │   ├── JoInn Phase 2.2 Implementation Plan.md
│   │   ├── JoInn Phase 3 Implementation Plan.md
│   │   └── JoInn Phase 5 Implementation Plan.md        ← this file
│   └── Findings\
│       ├── the-floor.md            floor-vocabulary.md
│       ├── phase-2-review.md       phase-2.1-review.md
│       ├── phase-2.2-review.md     phase-2.2-hashes.md
│       ├── phase-3-review.md            ← F25–F32
│       ├── phase-3-hashes.md       present-leaks.md
│       ├── phase-3-run.md               ← new, P5-00: what the commands printed
│       ├── counterfeit-strength.md decisions.md
│       ├── law-4-underspecified.md      ← new IF the adversary fires, P5-22
│       ├── phase-5-hashes.md            ← new, P5-23
│       ├── surviving-mutants.md    canonical-form-changes.md
│       ├── turn-annotations.md     live-engine-performance.md
│       └── witness-refreeze.md
└── joinn\
    ├── corpus\
    │   ├── phase0\  phase2\  phase21\  phase22\  phase3\
    │   ├── phase5\                      ← units.body, two_in_ports.body,
    │   │   └── controls\                  universe.universe, and the five
    │   │                                  refused universes
    │   ├── descriptions\
    │   ├── testimony\                   (still only .gitkeep)
    │   └── transcripts\
    ├── crates\                          ← ten
    │   ├── joinn-frame  joinn-dna  joinn-gate  joinn-prim  joinn-live
    │   ├── joinn-assay\                 ← ∂ and the assembly. No DNA. No homology
    │   ├── joinn-link\                  ← .universe, hyperedges, lenses, ∂(body)
    │   ├── joinn-host\                  ← protocol. Now link-aware
    │   │   └── host_fixtures\           ← the render/IO controls, inside the scan
    │   ├── joinn-cli\                   ← bin joinn. Generic. Names no instance
    │   └── joinn-test-host\
    └── xtask\
        ├── module_fixtures\             ← the opposed fixtures for rule 25
        ├── gate_fixtures\               ← the unresolvable-artifact table, rule 24
        └── src\ ( fns\  modules\ )
```

## Appendix C · Glossary delta for code

| Theory term | Rust identifier | Notes |
|---|---|---|
| the boundary operator | `joinn_assay::Complex::boundary` | over an abstract incidence table. Knows nothing about DNA (§2.2) |
| ∂∂ = 0 | `Complex::assemble` | returns `Verdict<()>`. The moment it returns a number, Phase 4 has begun |
| a body's membrane | `joinn_link::membrane(&Body, &cells)` | `∂(body)`. Derived, never stored (§2.5) |
| touch-only | `assemble` refusing an interior member | V16 as an output, not a rule (§2.1) |
| the link graph | `joinn_link::Universe` | `.universe`, tag `joinn.universe.v1`, hand-computed hash |
| a hyperedge | `Link`, `Member`, `Mark` | order declared, never inferred; CSR derived and never hashed (§3.5) |
| a lens | `Lens` | a tree of galaxies → systems → bodies. Membership lives here, not on the body (§2.6) |
| one body, one system | exclusivity check per lens | V77. Across lenses it may differ — that is V78 |
| the body bus vs the system | wire vs hyperedge | Law 4 as two refusals (§2.7) |
| refusal locality | the link's refusal, not the body's | G6, with `inner_reason.txt` as the control (§2.8) |
| a control | `GateItem::control_artifact` | a repo-relative path `run_gate_table` resolves. Rule 24 with a type (§2.10) |

---

*JoInn Phase 5 Implementation Plan (Draft 0.1). Delivers the roadmap's Phase 5, closes F25–F32 of `docs/Findings/phase-3-review.md`, and brings ∂ and the ∂∂ assembly forward from Phase 4 because V16 is unenforceable without them. Phase 4 runs after this one. Builds on the Phase 2, 2.1, 2.2 and 3 Implementation Plans and Parts I–IV. Everything marked PROPOSED · yours is a recommendation made while writing this plan; Cursor implements what the roadmap, the grammar document and your decisions say, not what this plan prefers.*
