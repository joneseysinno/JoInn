# Phase 5.2 Stop D: end of Chunk D (P52-13a to P52-16)

Recorded by Cursor on 25 Sep 2026 from `D:\JoInn` on Windows (PowerShell).
Values below are copied from the terminal, not summarised.

`git rev-parse HEAD` after P52-16 (last work commit of Chunk D):

```
d17e5240c51d6399b96d762308fa92e82ac83964
```

The stop-report commit that adds this file is `P52-stop-d` on `main` immediately after that.

Amendment C is commit `499d77c` (plan text only, not a numbered P52 step). It has no report block.

## Commit reports (in order)

```
Commit:     P52-13a 532542b
Done-when:  rustc --version → rustc 1.95.0 (59807616e 2026-04-14) · cargo fmt --all -- --check → (no output, exit 0) · cargo clippy --workspace --all-targets -- -D warnings → Finished `dev` profile [unoptimized + debuginfo] target(s) in 55.79s          MET
Suite:      cargo test --workspace --no-fail-fast → 178 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      CI status not readable here
```

```
Commit:     P52-13b a10d628
Done-when:  second_hop_reaches_again ok · again_refusal_is_reported_once ok · body_refusal_is_a_report_and_universe_keeps_running ok · universe_transcript_matches_golden ... ok · gate all phase 5: 8/8 phase 5.1: 4/4 phase 5.2: 3/3 exit 0          MET
Suite:      cargo test --workspace --no-fail-fast → 180 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      recipient-only follow sweep still printed left: 2 (second Refused { again, scale } on the next pass). Later sweeps of the same run skip a body already reported Refused.
```

Unfixed chain, pasted in that commit:

```
[Fired { body: "calc", instance: "cli_a" }, Fired { body: "calc", instance: "cli_b" }, Fired { body: "calc", instance: "sum" }, Fired { body: "units", instance: "scale" }]
```

Unfixed once: `assertion left == right failed` left 2 right 1, two `Refused { again, scale }`.

```
Commit:     P52-13c 704f05b
Done-when:  units intent_set[units] == {scale@1} · meters intent_set[meters] == {scale@1} · double_delivery_far_side_is_one_refusal ok · git grep only inside cfg(test)          MET
Suite:      cargo test --workspace --no-fail-fast → 181 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

```
Commit:     P52-13d 6591741
Done-when:  controls_cross_matrix ok · six controls false on CorruptHash/RenameLink/RenameAlias · gate all phase 5: 8/8 phase 5.1: 4/4 phase 5.2: 3/3 exit 0          MET
Suite:      cargo test --workspace --no-fail-fast → 182 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

Matrix printed by `controls_cross_matrix` (one line per gate item; empty means no mutation returned true):

```
Something crosses: DropLink("e0"), FlipMark("e0", "calc.sum@2"), ShiftPort("e0", "calc.sum@2", 9), CorruptHash("calc"), CorruptHash("units"), WireAcross("e0"), DropGrant("e0"), RenameAlias("units", "meters")
The membrane is measured:
The universe is well-formed: ShiftPort("e0", "calc.sum@2", 9)
Exclusivity holds: CopyMember("function", "units", "calculation")
Two lenses, one body: DropLens("deployment"), RenameAlias("units", "meters")
Law 4 is a check: WireAcross("e0")
A capability can be revoked: DropLink("e0"), FlipMark("e0", "calc.sum@2"), ShiftPort("e0", "calc.sum@2", 9), CorruptHash("calc"), CorruptHash("units"), WireAcross("e0"), DropGrant("e0"), RenameAlias("units", "meters")
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
Commit:     P52-14 d2aff16
Done-when:  the finding quotes the run and uses fired only as section 2.16 defines → the finding quotes Law 4: admitted and the typing refusal, and the word fired does not appear → MET
Suite:      cargo test --workspace --no-fail-fast → 183 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

The attempt printed:

```
lookup coding hash d999c0b83dfb9d1eee6b3cb4e8b461d38dd7dd585976b1aecf05ecdbece3acf3
Law 4: admitted
typing: link q0 member units.scale@1 is tail but direction is In; acceptance is Out
assembly: admitted
test law4_adversary_attempt ... ok
```

```
Commit:     P52-15 0623292
Done-when:  the perf line in the finding matches a fresh cargo xtask perf run except timing numbers → membrane load+parse milliseconds: 23 compute milliseconds: 2 bodies: 15 ports: 175 refused: 0 → MET
Suite:      cargo test --workspace --no-fail-fast → 183 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

Same `cargo xtask perf` run also printed:

```
two-body universe membrane ports: 6 milliseconds: 39
```

R63 was already `status: decided` in the backlog. It was not duplicated.

```
Commit:     P52-16 d17e524
Done-when:  cargo xtask gate all → harness fixtures: ok · phase 1: 4/4 legacy · phase 2: 4/4 legacy · phase 2.1: 8/8 legacy · phase 2.2: 9/9 legacy · phase 3: 8/8 legacy · phase 5: 8/8 · phase 5.1: 4/4 · phase 5.2: 3/3 · gate all wall milliseconds: 328265 · exit 0 · corpus verify: 35 hash(es) match; cells admitted → MET on this tree
Suite:      cargo test --workspace --no-fail-fast → 183 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      CI status not readable here. gh is not a recognized command, so a green run on both operating systems is not visible. The fresh-clone gate all is run after this commit and quoted below.
```

Fresh clone of `d17e524` into `%TEMP%\joinn-p52-fresh`, then `cargo xtask gate all` from that clone's `joinn/`:

```
phase 2.1: 7/8 legacy
6 fail  calculator.trace replays
phase 5.1: 3/4
1 fail  Two hosts, one universe
phase 5.2: 1/3
2 fail  A refusal is a report
3 fail  The host knows no ids
gate all: a phase failed
gate all wall milliseconds: 345658
CLONE_GATE_EXIT:1
corpus verify: 35 hash(es) match; cells admitted
CLONE_VERIFY_EXIT:0
```

`git ls-files --eol` on the clone:

```
i/lf    w/crlf  attr/                 	joinn/corpus/transcripts/calculator.txt
i/lf    w/crlf  attr/                 	joinn/corpus/transcripts/universe.txt
```

`universe.txt` clone_cr=7 work_cr=0 (clone_len=102, work_len=95). `calculator.trace` clone_cr=27 work_cr=0 (clone_len=472, work_len=445). `git config --get core.autocrlf` on this machine printed `true`. The committed blobs are LF. The LF work tree at `D:\JoInn` is the run that exited 0.

Done-when for a fresh clone that exits 0: **NOT MET** on this Windows checkout. `corpus verify` matches on both trees.

## Last line of each command (end of Chunk D, on d17e524, LF work tree)

- `cargo test --workspace --no-fail-fast` — `183 passed, 0 failed`
- `cargo xtask gate all` — `gate all wall milliseconds: 328265` (exit 0)
- `cargo xtask corpus verify` — `corpus verify: 35 hash(es) match; cells admitted`
- `cargo xtask vocab` — `vocab: ok`
- `cargo xtask modules` — `modules: ok (enforced 11 crate(s))`

### Every `gate all` line containing `fail`

LF work tree (exit 0):

```
1 ok  references agree; a blind seal fails the item
```

Fresh clone (exit 1), in addition to that same ok line:

```
6 fail  calculator.trace replays
1 fail  Two hosts, one universe
2 fail  A refusal is a report
3 fail  The host knows no ids
gate all: a phase failed
```

### Every failing test name

none (suite on d17e524: 183 passed, 0 failed)

### `gh run list --branch main --limit 3`

```
The term 'gh' is not recognized as a name of a cmdlet, function, script file, or executable program.
```

## Snags (all from the chunk)

| Commit | Snag |
|---|---|
| P52-13a | CI status not readable here |
| P52-13b | recipient-only follow sweep still printed `left: 2`; later sweeps of the same `run()` skip a body already reported `Refused` |
| P52-16 | `gh` is not installed, so CI on both operating systems is not visible. A fresh clone with `core.autocrlf=true` checks out CRLF and `cargo xtask gate all` exits 1 (`CLONE_GATE_EXIT:1`, wall milliseconds 345658). The LF work tree exits 0. |
