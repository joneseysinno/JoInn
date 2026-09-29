# Where we are

## The three ladders (bootstraps)

JoInn is being built in three big stages. Think of them as ladders: each one lets you climb farther.

| Stage | Nickname | What it means in plain words | Status (roughly) |
|-------|----------|------------------------------|------------------|
| **B0** | True but unseen | The universe is correct and checkable, and you can still meet it as text | **The truth core holds** |
| **B1** | Seen and touched | Same universe drawn and picked; zoom comes next | **Started — one body is on screen** |
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

## The picture (Phase 6)

JoInn draws one body. The calculator appears as a membrane holding three cells, their ports, and two wires. The picture is the tables: a pixel's color is the style of the row that owns it, so a refused cell and a filled port look different because those rows changed. A click prints that owner (`body.sum`, `body.cli_a@0`, `body wire cli_a@1 -> sum@0`). The GPU's ID for the same pixel names the same owner. Typing a number into an in-port on the membrane runs the body, and the window changes because the run wrote rows.

The window draws no text, no second body, and no zoom. Glyphs, images, and snapshots wait for Phase 7. A GPU that cannot read storage in the vertex stage is still an open question (R72).

## The assay (Phase 4)

JoInn can now measure the *shape* of a body or a universe without running it. The **assay** (`cargo xtask assay <path>`) counts the loops a value can travel: in from the outside world and back out, or around between bodies through links. For each loop it says whether something closes it — the loop keeps its kind of value (a number stays a number), or a cell carries a law that promises to undo the conversion, naming the cell it pairs with. A loop nothing closes is printed as `open:`.

What it measures:

- **regions** — how many separate pieces a body's inside falls into (a body whose cells aren't wired together is "two things");
- **islands** — how many separate pieces the whole universe falls into once the outside world is removed;
- **open loops** (the number printed as `H₁`) — round trips that no frame and no law closes.

What it does not measure: whether a value is *wrong*, how anything is implemented (alleles are invisible to it), labels, names, or lenses. It also never refuses on its own. Only a body or universe that writes `declarations { assert H₁ = 0 }` is refused at admission when a loop is open; without the declaration the same loop is just reported.

C1 shows a missing promise, not a wrong number. A narrower check (does a round-trip law name the cell it is linked to?) would also see this one case. No such check exists in the tree, which is why the rule kept the assay. What the assay adds is finding which loops need a promise at all, as universes grow.

Under the hood, the project has already built the early “truth core”: frames and values, DNA plans, the gate, the floor of sealed basics, the live runner, and the start of **hosts** (a host is proved by there being two — CLI and a test host).

The window is one body, not an editor. Building any app by clicking, zooming, and reading the screen aloud is still ahead in Bootstrap 1 and 2.

## What “Phase” language means

Builders and coding agents talk in **phases** (Phase 0 paper, Phase 1 truth core, Phase 2 floor + live engine, Phase 3 hosts, then later visual and creator phases).

You don’t need the phase numbers to understand the project. Use this translation:

- **Early phases** ≈ Bootstrap 0 (true but unseen)  
- **Middle phases** ≈ Bootstrap 1 (seen and touched)  
- **Late phases** ≈ Bootstrap 2 (self-editing)

If an agent mentions “Phase 3,” it means: proving hosts properly (description as a value; more than one host), still without the big visual product.

## Why stop at “unseen” for a while?

Because JoInn’s claim is **truth first**. The calculator transcript — including the refusal — is evidence that the system can say “no” and still compute when the input is honest.

The window is that view: one body, drawn from the same tables the engine mutates.

## Next

Try the demo yourself: [04-try-the-calculator.md](04-try-the-calculator.md).

Full build order (detailed): [../Plans/JoInn Build Roadmap.md](../Plans/JoInn Build Roadmap.md).
