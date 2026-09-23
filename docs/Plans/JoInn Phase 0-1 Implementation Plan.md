# JoInn Phase 0–1 Implementation Plan

**The paper phase and the truth core · a working plan for Cursor**

*Roadmap Part IV §3–§5 turned into decisions, commits, tests and gates*

Author: AJ · Draft 0.1 · September 17, 2026

> Status tags follow Parts I–IV: **DECIDED**, **PROPOSED**, **OPEN**. Anything marked **PROPOSED · yours** is a recommendation made while writing this plan and is yours to overrule; Cursor implements whatever lands there, not whatever is written here.

> **What this is.** Roadmap Phase 0 (paper, gaps, spikes) and Phase 1 (the truth core: `joinn-frame`, `joinn-dna`, `joinn-gate`) as an executable build plan. Phase 1's four-part exit gate is the target; every commit below exists to move one part of it. Nothing beyond Phase 1 is in scope — no engine, no primitives, no host, no pixels.

---

## 0. How to Drive Cursor With This Plan

| | |
|---|---|
| **Where this file lives** | `D:\JoInn\docs\Plans\JoInn Phase 0-1 Implementation Plan.md`, beside the roadmap |
| **Where the code lives** | a new repo, `D:\JoInn\joinn\` — greenfield, D1 |
| **Standing rules** | Appendix A, pasted into `.cursor/rules/joinn.mdc` and `AGENTS.md` at the repo root. Cursor re-reads rules every request; it does not re-read this plan |
| **Unit of work** | one numbered commit from §5. One Cursor session per commit, not one session for the phase |
| **How a commit ends** | its **done-when** line is checkable by a command. If the command does not exist yet, building it is part of the commit |
| **What Cursor may decide** | module layout inside a crate, function bodies, test names, error strings |
| **What Cursor may not decide** | anything in §2, the grammar (§3.3), the canonical form (§3.4), naming (§3.2), crate boundaries (§4.1), or whether an invariant is worth enforcing |

The plan is written so that a Cursor prompt can be as short as:

> Implement commit **P1-07** from `docs/Plans/JoInn Phase 0-1 Implementation Plan.md`. Follow the rules in `AGENTS.md`. Stop when the done-when command passes and report what it printed.

**Do not hand Cursor the whole plan and say "build Phase 1."** The failure mode of this project is a plausible thing arriving fast; a coding agent is very good at producing exactly that. One commit at a time, each with a test that can refuse it, is Law 1 applied to the tooling.

---

## 1. Scope Fence

### In scope

Phase 0's decisions and spikes, and Phase 1's three crates: DNA in, verdict out. A library with no host, no engine, no pixels and no CLI beyond a gate runner.

### Out of scope — Cursor must refuse these even when they look small

| Not now | Phase | Why it is tempting |
|---|---|---|
| The minimal primitive set (`eq`, `succ`, `pair`…) | 2 | Phase 1 needs *something* to run for the laws to be checked. It uses native Rust alleles instead (§4.5) |
| The live engine, body bus, `join`, `grant` | 2 | Contracts carry a join policy field in Phase 1 (§2.6); nothing executes it |
| Seals, fold/unfold, reference vs sealed alleles | 2 | Phase 1 decides *what a seal must carry*, and stores it. It does not fold anything |
| `Turn`, and therefore `sub` | 2, gated on S5 | The law language is relational enough to allow it later (§4.4); no solver is written |
| Hosts, `present`, `probe` | 3 | — |
| The assay layer, ∂, homology, declarations | 4, gated on S1 | The coding region reserves a `declarations` list that is always empty in Phase 1 (§2.6) |
| Hyperedges, lenses, bodies, wiring | 5 | A Phase 1 blueprint is a single cell. There is no body |
| Anything wgpu, winit, or visual | 6+ | S2 and S6 are throwaway spikes in `spikes/`, never in the workspace |

If a Phase 1 task seems to require something in that table, the task is wrong, not the fence. Say so and stop.

---

## 2. Decisions Assumed by This Plan

These are the calls that had to be made for Phase 1 to be implementable at all. Six of them are Phase 0 gap answers taken from the roadmap; five are new, found while writing this plan. Each new one says what it costs to reverse.

### 2.1 Founding witnesses are hashed; testimony is not · **G2, roadmap-recommended**

Three distinct things, three homes. Cursor will conflate them unless told:

| | Founding witness | Allele witness corpus | Accumulated testimony |
|---|---|---|---|
| Lives in | coding region | the allele (payload) | a side store, `TestimonyStore` |
| Hashed | **yes**, part of identity | as allele payload only | **never** |
| Chosen by | the creator, at write time | whoever writes the allele | the runtime |
| Replayed | exhaustively, every gate run | exhaustively for that allele | sampled under a budget |
| Grows by | an evolution event | admitting a new allele | using the cell correctly |

Exit gate 1 depends on this split: `add@ℚ` arrives with its own witnesses, and the cell's hash does not move.

### 2.2 Port identity is (direction, frame, position); names are regulatory · **G1, roadmap-recommended**

A `PortDecl` carries an ordinal fixed at birth and never reused. Display names live in the regulatory region, keyed by position. Renaming is a silent mutation; adding a port is an evolution event.

**Added here:** a removed port's position is recorded as a **tombstone in the coding region**. It changes no behavior, so hashing it looks wrong — but the contract's promise is *this position will never mean anything else*, and a promise a future version must honor is identity by §2.2 of Part III. Cost to reverse: a corpus rehash, cheap until Phase 9. **PROPOSED · yours · R24**

### 2.3 A coding region may name frames and other coding regions, never primitives or alleles · **new**

V18 says no primitive name appears in a coding region, and Cursor will immediately ask how `a + b = b + a` is written without naming addition. The answer is that the law never names an implementation of `+`:

- `self` denotes **the cell under definition**, read at one of its out-ports. Commutativity is `self(a,b) = self(b,a)` where `self` is the relation the coding region is defining.
- **Frame vocabulary** is nameable, because the frame is already named in the coding region and is part of its identity. `ℤ.zero` is legal in a law; `int.add` never is.
- **Other cells are nameable by hash**, never by name: `c19e@sum(a,b)`. This is what lets `CliInput`'s law `parse(format n) = n` be written at all, and it keeps the reference stable under renaming.
- **Alleles are never nameable.** If a law could name an allele, the coding region would be pinned to an implementation and §2.1 of Part III recurs.

Cost to reverse: the law AST changes shape, so every hand-written blueprint is rewritten. Decide it in Phase 0 and do not revisit. **PROPOSED · yours**

### 2.4 A frame's signature must stay small, and the gate warns when a law pins `self` to one frame op · **new**

Following from §2.3: if the ℤ frame exposes `add` in its signature, a creator can write `self(a,b) = ℤ.add(a,b)` and the coding region stops being laws and becomes a definition. That is legal and sometimes correct, but it is exactly the collapse Part III §8 warns about, arriving through the front door.

Two consequences for Phase 1:

1. Frames expose a **minimal signature** — for ℤ: `zero`, `succ`, `pred`, `eq`, and nothing else. Anything richer is a fold, and folds are Phase 2.
2. The gate emits a **degeneracy advisory** when a coding region's laws determine `self` pointwise on the sampled domain. Advisory, never a refusal (an assay never refuses — Part III V25 — and this is an assay in spirit).

This is Risk #1 approached from the other side: §6.3's mutant corpus catches laws that are too weak, and the degeneracy advisory catches laws that have given up and become code. **PROPOSED · yours**

### 2.5 A cross-frame allele is admitted through the frame's embedding · **new, and it is what makes exit gate 1 work**

The contradiction Cursor will hit in commit P1-13: identity = **frame** + contract + laws + witnesses, yet `add@ℚ` must join a cell whose coding frame is ℤ *without moving the hash*. If admitting a ℚ allele changed the coding region's frame, the hash would move and the reference example would be a changelog again.

It does not have to. The coding region keeps naming ℤ. The **allele** names ℚ, and it is admitted iff:

| # | Obligation | Checked how |
|---|---|---|
| 1 | ℚ declares an embedding ι: ℤ → ℚ and a restriction ρ: ℚ ⇀ ℤ | G3's frame obligation set; present or the frame is not admitted |
| 2 | ρ ∘ ι = id on ℤ | sampled over ℤ's generators, plus every founding witness value |
| 3 | ι is a **signature homomorphism**: ι(zero_ℤ) = zero_ℚ, ι(succ x) = succ(ι x), and eq is preserved | sampled. **This is an addition to G3's list** — without it, a law stated in ℤ's vocabulary cannot be evaluated in ℚ at all |
| 4 | The restricted allele agrees: for sampled ℤ inputs, ρ(allele_ℚ(ι(x⃗))) = allele_ℤ-expected | sampled; a failure is the counter-example |
| 5 | Every founding witness replays through the restriction | exhaustive |
| 6 | Every law holds in ℚ on ℚ-samples, with the law's frame constants mapped through ι | sampled |

Check 3 is new (R20 asks whether G3's list is complete; this is one answer). Checks 4–6 are what "conservative extension" means mechanically instead of aspirationally.

**Consequence worth stating:** the frame named in a coding region is the frame the cell's *truth is judged in*, not the only frame it can run in. That sentence belongs in Part I §9.1 if you adopt this. **PROPOSED · yours**

### 2.6 Phase 1 carries fields it does not use · **new**

Every field added to the coding region later rehashes the entire corpus. Four are already known to be coming, and all four are cheap to reserve now:

| Field | Used in | Phase 1 value |
|---|---|---|
| `join_policy` on the contract | Phase 2, G5 | present and hashed; default `Refuse`; nothing reads it |
| `declarations: Vec<Declaration>` | Phase 4, gated on S1 | present and hashed; must be empty; a non-empty list is a refusal with "declarations are not implemented" |
| `retired_positions: Vec<u32>` | whenever a port is removed | present and hashed; usually empty (§2.2) |
| `codex: u16` — canonical-form version | forever | present, hashed, and `1` |

The last is the one Cursor will not think of. Without a format version in the hashed bytes, two different canonical forms can produce the same hash for different meanings; with it, a format revision rehashes everything. For Draft 0.1 the corpus is a dozen files, so **rehash-on-format-change is accepted** and the corpus's golden hashes are regenerated deliberately, never automatically. **PROPOSED · yours · new item R27**

### 2.7 The hash is taken over the canonical text · **new**

V27 wants a textual form; Law 5 wants a canonical form. Making them the same artifact is cheaper than maintaining two and is what §1.1 of the roadmap already implies ("it is the canonical form, printed").

> **hash(cell) = BLAKE3(domain_tag ‖ canonical_text_bytes)**

where canonical text is a strict normal form: deterministic member ordering, no comments, no display names, normalized literals, UTF-8 NFC, LF endings, no trailing whitespace, exactly one trailing newline.

The risk is real and must be guarded, not argued away: a formatting bug becomes an identity bug. Three property tests hold it (§6.1): `parse ∘ print = id`, `print ∘ parse ∘ print = print`, and `print` is invariant under every regulatory perturbation.

The alternative — hashing a tag-and-length-prefixed binary encoding, with text as a view — is more robust and costs a second artifact plus an agreement obligation between them under V32. If S4 shows canonical text is fiddly, take the alternative; it is a one-commit change confined to `joinn-dna::canon`. **PROPOSED · yours**

### 2.8 A refusal is a value, never a Rust `Err` · **new, and Cursor gets this wrong by default**

`Verdict<T> { Ok(T), Refused(Refusal) }` is JoInn's own type. `Result` is reserved for things that are genuinely errors in the host sense: file not found, malformed UTF-8, a bug. The moment a refusal becomes an `Err`, `?` starts throwing away counter-examples and the gate silently stops carrying the evidence that makes it worth having. This is in Appendix A as a hard rule.

### 2.9 Determinism is promoted to DECIDED · **G4, roadmap-recommended**

No wall clock, no unseeded RNG, no `HashMap` iteration in any path that can reach a hash, a canonical form, a sample, or a refusal. Sampling takes an explicit seed, and a refusal carries the seed that produced it so it can be replayed exactly. `BTreeMap` and `BTreeSet` only.

### 2.10 Join semantics default to refuse · **G5, roadmap-recommended**

Carried as a hashed field (§2.6), unread until Phase 2.

### 2.11 Refusals do not cross a body boundary · **G6, roadmap-recommended**

No Phase 1 consequence; recorded so the `Refusal` type is not designed to be serialized across a membrane.

---

## 3. Phase 0 — Paper, and the Four Spikes That Can Kill It

### 3.1 Division of labor

Phase 0 is yours. Cursor's role is to build the throwaway instruments that make your decisions cheap, and to write nothing that survives into Phase 1.

| Task | Who | Output |
|---|---|---|
| P0-01 Vocabulary freeze (R13) | **AJ** | a decision table in the roadmap |
| P0-02 Coding-region grammar (R2) | **AJ**, from the checklist in §3.3 | `docs/Plans/JoInn Coding Region Grammar.md` |
| P0-03 Canonical form and hash rule | **AJ**, using S4's findings | a section of the same document |
| P0-04 G1–G7 answers | **AJ** | adopted into Parts I–III |
| P0-05 The two hand-written blueprints + hand-computed hashes | **AJ**, checked by Cursor's S4 tool | `corpus/phase0/` |
| P0-06 Minimal primitive set frozen with one paragraph and one example each | **AJ** | Part III §4 revision |
| S1 assay hand-test | AJ builds the complex; **Cursor** computes | `spikes/s1-assay/` |
| S2 core-limits GPU spike | **Cursor** | `spikes/s2-gpu/` |
| S3 underdetermination spike | **AJ** | paper |
| S4 canonical form spike | **Cursor** | `spikes/s4-canon/` |
| S5 turn spike | **Cursor** | `spikes/s5-turn/` |
| S6 deep-zoom spike | **Cursor** | `spikes/s6-zoom/` |

**Throwaway means throwaway.** `spikes/` is not a workspace member, is excluded in the root `Cargo.toml` with `exclude = ["spikes"]`, and no crate in `crates/` may depend on anything in it. Spike code is deleted at the start of Phase 1; its *findings* are written into the docs, including the negative ones. Appendix A makes this a rule because "this spike code is actually pretty good" is how a weekend experiment becomes load-bearing.

### 3.2 P0-01 · Vocabulary freeze, and a cheap way to enforce it

Three meanings of *block* coexist today (k-block, Blockly block, the `blocks` genome linker in the biomimicry work), and *frame* is carrying three loads. Renaming after Phase 5 means rewriting documents, tests and a published vocabulary.

Decide each term, then make the decision mechanical: `cargo xtask vocab` greps the workspace for banned synonyms and fails the build. It costs an afternoon and it is the only thing that actually holds a vocabulary in place across a hundred Cursor sessions, because an agent will cheerfully write `schema`, `spec`, `type`, `signature` and `contract` for the same concept in one file.

| Concept | Recommended term | Banned in code and docs |
|---|---|---|
| Hashed part of DNA | `coding region` / `CodingRegion` | genotype (prose only), schema, spec |
| Unhashed part | `regulatory region` / `RegulatoryRegion` | metadata, style, config |
| Candidate implementation | `allele` | impl, variant, backend |
| Declared context of truth | `frame` | type, domain, context |
| Topological building block | `k-block` | block (bare), cell |
| Blockly-style UI block | `piece` — *a suggestion, to free up "block"* | block |
| Structural instrument | `assay` | analysis, metric, check |
| What the gate returns | `verdict`, `refusal` | error, failure, invalid |
| A recorded true result | `witness` | test case, example, fixture |

**PROPOSED · yours.** Cursor uses whatever this table says after you edit it.

### 3.3 P0-02 · The coding-region grammar

This plan does not choose your syntax. It specifies what the grammar must be able to say, what it must be unable to say, and the test that admits it — so that when you write it, Phase 1 starts the same day.

**The grammar must express, and Phase 1 must parse:**

1. A **frame reference**: a name plus a version or hash.
2. A **contract**: an ordered list of port declarations `(position, direction, frame, required)`, plus a `join_policy`, plus `retired_positions`.
3. **Require and ensure** predicates attached to ports, in the formula language of item 5.
4. **Laws**: named, universally quantified formulas over frame-typed variables.
5. A **formula and term language** with exactly these atoms (§2.3): variables, literals in a named frame, frame-signature operations, `self` applied at an out-port, another coding region applied at an out-port **by hash**. Connectives: `=`, `¬`, `∧`, `∨`, `→`, `∀`. Nothing else — no arithmetic sugar that hides a frame op, no `let`, no recursion.
6. **Founding witnesses**: tuples of input values by port position to expected output values by port position.
7. **Declarations**: a list, empty in Phase 1 (§2.6).
8. **Lineage**: the parent hash, zero or one.
9. **A regulatory region**, in the same file but physically separated and visibly so — a creator must be able to see at a glance which half moves the hash. Display names, literals, prompts, styles, allele-selection rules.
10. **Alleles**, each with: its frame, its own witness corpus, and its body — which in Phase 1 is `native "<registered name>"` and in Phase 2 becomes a cell.

**The grammar must be unable to express** an allele inside the coding region, a primitive name anywhere in the coding region, a display name inside the coding region, or a law that names an allele.

**Decisions the grammar forces, listed so none is discovered in week three:**

| # | Question | Note |
|---|---|---|
| 1 | One file per cell, or one file per genome? | Genome is body-local (Part I §8.1), but there is no body until Phase 5. One file per cell is the smaller decision |
| 2 | Are the two regions two files, two sections, or two blocks with a hard delimiter? | §2.7's hash covers only the coding region either way |
| 3 | Comments: allowed in the source form? | If yes, they are stripped by canonicalization and therefore cannot be round-tripped. Say so explicitly |
| 4 | Unicode in identifiers and frame names (`ℤ`, `ℚ`)? | Recommend yes, with NFC normalization mandatory |
| 5 | Integer literal forms: leading zeros, underscores, signs, radix? | Every one is a canonicalization case and the `"007"` problem in disguise |
| 6 | Are laws named? | Recommend yes — a refusal that says which law broke is worth far more than one that says a law broke. Names are part of the coding region here, unlike port names, because a law name is how a refusal is reported |
| 7 | How is a port referred to in a law: by position or by name? | Position, per G1. Names would be regulatory data leaking into hashed content |
| 8 | How is another cell referenced — full hash, or a short prefix? | Recommend full hash in canonical form, short prefix allowed in the source form and expanded on canonicalization |
| 9 | Witness format for multi-out-port cells | Needed now, even though Phase 1 cells have one out-port |
| 10 | Line endings, encoding, maximum line length | All three are hash-relevant under §2.7 |

**Acceptance test — S4 is the exit gate for the grammar, not a nice-to-have.** Write `CliInput` and `Sum` four ways each: different port display names, different law order, different literal spellings, different whitespace and comments. Canonicalize all eight. If the four variants of each do not produce **byte-identical** output, the grammar is not done. That is S4, and it is also commit P1-06's test fixture, which is why S4's spike code and Phase 1's canonicalizer are deliberately separate: the spike is allowed to be a hacked-together script whose only job is to tell you the grammar is wrong while the grammar is still free.

### 3.4 P0-03 · The canonical form and hash rule

Written in prose, byte-exact, before any code. It must state:

- The domain separation tag per hashable object (`joinn.cell.v1`, `joinn.allele.v1`, `joinn.witness.v1`). Never hash bare concatenations.
- The hash function and output length. **Recommend BLAKE3-256**, displayed as the first four hex characters in prose (`c19e`) and never truncated in storage.
- Member ordering: ports by position; laws by name; witnesses by a stated total order over values; everything else alphabetically by a stated collation (recommend byte-wise over the NFC UTF-8 encoding — locale-aware collation is a non-determinism source).
- Exactly what is stripped: comments, display names, all whitespace not required by the grammar, the entire regulatory region.
- Literal normalization per frame — which means **each frame owns the canonical printing of its own values**, and the DNA canonicalizer calls into the frame. Cursor will otherwise write integer formatting into `joinn-dna` and the ℚ frame will not fit it.
- What `codex` is and when it increments (§2.6).

### 3.5 The seven gaps, as code consequences

Answer them on paper, then hand Cursor this table.

| Gap | Answer assumed here | What Phase 1 does differently if you overrule it |
|---|---|---|
| **G1** ports | (direction, frame, position); names regulatory; tombstones hashed (§2.2) | If names are hashed: `PortDecl` carries a name, V24-embryo's rename test inverts, and the creator can never rename a port |
| **G2** witnesses | founding vs accumulated (§2.1) | If all witnesses are hashed: the cell's hash changes on every correct use; exit gate 1 becomes unreachable and Phase 1 stops |
| **G3** frame obligations | the full set, plus the signature homomorphism of §2.5 check 3 | Each missing obligation removes one check from the gate. A frame with no generator makes its laws uncheckable — that is the whole point of the list |
| **G4** determinism | DECIDED (§2.9) | Nothing in Phase 1 is reproducible, and a refusal cannot be replayed |
| **G5** join | declared, hashed, default refuse (§2.10) | Field type changes; no behavior changes until Phase 2 |
| **G6** refusal locality | local (§2.11) | No Phase 1 effect |
| **G7** the ease gate | scheduled from Phase 9 | No Phase 1 effect, but §6.3's concept count starts collecting the evidence now |

### 3.6 The spikes Cursor builds

Each spike gets its own directory, its own `Cargo.toml` (or script), a `FINDINGS.md` written by you, and no dependency in either direction with `crates/`.

#### S1 · `spikes/s1-assay/` — can kill Phase 4

**You** build the calculator's complex by hand (~15 blocks): ports as 0-blocks, wires as 1-blocks, laws as fillings, listed as an incidence table in a plain text file. **Cursor** writes a script that reads that table, assembles ∂₁ and ∂₂ over ℤ, checks ∂∂ = 0, and computes H₀, H₁, H₂ by Smith normal form — returning generators, not ranks.

Done when: with `parse(format n) = n` present, H₁ = 0. With it deleted, H₁ = 1 **and the returned generator is the cycle `host → cli_a → sum → host`**. A rank without a named cycle is a failure of the spike, because a count is useless to a creator.

Kill criterion: if getting there needs special pleading about orientations or basepoints, Phase 4 is cut and Part III §9.5–9.9 with it. Keep ∂∂ = 0.

#### S2 · `spikes/s2-gpu/` — can kill Part II §7

A throwaway binary: ~10k instanced SDF rounded rects driven entirely from a storage-buffer table, plus an integer ID target with async readback, under WebGPU **core** limits — 4 bind groups, 8 storage buffers per stage, storage buffers read in the vertex stage. Run it on the weakest device in the test set and in a browser, not only on the desktop.

Done when: it runs at interactive rates on the weak device, and the readback returns correct IDs for a randomized sweep of points. Findings record: actual limits reported, whether vertex-stage storage buffers were available, and the frame time.

Kill criterion: vertex-stage storage buffers unavailable in compatibility mode → per-instance data moves into vertex buffers, with no change to the upward wrap. Finding this in Phase 6 costs months.

#### S3 · paper — can kill D2/D3's usefulness

**Yours alone.** Write a coding region for `sort` that a gate can usefully judge, then for `format`. Cursor's only contribution is that after S3, the mutant corpus of §6.3 gets a `sort` entry if one is writable.

#### S4 · `spikes/s4-canon/` — can kill Law 5 as implemented

The eight files of §3.3, and a canonicalizer hacked together in any language. It must be *separate* from Phase 1's canonicalizer and may be thrown away entirely — its job is to be wrong cheaply while the grammar can still change.

#### S5 · `spikes/s5-turn/` — can kill `Turn`, gates Phase 2

A tiny propagator over the `Sum` relation {a, b, sum}: supply any two, get the third. Then push it — three-port laws, chained cells, two unknowns. **Count how often direction must be annotated by hand**, and report that number, not an impression.

Kill criterion: near-universal annotation means `Turn` is a naming convention in a structural costume; `sub` goes back to being a cell and `Turn` defers to R18.

#### S6 · `spikes/s6-zoom/` — can kill Part II §9

Chart chain, camera anchored to the deepest chart, integer zoom level plus fraction, rebasing on crossing. Zoom through twelve orders of magnitude in a 2D toy with nothing else in it. Findings record where jitter first appears and at which zoom level rebasing becomes necessary.

### 3.7 Phase 0 exit gate

A single text file you can hand to a stranger, containing:

- [ ] `CliInput` and `Sum` in canonical form, hand-written
- [ ] A hash for each, computed by hand against the rule in P0-03
- [ ] One paragraph per gap G1–G7: what was decided and why
- [ ] S1, S3, S4 findings written down, including negative ones
- [ ] S2, S5, S6 findings written down, including negative ones
- [ ] The vocabulary table, decided
- [ ] The minimal primitive set, frozen, one paragraph and one example each
- [ ] A yes/no on Phase 4, recorded, with the reason

Phase 1 does not start until every box is ticked. This is the only phase where being wrong is free.

---

## 4. Phase 1 — Architecture

### 4.1 Crates and the dependency rule

```
joinn-frame  ──►  joinn-dna  ──►  joinn-gate
     ▲                                  │
     └──────────────────────────────────┘
                 (frames, values)

xtask  ──►  all three (gate runner, corpus tools, vocab lint)
```

| Crate | Owns | Must not know about |
|---|---|---|
| `joinn-frame` | `Value`, `Term`, `Verdict`, `Refusal`, `Hash`, the `Frame` trait and its obligation set, the frame registry, the conformance harness, `Text`, `ℤ`, `ℚ` | DNA, laws, contracts, the gate |
| `joinn-dna` | coding region, regulatory region, contract, ports, the law and formula AST, witnesses, alleles, lineage, canonical text, hashing | the gate, evaluation, native implementations |
| `joinn-gate` | law evaluation, sampling, witness replay, the testimony store, the native allele registry, the four checks, refusals with counter-examples | nothing above it — it is the top of Phase 1 |

`Verdict` and `Value` sit in `joinn-frame` rather than in a fourth `joinn-core` crate, because the roadmap's crate table has three and a fourth crate is a deviation not worth its paperwork. `joinn-frame` is the root; everything else re-exports from it.

**One rule Cursor must not bend:** `joinn-dna` is *data*. It parses, prints, canonicalizes and hashes; it never evaluates anything and never holds a function pointer. The moment DNA can execute, the coding/regulatory boundary becomes a runtime concern and V18 stops being checkable by reading a file.

### 4.2 Values and frames

```rust
// joinn-frame — illustrative signatures, not final
pub struct FrameRef { pub id: FrameId, pub version: u32 }

/// The only term shapes in Phase 1. A frame gives them meaning.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Term { Int(BigInt), Text(String), Seq(Vec<Term>) }

/// Always frame-tagged, always canonical. There is no way to build a
/// non-canonical Value outside its frame.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value { frame: FrameRef, term: Term }

pub trait Frame: Send + Sync {
    fn reference(&self) -> FrameRef;
    fn signature(&self) -> &Signature;                              // op names and arities
    fn apply_op(&self, op: &OpName, args: &[Value]) -> Verdict<Value>;
    fn contains(&self, t: &Term) -> bool;
    fn canonicalize(&self, t: Term) -> Verdict<Value>;              // 2/4 → 1/2, "007" → 7
    fn eq(&self, a: &Value, b: &Value) -> bool;
    fn print(&self, v: &Value) -> String;                           // canonical literal text
    fn parse(&self, s: &str) -> Verdict<Value>;
    fn generate(&self, seed: u64, size: u8) -> Value;               // deterministic
    fn shrink(&self, v: &Value) -> Vec<Value>;
    fn extensions(&self) -> &[FrameRef];                            // frames this one extends
    fn embed(&self, from: &FrameRef, v: &Value) -> Verdict<Value>;  // ι
    fn restrict(&self, to: &FrameRef, v: &Value) -> Verdict<Value>; // ρ, partial
}
```

**The obligation set is a test, not a comment.** `joinn_frame::conformance::check(frame, seed, n) -> Report` verifies every obligation, and it ships in the same commit as the first frame — V32 applied to frames, which is the first place in the build where two things must agree:

| # | Obligation | Checked by |
|---|---|---|
| FO1 | Equality coincides with canonical identity: `eq(a,b) ⟺ canon(a) == canon(b)` | sampled pairs, plus shrunk pairs |
| FO2 | `parse(print(v)) = v` for generated values | sampled |
| FO3 | `canonicalize` is idempotent | sampled |
| FO4 | Generated values satisfy `contains` | sampled |
| FO5 | `shrink` terminates, never grows, never cycles | bounded walk |
| FO6 | Signature ops never panic; out-of-frame arguments return a refusal | sampled, including adversarial terms |
| FO7 | For each extension: `ρ(ι(x)) = x`; ι preserves every signature op and `eq`; ι is injective on samples | sampled over the narrower frame's generators |

FO1 is the one that pays for itself immediately: it is what lets the gate compare a ℚ result to an ℤ expectation without every check re-deciding what equality means.

**Generators are not an afterthought.** §6.3 shows two mutant implementations that survive a weak generator and die against a good one. An ℤ generator must produce: 0, ±1, values near `i32::MAX` and `i64::MAX`, values well beyond both, and negatives at the same rate as positives. Write that in the frame, not in the tests.

### 4.3 DNA

```rust
// joinn-dna — illustrative
pub struct Cell {
    pub coding: CodingRegion,          // hashed
    pub regulatory: RegulatoryRegion,  // never hashed
    pub alleles: Vec<Allele>,          // payload; each hashed on its own
    pub lineage: Option<Hash>,
}

pub struct CodingRegion {
    pub codex: u16,                    // canonical-form version, §2.6
    pub frame: FrameRef,               // the frame truth is judged in, §2.5
    pub contract: Contract,
    pub laws: BTreeMap<LawName, Law>,
    pub founding: Vec<Witness>,        // §2.1
    pub declarations: Vec<Declaration>,// empty in Phase 1
}

pub struct Contract {
    pub ports: Vec<PortDecl>,          // ordered by position
    pub retired: Vec<u32>,             // tombstones, §2.2
    pub join_policy: JoinPolicy,       // Refuse | Latest | Queue — unread in Phase 1
    pub require: BTreeMap<u32, Formula>,
    pub ensure:  BTreeMap<u32, Formula>,
}

pub struct PortDecl { pub position: u32, pub direction: Direction, pub frame: FrameRef, pub required: bool }
```

**The separation is enforced by the type system, not by convention.** The rule is that it must be impossible to *accidentally* hash a style:

```rust
mod sealed { pub trait Sealed {} }

/// Implemented only for coding-region types, only inside this module.
pub trait Genotype: sealed::Sealed { fn encode(&self, w: &mut CanonWriter); }

pub fn hash<G: Genotype>(g: &G) -> Hash;   // the only way to obtain a Hash
```

`RegulatoryRegion` does not implement `Sealed`, so `hash(&cell.regulatory)` does not compile — and a `trybuild` compile-fail test asserts exactly that, so a future refactor that makes it compile fails the build. This is commit P1-05's real deliverable; the structs are the easy part.

### 4.4 Laws

The formula language of §2.3 and §3.3, as an AST:

```rust
pub enum Term_ {
    Var(VarId),
    Lit(Value),
    FrameOp { frame: FrameRef, op: OpName, args: Vec<Term_> },
    SelfAt   { out: u32, args: BTreeMap<u32, Term_> },              // the cell under definition
    CellAt   { cell: Hash, out: u32, args: BTreeMap<u32, Term_> },  // another coding region, by hash
}

pub enum Formula {
    Eq(Term_, Term_),                      // equality in the term's frame
    Not(Box<Formula>),
    And(Vec<Formula>), Or(Vec<Formula>),
    Implies(Box<Formula>, Box<Formula>),
    ForAll { vars: Vec<(VarId, FrameRef)>, body: Box<Formula> },
}
```

That is the whole language. No arithmetic sugar, no `let`, no recursion, no existentials in Phase 1. `Sum`'s laws become:

| Law | Formula |
|---|---|
| identity | `∀ a:ℤ. self@2(0→a, 1→ℤ.zero) = a` |
| commutative | `∀ a b:ℤ. self@2(0→a, 1→b) = self@2(0→b, 1→a)` |
| associative | `∀ a b c:ℤ. self@2(0→self@2(a,b), 1→c) = self@2(0→a, 1→self@2(b,c))` |
| opposition | `∀ a b:ℤ. self@2(0→a,1→b) = s → self@0(…) = a` *(needs Turn; deferred — in Phase 1 it is written as a two-cell law via `CellAt` or omitted with a note)* |

`SelfAt { out }` is written with the out-port position, which is what keeps the door open for Turn: a relational reading needs to ask for a different face of the same law set, and a formula that already names which face it is reading does not have to be rewritten when S5 lands.

**Evaluation** takes an oracle, which in Phase 1 is a native Rust allele:

```rust
// joinn-gate
pub trait Oracle { fn apply(&self, inputs: &BTreeMap<u32, Value>) -> Verdict<BTreeMap<u32, Value>>; }

pub fn check_law(law: &Law, frame: &dyn Frame, oracle: &dyn Oracle, budget: &Budget)
    -> Verdict<LawReport>;
```

`ForAll` draws `budget.law_samples` tuples from the frame's generators using `budget.seed`, evaluates the body, and on failure **shrinks** the binding before reporting. A refusal carries the shrunk binding and the seed, so it replays exactly. A law that produces `0` samples (an empty generator) is itself a refusal — an uncheckable law must not pass silently, which is the single most likely way "the gate accepts anything" happens by accident.

### 4.5 Alleles, and why Phase 1 has no engine

Laws are checked by *running* an implementation, and Phase 2 owns the engine. Phase 1 therefore runs Rust:

```rust
pub struct Allele { pub frame: FrameRef, pub body: AlleleBody, pub witnesses: Vec<Witness> }
pub enum AlleleBody { Native(NativeId) }          // Phase 2 adds Dna(BodyRef)

pub struct NativeRegistry { /* NativeId → Box<dyn Oracle> */ }
```

The allele's *declaration* — its frame, its `NativeId`, its witness corpus — is hashed as payload. Its Rust body is not: it is referenced by a registered name, resolved at gate time. This is deliberately the same shape a seal takes in Phase 2 (`reference allele` beside `sealed allele`), so Phase 2 adds a variant rather than reworking the model.

**What this buys and what it costs.** It buys a Phase 1 that can actually run its exit gate with no primitives and no engine. It costs one honest admission: in Phase 1 nothing proves that the *DNA* is executable, only that the coding region can judge something. Phase 2's first act is to register a DNA-backed oracle and re-run every Phase 1 gate unchanged, which is V28 doing its job.

### 4.6 The gate

```rust
pub struct Gate { frames: FrameRegistry, natives: NativeRegistry, testimony: TestimonyStore, budget: Budget }

impl Gate {
    pub fn admit_allele(&self, cell: &Cell, allele: &Allele) -> Verdict<Accepted>;
    pub fn admit_cell(&self, proposed: &Cell, parent: Option<&Cell>) -> Verdict<Accepted>;
}
```

| # | Check | Module | What refuses |
|---|---|---|---|
| 1 | **Contract** — ports well-formed, positions unique and not retired, frames registered, require/ensure well-typed, declarations empty | `check::contract` | a malformed or unimplementable coding region |
| 2 | **Laws** — every law holds on sampled inputs in the allele's frame | `check::laws` | an allele that breaks a stated property, with a shrunk counter-example |
| 3 | **Witnesses** — every founding witness replays exhaustively; accumulated testimony is sampled under the budget | `check::witnesses` | an allele that changes a recorded answer |
| 4 | **Extension** — accepts ⊇ old, guarantees ⊆ old, and the cross-frame obligations of §2.5 | `check::extension` | a non-conservative change, including an added in-port |

Check 4's subtyping is **sampled refutation, not proof** (Part I §9.5): for inputs the old contract accepted, the new one must accept; where the new one produces a result, the old `ensure` must still hold. Cursor must not be allowed to write this as though it decided implication — the report says *refuted* or *not refuted at this budget*, and the budget is printed.

```rust
pub struct Refusal {
    pub check: CheckId,
    pub subject: Subject,               // cell hash, allele id, law name
    pub reason: String,
    pub counterexample: Option<CounterExample>,
    pub seed: u64,                      // replays exactly, §2.9
}
```

A refusal without a counter-example is allowed only for check 1 and for structural impossibilities. Everywhere else, **a refusal that cannot show the creator what broke is a bug in the gate**, and there is a test asserting it for each check.

### 4.7 The testimony store

Append-only, keyed by cell hash, one canonical-text record per line, under `corpus/testimony/<hash>.log`. It is never hashed and never affects identity (§2.1). Replay is sampled with a budget so the gate's cost does not grow without bound as testimony accumulates — the second reason G2's split exists, and the one that shows up as a wall-clock number rather than as a contradiction.

---

## 5. Phase 1 — The Commit Plan

Seventeen commits. Each is one Cursor session. **Done-when** is a command; if it does not exist, building it is part of the commit.

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P1-00** | Scaffold | Workspace with the three crates plus `xtask`; `spikes/` excluded; `AGENTS.md` and `.cursor/rules/joinn.mdc` from Appendix A; rustfmt, clippy `-D warnings`, `#![forbid(unsafe_code)]` in every crate; CI running fmt, clippy, test, `xtask vocab`; empty `gates.lock` | `cargo xtask gate all` prints `phase 1: 0/4 (pending)` and exits 0; CI is green |
| **P1-01** | Core values | `Term`, `Value`, `FrameRef`, `Verdict`, `Refusal`, `Hash`, `CanonWriter`. `Value` constructible only through a frame | `cargo test -p joinn-frame`; a test asserts no public constructor bypasses `Frame::canonicalize` |
| **P1-02** | The `Frame` trait and its harness | Trait, `Signature`, `FrameRegistry`, and `conformance::check` covering FO1–FO7. Ships with a deliberately `BrokenFrame` in tests | The harness **refuses** `BrokenFrame` for each of FO1, FO2, FO3, FO5, FO7, naming the obligation |
| **P1-03** | `Text` frame | Full obligation set. Generators include empty, whitespace, NFC/NFD pairs, astral-plane characters | `conformance::check(Text)` passes at 10 000 samples |
| **P1-04** | `ℤ` frame | Arbitrary precision; minimal signature `{zero, succ, pred, eq}` (§2.4); generators per §4.2 | `conformance::check(Int)` passes; a test asserts generated values exceed `i64::MAX` at the default size |
| **P1-05** | DNA model + the seal | All structs of §4.3; `Genotype`/`Sealed`; `hash<G: Genotype>`; `trybuild` compile-fail test | `cargo test -p joinn-dna` passes **and** `hash(&regulatory)` is a compile error proven by the trybuild case |
| **P1-06** | Canonical text | Printer and parser for the P0-02 grammar; NFC, LF, ordering per P0-03; the S4 corpus imported as fixtures | Four spellings of `CliInput` and four of `Sum` canonicalize to byte-identical output; `parse∘print = id` and `print∘parse∘print = print` pass as properties |
| **P1-07** | Hashing | Domain tags, `codex`, `hash(&CodingRegion)`; goldens in `corpus/hashes.txt`; `cargo xtask corpus verify` | The computed hashes equal the hashes **you computed by hand** in P0-05. If they differ, one of the two is wrong and the plan stops here until it is known which |
| **P1-08** | Formula evaluation | Evaluator over `Oracle`, deterministic sampling, shrinking, `CounterExample` construction; an empty-generator law refuses | Property tests against a stub oracle; a broken stub produces a *minimal* counter-example, asserted by value |
| **P1-09** | Alleles | `Allele`, `AlleleBody::Native`, `NativeRegistry`; `add@ℤ`; `CliInput`'s parse allele | Both alleles resolve and answer through `Oracle`; allele payload hashes are stable across runs |
| **P1-10** | Witnesses | Founding replay (exhaustive), `TestimonyStore`, budgets | A founding witness that fails produces a refusal naming the witness; a cell's hash is unchanged after 1 000 testimony appends — the G2 regression test |
| **P1-11** | Gate checks 1–3 | `Gate::admit_allele`, `check::contract`, `check::laws`, `check::witnesses` | **Exit-gate demo 2 passes:** an allele that breaks commutativity is refused and the refusal carries the counter-example |
| **P1-12** | `ℚ` frame | Exact rationals, lowest terms, positive denominator; extension of ℤ with ι and ρ | `conformance::check(Rat)` passes including FO7, at 10 000 samples |
| **P1-13** | Check 4 and cross-frame admission | `check::extension`; the six obligations of §2.5; `add@ℚ` | **Exit-gate demo 1 passes:** `add@ℚ` is admitted, `(2,3) → 5` replays through the restriction, and the cell hash is **byte-identical** to before |
| **P1-14** | Regulatory and port invariants | Regulatory edits; rename vs add; V19, V20, V24-embryo as properties | **Exit-gate demos 3 and 4 pass:** a regulatory edit leaves the hash untouched; a renamed port is accepted silently; an added in-port is refused as non-conservative unless declared a new version |
| **P1-15** | The gate's own adversary | The mutant corpus of §6.3; `cargo xtask power`; the degeneracy advisory of §2.4 | `cargo xtask power` prints `gate power: 12/12` and fails the build below 100%. Any survivor is written up in `docs/Findings/` before the commit lands |
| **P1-16** | Gate runner and freeze | `cargo xtask gate 1`, `gates.lock`, corpus freeze, README, Phase 1 findings | `cargo xtask gate 1` prints `4/4` and writes `gates.lock`; `cargo xtask gate all` re-runs Phase 0's corpus checks and Phase 1's four demos from a clean checkout |

**Ordering notes.** P1-01 through P1-05 are independent of the grammar and can start the moment the vocabulary (P0-01) is frozen — which is worth knowing, because P0-02 is the slowest paper item. P1-06 is the first commit that requires the grammar. P1-12 and P1-13 are the substance of the phase; everything before them is the equipment that makes them checkable.

---

## 6. Test Strategy

### 6.1 Invariants as property tests, from the first commit

Not a test suite that grows to cover the invariants later — the invariants are the first tests written, because the invariants are the deliverable.

| Invariant | Test |
|---|---|
| **V18** no primitive name in a coding region | The law AST makes it unrepresentable (§4.4). A parser test asserts that a source file naming `int.add` in a law is a *parse* refusal, not a gate refusal |
| **V19** hash unchanged by adding, removing, swapping an allele | Property: for a random cell and random allele list permutations, hash is constant |
| **V20** hash unchanged by any regulatory edit | Property: for a random cell and a random regulatory perturbation (names, literals, styles, ordering), hash is constant |
| **V24-embryo** invariance under port rename | Property: renaming any port leaves the hash constant; adding one changes it |
| **FO1–FO7** | `conformance::check`, run against every registered frame in CI |
| **§2.7** canonical text | `parse∘print = id`, `print∘parse∘print = print`, and `print` constant under regulatory perturbation |
| **G2** identity stability under use | 1 000 testimony appends leave the hash unchanged |

### 6.2 Goldens, and the rebless rule

`corpus/` holds hand-written blueprints and their hashes. Hashes are **never** updated automatically. `cargo xtask corpus verify` compares; `cargo xtask corpus rebless` exists but prints a warning, requires `--i-changed-the-canonical-form`, and writes a line into `docs/Findings/canonical-form-changes.md` recording the date and the reason. This is the guard on §2.6's accepted rehash: rehashing is allowed, rehashing by accident is not.

Appendix A tells Cursor, in as many words, that reblessing a golden to make a test pass is forbidden. It is the single most likely dishonest move an agent makes under pressure, and the one that quietly destroys a witness corpus.

### 6.3 The mutant corpus, and gate power

**This is the instrument for Phase 1's stated adversary.** The roadmap's Phase 1 adversary is that the gate turns out to accept almost anything, and that it happens without anyone noticing. A number that can be watched is the only defense.

A **mutant** is a deliberately wrong allele for `Sum` or `CliInput`. The gate must refuse all of them. `cargo xtask power` reports the fraction refused as **gate power** and fails below 100%.

| # | Mutant | Should be caught by | Note |
|---|---|---|---|
| 1 | `a + b + 1` | identity law | the trivial case |
| 2 | `a - b` | commutativity, witness | — |
| 3 | `a * b` | identity law, witness | — |
| 4 | `max(a, b)` | identity law **only on negatives** | fails against a generator that never produces negatives — a test of the generator, not of the law |
| 5 | `a.saturating_add(b)` at `i64` | any law, **only beyond `i64::MAX`** | the reason §4.2 demands big generators. This is also the Phase 10 compiler risk, rehearsed nine phases early |
| 6 | `if (a,b) == (2,3) { 5 } else { 0 }` | identity, commutativity, associativity | **the impostor.** Replays every founding witness perfectly and is worthless. If this survives, the coding region states nothing and S3's finding has arrived |
| 7 | constant `0` | identity law | — |
| 8 | correct sum, wrong out-frame | contract check | — |
| 9 | correct sum, panics on negatives | law evaluation must treat a panic as a refusal, not a crash | catch unwind at the oracle boundary |
| 10 | correct on ℤ, wrong on ℚ | cross-frame check 6 | admitted only after P1-13 |
| 11 | ℚ allele that does not restrict to ℤ | cross-frame check 4 | the conservative-extension mutant |
| 12 | `CliInput` returning `7` for `"007"` | the `parse(format n) = n` law | the calculator's own hole, made into a test |

**When a mutant survives, the finding is written before the fix.** The surviving mutant names a law that nobody wrote, and the pair (mutant, missing law) is the raw material for R23 — the creator helping a non-expert write laws. Keep them in `docs/Findings/surviving-mutants.md` even after they are fixed; that file is the specification for a feature this project has not yet budgeted.

### 6.4 The gate runner and V28

`gates.lock` records which gates have ever passed. `cargo xtask gate all` runs every recorded gate and fails if one regresses. From Phase 1 onward, that file is how the path of truth is enforced on the build itself — and it is why `cargo xtask gate 1` must be runnable from a clean checkout with no network.

### 6.5 What Phase 1 deliberately does not test

Performance. Nothing in Phase 1 is on a hot path, and the live engine's speed — Risk #3, the one that can collapse the two-engine model — is a Phase 2 measurement. Writing benchmarks now would tune the wrong thing and give false comfort.

---

## 7. Dependencies and Forbidden Constructs

| Need | Crate | Note |
|---|---|---|
| Arbitrary-precision ℤ, ℚ | `num-bigint` + `num-rational` | Simple and adequate. `malachite` is the faster alternative if ℚ becomes a bottleneck; it is a regulatory decision, reversible behind the `Frame` trait |
| Hashing | `blake3` | 256-bit, with domain tags |
| Unicode | `unicode-normalization` | NFC on every parsed literal and identifier |
| Property testing | `proptest` (dev only) | For **our** invariants. Frame law sampling uses the frame's own generators, not proptest — frames are dynamic and proptest strategies are static |
| Compile-fail tests | `trybuild` (dev only) | P1-05 |
| Snapshots | `insta` (dev only) | For refusal messages and printed canonical text. **Not** for corpus hashes, which live in a plain checked-in file |

**Forbidden in `crates/`, enforced by clippy config, `xtask vocab`, and review:**

| Forbidden | Why |
|---|---|
| `unsafe` | `#![forbid(unsafe_code)]` in every crate |
| `HashMap` / `HashSet` anywhere reachable from a hash, a canonical form, a sample, or a refusal | iteration order is seeded per process — §2.9 |
| `f32` / `f64`, anywhere, at all | Part I §12.3's last row. There is no floating point in the truth core |
| `SystemTime`, `Instant`, `std::env`, any ambient input | §2.9 |
| `rand::thread_rng` or any unseeded RNG | §2.9. Sampling takes a seed and reports it |
| `#[derive(Hash)]` on any DNA type | Rust's `Hash` is not stable across versions or platforms and is not the hash Law 5 means. `Hash` here is our own type |
| `serde` derive on coding-region types | Field order, skip attributes and version drift silently change canonical bytes. The canonical writer is written by hand and tested |
| Refusals as `Err` | §2.8 |
| `unwrap` / `expect` outside tests | A refusal is a value; a panic is a bug. The one exception is the oracle boundary, which catches unwind deliberately (mutant 9) |
| Any dependency not listed above, without a line in `docs/Findings/dependencies.md` saying what it replaces | Part II §20 borrows freely, but not silently |

---

## 8. Exit Gate 1, As a Checklist

Scripted and repeatable, from a clean checkout, offline. `cargo xtask gate 1` runs all four and prints the transcript.

- [ ] **1 · The reference example.** `add@ℤ` exists. `add@ℚ` is admitted as a second allele. Founding witness `(2,3) → 5` replays. **The cell hash is byte-identical before and after.**
- [ ] **2 · A refusal with evidence.** An allele that breaks commutativity is refused, and the refusal carries the counter-example that broke it, shrunk, with its seed.
- [ ] **3 · Silent mutation.** A regulatory edit — change the prompt literal, change a style — leaves the hash untouched.
- [ ] **4 · The port rule.** An added in-port is refused as a non-conservative extension unless declared as a new version; a renamed port is accepted silently.

And, because a gate that only tests what it was built to pass is not a gate:

- [ ] **5 · Gate power.** `cargo xtask power` reports 12/12 mutants refused.
- [ ] **6 · Corpus.** Every hash in `corpus/hashes.txt` still matches, and matches what was computed by hand in Phase 0.

---

## 9. Risks Watched During This Phase

| Roadmap risk | Instrument in this plan | What to do when it fires |
|---|---|---|
| **1 · Laws underdetermine implementations** | Gate power (§6.3), especially mutant 6; the degeneracy advisory (§2.4) | Write the finding first, then the missing law. A pattern of missing laws is the R23 feature specification, not a series of one-off fixes |
| **2 · The witness/identity contradiction** | The 1 000-append hash-stability test (§6.1) | Stop. G2 has been implemented wrong and every later phase inherits it |
| **5 · Jet mismatch** (rehearsal) | Mutant 5, and the sampled cross-frame agreement of §2.5 | Phase 2's differential harness is already designed by the time it is needed |
| **10 · Naming collides** | `cargo xtask vocab` (§3.2) | Fix the table, not the code, then let the lint drive the rename |
| **New · the canonicalizer becomes the identity bug** | §6.1's three canonical-text properties; the rebless rule (§6.2) | Switch to the binary-encoding alternative of §2.7. One commit, confined to `joinn-dna::canon` |
| **New · Cursor produces a plausible gate that checks nothing** | Every "done when" is a demo or a refusal, never "the code compiles" | Reject the commit. A gate that has never refused anything has not been tested |

---

## 10. What This Plan Refuses To Do

| Refused | Because |
|---|---|
| Start Phase 1 before Phase 0's exit gate is ticked | V30. The grammar is the artifact Phase 1 reads |
| Let Cursor choose the grammar, the canonical form, or the vocabulary | Those are the decisions everything else is checked against. An agent optimizing for a passing test will choose whatever is easiest to parse |
| Build a general number tower | Exactly three frames: `Text`, `ℤ`, `ℚ`. The third exists to prove the obligation set generalizes, not to be useful |
| Promote spike code | §3.1. A weekend experiment that becomes load-bearing is how the core-limits finding turns into a core-limits assumption |
| Write benchmarks | §6.5 |
| Ship a gate whose refusals lack counter-examples | §4.6. A refusal without evidence teaches a creator nothing and is indistinguishable from a bug |
| Rebless a golden hash to make a test pass | §6.2. This is the move that destroys a witness corpus |
| Add a fourth Phase 1 crate | §4.1 |

---

## 11. Open Items This Plan Creates

Continuing the roadmap's numbering from R25.

| ID | Topic | Question |
|---|---|---|
| **R26** | Frame signature obligations | Is "ι is a signature homomorphism" (§2.5 check 3) sufficient, or must ι also preserve the frame's *laws*, not only its operations? What does a frame owe when the signature of the wider frame is strictly larger? Does the minimal-signature discipline of §2.4 survive a frame like AISC 360-22, whose vocabulary is inherently large? |
| **R27** | Canonical-form versioning | `codex` rehashes the corpus when it increments. Is rehash-on-format-change still acceptable after Phase 9, when blueprints are published? If not, what is the migration — a canonical re-encoding per version, or a permanent commitment to codex 1? |
| **R28** | Gate power as a standing metric | Is a curated mutant corpus an adequate proxy for the gate's strength, or does it only measure the mutants someone thought of? Can mutants be generated from the coding region itself — and if so, is that the same machinery as R23's law suggestion, arriving from the other direction? |

**Touchpoints with existing items:** R24 (port identity) is partly answered by the tombstone decision of §2.2 and will be tested in earnest at P1-14. R20 (frame obligations) gains one row (§2.5 check 3) and one open question (R26). R19 (witnesses and identity) is implemented at P1-10 and its replay budget becomes a real number there. R23 (law authoring) gains its first evidence file at P1-15.

---

## Appendix A · `AGENTS.md` and `.cursor/rules/joinn.mdc`

Paste this into both files at the repo root. It is written to be read cold, on every request, by an agent that has not read the plan.

```markdown
# JoInn — standing rules

This repo is the truth core of JoInn: DNA in, verdict out. It is a library.
There is no engine, no primitive set, no host, no UI, and no GPU in this phase.
The build plan is docs/Plans/JoInn Phase 0-1 Implementation Plan.md. Work one
numbered commit at a time. Do not start the next one.

## Hard rules

1. A refusal is a VALUE (`Verdict::Refused`), never a Rust `Err` and never a panic.
   `Result` is for host errors only: IO, malformed input, bugs.
2. Never `unwrap`, `expect`, or panic outside tests. `#![forbid(unsafe_code)]`.
3. No `f32`/`f64` anywhere. No `HashMap`/`HashSet` — `BTreeMap`/`BTreeSet` only.
4. No wall clock, no environment, no unseeded RNG. Sampling takes an explicit
   seed and every refusal reports the seed that produced it.
5. No `#[derive(Hash)]` and no `serde` derive on any DNA type. The canonical
   writer is hand-written and tested.
6. Never change a golden hash in corpus/ to make a test pass. If a hash moves,
   stop and report it. Reblessing requires an explicit flag and a written
   finding.
7. Never weaken, skip, `#[ignore]`, or delete a test to make a build green.
   Report the failure instead.
8. A primitive name (`int.add`, `text.parse_int`, …) must never appear in a
   coding region — not in a law, not in a contract, not in a witness. Laws may
   name: the cell under definition (`self`), frame signature operations, and
   other coding regions BY HASH.
9. Display names, literals, prompts, styles and layout are REGULATORY. They are
   never hashed. If a change to one of them moves a hash, that is a bug in the
   canonicalizer, not a fact about the change.
10. Nothing in crates/ may depend on anything in spikes/. Spike code is
    throwaway and is deleted, never promoted.

## Crate boundaries

- joinn-frame: values, frames, the obligation set, the conformance harness.
  Knows nothing about DNA.
- joinn-dna: coding region, regulatory region, laws, witnesses, canonical text,
  hashing. It is DATA — it never evaluates anything and never holds a function
  pointer.
- joinn-gate: evaluation, sampling, witness replay, the four checks, refusals.

## Vocabulary

Use exactly these words in identifiers, comments and commit messages:
coding region, regulatory region, allele, frame, witness, founding witness,
testimony, verdict, refusal, counter-example, contract, port, law, gate, assay.

Do not use: schema, spec, metadata, config, impl, variant, backend, type (for
frame), error/failure/invalid (for refusal), test case/fixture (for witness).

## Definition of done

A commit is done when the plan's "done when" command passes and you have
reported what it printed. "It compiles" is not done. "The tests pass" is not
done if the test does not refuse anything.
```

## Appendix B · Directory layout

```
D:\JoInn\
├── docs\
│   ├── Theory\                     (Parts I–III, calculator note)
│   ├── Plans\
│   │   ├── JoInn Build Roadmap.md
│   │   ├── JoInn Phase 0-1 Implementation Plan.md   ← this file
│   │   └── JoInn Coding Region Grammar.md           ← P0-02, P0-03
│   └── Findings\
│       ├── spikes\                 (S1–S6 findings, including negative ones)
│       ├── surviving-mutants.md    (the R23 specification, accumulating)
│       ├── canonical-form-changes.md
│       └── dependencies.md
└── joinn\                          (the repo — greenfield)
    ├── Cargo.toml                  (workspace; exclude = ["spikes"])
    ├── AGENTS.md                   (Appendix A)
    ├── .cursor\rules\joinn.mdc     (Appendix A)
    ├── gates.lock
    ├── crates\
    │   ├── joinn-frame\
    │   ├── joinn-dna\
    │   └── joinn-gate\
    ├── corpus\
    │   ├── phase0\                 (hand-written blueprints, four spellings each)
    │   ├── hashes.txt              (goldens; hand-computed first)
    │   ├── mutants\                (§6.3)
    │   └── testimony\              (append-only, never hashed)
    ├── xtask\                      (gate, corpus, vocab, power)
    └── spikes\                     (throwaway; deleted at Phase 1 start)
        ├── s1-assay\  s2-gpu\  s4-canon\  s5-turn\  s6-zoom\
```

## Appendix C · Glossary delta for code

| Theory term | Rust identifier | Notes |
|---|---|---|
| coding region | `CodingRegion` | the only thing `hash` accepts, besides `Allele` |
| regulatory region | `RegulatoryRegion` | does not implement `Genotype`; proven by a compile-fail test |
| allele | `Allele`, `AlleleBody`, `NativeId` | payload; `AlleleBody::Dna` arrives in Phase 2 |
| founding witness | `CodingRegion::founding` | hashed, replayed exhaustively |
| accumulated testimony | `TestimonyStore` | never hashed, replayed under a budget |
| frame | `Frame` trait, `FrameRef`, `FrameRegistry` | a frame owns its values' canonical printing |
| embedding / restriction | `Frame::embed` (ι) / `Frame::restrict` (ρ) | ρ is partial and returns a `Verdict` |
| law | `Law`, `Formula`, `Term_` | `Term_` is spelled with a trailing underscore to avoid colliding with `Term`, the value shape |
| contract | `Contract`, `PortDecl`, `JoinPolicy` | `join_policy` is hashed and unread until Phase 2 |
| verdict / refusal | `Verdict<T>`, `Refusal`, `CounterExample` | never `Result` |
| the gate | `Gate`, `check::{contract,laws,witnesses,extension}` | four checks; a fifth arrives with declarations in Phase 4 |
| assay | *(nothing in Phase 1)* | `declarations` exists, hashed, and must be empty |

---

*JoInn Phase 0–1 Implementation Plan (Draft 0.1). Builds on the Build Roadmap (Part IV) §3–§5 and on Parts I–III. Everything marked PROPOSED · yours is a recommendation made while writing this plan; Cursor implements what the roadmap and the grammar document say, not what this plan prefers.*

