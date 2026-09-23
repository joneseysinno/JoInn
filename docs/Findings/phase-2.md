# Phase 2 findings

Milestone 0 is `joinn run calculator`. The five-line transcript is a corpus
witness at `joinn/corpus/transcripts/calculator.txt`. `cargo xtask gate 2`
replays it, runs `agree`, checks the step trace, admits `sum_turn.cell` against
parent `6b32…`, and records R31.

- **R6** is closed in code: `text.parse_int` is a cell, sealed; `int.format` is
  its turn; `"007"` is the declared hole; `format ∘ parse` is one-way.
- **R31** is derived from the turn register (`int.add.turn0` and
  `add@ℤ.turn1`). See `turn-annotations.md`.
- **R32** is in the README: `hash` is total, `resolve` is partial, and the gap
  between them is the store.
- **Risk #3** (live engine too slow to build with) has not fired on step-count
  grounds. See `live-engine-performance.md`.
- **V38**: `joinn-live` and `joinn-prim` do not name a host, stdin, stdout, or
  the calculator. Grant order in engine tests uses the capability name `cli`.
- Gate power is 20/20: the original twelve mutants plus eight Phase 2 checks
  (wrapping beyond i64, sealed-as-reference, drop-under-Refuse, arrival-order,
  ungranted read, skipped require, turn-wrong-on-negatives, budget-as-membrane).
