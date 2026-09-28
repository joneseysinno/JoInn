# Phase 4 Stop C: end of chunk C (P4-10e to P4-14)

Recorded by Cursor on 28 Sep 2026 from `D:\JoInn` on Windows (PowerShell).
Values below are copied from the terminal, not summarised.

`git rev-parse HEAD` after P4-14 (last commit of chunk C):

```
ae81d8e1b1d45372a7dad0d629797e856b2d25f8
```

The stop-report commit that adds this file is `P4-stop-c` on `main` immediately after that.

## Commit reports (in order)

```
Commit:     P4-10e ddbff90
Done-when:  git show --stat HEAD → docs/Plans/JoInn Phase 4 Implementation Plan.md only → MET
Suite:      cargo test --workspace --no-fail-fast → 195 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

```
Commit:     P4-11 1ff4ec9
Done-when:  cargo xtask corpus verify → corpus verify: 43 hash(es) match; cells admitted → MET
git diff --stat -- corpus → joinn/corpus/hashes.txt | 2 ++ (plus the new phase4/loop_declared.universe); no other hash moved
phase4/loop_declared.universe: assemble admitted
phase4/loop_declared.universe SwapBinding(fmt, fmt_twin): refused: declaration assert H₁ = 0 does not hold: H₁: 1; open: calc →calc.cli_a@0→ fmt →fmt.fmt@0→ calc; acceptance is H₁: 0, every loop filled by a frame or a law
phase4/loop.universe SwapBinding(fmt, fmt_twin): assemble admitted
body with assert H₂ = 0 → declaration assert H₂ = 0 is not admitted; acceptance is assert H₁ = 0
calculator.body + assert H₁ = 0: insert admitted
calculator.body + assert H₁ = 0, SwapCell(cli_b, cli_input_open): refused: declaration assert H₁ = 0 does not hold: H₁: 1; open: outside →body.cli_b@0→ body →body.sum@2→ outside; acceptance is H₁: 0, every loop filled by a frame or a law
SwapCell(cli_b, cli_input_open) without declaration: insert admitted
cargo xtask assay agree → assay agree: 38 subject(s) agree, 1 not measured; injected disagreements: 2 refused as truth violation (ok)
cargo xtask assay invariance → phase4/loop_declared.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok (exit 0)
cargo xtask assay phase4/loop_declared.universe → the same report as phase4/loop.universe (Compare-Object printed nothing)
Suite:      cargo test --workspace --no-fail-fast → 203 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

```
Commit:     P4-12 4225f3d
Done-when:  cargo test -p xtask mutate:: → test result: ok. 22 passed; 0 failed → MET
fns::mutate::drop_declaration::tests::a_dropped_body_declaration_lets_the_swap_insert ... ok
  (declared SwapCell(cli_b, cli_input_open) calculator refused at insert naming assert H₁ = 0; after DropDeclaration it inserts; a body with no declaration → no declaration to drop)
fns::mutate::drop_declaration::tests::a_dropped_universe_declaration_lets_the_twin_assemble ... ok
  (loop_declared.universe SwapBinding(fmt, fmt_twin) refused naming assert H₁ = 0; after DropDeclaration it assembles; loop.universe → no declaration to drop)
fns::mutate::add_wire::tests::add_wire_joins_the_asker_into_one_region ... ok
  (asker.body AddWire(question@2, answer@0) prints Amendment C's report byte for byte, regions: body 1; AddWire(ghost@2, answer@0) refused naming ghost; the same wire twice refused naming question@2 -> answer@0)
Neutral edits unchanged (neutral.rs not touched).
Suite:      cargo test --workspace --no-fail-fast → 206 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

```
Commit:     P4-13 e0aad8e
Done-when:  cargo xtask gate all → exit 0 → MET
invariance phase2/calculator.body: 1 ok, allele strip ok, 3 n/a (no lens), 4 n/a (body)
invariance phase4/loop.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
invariance phase52/adversary/asker.body: 1 ok, 2 ok, 3 n/a (no lens), 4 n/a (body)
invariance phase4/loop_declared.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phase 4: 4/4
phase 5: 8/8
phase 5.1: 4/4
phase 5.2: 3/3
gates.lock: phase 4 written by the lock writer between phase 3 and phase 5
Shown then reverted (item 4 given item 2's artifact corpus/phase4/loop.universe):
distinct opposition: A promise names its partner and A declaration refuses; an assay doesn't share (corpus/phase4/loop.universe, SwapBinding("fmt", "f1b05fd17e9eeaa284a03a6ea586073b7b328e377c169628fa347cdec239b0cf"))
After the revert: cargo xtask gate 4 → phase 4: 4/4
Suite:      cargo test --workspace --no-fail-fast → 207 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

```
Commit:     P4-14 ae81d8e
Done-when:  fresh clone (git clone D:\JoInn D:\p4c-fresh), cargo xtask gate all → exit 0 → MET
harness fixtures: ok
phase 0: pass
phase 1: 4/4 legacy
phase 2: 4/4 legacy
phase 2.1: 8/8 legacy
phase 2.2: 9/9 legacy
phase 3: 8/8 legacy
phase 4: 4/4
phase 5: 8/8
phase 5.1: 4/4
phase 5.2: 3/3
(git status in the clone after gate all: clean; the lock writer rewrote gates.lock byte for byte)
cargo xtask corpus verify → corpus verify: 43 hash(es) match; cells admitted
cargo xtask assay agree → assay agree: 38 subject(s) agree, 1 not measured; injected disagreements: 2 refused as truth violation (ok) (exit 0)
cargo xtask assay invariance → phenotype reader: refused at edit 1 on phase4/loop.universe (ok) (exit 0)
phase-4-hashes.md: git diff 4b47cf6..HEAD -- joinn/corpus/hashes.txt prints only added lines; no Phase 0–5.2 hash moved
cargo xtask decisions → decisions holds: 40 open: 20 reversed: 0
Suite:      cargo test --workspace --no-fail-fast → 207 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

## Last line of each command (end of chunk C, on ae81d8e)

- `cargo test --workspace --no-fail-fast` — `207 passed, 0 failed` (sum of every `test result:` passed count, including doc-test zeros; exit 0)
- `cargo xtask gate all` (fresh clone `D:\p4c-fresh`) — `gate all wall milliseconds: 381527` (exit 0)
- `cargo xtask corpus verify` — `corpus verify: 43 hash(es) match; cells admitted` (exit 0)
- `cargo xtask vocab` — `vocab: ok`
- `cargo xtask modules` — `modules: ok (enforced 11 crate(s))`
- `cargo xtask assay agree` — `assay agree: 38 subject(s) agree, 1 not measured; injected disagreements: 2 refused as truth violation (ok)` (exit 0)
- `cargo xtask assay invariance` — `phenotype reader: refused at edit 1 on phase4/loop.universe (ok)` (exit 0)

Phase lines from that `gate all`:

```
phase 0: pass
phase 1: 4/4 legacy
phase 2: 4/4 legacy
phase 2.1: 8/8 legacy
phase 2.2: 9/9 legacy
phase 3: 8/8 legacy
phase 4: 4/4
phase 5: 8/8
phase 5.1: 4/4
phase 5.2: 3/3
```

Gate 4 item lines from that `gate all`:

```
1 ok  The instrument reads a known sample
2 ok  A promise names its partner
3 ok  A body that is two things is named
4 ok  A declaration refuses; an assay doesn't
phase 4: 4/4
```

### Every `gate all` line containing `fail`

```
1 ok  references agree; a blind seal fails the item
```

### Every failing test name

none (suite on ae81d8e: 207 passed, 0 failed)

## CI

Newest run after the push of `ae81d8e`, read with `Invoke-RestMethod` on `https://api.github.com/repos/joneseysinno/JoInn/actions/runs?branch=main&per_page=3` and that run's `jobs_url`.

```
head_sha: ae81d8e1b1d45372a7dad0d629797e856b2d25f8
status: completed
conclusion: success
check (ubuntu-latest): success
check (windows-latest): success
```

Run: https://github.com/joneseysinno/JoInn/actions/runs/36446515185

## Snags (all from the chunk)

none
