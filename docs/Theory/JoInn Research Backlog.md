# JoInn Research Backlog

Open research topics for the JoInn fractal architecture. Mirrors Section 14 of *JoInn Architecture and Theory — Part I: General Theory* (Draft 0.1, revised Sept 17). R14–R16 come from *Part II — JoInn Visual Host* (wrapping wgpu; `D:\JoInn\docs\Theory\JoInn Visual Host.md`), whose proposals were adopted on Sept 17 — what remains there is decided or open. R17–R18 come from *Part III — Primitives and DNA* (`D:\JoInn\docs\Theory\JoInn Primitives and DNA.md`), drafted Sept 17.

**Changed Sept 17 (Part III):** R5 and R6 are answered and closed. R4 is split into R4a (spatial) and R4b (relational) — the original item conflated two unrelated meanings of dimension. R17 and R18 are new.

---

## R1 — The Seed (stem-cell growth model)  ·  status: TO DO

**Idea:** An app does not start from a template or an empty canvas. It starts as a single *stem cell* carrying a genome. As requirements appear, the cell divides and differentiates into specialized cells and bodies. Scaffolding a project = planting a seed.

**Questions to research**
- What is the minimum genome a seed must carry to grow a useful app?
- What triggers differentiation — user action, a declared requirement, a signal from a neighboring body?
- Since DNA is local to a body, how does a seed hand a genome to a new body it grows? (Copy? Fork? Content-addressed reference?)
- Can a differentiated cell revert (regeneration / refactoring)?
- How does this appear in the visual creator — growth animation, suggested divisions?
- When a body seems to serve two systems, what signals that it should differentiate into two bodies? (see R8)

**Starting points**
- Developmental biology: morphogens, Turing reaction–diffusion patterns, cell fate/differentiation, Wolpert's positional information
- L-systems (Lindenmayer) and generative grammars
- Morphogenetic engineering / "artificial embryogeny" in evolutionary computation
- Prior AJ work: `biomimicry` genome linker (M12), GRN compilation ("compose before compile")

---

## R2 — DNA definition  ·  status: partly answered

- **Answered (Part III §2, §8.4 of Part I):** "declarative or executable" is answered *by region*. The **coding region** (frame, contract, laws, witnesses) is declarative and is what gets hashed. **Alleles** are executable and are payload, never identity. The **regulatory region** (expression rules, literals, styles) is neither hashed nor gated.
- **Answered:** no primitive name ever appears in a coding region. Primitives live in alleles, expressed *from* DNA and checked *against* it.
- **Open:** the concrete grammar of a coding region — how a law, a contract and a witness are actually written down
- **Open:** whether a declaration (an assay claim, R17) is a law or a fourth kind of entry
- **Open:** evolution & mutation mechanics; blueprint lineage and parent links
- Note: a regulatory-region edit is a *silent mutation* — meaning untouched, gate never fires. This is the degeneracy of the genetic code doing real work, not a metaphor

## R3 — Defining truth  ·  status: open
- Frames, laws (incl. each operation's opposite), witnesses/testimony, contracts
- Evolution gate: conservative extension / Liskov-style substitution check
- **New:** does a declaration (R17) add a fifth check to the gate's four?
- Note: declarations are checked at admission of bodies and universes; the gate keeps four checks (Phase 4)
- Note: the four layers of truth are exactly the contents of the coding region — the coding region is §9.1 written down and hashed

## R4 — SPLIT into R4a and R4b

The original item conflated two unrelated meanings of "dimension." Leaving them merged guarantees the naming collision R13 already worries about.

### R4a — Spatial dimension  ·  status: open
- Charts are 2D in Part II. Do they generalize to 3D bodies (a crane mat model, an FEA mesh)?
- Does the ID target become a 3D pick?
- A Visual Host question, not an assay question

### R4b — Relational dimension  ·  status: open
- Arity of relations: port = 0, wire = 1, three-port law = 2, hyperedge on five ports = k
- **Decided:** dimension is arity, *not* depth of nesting. An assay indexed on nesting depth would be measuring the lens, not the app (Part I §10.2)
- **Decided:** *slice* = fixing one port of a k-block to get a (k−1)-block; a k-block has k+1 faces, and choosing a face is what makes subtraction a direction of reading rather than a separate cell
- Assay vocabulary. Unrelated to space. Ties to R17

## R5 — Truth vs decision  ·  status: ANSWERED, closed

**The line is the hash boundary.** What is hashed is truth and faces the evolution gate; what is not hashed is decision and is merely versioned. Stated as a rule: *hash only what the gate judges; everything else is phenotype and is regrowable.* (Part I §9.4, Part III §2.4)

This makes Law 5 more precise — identity follows *truth-bearing* content — and turns two earlier Part II findings into corollaries rather than independent decisions:
- Part II §17.2: pick map, accessibility tree and intent set are truth (gated); pixels and styles are decision
- Part II §11.1: a link's order and direction are truth, so an ordered spine for an unordered edge is a false assertion, not a style choice
- Part II D9: GPU memory is phenotype, never genotype

## R6 — Primitives  ·  status: ANSWERED, closed

**The minimum set is roughly ten**, each with its opposite (Part III §4): `eq`, `zero`/`succ`/`pred`, `pair`/`split`, `choose`, `bound`/`fill`, `bind`/`unbind`, `hash` (declared one-way), `grant`/`revoke`, `join`/`fan`. The calculator note's 24 fold out of these (Part III §5).

**`text.parse_int` is a cell, sealed.** The stated tension — "minimal says cell, practical says primitive" — dissolves: both are true at different zoom levels. Fold and Unfold are semantic zoom at the bottom of the stack, so a primitive opens into the cell it was folded from and the fractal has no floor, only a seal.

**`int.sub` is not a primitive and not a cell.** It is `int.add` *turned* — the same law set solved for a different unknown port.

Remaining detail moves to **R18 (sealing)** and **R13 (naming)**. Unchanged implementation notes: visual primitives are a renderer-agnostic vocabulary (Shape, Text, Image, Region, Port, Link, Chart); cells never touch wgpu; host shells (L0) and the GPU Body Model (L1) stay hard-coded below cells.

## R7 — Lone cells  ·  status: open
- Is a cell outside any body an implicit body of one?

## R8 — Zoom / lenses  ·  status: partly decided
- **Decided (Sept 17):** exclusive membership within a lens — every body belongs to exactly one system, every system to exactly one galaxy; fractal, like cell-in-body (Part I §5.2, §7; Part II §10.4, D10)
- **Decided:** "used by" is a link, not membership — other systems reach a shared body by hyperedge through its owning system's contract; a hyperedge may touch many systems, a body may not belong to them
- **Decided:** other groupings (function, deployment, team) are other lenses; each lens is a tree, many lenses exist over the same bodies
- Consequence: a body is laid out and drawn exactly once per lens; the earlier home/reference alias-glyph proposal is withdrawn
- **Open:** how the owning system is chosen for a truly shared body when it isn't obvious (a shared kernel as its own small system?)
- **Open:** when a body that seems to serve two systems should differentiate into two bodies (pancreas: acinar vs islet cells) — ties to R1

## R9 — Life cycle  ·  status: open
- Failure, death, regrowth of cells (supervision)
- Renderer tie-in: GPU memory is phenotype; mobile surface loss / device loss = visual death; regrow from DNA + storage + active lens

## R10 — Resources  ·  status: open
- Compute/memory budgets (metabolism)
- Renderer tie-in: per-tick snapshot re-render budget; who sets and spends it

## R11 — Time  ·  status: open
- Ordering and causality across bodies without a global clock
- Tie-in: an ordered hyperedge orders *access* (turn-taking, delivery, capability passing). How does the owning container enforce that order at runtime?
- **New:** `grant` sits in the minimal primitive set, which makes capability-passing irreducible rather than derived. Is that right, or is `grant` itself a fold of something smaller?

## R12 — Trust  ·  status: open
- Security at system and galaxy boundaries
- Tie-in: the touch-only law means a link is never a hidden second opening into a body
- **New:** a trust-boundary assay is one of the first instruments worth writing (R17)

## R13 — Naming  ·  status: open
- "k-block" vs the existing `blocks` genome linker in the biomimicry project
- Part II names to confirm: *Chart* (coordinate system) vs *frame* (truth context) vs *tick* (render loop); organelles (possible rename: ribosomes); GPU Body Model; *spine* for an ordered hyperedge
- **Part III names to confirm:** *assay* (the structural instrument — chosen over *stain*, *lens*, *probe*; *probe* is already taken by the Visibility compartment); *fold* / *unfold* / *seal*; *allele*; *coding region* / *regulatory region*; *hash boundary*; *turn*; *silent mutation*
- Watch: *frame* now carries a third load — truth context, Part II's coordinate-system near-miss (resolved as *Chart*), and the genuine floor of the fold stack (Part III §7.4)

---

## R14 — Expression (Phenotype = Genotype × Environment)  ·  status: adopted, details open

**Idea:** A cell's blueprint does not contain per-device layouts. The same DNA *expresses* differently depending on environment signals emitted by the host. "Similar experience across desktop, mobile, web" means same meaning and intents, not same pixels. (Part II §14, decided)

**Questions to research**
- What is the environment signal set? (available space, pointer precision, hover present, GPU tier / limits, power budget, safe areas, text scale)
- Is the host environment itself a real body at the top of the place graph, emitting signals like morphogen gradients?
- How are expression rules written in DNA — thresholds, gradients, constraint rules?
- Intent vocabulary: select, open/zoom-in, zoom-out, pan, link, inspect, undo — every intent must map in every environment to be valid
- Rule: hover (and any enhancement) may never carry meaning — progressive enhancement only
- Embryo vs adult: general pipelines in the creator vs DNA-specialized pipelines in compiled release — same DNA, two developmental stages
- GPU baseline: **core WebGPU limits** (web is WebGPU-only); cells needing Extended limits must declare it in the genome; no-WebGPU devices get a minimal expression, not a WebGL2 backend

**Starting points**
- Developmental biology: gene expression, phenotypic plasticity, morphogen gradients
- Responsive design / container queries (as the thing to improve on)
- Intent-based input systems, gesture recognizers

---

## R15 — Visual truth (every primitive declares its inverse)  ·  status: adopted, details open

**Idea:** "Opposition in all things" applied to presentation. A visual primitive that cannot state its inverse is untrue and rejected. (Part II §6, §17, decided)

**Opposition pairs**
- Draw ↔ pick (ID target)
- Chart transform ↔ inverse transform
- Visual output ↔ accessibility tree (seen by eyes ↔ seen by screen reader)
- Gesture ↔ undo
- Zoom-in threshold ↔ zoom-out threshold (hysteresis)
- Layout constraint (parent) ↔ measure (child)
- CPU pick ↔ GPU pick; embryo renderer ↔ adult renderer (Law 6)

**Questions to research**
- Minimal inverse contract each primitive (Shape, Text, Image, Region, Port, Link, Chart) must satisfy
- Perceptual tolerance for golden-pixel comparison across backends
- Chart chain (lens charts above, place-graph charts below; camera anchored to deepest chart; integer zoom level + fraction) so deep semantic zoom stays precise in f32; exclusive membership makes each body's chain unique per lens

**Starting points**
- Picking/hit-testing in retained scene graphs; object-ID buffers in CAD; AccessKit tree model
- Snapshot/golden-image testing for GPU renderers
- Floating-origin / relative-to-eye rendering (planetary and CAD viewers); map-tile zoom levels

---

## R16 — GPU Body Model  ·  status: adopted, details open

**Idea:** Keep the place graph, link graph, and membrane rules alive on the GPU as tables (bodies, cells, ports, links, incidence) instead of flattening the UI into anonymous primitives. JoInn's unique contribution at the graphics level. (Part II §7–13, decided)

**Link model (decided Sept 17, Part II §11.1)**
- **Touch-only law:** a hyperedge attaches at ports and never crosses a membrane; a connection that transits a body is two pairwise edges
- One form, two parameters: **order** (none | ordered) and **source** (tail members marked in the incidence list: zero = pure relation, one = sender, many = fan-in)
- Drawing: unordered → region (far) · hub (mid) · bundle (near, a hub with a trunk); ordered → **spine**, the region narrowed to a line touching each member in order
- An ordered hyperedge orders access; a transforming pipeline is a chain of pairwise edges
- Under collapse, an edge touches a lens node once however many of its members are inside — no re-entry loops

**Questions to research**
- Final record layouts, WGSL alignment, generational slots, table growth strategy
- Bind group layout under core limits (Tick · Universe · Genome · Pass)
- Delta protocol: one protocol, synchronous on web, channel to render thread on native
- ID target channel packing (body/lens node · cell · part or link member index · generation) and async readback latency
- Semantic zoom cut on CPU first; when a GPU compute cut is worth it
- Spine layout: keeping drawn order monotonic and minimizing crossings; routing around every membrane
- Snapshot keys (DNA hash, state hash, band, scale bucket) and atlas management
- Whether browsers' WebGPU compatibility mode can serve as the Core tier (vertex-stage storage buffers)
- Self-hosting test: can the creator show the Visual Host's own tables and organelles as cells?

**Starting points**
- Zed GPUI (SDF quads, atlases), Makepad (live shader DSL), Bevy ECS (entity tables), Flutter Impeller (layer tree, raster cache)
- Unreal Nanite (DAG cut by screen-space error)
- Eagle Mode, Pad++ / Piccolo (zoomable UIs)
- Figma WebGPU renderer (graphics abstraction, shader translation, fallback)

---

## R17 — Assays (structural measurement over blueprints)  ·  status: adopted in principle, details open

**Idea:** Topological structure is **derived** from DNA and a body's wiring — an instrument applied to a blueprint, never part of one. An *assay* measures a property of a sample without altering it. (Part I §10, Part III §9; the derived-not-merged decision is **DECIDED**.)

**Why not merged into DNA** (all four reasons, so this doesn't get relitigated)
- It breaks the hash boundary: a complex is a derivation, so gluing the same laws in a different order changes the hash — hashing a representation choice
- The instrument's limits would become the platform's limits: a structural measure handles relational, abelian content, so JoInn could never state "sorted(x) is a permutation of x" *at all*
- One-way door: every published blueprint welded to one encoding; changing the encoding rehashes the community's work
- It forces one complex, when R8 established that many lenses live over one structure

**What it measures**
- Independent regions inside a body that cannot reach each other
- **Unopposed cycles** — round-trips no law fills. Law 1 read structurally says there are none
- Laws closing around an empty interior: a specified but unimplemented cell — which is what a stem cell is before it differentiates (ties to R1, and gives "how much of this app is still promise" a number)
- **The one contracts cannot reach:** a system where every law holds at every membrane and yet no consistent global state exists. `require`/`ensure` are local by design, so this class is invisible to every contract in the app. This is the argument for the whole layer

**Questions to research**
- What is the minimal valid assay, and what is the reference implementation it is witnessed against?
- How is invariance under the hash boundary actually tested? (Rename a port, restyle, swap an allele, change device — the answer must not move. If it moves, the assay is reading phenotype and is inadmissible)
- Are declarations (`assert` lines in the coding region) laws, or a fourth kind of entry? Ties to R2, R3
- Is the incremental whole-universe form practical? Per-cell complexes are free; whole-universe is the valuable one and the expensive one. Local-to-global by nature, so glue per body along the link graph
- Does the **theory-level complex** (operations, relations, syzygies — what would catch loss of commutativity) earn its keep, or is the wiring complex enough? Not for Draft 0.1
- Coefficients: the frame supplies the group. What happens when a frame's values aren't an abelian group?
- **Decoration check:** does it catch anything `require`/`ensure` cannot? Two candidates claimed. If neither survives a real example, cut it — because it's derived, cutting costs an instrument, not a language

Phase 4: decoration check → KEEP, see Findings/decoration-check.md

**The test that decides it (Part III §14.1)**
Build the calculator's complex by hand (~15 blocks). Delete the law `parse(format n) = n` and measure. If the unopposed-cycle count goes 0 → 1 and returns `host → cli_a → sum → host`, the layer is real: it found a missing law from structure alone. If it takes special pleading about orientations and basepoints, keep only "boundary of a boundary is empty" and drop the rest.

**Starting points**
- Algebraic topology: CW complexes, chain/cochain complexes, homology and cohomology
- Abramsky & Brandenburger, sheaf-theoretic contextuality (local consistency without global consistency)
- Persistent homology tooling (for the incremental problem)
- Mayer–Vietoris (for gluing per-body results along the link graph)
- Discrete exterior calculus; Kirchhoff/circuit analogies for δ as residual

---

## R18 — Sealing (how a cell becomes a primitive)  ·  status: adopted in principle, details open

**Idea:** Primitives are not a different kind of thing from cells. A validated composite is **folded** — sealed — into a primitive: laws stated, fast implementation frozen, slow reference definition kept beside it. Unfold opens it again, which makes Fold/Unfold semantic zoom at the bottom of the stack. (Part I §4.1, §11.1; Part III §6.2, §7.)

**What a seal contains**
- Coding region (it is a cell)
- Reference allele — slow, obviously true, expressible in the minimal set
- Sealed allele — the fast native implementation
- Witness corpus — recorded agreements between the two

**Decided in principle**
- Folding is a **platform act, not a creator act**. Creators publish blueprints freely and petition for primitives rarely. Forth shows the alternative: everyone seals everything and the small set is a fiction within a year
- Admission requires a coding region, a reference allele, a witness corpus, and demonstrated use across independent bodies
- A sealed primitive and its reference are witnessed against each other on sampled inputs; **a disagreement is a truth violation, not a performance bug** (Law 6)
- The genuine floor is a **frame** — a declared axiom set — not an irreducible object

**Questions to research**
- Who may petition, and what evidence is enough?
- How are sealed primitives versioned and deprecated?
- What happens to bodies already using a primitive later found untrue? (Ties to R9 supervision and to witness/testimony)
- The reference path is far too slow for real data — what is the sampling strategy?
- Which law sets are **turnable** (solvable for a different unknown port), and how is that declared? Two unknowns means no answer, and the compiler must choose a direction per instance

**Starting points**
- Urbit: Nock's twelve opcodes and the jet dashboard; jet mismatch as the known scar
- Forth dictionary; APL/J derived verbs
- Coq extraction, CakeML, proof-carrying code (an allele with a certificate)
- Sussman & Radul, propagator networks (for *turn*)

## R63 — A vocabulary ban against a host language  ·  status: decided

**Decided 24 Sep:** bans apply to JoInn's language only (§2.14). Concept-word
bans scan corpus files and names this project declares in Rust; calls into
Rust's standard library (`x.saturating_sub(1)`, `fs::metadata`) are exempt.
Engineering bans (`HashMap`, `f32`, …) still scan all Rust unmasked.

## R55 — An augmented complex on a link  ·  status: open

When one tail fans out to several heads, is the value conserved (one delivery split, like a sum over heads) or copied (each head gets it all)? Example: `calc.sum@2` feeding both `units.scale@1` and a logger body. If augmentation ε counts deliveries, copying breaks ε∘∂ = 0.

Part V reading (Claude, 29 Sep, not decided): copied, not split.

## R56 — Embeddings across a link  ·  status: open

May a link carry a value from one frame into another by an embedding (ℤ into ℚ)? Example: two bodies whose laws each hold on their own frame, joined by an embedding whose round-trip holds on one side and not the other. This is the most likely place for Phase 4's first H¹ candidate.

## R58 — Correlation of a reply with its question  ·  status: open

*Example (Phase 5.2, `ask.universe`):* `lookup` asks `units` by sending 1 on `ask` and reads the factor back on `reply`. One question per run pairs correctly (12, then 5). Two questions in one run are both refused and nothing comes back. Correlating a reply with its question across two links needs either one question in flight or an order that spans both links.

## R59 — Interactive hosts and races  ·  status: open

In canonical prompt order, the CLI can't deliver two values into one port before its partner arrives. Is that a property or a limitation?

## R60 — Every file kind ships its mutants  ·  status: decided

Option A (AJ, 26 Sep): new grammar ships its mutants. A new file kind, or new grammar in an existing kind, comes with its mutation catalogue before any control may point at it.

## R61 — Whose budget is it  ·  status: open

Is a body's budget its own (declared) or borrowed from its container?

## R62 — One host, both sides  ·  status: open

When a host presents several bodies, does it show a link-caused refusal's reason (near side) or only its kind (far side)?

## R64 — Legacy gates  ·  status: open

Upgrade gates 1–3 to parsed subjects and a catalogue that covers `.cell`, `.desc`, `.trace` and `.rs`. That will be a short cleanup phase.

## R65 — Laws that span bodies  ·  status: open

H¹ with values needs a coding-level claim that two values in different bodies are the same quantity, and a factor stated in DNA. Where does that claim live: a `laws` section in `.universe`, a typed quantity frame, or something else?

## R66 — Loops with many frame changes  ·  status: open

Rule 3 judges loops with at most two frame changes. What pairs the conversions on a longer loop, and is the answer independent of the cycle basis?

## R67 — Hyperedges in the assay  ·  status: open

A link with k + 1 members is a k-block in the theory. Phase 4 prints such a link `not measured`. What are its faces?

## R68 — Promises and allele bodies  ·  status: open

The round-trip promise for an allele lives in the cell that carries it. Should an allele body be assayed in its owner's context?

## R69 — Descriptions of an instance that fires twice in a round  ·  status: open

The test host describes each firing after the round ends, so both descriptions carry end-of-round values (in `loop.universe`, `cli_a` prints `in "5"` twice; its first value `"2"` shows only at `sum@0`). Should a host describe at firing time, and would that move any Phase 3 description golden?

## R70 — A real weak device  ·  status: open

S2 and Phase 6 ran on an RTX 2080 and on software adapters. V31 asks for a real weak device in the test set. The first one AJ gets reruns `spikes/s2-gpu` and `cargo xtask pick` unchanged.

## R71 — Authored layout  ·  status: open

Layout is computed from the coding region. When a creator places a cell by hand, where does that live? A regulatory `layout` section (new grammar, so R60 applies), or the creator's own store?

## R72 — The vertex-buffer fallback  ·  status: open

`joinn-gpu` refuses an adapter without vertex-stage storage. S2 showed the fallback draws the same IDs, but it rewrites every instance row when a chart moves. When a compatibility-only target matters, is the fallback worth that cost?

## R73 — Text on screen  ·  status: open

The glyph organelle, a borrowed shaping crate, and text's inverse (point → character index). Planned with Phase 7's bands.

## R74 — One address, one port  ·  status: open

Floor primitives number in-ports and out-ports separately (`eq`: in 0, in 1, out 0), while cells number all their ports in one sequence. So `a@0` can name two ports, and the ID target (`PORT_TAG | position`) can't tell them apart. Phase 6 refuses such a body in the scene (`columns_reader.body`, `environment.body`). Should primitives number ports the way cells do (new grammar, so R60 applies), or should an address carry its direction?

## R75 — Membrane ports  ·  status: open

A body's own ports (`self@n`) have no place in Phase 6's layout, so a body whose wires reach its own membrane is not drawn (the nine phase 2.1/2.2 reference bodies print `not measured`). Phase 7 links bodies through their membranes. Where does a membrane port sit, and does a wire to `self` end on the membrane's edge?

> **29 Sep 2026 · Part V.** R76–R81 come from *JoInn Cells, Bodies and Forces* (Part V), written after Phase 6 closed. Part V also reframes two earlier items. **R74** may dissolve if cells in a body are no longer addressed by position. **R75** becomes: where does a *system* wire meet a body, on a protective cell or on the surface?

## R76 — Order inside a body  ·  status: decided

Combine is order-free. Where do order-dependent things live? Proposal: arithmetic ÷ and − are combine read at a turn, so the turn carries the order; presentation ("{0} + {1} = {2}") belongs to the host; anything else that depends on order is a system matter, wired between bodies. AJ said R76 is decided once the forces are settled, and they now are (Part V C19–C21).

**Decided:** AJ 29 Sep: combine is order-free; − and ÷ are combine at a turn; presentation is the host's; other order is a system matter.

## R77 — Where a force's truth lives  ·  status: decided

A force is not DNA: it is determinant and real (C16). The response law lives in the thing being forced, at each level (C10). Part V §4.1.

## R78 — The Floor re-read as forces  ·  status: attempted

**Attempted:** see `Findings/the-floor.md` §5 (29 Sep): no pair left; eq/choose kept paired (AJ).

Part V proposes the Floor sorts into two opposed forces (combine / separate, carry / release), distinction (`eq`, which is opposition itself and not a force), and the membrane (`bound` / `fill`), with `bind` / `unbind` moving to systems. Test it by the Floor's own rule: try to rebuild each current pair from these, and record each reduction that succeeds or fails before any amendment. Also: where does contact between cells in a body fit, as a new pair or as combine / separate at a body's scale?

## R79 — Link policy  ·  status: open

Every link resolves to a pinned edition at the moment it is used (C13). Where is a link's mode (pinned, live, watch) written: the coding region, since it changes behavior? How is an edition named? Can a body with a live link be sealed?

## R80 — Wrapping a body  ·  status: decided

A body never becomes a cell. Its cells differentiate into roles, and a wrapped body is wrapped by its own protective cells, not by a membrane (C17). Part V §6.1.

## R81 — Roles of cells in a body  ·  status: decided

Cells in a body take roles: protect, respond, carry, store, and so on. Is the set of roles a short fixed list, or generative like kinds (C18)? Roles must not grow into a registry.

**Decided:** AJ 29 Sep: (faces out/in) × (holds/reacts) = protect, carry, store, respond; derived, never written.

## R82 — The opposite of a lossy combine  ·  status: open

Sum forgets its parts, so separate can only be a turn given one part back. Structural combine (`build`, `pair`) is lossless, and its separate (`case`, `split`) is exact. Is every combine one of these two, and should the register say which?

## R83 — Multiplication  ·  status: decided

Is `a × b` combine *across* dimensions (length × length = area) while `a + b` is combine *within* one? If so, a product is the same act as `pair`, with a dimension instead of no frame (R38).

**Decided:** AJ 30 Sep: multiply is combine across dimensions; Part VI K9–K10 (stacking, order-signed).

## R84 — Many members, chained forces  ·  status: open

A fold over more members than the response has in-ports, and layout for members that don't touch in one column. The beam will need both.

## R85 — One port, two forces  ·  status: open

Can a value be combined in two places? Part V's reading is that values are copied, not split (R55), which makes it carry, not combine.

## R86 — The two open Floor reductions  ·  status: open

`pair`/`split` into a frameless combine (needs R38), and `join` as combine's completion rule. Attempt them in code when carry arrives.

> **1 Oct 2026 · Part VI.** R87–R97 come from *JoInn Dimension* (Part VI) and the
> Phase 7 Stop C review. Part VI answers R88 and R89 and partly answers R92.

## R87 — Presentation names parts  ·  status: open

From the Phase 7 Stop C review: `{0} + {1} = {2}` indexes ports only the engine assigns. Should presentation name parts (`{cli_a} + {cli_b} = {sum}`) instead? The beam's part names are the natural test.

## R88 — Base kinds  ·  status: answered (Part VI K18–K19: no base kinds; counting, where, side)

**Answered by K18–K19:** there are no base kinds. The base is counting, where and side; the rest is derived or a bridge.

## R89 — Orientation  ·  status: answered (Part VI K13a: part of where; a sign convention is presentation)

**Answered by K13a:** orientation is part of where, carried by every quantity; a sign convention is presentation. Open: how inner and outer orientation (Tonti's terms) are written in the tag.

## R90 — Is carry the mirror?  ·  status: open

Combine and separate stay on one side. Does carry, across a relationship, *always* cross from one body's source side to another's, as *bears on* does? If so, the force pairs and the mirror are one structure.

## R91 — Time as pieces  ·  status: open

Instants and spans of time as pieces. Does a load held over time, or a run of the engine, use the same complex? Ties to R11 (ordering without a global clock).

## R92 — The continuum  ·  status: partly answered (Part VI K20: polynomial loads derive exactly)

**Partly answered by K20:** for polynomial loads, adding up along the span in ℚ is exact, so wL²/8 and 5wL⁴/384EI are derived. Open: loads and shapes that are not polynomial (a sine load, a tapered member), where exact adding up has no finite answer. Are those derived to a stated bound, or witnessed only?

## R93 — Beyond linear bridges  ·  status: open

Across-the-mirror laws are often not straight lines (yielding, cracking, buckling). Does the bridge stay one pinned edition with witnesses, or does it need state?

## R94 — Tags and the gate  ·  status: open

How does the evolution gate treat a tag change? Adding a tag to an untagged ℤ port is new truth. Changing "force per line" to "force" is a different cell. Ties to R27 (conservative extension).

## R95 — Sensitive problems  ·  status: open

Buckling, slender members and diverging analyses are chaotic in the scientific sense (K17). How does JoInn show that a result is determined but sensitive: a witness of how far the answer moves when the input moves by the smallest step its frame can state?

## R96 — Kinds as bodies  ·  status: open (murky; go forward)

K19 builds kinds into cells and bodies. What exactly does a kind-defining body hold: the pair, its bridge, its energy product? How does a cell say which pair it belongs to without that becoming a registry? Murky (AJ, 1 Oct); JoInn goes forward and lets testimony grow.

## R97 — Witness libraries  ·  status: open

Roark is the first witness (K20). How is a witness library entered (pinned edition, cases, conditions), how many must agree, and what happens when two libraries disagree with each other but not with JoInn?

## R98 — Indeterminate beams  ·  status: open

A propped cantilever needs compatibility (placement side) and a bridge to find its reactions. Is "balance reads no bridge" then only true for determinate beams, and does the order of the mirror change (placement first)?

## R99 — Where a section property lives  ·  status: open

Ix is the W12x26's section, a 2D body one zoom level in (Part VI §9). Is it a bridge (testimony from the shapes table) or derived from the section's own complex, with the table as its witness?

## R100 — The decimal  ·  status: open

A printed decimal is presentation. Who chooses its places: the host, the quantity's tolerance, or the edition's precision?

> **3 Oct 2026 · Phase 7.2.** R101–R104 come from the zoom plan.

## R101 — Where the cut lives  ·  status: open

Decided twice (GPU per instance, CPU on demand) from the same integers. Is "two cuts, one truth" the general rule for every derived view (accessibility in Phase 8)?

## R102 — Hysteresis or fade  ·  status: open

A size-driven fade replaces separate up/down thresholds. Does any band change need hysteresis once bands carry live interiors (editing in Phase 9)?

## R103 — Text as cells  ·  status: open

Labels and values are strokes owned by a cell or port. When a label is edited in the creator, is a label a cell of its own, or presentation of a name?

## R104 — Layout of a lens  ·  status: open

The grove's grid is fixed (6 / 4 / 4). Is a lens's layout a body of its own (a layout system), so other lenses can lay out differently?

> **3 Oct 2026 · Phase 7.3.** R105–R108 come from the links plan and the 7.2 review.

## R105 — Snapshots  ·  status: open

What measurement would make them necessary? Draw-time and instance counts are in `phase-7.3-links.md`. Until something is slow, they wait.

## R106 — Streets as structure  ·  status: open

The gutters come from the grid. If a lens's layout becomes a body of its own (R104), are its gutters part of its contract?

## R107 — The knot as a place  ·  status: open

A hub's knot is a 1-median on the streets. Does it mean anything (a junction, a bus, a shear tab), or is it presentation only?

## R108 — The engine's cost per fire  ·  status: open

`agree` (Phase 2.1/2.2's gates and CI's `agree` step) spends about 100 s cloning cells per delivery and re-hashing coding regions per fire. Should a cell's hash be computed once at admission and carried, and deliveries share cells instead of cloning them? A truth-core change: planned on its own, with `agree`'s output byte-identical as its gate.

> **6 Oct 2026 · Part VII and Phase 7.4.** R109–R117 come from *Systems, Forces and Growth* (Part VII) and the 7.4 plan.

## R109 — Systems and lenses  ·  status: answered (G17)

If a system is the bodies its forces reach, what is a lens's system? Proposal: the force system is the one real grouping, and lenses group systems, never bodies. Answered by G17: the lens is the view.

## R110 — Links as carry  ·  status: answered (G18)

Is a link a lasso of kind carry? Test: write Phase 5's link checks as carry's laws and see what is left over. Answered by G18: links transfer data while forces stimulate.

## R110a — Carry, the force  ·  status: open

From Part VII: is *bears on* a link or a force? Not needed until the beam.

## R111 — What is consumed  ·  status: open

If combine keeps its inputs (G15), is "consumed" a property of carry instead?

## R112 — Counting and measuring  ·  status: open

Where exactly does counting end? Exact rationals are counting; √2 is not. Is "measuring" one new system, or the edge Part VI's continuum (R92) already names?

## R113 — The app as a cell  ·  status: open

Is "an app has the shape of a cell" a law the gate checks, or a habit?

## R114 — Lineage as learning  ·  status: open

Can the saves system show an app's evolution as a learning record, and who may see it?

## R115 — Removing a number  ·  status: open

Growth only adds. Can a body shrink (she erases a number), and is that growth backwards, or an edit?

## R116 — Where state lives  ·  status: open

A grown state is the input list. When the saves system exists, it stores that list. Is the list itself testimony with lineage (her teacher sees *what* she counted and *when*, R114)?

## R117 — Systems of several bodies  ·  status: open

A system holds one body here. With two (the beam and its column), does one lasso hold both, or one lasso per body?
