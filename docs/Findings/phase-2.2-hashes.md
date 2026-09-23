# Phase 2.2 corpus hash moves

Date: 2026-09-18

These hashes moved because the artifacts were corrected on purpose. That is not
reblessing a golden to make a test pass. Each old value is the hash of the
hollow or false artifact; each new value is the hash of the corrected one.

## `mul.cell` (P22-11)

Old: `4a37…` (identity law `mul(a, 0) = a`, which is false)
New: `12b6e5458891b14aa74e43cee34b1184be04547fe58e035b1882125467120fa7`

The law is now `annihilator: mul(a, 0) = 0`. Recorded also in
`canonical-form-changes.md`.

## `text_parse_ref.body` (P22-05 / P22-18)

Old: `14e3e6a755f8852675fe33d793ded22555958ac5a6c657e17a5dd5a589d3b74d`
(hollow: `eq(a, a)` forwarded, does not parse)

New: `b7dd6f306c220c94b8b812ae900ee89ee666b0c3b13b44fee2cd390a34dc5120`

The Phase 2.2 body walks `Text` by `empty`/`cons`/`chr`, accumulates
`mul(acc, 10)+digit`, and refuses a non-digit. `"007"` remains the declared
hole.

## `int_format_ref.body` (P22-06 / P22-18)

Old: `f9d6dbb5c870cea2475d3d21cbaa5bc9e57515cc75219c276bbf50662c9a3453`
(hollow: `prim:hash` as a frame witness)

New: `67c1caa7b976a89f515d94c9eff68cb07f883001a0da918a8de8fce791da9032`

No `prim:hash`. Division by ten is repeated `Sum` at a declared turn. Text is
built by `build` with an ℤ-tagged pair whose left part is `1` as the Text
frame selector.

## `rat_add_ref.body` (P22-07 / P22-18)

Old: `26d758a0a7d9d0c695bea1183c41f368d3d6e30f4f0bcb383f239fc4ea896572`
(hollow: `add(a, b) = a`)

New: `6385aba04808778623f5ef2bb08f92817247fd1539537191cc1b3b225f60e136`

Cross-multiply over `int.mul` and `int.add`; `build` on ℚ performs the
reduction. No gcd body.

## `int_mul_ref.body` (P22-12 / P22-18)

Old: `8c092f3cfb4f64022685cd1d59726f65acb6c152d22a3bf40e0332cc4d4d3809`

New: `2327f43c84175eee18aeb3125dcfd72a21396c3f6e77f3003c4f8ada308ee0bc`

The genome named the Sum cell for multiplication (F18). It now names
`mul.cell` at `12b6e545…`. The body is otherwise the honest Phase 2.1
reference.
