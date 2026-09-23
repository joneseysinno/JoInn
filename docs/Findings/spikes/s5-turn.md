# S5 · Turn spike

**Can kill `Turn`; gates Phase 2.**

## What was built

A tiny propagator over the `Sum` relation `{a, b, sum}` with law `a + b = sum`, in `joinn/spikes/s5-turn/`.

Supply any two, get the third: addition and both subtractions fall out of one relation.

Then the spike is pushed:

1. Three-port law on one cell — no extra annotation.
2. Two `Sum` cells chained (`(a+b)+c`) — three ports unknown if only `a` and the total are given; the middle value is unconstrained without associativity as a propagator.
3. Two unknowns on a single `Sum` — infinitely many solutions; the propagator correctly refuses to pick one.

## Annotation count

Direction had to be annotated by hand in **2 of 7** tried configurations:

| # | Given | Unknown | Hand annotation? |
|---|---|---|---|
| 1 | a, b | sum | no |
| 2 | a, sum | b | no |
| 3 | b, sum | a | no |
| 4 | a, b, c and (a+b)+c | middles | no (once chained as two cells with the middle named) |
| 5 | a, total of (a+b)+c | b, c, middles | **yes** — underdetermined; a direction (or a second given) must be supplied |
| 6 | all three of one Sum | — | no (already solved) |
| 7 | one port of one Sum | other two | **yes** — underdetermined |

Near-universal annotation would have killed `Turn`. Two annotations, both on genuinely underdetermined systems, is the propagator doing its job.

## Kill criterion

Not fired. `sub` stays as `turn(Sum, solve for a)`. Phase 2 may implement Turn. Chained cells with two unknowns still need a second given, which is mathematics, not a costume.
