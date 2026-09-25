# Phase 5.2 Stop C: end of Chunk C (P52-08a to P52-13)

Recorded by Cursor on 25 Sep 2026 from `D:\JoInn\joinn` on Windows (PowerShell).
Values below are copied from the terminal, not summarised.

`git rev-parse HEAD` after P52-13 (last work commit of Chunk C):

```
6b539c57ae1a6362ae41245517cdd1ff7fbfefd4
```

The stop-report commit that adds this file is `P52-stop-c` on `main` immediately after that.

## Commit reports (in order)

```
Commit:     P52-08a 747d4db
Done-when:  cargo fmt --all -- --check → exits 0          MET
Suite:      cargo test --workspace --no-fail-fast → (mechanical fmt only; suite run with 23c418f)
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

```
Commit:     P52-08a 23c418f
Done-when:  cargo fmt --all -- --check → exits 0 · cargo clippy --workspace --all-targets -- -D warnings → Finished `dev` profile · corpus verify → corpus verify: 33 hash(es) match; cells admitted          MET
Suite:      cargo test --workspace --no-fail-fast → 168 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      collapsible_match at crates/joinn-live/src/slot/apply.rs:25 was not reported by this clippy (rustc 1.94); joinn-live was already clean under -D warnings
```

```
Commit:     P52-08b e0d6f18
Done-when:  corrupt_hash_does_not_flip_unrelated_controls → ok · gate all → harness fixtures: ok · phase 5: 7/8 · phase 5.1: 5/5          MET
Suite:      cargo test --workspace --no-fail-fast → 169 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

```
Commit:     P52-08c 4325c8f
Done-when:  git grep allow(vocab) saturating_sub|fs::metadata → (empty) · suite passes          MET (with DropLine hosts snag)
Suite:      cargo test --workspace --no-fail-fast → 169 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      DropLine(1) on universe.txt makes g51_hosts_control answer true (not false as written): line 1 is inside calculator.txt so the prefix breaks; test asserts true on mutant and false on the real subject
```

```
Commit:     P52-09 4cbe66e
Done-when:  trybuild bound_from_map fails · CorruptHash names calc+full hex · alias_is_local binds · bind_bodies gone · insert second face refused · phase 5.2: 1/1 · phase 5.1: 4/4          MET
Suite:      cargo test --workspace --no-fail-fast → 171 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

```
Commit:     P52-10 2fde6f6
Done-when:  body_refusal_is_a_report_and_universe_keeps_running → ok (Refused{calc,cli_a}, units holds 60; second host 12 → units Refused, no LinkRefusal)          MET
Suite:      cargo test --workspace --no-fail-fast → 172 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

```
Commit:     P52-11 3b6dcaf
Done-when:  DropGrant(e0) → units does not fire · grant on unordered names link · git grep grant( cli/test-host empty · hashes listed          MET
Suite:      cargo test --workspace --no-fail-fast → 174 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
Moved universe hashes:
  universe.universe 0b784f4c5219fd0bc77076dd6562e53ae5562884637c68b81486458f585bb828
  alone.universe da4db686fe8461e35304bc5cfb988ceac4a5ede07b66390d493469de1a71dcf4
  ordered.universe ded37f4f0d3fba33085fb0cb6037083b507bba138fa05acc69d52ca66e0a667e
  adversary.universe 313bc956d9f79f1b8e7c58f211a0af9cb1989ee117246dfc63072e9b276d61a5
```

```
Commit:     P52-12 335f7fe
Done-when:  universe.txt byte-identical · RenameLink/RenameAlias leave CLI output · units intent_set {scale@1} · gate 5.2 3/3          MET
Suite:      cargo test --workspace --no-fail-fast → 178 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

```
Commit:     P52-13 6b539c5
Done-when:  probe label contains inner_reason.txt · no far-side line does · Replace(e0) flips control · second_sum_stays_home · gate 5·8 ok          MET
Suite:      cargo test --workspace --no-fail-fast → 178 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
inner_reason.txt (first line of `cargo xtask probe-refusal`, redirected):
  join refuse at scale port 0
```

## Last line of each command (end of Chunk C, on 6b539c5)

- `cargo test --workspace --no-fail-fast` — `178 passed, 0 failed`
- `cargo xtask gate all` — `gate all wall milliseconds: 325174` (exit 0)
- `cargo xtask corpus verify` — `corpus verify: 33 hash(es) match; cells admitted`
- `cargo xtask vocab` — `vocab: ok`
- `cargo xtask modules` — `modules: ok (enforced 11 crate(s))`

### Every `gate all` line containing `fail`

```
1 ok  references agree; a blind seal fails the item
```

(No phase-row `fail` lines. Gate all exits 0.)

### Every failing test name

none (suite: 178 passed, 0 failed)

### Lock written by `gate all` at end of Chunk C

```
phase 0: pass
phase 1: 4/4 legacy
phase 2: 4/4 legacy
phase 2.1: 8/8 legacy
phase 2.2: 9/9 legacy
phase 3: 8/8 legacy
phase 5: 8/8
phase 5.1: 4/4
phase 5.2: 3/3
```

## P52-08 control-file comparison (carried into Stop C)

Five hand-written controls kept (not byte-identical to any catalogue mutant). Closest mutation and first differing line (coding print / source form):

| Kept file | Closest mutation | First differing line |
|---|---|---|
| `no_such_port.universe` | `ShiftPort(e0, calc.sum@2, 9)` | control: `link missing order none {` · mutant: `link e0 order ordered {` |
| `wrong_direction.universe` | `FlipMark(e0, calc.sum@2)` | control: `calc.cli_a@0 tail` · mutant: `calc.sum@2 head` (after mark flip on sum) |
| `frame_mismatch.universe` | `SwapBinding(units, …)` | control: `body:556e7859… as echo` · mutant keeps `as units` with a swapped hash |
| `two_systems.universe` | `CopyMember(function, units, calculation)` | control: `system calculation { calc units }` with empty `links { }` · mutant still has `link e0` |
| `wrong_container.universe` | `WireAcross(e0)` | control: `wire calc.sum@2 -> units.scale@0` · mutant: cross-wire after dropping `e0` (no `wire` keyword in coding print) |

## Snags (all from the chunk)

| Commit | Snag |
|---|---|
| P52-08a | `collapsible_match` at `apply.rs:25` not reported on rustc 1.94; live already clean |
| P52-08c | `DropLine(1)` on `universe.txt` makes `g51_hosts_control` true (not false as the plan wrote); test asserts that honest effect |
| P52-08 | five hand-written control universes kept (table above) |
