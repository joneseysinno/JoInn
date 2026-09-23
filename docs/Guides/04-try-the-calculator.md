# Try the calculator

This walkthrough runs JoInn’s small demo: a calculator that asks for two integers, refuses bad input, then prints `2 + 3 = 5`.

You need a computer with **Rust** installed (`cargo` available in a terminal). If that isn’t set up yet, ask someone technical to install the Rust toolchain first, then come back here.

## 1. Open a terminal in the code folder

The code lives in the `joinn` folder next to `docs`.

**Windows (PowerShell example):**

```powershell
cd d:\JoInn\joinn
```

Use your real path if the project lives somewhere else.

## 2. Run the calculator

```powershell
cargo run -p joinn-cli -- run calculator
```

Or, if the `joinn` command is already on your path from a prior install:

```text
joinn run calculator
```

## 3. What good output looks like

You should see something like this (order and wording may match the golden transcript):

```text
a: two
   refused at membrane: "two" is not in ℤ
a: 2
b: 3
2 + 3 = 5
```

What that means:

| Line | Meaning |
|------|---------|
| `a: two` | First prompt got the word “two” |
| `refused…` | The system **refused** — “two” is not an integer in the number frame (ℤ means integers) |
| `a: 2` / `b: 3` | Honest integers were accepted |
| `2 + 3 = 5` | The sum cell did its job |

A **refusal is success** for this demo. It proves the membrane (boundary) won’t pretend words are numbers.

The saved expected transcript lives at `joinn/corpus/transcripts/calculator.txt`.

## 4. Optional: proof commands (you don’t need these daily)

From the same `joinn` folder, builders often run checks like:

| Command | Plain meaning |
|---------|----------------|
| `cargo test --workspace` | Run the automated tests |
| `cargo xtask vocab` | Check naming / vocabulary rules |
| `cargo xtask floor` | Check the sealed foundation (“the floor”) |
| `cargo xtask agree` | Check that sealed pairs really oppose / agree as claimed |
| `cargo xtask gate all` | Run the gate’s proof suite |
| `cargo xtask corpus verify` | Confirm golden corpus files still match |
| `cargo xtask power` | Power / strength checks for the current freeze |

If one fails, don’t “fix” hashes or delete tests to make it green. That’s a standing rule of the project: report the failure.

## 5. If something goes wrong

- **`cargo` not found** — Rust isn’t installed or the terminal doesn’t see it.  
- **Wrong folder** — you must be inside `joinn` (the folder with the workspace `Cargo.toml`).  
- **Usage message** — the CLI only knows `joinn run calculator` right now.

## Next

- Word list: [05-glossary.md](05-glossary.md)  
- How this run fits the stack: [02-how-parts-connect.md](02-how-parts-connect.md)  
- Code README: [../../joinn/README.md](../../joinn/README.md)
