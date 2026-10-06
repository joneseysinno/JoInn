# JoInn Systems, Forces and Growth

**Part VII: a child's counting app, and how a system grows**

*Theory only · no implementation implied*

Author: AJ, with Claude · Draft 0.2 · October 6, 2026

> Status tags follow Part I: **DECIDED** (AJ said it), **PROPOSED** (Claude's reading, open to AJ's veto), **OPEN**. Decisions are numbered G1–G16 in §11. Research items R109–R114 are in §12.

> **Thesis.** A system is real because its forces are real. A force is a directed hyperedge, drawn as a lasso around what it acts on and cinched to what it produces. A system starts as a seed, and its DNA grows its bodies as inputs arrive. The basic operations of arithmetic are all ways of counting, so they grow inside one counting system. **A new system is warranted only when what is needed cannot be derived from what a system already does.**

---

## For AJ: this note in plain English

- **The calculator is a system, not a body.** A system holds forces, and it can hold several.
- **A force is drawn as a lasso.** It's a hyperedge with a direction: it surrounds the cells or bodies it acts on, and it cinches to a point at what it produces. It is not a wire between cells. Inside a body, cells still only touch.
- **A system starts as a seed.** Its DNA says how its bodies grow when inputs come in. Type another number and the body grows another cell, and the lasso stretches to hold it.
- **Growing is not evolving.** Typing more numbers is *growth*: same app, same identity. Teaching the app to add is *evolution*: the DNA gains a force, and everything it could do before still works.
- **All the basic math is counting.** Adding is counting on, subtracting is counting back, multiplying is counting by groups, dividing is counting how many groups fit, and fractions are counting pieces of a split unit. So they all grow inside one counting system.
- **Counting is its own witness.** Every answer can be checked by unrolling it into counting. The engine may calculate fast, but counting decides what is true.
- **A new system appears when something can't be derived.** Her UI and her save system aren't counting, so they're separate systems. Someday, measuring a smooth curve won't be counting either, and that will be a new system.
- **Her app has the shape of a cell, one size up:** an engine (counting), the ability to be seen (the UI), and storage (the saves).

The running example is a child's counting app (§1).

---

## 1. The Running Example: Her Counting App

A child is learning math, and she starts with counting. She builds an app that counts. It has three systems (**DECIDED**, G1):

```
UNIVERSE: her app
  ┌──────────────────┐     ┌──────────────────┐     ┌──────────────────┐
  │  SYSTEM counting │────▶│  SYSTEM ui       │     │  SYSTEM saves    │
  │  (the engine)    │──────────────────────────────▶│  (for her teacher)│
  └──────────────────┘     └──────────────────┘     └──────────────────┘
```

**Then she learns to add.** She doesn't make a new system. She adds to the one she has, because counting and adding are the same thing (**DECIDED**, G2). As she grows, the counting system gains more abilities: subtraction, multiplication, fractions. Eventually something comes along that counting can't do, and *then* a new system is warranted (**DECIDED**, G3; §7 gives the test).

This replaces Part III's first grader, whose add *cell* kept its hash while it learned fractions. Part V already moved `+` out of the cell and into combine (Part V §4.5). This note puts combine where it lives, which is in a system, and shows how the system grows.

---

## 2. What a System Is

### 2.1 A system holds forces

> **The calculator is a system. A system holds forces, and it may hold several.** (G4, **DECIDED**)

Part V decided that operations are forces from outside (C5), and on 30 Sep AJ framed where "outside" is: *the force comes from the system; adding forces to a body makes it the system; the body's reaction is part of the body, so bodies can be swapped under the same forces.* This note keeps that framing:

| Level | Holds | Example in her app |
|---|---|---|
| **Cell** | data and its metadata | one number |
| **Body** | cells in contact, plus the DNA that grows them and their response laws (C10) | the numbers she has typed |
| **System** | bodies, and the **forces** that act on them | counting: the numbers, plus combine |
| **Universe** | systems, and the links between them | her app |

### 2.2 A system is real because its forces are

Part I said systems are hyperedge structures, not true nesting. That is still true, and now it says what a system *is*: **the bodies its forces reach** (G5, **PROPOSED**). A system isn't a box drawn around bodies. It's the set of things its lassos hold.

**This puts pressure on an earlier decision.** In September, AJ decided that a body is in exactly one system *within a lens*, and that other groupings are other lenses. If systems are defined by their forces, a force can't be in one system under one lens and another system under a different lens. The force is real and lenses are only ways of looking. Proposal: **the force system is the one real grouping, and lenses group systems, never bodies directly** (R109, **OPEN**).

---

## 3. A Force Is a Lasso

### 3.1 What it is

> **A force is a directed hyperedge. It surrounds the cells or bodies it acts on and points to what it produces.** (G6, **DECIDED**)

```
  SYSTEM counting
  ╭──────────── combine ────────────╮
  │   [ 2 ]    [ 3 ]    [ 4 ]   ... ├───▶ [ 9 ]
  ╰─────────────────────────────────╯
     body: numbers, touching            response
```

- **Tails** are what the force acts on: the cells inside the lasso.
- **The head** is what it produces: the response, at the lasso's point.
- **It is not a wire.** Part V's rule stands: a body has no wires, and its cells only touch (C2). The lasso belongs to the system and acts on the body from outside. It doesn't connect cells to each other.

It also makes Part V's picture of integer cells "contracting into the sum" visible: **the lasso is the contraction.**

### 3.2 How it is drawn

The lasso is a closed outline hugging the cells it holds, tightening into an arrow at its head (**PROPOSED**, G7). Phase 7.3's *region* form (a link's lanes drawn wide and pale) is already close to this. The difference is that a lasso wraps *what it acts on*, where a region follows streets *between* bodies.

### 3.3 Links may be carry forces

Part V named two force pairs: combine/separate and carry/release (C19). A link that takes a value from one body to another *is* carrying. So **a link may be a lasso of kind carry**: one directed hyperedge, in one of four kinds, for everything outside a body (R110, **OPEN**).

If that holds, it does more for "kinds must stay few" (C18) than anything since C19. Phase 7.3's links and this note's lassos would be one thing.

Why it might not hold: today a link has no response law and a force does. A test before anything is built: write Phase 5's link checks (direction, marks, grants) as carry's laws, and see whether anything is left over.

---

## 4. The Seed and Growth

### 4.1 The seed

> **A system starts as a seed. Its DNA controls how its bodies grow when inputs are put in.** (G8, **DECIDED**)

This is the stem-cell idea from the research backlog: an app starts as one seed carrying the genome, and it differentiates as requirements appear.

**What size is the seed?** AJ said adding needs at least 2 cells. Claude's reading: **2 is the starting shape, not a law** (G9, **PROPOSED**). Combine has an identity: adding nothing gives 0, and adding one number gives that number. So a body of 0 or 1 cells is still true; it just isn't interesting yet. If 2 is not a law, the body can grow from 0 and shrink back to 0 and be true at every size. A first-grader still sees two boxes when she opens it, because the seed's DNA starts it there.

### 4.2 Growth

> **A body can grow to as many cells as it needs.** (G10, **DECIDED**)

When she types a number, the DNA adds a cell inside the lasso, and the lasso stretches to hold it.

**How growth stays true.** Every growth step is checked against the force's law. That's opposition at the moment of growth:

| Force | Where a new cell may grow | Why |
|---|---|---|
| combine (adding) | anywhere in the lasso | combine is order-blind (Part VI K15) |
| combine at a turn (subtracting) | on a declared side of the turn | the turn carries the order (R76) |
| combine across dimensions (multiplying) | anywhere, if its units stack | Part VI K9 |

A growth step that breaks the force's law is refused, the same way admission refuses a bad file.

### 4.3 Growth is a fourth kind of change

Part V separated edit, edition and evolution (C14). Her story shows a fourth, **growth** (G11, **PROPOSED**):

| Change | What moves | Her app |
|---|---|---|
| **Edit** | one design's data | she renames the app |
| **Edition** | a library's testimony | a new edition of a times table she links to |
| **Evolution** | a kind's abilities: the DNA gains something, and every old witness still holds | she teaches it to add |
| **Growth** | a body's size, by its DNA, from inputs; the DNA does not change | she types a fifth number |

**Identity follows the DNA, not the size** (G12, **PROPOSED**). The hash covers the genome and the forces. The cells that grew from inputs are *state*. Otherwise every typed number would make a "new" app, and her teacher couldn't tell that today's app is the one from yesterday.

Biology has the same split: **development** (one organism growing by its genome) and **evolution** (the genome itself changing).

---

## 5. Counting Is the Root

### 5.1 Every basic operation is a way of counting

Part VI already put counting at the base (K18–K19: the base is counting, where and side). Her story says what that means for arithmetic (G13, **DECIDED** in AJ's words, "all the basic math functions are just different ways of counting"):

| Operation | As counting | In Part V / VI terms |
|---|---|---|
| **Counting** | take the next one | the base |
| **Addition** | count on: 3 + 4 is four more after 3 | combine |
| **Subtraction** | count back | combine at a turn |
| **Negative numbers** | count past zero the other way | a turn |
| **Multiplication** | count by groups: 3 × 4 is four, three times | combine across dimensions |
| **Division** | count how many groups fit | multiply at a turn |
| **Fractions** | count pieces of a split unit: ¾ is three quarters | separate, then count |
| **Exponents** | count multiplications | combine across dimensions, repeated |

### 5.2 Counting is the witness

Since every operation unrolls into counting, **counting can check every answer**: 3 + 4 = 7 because 7 is four steps after 3 (G14, **PROPOSED**). This is the same idea as Part VI K20, where formulas are derived and Roark is a witness. Inside her counting system, counting itself is the witness.

**A caution.** Counting defines what is *true*, not how the engine must *calculate*. Multiplying large numbers one step at a time would take forever. The engine may use fast methods, but its answer must always agree with the counting it stands for, and the gate checks that agreement with witnesses.

---

## 6. Where the Answer Lives

Part V §6.2 said combine consumes its inputs: `{2, 3}` combined is `{5}`. In her app, the 2 and the 3 disappearing is wrong. She wants to see `2 + 3 = 5`, and her teacher wants to see what she typed.

Proposal: **the response is a cell at the lasso's point, and the inputs stay** (G15, **PROPOSED**). This is what Phase 7's contact calculator already does (`combine … as sum from cli_a@1, cli_b@1`). It also fits opposition: the inputs are the testimony and the response is the verdict, and you need both to check the verdict.

What happens to Part V's "consumed": it becomes a property of *carry*, not combine. Carrying something across moves it (R111, **OPEN**).

---

## 7. When a New System Is Warranted

> **A new force that can be derived from what a system already does belongs in that system. A new system is warranted when what is needed cannot be derived from it.** (G16, **PROPOSED**)

This is AJ's own rule from 1 Oct, *anything that can be derived should not be a base kind*, applied to systems.

| Her need | Derived from counting? | Where it goes |
|---|---|---|
| adding, subtracting, multiplying, dividing, fractions | yes | the counting system |
| showing her numbers | no: presenting isn't counting | the UI system |
| keeping her counts for her teacher | no: remembering isn't counting | the saves system |
| √2, the diagonal of a square | **no**: no count of any unit reaches it | a new system: measuring |

**The edge of counting is old.** Greek mathematics kept two kinds of quantity apart: *number*, which is counted, and *magnitude*, which is measured (Euclid, Books V and VII). They split over exactly this: the diagonal of a square can't be counted in pieces of its side. For JoInn, that is where her counting system ends and a measuring system begins (R112, **OPEN**).

---

## 8. Her App Has the Shape of a Cell

Part I gave a cell an engine, the ability to be seen, and storage. Her app has the same three, one size up (**PROPOSED**):

| A cell | Her app |
|---|---|
| engine | the counting system |
| ability to be seen | the UI system |
| storage | the saves system |
| validation (opposition) | counting as the witness (§5.2) |
| DNA | each system's seed |

That's the fractal: the same shape at every zoom, so a first-grader's app looks like a cell from far away and like systems close up. Whether this is a law or just a good habit is R113 (**OPEN**).

---

## 9. What Her Teacher Sees

The saves system can keep more than her counts. Because growth and evolution are different changes (§4.3), it can also keep **when her app evolved**: the day counting learned to add, the day it learned fractions. That isn't the app showing off; it's the app's lineage, and JoInn already records lineage. Her teacher would see her learning, not just her answers (**PROPOSED**; R114, **OPEN**).

---

## 10. What This Means for What Is Built

Nothing is replanned by this note. Phase 7.3 is accepted and stands.

| Built | Under Part VII |
|---|---|
| Phase 7's contact calculator (`.contact`, a `forces` block, `combine … as sum`) | The right idea in the wrong place: its force is written inside the body's file. It moves to a system, with the body keeping its DNA and response laws |
| Universes (Phase 5) | Hold only wired `.body` files today. They must hold systems made of contact bodies and forces |
| The grove (7.2) | Grown from wired bodies, so it never shows a force. Regrow it from counting systems |
| The 7.2 layout | Every body must fit 40 × 24, and 7.3's gutters depend on that grid. A growing body breaks both: the layout must give a body room to grow |
| 7.3's links | Stand. If R110 holds, they become carry lassos; their gutter routes stay |
| 7.3's drawing | Links are hard to see and never say what they connect (the 7.3 window check). The lasso and the level looks fix both |

**How to move** (**PROPOSED**):

1. **AJ reviews this note:** accepts or vetoes each PROPOSED item.
2. **R110 on paper:** try writing Phase 5's link rules as carry's laws.
3. **Phase 7.4, "her counting app":** universes hold systems, the counting seed, growth by DNA with each step checked, the lasso drawn, a layout that lets bodies grow, links that show their ends, and adding learned by evolution. The three fixes from the 7.3 review open it.
4. **Then Phase 8** (the reader and keyboard), unchanged.

---

## 11. Decisions

| # | Decision | Status |
|---|---|---|
| G1 | The running example is a child's counting app with three systems: counting, UI, saves | **DECIDED** |
| G2 | Adding addition adds to the counting system; it doesn't make a new one | **DECIDED** |
| G3 | New systems are warranted as things progress, when the existing ones can't do what is needed | **DECIDED** |
| G4 | The calculator is a system. A system holds forces, and may hold several | **DECIDED** |
| G5 | A system is the bodies its forces reach | **DECIDED** |
| G6 | A force is a directed hyperedge: it surrounds what it acts on and points to what it produces | **DECIDED** |
| G7 | A force is drawn as a lasso hugging its tails, cinched to an arrow at its head | **DECIDED** |
| G8 | A system starts as a seed; its DNA controls how its bodies grow when inputs are put in | **DECIDED** |
| G9 | Two cells is the adding seed's starting shape, not a law; every size, 0 included, is true | **DECIDED** |
| G10 | A body can grow to as many cells as it needs | **DECIDED** |
| G11 | Growth is a fourth change, beside edit, edition and evolution | **DECIDED** |
| G12 | Identity follows the DNA and forces; grown cells are state | **DECIDED** |
| G13 | The basic math operations are all ways of counting, so they stay in one system | **DECIDED** |
| G14 | Counting is the witness: every answer must agree with the counting it stands for | **DECIDED** |
| G15 | The response is a cell at the lasso's point, and the inputs stay | **DECIDED** |
| G16 | A new system is warranted exactly when what is needed can't be derived from an existing one | **DECIDED** |

---

## 12. Open Questions

| ID | Topic | Question | Status |
|---|---|---|---|
| **R109** | Systems and lenses | If a system is the bodies its forces reach, what is a lens's system? Proposal: the force system is the one real grouping, and lenses group systems, never bodies | open |
| **R110** | Links as carry | Is a link a lasso of kind carry? Test: write Phase 5's link checks as carry's laws and see what is left over | open, first |
| **R111** | What is consumed | If combine keeps its inputs (G15), is "consumed" a property of carry instead? | open |
| **R112** | Counting and measuring | Where exactly does counting end? Exact rationals are counting; √2 is not. Is "measuring" one new system, or the edge Part VI's continuum (R92) already names? | open |
| **R113** | The app as a cell | Is "an app has the shape of a cell" a law the gate checks, or a habit? | open |
| **R114** | Lineage as learning | Can the saves system show an app's evolution as a learning record, and who may see it? | open |

---

## 13. Prior Art

| Idea | Source | Lesson |
|---|---|---|
| Every natural number is "the next one" after zero; arithmetic built from it | Peano's axioms (1889) | §5: counting is the root |
| A number *is* counting: 3 means "do something three times" | Church numerals (lambda calculus) | Operations as ways of counting are an old, exact idea |
| Number (counted) vs magnitude (measured); the diagonal of the square | Euclid, *Elements* V, VII, X | §7: the edge of the counting system |
| "God made the integers; all else is the work of man" | Kronecker | Everything else derived from counting |
| Plants grow by rewriting rules applied to their parts | Lindenmayer systems (L-systems) | §4: DNA as growth rules, checked step by step |
| Development vs evolution | Developmental biology | §4.3: growth is not evolution |
| Objects as biological cells: each a small computer, talking only by messages | Alan Kay, Smalltalk | §8: the app shaped like a cell |
| Hyperedges with tails and heads | Directed hypergraphs (Gallo et al., 1993) | §3: a force as a directed hyperedge |

---

## 14. Glossary

| Term | Definition |
|---|---|
| **System** | Bodies and the forces that act on them. Real because its forces are |
| **Lasso** | How a force is drawn: an outline around what it acts on, cinched to an arrow at what it produces |
| **Tail / head** | What a force acts on / what it produces |
| **Seed** | A system's starting shape: its DNA, its forces, and the bodies it starts with |
| **Growth** | A body getting bigger by its DNA, from inputs. The DNA doesn't change |
| **Evolution** | The DNA gaining an ability, with every old witness still holding |
| **Counting** | Taking the next one. The root every basic operation unrolls into |
| **Witness (counting)** | Checking an answer by unrolling it into counting |

---

## 15. Start Here (for the next planning chat)

**Where things stand, 6 Oct 2026.**

- **Phase 7.3 is accepted** (`claude/JoInn Phase 7.3 Review.md`). AJ ran the window: links are hard to see and never say what they connect; bodies, systems and cells are hard to tell apart; the grove's bodies still use wires.
- **Why:** universes hold only wired `.body` files, so Phase 7's forces never reached the grove.
- **Decided today:** G1–G4, G6, G8, G10, G13. Everything else here is proposed.

**The order to work in:**

1. AJ accepts or vetoes G5, G7, G9, G11, G12, G14, G15, G16.
2. R110 on paper (links as carry).
3. Write the Phase 7.4 plan, *her counting app*, opening with the three fixes from the 7.3 review.

**Documents to read with this one:** Part V (*Cells, Bodies and Forces*), Part VI (*Dimension*: K15, K18–K20), the *Phase 7.3 Review*, and the *Research Backlog* (R76, R92, R105–R108).

---

*JoInn Architecture and Theory, Part VII. Draft 0.1, 6 Oct 2026, from AJ's conversation with Claude after Phase 7.3's window check. AJ said: the calculator is a system holding several forces; forces are directed hyperedges that look like a lasso around a body or bodies; the system is a seed whose DNA grows the body as inputs arrive; and a child's counting app keeps every basic math function in one counting system, because they are all ways of counting, with new systems coming only as things progress. Decided items are AJ's; proposed items are open to his veto. Theory only: nothing is replanned until AJ decides.*
