# Counterfeit strength — R39

Opened: 19 September 2026. No date closed.

R39 asks where a counterfeit comes from, and whether a plausible wrong
sibling is enough. Phase 2.2 harvested three counterfeits from history and
invented two more as identity-forward bodies. `agree` separates all five.
The question is how much of the declared bound that separation actually
uses.

## Five separating samples (seed 1, 256 samples)

Printed by `cargo xtask agree` on the tree after P3-03.

| Seal | Separating sample | Drive |
|---|---|---|
| `add@ℤ` (carry-theft) | sample 9 (`-2147483616`) | port 1 bound 32, admits 65 |
| `parse@Text` | sample 1 (`-6`) | port 0 bound 8, admits 9 |
| `format@ℤ` | sample 0 (`"-31"`) | port 0 bound 32, admits 65 |
| `mul@ℤ` | sample 0 (`7`) | port 1 bound 8, admits 17 |
| `add@ℚ` | sample 0 (`49978755/7139822`) | port 1 bound 8, admits 17 |

The crude identity-forward body (`corpus/phase22/counterfeit/int_add.body`)
separated at sample 1 (`-2147483630`) on the same seed and bound; it is
kept in the corpus as the cruder sibling. Four of the other four seals
still separate at sample 0.

`add@ℤ`'s identity-forward body lasted one extra sample because seed 1,
drive port 1, bound 32 draws `b = 0` at sample 0, and identity agrees
with `a + 0`. Sample 1 was then the first nonzero `b`. That is not a
carry. It is the first time the ignored port is looked at. The carry body
agrees through sample 8 and separates at sample 9, which is the first
draw whose second addend is 32.

## Bound-1 experiment

Every `drive_bound` in `seal_register()` set to `1` — the narrowest the
type permits — and `agree` re-run (Phase 2.2 review, 18 September 2026):

```
BLIND SEAL format@ℤ: counterfeit body … agreed on all 256 samples
  drive port 0 bound 1 admits 3 values; widen it or the harness sees nothing
```

Four of five seals still separate their counterfeit at bound 1. Only
`format@ℤ` goes blind. For 80% of the register, the counterfeit places
essentially no constraint on the bound the author chose. The knob F9 was
about is still, for those four, the author's.

## The subtler `add@ℤ` counterfeit

`corpus/phase22/counterfeit/int_add_carry.body` is the Peano reference
adder with one theft: when the second addend equals 32, it forwards `a`
and drops the sum. 32 is the carry into bit 5 of an 6-bit magnitude, and
it is the first drive-port-1 sample (seed 1, bound 32) whose value is 32,
which is sample 9.

The crude identity-forward body remains at
`corpus/phase22/counterfeit/int_add.body`. The seal register points at
the carry body. Two ways of being wrong, both readable.

A counterfeit separated at sample 9 is evidence the bound is doing
something. It is not a measure of counterfeit quality. R39 stays open.

## R39

Still open. One subtler sibling is evidence, not a metric. The question
whether a seal wants several counterfeits — one per way of being wrong —
and whether the floor's pairing arithmetic has anything to say about how
many, is not answered here.
