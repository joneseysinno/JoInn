# Phase 7.1 Stop A: end of chunk A (P71-01 to P71-04)

Recorded by Cursor on 1 Oct 2026 from `D:\JoInn` on Windows (PowerShell).
Values below are copied from the terminal, not summarised.

`git rev-parse HEAD` after P71-04 (last commit of chunk A):

```
39fa0c41b2a2ede240ea063a7d58ff940b87fe8a
```

The stop-report commit that adds this file is `P71-stop-a` on `main` immediately after that.

## Commit reports (in order)

```
Commit:     P71-01 0b1dc860b8303d1c3c411c0747fe07c37b10a2fe
Done-when:  git show --stat HEAD → the plan, joinn/AGENTS.md, docs/Theory/JoInn Dimension.md, the backlog, decisions.md and the roadmap; nothing else under joinn/          MET
Suite:      cargo test --workspace --no-fail-fast → 323 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      "Claude outputs/JoInn Cells, Bodies and Forces.md" is untracked and not committed: it is an older copy of the committed docs/Theory/JoInn Cells, Bodies and Forces.md (git diff --no-index --stat: 18 deletions), not a doc edit.
```

`git status` before staging, and `git show --stat` of P71-01:

```
 M docs/Findings/decisions.md
 M "docs/Plans/JoInn Build Roadmap.md"
 M "docs/Theory/JoInn Research Backlog.md"
 M joinn/AGENTS.md
?? "Claude outputs/"
?? "docs/Plans/JoInn Phase 7.1 Implementation Plan.md"
?? "docs/Theory/JoInn Dimension.md"

 docs/Findings/decisions.md                        |  23 +-
 docs/Plans/JoInn Build Roadmap.md                 |  13 +-
 docs/Plans/JoInn Phase 7.1 Implementation Plan.md | 556 ++++++++++++++++++++++
 docs/Theory/JoInn Dimension.md                    | 383 +++++++++++++++
 docs/Theory/JoInn Research Backlog.md             |  63 ++-
 joinn/AGENTS.md                                   |  30 +-
 6 files changed, 1053 insertions(+), 15 deletions(-)
```

```
Commit:     P71-02 49514f0e106c8cd4055aa7dc0ad735f66e663b93
Done-when:  cargo test -p joinn-visual → scene::present::tests::the_wired_calculator_refuses_a_port_it_does_not_hold ... ok, scene::present::tests::the_contact_calculator_skips_interior_ports_and_writes_only_sum_2 ... ok          MET
            cargo xtask gate all → phase 0: pass … phase 6: 3/3, phase 7: 3/3 (every phase line as at Phase 7 Stop C); gate all wall milliseconds: 442858; exit 0          MET
            Compare-Object base-regrow.txt p02b-regrow.txt (cargo xtask regrow before and after) → prints nothing          MET
            Compare-Object base-pick.txt p02b-pick.txt, frame median lines excluded → prints nothing          MET
Suite:      cargo test --workspace --no-fail-fast → 325 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      the baseline is `cargo xtask regrow` and `cargo xtask pick` run on 0b1dc86 (P71-01; the same code as Phase 7 Stop C's ab46298), since Stop C did not paste those outputs whole. Compare-Object on the pick output, frame median lines included, prints only the three `frame median <n> ms (calculator 1280x720; information, not a check)` lines on each side (3.390/0.925/0.498 before, 3.339/0.868/0.493 after): wall time, which pick itself labels information, not a check.
```

The wired refusal, as the test asserts it:

```
present: port sum@9 is not in this scene; acceptance is a port of body body
```

```
Commit:     P71-03 c2493d80d964231a0ab48de4c660da1b60181586
Done-when:  cargo xtask forces → §2.1's seven lines byte for byte (1159 bytes, equal)          MET
            grep -rni "order.free" joinn/crates joinn/xtask joinn/AGENTS.md joinn/README.md docs/Guides → prints nothing (exit 1)          MET
            cargo xtask gate 7 → 2 ok  Combine is order-blind, and it fits what it reaches          MET
Suite:      cargo test --workspace --no-fail-fast → 325 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      none
```

`cargo xtask forces` after P71-03 (stdout captured as bytes and compared with §2.1's block: equal):

```
combine ℤ 1 by cell:6b32…: order-blind (64 pairs, 64 triples, seed 7), opposed by separate cell:6fcb… (turn 0 from {1 2})
planted: order_blind on mutant.difference: refused (ok): not order-blind: f(a, b) = 9223372039002259455 but f(b, a) = -9223372039002259455 at a = 9223372036854775807, b = -2147483648; acceptance is a response whose result does not depend on member order
planted: order_blind on mutant.midpoint: refused (ok): not order-blind: f(f(a, b), c) = -1535576763092620387 but f(a, f(b, c)) = -3841419772306314346 at a = -9223372036854775837, b = 3081064984484294291, c = 0; acceptance is a response whose result does not depend on member order
planted: order_blind on mutant.max: order-blind (ok), not registered
planted: order_blind on mutant.plus1: order-blind (ok), not registered
planted: check_register with separate = response: refused (ok): combine on ℤ 1 is unopposed: no separate; acceptance is a turn of cell:6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39
forces: 1 combine registered, 1 opposed; plants: difference refused, midpoint refused, max and plus1 order-blind but unregistered, unopposed refused (ok)
```

`cargo xtask gate 7` after P71-03, the table:

```
1 ok  Two forms, one truth
2 ok  Combine is order-blind, and it fits what it reaches
3 ok  The body is drawn as cells in contact
phase 7: 3/3
```

```
Commit:     P71-04 39fa0c41b2a2ede240ea063a7d58ff940b87fe8a
Done-when:  grep -rn "alleles.is_empty() && !cell.alleles.is_empty()" crates xtask → crates/joinn-dna/src/model/keep_first_with_alleles.rs:13:        Some(prev) => prev.alleles.is_empty() && !cell.alleles.is_empty(),          MET
            cargo test --workspace → 329 passed, 0 failed          MET
            Compare-Object of cargo xtask contact (28 lines) and cargo xtask forces (7 lines), after P71-03 and after P71-04 → prints nothing          MET
Suite:      cargo test --workspace --no-fail-fast → 329 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      none
```

## Last line of each command (end of chunk A, on 39fa0c4)

- `cargo test --workspace --no-fail-fast` — summed over every `test result:` line: `329 passed, 0 failed` (exit 0)
- `cargo xtask gate all` — `gate all wall milliseconds: 416830` (exit 0)
- `cargo xtask corpus verify` — `corpus verify: 44 hash(es) match; cells admitted` (exit 0)
- `cargo xtask vocab` — `vocab: ok` (exit 0)
- `cargo xtask modules` — `modules: ok (enforced 14 crate(s))` (exit 0)
- `cargo xtask layers` — `layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)` (exit 0)
- `cargo xtask floor` — `floor: 16 members, paired` (exit 0)
- `cargo xtask forces` — `forces: 1 combine registered, 1 opposed; plants: difference refused, midpoint refused, max and plus1 order-blind but unregistered, unopposed refused (ok)` (exit 0)
- `cargo xtask contact` — `contact: 1 body(ies); two forms, one truth` (exit 0)
- `cargo xtask pick` — `pick: 3 adapter(s), 12 subject(s) agree; planted disagreement: refused as truth violation (ok)` (exit 0)
- `cargo xtask regrow` — `regrow: 3 adapter(s); planted difference: refused (ok)` (exit 0)

Phase lines from `gate all` on 39fa0c4 (the lock text it printed):

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
phase 6: 3/3
phase 7: 3/3
```

`git status` after `gate all` shows only the untracked `Claude outputs/`: `gates.lock` did not change.

### Every `gate all` line containing `fail`

```
1 ok  references agree; a blind seal fails the item
```

### Every failing test name

none (suite on 39fa0c4: 329 passed, 0 failed)

## CI

Newest run after the push of `39fa0c4` (P71-01 to P71-04, pushed together as `5c598f1..39fa0c4`), read with `Invoke-RestMethod` on `https://api.github.com/repos/joneseysinno/JoInn/actions/runs?branch=main&per_page=3` and that run's `jobs_url`. The first read after the push printed `status in_progress` (ubuntu `completed success`, windows `in_progress`); this is the read at 18:46 after it finished.

```
head_sha: 39fa0c41b2a2ede240ea063a7d58ff940b87fe8a
status: completed
conclusion: success
check (ubuntu-24.04): success
check (windows-latest): success
```

Every step of both jobs completed `success`, except the windows job's `lavapipe` step, which is `skipped` (the Linux-only software Vulkan install).

Run: https://github.com/joneseysinno/JoInn/actions/runs/36946248464

## Snags (all from the chunk)

- **P71-01**: `Claude outputs/JoInn Cells, Bodies and Forces.md` is untracked and was not committed. It is an older copy of the committed `docs/Theory/JoInn Cells, Bodies and Forces.md` (`git diff --no-index --stat`: 18 deletions), not a doc edit.
- **P71-02**: the regrow and pick baseline was taken on `0b1dc86` (P71-01, the same code as Phase 7 Stop C's `ab46298`), because Stop C did not paste those outputs whole. With frame median lines included, pick's Compare-Object prints only the three wall-time `frame median <n> ms (calculator 1280x720; information, not a check)` lines on each side (3.390/0.925/0.498 before, 3.339/0.868/0.493 after); without them it prints nothing. regrow's prints nothing.
- **P71-02**: the first P71-02 commit (`2e286db`) failed `cargo fmt --all -- --check` on one assert in its own test. It was amended before any push, with only that file re-wrapped by rustfmt; the pushed P71-02 is `49514f0`.
- **Stop A**: the `git push origin main` of P71-01 to P71-04 was held for AJ's approval (a protected push) and went through after it: `5c598f1..39fa0c4  main -> main`.
