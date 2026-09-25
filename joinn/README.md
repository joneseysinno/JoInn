# JoInn

This folder is the **running code** for JoInn: libraries (crates), the calculator demo, golden examples (`corpus/`), and build checks (`xtask`).

JoInn is a **truth-first** platform for composing apps from small blocks called **cells**. Today you can run a terminal calculator that refuses bad input and computes `2 + 3`. A full visual editor comes later.

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

## Gates

`cargo xtask gate all` runs the proof in order: phase 0 (the corpus verifies), gate 1 (admission), gate 2 (the calculator, opposed), gate 2.1 (seals), gate 2.2 (a reference allele is a body), gate 3 (two hosts, one body), gate 5 (a typed link between two bodies), gate 5.1 (a value crosses), and gate 5.2 (a control sees a parsed value and flips on a mutant that still parses). There is no gate 4; that phase inserts later. The lock records the score each gate returned. This file does not quote one.

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
| `cargo xtask perf` | Performance measurements |

Edition 2024, workspace resolver 3, `module.rs` layout (no `mod.rs`).

## For agents / builders

Standing engineering rules: **[AGENTS.md](AGENTS.md)**

Deep theory and phase plans live under [`../docs/Theory/`](../docs/Theory/) and [`../docs/Plans/`](../docs/Plans/). Prefer Guides when explaining the project to a human.
