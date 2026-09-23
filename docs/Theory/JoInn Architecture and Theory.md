**JoInn Architecture and Theory**

Part I — General Theory of Fractal Architecture

Author: AJ

Draft 0.1 — September 16, 2026 · revised September 17, 2026 (exclusive membership within a lens and the touch-only law for hyperedges: §3, §5.2, §7, R8) · revised again September 17, 2026 (the hash boundary, DNA regions, sealed primitives, and topology as a derived assay: §3, §4.1, §8, §9.4, §10, §11.1, §14 — worked out in Part III)

Status: working theory; specifics of each item to follow in later parts

> *My purpose is to create. We must subdue all things.*

JoInn is to be a software architecture that provides application creators a platform to build any app that they want.

# 0. How to Read This Document

This part records the **general theory** of JoInn: its purpose, laws, vocabulary, and overall shape. It deliberately stops short of implementation detail. Each later part takes one item from this theory and works it out.

| **Part** | **Covers** |
|----------|------------|
| I — General Theory | This document: purpose, laws, vocabulary, structure |
| *JoInn Calculator Primitives* | Part I applied to the §12 reference example: what it takes to build 2 + 3 |
| II — JoInn Visual Host | Wrapping wgpu; the GPU Body Model; semantic zoom, picking, expression |
| III — Primitives and DNA | What is below the cell; the hash boundary; folding; the assay layer |

Every claim is tagged with its status so the theory stays honest about what is settled:

| **Tag**      | **Meaning**                                                       |
|--------------|-------------------------------------------------------------------|
| **DECIDED**  | Agreed direction. Changing it requires a deliberate decision.     |
| **PROPOSED** | Recommended direction that came out of critique; not yet adopted. |
| **OPEN**     | Known question without an answer yet. Tracked in Section 14.      |

# 1. Purpose and Vision

JoInn combines many technologies — user interface, storage, computation, validation, communication, and deployment — into a **single technology** so that anyone can create robust and powerful applications. The approach is called **fractal architecture**.

## 1.1 Positioning

- JoInn is a **performant, compiled, visual app creator**. **DECIDED**

- Creators build visually, see results instantly, and then compile the visual program into real Rust. **DECIDED**

- Creators in the open-source community build **cells** and publish them as **blueprints** that others reuse. A picture is a thousand words: a blueprint is seen, not just read. **DECIDED**

## 1.2 The Three Promises

JoInn aims to be **easy** (anyone can build), **robust** (what is built stays correct), and **powerful** (anything can be built). History shows these pull against each other. JoInn’s answer is that ease is earned through the other two: the law of opposition (Section 2) and well-built DNA (Section 8) make it safe for non-experts to compose powerful parts. Building that foundation is the hard part.

# 2. Foundational Laws

These laws govern every level of the architecture. Where a design choice conflicts with a law, the law wins.

## Law 1 — Opposition in All Things

Validation in JoInn is not a feature bolted onto an app; it is a law of the system, the same way an action has an equal and opposite reaction in physics and the same way truth is tested in life. Every operation carries its opposition. Most builders get this wrong by treating validation as optional. **DECIDED**

Opposition appears in several forms, all covered by the one law:

- **Contracts** — what a membrane accepts and what it guarantees.

- **Invariants** — what must remain true inside a cell.

- **Inverses** — every forward operation declares its opposite (addition ↔ subtraction, encode ↔ decode).

- **Adversaries** — counterparts that actively try to break a cell.

- **Immunity** — detection and isolation of untrue parts at the body or system level.

## Law 2 — The Path of Truth

A cell evolves by adding what is true. What was true remains true, so growth is naturally backward compatible. When something untrue is put in, evolution breaks apart. Therefore truth must be **defined**, not assumed (Section 9). **DECIDED**

## Law 3 — Fractal Semantic Zoom

“Fractal” describes the creator’s experience. At any zoom level, the app looks like nodes connected by hyperedges, and systems look similar to one another. Zoomed in, they are revealed to be very different. Similarity is in the view; the substance underneath is free to differ. **DECIDED**

## Law 4 — Consistent Containers

Each kind of container (body, system, galaxy) has its own way of communicating with the entities inside it, and that way is consistent across the whole platform. This gives apps from different parties a similar look, feel, and behavior. **DECIDED**

## Law 5 — Identity Follows Content

Things are identified by what they are, through content addressing. Identical DNA has identical identity; changed DNA gets a new identity linked to its parent. Content addressing goes hand in hand with testimony and truth. **DECIDED**

More precisely: **identity follows truth-bearing content.** Only what the evolution gate judges is hashed; everything else is phenotype and is regrowable. This is what makes the line between truth and decision a mechanical one rather than a matter of taste (§8.4, §9.4). **PROPOSED**

## Law 6 — Two Engines, One Truth

JoInn has a live engine for instant feedback and a compiler for performance (Section 11). Both must produce the same answers; each validates the other, which is Law 1 applied to JoInn itself. Two engines: **DECIDED**. Mutual validation: **PROPOSED**

# 3. Core Vocabulary

Naming precision matters: vague terms become bugs. These definitions are normative for the rest of the document.

| **Term**        | **Definition**                                                                                                                    | **Status**          |
|-----------------|-----------------------------------------------------------------------------------------------------------------------------------|---------------------|
| Primitive       | A sealed entity below the cell level. Primitives build cells. A primitive is a cell whose interior has been frozen, so it unfolds (Part III §6.2). | **DECIDED** concept |
| Cell            | The smallest visual building block. A cell can be shown in the UI and still have its function available.                          | **DECIDED**         |
| DNA             | The building plan that defines what a cell is and what laws it obeys.                                                             | **DECIDED**         |
| Genome          | The complete set of DNA shared by the cells of one body.                                                                          | **DECIDED**         |
| Membrane        | The boundary of a cell; the only place a cell meets anything outside itself.                                                      | **DECIDED**         |
| Body            | A cluster of cells sharing a genome, enclosed by its own boundary. The only true container.                                       | **DECIDED**         |
| System          | A link-graph grouping of bodies that controls how those bodies interact. Within one lens, a body belongs to exactly one system.   | **DECIDED**         |
| Galaxy          | A link-graph grouping of systems with thinner, less concrete boundaries. Within one lens, a system belongs to exactly one galaxy.  | **DECIDED**         |
| Universe        | The whole application.                                                                                                            | **DECIDED**         |
| Place graph     | The graph of what is inside what. Only cells-in-body.                                                                             | **DECIDED**         |
| Link graph      | The graph of what connects to what, made of hyperedges.                                                                           | **DECIDED**         |
| Hyperedge       | A connection that may join more than two nodes. It touches ports only and never crosses a membrane.                               | **DECIDED**         |
| Lens            | A zoomed view that collapses a group into a single node.                                                                          | **DECIDED**         |
| Blueprint       | A shareable, content-addressed cell design others can reuse.                                                                      | **DECIDED**         |
| k-block         | A topological building block of dimension k (called a k-cell in standard topology; renamed to avoid collision with JoInn “cell”). Assay vocabulary: k is arity of relation, not depth of nesting (§10). | **DECIDED**         |
| Frame           | The declared context in which truth is judged (for example: integers; AISC 360-22).                                               | **DECIDED**         |
| Law (of a cell) | A property that must always hold for a cell within its frame.                                                                     | **DECIDED**         |
| Witness         | A recorded true result that every future version must continue to satisfy; a form of testimony.                                   | **DECIDED**         |
| Slice           | Cut through dimension. Fixing one port of a k-block yields a (k−1)-block.                                                         | **DECIDED**         |
| Coding region   | The hashed part of DNA: frame, contract, laws, witnesses. The cell’s identity (§8.4).                                             | **PROPOSED**        |
| Regulatory region | The unhashed part of DNA: expression rules, literals, styles, environment rules. Versioned, never gated.                         | **PROPOSED**        |
| Allele          | A candidate implementation that satisfies a coding region. Payload, never identity.                                               | **PROPOSED**        |
| Hash boundary   | The line between what is hashed and what is not; equivalently, between truth and decision (§9.4).                                 | **PROPOSED**        |
| Fold / Unfold   | Sealing a validated composite into a primitive, and opening it again. Semantic zoom at the bottom of the stack.                    | **PROPOSED**        |
| Assay           | A derivation that measures a blueprint’s structure without altering it. An instrument, not part of DNA (§10).                     | **DECIDED** concept |

# 4. The Cell

## 4.1 Definition

A cell is **the smallest visual building block** of a JoInn application. It is not merely data: it can be seen in the UI and its function remains available there. A single cell can act as a complete, full-stack application. **DECIDED**

A cell is not the smallest *thing*. Something has to build the cell: **primitives**, which are hard-coded first (Section 11). **DECIDED**

A primitive is not, however, a different *kind* of thing from a cell. It is a cell whose interior has been sealed: its laws stated, its implementation frozen to native code, its reference definition kept. Sealing can be cut, so a primitive **unfolds** to the cell it was folded from. The fractal therefore has no floor — it has a seal. What is genuinely irreducible is not an object but a declared axiom set: a **frame**. Part III §6.2 and §7. **PROPOSED**

## 4.2 Compartments

| **Compartment** | **Role**                                                                      | **Notes**                          |
|-----------------|-------------------------------------------------------------------------------|------------------------------------|
| DNA             | The building plan: what the cell is and the laws it obeys.                    | Shared across the body (Section 8) |
| Membrane        | Boundary; governs everything that enters or leaves the cell.                  | Contracts live here                |
| Storage         | The cell’s persistent state.                                                  |                                    |
| Engine          | Computation; the cell’s function.                                             |                                    |
| Validation      | Opposition: contracts, invariants, inverses, witnesses.                       | Law 1                              |
| Visibility      | Both presentation (UI for users) and observability (inspection for creators). | Both required **DECIDED**          |

## 4.3 Proposed Refinements

- **Zero-cost compartments.** Every cell has all six slots, but an unused slot costs nothing. A full-stack cell and a pure-function cell are the same kind of thing. **PROPOSED**

- **Values between cells.** Not everything in an app is a cell. Plain values and messages flow between cells, as plasma and hormones flow between biological cells. Cells are the units of agency; values are the matter they act on. **PROPOSED**

- **Side effects at the membrane.** The engine stays pure; only the membrane touches the outside world. This protects determinism and makes the two engines agree. **PROPOSED**

# 5. Structure: Place Graph and Link Graph

JoInn separates **what is inside what** from **what connects to what**. This follows the idea of bigraphs, which describe systems with a place graph and a link graph over the same nodes.

## 5.1 Place Graph

- The **only true nesting** in JoInn is **cells inside a body and primitives creating a cell**. **DECIDED**

- A cell belongs to exactly **one** body. No overlap. **DECIDED**

- Bodies do not nest inside other bodies. Reuse happens by blueprint, not by shared instances. **DECIDED**

## 5.2 Link Graph

- Systems, galaxies, and the universe are **link-graph structures** built from hyperedges. **DECIDED**

- They only *appear* nested because lenses collapse them into nodes when zoomed out. **DECIDED**

- **Exclusive membership within a lens.** Within one lens, every body belongs to exactly one system, and every system to exactly one galaxy. The rule is fractal: exclusive membership holds at every level, the same way a cell belongs to exactly one body. **DECIDED**

- **"Used by" is a link, not membership.** A body that several systems use is still *in* only one system. The others reach it by hyperedge through the owning system's contract. A hyperedge may traverse many systems; a body may not belong to them. **DECIDED**

- **Other groupings are other lenses.** A body can sit in different systems under different lenses (by function, by deployment, by team), but never in two systems within one lens. Each lens is a tree; many lenses can exist over the same bodies, so no single tree is forced on the app. **DECIDED**

- **Touch-only law.** A hyperedge attaches at ports and never crosses a membrane. A connection that enters a body and leaves on the other side is not one hyperedge; it is two pairwise edges with that body between them. The membrane is the only place a cell meets the outside, and a port is its only opening (§6), so a link passing through would be a second, undeclared way in. **DECIDED**

- **Order is a property of the incidence, not of the drawing.** A hyperedge's members may be ordered, and its members may be marked as senders (tail) or receivers (head): none, one, or many. An ordered hyperedge orders *access* — turn-taking, priority, delivery, passing a capability. A pipeline where each stage transforms a value and hands it on is a chain of pairwise edges, not one hyperedge. **DECIDED**

## 5.3 The Levels

| **Level** | **Graph**         | **Made of**                  | **Role**                                  |
|-----------|-------------------|------------------------------|-------------------------------------------|
| Cell      | Place (member)    | Six compartments             | Smallest visual unit with function        |
| Body      | Place (container) | Cells sharing a genome       | The one true container; owns its boundary |
| System    | Link              | Bodies joined by hyperedges  | Controls how bodies interact              |
| Galaxy    | Link              | Systems joined by hyperedges | Loose, large-scale coordination           |
| Universe  | Link              | Galaxies                     | The whole application                     |

> **Why this matters.** Real structure stays shallow — one level of nesting — which is easy to compile, store, and reason about. Depth is provided by the view, not by the data. Cross-cutting concerns (a record used by both billing and authentication) are expressed as links instead of forcing a tree.

# 6. Communication and Boundaries

## 6.1 Rules

1.  Cells communicate **directly only within their own body**, across their membranes. **DECIDED**

2.  Cells in different bodies **cannot** talk directly. They must be linked according to the container they are in. **DECIDED**

3.  Each container type defines one communication method for its entities, consistent across the platform. **DECIDED**

## 6.2 Boundary Properties

A boundary carries more than one property, and they behave differently as scale grows:

| **Property**     | **Inner levels (cell, body)**                   | **Outer levels (system, galaxy)**                                  |
|------------------|-------------------------------------------------|--------------------------------------------------------------------|
| Coupling         | Tight; fast direct messages                     | Loose; thin boundaries, eventual agreement                         |
| Trust            | High; same genome                               | Lower; may cross machines or organizations                         |
| Cost of crossing | Nanoseconds to microseconds; cannot partly fail | Milliseconds or more; can time out, repeat, or arrive out of order |

- Coupling and trust are separate settings on a boundary, because coupling weakens outward while the need for trust checks grows outward. **PROPOSED**

- The cost of crossing a boundary is visible to the creator and to the compiler (for example, a local message type versus a remote one), so failure is handled on purpose. **PROPOSED**

- A cell may only message what it has been granted a link to (capability model). The body boundary acts as a firewall. **PROPOSED**

# 7. Semantic Zoom and Lenses

- As the creator zooms out, a system of bodies **collapses into a single node**. The node shows that it connects to other systems. **DECIDED**

- Specifics are not shown at every level. From outside, a hyperedge is seen only as “goes to that system,” not its internal detail. **DECIDED**

- A collapsed view is a **lens**, not structure. The same bodies may be grouped by more than one valid lens, the way one map can show roads or terrain. **DECIDED**

- Presentation follows level of detail: at low zoom a body shows a summary; zooming in reveals its cells. **PROPOSED**

- Because membership is exclusive within a lens (§5.2), a body is laid out and drawn **exactly once per lens**. Hyperedges arriving from other systems are drawn to the collapsed system's boundary. **DECIDED**

- Examples that look like a body in two systems are link facts: a customer record used by billing and authentication is owned by an identity system; material properties used by design checks, the calc report, and the fabrication BOM are owned by a materials system; an audit log is its own system that everything links to.

- A body that seems to serve two systems is usually **two bodies that have not yet differentiated**. The pancreas belongs to both the digestive and endocrine systems, but its digestive and endocrine parts are different cell populations. The answer is differentiation (R1), not shared membership. **PROPOSED**

- How the owning system is chosen for a truly shared body, when the choice isn't obvious. **OPEN**

# 8. DNA and the Genome

## 8.1 Settled

- DNA is **local to a body**. The cells of one body share its DNA and genome; a separate body has separate DNA. **DECIDED**

- DNA is **content-addressed**. Two bodies built from the same blueprint share identity while their DNA is identical; when one changes, it receives a new identity that links back to its parent. **DECIDED**

## 8.2 Proposed: DNA Holds Laws, Not Algorithms

In biology, genes do not do the work — they encode proteins, and proteins do the work. In JoInn, DNA states the **laws** a cell must obey, and **implementations** are expressed from it. An add cell’s DNA holds the laws of addition; integer addition and fraction addition are two expressions of the same DNA, each bound to obey those laws. Evolving a cell means expressing a new implementation that passes the existing laws, not rewriting what was true. **PROPOSED**

Stated loosely, this claim fails on contact: laws underdetermine implementations, and commutativity does not tell anyone how to add. The precise form is the one §8.4 gives — **DNA’s *identity* is its laws; its *payload* may carry implementations.**

## 8.3 Why It Matters: Evolution Requires It

If DNA is a tree of primitives and the hash is taken over that tree, then evolution is impossible. Follow the reference example of Section 12:

- The add cell at ℤ contains `int.add`. It has some hash.
- The add cell grown to ℚ contains `rat.add`. New tree, new canonical form, **new hash**.
- Law 5 says identity follows content, so this is a *different cell*, related to the old one only by a parent pointer.

“It does not change: he can still add two integers” has quietly become “he made a second cell and the system remembers the first one’s name.” The path of truth degrades into a changelog. What prevents this is deciding not what DNA contains, but **what part of it is hashed**. **PROPOSED**

## 8.4 The Two Regions

DNA has two regions, as a gene does.

| **Region** | **Contents** | **Hashed?** | **Gated?** |
|------------|--------------|-------------|------------|
| **Coding region** — the genotype | frame, contract (accepts and guarantees), laws, witnesses | **Yes. This is the identity** | Yes |
| **Regulatory region** | expression rules: which allele in which environment, thresholds, literals, styles, layout, engine choice | No | No — versioned and reversible |
| **Alleles** | candidate implementations, each carrying a proof or witness corpus | As payload only, never as the cell’s identity | Each admitted through the gate |
| **Lineage** | parent hash | — | — |

So **identity = frame + contract + laws + witnesses**. `add@ℤ` and `add@ℚ` are two alleles of one cell, and the first grader’s add cell keeps its name as he grows. The evolution gate becomes mechanical: *does this allele replay every witness and satisfy every law of the coding region, restricted to its frame?* **PROPOSED**

Two rules follow, and both are load-bearing:

- **No primitive name ever appears in a coding region.** Primitives live in alleles, which are expressed *from* DNA and checked *against* it — as proteins are expressed from a genome that contains none of them. **PROPOSED**

- **Phenotype = genotype × environment**, at every level. Part II §14 already says this for layout: a blueprint holds expression rules, not a phone layout and a desktop layout. The same mechanism covers arithmetic, where the environment is the frame of the incoming message rather than a screen. **PROPOSED**

Worked out in Part III §2–3 and §11.

## 8.5 Deferred

- Evolution and mutation mechanics. **OPEN**

- The concrete grammar of a coding region. (“Declarative or executable” is answered by region: the coding region is declarative; alleles are executable.) **OPEN**

> **Note.** The reference example in Section 12 is itself an evolution case, so part of the evolution design will be needed early.

# 9. Truth and Opposition

JoInn depends on being able to say what is true. Law 2 cannot work without it. This section proposes a working definition.

The layered definition below is **PROPOSED**:

## 9.1 Four Layers of Truth

| **Layer** | **Meaning**                                                                    | **Add-cell example**                      |
|-----------|--------------------------------------------------------------------------------|-------------------------------------------|
| Frame     | The declared axioms or context. Truth is always judged within a frame.         | Integers; later rationals                 |
| Laws      | Properties that always hold in the frame, including each operation’s opposite. | a + 0 = a; a + b = b + a; (a + b) − b = a |
| Witnesses | Every true result ever recorded. Truth accumulates as testimony.               | 2 + 3 = 5 stays on record forever         |
| Contracts | What the membrane accepts and guarantees.                                      | Accepts two numbers; returns their sum    |

## 9.2 The Evolution Gate

A change to a cell is accepted only if, within the old frame:

1.  it still satisfies every earlier **witness** and **law**;

2.  it accepts at least everything the old version accepted; and

3.  it guarantees at least everything the old version guaranteed.

Untrue changes are rejected **at the moment they are made**, not discovered later when the app fails. This is the path of truth made enforceable. **PROPOSED**

## 9.3 Frames Accumulate

A result computed under AISC 360-16 was not made false by AISC 360-22; it was true **in its frame**. Evolution adds frames rather than overwriting them, so a body can answer under either and opposition can compare them side by side. **PROPOSED**

## 9.4 Truth Versus Decision

Much application logic is not truth but **decision**: “shipping is free over \$50” is a policy that may rightly change. Decisions are versioned and reversible but are not held to the evolution gate.

**The line is the hash boundary (§8.4).** What is hashed is truth and faces the gate; what is not hashed is decision and is merely versioned. Stated as a rule:

> **Hash only what the gate judges. Everything else is phenotype, and phenotype is regrowable.**

This is Law 5 made more precise — *identity follows truth-bearing content* — and it is the same rule Part II arrived at twice without naming it. Part II §17.2 holds that a cell’s pick map, accessibility tree and intent set are truth and gated, while color, corner radius and font are decision. Part II D9 holds that GPU memory is phenotype and never genotype. Both are corollaries of the one line rather than independent findings. **PROPOSED · answers R5**

Note that the four layers of §9.1 are exactly the contents of the coding region. That is not a coincidence: the coding region is §9.1 written down and hashed.

## 9.5 Limits

- Laws and witnesses test truth on samples; they do not prove it for every case. Full proof would require proof-assistant methods. **PROPOSED**

- Exact arithmetic narrows the gap between mathematical truth and computed results.

# 10. Dimension and Topology

- Dimension in JoInn is defined **topologically**. **DECIDED**

- The building blocks of the topological model are called **k-blocks**: a 0-block is a point, a 1-block an edge, a 2-block a face, a 3-block a solid, and so on to any dimension. **DECIDED**

- A body’s dimension is the highest dimension of k-block in its structure. **PROPOSED**

## 10.1 Topology Is an Instrument, Not an Alphabet

> **Topological structure is derived from DNA and a body’s wiring. It is applied to a blueprint, never part of one.** **DECIDED**

The alternative — writing DNA itself in k-blocks and attaching maps — was considered and rejected. It breaks §8.4’s own rule: a complex is a derivation, so glue the same three laws in a different order and the hash changes, which means hashing a representation choice. It would also put the instrument’s limits inside the platform (a structural measure handles relational content, so JoInn could never state “sorted(x) is a permutation of x” at all), weld every published blueprint to one encoding forever, and force a single complex where §7 established that many lenses live over one structure.

A derivation of this kind is called an **assay**: a procedure applied to a sample that measures a property without altering it. The intuition is staining a tissue section — the structure was always there, the stain makes it visible, different stains show different things, and the stain is not part of the tissue.

## 10.2 What an Assay Reads

| **k** | **Block** | **Read from** |
|-------|-----------|----------------|
| 0     | point     | a port |
| 1     | edge      | a wire: a binary relation |
| 2     | face      | a law closing a cycle of relations |
| k     | k-block   | a hyperedge incident to k+1 ports |

**Dimension is arity of relation, not depth of nesting.** A wire is one-dimensional whether it sits inside a cell or spans a galaxy. It must not track the cell → body → system → galaxy ladder, because §5.2 and §7 make the upper levels a property of the active lens: an assay indexed on nesting depth would be measuring the camera. **PROPOSED**

**Slice** then has a precise meaning: fixing one port of a k-block yields a (k−1)-block. A k-block has k+1 faces, and choosing which face to read it from is what makes subtraction a direction of reading rather than a separate cell (Part III §6.1).

## 10.3 The Boundary of a Boundary

In topology the boundary of a boundary is empty. Read architecturally: ∂ applied to a cell gives its ports, so a **membrane is ∂ applied to a cell** rather than something a cell has — and a membrane has no membrane of its own, so boundaries do not recurse without end. **PROPOSED**

Several decisions already made for unrelated reasons are exactly what buys this: only cells-in-body is true nesting (§5.1), a fragment tests at most two membranes (Part II V6), and the touch-only law (§5.2, Part II V16). V6 came from GPU clipping cost and V16 from “a link must not be a second undeclared opening,” and they are the same identity. That identity is also the condition under which *what survives deformation* is computable at all — so the price of the measurement has already been paid.

Because the assay is derived rather than constitutive, the assembly **can fail**, and a failure is a finding: a link that is a second opening, a membrane that is not closed, a hyperedge that transits a body.

## 10.4 What It Measures, and How It May Refuse

A law is a constraint closing a cycle of relations, so **a law is a filling**, and an unfilled cycle is a round-trip whose consistency nothing guarantees. Law 1, read structurally, says there are none. The measurement also finds regions inside a body that cannot reach each other, and laws that close around an empty interior — a specified but unimplemented cell, which is what a stem cell is before it differentiates (R1).

Its most valuable finding is the one contracts cannot reach. `require` and `ensure` are local by design: a membrane sees only what crosses it. A system in which **every law holds at every membrane and yet no consistent global state exists** is invisible to every contract in it, and that is the class of failure an assay detects. This is the argument for the layer — not elegance, but that Law 1 as currently specified cannot catch a real kind of untruth.

**An assay never refuses on its own.** It reports, and its findings are advisory. A creator who wants one enforced writes a **declaration** into the coding region — a law about structure rather than about values — and the gate then checks the declaration against the measurement like any other law. Topology reaches DNA as a claim about a measurement, costing a line rather than an alphabet. **PROPOSED**

An assay is admitted only if its result is **invariant under everything outside the hash boundary**: rename a port, restyle a cell, swap an allele, change device, and the answer must not move. The hash boundary says what to hash and thereby says exactly what an instrument must be blind to.

## 10.5 Limits

An assay measures **structural** truth — connectivity, opposition, realizability. Laws and witnesses remain the instrument for **algebraic** truth. Of the six extensions in §12.3, an assay judges the first two and is silent on the rest: ordering, commutativity, overflow and floating-point inexactness are not structural properties. Structure is what survives deformation; algebra is what survives substitution. The gate keeps its four checks, and the assay sits beside it. **PROPOSED**

Worked out in Part III §9–10.

# 11. Build and Execution Model

## 11.1 Layers of Building

| **Layer**        | **Built by**     | **How**                                        |
|------------------|------------------|------------------------------------------------|
| Frames           | Platform authors | Declared axiom sets. The genuine floor **PROPOSED** |
| Primitives       | Platform authors | A small irreducible set hard-coded in Rust, plus sealed folds **DECIDED** |
| Cells            | Creators         | Visual programming from primitives **DECIDED** |
| Bodies and above | Creators         | Visual composition and linking                 |
| Blueprints       | Community        | Published, content-addressed, reused           |
| Assays           | Community        | Instruments applied to blueprints, published like blueprints **PROPOSED** |

### Folding

The irreducible set is small — roughly ten primitives, each with its opposite (Part III §4). The rest of what a toolbox needs is produced by **folding**: a validated composite cell is sealed into a primitive, its laws stated, its fast implementation frozen, its slow reference definition kept beside it. Sealing can be cut, so a primitive **unfolds** into the cell it came from. Fold and Unfold are Law 3 at the bottom of the stack, which is what keeps the architecture fractal all the way down. **PROPOSED**

Two rules keep this from collapsing:

- **A sealed primitive carries a reference definition, and the two are witnessed against each other.** A fast implementation that quietly disagrees with its own definition is the hardest bug class in every system that has tried this. A disagreement is a truth violation, not a performance bug (Law 6). **PROPOSED**

- **Folding is a platform act, not a creator act.** Creators publish blueprints freely and petition for primitives rarely. Without that, everyone seals everything and the small set is a fiction within a year. **PROPOSED**

## 11.2 Two Engines

JoInn runs a visual program in two ways. **DECIDED**

|         | **Live engine**                 | **Compiler**                 |
|---------|---------------------------------|------------------------------|
| Purpose | Instant feedback: debug and see | Performance                  |
| Speed   | Slower; may struggle at scale   | Fast compiled Rust           |
| When    | While building and debugging    | After debugging is satisfied |

- Both engines are checked against each other on the same inputs so their answers never drift apart (Law 6). **PROPOSED**

- DNA states the truth (for example, unbounded integers); the compiler chooses a fast representation; validation detects where that representation departs from the truth (for example, overflow). **PROPOSED**

# 12. Reference Example: The Addition Cell

## 12.1 The Story

A first grader is learning addition. He builds a body that presents two integers the user can add together. The body contains an **add cell** that adds two integers and a presentation that shows the result, with the option to show or hide the symbolic form.

As his learning advances, he needs to add fractions. The add cell evolves to add fractions too. Because adding two integers was true, it does not change: he can still add two integers. The cell is backward compatible naturally. He continues growing the add cell as his mathematics grows. This is **the path of truth**.

## 12.2 Why It Works

The integers sit inside the rationals, so every true statement about adding integers stays true when fractions arrive. Mathematicians call this a conservative extension. Number systems are built this way: natural numbers inside integers inside rationals inside reals inside complex numbers.

## 12.3 Where Truth Is Tested

Not every extension keeps every truth. These cases are what the evolution gate must catch:

| **Step**               | **What was true**                 | **What happens**                          |
|------------------------|-----------------------------------|-------------------------------------------|
| Naturals → integers    | “x + 5 = 3 has no solution”       | No longer true                            |
| Integers → rationals   | The result is an integer          | 5 becomes 5/1: same value, different data |
| Reals → complex        | Every number is ≥ 0 or \< 0       | Ordering is lost                          |
| Numbers → matrices     | a × b = b × a                     | Commutativity is lost                     |
| Math → 32-bit hardware | 2,147,483,647 + 1 = 2,147,483,648 | Overflow                                  |
| Math → floating point  | 0.1 + 0.2 = 0.3                   | Not exactly true                          |

## 12.4 The Add Cell’s DNA (Sketch)

- **Frame:** integers; later extended with rationals.

- **Laws:** identity (a + 0 = a), commutativity, associativity.

- **Opposition:** subtraction; a + b = c holds only if c − b = a.

- **Witnesses:** every result the first grader ever confirmed.

- **Contract:** accepts two numbers in the frame; returns their sum in the frame.

- **Presentation:** result, with optional symbolic form.

# 13. Proving the Theory

The theory holds if the same concepts build apps at very different scales without special cases. **PROPOSED**

| **Scale** | **Reference app**                | **What it proves**                                |
|-----------|----------------------------------|---------------------------------------------------|
| Cell      | Addition cell or unit converter  | A single cell is a complete app                   |
| Body      | Crane mat calculation package    | Inputs, checks, report, and revisions in one body |
| Galaxy    | Multi-user collaborative project | Linking, trust, and sync across machines          |

If the largest app forces concepts the smaller ones never needed, the architecture is not yet fractal.

# 14. Open Questions and Research Backlog

| **ID** | **Topic**         | **Question**                                                                                          |
|--------|-------------------|-------------------------------------------------------------------------------------------------------|
| R1     | The seed          | Can an app start as one stem cell with a genome and differentiate into bodies as requirements appear? |
| R2     | DNA               | The concrete grammar of a coding region. (Declarative or executable is answered by region: §8.4.) Evolution and mutation mechanics. |
| R3     | Truth             | Formalize frames, laws, witnesses, contracts, and the evolution gate. Does a declaration (§10.4) make it a fifth check? |
| ~~R4~~ | Topology          | **Split.** The original item conflated two unrelated meanings of dimension.                           |
| R4a    | Spatial dimension | Charts are 2D in Part II. Do they generalize to 3D bodies (a crane mat model), and does the ID target become a 3D pick? |
| R4b    | Relational dimension | Arity of relations as assay vocabulary (§10.2). Unrelated to space.                                |
| ~~R5~~ | Truth vs decision | **Answered:** the hash boundary is the line (§9.4).                                                   |
| ~~R6~~ | Primitives        | **Answered:** the minimum set is Part III §4; `text.parse_int` is a cell, sealed.                      |
| R7     | Lone cells        | Is a cell outside any body an implicit body of one?                                                   |
| R8     | Zoom              | Membership is exclusive within a lens (decided). How is the owner chosen for a truly shared body, and when should it split in two? |
| R9     | Life cycle        | Failure, death, and regrowth of cells (supervision).                                                  |
| R10    | Resources         | Budgets for computation and memory (metabolism).                                                      |
| R11    | Time              | Ordering and causality across bodies without a global clock. `grant` is in the minimal set — is capability-passing primitive, or a fold of something smaller? |
| R12    | Trust             | Security at system and galaxy boundaries.                                                             |
| R13    | Naming            | “k-block” vs the existing “blocks” genome linker in the biomimicry project. Also *assay*, *fold*, *seal*, *allele*, *coding region*. |
| R14    | Expression        | Part II. The environment signal set; how expression rules are written in DNA.                         |
| R15    | Visual truth      | Part II. The minimal inverse contract per visual primitive; perceptual tolerance.                      |
| R16    | GPU Body Model    | Part II. Record layouts, bind groups, delta protocol, ID target packing.                               |
| R17    | Assays            | Part III. What is the minimal valid assay? How is invariance under the hash boundary tested? Does a theory-level complex earn its keep? |
| R18    | Sealing           | Part III. Who may petition for a fold, and on what evidence? How are sealed primitives versioned, deprecated, and retracted when found untrue? |

# 15. Prior Art and Influences

| **Idea**                           | **Source**                     | **Lesson for JoInn**                        |
|------------------------------------|--------------------------------|---------------------------------------------|
| Cells that communicate by messages | Actor model; Erlang; Smalltalk | Isolation plus messages scales              |
| Nested membranes                   | Membrane computing (P systems) | Formal model of computing inside membranes  |
| Place graph and link graph         | Bigraphs (Robin Milner)        | Separate containment from connection        |
| Part and whole at once             | Holons (Arthur Koestler)       | Each unit is whole inward, part outward     |
| DNA, cells, membranes in software  | Holochain                      | Study its design and its adoption           |
| Cells as failure boundaries        | Cell-based architecture        | Cells contain failure, not just code        |
| Remote is not local                | Waldo et al., 1994             | Do not hide the cost of crossing boundaries |
| Substitution                       | Liskov substitution principle  | Basis for the evolution gate                |
| Growing a number tower with proofs | Lean and its math library      | The path of truth at scale                  |
| Zoomable interfaces                | Semantic zoom research         | Detail follows zoom level                   |
| Few opcodes plus native acceleration that must match | Urbit: Nock and jets | Folding works; the scar is native code that quietly disagrees with its own definition |
| Any word can join the dictionary   | Forth                          | Unrestricted folding destroys a small set   |
| Relations solved for whichever port is unknown | Sussman and Radul, propagator networks | Opposition becomes structure, not duplication |
| Genotype, phenotype, a degenerate code | Molecular biology          | Redundancy is robustness; expression is not mutation |
| Local consistency without global consistency | Abramsky, sheaf-theoretic contextuality | The failure class contracts cannot see      |
| Invariants that survive deformation | Algebraic topology            | What “stays true” could mean structurally   |
