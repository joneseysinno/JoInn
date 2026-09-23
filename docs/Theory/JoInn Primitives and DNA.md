# JoInn Primitives and DNA

**Part III — What is below the cell, and what DNA is made of**

*Theory only · no implementation implied*

Author: AJ · Draft 0.1 · September 17, 2026

> Status tags follow Part I: **DECIDED**, **PROPOSED**, **OPEN**. Everything in this part is **PROPOSED** unless marked otherwise. New research items R17–R18 are tracked in §15, and R4 is split there.

> **Thesis.** One word — *primitive* — is currently doing three unrelated jobs, and DNA has never been told what it is made of. Separating the three jobs makes the primitive set small. Deciding what DNA is made of makes evolution possible. Those two answers are the same answer, and it is a line: **hash only what the gate judges.**

---

## 0. How to Read This Part

Part I defined the cell, the six compartments, the laws, and DNA-as-building-plan. *JoInn Calculator Primitives* listed 24 primitives in six drawers and admitted at the end that it did not know whether `text.parse_int` was a primitive or a cell (R6). Part II defined the Visual Host and, along the way, decided that pick maps are truth and pixels are decision (§17.2) — a local answer to a question Part I left open at §9.4.

This part answers the general form of both. It defines what goes inside DNA, what a primitive is, how primitives can make more primitives, and where topology belongs.

| Section | Covers |
|---|---|
| §1–3 | The three jobs, the hash boundary, and the two alphabets |
| §4–6 | The minimal primitive set, how the 24 fold out of it, the composition moves |
| §7–8 | Sealing: how a cell becomes a primitive, and who is allowed to do it |
| §9–11 | The assay layer: structural measurement over DNA, and its limits |
| §12–14 | Decisions, invariants, costs |
| §15–17 | Open questions, prior art, glossary |

---

## 1. One Word, Three Jobs

The calculator note's 24 primitives are not one kind of thing. Sorted by what they actually do:

| Drawer | What it actually is | Can it shrink? |
|---|---|---|
| Grammar (`body`, `cell`, `port`, `wire`, `frame`, `literal`) | **Chemistry** — bonding rules. Not computation | Yes, to two |
| Kernel (`int.add`, `int.sub`, `text.parse_int`, `int.format`) | **Derived work** — each is a chain of something smaller | Yes, to zero |
| Validation, Visibility, Host (`require`, `ensure`, `law`, `witness`, `verdict`, `present`, `probe`, `host.*`) | **Grammar applied in a direction** — ports and predicates wearing different hats | Yes, to zero |
| Substrate (`hash`, `message`, `join`, `grant`) | **Physics** — identity and time | No |

`require` and `ensure` are not primitives; they are a predicate cell bound to a membrane, inward and outward. `present` is not a primitive; it is a port whose far end is the environment body (Part II §14.2). `verdict` is a two-case choice over an equality test. `int.sub` is not a primitive at all — the calculator note found the reason itself in §7 and did not follow it through.

**Separate the three jobs and the small-set question has an answer. Keep them merged and it does not.** **PROPOSED**

---

## 2. The Hash Boundary

This is the decision the rest of the part rests on.

### 2.1 The problem it fixes

The calculator note's build chain is `block tree → canonical DNA → hash`. That makes DNA a tree of primitives and hashes that tree. Part I §8.2 says DNA holds laws, not algorithms. **These cannot both be true**, and the reference example is where they collide:

- Add cell at ℤ: DNA contains `int.add`. Hash `c19e`.
- Add cell grows to ℚ: DNA contains `rat.add`. New tree, new canonical form, **new hash**.
- Law 5 says identity follows content, so this is a *different cell* related to the old one only by a parent pointer.

The story Part I §12 wants to tell — "it does not change: he can still add two integers" — has quietly become "he made a second cell and the system remembers the first one's name." The path of truth degrades into a changelog.

### 2.2 The decision

DNA has two regions, as a gene does.

| Region | Contents | Hashed? | Judged by the gate? |
|---|---|---|---|
| **Coding region** — the genotype | frame, contract (accepts and guarantees), laws, witnesses | **Yes. This is the identity** | Yes |
| **Regulatory region** | expression rules: which allele in which environment, thresholds, bands, device rules, engine choice, literals, styles, layout | No | No — versioned and reversible |
| **Alleles** | candidate implementations, each carrying a proof or witness corpus showing it satisfies the coding region | Individually yes, as payload — never as the cell's identity | Each admitted through the gate |
| **Lineage** | parent hash | — | — |

So **identity = frame + contract + laws + witnesses**. `add@ℤ` and `add@ℚ` are two alleles of one cell. The first grader's add cell keeps its name as he grows.

The evolution gate becomes small and mechanical: *does this allele replay every witness and satisfy every law of the coding region, restricted to its frame?* Nothing else. **PROPOSED**

### 2.3 Law 5, stated more precisely

> **Identity follows truth-bearing content. Hash only what the gate judges. Everything else is phenotype, and phenotype is regrowable.**

### 2.4 This answers R5

Part I §9.4 asks where the line falls between truth and decision. The line is the hash boundary. What is hashed is truth and faces the gate; what is not hashed is decision and is merely versioned.

Part II §17.2 already applied this rule without naming it: pick map, accessibility tree and intent set are truth and gated; color, corner radius and font are decision. Part II D9 — "GPU memory is phenotype, never genotype" — is the same rule a third time. One law replaces three separate decisions, and §17.2 and D9 become corollaries rather than independent findings. **PROPOSED · closes R5**

---

## 3. Two Alphabets and a Translator

The common reading of the biological analogy — "DNA is the code" — is the one that misleads. The real structure is two alphabets and a translation between them.

| Biology | JoInn |
|---|---|
| Four bases, read in triplets | The coding region: frames, laws, contracts, witnesses |
| Twenty amino acids | **Primitives** — what does the work |
| The genetic code, degenerate: many codons per amino acid | Expression: many alleles satisfy one coding region |
| Ribosome | The **evolution gate**: translates and proofreads |
| Environment | Frame of the incoming message; device signals; which engine is running |
| Phenotype | Running cells, GPU tables, pixels |

The load-bearing detail is that **DNA is not made of proteins**. Nothing in a genome is an enzyme. Read across: **primitives must not appear in the coding region.** They appear in alleles, which are expressed *from* DNA and checked *against* it.

Part II §14 already says this for layout — "a blueprint does not contain a phone layout and a desktop layout, it contains expression rules." Pushed one floor down, the same mechanism covers arithmetic. One rule, whole stack: **phenotype = genotype × environment**, whether the environment is a phone screen or the ring ℚ.

### 3.1 Degeneracy is the point

The genetic code is redundant on purpose: most third-position mutations are silent, which is what makes a genome robust to mutation. The JoInn analogue is exact rather than metaphorical — a many-to-one map with large fibres, and the same payoff. **The regulatory region is the wobble position.** Changing a literal, a style, a pipeline specialization or a device rule is a silent mutation: the meaning is untouched, so the gate never fires. **PROPOSED**

---

## 4. The Minimal Set

The proposed irreducible floor. Grouped by the kind of work it does. Every entry has its opposite, because Law 1 must hold at the bottom or it holds nowhere.

### Matter — computation

| Primitive | Opposite | Why irreducible |
|---|---|---|
| `eq` | ≠ | The root of Law 1. Without a way to detect disagreement there is no validation, no dispatch, no truth |
| `zero` / `succ` | `pred` | Counting. Bottoms out in hardware |
| `pair` | `split` | Compounding. Constructor ↔ destructor is Law 1 at the bottom |
| `choose` | — | Branch on a distinction |

### Space — structure

| Primitive | Opposite | Why irreducible |
|---|---|---|
| `bound` | `fill` | Declares a membrane: separates inside from outside and exposes ports. The only structural act |
| `bind` | `unbind` | Attaches a port to a port — the incidence relation itself. The one non-computational combinator |

### Name and time — physics

| Primitive | Opposite | Why irreducible |
|---|---|---|
| `hash` | **declared one-way** | Law 5. The only irreducibly one-way primitive is the identity primitive |
| `grant` | `revoke` | Capability — the ordering mechanism instead of a clock (R11) |
| `join` | `fan` | Fire when required ports are filled |

Nine to eleven depending on how pairs are counted. For calibration: SKI is three combinators, the lambda calculus three forms, Nock twelve opcodes, Forth roughly thirty words. **PROPOSED**

### 4.1 A note on `bound`

A membrane is not a thing a cell *has*. It is `bound` *applied to* a cell. This matters in §9: it makes the membrane an operation, and operations can be composed, inverted, and measured.

---

## 5. How the Twenty-Four Fold Out

| In the calculator note | Actually |
|---|---|
| `int.add` | A sealed fold of a `succ`-recursion cell (§7) |
| `int.sub` | **Not a separate thing.** `add` turned — the same law set solved for a different unknown port (§6) |
| `text.parse_int`, `int.format` | Sealed folds of a digit-accumulation cell and its turn. **This closes R6** |
| `require`, `ensure` | `bound` plus a predicate cell, inward and outward |
| `law`, `witness` | Stored predicate cells and stored message tuples. Data, not operations |
| `verdict` | `choose` over `eq` |
| `present`, `probe` | A port bound to the environment body, outward and inward |
| `host.read_line`, `host.write_line` | A **granted** port on the environment body |
| `frame` | A named set of laws. A fragment of a coding region |
| `literal` | Regulatory region — the calculator note's own conclusion in §7 |
| `message` | A `pair` of value and frame crossing a `bound` |
| `body`, `cell`, `port`, `wire` | `bound` and `bind` at two scales. Cell-in-body and body-bus differ in scale, not in kind |

Twenty-four to ten, with nothing lost. **PROPOSED**

---

## 6. The Composition Moves

A small alphabet is useless without a small grammar, and the grammar is where the design work actually lives. Four moves, each with its opposite.

| Move | Does | Opposite |
|---|---|---|
| **Chain** | out-port to in-port; series composition | Cut |
| **Braid** | several ports fire one cell (`join`); parallel composition | Fan |
| **Turn** | re-solve the same law set for a different unknown port | Itself — an involution |
| **Fold** | seal a validated composite into a new primitive | **Unfold** |

### 6.1 Turn

**Turn** is what halves the kernel. Treat a cell's law set as a relation over its ports rather than as a function from inputs to an output. `Sum` is a three-way relation over {a, b, sum}. Supply a and b and it adds; supply sum and b and it subtracts.

`sub` therefore stops being a primitive, stops being a cell, and becomes **a direction of reading**. Opposition stops being something a creator writes twice and becomes structural. This is a stronger form of Law 1 than a naming convention, and its prior art is Sussman and Radul's propagator networks.

*Cost:* the compiler must choose a direction per instance, and two unknowns means no answer. Which law sets are turnable has to be declared — the inverse-or-declare rule applied to law sets instead of to primitives.

### 6.2 Fold and Unfold

**Fold** is the answer to whether primitives can chain to make primitives. Yes — but only if there is an operation that takes a validated composite and *seals* it: freezes an allele to native code and makes its interior no longer live. And then:

> **Fold and Unfold are semantic zoom at the bottom of the stack.**

Zoom into a sealed primitive and it opens to reveal the cell it was folded from. Law 3 stops being a property of the creator's camera and becomes the structural relationship between a primitive and a cell. **There is no floor to the fractal. There is only a seal, and a seal can be cut.**

This also dissolves R6's stated tension — "minimal says cell; practical says primitive." Both are true at different zoom levels. `text.parse_int` is a cell, sealed. The seal is the answer, not a compromise between the answers.

And it answers Part II §22's self-hosting test structurally: the creator can display the Visual Host's own organelles as cells, because sealed things unfold. **PROPOSED**

---

## 7. Sealing

### 7.1 What a seal contains

A sealed primitive is not a black box. It carries:

| Part | Role |
|---|---|
| **Coding region** | Its own DNA: frame, contract, laws, witnesses. It is a cell |
| **Reference allele** | A slow, obviously-true definition, expressible in the minimal set |
| **Sealed allele** | The fast native implementation |
| **Witness corpus** | Recorded agreements between the two |

The reference allele is what `Unfold` reveals. The live engine may run it. The compiler runs the sealed one.

### 7.2 The failure mode, and who has already paid for it

Two pieces of prior art matter here.

**Urbit's Nock and jets.** Nock is twelve opcodes; everything is built from them. A *jet* is a native implementation of some Nock code that must be semantically identical to its formal definition. This is the calculator note's §7 idea — "reference kernel beside the native kernel," reducing to `zero`, `succ`, `pred`, `eq` — arrived at independently. The hardest bug class in that system is **jet mismatch**: native code that quietly disagrees with its formal definition, hard to find precisely because the two paths are never both run.

Law 6 already answers this better than they did, but only if it is made explicit:

> **A sealed primitive's reference and sealed alleles are witnessed against each other on sampled inputs. A disagreement is a truth violation, not a performance bug.**

**Forth's dictionary.** Any word can be defined and becomes indistinguishable from a built-in. Forth shows the other risk: unrestricted folding means everyone seals everything, the toolbox becomes unbrowsable, and the "small set" is a fiction within a year.

### 7.3 Sealing policy

Folding is a **platform act, not a creator act**. Creators propose folds; a fold is admitted only with:

1. a coding region;
2. a reference allele in the minimal set;
3. a witness corpus;
4. demonstrated use across independent bodies.

The community publishes **blueprints** freely and petitions for **primitives** rarely. The alphabet stays small on purpose rather than by accident. Sealed primitives are content-addressed and versioned like everything else. **PROPOSED · R18**

### 7.4 The bottom is a frame, not a thing

`succ` and `zero` bottom out in hardware, so unfolding must stop somewhere. The honest statement is that the floor of the fractal is a **declared axiom set — a frame** — not an irreducible object. The bottom is not an arbitrary decision hidden from view; it is a named one.

---

## 8. What DNA Does Not Contain

Restating the consequence of §2 and §3 as a rule, because it is the one most likely to erode:

| In the coding region | Not in the coding region |
|---|---|
| Frame | Any primitive |
| Contract: accepts, guarantees | Any allele |
| Laws, including declared opposites | Literals, prompts, styles, layouts |
| Witnesses | Device or environment rules |
| Declarations about measurements (§9.7) | Anything derived from the above |

If a primitive name ever appears in a coding region, evolution is broken again and §2.1 recurs. **PROPOSED**

---

## 9. The Assay Layer

Part I §10 decided that dimension is topological and named **k-blocks**, and R4 asks how they map onto cells, links and hyperedges. This section answers that — and the answer is that they do not go *into* DNA.

### 9.1 The decision

> **Topological structure is derived from DNA and the body's wiring. It is an instrument applied to a blueprint, never part of it.** **DECIDED**

The reason is §2's own rule. A complex is a derivation: glue the same three laws in a different order and the attaching list differs, the canonical form differs, the hash differs, and Law 5 calls it a different cell. That is hashing a representation choice — the same failure §2.1 diagnoses in block-tree DNA, better dressed. **The complex is phenotype. It belongs on the far side of the hash boundary.**

Three further consequences of merging it in, all avoided:

- **The instrument's limits would become the platform's limits.** Homology handles relational, abelian content (§10). If k-blocks were the alphabet, JoInn could never state "sorted(x) is a permutation of x" *at all*. Derived, that limitation only means the law is invisible to one instrument.
- **It would be a one-way door.** Every published blueprint would be welded to one encoding, and changing the encoding rehashes the community's work. Derived, the instrument can be revised, replaced, or deleted without touching a blueprint.
- **It would force one complex.** R8 established that many lenses live over one structure. A complex is a lens. Merging picks one forever.

### 9.2 The name

An **assay**: a procedure applied to a sample that measures a property without altering it. The intuition is histological staining — the structure was always there, the stain makes it visible, different stains show different things, and the stain is not part of the tissue. *Assay* is proposed as the term and goes to R13. **PROPOSED**

### 9.3 What an assay reads

| k | Block | Read from |
|---|---|---|
| 0 | point | a **port** |
| 1 | edge | a **wire**: a binary relation |
| 2 | face | a **law** closing a cycle of relations |
| k | k-block | a **hyperedge** incident to k+1 ports |

**Dimension is arity of relation, not depth of nesting.** A wire is 1-dimensional whether it sits inside a cell or spans a galaxy. This must not be allowed to track the universe → galaxy → system → body ladder: Part II §9.2 and D10 make the upper levels a property of the active lens, so an assay indexed on nesting depth would return a property of the camera. **PROPOSED**

Part I's existing term **slice** — "cut through dimension" — then means: fix one port of a k-block and get a (k−1)-block. Slicing is partial application; **Turn** (§6.1) is the choice of which face to read the block from. A k-block has k+1 faces, and `sub` is `add` read from another one.

### 9.4 The boundary operator, and why the existing decisions were its price

∂ takes a block to its boundary. ∂(cell) is its ports — so a membrane *is* ∂ applied to a cell, which is why §4.1 makes `bound` an operation. ∂(wire) is head minus tail. ∂(law) is the cycle of wires it fills. And ∂(membrane) is empty: **a membrane has no membrane of its own.**

That last line is ∂∂ = 0. Read against decisions already made for other reasons:

| Decision | Restated |
|---|---|
| Only cells-in-body is true nesting (I §5.1) | ∂ applied twice terminates |
| A fragment tests at most two membranes (II V6) | bounded ∂-depth |
| Touch-only law (I §5.2, II V16) | ∂(hyperedge) ⊆ ∪ ∂(cell): boundaries meet only at boundaries |
| A connection transiting a body is two edges | a chain whose composite boundary is well defined |

V6 was derived from GPU clipping cost and V16 from "a link must not be a second undeclared opening." They are the same identity. A performance argument and a security argument landing on one equation is the evidence worth having.

And ∂∂ = 0 is exactly the condition under which *what survives deformation* is computable at all. **The price was already paid; this part collects.**

Note the gain from keeping the assay separate: merged, ∂∂ = 0 would hold by construction and therefore detect nothing. Derived, **the assembly can fail**, and failure is a finding — a link that is a second opening, a membrane that is not closed, a hyperedge that transits. V16 becomes a checkable output instead of a rule one hopes was followed.

### 9.5 Laws are fillings

A law is a constraint that closes a cycle of relations. Filling a cycle kills it. Therefore:

> **Every unfilled cycle is a round-trip whose consistency nothing guarantees.**

Law 1 — opposition in all things — read structurally, says H₁ = 0: every loop is closed by a law, and there is no path from a back to a that the system cannot certify returns a.

The measurements and their readings:

| Group | Reading in JoInn |
|---|---|
| **H₀** | Independent regions that cannot reach each other. Rank above one inside a body means the body is not one thing |
| **H₁** | **Unopposed round-trips** — the Law 1 detector. Returns the specific cycle, not just a count |
| **H₂** | Laws closing around an empty interior: a **specified, unimplemented** cell |

H₂ is worth keeping. A shell of laws with nothing inside is what a published blueprint looks like before anyone expresses an allele for it — and it is what R1's stem cell is before differentiation. **Differentiation fills it**, which gives R1 a measurable notion of how much of an app is still promise.

### 9.6 The bug class membranes cannot see

`require` and `ensure` are local by design: a membrane sees what crosses it and nothing else. That is correct, and it is what makes cells composable. But local validation has a blind spot with a name.

Assign values to ports and the coboundary δ measures how they change across boundaries: on a wire, the difference the wire must account for; on a law, the residual around the filled cycle. A law is satisfied exactly when that residual vanishes. Two words then matter:

- **Closed:** every law holds locally. Every membrane is happy.
- **Exact:** the flow is derivable from an actual consistent assignment of values to ports.

Not every closed form is exact, and:

> **H¹ ≠ 0 is a system where every law holds at every membrane and no consistent global state exists.**

The Penrose staircase: every step goes down and you end where you started. This is the structure of local-versus-global consistency in distributed systems and in Abramsky's sheaf-theoretic treatment of contextuality — the formal home of the hardest distributed bug class there is. **No amount of `require` and `ensure` can detect it**, because each one only ever sees one membrane.

That is the argument for the assay layer. Not that topology is elegant: that **Law 1 as currently specified cannot catch a real class of untruth, and this is what it misses.** **PROPOSED**

### 9.7 Declarations: how an assay may gate

An assay never refuses anything on its own. It reports.

A creator may write a **declaration** into the coding region — one line, a law about structure rather than about values:

```
assert H₁ = 0
```

The gate then checks the declaration against the assay result, exactly as it checks any other law. Topology reaches DNA as a *claim about a measurement*, costing a sentence rather than an alphabet. Without a declaration, an assay finding is advisory: shown in the creator, never a refusal. **PROPOSED**

### 9.8 When an assay is valid

Deriving the complex instead of writing DNA in it creates one genuinely new problem: two people write two derivations and get two different answers for the same body. Which one is measuring the cell, and which is measuring itself?

The answer falls out of §2:

> **An assay is valid only if its result is invariant under everything outside the hash boundary.**

Rename a port, restyle a cell, change a literal, swap alleles, switch device — if the answer moves, the assay is reading phenotype and is inadmissible. The hash boundary does not only say what to hash; **it says exactly what an instrument must be blind to.**

Assays are then admitted the same way sealed primitives are (§7.3): a reference implementation, a witness corpus of bodies with known findings, and Law 6 cross-checking. No new machinery.

### 9.9 Assays are publishable

Because an assay is an instrument rather than an encoding, assays are content-addressed and shared like blueprints: a dimensional-consistency assay for engineering units, a trust-boundary assay for R12, a dead-region assay, a seed-maturity assay reading H₂ for R1. The community extends the instrument set without touching a single blueprint — which fits Part I §1.1 far better than a fixed encoding would.

An assay result is also drawable. An unfilled cycle is a highlighted loop in the creator. Part II §8's claim that "the UI is a validator you can see" extends from *forbidden connections* to *missing laws*.

---

## 10. What the Assay Does Not Catch

Part I §12.3 lists six extensions the evolution gate must judge. An assay handles half of them, and the split is clean rather than embarrassing.

| Step | What is lost | Caught by an assay? |
|---|---|---|
| Integers → rationals | nothing; representation changes | **Yes** — the induced map is injective |
| Naturals → integers | "x + 5 = 3 has no solution" | **Partly** — a cycle that was unfillable becomes fillable; H₁ drops. A drop is legal, a rise is not |
| Reals → complex | ordering | **No.** Order is not a structural invariant |
| Numbers → matrices | commutativity | **No.** Not at this level |
| Math → 32-bit | overflow | **No.** A frame mismatch, not a deformation |
| Math → floating point | exactness | **No.** Same |

So the claim is bounded:

> **An assay is the instrument for *structural* truth — connectivity, opposition, realizability. Laws and witnesses remain the instrument for *algebraic* truth. Structure is what survives deformation; algebra is what survives substitution.**

The evolution gate keeps its four checks (I §9.2). The assay sits beside it, not inside it.

### 10.1 The further frontier

Algebraic loss is in principle structural too, but in a different complex. An algebraic theory is generators and relations; relations among relations are syzygies; the whole is a free resolution. "Commutativity is lost when numbers become matrices" is a change in the relation module.

| Complex | Generators | Measures |
|---|---|---|
| **Wiring** | ports, wires, laws | opposition, realizability, completeness |
| **Theory** | operations, relations, syzygies | what an algebraic extension preserves |

The frame is the bridge: it supplies the coefficient group to the wiring complex and is what the theory complex presents. Two assays over one DNA is cheap; it was only expensive when one of them was going to *be* the DNA. Not for Draft 0.1. **OPEN · R17**

---

## 11. The Add Cell, Retold

| | |
|---|---|
| **Coding region** | frame ℤ; laws {a+0 = a, a+b = b+a, associativity, a+b = c ⟹ c−b = a}; contract {two in, one out, same frame}; witnesses {(2,3) → 5} |
| **Hash** | `c19e`. This is "the add cell," permanently |
| **Allele `add@ℤ`** | a sealed fold of a `succ`-recursion cell |
| **Allele `add@ℚ`** | a sealed fold of a cross-multiply-and-reduce cell, itself built from `add@ℤ` and `mul@ℤ` |
| **`sub`** | does not exist. `turn(c19e, solve for a)` |
| **Selection** | the frame of the incoming message picks the allele |
| **What happened when he learned fractions** | a new allele was admitted; the gate replayed (2,3) → 5; hash `c19e` unchanged |

The cell did not change. It expressed. That is the sentence Part I §12 has been reaching for.

### 11.1 The calculator's hole

`CliInput`'s DNA already carries the law `parse(format n) = n`. Structurally, that law is the 2-block filling the round trip between the universe and the terminal: `host → cli_a → sum → host`.

And the calculator note's own §7 observes that `format(parse s) = s` fails for `"007"`. Read structurally: **the cycle is filled in one orientation and not the other.** The `"007"` case is not a quirk to be mentioned in prose; it is a specific, nameable, computable hole.

Which generalizes:

> **A declared one-way primitive is a declared hole.**

`hash` has no inverse and never will. The inverse-or-declare rule is therefore not "state your opposite or admit laziness." It is **fill the cycle, or declare the hole and let the measurement carry it forever.** A universe built on content addressing has a permanent hole at its identity operator, by construction, and that is not a defect.

---

## 12. Decisions

| # | Decision | Status |
|---|---|---|
| D1 | "Primitive" covers three distinct jobs — chemistry, derived work, physics — and they are separated. | **PROPOSED** |
| D2 | DNA has a coding region (hashed, gated) and a regulatory region (versioned, not gated). | **PROPOSED** |
| D3 | Identity = frame + contract + laws + witnesses. Alleles are payload, never identity. | **PROPOSED** |
| D4 | Hash only what the gate judges. Everything else is phenotype and regrowable. | **PROPOSED** |
| D5 | Primitives never appear in a coding region. | **PROPOSED** |
| D6 | The irreducible set is roughly ten primitives (§4). | **PROPOSED** |
| D7 | Four composition moves: Chain, Braid, Turn, Fold — each with its opposite. | **PROPOSED** |
| D8 | Fold and Unfold are semantic zoom at the primitive level. A sealed primitive unfolds to its reference allele. | **PROPOSED** |
| D9 | Folding is a platform act. Creators publish blueprints and petition for primitives. | **PROPOSED** |
| D10 | **Topological structure is derived from DNA, never part of it. An assay is an instrument, not an encoding.** | **DECIDED** |
| D11 | An assay gates only through a declaration written in the coding region. Otherwise it is advisory. | **PROPOSED** |
| D12 | An assay is valid only if invariant under everything outside the hash boundary. | **PROPOSED** |
| D13 | Assays measure structural truth. Laws and witnesses measure algebraic truth. | **PROPOSED** |

---

## 13. Invariants

Continuing Part II's numbering.

| # | Invariant | Law or decision |
|---|---|---|
| V18 | No primitive name appears in any coding region. | D5 |
| V19 | A cell's hash is unchanged by adding, removing, or swapping an allele. | D3 |
| V20 | A cell's hash is unchanged by any edit confined to the regulatory region. | D2, D4 |
| V21 | Every sealed primitive carries a reference allele expressible in the minimal set. | §7.1 |
| V22 | A sealed primitive and its reference allele are witnessed against each other on sampled inputs; disagreement is a truth violation. | Law 6, §7.2 |
| V23 | Every composition move has its opposite, and Unfold of Fold returns the original cell. | Law 1, D7 |
| V24 | An assay result is invariant under every change outside the hash boundary. | D12 |
| V25 | An assay never refuses. Only a declaration checked by the gate refuses. | D11 |
| V26 | A body whose structure will not assemble with ∂∂ = 0 is reported as a structural finding. | §9.4 |

---

## 14. Costs

Named honestly, in descending order of how likely each is to sink the part.

1. **Laws underdetermine implementation.** Commutativity does not tell you how to add. An allele cannot generally be *derived* from a coding region; it must be supplied and checked. So §8's rule needs its precise form — *DNA's identity is its laws; its payload may carry alleles* — or it is a slogan that fails the first time someone needs a sort.
2. **Assays need abelian, relational content.** ∂ and δ are linear. Affine and conservation-style laws are fine; "sorted(x) is a permutation of x" is not a cochain condition and never will be. Scope the instrument to relations and it survives; claim it covers all laws and it dies on contact.
3. **Fold explosion.** Without §7.3's policy, the alphabet is small only on paper.
4. **Jet mismatch.** §7.2. The mitigation is V22, and it is not free: the reference path is far too slow for real data and is only usable for sampling.
5. **Assay scale.** Per-cell complexes are tiny and free. Whole-universe H¹ is not — and whole-universe H¹ is the one most worth having. It is local-to-global by nature, so an incremental form exists (compute per body, glue along the link graph, recompute only what changed), but it is work.
6. **Shallow complexes.** If nearly every law is a 2-block over a 3-cycle, the assay is a cycle count obtainable from a spanning tree, and the machinery is not earning its keep. Watch for it. If it happens, keep ∂∂ = 0 and "laws are fillings," and drop the rest.
7. **Decoration risk.** The discipline: the assay layer stays only if it catches something `require` and `ensure` cannot. Two candidates are claimed — H¹ (§9.6) and H₂ (§9.5). If neither survives contact with a real example, cut it without sentiment. Because it is derived, cutting it costs an instrument rather than a language.

### 14.1 The test that decides §9

Build the calculator's complex by hand — roughly fifteen blocks. Delete the law `parse(format n) = n` and compute H₁.

If it goes from 0 to 1 and returns the cycle `host → cli_a → sum → host`, the assay layer is real: a validation operator found a missing law from structure alone, without being told what to look for.

If it does not, or if getting it there takes special pleading about orientations and basepoints, cut §9.5 through §9.10 and keep only ∂∂ = 0, which earns its place on its own as the reason bodies do not nest and hyperedges only touch.

---

## 15. Open Questions and Research Backlog

**R4 splits in two.** The original item conflated two unrelated meanings of dimension, and leaving them merged guarantees the collision R13 already worries about:

| ID | Topic | Question |
|---|---|---|
| **R4a** | Spatial dimension | Charts are 2D in Part II. Do they generalize to 3D bodies (a crane mat model), and does the ID target become a 3D pick? A Visual Host question |
| **R4b** | Relational dimension | Arity of relations: wire = 1, three-port law = 2, hyperedge on five ports = 4. Assay vocabulary. Unrelated to space |

**Closed by this part:**

- **R5 · truth vs decision** — the hash boundary is the line (§2.4)
- **R6 · primitives** — the minimum set is §4; `text.parse_int` is a cell, sealed (§5, §6.2)

**New:**

| ID | Topic | Question |
|---|---|---|
| **R17** | Assays | What is the minimal valid assay? How is invariance under the hash boundary tested? Does the theory-level complex (§10.1) earn its keep? Is the incremental whole-universe form practical? |
| **R18** | Sealing | Who may petition for a fold, and what admission evidence is required? How are sealed primitives versioned and deprecated? What happens to bodies using a primitive that is later found untrue? |

**Still open, now sharper:**

- **R2 · DNA.** "Declarative or executable" is answered by region: the coding region is declarative, alleles are executable. What remains is the concrete grammar of a coding region, and whether declarations (§9.7) are laws or a fourth kind of entry. **OPEN**
- **R3 · truth.** The gate keeps its four checks. Does a declaration make it five? **OPEN**
- **R11 · time.** `grant` is in the minimal set, which makes capability-passing primitive rather than derived. Is that right, or is `grant` a fold of something smaller? **OPEN**
- **R13 · naming.** *Assay* for the instrument; *fold* and *seal*; *allele*; *coding region* and *regulatory region*. Do these hold, and do they collide with anything in the biomimicry project? **OPEN**

---

## 16. Prior Art

| Idea | Source | Lesson for JoInn |
|---|---|---|
| Twelve opcodes plus native acceleration that must match | Urbit: Nock and jets | The reference/sealed pair arrived at independently. Their scar is jet mismatch — hence V22 |
| Any word can join the dictionary | Forth | Folding works; unrestricted folding destroys the small set. Hence §7.3 |
| Relations solved for whichever port is unknown | Sussman and Radul, propagator networks | Turn (§6.1). Opposition becomes structure |
| Genotype, phenotype, and a degenerate code | Molecular biology | §3. Redundancy is robustness, not waste |
| Local consistency without global consistency | Abramsky, sheaf-theoretic contextuality | H¹ (§9.6): the bug class membranes cannot see |
| Invariants that survive deformation | Algebraic topology, CW complexes and homology | What "stays true" could mean structurally |
| Programs extracted from specifications | Coq extraction, CakeML, proof-carrying code | An allele with a certificate |
| Specify by laws before choosing a representation | Denotational design | The coding region comes first |
| Conservative extension | Mathematical logic; Lean's mathlib | Already in Part I §12.2; now the gate's actual test |

---

## 17. Glossary

| Term | Definition |
|---|---|
| **Coding region** | The hashed part of DNA: frame, contract, laws, witnesses, declarations. The cell's identity |
| **Regulatory region** | The unhashed part: expression rules, literals, styles, environment rules. Versioned, never gated |
| **Allele** | A candidate implementation satisfying a coding region. Payload, never identity |
| **Hash boundary** | The line between what is hashed and what is not; equivalently, between truth and decision |
| **Chain, Braid** | Series and parallel composition of cells |
| **Turn** | Re-solving a law set for a different unknown port. An involution |
| **Fold / Unfold** | Sealing a validated composite into a primitive, and opening it again. Semantic zoom at the bottom |
| **Seal** | A folded primitive: coding region, reference allele, sealed allele, witness corpus |
| **Reference allele** | The slow, obviously-true definition a sealed primitive is witnessed against |
| **Assay** | A derivation that measures a blueprint's structure without altering it. An instrument, not an encoding |
| **Declaration** | A line in a coding region claiming what an assay will find. A law about structure |
| **Unopposed cycle** | A round-trip through the wiring that no law fills |
| **Silent mutation** | A regulatory-region change that leaves the hash and the meaning untouched |

---

*JoInn Architecture and Theory, Part III (Draft 0.1). Builds on Part I and the Calculator Primitives companion; §9 relies on Part II's lens decisions. Theory only: no implementation implied. Everything is PROPOSED except D10.*
