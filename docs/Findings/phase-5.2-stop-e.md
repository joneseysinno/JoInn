# Phase 5.2 Stop E: end of Chunk E (P52-16a to P52-16e)

Recorded by Cursor on 25 Sep 2026 from `D:\JoInn` on Windows (PowerShell).
Values below are copied from the terminal, not summarised.

`git rev-parse HEAD` after P52-16d (last code commit of Chunk E):

```
ae363b1facc4ba88ce6451ca255267a9fe281997
```

The stop-report commit that adds this file is `P52-stop-e` on `main` immediately after that.

The plan amendment (what P52-16a asked to commit) was already on `main` before this chunk's code, as `d7a5e592484461c5406f6819d9f7c3f6eb001a58`, with message id `P52-16e`. The diff is the plan text only. That commit was not rewritten.

## Commit reports (in order)

```
Commit:     P52-16a d7a5e59
Done-when:  plan text only, already committed and pushed before this session → the diff against 3ffe041 is docs/Plans/JoInn Phase 5.2 Implementation Plan.md, 89 insertions, 1 deletion
Suite:      not re-run for this already-pushed commit
Scans:      not re-run for this already-pushed commit
Snags:      the message id is P52-16e, not P52-16a. The message also claims line endings and CI, which that diff does not contain.
```

```
Commit:     P52-16b b0c9996
Done-when:  git add --renormalize . staged numstat `1	0	joinn/xtask/src/fns.rs` and no other tracked path (content, then unstaged) · corpus_has_no_carriage_returns ok · CRLF universe.txt panicked `D:\JoInn\joinn\corpus\transcripts\universe.txt contains a carriage return` · restored `i/lf    w/lf    attr/text=auto eol=lf` · after push, clone -c core.autocrlf=true printed `W_CRLF_COUNT 0` and `cargo xtask gate all` from that clone printed `phase 5: 8/8` `phase 5.1: 4/4` `phase 5.2: 3/3` `gate all wall milliseconds: 294440` exit 0 → MET
Suite:      cargo test --workspace --no-fail-fast → 184 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      P52-16a was already pushed as d7a5e59 with message id P52-16e
```

```
Commit:     P52-16c 30dd0de
Done-when:  controls_cross_matrix ok · nine controls false on CorruptHash(calc), CorruptHash(units), RenameLink(e0, e1), RenameAlias(units, meters) · Something crosses and A capability can be revoked rows differ · gate all phase 5: 8/8 phase 5.1: 4/4 phase 5.2: 3/3 exit 0 · git grep '"units"' on the three control files prints nothing (exit 1) → MET
Suite:      cargo test --workspace --no-fail-fast → 184 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

Matrix printed by `controls_cross_matrix`:

```
Something crosses: DropLink("e0"), WireAcross("e0"), DropGrant("e0")
The membrane is measured:
The universe is well-formed: ShiftPort("e0", "calc.sum@2", 9)
Exclusivity holds: CopyMember("function", "units", "calculation")
Two lenses, one body: DropLens("deployment")
Law 4 is a check: WireAcross("e0")
A capability can be revoked: DropGrant("e0")
A refusal stays home: Replace("e0"), Replace("units"), Replace("link")
Two hosts, one universe: DropLine(0), DropLine(1), DropLine(2), SwapLines(0, 1), SwapLines(2, 3)
Tails are out, heads are in: FlipMark("e0", "calc.sum@2")
Members share a frame:
The boundary is total:
Binding is by store: CorruptHash("calc")
A refusal is a report: DropLine(0), DropLine(1), DropLine(2)
The host knows no ids: DropGrant("path")
```

```
Commit:     P52-16d ae363b1
Done-when:  law4_adversary_asks_by_one printed and asserted the lines below · corpus verify: 37 hash(es) match; cells admitted → MET
Suite:      cargo test --workspace --no-fail-fast → 185 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      The research backlog had no R58 section. The example is under a new heading, R58 — Correlation of a reply with its question, using the title already in decisions.md.
```

Printed by `cargo test -p joinn-link --test crossing law4_adversary_asks_by_one -- --nocapture`:

```
Law 4: admitted
typing: admitted
assembly: admitted
round 0 factor 12: [Fired { body: "lookup", instance: "question" }, Fired { body: "units", instance: "scale" }, Fired { body: "lookup", instance: "answer" }] answer 12
round 1 factor 5: [Fired { body: "lookup", instance: "question" }, Fired { body: "units", instance: "scale" }, Fired { body: "lookup", instance: "answer" }] answer 5
one run: [Refused { body: "lookup", instance: "answer" }, Refused { body: "units", instance: "scale" }]
test law4_adversary_asks_by_one ... ok
```

```
Commit:     P52-16e (no tree commit; the step changes no file)
Done-when:  Invoke-RestMethod on the newest run after the push of ae363b1 → head_sha=ae363b1facc4ba88ce6451ca255267a9fe281997 status=completed conclusion=success · job name=check (ubuntu-latest) conclusion=success · job name=check (windows-latest) conclusion=success → MET
Suite:      cargo test --workspace --no-fail-fast → 185 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

## Last line of each command (end of Chunk E, on ae363b1)

- `cargo test --workspace --no-fail-fast` — `185 passed, 0 failed`
- `cargo xtask gate all` — `gate all wall milliseconds: 294440` (exit 0), from `git clone -c core.autocrlf=true` of `ae363b1` at `D:\JoInn-crlf-clone\joinn`. The same tree's earlier gate all, before the adversary files, printed `gate all wall milliseconds: 289094` (exit 0) with `corpus verify: 35 hash(es) match; cells admitted`.
- `cargo xtask corpus verify` — `corpus verify: 37 hash(es) match; cells admitted`
- `cargo xtask vocab` — `vocab: ok`
- `cargo xtask modules` — `modules: ok (enforced 11 crate(s))`

Clone `git ls-files --eol` lines containing `w/crlf`: count `0`.

Phase lines from that clone's `gate all`:

```
phase 5: 8/8
phase 5.1: 4/4
phase 5.2: 3/3
```

### Every `gate all` line containing `fail`

```
1 ok  references agree; a blind seal fails the item
```

### Every failing test name

none (suite on ae363b1: 185 passed, 0 failed)

## Snags (all from the chunk)

| Commit | Snag |
|---|---|
| P52-16a | Already pushed as `d7a5e59` with message id `P52-16e`. The diff is the plan text only. Not rewritten. |
| P52-16d | The research backlog had no R58 section. The example was placed under a new heading using the title in `decisions.md`. |
| P52-16e | No file changed, so there is no commit. The CI paste is in the report block above. |
