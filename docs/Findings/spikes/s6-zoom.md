# S6 · Deep-zoom precision spike

**Can kill Part II §9.**

## What was built

A 2D toy in `joinn/spikes/s6-zoom/` with no renderer: a chart chain, a camera anchored to the deepest chart, an integer zoom level plus a fraction, and rebasing when the fraction crosses 1.

Twelve orders of magnitude: zoom from scale 1 to scale 10¹² on a point lattice.

## Result

- **Jitter first appears** when a world coordinate and a camera origin that differ by ~10⁷ are subtracted in IEEE-754 binary64 and then multiplied by a large scale. That is around zoom level 7 if the camera is kept at world origin. With camera-relative coordinates, the same subtraction stays well-conditioned.
- **Rebasing becomes necessary** at each crossing of an integer zoom level, i.e. whenever the fraction would leave `[0, 1)`. Without rebase, the scale factor itself overflows the pattern Part II wants (integer level + fraction). With rebase, twelve orders stay exact in the toy because the stored origin is integer and the fraction is in `[0, 1)`.

## Kill criterion

Not fired. The chart chain holds in isolation. The finding to carry into Phase 7: never subtract two large world-space f64s; rebase the camera origin onto the deepest chart's integer lattice; keep the uploaded float as camera-relative.

Phase 1 forbids `f32`/`f64` in `crates/`. This spike is excluded from the workspace and is allowed to use f64, because the finding is about the host's camera math, not about the truth core.
