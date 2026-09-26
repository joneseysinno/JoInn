# Phase 4 Stop B2: end of Amendment B (P4-10a to P4-10d)

Recorded by Cursor on 26 Sep 2026 from `D:\JoInn` on Windows (PowerShell).
Values below are copied from the terminal, not summarised.

`git rev-parse HEAD` after P4-10d (last commit of Amendment B):

```
189edcdb7c1f563dca3157e34d8085d918d8a74d
```

The stop-report commit that adds this file is `P4-stop-b2` on `main` immediately after that.

## Commit reports (in order)

```
Commit:     P4-10a b5425b5
Done-when:  git show --stat HEAD lists only this plan → MET
Suite:      cargo test --workspace --no-fail-fast → 194 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

```
Commit:     P4-10b 14fdc9f
Done-when:  asker_reports_ignore_genome_instance_order → ok; cargo xtask assay agree → assay agree: 37 subject(s) agree, 1 not measured; injected disagreements: 2 refused as truth violation (ok); cargo xtask assay invariance → phenotype reader: refused at edit 1 on phase4/loop.universe (ok); assay --all identical apart from piece order → MET
lookup/asker lines:
phase52/adversary/ask.universe: regions: lookup 2 {answer} {question}
phase52/adversary/asker.body: regions: body 2 {answer} {question}
phase52/adversary/lookup.body: regions: body 2 {answer} {question}
phase52/adversary/lookup.universe: regions: lookup 2 {answer} {question}
Suite:      cargo test --workspace --no-fail-fast → 195 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

```
Commit:     P4-10c 5e7b59b
Done-when:  cargo xtask decoration → exit 0
control run label-only: same (ok)
control run input changed: differs (ok)
gate admission: admitted | admitted | same
insert: admitted | admitted | same
bind: admitted | admitted | same
assemble: admitted | admitted | same
check_link_types: admitted | admitted | same
check_law4: admitted | admitted | same
check_lenses: admitted | admitted | same
require/ensure: none declared | none declared | same
run: cli_a@0 in "5", cli_a@1 out 5, cli_b@0 in "3", cli_b@1 out 3, sum@0 in 2, sum@1 in 3, sum@2 out 5, fmt@0 in 5, fmt@1 out "5", cli_a@0 in "5", cli_a@1 out 5 | cli_a@0 in "5", cli_a@1 out 5, cli_b@0 in "3", cli_b@1 out 3, sum@0 in 2, sum@1 in 3, sum@2 out 5, fmt@0 in 5, fmt@1 out "5", cli_a@0 in "5", cli_a@1 out 5 | same
assay: H₁: 0 | H₁: 1 | differs
distinguishing: assay
→ MET
Suite:      cargo test --workspace --no-fail-fast → 195 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

```
Commit:     P4-10d 189edcd
Done-when:  Verdict: KEEP matches distinguishing: assay → MET
git grep -n "P4-adv" -- docs/Findings/decisions.md → docs/Findings/decisions.md:61:| P4-adv | decoration check | 4 | held | Findings/decoration-check.md |
git grep -n "decoration check →" -- docs also printed the plan at lines 398 and 418, and docs/Theory/JoInn Research Backlog.md:215:Phase 4: decoration check → KEEP, see Findings/decoration-check.md
Suite:      cargo test --workspace --no-fail-fast → 195 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

## Last line of each command (end of Amendment B, on 189edcd)

- `cargo test --workspace --no-fail-fast` — `195 passed, 0 failed` (sum of every `test result:` passed count, including doc-test zeros; exit 0)
- `cargo xtask gate all` — `gate all wall milliseconds: 322157` (exit 0)
- `cargo xtask corpus verify` — `corpus verify: 42 hash(es) match; cells admitted` (exit 0)
- `cargo xtask vocab` — `vocab: ok`
- `cargo xtask modules` — `modules: ok (enforced 11 crate(s))`
- `cargo xtask assay agree` — `assay agree: 37 subject(s) agree, 1 not measured; injected disagreements: 2 refused as truth violation (ok)` (exit 0)
- `cargo xtask assay invariance` — `phenotype reader: refused at edit 1 on phase4/loop.universe (ok)` (exit 0)

Phase lines from that `gate all`:

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

### Every `gate all` line containing `fail`

```
1 ok  references agree; a blind seal fails the item
```

### Every failing test name

none (suite on 189edcd: 195 passed, 0 failed)

## CI

Newest run after the push of `189edcd`, read with `Invoke-RestMethod` on `https://api.github.com/repos/joneseysinno/JoInn/actions/runs?branch=main&per_page=3` and that run's `jobs_url`.

```
head_sha: 189edcdb7c1f563dca3157e34d8085d918d8a74d
status: completed
conclusion: success
check (windows-latest): success
check (ubuntu-latest): success
```

Run: https://github.com/joneseysinno/JoInn/actions/runs/36273863810

## Snags (all from the chunk)

none
