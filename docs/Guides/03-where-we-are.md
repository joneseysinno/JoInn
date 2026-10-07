# Where we are

## The three ladders (bootstraps)

JoInn is being built in three big stages. Think of them as ladders: each one lets you climb farther.

| Stage | Nickname | What it means in plain words | Status (roughly) |
|-------|----------|------------------------------|------------------|
| **B0** | True but unseen | The universe is correct and checkable, and you can still meet it as text | **The truth core holds** |
| **B1** | Seen and touched | Same universe drawn and picked, at any zoom | **Under way — a universe of thousands of bodies zooms exactly, its links drawn; a system grows by its DNA, its force drawn as a lasso; systems in universes next** |
| **B2** | Self-editing | You build JoInn *inside* JoInn; the visual creator and the text form match byte-for-byte | Future |

The rule of the road: **pixels come after truth.** A gorgeous editor with a weak core would force the core to bend. So Bootstrap 0 comes first on purpose.

## What already works (today)

You can, from the `joinn` code folder:

- Run the **calculator** demo in a terminal (`joinn run calculator`)
- Run the **universe** (`joinn run universe`): the calculator's transcript, then `factor: 12` and `5 ft = 60 in`
- See a **refusal** when input isn’t a real integer (`"two"`), then a correct `2 + 3 = 5`
- Run machines that check vocabulary, the floor, agreement, gates, and the corpus (see the code README)
- A gate control reads the parsed body, universe, or transcript, and flips when that artifact is damaged in a way that still parses
- Assay a body or universe for loops nothing closes (`cargo xtask assay phase4/loop.universe`), described below
- Open one body in a window (`cargo run -p joinn-shell-desktop -- corpus/phase2/calculator.body`), described below
- Run and open the calculator as **cells in contact** (`joinn run corpus/phase7/calculator.contact`, and the same path in the window), described below
- Work five steel beams exactly (`cargo run --manifest-path spikes/s8-beam/Cargo.toml`), described below
- Open a universe and zoom it (`cargo run -p joinn-shell-desktop -- corpus/phase5/universe.universe`), and print the generated test universe's counts (`cargo xtask grove`) and what each zoom draws (`cargo xtask zoom`), described below
- See its links drawn in the same window, and print every route (`cargo xtask links`), the form each link takes at each zoom (`--forms`) and how long routing takes (`--measure`), described below
- Open a counting system and grow it by typing (`cargo run -p joinn-shell-desktop -- corpus/phase74/counting.system`), and print every growth with its counting witness (`cargo xtask grow`), described below

## A system grows (Phase 7.4)

A **system** is a body and the forces that act on it. It starts as a seed: no cells yet, only the DNA that says how the body grows. The first is `corpus/phase74/counting.system`: a body of numbers and one force, *combine*, answered by a cell called `count`.

- **The DNA says how it grows.** The body's `.contact` file says `grows { cell:… as numbers accepts one }`: each new cell is a copy of that one cell, named `numbers.0`, `numbers.1`, and so on, and counting accepts only the number 1. Growing adds no line to any file; the cells grown so far are the system's **grown state**, which is never hashed or stored. The system's hash is the same at every size.
- **Every size is true.** With no cells, `count` is 0 (combine's identity). With one, it is that input. With n, it is all of them combined in order. `3` is refused in counting's own words: `counting: 3 is not one; acceptance is 1 (counting grows by one)`.
- **Counting is the witness.** After every step, a second derivation that only counts up and down by one (`count_witness`) must give the same answer as the engine. `cargo xtask grow` runs ten transcripts and prints both side by side.
- **Adding evolved from counting.** `adding.system` names counting as its parent and accepts any integer. The evolution check holds it to every one of counting's witnesses (1 1 1 still gives 3), and requires it to gain something counting refuses: adding accepts `3`. A child that accepts nothing new is refused, because that would be an edit, not an evolution.
- **A force is seen as a lasso.** The force is drawn as a loop around the whole body, clear of every cell, with a neck and an arrowhead pointing at `count`. It is drawn under the cells, so a cell always owns its own pixels. A click on the lasso prints `force count`, and the GPU's ID for that pixel names the same force on every adapter.

In the window, an empty **waiting box** sits beside the grown cells. Click it, type `1` and press Enter: the body grows by one cell and `count` goes up. Typing `3` prints the refusal and nothing grows. The camera doesn't follow the growing body; `F` frames it again.

Each step costs about 0.15 ms with no cells and about 8 ms at 96, because the engine rebuilds the body and runs it again on every step (`cargo xtask grow --measure`). Gate 7.4 checks all of this, on every graphics adapter for the lasso.

Not built yet: systems inside a universe, a system of several bodies, removing a number, and growth that doesn't rebuild the body each step. Findings: [phase-7.4-growth.md](../Findings/phase-7.4-growth.md).

## Links are drawn (Phase 7.3)

A link between bodies is a **hyperedge**: one connection that can touch many bodies at once. The window now draws every link of any universe.

- **Streets only.** Links travel on the **gutters**, the empty lanes between bodies, systems and galaxies. They never cross a cell, a body that isn't a member, or a folded system. A short **stub** runs from each member's port out to its gutter. `cargo xtask links` checks every route exactly and prints `crossings 0`.
- **One meeting point.** An unordered link's **legs** meet at a **knot**, the street corner closest to all its members together. A stretch several legs share is **trunk**.
- **Form follows size.** A small link (under 240 px across) is a pale, wide **region**; a medium one is thin lines meeting at the knot (**hub**); a large one (from 1920 px) draws each shared stretch thicker the more legs share it (**bundle**). Near each threshold the two forms crossfade. A link whose order is declared is a **spine** with arrowheads; an unordered link never has an arrow along it.
- **A folded system is touched once.** Zoomed out, a system folds into one tile, and a link touches it at one point with one leg, however many of its members are inside.
- **Under bodies, and picked like them.** Links are drawn beneath bodies, so a body always owns its own pixels. A click on a line prints `link sys_g0s00 member 3`, and the GPU's ID for that pixel names the same thing on every adapter.
- **Grown once.** Every route for every fold state is computed when the universe opens. Zooming and panning still write no row.

On the grove (137 links), routing takes about 1 s in the everyday build and 0.63 s optimized, once, when it opens. Gate 7.3 checks all of this on every graphics adapter.

Not built yet: snapshots (a saved picture of a view), routes that move when the layout changes, and streets or knots as things in their own right. Findings: [phase-7.3-links.md](../Findings/phase-7.3-links.md).

## The universe zooms (Phase 7.2)

The window now opens a whole universe, not only one body. Galaxies hold systems, systems hold bodies, and each sits on a fixed grid, in its own coordinate system (a **chart**). You zoom with the wheel or `+` and `-`, pan by dragging or with the arrow keys, and `F` frames everything again. From the whole universe down to a single letter of a cell's label is a range of 8192 times (zoom level −4 to 9).

What makes it trustworthy:

- **An exact camera.** The scale is a power of two times a whole step out of 256, the point held still is kept in 2^-16 of a unit, and the pixel it sits on is a whole pixel. Eight notches in and eight out about the same pixel return the identical camera and the identical picture.
- **Size decides how a body is drawn.** Below 4 px a body is a dot; then its surface; from 32 px its cells and surface ports; from 240 px everything, with text. Near each threshold the two looks crossfade, so you can see the dot become the body. A system too small to open is drawn as one node.
- **Two cuts, one truth.** The CPU decides what is drawn under each camera (the **cut**); the GPU makes the same whole-number decisions for each thing it draws. On every adapter, the owners in the GPU's picture are exactly the ones the cut allows, and every pixel away from an edge has the owner the exact CPU pick names.
- **Moving writes nothing.** A pan or zoom changes only the camera; no table row is written. When the centre moves into another chart, a **rebase** rewrites chart rows only, and not one pixel changes.
- **JoInn's own letters.** Titles, cell labels and values are drawn in a 48-glyph stroke font, the same on every machine.

It is measured on **the grove**, a universe JoInn generates from seed 7: 8 galaxies, 128 systems, 3072 bodies and 137 links. The CPU cut takes 19–66 µs per view. A link already knows which drawn bodies or nodes it touches (264 at the farthest zoom, 1182 when framed); Phase 7.3 draws them (above).

Findings: [phase-7.2-zoom.md](../Findings/phase-7.2-zoom.md).

## The beam, worked exactly (Phase 7.1, stage 1)

JoInn has worked its first real engineering: five W12x26 beams in A992 steel, every answer an exact fraction with its decimal beside it. A 24 ft span under 1.2 kip/ft gives reactions of 72/5 kip (14.4), a midspan moment of 432/5 kip·ft (86.4) and a deflection of 93312/61625 in (about 1.514). The other four are a midspan point load, a point load at 6 ft, a 10 ft cantilever (its wall moment prints **hogging**), and the uniform and point loads together, a case no table lists.

How it gets them:

- **Balance first, with no material.** The reactions come from the forces and turning effects summing to zero. Nothing from a library is read; the program counts its reads of E and I and prints `no bridge read`.
- **Adding up along the span.** Shear is the load added up and moment is the shear added up, exactly, so wL²/8 comes out without being looked up. The moment at the far support must come out exactly 0.
- **One crossing of the mirror.** Deflection is the only answer that needs testimony: E = 29,000 ksi and Ix = 204 in⁴, from a small pinned file (`bridges.edition`, AISC Manual 16th ed.).
- **Libraries as witnesses.** Each answer is compared with the formula AISC Table 3-23 and Roark Table 8.1 state; all 12 agree. A formula is never used to get an answer.
- **Refinement.** Cutting the beam into 1 ft pieces changes no answer.

Every quantity carries a tag: its side (placement, source, or energy), a length count, and where it lives in the beam's elevation plane (along the span, up, or the plane itself). Five planted mistakes are each refused: a moment taken in mixed order, a sloppy rectangle-rule sum, a wrong witness (wL²/12), a moment added to a work (both kip·in, told apart only by their tags), and deflection with no edition.

This is a **spike**: a small separate program in `joinn/spikes/s8-beam`, outside JoInn's own files, and no gate measures it yet. Not built yet: the tag in JoInn's own grammar, ℚ with a turn and multiply as a force, balance and adding-up as forces in a contact body, the column, links between members, and beams that need their material to find their reactions (propped or fixed ends). Those are stage 2 and later. Findings: [phase-7.1-beam-examples.md](../Findings/phase-7.1-beam-examples.md).

## Cells in contact (Phase 7)

A body can now be cells in contact. Inside it there are no wires: the cells touch, and an operation such as sum is a **force** applied from outside. The contact calculator (`corpus/phase7/calculator.contact`) lists two input cells, `cli_a` and `cli_b`, and one force: *combine* the numbers they hold, and put the result in a response named `sum`.

What makes that safe:

- **The force register.** A force is only allowed on a frame that has a row in a short table: combine on integers answers with the sum cell. The row is admitted only if the answer doesn't depend on the order of what's combined (checked on 64 pairs and 64 triples), and only if its opposite is on file (`sum_turn`, which gives one part back). A difference or a midpoint is refused with a counterexample.
- **Admission.** A contact body is refused if a member can't feed the force (`receptor: cli_a@0 is Text 1, …`), if the response isn't the registered one, if a force has the wrong number of members, or if a force reaches back to itself.
- **One truth, two forms.** The engine derives, for its own use, the deliveries a contact body needs to run (`lower`). That derivation is never written to a file, described, or drawn. For the calculator it is exactly the wired `calculator.body`, so both forms print the same transcript, give the same descriptions, and have the same surface (`cli_a@0`, `cli_b@0`, `sum@2`). Swapping which member feeds which port still gives `5`.
- **Roles are derived, never written.** Each cell faces out (it has a port on the surface) or in, and holds or reacts (it is a force's response). The four pairs print as protect, carry, store and respond: the calculator is two protects and a carry (`cargo xtask roles corpus/phase7/calculator.contact`).

In the window, the contact calculator is three cells touching: `cli_a` above `cli_b`, and `sum` beside both. There is no wire to draw, and a force owns no pixel. `sum` is drawn dim (*latent*) until it holds a value, and takes its normal color when the second number arrives.

Not built yet: multiplication (R83), writing *separate* as a force, many members and chains of forces beyond one test fixture (R84), one port feeding two forces (R85), and wires between bodies as systems. Text on screen, zoom and charts came in Phase 7.2, above. The steel beam (Phase 7.1) is above too.

## The picture (Phase 6)

JoInn draws one body. The calculator appears as a surface holding three cells, their ports, and two wires. The picture is the tables: a pixel's color is the style of the row that owns it, so a refused cell and a filled port look different because those rows changed. A click prints that owner (`body.sum`, `body.cli_a@0`, `body wire cli_a@1 -> sum@0`). The GPU's ID for the same pixel names the same owner. Typing a number into an in-port on the surface runs the body, and the window changes because the run wrote rows.

Phase 6's window drew no text, no second body, and no zoom; Phase 7.2 added all three (above). Images and snapshots are still ahead. A GPU that cannot read storage in the vertex stage is still an open question (R72).

## The assay (Phase 4)

JoInn can now measure the *shape* of a body or a universe without running it. The **assay** (`cargo xtask assay <path>`) counts the loops a value can travel: in from the outside world and back out, or around between bodies through links. For each loop it says whether something closes it — the loop keeps its kind of value (a number stays a number), or a cell carries a law that promises to undo the conversion, naming the cell it pairs with. A loop nothing closes is printed as `open:`.

What it measures:

- **regions** — how many separate pieces a body's inside falls into (a body whose cells aren't wired together is "two things");
- **islands** — how many separate pieces the whole universe falls into once the outside world is removed;
- **open loops** (the number printed as `H₁`) — round trips that no frame and no law closes.

What it does not measure: whether a value is *wrong*, how anything is implemented (alleles are invisible to it), labels, names, or lenses. It also never refuses on its own. Only a body or universe that writes `declarations { assert H₁ = 0 }` is refused at admission when a loop is open; without the declaration the same loop is just reported.

C1 shows a missing promise, not a wrong number. A narrower check (does a round-trip law name the cell it is linked to?) would also see this one case. No such check exists in the tree, which is why the rule kept the assay. What the assay adds is finding which loops need a promise at all, as universes grow.

Under the hood, the project has already built the early “truth core”: frames and values, DNA plans, the gate, the floor of sealed basics, the live runner, and the start of **hosts** (a host is proved by there being two — CLI and a test host).

The window shows a universe with its links and zooms it, but it is not an editor. Building any app by clicking, zooming, and reading the screen aloud is still ahead in Bootstrap 1 and 2.

## What “Phase” language means

Builders and coding agents talk in **phases** (Phase 0 paper, Phase 1 truth core, Phase 2 floor + live engine, Phase 3 hosts, then later visual and creator phases).

You don’t need the phase numbers to understand the project. Use this translation:

- **Early phases** ≈ Bootstrap 0 (true but unseen)  
- **Middle phases** ≈ Bootstrap 1 (seen and touched)  
- **Late phases** ≈ Bootstrap 2 (self-editing)

If an agent mentions “Phase 3,” it means: proving hosts properly (description as a value; more than one host), still without the big visual product.

## Why stop at “unseen” for a while?

Because JoInn’s claim is **truth first**. The calculator transcript — including the refusal — is evidence that the system can say “no” and still compute when the input is honest.

The window is that view: a universe, drawn from the same tables the engine mutates.

## Next

Try the demo yourself: [04-try-the-calculator.md](04-try-the-calculator.md).

Full build order (detailed): [../Plans/JoInn Build Roadmap.md](../Plans/JoInn Build Roadmap.md).
