# Glossary (plain English)

Short definitions for words you’ll see in JoInn docs. Deeper, precise meanings live in Theory; this page is for clarity.

| Term | Plain meaning |
|------|----------------|
| **Allele** | One candidate way to *implement* a cell’s plan. The plan’s identity is the laws; the allele is a payload that must pass those laws. |
| **Anchor** | The chart the camera's focus is measured in: the deepest chart under the window's centre pixel. Positions are kept relative to it, so the numbers stay small at any zoom. |
| **Arrowhead** | A small triangle on a link. A spine has one at the middle of each stretch between consecutive members, pointing along it; any link has one on the stub of each tail member, pointing away from the port. An unordered link never has one along its legs. |
| **Assay** | A measurement of a body or universe's shape without running it: the loops a value can travel, and which of them something closes. It reports; it never refuses on its own. |
| **Band** | How a body is drawn, chosen by its size on screen: a dot below 4 px, a glyph (the surface) up to 32, a summary (cells and surface ports) up to 240, and full (everything, with text) above. Every band decision is a whole-number test, the same on the CPU and the GPU. |
| **Blueprint** | A shareable cell design others can reuse — meant to be seen, not only read. |
| **Body** | A cluster of cells that belong together, with one shared genome. The main “container” for a small machine or app piece. |
| **Bootstrap** | One of the three big build stages (true-but-unseen → seen-and-touched → self-editing). |
| **Bridge** | A law that crosses from the placement side to the source side and back, such as moment to curvature through E·I. It cannot be derived, so it is testimony, read from a pinned edition. Balance and moment need none; deflection needs one. |
| **Bridge edition** | The small pinned file a bridge is read from, naming its source: the beam spike's is `bridges.edition` (AISC Manual 16th ed.: A992 E 29000 ksi, W12x26 Ix 204 in⁴). Every read of it is counted. |
| **Bundle** | A large link's form (from 1920 px across): its legs drawn thicker where more of them share a stretch of street, so you can see how much travels together. |
| **Camera** | How layout units become pixels. Since Phase 7.2 it is exact: a zoom level and step, a focus, a pin, and the window's size. A pan or zoom writes the tick and no table row. Phase 6's integer-scale camera converts to it exactly. |
| **Cell** | The smallest visual building block. Something you can show and still use. |
| **Chaos** | Only the scientific sense: behavior fully fixed by its laws and its starting state, which looks random only to someone who doesn't know that state exactly. Never "disorder" or "chance". JoInn has no randomness. |
| **Chart** | A coordinate system with a whole-unit origin in its parent: the universe, then galaxies, systems and bodies. A body's cells, ports and text live in its chart. |
| **Coding region** | The part of DNA that is hashed and gated: frame, contract, laws, witnesses — the cell’s true identity. |
| **Combine** | The one force Phase 7 writes: put the values of its members together with the frame's answer (on integers, the sum). Its answer must not depend on member order. |
| **Contact body** | A body whose cells touch, with no wires inside; its operations are forces. Written as a `.contact` file, which cannot express a wire. |
| **Corpus** | The library of golden examples (and deliberate fakes) used as testimony and regression checks. |
| **Declaration** | A sentence in a body's or universe's hashed part that the assay must satisfy. Today the only one is `assert H₁ = 0` (“no loop is open”); if it fails, the body or universe is refused at admission. |
| **Delta** | The table rows one run changed. The picture updates by writing those rows. |
| **Crate** | A Rust library or program package in the repo (for example `joinn-gate`). |
| **Crossfade** | Near a band threshold T (from T up to 1.2·T) the lower band fades out while the higher fades in, so the dot you zoom into is seen to become the body. |
| **Cut (the)** | What the CPU decides is drawn under one camera: which charts are visible, which galaxies and systems are open, and each body's band. The GPU makes the same whole-number decisions for each thing it draws, and the owners in its picture must be exactly the cut's. |
| **DNA** | The building plan for a cell: what it is and what laws it obeys. |
| **Edge pixel** | A pixel within 1/16 of a pixel of a shape's boundary. It is counted and never judged: the two pickers need not agree there. |
| **Elevation plane** | The flat picture a beam lives in: x along the span, y up. A moment lives on the plane itself (x ∧ y), which is why a beam is not only a line. |
| **Filling** | What closes a loop in the assay: a frame (the value keeps its kind all the way round), or one law that promises to undo a conversion, named by body, instance and law. Nothing else closes a loop. |
| **Floor** | The sealed set of basic operations JoInn is written in. Admitted only if irreducible and opposed. Never “fixed” by renaming its size. |
| **Fold state** | Which charts are folded into lens nodes at a zoom: *open*, *systems folded* or *galaxies folded*. Every system has one size and every galaxy another, so a universe has at most three, and every link has one route per fold state, computed when the universe opens. |
| **Focus** | The point of the anchor chart the camera holds still, kept in 2^-16 layout units. Zooming never moves it. Zooming about a different pixel moves it once (a refocus), the camera's only rounding. |
| **Force** | An operation applied to a body's cells from outside, such as combine. It names its members (cell ports) and a response. A force owns no pixel. |
| **Force register** | The short table of which forces a frame answers, and with which cell: combine on integers answers with the sum cell. A row is admitted only if the answer is order-blind and its opposite (a turn of the response) is on file. |
| **Frame** | The context in which truth is judged (example: integers, or a text frame). |
| **Gate** | The checker that admits good growth and refuses bad growth. |
| **Genome** | The full set of DNA shared by the cells of one body. |
| **Gutter** | An empty lane of the grid layout between bodies, systems or galaxies. Links travel only on gutters, so they never cross a cell, a body that isn't a member, or a folded node. |
| **Grove (the)** | JoInn's generated test universe: seed 7 gives 8 galaxies, 128 systems, 3072 bodies and 137 links, bound from corpus bodies by hash. It is grown wherever it is needed and never stored in the corpus or the docs. |
| **Island** | In the assay, one connected piece of a universe once the outside world is removed. |
| **Hub** | A medium link's form (240 to 1920 px across): thin legs meeting at a knot, drawn as a small disc. |
| **Host** | Whatever presents the universe to the outside (terminal, test harness, desktop window). A host is proved by there being more than one. |
| **ID target** | The second picture: one integer per pixel naming its owner. A click reads one texel of it. |
| **Knot** | Where an unordered link's legs meet: the street corner with the smallest total distance to all its touch points (ties go to the smallest y, then x). A link whose touch points are one point has none. |
| **Latent** | A response that holds no value yet. It is drawn dim, and takes its normal color once a value arrives. |
| **Layout unit** | The integer length the tables use for positions. Pixels are what the camera makes of them. |
| **Leg** | The shortest street path from a link's knot to one touch point. A leg serves one member; where several legs share a stretch, that stretch is trunk. A click on a leg prints `link <id> member <i>`. |
| **Lens node** | A system or galaxy too small on screen to open (below the summary band), drawn as one node. Its bodies are not walked. |
| **Live engine** | The runner that executes a body now, with an explicit step budget. |
| **Lower** | The engine's own derivation of the deliveries a contact body needs to run. Its only inputs are the contact, its cells and the force register. It is never written to a file, described, or drawn. |
| **Membrane** | The boundary of a cell — the only place it meets anything outside itself. A body has no membrane; its outline is its *surface*. |
| **Order-blind** | A result that doesn't depend on the order its members were combined in, such as a sum. The order still happened, and JoInn records it. (Earlier documents said *order-free*.) |
| **Order-bound** | A result that depends on order completely, so the order is part of its truth. |
| **Order-signed** | A result whose order changes at most its sign: lever arm ∧ force is a moment, and force ∧ lever arm is the same moment with the opposite sign. |
| **Organelle** | A small fixed shader that draws one kind of shape from the tables. Phase 6 has the shape and the curve; Phase 7.2 adds frames, the dot and strokes; Phase 7.3 adds link segments (capsules, knot discs and arrowheads). Images and snapshots are later. |
| **Owner** | The one thing a pixel belongs to: the background, a galaxy or system frame, the surface, a cell, a port, a wire, or a link (`link <id>`, or `link <id> member <i>` on a leg or stub). In a contact body there is no wire, and no force owns a pixel. |
| **Owner band** | The band that owns a body's pixels in the ID picture: the higher band once 10·s ≥ 11·T, the middle of its fade window. The other band only fades, in colour. |
| **Pair** | The two partners across one mirror, such as force and displacement. Multiplied, they give energy. Part of a quantity's tag. |
| **Pin** | The whole pixel the focus sits on. Panning moves the pin; zooming does not. |
| **Primitive** | A sealed basic piece below ordinary cells. Creators compose with them; they don’t invent new floor primitives casually. |
| **Rebase** | Moving the camera's anchor to the chart now under the centre pixel. It writes chart rows only and moves no pixel. |
| **Refinement** | Cutting a model into finer pieces (the beam into 1 ft lines) and checking that no answer at a shared point moves. An exact derivation cannot be changed by it; a sloppy one is caught. |
| **Region** | In the assay, one connected piece of a body's inside (cells joined by wires). A body with two regions is two separate things sharing a container. As a link's form: a small link (under 240 px across) drawn as a pale, wide lane joining its members. |
| **Refusal / Verdict** | A normal value meaning “no” (`Refused`), not a crash. Host IO errors are different. |
| **Regrow** | Rebuild the tables from the body's DNA and its live state. They must match the delta-built tables byte for byte, and a discarded GPU redraws the same picture from them. |
| **Regulatory region** | The unhashed part of DNA: looks, labels, literals, layout choices — versioned, not the identity. |
| **Response** | The cell a force answers with, named in the force (`as sum`) and pinned by hash to the register's cell. Its out-port holds the result. |
| **Routing graph** | Every gutter of a fold state, split at every crossing and every stub's end: the streets a link can take. The grove's open graph has 6725 corners and 13028 stretches. |
| **Role** | What a cell does in a contact body, derived and never written: it faces out (has a surface port) or in, and holds or reacts (is a response). The four pairs print as **protect** (out, holds), **carry** (out, reacts), **store** (in, holds) and **respond** (in, reacts). |
| **Stub** | The short line from a member's port out to its gutter, drawn when the member's body is drawn. It ends exactly on the port's centre. The grove has 1182 when framed. |
| **Surface** | The derived outline of a body: every cell port nothing inside uses up. Never declared, and never called a membrane. |
| **Surface port** | One port on a body's surface. In a contact body: every genome cell port that is not a force's member, and each response's out-port. The calculator's are `cli_a@0`, `cli_b@0` and `sum@2`. |
| **Seal** | A locked pair: a reference body and checks (including a counterfeit that must be caught). |
| **Side** | Which side of the mirror a quantity is on: placement (how things sit and move: lengths, deflection, rotation) or source (what pushes: loads, reactions, moment). Energy is their shared product. Part of a quantity's tag. |
| **Spine** | How a link with declared order is drawn, at every size: one path through its members in order, with an arrowhead in the middle of each stretch. |
| **Step** | The fraction of a zoom level: k = 2^level·(256 + step)/256, with step 0 to 255. One notch is 32 steps, so eight notches double k. |
| **Stroke font** | JoInn's own letters: 48 glyphs drawn as straight strokes on a 6 × 8 grid (digits, a–z and a few signs), used for titles, cell labels and values. A character outside the set is refused. |
| **Tick** | One redraw. It carries the camera. An idle scene has nothing to draw, so the window waits. |
| **Touch** | A link reaching a cut node: a member's body if it is drawn, otherwise the lens node that holds it. Each node is touched at most once per link. |
| **Touch point** | Where a link's route meets one node: a port's centre when the body is drawn, or the side of a folded system or galaxy that faces the knot. Members inside one folded node share one touch point and one leg. |
| **Trunk** | A stretch of street that two or more legs of one link share. It serves no one member, so a click on it prints `link <id>`. |
| **Where** | The piece a quantity lives on, with its orientation: a point, a line, or the plane, and which way it faces. Its dimension, as opposed to its unit. Part of a quantity's tag. |
| **Witness** | A recorded true result that future versions must still satisfy — testimony the gate can replay. |
| **Witness library** | A library that states formulas JoInn derives itself, such as AISC Table 3-23 or Roark Table 8.1. Compared exactly and never used to derive; a disagreement is a truth violation. |
| **xtask** | Helper commands that measure and enforce project rules (`floor`, `agree`, `gate`, and so on). |
| **Zoom level** | The whole doublings of the camera's scale, from −4 to 9; any other level is refused. With the step it gives k, the exact number of pixels per layout unit. |

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
>
> **5 Oct 2026.** Phase 7.2 adds the zoom: charts, the exact camera, bands, the cut, lens nodes, touches and the stroke font, measured on the grove.
>
> **6 Oct 2026.** Phase 7.3 draws links: gutters, the routing graph, touch points, stubs, knots, legs and trunk, one route per fold state, and four forms (region, hub, bundle, spine) with arrowheads.
