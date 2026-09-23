# S3 · Underdetermination (paper)

**Can kill D2/D3's usefulness.** AJ's spike; Cursor's job is the finding and the mutant-corpus consequence.

## `sort`

A coding region that a gate can usefully judge cannot name a sorting algorithm (that would be an allele) and cannot name `int.cmp` as a primitive.

What it *can* say, with Phase 1's formula language:

- **permutation of length:** `forall xs. length(self(xs)) = length(xs)` — needs a length frame-op, which a Seq-like frame does not currently expose beyond `Text` / `ℤ` / `ℚ`.
- **ordered:** `forall i. self(xs)[i] ≤ self(xs)[i+1]` — needs indexing and `≤`, neither of which is in ℤ's minimal signature `{zero, succ, pred, eq}`.
- **bag equality with the input:** not an equality of canonical values unless the frame quotients by permutation, which would make the output equal to the input and the cell the identity.

So a Phase 1 coding region for `sort` either:

1. restates an algorithm (illegal: primitive names, or `self` pinned to a frame op — degeneracy), or
2. states nothing the gate can sample (`forall xs. self(xs) = self(xs)`).

**Finding.** D2/D3 are useful for *algebraic* cells (Sum, and CliInput's round-trip). They are not, by themselves, useful for `sort`. The gate's power on `sort` would be entirely a function of laws nobody who cannot program can write. That is Risk #1, confirmed on paper, before any mutant ran.

No `sort` entry is added to the Phase 1 mutant corpus: there is no coding region to mutate against that would refuse a wrong sort.

## `format`

`format: ℤ → Text` is the other face of CliInput. The law that holds is `parse(format n) = n`. The law that does not is `format(parse s) = s` (`"007"`).

A coding region for `format` that states `parse(format n) = n` is checkable, and is exactly CliInput's law written from the other end, naming CliInput by hash. That is useful. It does not determine *which* string is the format of `7` (`"7"` vs `"007"` vs `"7.0"`). Founding witnesses pin the chosen spelling. Without them, any injection ℤ ↪ Text survives.

**Finding.** `format` is the good case of S3: a round-trip law plus founding witnesses is enough for a gate to refuse a padded or truncated printer. `sort` is the bad case. D2/D3's scope is "cells whose meaning is algebraic identity, not search". Write that down before Phase 2, not after.
