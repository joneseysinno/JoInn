# JoInn

This folder is the **running code** for JoInn: libraries (crates), the calculator demo, golden examples (`corpus/`), and build checks (`xtask`).

JoInn is a **truth-first** platform for composing apps from small blocks called **cells**. The terminal calculator still refuses bad input and computes `2 + 3`. A desktop window draws a body or a whole universe: the picture is the tables, and every pixel names its owner. A universe zooms exactly from all its galaxies down to one letter of a cell's label (Phase 7.2). Links between bodies are not drawn yet, and there are no snapshots.

The calculator now exists in two forms. `corpus/phase2/calculator.body` joins its cells with wires. `corpus/phase7/calculator.contact` is a **contact body**: its cells touch, and sum is a **force** (combine) with no wire inside. Both run the same transcript, give the same descriptions, and draw from the same kind of tables.

## Start here (humans)

Plain-English guides — big picture, connection diagrams, where we are, try-it steps, glossary:

**[../docs/Guides/](../docs/Guides/)**

Docs index: [../docs/README.md](../docs/README.md)

## Try it

From this folder (`joinn/`):

```text
cargo run -p joinn-cli -- run calculator
```

You should see a refusal for `"two"`, then `2 + 3 = 5`. Details: [Try the calculator](../docs/Guides/04-try-the-calculator.md).

The contact form prints the same lines:

```text
cargo run -p joinn-cli -- run corpus/phase7/calculator.contact
```

Open a universe in the window and zoom it (wheel or `+`/`-`, drag or arrows to pan, `F` to frame); it prints one `tick:` line per change and nothing when idle:

```text
cargo run -p joinn-shell-desktop -- corpus/phase5/universe.universe
```

Open a system and grow it: click the empty box, type `1` and press Enter, and the body grows by one cell and its count goes up; `3` is refused in counting's words. A click on the lasso prints `force count`:

```text
cargo run -p joinn-shell-desktop -- corpus/phase74/counting.system
```

## Gates

`cargo xtask gate all` runs the proof in order: phase 0 (the corpus verifies), gate 1 (admission), gate 2 (the calculator, opposed), gate 2.1 (seals), gate 2.2 (a reference allele is a body), gate 3 (two hosts, one body), gate 4 (the assay reads a known sample, a promise names its partner, a body that is two things is named, and a declaration refuses where an assay only reports), gate 5 (a typed link between two bodies), gate 5.1 (a value crosses), gate 5.2 (a control sees a parsed value and flips on a mutant that still parses), gate 6 (one body drawn from its tables: every pixel has one owner and both pickers name it, the picture shows the row the engine computed, and a stale click is refused), gate 7 (the contact calculator and the wired one are one truth, combine is order-blind and fits what it reaches, and the contact body is drawn as cells in contact with no link), gate 7.2 (the zoom is exact and a rebase moves nothing, bands follow size and two pickers name every pixel, and a folded system is one node that a link touches once), gate 7.3 (a hyperedge touches and never crosses, order is drawn only when declared and the form follows size, and a folded system is touched once), and gate 7.4 (a body grows by its DNA and every size is true, evolution keeps every old witness and gains, and a force is seen as a lasso while growth is not identity). Gate 4 was built after gate 5.2 and runs in its phase-number place. The lock records the score each gate returned. This file does not quote one.

## The assay

`cargo xtask assay <path>` measures a body or universe without running it. It prints each body's **regions** (connected pieces of its inside), the **islands** of the universe with the outside removed, and every loop a value can travel through the outside or across links, each either `filled:` (closed by a frame, or by one law that names the cell it undoes) or `open:`. It does not see values, alleles, labels, names or lenses, and it never refuses on its own. A body or universe that declares `declarations { assert H₁ = 0 }` is refused at admission when a loop is open. Findings: [assay-corpus.md](../docs/Findings/assay-corpus.md), [decoration-check.md](../docs/Findings/decoration-check.md).

## Proof commands

| Command | Plain meaning |
|---------|----------------|
| `cargo test --workspace` | Run automated tests |
| `cargo xtask vocab` | Vocabulary / layout rules |
| `cargo xtask floor` | Check the sealed foundation |
| `cargo xtask agree` | Opposition / agreement checks |
| `cargo xtask gate all` | Gate proof suite |
| `cargo xtask power` | Strength checks for the current freeze |
| `cargo xtask corpus verify` | Golden corpus still matches |
| `cargo xtask assay <path>` | Loops in one body or universe, filled or open |
| `cargo xtask assay agree` | Two derivations of the assay agree on the corpus |
| `cargo xtask assay invariance` | The assay holds still under labels, alleles, lenses and renames |
| `cargo xtask forces` | The force register is order-blind and opposed; its plants are refused |
| `cargo xtask contact` | Every `.contact` in the corpus is one truth with its wired twin |
| `cargo xtask roles <path>` | The roles of a contact body's cells: protect, carry, store, respond |
| `cargo xtask grove` | Grow the generated test universe (seed 7) and print its counts; never stored in the corpus |
| `cargo xtask layout --universe <path\|grove>` | Every chart of a universe's lens on the fixed grid |
| `cargo xtask zoom` | What the cut draws at each predicted view of the grove, and its touches (`--measure` times the cut) |
| `cargo xtask links [--universe <path\|grove> \| --forms \| --measure]` | Every link's route in every fold state: legs, stubs, spines, knots, and §2.6's crossings (none). `--forms`: the grove's link forms (region, hub, bundle, spine) at each view; `--measure`: routing's wall time per universe, least of 5 runs (information) |
| `cargo xtask pick` | The GPU's ID picture against the exact CPU pick, on every adapter, link owners included |
| `cargo xtask regrow` | Delta-built tables equal regrown ones, and a rebase moves no pixel |
| `cargo xtask grow [--measure]` | Grow counting and adding through every transcript: each size's count beside the counting witness, the lasso's rules and the unchanged hash; evolution holds; its plants are refused. `--measure`: µs per growth step at n = 0, 6, 24, 96 (information) |
| `cargo xtask perf` | Performance measurements |

Edition 2024, workspace resolver 3, `module.rs` layout (no `mod.rs`).

## For agents / builders

Standing engineering rules: **[AGENTS.md](AGENTS.md)**

Deep theory and phase plans live under [`../docs/Theory/`](../docs/Theory/) and [`../docs/Plans/`](../docs/Plans/). Prefer Guides when explaining the project to a human.
