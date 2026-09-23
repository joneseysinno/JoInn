# Live engine performance (P21-18)

Measured 2026-09-18 by `cargo xtask perf`. Typed by hand from stdout.
`xtask` writes no file under this directory.

| Probe | Steps | Milliseconds |
|---|---|---|
| Calculator body (cli_a + cli_b + sum, inputs 2 and 3) | 5 | 3 |
| 50 independent `Sum` instances, each fired once | 101 | 3 |
| Reference `int.add` of `0 + 64` on the live engine | 64 | 52 |

The calculator is one more step than Phase 2 recorded, because the quiescent
report now has its own index (`step 5`). Deliveries remain four.

**Extrapolation.** Steps still scale linearly with deliveries: the 50-cell body
is about 20× the calculator. A creator-sized body of a few hundred cells is
therefore a few hundred to a few thousand steps. The reference `add` on a
large second addend is linear in the addend's magnitude — that is the cost of
an honest reference allele, and the sealed path exists so the creator does not
wait on it. Wall time on this machine is milliseconds, not minutes.

**Risk #3 has fired: no.** There is no superlinear blow-up in the live engine
on these three probes. The two-engine model stays sound at Phase 2.1 scale.
A later phase that sees a creator-sized body spend minutes on a single
edit should reopen this file rather than silently optimize.

## P3-16 · four probes (19 September 2026)

Measured by `cargo xtask perf`. Typed by hand from stdout. `xtask` writes
no file under this directory.

| Probe | Steps or bytes | Milliseconds |
|---|---|---|
| Calculator body (cli_a + cli_b + sum, inputs 2 and 3) | 5 steps | 3 |
| 50 independent `Sum` instances, each fired once | 101 steps | 3 |
| Reference `int.add` of `0 + 64` on the live engine | 64 steps | 65 |
| CLI host (`joinn run calculator`) | 71 bytes of transcript | 301 |
| Test-host calculator (in-process inject/run) | 5 steps | 3 |

The CLI number is process spawn (`cargo run -p joinn-cli`), not engine
time: the same calculator under the test host is 5 steps in 3 ms, matching
the first probe. The 301 ms is the cost of proving a second host, not a
cost inside `BodyState::run`.

**Risk #3 has fired: no.** The fourth probe does not change the
extrapolation. Two hosts, same steps.

## P5-21 · six probes (22 September 2026)

Measured by `cargo xtask perf`. Typed by hand from stdout. `xtask` writes
no file under this directory.

| Probe | Result | Milliseconds |
|---|---|---|
| Calculator body | 5 steps | 5 |
| 50 independent Sum instances | 101 steps | 4 |
| Reference int.add of 0 + 64 | 64 steps | 65 |
| CLI host (`joinn run calculator`) | 71 bytes | 2525 |
| Test-host calculator | 5 steps | 3 |
| Two-body universe membrane ports | 5 ports | 32 |
| membrane() over the corpus | 23 bodies, 18 ports | 28 |

Six probes printed. The CLI figure remains spawn cost. membrane() over the
corpus is 28 ms — more than a few milliseconds — and that number is recorded
in `r50-membrane-cost.md`. No membrane is cached.
