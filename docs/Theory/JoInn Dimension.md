# JoInn Dimension

**Part VI: where a quantity lives, and why opposition is a mirror**

*Theory only · no implementation implied*

Author: AJ, with Claude · Draft 0.2 · October 1, 2026

> Status tags follow Part I: **DECIDED** (AJ said it), **PROPOSED** (Claude's reading, open to AJ's veto), **OPEN**. Decisions are numbered K1–K20 in §12. Research items R87–R97 are in §13.

> **Thesis.** A dimension is not a list of exponents. **A dimension is where a quantity lives**: on a point, a line, a surface or a volume, at an instant or across a span of time. It also says which side of the mirror the quantity is on. **Nothing that can be derived is a base**: under counting, where and side there are no base kinds, only bridges, and bridges are testimony. Every physical theory has two sides that mirror each other: how things are placed, and what pushes them. **Opposition is that mirror.** The laws that connect pieces of space are exact, and they are JoInn's truth. The laws that cross the mirror are material and measured constants, and they are JoInn's pinned library editions. Formulas are derived, and libraries such as Roark are witnesses.

---

## For AJ: this note in plain English

- **A dimension says where a quantity lives.** Deflection lives at a point. A load spread along a beam lives on a line. Stress lives on a surface (a cut). Weight lives in a volume. That is what "defined topologically" means: the dimension is the size of the piece of space the quantity sits on, its *k* (point 0, line 1, surface 2, volume 3).
- **The old exponents are a shadow.** "kip per foot" is a force spread over a line. "ksi" is a force spread over a surface. Flatten the picture and you get the exponent list. The picture says more: stress and pressure share ksi but are different things.
- **Every quantity is on one side of a mirror.** The *placement* side is how things sit and move: displacement, strain, curvature. The *source* side is what pushes: loads, stresses, moments, reactions. Each quantity on one side has a partner on the other.
- **Opposition is the mirror.** "Equal and opposite" is the statement that the two sides balance. A *bears on* relationship is the mirror at a support: the beam's reaction going down is the column's load coming up.
- **What connects pieces of space is exact, and that is truth.** Forces at a joint sum to zero. Pieces that touch move together. Neither law needs a measurement, so neither can be wrong in the way a formula can.
- **What crosses the mirror is testimony, and that is the code.** Stress = E × strain is the only place material and measurement enter. Codes and material tables are the bridge between the two sides. That is why they are pinned editions: the bridge can be refined, and the topology under it never moves.
- **Adding stays on one piece of space. Multiplying stacks pieces.** A line times a line gives a surface; the k's add. Adding is **order-blind**: it always happens in an order, but the answer can't see it. For multiplying, order can flip the sign (r × F = −F × r). A moment has a sign convention for a reason.
- **Nothing is unordered, and nothing is random.** Everything happens in an order that can be traced. A law says only how much a result depends on that order. What looks like chaos is order not yet known, which is what chaos theory itself says.
- **Nothing derived is a base.** Force is derived (the rate momentum changes). Mass is a bridge, like E. Moment is the shears added up along the span. Even SI now builds the second, the meter and the kilogram from counting and a few fixed constants. So JoInn's base is **counting, where, and side**. Everything else is built.
- **Kinds are built in cells and bodies, not listed.** A body that defines a pair (force with displacement, heat flow with temperature) and its bridge is where testimony starts. It grows as we learn.
- **Formulas are derived; libraries witness.** JoInn adds up w to get V and V to get M, exactly, so wL²/8 comes out on its own. The library holds only the bridges (E, Fy, section properties, φ, constants). Roark and other libraries become witnesses to check against, which lets JoInn go where Roark has no case.
- **One word, three counts.** *Dimension* is this. *Arity* is how many ports a relation touches (R4b). *Space* is whether a picture is 2D or 3D (R4a). All three count k, on three different things.

The running example is the steel beam from Part V §9, read again in §9.

---

## 1. Why This Note

Part V §3 lists **dimension** as cell metadata ("what can combine with what"). Part V C9 says a kind is a frame plus a dimension. AJ decided on 30 Sep that multiplication is **combine across dimensions** (R83). And Part I said, early, that **dimension is to be defined topologically**. Nothing yet said what a dimension *is*.

The obvious answer is the one every units library uses: a quantity carries exponents of a few base units, so ksi is force¹ length⁻². AJ chose not to start there (1 Oct): **work out what topological dimension means first, so units are built on it.** This note does that. It ends at the same exponents, as a derived view, but it starts from where the quantity lives.

---

## 2. Three Meanings of Dimension, One k

JoInn has used "dimension" three ways. The backlog already split two of them (R4a, R4b) to stop a naming collision.

| Meaning | Counts | Lives on | Proposed word |
|---|---|---|---|
| Relational | how many ports a relation touches: port 0, wire 1, law 2 | **the program's** structure (the assay, Part III §9) | **arity** |
| Spatial | whether a picture is flat or solid | **the screen** (Part II, R4a) | **space** |
| Physical | where a quantity lives: point 0, line 1, surface 2, volume 3 | **the thing being modeled** | **dimension** |

The claim of this note (K2, **PROPOSED**): **these are one idea, the k of a cell in a complex, applied to three different complexes.** The assay already reads a program as points, edges and faces. The picture already places shapes in a 2D space. Part VI reads the *modeled thing* the same way. So one machinery serves all three, which is the fractal at work, and three words keep them from being confused (K3, **PROPOSED**).

---

## 3. Where a Quantity Lives

A model of a physical thing is a **complex**: points, lines between them, surfaces bounded by lines, volumes bounded by surfaces. Time has its own: instants, and the spans between them. A quantity is **a number attached to each piece of one kind** (K1, **DECIDED**: AJ, 1 Oct).

| Lives on | k | Beam examples | Other examples |
|---|---|---|---|
| a **point** | 0 | deflection at midspan; a support reaction | temperature at a spot; voltage at a node |
| a **line** | 1 | the span; a load along the beam (w) | strain along a fiber; current through a wire |
| a **surface** | 2 | stress across a cut | flow through a face; heat through a wall |
| a **volume** | 3 | self-weight of the member | mass; stored energy |
| an **instant** | 0 in time | a reading taken now | |
| a **span** of time | 1 in time | an impulse; a load held for 10 minutes | |

**Total or per-piece.** A load of 28.8 kip on a 24 ft span is a total on the line. 1.2 kip/ft is the same quantity *per* foot of that line. A total attached to a k-piece, divided by the size of the piece, picks up "per length^k". That is where the exponents come from.

**The shadow** (K4, **PROPOSED**). The exponent list of ordinary units is what is left when the picture is flattened: the kind contributes its base units, and the piece of space contributes length or time to the power of its k. Nothing is lost by computing the exponents this way. What is lost by *starting* from exponents is the difference between two quantities with the same exponents on different pieces: stress (a force across a surface) and pressure on a wall (a force against one) share ksi but differ in direction and orientation.

---

## 4. The Two Sides

Every classical physical theory, read this way, has the same shape. This is Enzo Tonti's finding (§14): the variables of mechanics, heat, electromagnetism and fluid flow sort into **two families** that mirror each other.

| | **Placement** side | **Source** side |
|---|---|---|
| Says | how things are placed and how they move | what pushes, pulls, flows |
| Mechanics | displacement, rotation, strain, curvature | load, reaction, stress, shear, moment |
| Heat | temperature, temperature drop | heat flow, heat source |
| Circuits | voltage at a node, voltage across a wire | current in a wire, current into a node |

Each side lives on its own complex, and the two complexes are mirror images of each other: **a point on one side faces a volume on the other, a line faces a surface**. In a flat (2D) model, a point faces a surface and a line faces a line. This mirror is called **duality** (K5, **PROPOSED**).

**Two kinds of law follow, and they are different in kind** (K6, **PROPOSED**):

1. **Laws within one side.** On the source side: the forces on any piece balance (equilibrium). On the placement side: pieces that meet move together (compatibility). These laws use **only how the pieces connect**. No length, no material, no number from a table. They are exact, and topology alone guarantees them. Kirchhoff's two laws for circuits are the same two laws.
2. **Laws across the mirror.** Stress = E × strain. Heat flow = conductivity × temperature drop. Current = voltage ÷ resistance. These need the **metric** (real lengths and areas) and the **material**. They are measured, approximate, and refined over time.

---

## 5. Opposition Is the Mirror

Part I made *opposition in all things* the validation law. Part V said distinguishing is opposition itself, the gate's eye, and that *bears on* is where opposition is physical. This note gives that a structure (K7, **DECIDED**: AJ, 1 Oct):

> **Opposition is the mirror between the placement side and the source side. Every quantity has its partner across the mirror, and balance is the statement that the two agree.**

- **Equal and opposite reaction** is the source side's balance law on a piece that two bodies share. At a support, the beam's reaction and the column's load are the same quantity read from the two bodies' sides.
- **A *bears on* relationship is the mirror at the boundary between two bodies.** It carries one source quantity across, and the balance law holds it equal and opposite. That gives the relationship in Part V §7.1 its law.
- **Distinction is where balance is checked.** `eq` sees whether the two sides agree. A ratio above 1.0 is a source quantity the placement side can't carry: the gate refuses it. Part V's beam drawing red is this check.
- **The pairs of forces are the same mirror, one level up** (**PROPOSED**). Combine and separate stay on one side: they gather or split quantities that live on the same kind of piece. Carry and release cross a relationship. Whether carry *is* the crossing of the mirror is R90.

---

## 6. Truth and Testimony

This is the result AJ will use most often, so it is stated on its own (K8, **PROPOSED**).

| Laws within one side | Laws across the mirror |
|---|---|
| connection only | metric and material |
| exact; cannot be wrong if the pieces connect correctly | approximate; testimony about nature |
| equilibrium, compatibility, equal and opposite, and what is derived from them by adding up | E, Fy, section properties, resistance factors φ, measured constants |
| **truth**: gate laws, checked exactly | **testimony**: pinned library editions (Part V C13, C14) |

Part V decided that engineering stays pinned to the code it was built to, while codes change as testimony about nature grows. Under this note that is not a policy laid on top of the model. **It is where the model's seam is.** The codes and material tables are exactly the bridge between the two sides. A new edition of AISC replaces the bridge and leaves the topology under it alone. A design signed under one edition stays true for that edition, because the exact half never moved and the bridge it used is sealed.

This also says what JoInn can check **exactly** and what it can only **witness**:

- **Checked exactly:** the beam's reactions add up to its total load; the column receives what the beam gives; the pieces connect. No tolerance, no edition.
- **Derived exactly, then witnessed:** M = wL²/8 and δ = 5wL⁴/384EI. JoInn derives both by adding up along the span (§9). Roark's edition states the same formulas, and the two are compared: a disagreement is a truth violation, as when a sealed primitive disagrees with its reference.
- **Testimony:** E, Fy, the section's properties, φ, and the constants under the units. These cannot be derived from connection and counting. They are the bridges, held as pinned editions.

---

## 7. Combine Within, Combine Across

AJ decided (30 Sep, R83) that adding is combine *within* a dimension and multiplying is combine *across* dimensions. Under this note (K9, **PROPOSED**):

- **Adding** combines quantities on **the same kind of piece, on the same side, of the same kind**: kip/ft with kip/ft on one line. It is **order-blind** (§7.1), exactly as Phase 7's register checks.
- **Multiplying** **stacks pieces**: a line times a line is a surface. **The k's add.** A number with no piece (a count, a ratio, the 8 in wL²/8) has k = 0 and changes nothing.
- **Dividing** is multiplying read at a turn, as subtraction is sum read at a turn. The k's subtract.
- **Stacking has orientation.** A line has a direction; a surface has a side. Stack two lines in the other order and the surface faces the other way: **a × b = −(b × a)** when both have odd k. That is why a moment, lever arm × force, has a sign convention, and why r × F = −F × r.

### 7.1 Order is always real

Phase 7 called combine **order-free**. AJ rejected the word (1 Oct), and he is right: **nothing happens without an order.** When JoInn's engine combines 2 and 3, it combines them in a definite order (members sorted by name), every time, and that order can be traced and predicted. The Phase 7 Stop C review even caught it showing through: rename `cli_a` and the present line flips to `3 + 2 = 5`.

What Phase 7 tested was something narrower and true: **the answer doesn't depend on the order.** The order happened; the answer can't see it. So the word is **order-blind** (K15, **DECIDED**: AJ, 1 Oct).

The full principle (K16, **DECIDED**: AJ, 1 Oct):

> **Order is always real, and JoInn records it. A law may say how much a result depends on that order: not at all (order-blind), only by a sign (order-signed), or completely (order-bound).**

| How a result depends on order | Name | Examples |
|---|---|---|
| not at all | **order-blind** | adding loads on one line; combining counts |
| only by a sign, which the members' k's decide | **order-signed** | lever arm × force (r × F = −F × r); stacking two lines into a surface |
| completely | **order-bound** | a load sequence; construction stages; subtraction read at a turn; any system's wires |

**What this does to Phase 7's check** (K10, **PROPOSED**). Phase 7 requires every registered combine to pass the order check. That stays true for combine within (adding): order-blind. Combine across (multiplying) is **order-signed**: swapping the members either changes nothing or flips the sign, and *which one* is decided by the members' k's alone, never by the values. The register's check grows from "order changes nothing" to "order changes at most the sign, as the k's say". JoInn must refuse a result whose sign depends on which member happened to come first. In code, `order_free` and rule 63 are renamed to *order-blind* in the next plan; the rename changes the printed lines of `cargo xtask forces`, so it is a planned commit.

### 7.2 Chaos is order not yet known

JoInn has no randomness (K17, **DECIDED**: AJ, 1 Oct). Samples are seeded (the order check uses seed 7 so every run is the same), combine order is canonical, identity is a hash, and every run replays exactly.

The word **chaos** is used only in its scientific sense, which matches AJ's position. Chaos theory studies systems that follow **deterministic laws** yet are **highly sensitive to their starting conditions**. Lorenz: *"when the present determines the future, but the approximate present does not approximately determine the future."* A chaotic system has a complete order. It is unpredictable only to someone who doesn't know its starting state exactly. **What looks like chaos is a lack of knowledge, not a lack of order.**

The everyday sense of chaos (disorder, chance in charge) is never meant. Chaos is not a property of combine: adding is the opposite, a result that doesn't care about its starting order at all. Chaos belongs to problems JoInn will meet later: buckling near a critical load, slender members, an analysis that diverges. JoInn treats them as fully determined, records the exact inputs, and replays them exactly. It never calls them random (R95).

---

## 8. The Tag on a Quantity

Every quantity carries a tag with three parts and a measuring stick (K11, **PROPOSED**, revised in Draft 0.2):

| Part | Says | Beam examples |
|---|---|---|
| **Where** | the piece it lives on (k in space; instant or span in time), **with its orientation**, and total or per-piece | w: per-piece, on a line. δ: on a point |
| **Side** | placement or source | w, M, reactions: source. δ, L: placement |
| **Pair** | which mirror it belongs to: the body that defines the pair and its bridge | mechanical (force with displacement); thermal (heat flow with temperature) |
| *Unit* | the measuring stick: ft, in, kip, kN | not part of the dimension |

**Units are not dimensions.** A foot and an inch measure the same thing. Converting between them is an exact fraction (12 in = 1 ft; 1 kip = 1000 lb; 1 in = 2.54 cm exactly), so JoInn checks conversions exactly over ℚ and never rounds. A unit belongs to presentation and input, the way a regulatory label does. The dimension belongs to truth.

**Where the tag lives** (K12, **PROPOSED**). The tag is part of a port's **frame**: in the coding region, hashed and judged. A port that says "force per length, on a line, source side, in ℚ" refuses a length, the way a membrane refuses `"two"` at a port in ℤ. Part V's *receptor* rule becomes exact: **a force reaches only the cells whose tag fits it.** The body's actual complex (which points and lines, which pieces touch) is **derived** from the body, never written in it. That keeps Part III §9.1 and AJ's rule that topology is an added layer, never DNA written in k-blocks.

### 8.1 Nothing derived is a base

Draft 0.1 said length and time are not kinds (K14) but kept force, mass, temperature and charge as base kinds. AJ rejected that (1 Oct): **force and mass depend on length and time, so they can't be bases either. A thing that can be derived is not a base** (K18, **DECIDED**).

Followed strictly:

- **Force is derived.** It is the rate at which momentum changes: balance on the source side.
- **Mass is a bridge.** It links how fast something moves (placement side) to the momentum it carries (source side). It is a property of the material, like E. Weight is mass times g, and g is a measured constant.
- **Moment is derived.** It is the shears added up along the span, and shear is the load added up (§9).
- **The units themselves agree.** Since 2019, SI defines the second as a **count** of a cesium atom's oscillations, the meter as how far light goes in a counted span (fixing the speed of light, c), and the kilogram by fixing Planck's constant, h. Units are counting plus a few fixed constants. The constants are bridges: c links space to time, h links a frequency to an energy. Their values were fixed by agreement, which is an edition.

So the base of JoInn's dimensions is (K19, **DECIDED**: AJ, 1 Oct, as the direction; details still open):

| Base | What it is | Where JoInn already has it |
|---|---|---|
| **Counting** | exact numbers | the frames ℤ and ℚ |
| **Where** | the piece, with its orientation | the derived complex (K1) |
| **Side** | placement or source | the mirror (K7) |
| **Bridges** | constants and material laws that cross the mirror | pinned library editions; testimony, not base |

**There are no base kinds.** A foot, a kip and a second are measuring sticks defined through pinned bridges.

### 8.2 Where kinds come from

Something still tells a force from a heat flow: both are source-side quantities that can sit on the same kind of piece. What separates them is **which pair they belong to**. Force pairs with displacement, heat flow with temperature, current with voltage. In every pair, the two partners multiplied together give **energy**, the one currency every mirror shares, and energy itself comes from counting through h.

AJ's reading (1 Oct, **DECIDED** as direction): **kinds are built into a cell or body that defines them and processes them, perhaps in a system.** A mechanics body defines the force–displacement pair and holds its bridges. That body is **where testimony starts, and it grows as we learn**. Kinds are never a list: each is a body, admitted and evolved like any other, by the path of truth. This is still murky in places (R96), and it is the direction JoInn goes forward in.

### 8.3 Orientation is part of where

R88 and R89 are one question (AJ, 1 Oct). Once kinds dissolve, orientation can't be an optional extra: it belongs to *where*. A line has a direction; a surface has a side. So **every quantity carries its orientation as part of where it lives** (K13a, **PROPOSED**). A moment's sign comes from the orientation of the section and the bending plane; it is not chosen. The **sign convention** engineers argue about becomes presentation: the host chooses which way shows as positive, and the truth underneath does not move.

---

## 9. The Beam, Read Again

Part V §9's beam, B1: 24 ft, pinned–pinned, W12x26, A992, w = 1.2 kip/ft.

**First finding: the beam is not only a line.** In M = wL²/8, L² is not the area of anything on a line. It is span × lever arm, stacked in the **plane the beam bends in**: the span direction and the load direction. So the beam's complex is its **elevation**: a 2D picture, which is the one engineers already draw. A 1D model has no surfaces, so it has nowhere to put a moment (K13, **PROPOSED**).

| Quantity | Where | Side | Pair | Comes from |
|---|---|---|---|---|
| L, span | line | placement | mechanical | the grid (live link) or the cell's own value |
| w, load | line, per-piece | source | mechanical | the load case: a force from the system |
| R_A, R_B, reactions | points (supports) | source | mechanical | balance: exact |
| V(x), shear | at a section | source | mechanical | w added up from the support: exact |
| M(x), moment | at a section, in the elevation plane | source | mechanical, oriented | V added up along the span: exact |
| E | **crosses the mirror** | — | the mechanical bridge | material library, pinned edition |
| I_x | the **section's own** surface | placement | geometry of the section | shapes library, pinned edition (or derived from the section's own complex) |
| δ, deflection | point | placement | mechanical | M/EI added up twice: exact, given the bridges E and I |
| ratio M / φMn | a plain count | — | none | distinction checks it ≤ 1.0 |

What the table shows:

- **Reactions, shear and moment are exact.** For a statically determinate beam they come from balance alone: R_A + R_B = wL, with no material and no edition. JoInn can check them as truth.
- **The moment is the shears added up** (AJ, 1 Oct). Load gives shear and shear gives moment: dV/dx = −w and dM/dx = V. On JoInn's pieces, integrating is adding up along the span, which is **combine**, and adding is order-blind. Under a uniform load the shear is a straight line, so adding it up exactly in ℚ gives M = w·x·(L − x)/2 at every point, and wL²/8 at midspan. No formula is looked up.
- **Deflection is derived, through a bridge.** It is M/EI added up twice, exactly; 5wL⁴/384EI comes out. What it needs from testimony is only E (material) and I (section). Change A992 to A572 Gr 65 and E stays 29,000 ksi, so δ does not move. That is Part V §9's Rev A → Rev B, now with a reason: **Fy and E sit on different bridges.**
- **The section is a body of its own, one zoom level in.** I_x is a property of the W12x26's cross-section, which is a 2D shape with its own complex. Zoom into the beam and the section opens into a surface whose pieces give I_x. That is semantic zoom meeting dimension: the fractal holds.
- **The 8 and the 384 are derived, and Roark witnesses them** (K20, **DECIDED**: AJ, 1 Oct). Draft 0.1 said they were the library's. They are not: they come from adding up exactly along the span. The library's job changes. It holds **only the bridges** (E, Fy, section properties, φ, constants), which cannot be derived. **Roark and other libraries become witnesses**: JoInn derives each formula and compares it with every library that states one. A disagreement is a truth violation. Because the formulas are derived and not looked up, JoInn can reach **cases no library covers**, and each library that does cover a case adds a witness.

---

## 10. What This Means for Phase 7.1

Nothing is replanned by this note. Phase 7 is closed and stands. AJ's 30 Sep decisions stand: the beam is Phase 7.1, split into *the beam computes* and *the beam connects*; multiply is combine across dimensions. The 30 Sep choice that formulas come from a pinned library edition is **revised by K20** (AJ, 1 Oct): formulas are derived; the library holds bridges; Roark is a witness.

What Phase 7.1 would need, if this note is accepted (**PROPOSED**, settled in the plan):

1. **ℚ with a turn**, so fractions can combine in a contact body (Phase 7's finding: no turn, no combine).
2. **The tag in the frame** (§8): where (with orientation), side, pair. Units as exact conversions at input and presentation only.
3. **A register row for multiply**, with the signed order check (§7).
4. **The elevation complex** for the beam (§9): supports as points, the span as a line, the bending plane as the place a moment lives.
5. **Balance as the first exact law**: reactions sum to the total load, checked as truth, before anything else.
6. **Derivation by adding up**: V from w, M from V, δ from M/EI, all exact in ℚ.
7. **A tiny pinned bridge edition**: E for A992 and I for W12x26, entered like a stranger's data.
8. **Roark as the first witness**: the derived wL²/8 and 5wL⁴/384EI compared with Roark's statement of them.

Items 5 and 6 are the strongest first test. If JoInn can say the beam's reactions balance its load, and derive wL²/8, without a single material number or looked-up formula, the mirror is real in the code, not just in this note.

---

## 11. The Honest Adversary

*This is just units with extra words.* A units library with exponent lists computes wL²/8 correctly and needs none of this.

The note answers it with three things exponents cannot do, each testable:

1. **Tell stress from pressure.** Same exponents, different pieces of space and different orientation. If no JoInn test ever needs that distinction, the "where" part is decoration and should be cut to exponents.
2. **Separate truth from testimony mechanically.** Balance laws must be checkable with no material and no edition. If the beam's reactions can't be checked without a library, the mirror isn't doing work.
3. **Catch a sign that depends on order.** A moment computed as F × r in one place and r × F in another must be refused. Exponents can't see this.

If all three fail on the beam, this note is a vocabulary, not a theory, and Phase 7.1 should fall back to exponents.

---

## 12. Decisions

| # | Decision | Status |
|---|---|---|
| K1 | A dimension says where a quantity lives: the piece of space (point, line, surface, volume) and of time (instant, span) it is attached to | **DECIDED** (AJ, 1 Oct) |
| K2 | Arity, space and dimension are one idea, the k of a piece in a complex, applied to the program, the screen and the modeled thing | **PROPOSED** |
| K3 | Words: *dimension* is the physical one; R4b's is *arity*; R4a's is *space* | **PROPOSED** |
| K4 | Unit exponents are a derived view: kind plus where, flattened | **PROPOSED** |
| K5 | Every quantity is on the placement side or the source side; the two sides mirror each other (duality) | **PROPOSED** |
| K6 | Laws within one side use connection only and are exact; laws across the mirror need metric and material | **PROPOSED** |
| K7 | Opposition is the mirror between placement and source. Balance is the statement that the two agree | **DECIDED** (AJ, 1 Oct) |
| K8 | Exact laws are truth (gate laws). Laws across the mirror are testimony, held as pinned library editions | **PROPOSED** |
| K9 | Adding combines on one piece, side and kind. Multiplying stacks pieces, and the k's add. Dividing is multiplying at a turn | **PROPOSED** (refines AJ's R83 decision) |
| K10 | Combine within is order-blind; combine across is order-signed, by a sign the k's decide; the register checks both | **PROPOSED** |
| K11 | A quantity's tag is where (with orientation), side and pair. A unit is a measuring stick, converted exactly, and is not part of the dimension | **PROPOSED** (revised in 0.2) |
| K12 | The tag is part of a port's frame, in the coding region. The modeled thing's complex is derived, never written | **PROPOSED** |
| K13 | A beam is modeled in its elevation plane; a 1D line has nowhere to put a moment | **PROPOSED** |
| K13a | Orientation is part of where, carried by every quantity. A sign convention is presentation | **PROPOSED** (answers R89) |
| K14 | Length and time are not kinds; they are where a quantity lives | **superseded by K18** |
| K15 | The word is **order-blind**, not order-free: the order happens, the answer can't see it | **DECIDED** (AJ, 1 Oct) |
| K16 | Order is always real, and JoInn records it. A law says how much a result depends on it: order-blind, order-signed, or order-bound | **DECIDED** (AJ, 1 Oct) |
| K17 | JoInn has no randomness. Chaos means only the scientific sense: determined by its laws, unpredictable only without exact knowledge of the start. What looks like chaos is a lack of knowledge | **DECIDED** (AJ, 1 Oct) |
| K18 | Nothing that can be derived is a base. Force is derived; mass is a bridge; moment is the shears added up | **DECIDED** (AJ, 1 Oct) |
| K19 | The base is counting, where and side. There are no base kinds. Bridges (constants, material laws) are testimony. Kinds are built into cells and bodies that define and process them, where testimony starts and grows | **DECIDED** (AJ, 1 Oct, as direction) |
| K20 | Formulas are derived exactly; the library holds only bridges; Roark and other libraries are witnesses, and a disagreement is a truth violation. Revises the 30 Sep choice | **DECIDED** (AJ, 1 Oct) |

---

## 13. Open Questions

| ID | Topic | Question |
|---|---|---|
| **R87** | Presentation names parts | From the Phase 7 Stop C review: `{0} + {1} = {2}` indexes ports only the engine assigns. Should presentation name parts (`{cli_a} + {cli_b} = {sum}`) instead? The beam's part names are the natural test |
| **R88** | The base kinds | **Answered by K18–K19:** there are no base kinds. The base is counting, where and side; the rest is derived or a bridge |
| **R89** | Orientation in practice | **Answered by K13a:** orientation is part of where, carried by every quantity; a sign convention is presentation. Open: how inner and outer orientation (Tonti's terms) are written in the tag |
| **R90** | Is carry the mirror? | Combine and separate stay on one side. Does carry, across a relationship, *always* cross from one body's source side to another's, as *bears on* does? If so, the force pairs and the mirror are one structure |
| **R91** | Time | Instants and spans of time as pieces. Does a load held over time, or a run of the engine, use the same complex? Ties to R11 (ordering without a global clock) |
| **R92** | The continuum | **Partly answered by K20:** for polynomial loads, adding up along the span in ℚ is exact, so wL²/8 and 5wL⁴/384EI are derived. Open: loads and shapes that are not polynomial (a sine load, a tapered member), where exact adding up has no finite answer. Are those derived to a stated bound, or witnessed only? |
| **R93** | Beyond linear | Across-the-mirror laws are often not straight lines (yielding, cracking, buckling). Does the bridge stay one pinned edition with witnesses, or does it need state? |
| **R94** | Tags and the gate | How does the evolution gate treat a tag change? Adding a tag to an untagged ℤ port is new truth. Changing "force per line" to "force" is a different cell. Ties to R27 (conservative extension) |
| **R95** | Sensitive problems | Buckling, slender members and diverging analyses are chaotic in the scientific sense (K17). How does JoInn show that a result is determined but sensitive: a witness of how far the answer moves when the input moves by the smallest step its frame can state? |
| **R96** | Kinds as bodies | K19 builds kinds into cells and bodies. What exactly does a kind-defining body hold: the pair, its bridge, its energy product? How does a cell say which pair it belongs to without that becoming a registry? Murky (AJ, 1 Oct); JoInn goes forward and lets testimony grow |
| **R97** | Witness libraries | Roark is the first witness (K20). How is a witness library entered (pinned edition, cases, conditions), how many must agree, and what happens when two libraries disagree with each other but not with JoInn? |

---

## 14. Prior Art

| Idea | Source | Lesson |
|---|---|---|
| Physical variables classified by the piece of space and time they live on, and by two families that mirror each other | Enzo Tonti, *The Mathematical Structure of Classical and Relativistic Physics* (2013), and his Cell Method | The core of this note: K1, K5, K6 |
| Quantities as numbers on cells of a mesh; exact topological operators; the metric and material only in one operator (the Hodge star) | Discrete exterior calculus: Hirani (2003); Desbrun, Kanso and Tong | K6, K8: the exact/testimony seam is a known split, not a JoInn invention |
| Whitney forms; electromagnetism with topology and metric separated | Alain Bossavit | The same split in a second physics |
| Kirchhoff's laws as the topology of a network | Roth, Branin, Kron | Balance and compatibility as connection-only laws, the oldest example |
| Dimensional analysis done with linear algebra | George W. Hart, *Multidimensional Analysis* (1995) | The exponent view, done carefully: the shadow in K4 |
| Oriented products; a moment as a plane, not an arrow | Grassmann; geometric algebra | K9, K10: why stacking has a sign |
| Deterministic laws that look random; sensitive dependence on starting conditions | Edward Lorenz (1963); chaos theory | K17: chaos is order not yet known, never chance |
| Units defined by counting and fixed constants | SI redefinition (2019): the second from a cesium count, the meter from c, the kilogram from h | K18–K19: units are counting plus bridges; the constants are a pinned edition |
| Every theory's two families multiply to energy | Tonti; energy-conjugate pairs in mechanics and thermodynamics | §8.2: a kind is which pair a quantity belongs to |
| Type-checked units | units libraries (Rust `uom`, F# units of measure) | What exponents can do, and the baseline the adversary in §11 says to fall back to |

---

## 15. Glossary

| Term | Definition |
|---|---|
| **Dimension** | Where a quantity lives: the piece of space and time it is attached to. Not its unit |
| **Arity** | How many ports a relation touches (R4b, the assay) |
| **Space** | Whether a picture is flat or solid (R4a, the Visual Host) |
| **Piece** | A point, line, surface or volume (or an instant or a span of time) in a complex |
| **k** | The size of a piece: point 0, line 1, surface 2, volume 3 |
| **Complex** | A model made of pieces and how they bound each other |
| **Placement side** | Quantities that say how things sit and move: displacement, strain, curvature |
| **Source side** | Quantities that say what pushes: load, reaction, stress, moment |
| **Mirror** | The pairing of the two sides: each quantity has its partner on the other side. Opposition, in Part I's word |
| **Exact law** | A law that uses connection only, such as balance or compatibility. Truth |
| **Bridge** | A law across the mirror, such as stress = E × strain. Testimony, held as a pinned edition |
| **Tag** | A quantity's where (with orientation), side and pair. Part of a port's frame |
| **Pair** | The two partners across one mirror (force and displacement; heat flow and temperature). Multiplied, they give energy. Defined by a body, never listed |
| **Witness library** | A library that states formulas JoInn derives itself (Roark). Compared, never relied on |
| **Unit** | A measuring stick, defined through pinned bridges. Converted exactly; not part of truth |
| **Stacking** | What multiplying does to pieces: a line times a line is a surface. The k's add |
| **Order-blind** | A result that doesn't depend on the order its members were combined in. The order still happened, and JoInn records it |
| **Order-signed** | A result whose order changes at most its sign, as the members' k's decide |
| **Order-bound** | A result that depends on order completely, so the order is part of its truth |
| **Chaos** | Behavior fully fixed by its laws and its starting state, which looks random only to someone who doesn't know that state exactly. Never "disorder" or "chance" |

---

## 16. Start Here (for the next planning chat)

**Where things stand, 1 Oct 2026.**

- **Phase 7 is closed.** Gate 7 passes 3/3, CI is green, and AJ ran the window (30 Sep). The Stop C review carries forward: `present` refusing unknown ports again, a rule that Cursor never moves AJ's mouse, and one shared helper for loading cells. R87 is new.
- **Decided 30 Sep:** the beam is Phase 7.1, in two parts; multiply is combine across dimensions; Visual Host II is Phase 7.2. (The 30 Sep choice that formulas come from a library is revised by K20.)
- **Decided 1 Oct:** K1 (a dimension is where a quantity lives), K7 (opposition is the mirror), K15–K17 (order-blind; order is always real and recorded; no randomness, and chaos only in the scientific sense), K18–K20 (nothing derived is a base; the base is counting, where and side; formulas derived, libraries witness). The rest of this note is Claude's reading, open to AJ's veto.

**The order to work in:**

1. AJ reviews K2–K6, K8–K13a and vetoes or accepts each. (R88 and R89 are answered.)
2. Write the Phase 7.1 plan (*the beam computes*), starting with the Stop C carry-forwards, then ℚ's turn, the tag, balance as the first exact law, derivation by adding up, multiply, a tiny bridge edition, and Roark as the first witness.

**Documents to read with this one:** Part V (*Cells, Bodies and Forces*), Part III §9 (the assay and why topology is derived), the *Research Backlog* (R4a, R4b, R83–R97), and the *Phase 7 Stop C Review*.

---

*JoInn Architecture and Theory, Part VI. Draft 0.1, 1 Oct 2026, from AJ's conversation with Claude after Phase 7 closed. AJ asked to define topological dimension before units were built, and agreed (1 Oct) that a dimension is where a quantity lives and that opposition is the mirror between placement and source. The same day he rejected "order-free" (nothing happens without an order; what looks like chaos is a lack of knowledge), which gave K15–K17. Draft 0.2 (same day): AJ held that nothing derived can be a base, so force and mass are not kinds; the base is counting, where and side, with kinds built into bodies where testimony grows; and formulas are derived, with Roark and other libraries as witnesses (K18–K20). Every other item is proposed, open to his veto. Theory only: nothing is replanned until AJ decides.*
