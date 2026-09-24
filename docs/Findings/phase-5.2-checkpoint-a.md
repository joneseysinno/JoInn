# Phase 5.2 Checkpoint A — end of Sitting A (P52-00 to P52-02)

> **SKELETON — not yet a checkpoint.** Every `___` below is typed by AJ from
> his own terminal (plan §2.9, rules 38 and 46). Delete this box once every
> blank is filled. Until this file is **committed**, `cargo xtask witness`
> refuses it: `witness: … is not in git; acceptance is a committed finding`.

Date: ___ September 2026. Windows, PowerShell. Commands run from `D:\JoInn\joinn`
on the tree as P52-02 left it. AJ ran the commands and typed or pasted what they
printed. Nothing below is paraphrased from a plan or a review.

Git HEAD when the commands were run: `___` (`git log -1 --format="%h %ci %s"`)

## Files changed outside `target/`

- `gates.lock`, rewritten by `cargo xtask gate all` (expected).
- Anything else (`git status --short`): ___

## Last line of each command

- `cargo test --workspace --no-fail-fast` — `___`
- `cargo xtask gate all` — `___` (exit code ___)
- `cargo xtask vocab` — `___` (exit code ___)

## `cargo xtask gate all` — the gate rows, as printed

```
___
```

Any line printed with no item label (in 5.1's run: `differing field: value`): ___

The lock `gate all` wrote (`type gates.lock`):

```
___
```

## Every `fail`

Every test that failed, every gate row containing `fail`, every harness refusal.
If none, write *none*.

| Command | Item or test | What it printed |
|---|---|---|
| ___ | ___ | ___ |

## Sitting A's done-whens, checked

| Commit | Done-when | Printed | Met? |
|---|---|---|---|
| P52-00 | `phase-5.1-run.md` exists and quotes what was printed | (file exists, typed 24 Sep) | ___ |
| P52-01 | `cargo test --workspace --no-fail-fast` passes | ___ | ___ |
| P52-01 | `cargo xtask vocab` reports no hits | ___ | ___ |
| P52-01 | commit message lists every changed assertion, old and new, none removed | ___ | ___ |
| P52-02 | `cargo xtask witness docs/Findings/phase-3-run.md` prints `stale:` naming a newer file, exits non-zero | ___ | ___ |

## How §2.11 (rule 13 vs `saturating_sub`) was resolved

Option chosen, (a), (b) or (c): ___ · R63 recorded in `decisions.md`: yes

## Carried into Sitting B

- The six extra failing tests `phase-5.1-run.md` found (assemble ×2, law4,
  universe ×3, test-host `units`): how each was resolved, and whether any
  expectation changed (rule 7): ___
- Gates 2, 2.1, 2.2 at `0/0`: expected to stay until P52-03 to P52-08. Still `0/0`? ___
- Gate 5·8 / 5.1·7 *A refusal stays home*: expected to fail until P52-13. Still failing? ___

## After committing this file

`cargo xtask witness docs/Findings/phase-5.2-checkpoint-a.md` → `___`
(must print `current` before P52-03 starts)
