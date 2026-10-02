# Glossary (plain English)

Short definitions for words you’ll see in JoInn docs. Deeper, precise meanings live in Theory; this page is for clarity.

| Term | Plain meaning |
|------|----------------|
| **Allele** | One candidate way to *implement* a cell’s plan. The plan’s identity is the laws; the allele is a payload that must pass those laws. |
| **Assay** | A measurement of a body or universe's shape without running it: the loops a value can travel, and which of them something closes. It reports; it never refuses on its own. |
| **Blueprint** | A shareable cell design others can reuse — meant to be seen, not only read. |
| **Body** | A cluster of cells that belong together, with one shared genome. The main “container” for a small machine or app piece. |
| **Bootstrap** | One of the three big build stages (true-but-unseen → seen-and-touched → self-editing). |
| **Bridge** | A law that crosses from the placement side to the source side and back, such as moment to curvature through E·I. It cannot be derived, so it is testimony, read from a pinned edition. Balance and moment need none; deflection needs one. |
| **Bridge edition** | The small pinned file a bridge is read from, naming its source: the beam spike's is `bridges.edition` (AISC Manual 16th ed.: A992 E 29000 ksi, W12x26 Ix 204 in⁴). Every read of it is counted. |
| **Camera** | How a layout fits a window: an integer scale, an origin, and a width and height. Phase 6 does not zoom or pan. Changing the camera writes the tick and no table row. |
| **Cell** | The smallest visual building block. Something you can show and still use. |
| **Chaos** | Only the scientific sense: behavior fully fixed by its laws and its starting state, which looks random only to someone who doesn't know that state exactly. Never "disorder" or "chance". JoInn has no randomness. |
| **Coding region** | The part of DNA that is hashed and gated: frame, contract, laws, witnesses — the cell’s true identity. |
| **Combine** | The one force Phase 7 writes: put the values of its members together with the frame's answer (on integers, the sum). Its answer must not depend on member order. |
| **Contact body** | A body whose cells touch, with no wires inside; its operations are forces. Written as a `.contact` file, which cannot express a wire. |
| **Corpus** | The library of golden examples (and deliberate fakes) used as testimony and regression checks. |
| **Declaration** | A sentence in a body's or universe's hashed part that the assay must satisfy. Today the only one is `assert H₁ = 0` (“no loop is open”); if it fails, the body or universe is refused at admission. |
| **Delta** | The table rows one run changed. The picture updates by writing those rows. |
| **Crate** | A Rust library or program package in the repo (for example `joinn-gate`). |
| **DNA** | The building plan for a cell: what it is and what laws it obeys. |
| **Edge pixel** | A pixel within 1/16 of a pixel of a shape's boundary. It is counted and never judged: the two pickers need not agree there. |
| **Elevation plane** | The flat picture a beam lives in: x along the span, y up. A moment lives on the plane itself (x ∧ y), which is why a beam is not only a line. |
| **Filling** | What closes a loop in the assay: a frame (the value keeps its kind all the way round), or one law that promises to undo a conversion, named by body, instance and law. Nothing else closes a loop. |
| **Floor** | The sealed set of basic operations JoInn is written in. Admitted only if irreducible and opposed. Never “fixed” by renaming its size. |
| **Force** | An operation applied to a body's cells from outside, such as combine. It names its members (cell ports) and a response. A force owns no pixel. |
| **Force register** | The short table of which forces a frame answers, and with which cell: combine on integers answers with the sum cell. A row is admitted only if the answer is order-blind and its opposite (a turn of the response) is on file. |
| **Frame** | The context in which truth is judged (example: integers, or a text frame). |
| **Gate** | The checker that admits good growth and refuses bad growth. |
| **Genome** | The full set of DNA shared by the cells of one body. |
| **Island** | In the assay, one connected piece of a universe once the outside world is removed. |
| **Host** | Whatever presents the universe to the outside (terminal, test harness, desktop window). A host is proved by there being more than one. |
| **ID target** | The second picture: one integer per pixel naming its owner. A click reads one texel of it. |
| **Latent** | A response that holds no value yet. It is drawn dim, and takes its normal color once a value arrives. |
| **Layout unit** | The integer length the tables use for positions. Pixels are what the camera makes of them. |
| **Live engine** | The runner that executes a body now, with an explicit step budget. |
| **Lower** | The engine's own derivation of the deliveries a contact body needs to run. Its only inputs are the contact, its cells and the force register. It is never written to a file, described, or drawn. |
| **Membrane** | The boundary of a cell — the only place it meets anything outside itself. A body has no membrane; its outline is its *surface*. |
| **Order-blind** | A result that doesn't depend on the order its members were combined in, such as a sum. The order still happened, and JoInn records it. (Earlier documents said *order-free*.) |
| **Order-bound** | A result that depends on order completely, so the order is part of its truth. |
| **Order-signed** | A result whose order changes at most its sign: lever arm ∧ force is a moment, and force ∧ lever arm is the same moment with the opposite sign. |
| **Organelle** | A small fixed shader that draws one kind of shape from the tables. Phase 6 has the shape and the curve. Glyph, image, and snapshot are Phase 7.2. |
| **Owner** | The one thing a pixel belongs to: the background, the surface, a cell, a port, or a wire. In a contact body there is no wire, and no force owns a pixel. |
| **Pair** | The two partners across one mirror, such as force and displacement. Multiplied, they give energy. Part of a quantity's tag. |
| **Primitive** | A sealed basic piece below ordinary cells. Creators compose with them; they don’t invent new floor primitives casually. |
| **Refinement** | Cutting a model into finer pieces (the beam into 1 ft lines) and checking that no answer at a shared point moves. An exact derivation cannot be changed by it; a sloppy one is caught. |
| **Region** | In the assay, one connected piece of a body's inside (cells joined by wires). A body with two regions is two separate things sharing a container. |
| **Refusal / Verdict** | A normal value meaning “no” (`Refused`), not a crash. Host IO errors are different. |
| **Regrow** | Rebuild the tables from the body's DNA and its live state. They must match the delta-built tables byte for byte, and a discarded GPU redraws the same picture from them. |
| **Regulatory region** | The unhashed part of DNA: looks, labels, literals, layout choices — versioned, not the identity. |
| **Response** | The cell a force answers with, named in the force (`as sum`) and pinned by hash to the register's cell. Its out-port holds the result. |
| **Role** | What a cell does in a contact body, derived and never written: it faces out (has a surface port) or in, and holds or reacts (is a response). The four pairs print as **protect** (out, holds), **carry** (out, reacts), **store** (in, holds) and **respond** (in, reacts). |
| **Surface** | The derived outline of a body: every cell port nothing inside uses up. Never declared, and never called a membrane. |
| **Surface port** | One port on a body's surface. In a contact body: every genome cell port that is not a force's member, and each response's out-port. The calculator's are `cli_a@0`, `cli_b@0` and `sum@2`. |
| **Seal** | A locked pair: a reference body and checks (including a counterfeit that must be caught). |
| **Side** | Which side of the mirror a quantity is on: placement (how things sit and move: lengths, deflection, rotation) or source (what pushes: loads, reactions, moment). Energy is their shared product. Part of a quantity's tag. |
| **Tick** | One redraw. It carries the camera. An idle scene has nothing to draw, so the window waits. |
| **Where** | The piece a quantity lives on, with its orientation: a point, a line, or the plane, and which way it faces. Its dimension, as opposed to its unit. Part of a quantity's tag. |
| **Witness** | A recorded true result that future versions must still satisfy — testimony the gate can replay. |
| **Witness library** | A library that states formulas JoInn derives itself, such as AISC Table 3-23 or Roark Table 8.1. Compared exactly and never used to derive; a disagreement is a truth violation. |
| **xtask** | Helper commands that measure and enforce project rules (`floor`, `agree`, `gate`, and so on). |

## Quick analogies

- **Cell** ≈ Lego brick  
- **DNA** ≈ recipe + rules printed on the brick  
- **Gate** ≈ health inspector  
- **Host** ≈ storefront  
- **Corpus** ≈ the scrapbook of “this worked” (and “this must fail”)  
- **Floor** ≈ the foundation of the building  

## Next

Back to the start: [README.md](README.md).

> **29 Sep 2026.** The theory moved ahead of the code: see *Theory/JoInn Cells, Bodies and Forces.md* (Part V). In the theory, a body has no wires inside (its cells touch), operations like sum are **forces** from outside (combine / separate, carry / release), and wires belong to systems.
>
> **30 Sep 2026.** Phase 7 builds the first of it: contact bodies with one force, combine. The wired form still runs beside it.
