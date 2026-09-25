# Law 4 adversary

Date: 25 September 2026. The lines below are copied from
`cargo test -p joinn-link --test crossing law4_adversary_attempt -- --nocapture`.

## Phase 5 entry, corrected

The 22 September note tried a three-member hyperedge (`calc.sum@2` tail,
`units.scale@1` head, `bus.listen@0` head) and found that want expressible as
a hyperedge between bodies. That attempt did not test Law 4.

**held — the attempt did not test Law 4**

## This attempt

The spec is `docs/Findings/law-4-adversary-spec.md`, copied from the plan.

`lookup` asks `units`, "what factor did you last use?", with two hyperedges.
Each tail is `units.scale@1`, the port the factor enters. The heads are
`lookup.answer@0` (`q0`) and `lookup.question@0` (`q1`). Nothing in the
universe is a wire between bodies. `lookup` is built from the same cell
`units` already uses. Grants stay inside `lookup` (`stdin: question`).

Printed:

```
lookup coding hash d999c0b83dfb9d1eee6b3cb4e8b461d38dd7dd585976b1aecf05ecdbece3acf3
Law 4: admitted
typing: link q0 member units.scale@1 is tail but direction is In; acceptance is Out
assembly: admitted
test law4_adversary_attempt ... ok
```

The result is the typing refusal:

`link q0 member units.scale@1 is tail but direction is In; acceptance is Out`

Law 4 admitted both hyperedges. The factor port is an in-port, so a hyperedge
cannot carry that value out to `lookup`. The run never produced two answers,
so it does not show that pairing needs an order across two links.
