# Phase 5.1 witness run (P52-00)

Date: 24 September 2026. Windows, PowerShell. Commands run from `D:\JoInn\joinn`
on the tree as Phase 5.1 left it. AJ ran the commands. The file was transcribed
from AJ's terminal output. Nothing below is paraphrased from a plan or a review.

## Files changed outside `target/`

- `gates.lock`, rewritten by `cargo xtask gate all` (expected).
- `crates/joinn-link/tests/fail/string_reason.stderr` already existed before this
  run. It was written at 22:32 UTC on 23 September, during the earlier P52-00
  Cursor session, not by Phase 5.1 (see the review addendum). This run did not
  change it.
- Anything else: confirm with `git status` → ___

## Last line of each command

- `cargo test --workspace --no-fail-fast` — `error: 6 targets failed`
- `cargo xtask gate all` — `gate all: a phase failed` (exit code 1)
- `cargo xtask corpus verify` — `corpus verify: 33 hash(es) match; cells admitted`
- `cargo xtask vocab` — `vocab: 3 hit(s)` (exit code 1)
- `cargo xtask modules` — `modules: ok (enforced 11 crate(s))`
- `cargo xtask perf` — `membrane load+parse milliseconds: 24 compute milliseconds: 2 bodies: 18 ports: 184 refused: 0`

## Every failure

### `cargo test` — 9 tests fail in 6 targets

| Target | Test | Panic |
|---|---|---|
| `joinn-link --test adversary` | `three_body_bus_is_expressible_under_law4` | `adversary.rs:74` — `instance scale cell 12b6 was not supplied` |
| `joinn-link --test assemble` | `universe_universe_assembles` | `assemble.rs:71` — `instance scale cell 12b6 was not supplied` |
| `joinn-link --test assemble` | `transits_refuses_naming_calc_sum_0_and_the_wire` | `assemble.rs:92` — `instance scale cell 12b6 was not supplied` |
| `joinn-link --test capability` | `grant_succeeds_then_revoke_refuses_naming_the_capability` | `capability.rs:27` — `link g0 is not in this universe` |
| `joinn-link --test law4` | `well_formed_universe_still_assembles` | `law4.rs:117` — `instance scale cell 12b6 was not supplied` |
| `joinn-link --test universe` | `hand_computed_universe_hash_matches` | `universe.rs:81` — left `2ccbb067…c9d1ad`, right `80083072…ef9d9e` |
| `joinn-link --test universe` | `ordered_member_order_survives_csr_and_unordered_sorts` | `universe.rs:109` — left `"sum"`, right `"scale"` |
| `joinn-link --test universe` | `three_universes_round_trip` | `universe.rs:54` — `assertion failed: idx_units < idx_calc` |
| `joinn-test-host --test units` | `units_runs_standalone_under_test_host` | `units.rs:57` — `unknown cell hash 12b6e545…120fa7` |

Everything else passed, including `crossing.rs` (6/6),
`tests/fail/string_reason.rs`, the CLI's `universe_transcript_matches_golden`,
and xtask's unit tests (18/18, including V86's
`no_two_gate_items_share_a_check_or_control`).

### `cargo xtask gate all`

```
phase 5: 7/8        8 fail  A refusal stays home
phase 5.1: 8/9      7 fail  A refusal stays home
phase 2: control does not read its artifact: references agree
phase 2.1: control does not read its artifact: references agree; a blind seal fails the item; V33 refuses mutant 14; floor pairing; agree catches wrapping
phase 2.2: control does not read its artifact: every seal can see; the references are true; a bound cannot be empty; V33 walks the whole genome; registers are enforced; the round-trip holds
```

Also printed, between phase 1 and phase 3, with no item label: `differing field: value`.

The lock `gate all` wrote:

```
# JoInn gates.lock — recorded passed gates. Never edit to make a check pass.
phase 0: pass
phase 1: 4/4
phase 2: 0/0
phase 2.1: 0/0
phase 2.2: 0/0
phase 3: 9/9
phase 5: 7/8
phase 5.1: 8/9
gate all wall milliseconds: 8597
```

### `cargo xtask vocab`

```
crates\joinn-link\src\universe_state\run.rs:27: turn-ident sub
crates\joinn-link\src\universe_state\run.rs:36: turn-ident sub
xtask\src\fns\g51_lock.rs:9: turn-ident sub
```

## The review's predictions, graded

| Prediction | Verdict |
|---|---|
| F44 · `gate all` has never run to completion on a 5.1 tree | **Confirmed, and now closed.** This run wrote the first lock with a `phase 5.1` row |
| F49 · gate 5 prints 7/8 and gate 5.1 prints 8/9, both on *A refusal stays home* | **Confirmed**, exactly |
| F52 · `capability.rs` panics on `g0` | **Confirmed** |
| F52 · `adversary.rs` fails for lack of the mul cell `12b6` | **Confirmed** |
| F52 · `string_reason.rs` fails with no `.stderr` | **Not observable on this run.** The `.stderr` was written during the earlier P52-00 session, so the test passes. Its content still has to be confirmed in P52-01 |
| F52 · "red in **at least** three places" | **Confirmed, but the count was low.** 9 tests fail, not 3 (see below) |
| F55 · `vocab` has 3 hits | **Confirmed** |
| `phase-5.1-hashes.md` · "`corpus verify` matches the new goldens" | **Confirmed**: 33 hashes match |
| Addendum · membrane cost is mostly loading (25 ms load, 2 ms compute) | **Confirmed**: 24 ms load+parse, 2 ms compute, 18 bodies, 184 ports, 0 refused |

## What the review did not predict

1. **The missing mul cell breaks four more tests, not one.** `assemble.rs` (both
   tests), `law4.rs::well_formed_universe_still_assembles`, and the test host's
   `units.rs` fail the same way `adversary.rs` does: they load the new
   `units.body` without supplying cell `12b6…`, and the total ∂ (P51-05)
   correctly refuses. The cause is the same as F52's second item. Only the reach
   is wider.
2. **`universe.rs` still expects the Phase 5 universe.**
   `hand_computed_universe_hash_matches` expects `8008307…`, which
   `phase-5.1-hashes.md` records as the *was* hash. The actual `2ccbb067…` is
   the recorded *now* hash. The other two failures look like the same staleness:
   member order (`"sum"` vs `"scale"`) and alias order (`idx_units < idx_calc`)
   were written against the file P51-12 replaced. This needs a reviewer's read
   before any expectation is changed (rule 7). A hash that moved on purpose and
   is recorded in `phase-5.1-hashes.md` is not a rebless. An order assertion
   might be.
3. **Gates 2, 2.1 and 2.2 are refused whole by the harness and record `0/0`.**
   P51-02's damage check ("control does not read its artifact") refuses 1, 5 and
   6 of their controls respectively. The review studied gates 5 and 5.1 and did
   not predict this. It could only show up once `gate all` actually ran (F44).
   These are exactly the controls P52-03 to P52-08 rewrite, but until then the
   lock says phases 2–2.2 pass nothing.
4. **A stray line, `differing field: value`,** prints during `gate all` with no
   item attached. Source not yet identified.

## Consequence for P52-01

P52-01's done-when is "`cargo test --workspace --no-fail-fast` passes." Its
delivery list names only `capability.rs`, `adversary.rs` and `string_reason`.
The six other failing tests above are also inside that done-when, and the plan
does not say how to fix them. Per the ordering note ("if a prediction was wrong,
the review is corrected before P52-01"), the review's F52 needs an addendum
listing all nine before P52-01 starts.
