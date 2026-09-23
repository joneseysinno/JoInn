# Minimal primitive set (frozen on paper)

Phase 1 does not implement these. Native Rust alleles stand in. Each primitive gets one paragraph and one example so Phase 2 starts from a freeze, not a debate.

## `eq`

The root of Law 1. Two values in the same frame are either the same canonical value or they are not. There is no third state and no tolerance. Example: `eq(ℤ 2, ℤ 2)` holds; `eq(ℤ 2, ℤ 3)` does not.

## `zero` / `succ` / `pred`

The generator and opposition of the ℤ frame (and any frame that extends it). `zero` is the identity witness for addition; `succ` and `pred` are inverses on the whole frame. Example: `pred(succ(zero)) = zero`.

## `pair` / `split`

The only product. `pair` takes two values and makes one; `split` is its opposite and recovers both. Example: `split(pair(2, 3)) = (2, 3)`.

## `choose`

A two-way case on an equality test. It is how `verdict` will be expressed, not a separate control-flow primitive. Example: `choose(eq(x, zero), a, b)` is `a` when `x` is zero.

## `bound` / `fill`

`bound` declares a membrane — inside versus outside, with ports. `fill` is its opposite: occupying a declared hole. Example: a cell is a bound with its ports as the boundary.

## `bind` / `unbind`

Incidence. `bind` attaches a port to a port; `unbind` severs it. Example: wiring `cli_a.value` to `sum.a`.

## `hash`

Content address. One-way by construction; the hole is declared, not filled. Example: `hash(coding region of Sum)` is the cell's identity.

## `grant` / `revoke`

Capability movement. Whoever holds the grant may use the port; `revoke` is the opposite. Example: stdin passed from `cli_a` to `cli_b`.

## `join` / `fan`

`join` fires when required in-ports hold messages; `fan` is the opposite split of one message to many. Example: `Sum` joins on `a` and `b`.
