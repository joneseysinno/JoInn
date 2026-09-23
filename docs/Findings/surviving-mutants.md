# Surviving mutants

The R23 specification, accumulating. When a mutant survives `cargo xtask power`, the pair (mutant, missing law) is written here **before** the fix.

Phase 1 target: 12/12 refused. This file stays as the log even after survivors are fixed.

## Mutant 14 — sealed allele as reference body

Opened P21-01: `v33_reference_not_sealed` compared two Rust type names and could
not fail, so `mutant.sealed_as_reference` was a survivor wearing a pass.

Closed P21-09, 2026-09-18: `check::v33` walks the reference genome. A purpose-built
body whose genome names `add@ℤ` is refused with `refused by check::v33: genome
names add@ℤ`. The negative control (a body that names only floor members `eq`
and `case`) is accepted.

## Blind seals — three reference alleles that compute nothing

Opened P22-01, 2026-09-18. Closed P22-05–P22-07 / P22-18, 2026-09-18:
the three hollow bodies moved to `corpus/phase22/counterfeit/` and the
agreement harness separates each at the declared drive bound. These are F9 of
`docs/Findings/phase-2.1-review.md`. The date closed is 2026-09-18.

The three seals hid behind `Drive { bound: 0 }`. A bound of zero pins the
driving port to a single value, so `agree` sampled a point, not a domain. The
review raised each bound to 4, seed 1 unchanged, and recorded the
counter-examples below. The hollow bodies that produced them are kept as the
counterfeits of Phase 2.2.

### `text.parse_int`

Opened 2026-09-18. Closed 2026-09-18. Drive port 0, bound 0: every sample is the literal `"0"`.
The body (`corpus/phase21/text_parse_ref.body`, later
`corpus/phase22/counterfeit/text_parse.body`) computes `eq(a, a)` and does not
parse.

Raised to bound 4, seed 1: `TRUTH VIOLATION` — expected `-31`, got `0`.

### `int.format`

Opened 2026-09-18. Closed 2026-09-18. Drive port 0, bound 0: every sample is `0`. The body
(`corpus/phase21/int_format_ref.body`, later
`corpus/phase22/counterfeit/int_format.body`) wires `prim:hash` into `build`'s
frame port.

Raised to bound 4, seed 1: `TRUTH VIOLATION` — expected `"-3"`, got `"0"`.

### `rat.add`

Opened 2026-09-18. Closed 2026-09-18. Drive port 1, bound 0: `b` is the literal `0/1`. The body
(`corpus/phase21/rat_add_ref.body`, later
`corpus/phase22/counterfeit/rat_add.body`) is one genome entry and one wire:
`add(a, b) = a`.

Raised to bound 4, seed 1: `TRUTH VIOLATION` — expected
`-65853234582920539119026175/7139822`, got `1/7139822`.
