# R50 · What does ∂ cost at scale?

Date: 22 September 2026. Measured by `cargo xtask perf` from `joinn/`.

`membrane()` over the corpus printed:

```
membrane over corpus bodies: 23 ports: 18 milliseconds: 28
```

Twenty-eight milliseconds for twenty-three bodies is more than a few milliseconds. It is not yet a reason to cache ∂ in a `.universe` file — that remains the phase's failure mode — but it is the number R50 asked for. At Phase 7's cut the same question returns under R10 and R48.

The two-body universe probe on the same run was 32 ms for five membrane ports (load and Law 4 included).
