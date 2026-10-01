# JoInn

This folder is the **running code** for JoInn: libraries (crates), the calculator demo, golden examples (`corpus/`), and build checks (`xtask`).

JoInn is a **truth-first** platform for composing apps from small blocks called **cells**. The terminal calculator still refuses bad input and computes `2 + 3`. A desktop window now draws that one body: the picture is the tables, and every pixel names its owner. It does not draw text, a second body, or a zoom.

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

## Gates

`cargo xtask gate all` runs the proof in order: phase 0 (the corpus verifies), gate 1 (admission), gate 2 (the calculator, opposed), gate 2.1 (seals), gate 2.2 (a reference allele is a body), gate 3 (two hosts, one body), gate 4 (the assay reads a known sample, a promise names its partner, a body that is two things is named, and a declaration refuses where an assay only reports), gate 5 (a typed link between two bodies), gate 5.1 (a value crosses), gate 5.2 (a control sees a parsed value and flips on a mutant that still parses), gate 6 (one body drawn from its tables: every pixel has one owner and both pickers name it, the picture shows the row the engine computed, and a stale click is refused), and gate 7 (the contact calculator and the wired one are one truth, combine is order-blind and fits what it reaches, and the contact body is drawn as cells in contact with no link). Gate 4 was built after gate 5.2 and runs in its phase-number place. The lock records the score each gate returned. This file does not quote one.

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
| `cargo xtask perf` | Performance measurements |

Edition 2024, workspace resolver 3, `module.rs` layout (no `mod.rs`).

## For agents / builders

Standing engineering rules: **[AGENTS.md](AGENTS.md)**

Deep theory and phase plans live under [`../docs/Theory/`](../docs/Theory/) and [`../docs/Plans/`](../docs/Plans/). Prefer Guides when explaining the project to a human.
