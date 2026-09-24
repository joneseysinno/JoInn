# Phase 5.2 Stop B: end of Chunk B (P52-02b to P52-08)

Recorded by Cursor on 24 Sep 2026 from `D:\JoInn\joinn` on Windows (PowerShell).
Values below are copied from the terminal, not summarised.

`git rev-parse HEAD` after P52-08 (last work commit of Chunk B):

```
0dcad2659784a6da22e996cebaea96b03112374a
```

The stop-report commit that adds this file is `P52-stop-b` on `main` immediately after that.

## Commit reports (in order)

```
Commit:     P52-02b 8a7229a
Done-when:  universe_transcript_matches_golden → ok · modules → modules: ok (enforced 11 crate(s)) · vocab → vocab: ok · duplicate_face → ok · git grep allow(vocab) saturating_sub|fs::metadata → (empty)          MET
Suite:      cargo test --workspace --no-fail-fast → 148 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      test-host load_body_set is IO-free (rule 15); refusal test under tests/one_face.rs · cargo test -- --exact needs full path session::run_universe::tests::universe_transcript_matches_golden
```

```
Commit:     P52-03 6fca02d
Done-when:  run_gate_table refusal → gate item 1 (bad universe): expected universe · gate all → phase 2: 8/8 legacy · no artifact_loads under xtask/src          MET
Suite:      cargo test --workspace --no-fail-fast → 149 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      dropped gate 5.1 item Every control reads its artifact from the table early (.rs is not a Subject kind; §2.3 would delete it in P52-06); planted .body parse was too permissive so refusal fixture is .universe
```

```
Commit:     P52-04 bfe7f09
Done-when:  cargo test -p xtask drop_link_e0_stops_units → ok · 18 mutation tests with downstream effects · DropGrant stub refuses until P52-11          MET (with DropGrant snag)
Suite:      cargo test --workspace --no-fail-fast → 168 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      DropGrant always refuses naming the link until grants land in P52-11 (Universe has no grants field): no grant on link e0; acceptance is a declared grants section (P52-11)
```

```
Commit:     P52-05 fd44602
Done-when:  cargo xtask gate all refused listing items → MET
Refused:    5·1 Something crosses (false on mutant); 5·2 The membrane is measured (false on mutant); 5·3–5·7 (true on real); 5.1·1 Something crosses (false on mutant); 5.1·3 Bodies are bound by hash (true on real); 5.1·4 Links are typed (true on real); 5.1·5 The boundary is total (no such instance orphan); 5.1·6 Revocation stops delivery (true on real)
Suite:      cargo test --workspace --no-fail-fast → 168 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      predicted 5·8 fail but it passes opposition (Replace(e0) flips dishonest control because far-side already contains e0) · missing_cell.body still has scale not orphan · Links are typed not yet split
```

```
Commit:     P52-06 cb59394
Done-when:  broken fixture 1 → harness fixture 1: a control that ignores its subject must be refused (no gate rows) · revert → harness fixtures: ok then gates · phase 3: 8/8 legacy          MET
Suite:      cargo test --workspace --no-fail-fast → 168 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      set_score unit test pointed at in-test fixture lock (live gates.lock became 0/0 after refused gate all)
```

```
Commit:     P52-07 21e2a05
Done-when:  duplicate (artifact, opposes) → distinct opposition: The universe is well-formed and Tails are out, heads are in share (...) · revert → harness fixtures: ok · phase 2: 4/4 legacy          MET
Suite:      cargo test --workspace --no-fail-fast → 168 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      gate 5.1 does not print numbered rows yet — grade_opposed still refuses Bodies and boundary from P52-05 until P52-08
```

```
Commit:     P52-08 0dcad26
Done-when:  gate all harness ok · phase 5: 7/8 (8 fail A refusal stays home) · phase 5.1: 5/5 · cmp unlinked vs DropLink equal=true deleted · others equal=false kept          MET
Suite:      cargo test --workspace --no-fail-fast → 168 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      five hand-written control universes kept (mutations close but not byte-identical): no_such_port, wrong_direction, frame_mismatch, two_systems, wrong_container
```

## Last line of each command (end of Chunk B, on 0dcad26)

- `cargo test --workspace --no-fail-fast` — `168 passed, 0 failed`
- `cargo xtask gate all` — `gate all: a phase failed` (exit 1)
- `cargo xtask corpus verify` — `corpus verify: 33 hash(es) match; cells admitted`
- `cargo xtask vocab` — `vocab: ok`
- `cargo xtask modules` — `modules: ok (enforced 11 crate(s))`

### Every `gate all` line containing `fail`

```
8 fail  A refusal stays home
gate all: a phase failed
```

### Every failing test name

none (suite: 168 passed, 0 failed)

### Lock written by `gate all` at end of Chunk B

```
phase 0: pass
phase 1: 4/4 legacy
phase 2: 4/4 legacy
phase 2.1: 8/8 legacy
phase 2.2: 9/9 legacy
phase 3: 8/8 legacy
phase 5: 7/8
phase 5.1: 5/5
```

## Snags (all from the chunk)

| Commit | Snag |
|---|---|
| P52-02b | test-host `load_body_set` is IO-free (rule 15); refusal test under `tests/one_face.rs` |
| P52-02b | `cargo test -- --exact` needs full path `session::run_universe::tests::universe_transcript_matches_golden` |
| P52-03 | dropped gate 5.1 *Every control reads its artifact* early (`.rs` not a Subject kind; deleted properly in P52-06) |
| P52-03 | planted `.body` parse too permissive; refusal fixture is `.universe` |
| P52-04 | `DropGrant` always refuses until P52-11: `no grant on link e0; acceptance is a declared grants section (P52-11)` |
| P52-05 | predicted 5·8 opposition fail; it passed (`Replace("e0")` flips dishonest control) |
| P52-05 | `missing_cell.body` still had `scale` not `orphan` (fixed in P52-08) |
| P52-05 | *Links are typed* not yet split (split in P52-07) |
| P52-06 | `set_score` unit test pointed at in-test fixture lock |
| P52-07 | gate 5.1 did not print numbered rows until P52-08 fixed opposition |
| P52-08 | five hand-written control universes kept (not byte-identical to mutants): `no_such_port`, `wrong_direction`, `frame_mismatch`, `two_systems`, `wrong_container` |
