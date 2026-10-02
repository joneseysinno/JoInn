# Phase 7.1 stage 1 — the beam computes: findings

Recorded by Cursor on 2 Oct 2026 from `D:\JoInn` on Windows (PowerShell), on
`a8fb311` (P71-09) and the stop-report commit after it. Every output below is
copied from the terminal. Spike S8 lives in `joinn/spikes/s8-beam`, its own
workspace, depending on nothing in `crates/`. It works five W12x26 / A992
beams exactly in ℚ, in the elevation plane, with formulas derived and AISC
Table 3-23 and Roark Table 8.1 as witnesses. Every line of §2.8 and of §2.1's
seven `forces` lines came out **as predicted**: nothing differs.

## The spike's output

`cargo run --manifest-path spikes/s8-beam/Cargo.toml`, stdout captured as bytes
(3355 bytes, SHA-256 `8C60B6D8116F9AFA0AD30D992AC134544623400E072781CC4997E42D8C36F578`,
the same as `spikes/s8-beam/expected.txt`); exit 0:

```
s8 beam · exact in ℚ · kip and inch inside, feet shown
bridges: A992 E 29000 ksi · W12x26 Ix 204 in⁴ (pinned: AISC Manual, 16th ed.)
witnesses: AISC Manual Table 3-23 · Roark Table 8.1 (stated, compared, never used to derive)
E1 simple span, uniform load · L 24 ft · w 6/5 kip/ft down
  complex: 3 points, 2 lines (elevation plane)
  balance: R_A 72/5 kip up, R_B 72/5 kip up · ΣF 0 · ΣM 0 · M at end 0 · no bridge read
  M(12 ft) 432/5 kip·ft sagging (86.4)
  v(12 ft) 93312/61625 in down (1.5142~)
  witness wL²/8 = 432/5 kip·ft: agree
  witness 5wL⁴/384EI = 93312/61625 in: agree
  refined to 24 lines: equal at every point of the 3
E2 simple span, point load at midspan · L 24 ft · P 10 kip down
  complex: 3 points, 2 lines (elevation plane)
  balance: R_A 5 kip up, R_B 5 kip up · ΣF 0 · ΣM 0 · M at end 0 · no bridge read
  M(12 ft) 60 kip·ft sagging
  v(12 ft) 10368/12325 in down (0.8412~)
  witness PL/4 = 60 kip·ft: agree
  witness PL³/48EI = 10368/12325 in: agree
  refined to 24 lines: equal at every point of the 3
E3 simple span, point load at 6 ft · L 24 ft · P 10 kip down
  complex: 3 points, 2 lines (elevation plane)
  balance: R_A 15/2 kip up, R_B 5/2 kip up · ΣF 0 · ΣM 0 · M at end 0 · no bridge read
  M(6 ft) 45 kip·ft sagging
  v(6 ft) 5832/12325 in down (0.4732~)
  witness Pab/L = 45 kip·ft: agree
  witness Pa²b²/3EIL = 5832/12325 in: agree
  refined to 24 lines: equal at every point of the 3
E4 cantilever, point load at the tip · L 10 ft · P 5 kip down
  complex: 2 points, 1 line (elevation plane)
  balance: R_A 5 kip up, M_A 50 kip·ft hogging · ΣF 0 · ΣM 0 · M at end 0 · no bridge read
  M(0 ft) 50 kip·ft hogging
  v(10 ft) 240/493 in down (0.4868~)
  witness PL = 50 kip·ft: agree
  witness PL³/3EI = 240/493 in: agree
  refined to 10 lines: equal at every point of the 2
E5 simple span, uniform load and point load at 6 ft · L 24 ft · w 6/5 kip/ft · P 10 kip
  complex: 4 points, 3 lines (elevation plane)
  balance: R_A 219/10 kip up, R_B 169/10 kip up · ΣF 0 · ΣM 0 · M at end 0 · no bridge read
  M(6 ft) 549/5 kip·ft sagging (109.8)
  M(12 ft) 582/5 kip·ft sagging (116.4)
  v(6 ft) 478224/308125 in down (1.5520~)
  v(12 ft) 128952/61625 in down (2.0925~)
  witness E1 + E3 at 6 ft = 549/5 kip·ft: agree
  witness E1 + E3 at 12 ft = 582/5 kip·ft: agree
  witness E1 + E3 at 6 ft = 478224/308125 in: agree
  witness E1 + E3 at 12 ft = 128952/61625 in: agree
  refined to 24 lines: equal at every point of the 4
plant mixed order: refused (ok): balance: E1 M at end 1728/5 kip·ft, want 0; acceptance is one order for every moment
plant rectangle rule: refused (ok): refinement: E1 M(12 ft) 864/5 kip·ft with 2 lines, 468/5 kip·ft with 24 lines; acceptance is a derivation the complex cannot change
plant witness wL²/12: refused (ok): E1 witness wL²/12 = 288/5 kip·ft, derived 432/5 kip·ft; a disagreement is a truth violation
plant moment + work: refused (ok): add: source · length 1 · plane and energy · length 1 · none differ, though both are kip·in; acceptance is two quantities on one piece, side and pair
plant no bridge: refused (ok): v(12 ft) needs the bridge E·I; acceptance is a pinned edition. balance and M read no bridge
s8: 5 example(s), 12 witness(es) agree, 0 disagree; refinement equal in 5; plants: 5 refused (ok)
```

`cargo test --manifest-path spikes/s8-beam/Cargo.toml` on `a8fb311`:
`test result: ok. 54 passed; 0 failed`. CI runs it as step `spike s8` on both
jobs; it passed on ubuntu-24.04 and windows-latest in run
[37037569216](https://github.com/joneseysinno/JoInn/actions/runs/37037569216).

## `cargo xtask forces` after the order-blind rename

Stdout captured as bytes: 1159 bytes, equal to §2.1's block.

```
combine ℤ 1 by cell:6b32…: order-blind (64 pairs, 64 triples, seed 7), opposed by separate cell:6fcb… (turn 0 from {1 2})
planted: order_blind on mutant.difference: refused (ok): not order-blind: f(a, b) = 9223372039002259455 but f(b, a) = -9223372039002259455 at a = 9223372036854775807, b = -2147483648; acceptance is a response whose result does not depend on member order
planted: order_blind on mutant.midpoint: refused (ok): not order-blind: f(f(a, b), c) = -1535576763092620387 but f(a, f(b, c)) = -3841419772306314346 at a = -9223372036854775837, b = 3081064984484294291, c = 0; acceptance is a response whose result does not depend on member order
planted: order_blind on mutant.max: order-blind (ok), not registered
planted: order_blind on mutant.plus1: order-blind (ok), not registered
planted: check_register with separate = response: refused (ok): combine on ℤ 1 is unopposed: no separate; acceptance is a turn of cell:6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39
forces: 1 combine registered, 1 opposed; plants: difference refused, midpoint refused, max and plus1 order-blind but unregistered, unopposed refused (ok)
```

## Predictions

Each predicted line was compared with the printed line, case-sensitive, from
the captured bytes. `as predicted` means the two strings are equal; a line
that differs would show both.

### §2.1 — the seven `forces` lines

| # | Predicted and printed | Mark |
|---|---|---|
| 1 | `combine ℤ 1 by cell:6b32…: order-blind (64 pairs, 64 triples, seed 7), opposed by separate cell:6fcb… (turn 0 from {1 2})` | as predicted |
| 2 | `planted: order_blind on mutant.difference: refused (ok): not order-blind: f(a, b) = 9223372039002259455 but f(b, a) = -9223372039002259455 at a = 9223372036854775807, b = -2147483648; acceptance is a response whose result does not depend on member order` | as predicted |
| 3 | `planted: order_blind on mutant.midpoint: refused (ok): not order-blind: f(f(a, b), c) = -1535576763092620387 but f(a, f(b, c)) = -3841419772306314346 at a = -9223372036854775837, b = 3081064984484294291, c = 0; acceptance is a response whose result does not depend on member order` | as predicted |
| 4 | `planted: order_blind on mutant.max: order-blind (ok), not registered` | as predicted |
| 5 | `planted: order_blind on mutant.plus1: order-blind (ok), not registered` | as predicted |
| 6 | `planted: check_register with separate = response: refused (ok): combine on ℤ 1 is unopposed: no separate; acceptance is a turn of cell:6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39` | as predicted |
| 7 | `forces: 1 combine registered, 1 opposed; plants: difference refused, midpoint refused, max and plus1 order-blind but unregistered, unopposed refused (ok)` | as predicted |

### §2.8 — the spike's block

| # | Predicted and printed | Mark |
|---|---|---|
| 1 | `s8 beam · exact in ℚ · kip and inch inside, feet shown` | as predicted |
| 2 | `bridges: A992 E 29000 ksi · W12x26 Ix 204 in⁴ (pinned: AISC Manual, 16th ed.)` | as predicted |
| 3 | `witnesses: AISC Manual Table 3-23 · Roark Table 8.1 (stated, compared, never used to derive)` | as predicted |
| 4 | `E1 simple span, uniform load · L 24 ft · w 6/5 kip/ft down` | as predicted |
| 5 | `  complex: 3 points, 2 lines (elevation plane)` | as predicted |
| 6 | `  balance: R_A 72/5 kip up, R_B 72/5 kip up · ΣF 0 · ΣM 0 · M at end 0 · no bridge read` | as predicted |
| 7 | `  M(12 ft) 432/5 kip·ft sagging (86.4)` | as predicted |
| 8 | `  v(12 ft) 93312/61625 in down (1.5142~)` | as predicted |
| 9 | `  witness wL²/8 = 432/5 kip·ft: agree` | as predicted |
| 10 | `  witness 5wL⁴/384EI = 93312/61625 in: agree` | as predicted |
| 11 | `  refined to 24 lines: equal at every point of the 3` | as predicted |
| 12 | `E2 simple span, point load at midspan · L 24 ft · P 10 kip down` | as predicted |
| 13 | `  complex: 3 points, 2 lines (elevation plane)` | as predicted |
| 14 | `  balance: R_A 5 kip up, R_B 5 kip up · ΣF 0 · ΣM 0 · M at end 0 · no bridge read` | as predicted |
| 15 | `  M(12 ft) 60 kip·ft sagging` | as predicted |
| 16 | `  v(12 ft) 10368/12325 in down (0.8412~)` | as predicted |
| 17 | `  witness PL/4 = 60 kip·ft: agree` | as predicted |
| 18 | `  witness PL³/48EI = 10368/12325 in: agree` | as predicted |
| 19 | `  refined to 24 lines: equal at every point of the 3` | as predicted |
| 20 | `E3 simple span, point load at 6 ft · L 24 ft · P 10 kip down` | as predicted |
| 21 | `  complex: 3 points, 2 lines (elevation plane)` | as predicted |
| 22 | `  balance: R_A 15/2 kip up, R_B 5/2 kip up · ΣF 0 · ΣM 0 · M at end 0 · no bridge read` | as predicted |
| 23 | `  M(6 ft) 45 kip·ft sagging` | as predicted |
| 24 | `  v(6 ft) 5832/12325 in down (0.4732~)` | as predicted |
| 25 | `  witness Pab/L = 45 kip·ft: agree` | as predicted |
| 26 | `  witness Pa²b²/3EIL = 5832/12325 in: agree` | as predicted |
| 27 | `  refined to 24 lines: equal at every point of the 3` | as predicted |
| 28 | `E4 cantilever, point load at the tip · L 10 ft · P 5 kip down` | as predicted |
| 29 | `  complex: 2 points, 1 line (elevation plane)` | as predicted |
| 30 | `  balance: R_A 5 kip up, M_A 50 kip·ft hogging · ΣF 0 · ΣM 0 · M at end 0 · no bridge read` | as predicted |
| 31 | `  M(0 ft) 50 kip·ft hogging` | as predicted |
| 32 | `  v(10 ft) 240/493 in down (0.4868~)` | as predicted |
| 33 | `  witness PL = 50 kip·ft: agree` | as predicted |
| 34 | `  witness PL³/3EI = 240/493 in: agree` | as predicted |
| 35 | `  refined to 10 lines: equal at every point of the 2` | as predicted |
| 36 | `E5 simple span, uniform load and point load at 6 ft · L 24 ft · w 6/5 kip/ft · P 10 kip` | as predicted |
| 37 | `  complex: 4 points, 3 lines (elevation plane)` | as predicted |
| 38 | `  balance: R_A 219/10 kip up, R_B 169/10 kip up · ΣF 0 · ΣM 0 · M at end 0 · no bridge read` | as predicted |
| 39 | `  M(6 ft) 549/5 kip·ft sagging (109.8)` | as predicted |
| 40 | `  M(12 ft) 582/5 kip·ft sagging (116.4)` | as predicted |
| 41 | `  v(6 ft) 478224/308125 in down (1.5520~)` | as predicted |
| 42 | `  v(12 ft) 128952/61625 in down (2.0925~)` | as predicted |
| 43 | `  witness E1 + E3 at 6 ft = 549/5 kip·ft: agree` | as predicted |
| 44 | `  witness E1 + E3 at 12 ft = 582/5 kip·ft: agree` | as predicted |
| 45 | `  witness E1 + E3 at 6 ft = 478224/308125 in: agree` | as predicted |
| 46 | `  witness E1 + E3 at 12 ft = 128952/61625 in: agree` | as predicted |
| 47 | `  refined to 24 lines: equal at every point of the 4` | as predicted |
| 48 | `plant mixed order: refused (ok): balance: E1 M at end 1728/5 kip·ft, want 0; acceptance is one order for every moment` | as predicted |
| 49 | `plant rectangle rule: refused (ok): refinement: E1 M(12 ft) 864/5 kip·ft with 2 lines, 468/5 kip·ft with 24 lines; acceptance is a derivation the complex cannot change` | as predicted |
| 50 | `plant witness wL²/12: refused (ok): E1 witness wL²/12 = 288/5 kip·ft, derived 432/5 kip·ft; a disagreement is a truth violation` | as predicted |
| 51 | `plant moment + work: refused (ok): add: source · length 1 · plane and energy · length 1 · none differ, though both are kip·in; acceptance is two quantities on one piece, side and pair` | as predicted |
| 52 | `plant no bridge: refused (ok): v(12 ft) needs the bridge E·I; acceptance is a pinned edition. balance and M read no bridge` | as predicted |
| 53 | `s8: 5 example(s), 12 witness(es) agree, 0 disagree; refinement equal in 5; plants: 5 refused (ok)` | as predicted |

Totals: 7 of 7 `forces` lines and 53 of 53 spike lines `as predicted`; 0 differ.

## Part VI §11: the three answers

Part VI §11's adversary says the tag is "just units with extra words". It
names three things unit exponents cannot do. The spike was built so that each
one is a plant that must be refused.

**1. Moment versus work told apart** (the plan's stand-in for stress versus
pressure: in the elevation plane the same challenge is a quantity on a plane
versus one with no orientation, both kip·in). E1's moment at mid-span and E2's
work, the load dotted with the deflection there, have the same unit. `add`
refuses them on their tags alone:

```
plant moment + work: refused (ok): add: source · length 1 · plane and energy · length 1 · none differ, though both are kip·in; acceptance is two quantities on one piece, side and pair
```

Both units print `kip·in`, which is all a list of exponents could compare. The
tags differ in side (source against energy) and in axis (plane against none),
and that difference is what refused the sum.

**2. Truth separated from testimony.** Balance and the moment walk take no
bridge at all, and the one bridge each example holds counted its reads after
them. Every example prints:

```
  balance: R_A 72/5 kip up, R_B 72/5 kip up · ΣF 0 · ΣM 0 · M at end 0 · no bridge read
```

(E1's line; the other four end `no bridge read` too). The test
`balance_and_m_read_no_bridge_and_v_reads_two_per_line` asserts the counts as
(reads after balance and M, reads after v) = `[(0, 4), (0, 4), (0, 4), (0, 2), (0, 6)]`:
none before deflection, two per line during it. With the edition absent, the
reactions and moments still come out and only v is refused:

```
plant no bridge: refused (ok): v(12 ft) needs the bridge E·I; acceptance is a pinned edition. balance and M read no bridge
```

**3. A sign that depends on order caught.** `wedge` is order-signed: x ∧ y is
+a·b and y ∧ x is −a·b. When the walk takes the load's moment as F ∧ r while
the reactions stay r ∧ F, M no longer closes at the far support:

```
plant mixed order: refused (ok): balance: E1 M at end 1728/5 kip·ft, want 0; acceptance is one order for every moment
```

The faithful walk ends at exactly 0 in all five examples (`M at end 0`).

### Are the tags decoration? (risk §7)

- **The moment + work plant cannot be caught by units.** Both units print
  `kip·in`; only the tag refused it.
- **No step of the faithful derivation was ever refused by a tag.** Every
  `add`, `wedge`, `dot`, `total` and `bridge` in the five examples was
  admitted; the tags refuse only in the plants and in the unit tests. In this
  stage the tags' work is to refuse wrong sums and to supply the sign of
  `wedge`. They never changed a correct number.
- **Only order-signed `wedge` exposes mixed order.** It is the tag's axes that
  give `wedge` its sign. A plain commutative multiply computes F × r and r × F
  as the same number, so the mixed-order mistake would neither be caught nor
  even be expressible.

## What stage 2 needs

What the spike had to invent that JoInn's grammar does not have yet, one line
each, as observed while building it:

- **A division, as a count.** Balance's exact solve needed `ratio(a, b)`: a plain number from two quantities of one tag (P71-06 snag). §2.3 has no division.
- **An unknown as a count times a unit.** Each solve wrote the unknown as `scale(unit of its tag, count)`: R_B with a unit force, θ at A with a unit rotation.
- **A contraction.** Turning a rotation θ across a line into a deflection v needed `contract(r, a)`, x ⌋ (x ∧ y) = y (P71-08 snag). It is order-signed, so it is not `dot`.
- **A zero of every tag.** Every sum started from a zero of its own tag (force, moment, rotation, deflection); there is no untagged zero.
- **Curvature's tag.** κ = M / EI is placement · length −1 · plane, a tag §2.3's table does not list (§2.5 names it).
- **`total` only over a density.** The spike refuses `total` of a quantity whose length is not negative. §2.3 does not say what `total` of a force means.
- **E and I only as one product.** `bridge` reads E·I together, never E or I alone. The edition holds them as two lines.
- **Witnesses read the edition outside the count.** The witness formulas need E·I and take it from the edition directly, not through the counted `bridge`; only derivation is counted.
- **A boundary condition.** The simple span needed one solve against a support (v = 0 at B) to fix θ at A. The cantilever needed none (θ = v = 0 at the wall).
- **Sections must be points.** A printed section had to be a point of the complex; the complex is derived from the ends, the loads and the printed sections.
- **Sense words from sign.** `up`/`down` and `sagging`/`hogging` print from the sign of a value, and a zero prints no word. Up and sagging are positive inside.
- **Units only at input and printing.** Feet enter as `scale` by 12 and kip/ft as `scale` by 1/12; kip·in prints as kip·ft by `scale` 1/12. No tag carries a unit.
- **A decimal rule.** At most 4 places, trailing zeros dropped when exact, else rounded half up with `~`: presentation chosen by the plan (R100).
- **A plant needs a switch inside the derivation.** The mixed-order and rectangle plants are `Order` variants passed to the walk; the derivation itself had to admit the mistake to be shown it.
