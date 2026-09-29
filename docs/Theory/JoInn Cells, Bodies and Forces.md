# JoInn Cells, Bodies and Forces

**Part V: what a cell is, how a body talks, and where operations come from**

*Theory only · no implementation implied*

Author: AJ, with Claude · Draft 0.2 · September 29, 2026

> Status tags follow Part I: **DECIDED** (AJ said it), **PROPOSED** (Claude's reading, open to AJ's veto), **OPEN**. Research items R76–R81 are in §12. Decisions are numbered C1–C23 in §11.

> **Thesis.** Phases 2 to 6 built a body as cells joined by wires, made `sum` a cell, and called the body's edge its membrane. Those were the wrong words for the right machinery. **A cell is data that knows what it is. A body is cells in contact. An operation is a force from outside. Wires belong to systems. And opposition is what keeps the forces few.**

---

## For AJ: this note in plain English

- **The membrane belongs to a cell, and only a cell.** A body's outline is its *surface*, which is derived, never declared.
- **A body is a group of cells touching each other.** It has no wires inside. Cells talk directly through their membranes. A body can hold many kinds of cells, and its cells take **roles**: some protect, some respond. A body never turns into a cell; it gets more complex.
- **A cell is data plus its metadata.** Kinds must stay few, so they are built from a small base (frames and dimensions) and never kept as a growing list.
- **Operations are forces from outside.** A force is not DNA. It is real and determinant, like physics. The rule for how something responds lives in the thing being forced.
- **There are two pairs of forces:** **combine / separate** and **carry / release**. Transform is only these two in sequence. Distinguishing is not a force at all: it is **opposition itself**, how truth sees whether things balance.
- **Opposition is what simplifies the Floor.** Asking "what is its opposite?" folded four families into two pairs.
- **Wires belong to systems.** They connect bodies (a beam *bears on* a column), and they connect a cell to the library its data comes from.
- **A link is pinned, live, or pinned with a watch.** Every link records the edition it actually used.

The running example is a steel beam (§9).

---

## 1. The Wrong Turn

Phase 2 wrote the calculator as three cells with two wires:

```
cli_a@1 -> sum@0
cli_b@1 -> sum@1
```

That made `sum` a cell, which is a verb dressed as a noun. It put wires inside a body, which is the system's way of talking used one level too low. And the code named a body's derived edge (`∂(body)`, every port no wire consumes) its `membrane`, and Phase 6 drew it as one. Part I had it right all along: *"Membrane: the boundary of a cell; the only place a cell meets anything outside itself"* (Part I §3, DECIDED), and *"Cells communicate directly only within their own body, across their membranes"* (Part I §6.1, DECIDED). The build drifted from the theory, not the other way around.

None of the machinery is wasted. Frames, the gate, the live engine, hosts, the tables, delta and regrow, and the two pickers all stand. What changes is **where wires live**, **what a cell is**, and **what an operation is** (§10).

---

## 2. Three Levels, Three Ways of Talking

| Level | Holds | How its members talk | Status |
|---|---|---|---|
| **Cell** | data and metadata | through its **membrane** | **DECIDED** (C1) |
| **Body** | cells in contact | **contact**: direct, through membranes, no addresses | **DECIDED** (C2) |
| **System** | bodies | **wires** (hyperedges): addressed and explicit | **DECIDED** (C3) |

**Words.** The *membrane* is a cell's boundary. The derived outline of a body, every cell port nothing inside uses up, is its **surface** (C8, DECIDED).

**Supersedes.** Part III §5 says "cell-in-body and body-bus differ in scale, not in kind." They differ in kind: contact is not a small wire.

---

## 3. The Cell: Data That Knows What It Is

A cell is a piece of data with its metadata (C4, **DECIDED**). Cells can be simple (one integer) or complex (a whole steel shape with its section properties).

| Metadata | Example | Why it matters |
|---|---|---|
| **Frame** | ℤ, ℚ, text | Which laws the data obeys. A frame is a named set of laws (Part III) |
| **Dimension** | length, force per area | What can combine with what |
| **Turn** | the direction a value is read in | Subtraction is sum read at a turn; no second operation |
| **Source** | a link to a library (§7) | Where the data came from, and which edition |
| **Laws** | how this data responds to each force (§4) | The truth the gate judges |

### 3.1 Kinds without a growing list

A list of kinds that grows forever is a registry, and registries rot (C18, **DECIDED**). So kinds are **built, never listed**, the way nature does it:

- **Physics:** 7 base dimensions, and every quantity is built from them.
- **Chemistry:** about 100 elements, and molecules are never catalogued.
- **Biology:** one genome and about 200 cell types, all expressed rather than listed.

The mechanism (C9, **PROPOSED**):

1. **A kind is a frame plus a dimension.** Frames come from `build`'s constructors; dimensions come from a few base dimensions. Both are generative.
2. **Meaning is local.** E and Fy have the same dimension, and no global kind tries to tell them apart. They are **different named parts of one complex cell**: the material cell is built from parts `E`, `Fy`, `density`, and the beam's deflection law reaches for "the material's `E`." The name lives in the material's own DNA, not in a registry.

**The membrane acts as a receptor** (**PROPOSED**). A force reaches only the cells whose metadata fits it. A length and a force never combine.

---

## 4. Forces

### 4.1 What a force is

An operation is not a thing in a body. It is a **force applied from outside**, and cells **react** (C5, **DECIDED**).

> **A force is not DNA. It is not a seed or a blueprint. It is determinant and real.** (C16, **DECIDED**)

> **A force is a stimulus. The response law lives in the thing being forced.** (C10, **DECIDED**)

A load never says how steel deforms; the material law lives in the member. Layered, which is the fractal:

| Level | Owns | Beam example |
|---|---|---|
| Cell | its data's law | A992's E and Fy, and how steel behaves |
| Body | the body's behavior | how a pinned–pinned member responds to a uniform load (the Roark crate) |
| Force | only what it is and where it acts | "w = 1.2 kip/ft acts on B1" |

### 4.2 Two pairs of forces

Asking each candidate family for its opposite folded four families into two pairs (C19, **DECIDED**):

| Force | Opposite | Does | Example |
|---|---|---|---|
| **Combine** | **Separate** | many into one / one into its parts | sum; a resultant; parts of a date |
| **Carry** | **Release** | move something across a relationship / let it go where it is | a beam's reaction into a column / a load left in place |

- **Separate is not ÷.** Arithmetic division is combine read at a turn (multiplication backwards), the way subtraction is sum at a turn. The force is named *separate* so a first grader never confuses it with ÷.
- **Release destroys nothing.** Released means not carried: the thing stays where it is. A force that discards information must say so, because opposition requires it.

### 4.3 What is not a force

**Transform is not a family** (C20, **DECIDED**). Every transform is either:

- **separate, then combine**: 18 in → 1 ft 6 in; or
- **a turn**: a direction of reading, where nothing changes but which face you read.

**Distinguishing is not a force. It is opposition itself** (C21, **DECIDED**). The Floor's `eq` / `choose` splits in two:

- **`choose` is carry and release**: carry one outcome, release the other.
- **`eq` does not dissolve.** The Floor proved equality cannot be built from anything else. And it has no opposite act, because it acts on nothing: it moves nothing, combines nothing, changes nothing. It is how truth sees whether two things balance. Loads act on a beam; the ratio check judges it, and the check is not a load. **That is why it has no opposite: it is what gives "opposite" its meaning.** It is also where the gate lives.

### 4.4 Opposition keeps the forces few

> **We need opposition.** (C23, **DECIDED**)

The rule "every force has an equal and opposite force" did the simplifying here. A candidate force with no opposite is either built from forces that have one (transform), or it is not a force (distinction). This is the Floor's own rule, *irreducible and opposed*, applied to forces.

### 4.5 The first grader, retold

In Part III §11, "the add cell" kept its hash while it learned fractions. Under this note there is no add cell. His `+` is **combine**, and **his numbers evolve**: integers learn to be fractions, and each kind of number knows how it responds to combine. The add cell's coding region (laws {a + 0 = a, a + b = b + a, …}, witnesses {(2, 3) → 5}) moves to where it belongs: **the laws of the frame ℤ under combine**. The path of truth survives, one level lower.

---

## 5. The Floor, Re-read

**The Floor is what everything in JoInn is finally written in.** It is sealed, it is not judged by the gate (the gate is written in it), and its rule is that a pair enters only if it is **irreducible** (nothing else builds it) and **opposed** (its opposite is in the Floor too). Today it has eight pairs (see `Findings/the-floor.md`).

Part V proposes that the Floor sorts into two force pairs, one distinction, and one boundary (C22, **PROPOSED**):

| What it is | Floor pairs it would absorb |
|---|---|
| **Combine / separate** (force) | `build` / `case`, `pair` / `split`, `join` (gather) |
| **Carry / release** (force) | `fan`, `grant` / `revoke`, `choose` (carry one, release the other) |
| **Distinction** (opposition itself; the gate's eye) | `eq`; `hash` (identical hash means identical thing) |
| **Membrane** (the cell's boundary) | `bound` / `fill` |
| *moves to systems* | `bind` / `unbind` becomes a relationship's attach and detach (C3) |

**How this is tested, by the Floor's own rule (R78):** try to rebuild each old pair from the new ones. A pair that can be rebuilt leaves the Floor and says why, as `zero`, `succ` and `pred` did on 18 Sep. A pair that cannot be rebuilt stays. No Floor amendment is made until that reduction has been tried and recorded.

**What is still missing:** contact (C2) is not in the Floor yet. Either it is a new pair, or it is combine/separate read at a body's scale.

---

## 6. The Body: Cells in Contact

Cells in a body talk directly (C2, **DECIDED**). No cell names another.

### 6.1 Many kinds, many roles

A body can hold many kinds of cells (C11 withdrawn; C17, **DECIDED**). What a body does with them is **differentiation**: its cells take **roles**, and some protect, some respond, some carry, some store. This is the seed idea already in the backlog: one stem cell carrying the genome, differentiating as requirements appear. Roles must stay few, like kinds (R81).

**A body is wrapped by its own protective cells, not by a membrane** (C17). Skin is not a membrane; it is a layer of cells whose role is protection. An open body has no protectors, and its responders are reachable. A wrapped body has a layer of protectors that decides which contacts reach the inside. **A body never becomes a cell.** It gets more complex.

### 6.2 How contact finds the right cell

Without addresses, a force still has to reach the right cells. Three rules (**PROPOSED**):

1. **Metadata and part names do the addressing** (§3.1). "The material's `E`" is unambiguous in a beam, even though the beam holds many kinds of cells.
2. **Combine inside a body is order-free.** Many cells into one must not depend on who arrived first: it must be commutative and associative, and the gate checks that with witnesses. The inputs are consumed, and only the result remains: `{2, 3}` combined is `{5}`. Prior art: Gamma, and Păun's membrane computing (§13).
3. **A force completes when everyone is present.** A body knows its members, so a force finishes once every cell it reaches has its value. That is deterministic, because combine ignores order.

**The composite beam.** Steel and concrete each have an E. Modelled as engineers do, it is two bodies, the steel beam and the slab, with a *composite with* relationship between them (the shear studs). The one-body-per-material rule is withdrawn (C11), but this case still reads most naturally as two bodies.

---

## 7. Connections and Links

### 7.1 Three kinds of connection

| Connection | Level | Means | Direction | Status |
|---|---|---|---|---|
| **Contact** | cell ↔ cell, inside a body | the cells are together | none | **DECIDED** |
| **Source link** | cell → library | "my data comes from there": W12x26 from the AISC shapes, A992 from the materials library | one way | **DECIDED** (C6) |
| **Relationship** | body ↔ body | *bears on*, *framed into*, *composite with* | both ways; it carries something | **DECIDED** (C6) |

Relationships belong to **the body**, not to any one cell. So there are two outside levels to link to (C6).

- **A relationship is where opposition is physical.** *Bears on* carries the beam's end reaction down and the equal and opposite force up into the column (**PROPOSED**).
- **Relationships are hyperedges.** A shear tab joins a beam, a column and a plate: one relationship, three bodies.

### 7.2 Pinned, live, and watched

Both pinned and live are required; different applications need different ones (C7, **DECIDED**).

> Engineering demands that structures be built as the code defines, but the code changes as we discover the power of nature. A pinned link lets the testimony grow and be refined without moving what was signed.

> **Every link resolves to a pinned edition at the moment it is used. Pinned and live only differ in who may move the pin.** (C13, **DECIDED**)

| Mode | Who moves the pin | Records | Example |
|---|---|---|---|
| **Pinned** | the engineer, by an explicit edit | the edition it was signed with | A992 per AISC 360-16 |
| **Live** | the library, at every use | **the edition it actually used**, every time | the architect's grid; a steel price; sensor data |
| **Pinned, with a watch** | the engineer; the library only reports | the pinned edition, plus a shadow result against the newest | "Under 360-22 this ratio would move from 0.87 to 1.04" |

A live result is never "whatever the library said"; it is "this answer, from edition X, on date Y." The watch mode is retrofit screening for free. The choice is **per link**: one beam can hold a pinned material and a live grid.

---

## 8. Change: Edits, Editions, Evolution

Three different changes, never sharing a word (C14, **DECIDED**):

| Change | What moves | JoInn form | Beam example |
|---|---|---|---|
| **Edit** | one design's data | a new revision with `lineage` | Rev A → Rev B: A992 becomes A572 Gr 65 |
| **Edition** | a library's testimony | a new sealed edition with lineage; the old one stays true for what it witnessed | AISC 360-16 → 360-22 |
| **Evolution** | a kind's abilities | a new allele or law that keeps every old witness | "beam" learns lateral-torsional buckling; every old beam checks the same |

An edition is not evolution. A code edition may be stricter than the last, and the old one was not false: it was true for everything it had witnessed, and it stays sealed. **Libraries grow by adding editions, never by overwriting.** This is grandfathering, and the repo already has a `grandfather.txt`.

---

## 9. The Running Example: A Steel Beam

```
SYSTEM: floor framing
  [Column C1] ═══ bears on ═══ [Beam B1] ═══ bears on ═══ [Column C2]
                                   ║
                             load case: w = 1.2 kip/ft          ← a force, from outside

BODY B1 (beam): cells in contact, no wires between them
  ┌──────────────────────────────────────────────────────┐
  │  length    24 ft                                     │
  │  supports  pinned–pinned                             │
  │  profile   W12x26 ─── pinned ──▶ AISC shapes library   (d, Ix, Sx, Zx …)
  │  material  A992   ─── pinned ──▶ materials library     (parts: E 29,000 ksi, Fy 50 ksi)
  │  grid      C–D    ─── live ────▶ architect's model     (sets the length)
  │  ─ response: appears when a force acts ─             │
  │  M = wL²/8    δ = 5wL⁴/384EI    φMn    ratio          │
  └──────────────────────────────────────────────────────┘
```

**Rev A → Rev B.** The engineer swaps A992 for A572 Gr 65:

1. The material cell's source link now points to another entry. Its part `Fy` goes 50 → 65; its part `E` stays 29,000.
2. Inside B1, only what reaches for `Fy` reacts: φMn rises and the ratio falls. δ does not move, because `E` did not change.
3. The *bears on* reactions into C1 and C2 do not change. **The ripple stops at the beam's edge.**
4. Had the ratio gone above 1.0, distinction would refuse it, and the body would draw red like the refused cell in Phase 6's window.

Step 3 is Phase 6's delta rule at design scale: only what changed is rewritten, and it stops where nothing changed.

---

## 10. What This Means for What Is Built

Nothing is replanned by this note. Phase 6 is closed and stands.

| Built | Under Part V |
|---|---|
| Frames, gate, live engine, hosts, tables, both pickers, delta and regrow | **Stand.** Tables become rows of cells and forces instead of cells and wires |
| `.body` with a `wires` section | A system in miniature: one-cell bodies wired together. Kept as the legacy form; its 43 hashes untouched |
| `sum` as a cell | Becomes the combine force; its laws move to the frame ℤ (§4.5) |
| `joinn-link::membrane` | Renamed `surface` (C8) |
| Phase 5 links | Already the system's wires; they keep their meaning |
| Phase 6 layout (columns from wires) | A body needs a contact layout; a system layout keeps wires |
| The Floor | Unchanged until R78's reductions are tried and recorded (§5) |

**How to move** (**PROPOSED**):

1. **R78 first, on paper.** Try to rebuild each Floor pair from combine/separate, carry/release, distinction and membrane. Record every reduction that succeeds or fails.
2. **Spike S7:** rebuild the calculator as a contact body with the combine force. It must produce the same transcript, including the refusal of `"two"`, and pass Phase 6's picture rules (every pixel one owner; regrow equals deltas). S1–S6 are already used.
3. **A new file kind** for contact bodies, so no existing hash moves (the way `.body` arrived beside `.cell`). R60 applies: its mutation catalogue ships first.
4. **Then the beam**, the first body that is not a calculator: it tests part names, source links, a relationship, and roles at once.

---

## 11. Decisions

| # | Decision | Status |
|---|---|---|
| C1 | The membrane is a cell's boundary, and only a cell has one | **DECIDED** |
| C2 | A body is cells in contact. It has no wires; cells talk directly through their membranes | **DECIDED** |
| C3 | Wires belong to systems | **DECIDED** |
| C4 | A cell is data plus metadata. Cells and bodies may be complex | **DECIDED** |
| C5 | Operations are not cells. They are forces from outside, and cells react | **DECIDED** |
| C6 | Two outside link levels: a cell's source link to a library, and a body's relationships to other bodies | **DECIDED** |
| C7 | Links are pinned or live, by application | **DECIDED** |
| C8 | A body's derived outline is its *surface* | **DECIDED** |
| C9 | A kind is a frame plus a dimension; meaning comes from local part names inside complex cells | **PROPOSED** (revised from "quantity kind") |
| C10 | A force is a stimulus; the response law lives in the thing forced, at each level | **DECIDED** |
| C11 | ~~One of each kind per body~~ | **WITHDRAWN** (AJ: a body can hold many kinds) |
| C12 | ~~Contraction as the only body force~~ | **SUPERSEDED** by C19 |
| C13 | Every link resolves to a pinned edition at use; live links record the edition used; a watch mode shadows the newest | **DECIDED** |
| C14 | Edit, edition and evolution are three different changes | **DECIDED** |
| C15 | ~~The Floor is the set of fundamental forces~~ | **RESOLVED** into C19–C22 |
| C16 | A force is not DNA. It is not a seed or a blueprint; it is determinant and real | **DECIDED** |
| C17 | A body never becomes a cell. Its cells differentiate into roles; a wrapped body is wrapped by protective cells | **DECIDED** |
| C18 | Kinds must stay few: built from a small base, never a growing list | **DECIDED** |
| C19 | The forces are two opposed pairs: **combine / separate** and **carry / release** | **DECIDED** |
| C20 | Transform is not a force family: it is separate then combine, or a turn | **DECIDED** |
| C21 | Distinguishing is not a force. It is opposition itself: how truth sees whether things balance, and where the gate lives | **DECIDED** |
| C22 | The Floor sorts into the two force pairs, distinction, and the membrane; tested by attempted reduction before any amendment | **PROPOSED** |
| C23 | We need opposition: every force has its opposite, and asking for it is what keeps the forces few | **DECIDED** |

---

## 12. Open Questions

| ID | Topic | Question | Status |
|---|---|---|---|
| **R76** | Order inside a body | Combine is order-free. Where do order-dependent things live? Proposal: arithmetic ÷ and − are combine at a turn (the turn carries the order); presentation ("{0} + {1} = {2}") belongs to the host; anything else order-dependent is a system matter. AJ said R76 is decided once C12 and C15 are settled, and they now are | **ready to decide** |
| **R77** | Where a force's truth lives | Settled by C10 and C16 | **settled** |
| **R78** | The Floor re-read | Try to rebuild each Floor pair from C22's four. Where does contact fit: a new pair, or combine/separate at a body's scale? | **open, first** |
| **R79** | Link policy | Where a link's mode is written (coding region, since it changes behavior), how an edition is named, and whether a body with a live link can be sealed | open |
| **R80** | Wrapping | Settled by C17 | **settled** |
| **R81** | Roles | Protect, respond, carry, store, …: is the set of roles a short fixed list, or generative like kinds? | open |

Also reframed: **R74** (one address, one port) may dissolve if cells in a body are no longer addressed by position. **R75** (membrane ports) becomes: where does a *system* wire meet a body, on a protective cell or on the surface?

---

## 13. Prior Art

| Idea | Source | Lesson |
|---|---|---|
| Cells with membranes, multisets inside, rules reacting across membranes | Păun, membrane computing (P systems) | The closest formal model to C1–C5. Part I already lists it |
| Data in a pool, reaction rules outside it: `x, y → x + y` | Banâtre and Le Métayer, Gamma | Combine as a reaction that consumes its inputs |
| Entities hold data; systems act on every entity with the right data | Entity–component–system (Bevy, in Rust) | Forces as queries over cells by kind |
| Loads applied to a structure; material laws live in the members | Structural analysis, FEA | C10: a force is a stimulus; the response is the member's |
| 7 base dimensions, every quantity derived | SI, dimensional analysis | C9, C18: kinds are built, not listed |
| Grandfathering; code editions | Building codes (AISC, ASCE 7) | C13, C14 |
| Lockfiles | Cargo, npm | A pin by hash; an upgrade as an explicit edit |

---

## 14. Glossary

| Term | Definition |
|---|---|
| **Cell** | Data with its metadata. The only thing with a membrane |
| **Membrane** | A cell's boundary: the only place it meets anything else |
| **Body** | Cells in contact, of many kinds and roles. No wires inside |
| **Surface** | A body's derived outline: every cell port nothing inside uses up |
| **Role** | The job a cell takes in its body: protect, respond, carry, store, … |
| **Force** | An operation applied from outside. Real, not DNA. Cells react to it |
| **Combine / separate** | The force pair that makes many into one, and one into its parts |
| **Carry / release** | The force pair that moves something across a relationship, and lets it be |
| **Distinction** | Opposition itself: how truth sees whether two things balance. Not a force |
| **Turn** | A direction of reading. Subtraction and division are combine at a turn |
| **Kind** | A frame plus a dimension. Built from a small base, never listed |
| **Part name** | A local name inside a complex cell (`E`, `Fy`) that forces reach for |
| **Source link** | A cell's link to the library its data comes from |
| **Relationship** | A link between bodies: a hyperedge that carries something |
| **Pinned / live / watch** | Who may move a link's edition: the engineer, the library, or the engineer with a shadow check |
| **Edit / edition / evolution** | A design's revision / a library's new testimony / a kind's new ability |

---

## 15. Start Here (for the next planning chat)

**Where things stand, 29 Sep 2026.**

- **Phase 6 is closed.** Gate 6 passes 3/3, and AJ ran the window. The Stop C review lists three items carried forward: make the shell's scratch regrow test permanent; hook wgpu's device-lost callback; decide what Enter on an empty box does.
- **Phase 7 is not planned.** The roadmap's Phase 7 (charts, zoom, bands, links) assumed wired bodies. This note changes what a body is, so Phase 7 is replanned after Part V settles.
- **Decided today:** C1–C8, C10, C13, C14, C16–C21, C23.

**The order to work in:**

1. **Decide R76** (order inside a body). The proposal in §12 is ready.
2. **R78 on paper:** attempt the Floor reductions (§5), and place contact.
3. **R81:** roles, fixed list or generative.
4. **Plan spike S7** (the calculator as a contact body with combine), then the new body file kind, then the beam.
5. **Then replan Phase 7** around bodies, forces and systems.

**Documents to read with this one:** Part I (*Architecture and Theory*: the laws and the original membrane decision), Part III (*Primitives and DNA*: the hash boundary, alleles, Turn), `Findings/the-floor.md`, the *Research Backlog* (R76–R81), and the *Phase 6 Stop C Review*.

## 16. After the decisions of 29 Sep (Draft 0.3 notes)

- **R76 decided** as proposed in §12.
- **R78 attempted, no amendment.** See `Findings/the-floor.md` §5. `eq` / `choose`
  stay a pair (AJ). Contact needs no new pair: it is cells filling one `bound`
  with no `bind`.
- **R81 decided.** Roles are (faces out | in) × (holds | reacts): protect, carry,
  store, respond. Derived from the body, never written.
- **§6.2 rule 2 revised (Claude's, open to AJ's veto).** Combine keeps its
  inputs, and the response appears beside them. Sum forgets its parts, so its
  separate can only be a turn given one part back, and destroying the inputs
  would leave combine without a working opposite (R82).
- **Every force must have its opposite on file.** Combine on ℤ is admitted with
  its separate, `sum_turn`. ℚ cannot combine in a contact body until it has a
  turn.
- **Phase 7 builds this for the calculator** (`Plans/JoInn Phase 7 Implementation
  Plan.md`). Numbering: this is Phase 7; the old Phase 7 becomes Phase 7.1.

---

*JoInn Architecture and Theory, Part V. Draft 0.1 and Draft 0.2 both written 29 Sep 2026 from AJ's conversation with Claude after Phase 6 closed. 0.2 consolidates AJ's review: the forces are combine/separate and carry/release, transform is not a family, distinguishing is opposition itself, bodies differentiate into roles, and kinds are built from a small base. Decided items are AJ's; proposed items are open to his veto. Theory only: nothing is replanned until AJ decides.*
