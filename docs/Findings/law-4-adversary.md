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

## Second attempt

`asker.body` is that same body with grants `stdin: question, answer`.
`ask.universe` tails `lookup.question@2` into `units.scale@0` and tails
`units.scale@2` back into `lookup.answer@0`. The question is multiply by 1.
Lines copied from
`cargo test -p joinn-link --test crossing law4_adversary_asks_by_one -- --nocapture`:

```
Law 4: admitted
typing: admitted
assembly: admitted
round 0 factor 12: [Fired { body: "lookup", instance: "question" }, Fired { body: "units", instance: "scale" }, Fired { body: "lookup", instance: "answer" }] answer 12
round 1 factor 5: [Fired { body: "lookup", instance: "question" }, Fired { body: "units", instance: "scale" }, Fired { body: "lookup", instance: "answer" }] answer 5
one run: [Refused { body: "lookup", instance: "answer" }, Refused { body: "units", instance: "scale" }]
```

**Held.** Law 4 admitted a body that asks another body a question and pairs each answer with its question, using only wires, two hyperedges and declared grants. Pairing held with one question in flight per round. With two questions in one run, both were refused and no answer came back, so the pairing came from the host's rounds, not from the universe.

Two things this shows. `units` keeps no memory of its factor, so "the factor you last used" can only be asked by making `units` fire again with 1, which spends a firing: asking is not free. And a universe with two questions in flight needs an order that spans the `ask` and `reply` links, which is R58.
