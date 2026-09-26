# Phase 4 Stop A: end of Chunk A (P4-01 to P4-07)

Recorded by Cursor on 26 Sep 2026 from `D:\JoInn` on Windows (PowerShell).
Values below are copied from the terminal, not summarised.

`git rev-parse HEAD` after P4-07 (last code commit of Chunk A):

```
be0ad436c16a3c761955137f2e242f5768f7df94
```

The stop-report commit that adds this file is `P4-stop-a` on `main` immediately after that.

## Commit reports (in order)

```
Commit:     P4-01 c2b4f5e
Done-when:  git show --stat HEAD lists the plan, AGENTS.md, decisions.md and the backlog, and nothing under joinn/crates → MET
Suite:      cargo test --workspace --no-fail-fast → 185 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

```
Commit:     P4-02 4b47cf6
Done-when:  cargo test -p joinn-assay → test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s → MET
Suite:      cargo test --workspace --no-fail-fast → 191 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

```
Commit:     P4-03 d249a03
Done-when:  cargo xtask corpus verify → corpus verify: 42 hash(es) match; cells admitted → MET
Suite:      cargo test --workspace --no-fail-fast → 191 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

```
Commit:     P4-04 97d9bd4
Done-when:  cargo xtask assay on each §3.2 subject → reports matching §3.2; calculator.body prints §2.4 byte for byte → MET
Suite:      cargo test --workspace --no-fail-fast → 192 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

```
Commit:     P4-05 4316ed1
Done-when:  cargo xtask assay agree → assay agree: 37 subject(s) agree; injected disagreement: refused as truth violation (ok) → MET
Suite:      cargo test --workspace --no-fail-fast → 193 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      phase52/controls/missing_cell.body binds, and both derivations refuse `instance orphan cell 4a37 was not supplied; acceptance is that cell in the cell map`, so it has no agree line. Chain::from_coeffs now sums a repeated block, so a link inside one region is a cycle; wrong_container.universe agrees.
```

```
Commit:     P4-06 d01209f
Done-when:  cargo xtask assay invariance → phenotype reader: accepted at edit 1; label counts 1 and 1 → NOT MET
Suite:      cargo test --workspace --no-fail-fast → 193 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      The neutral edit changes one label's text and leaves the count at 1. A fake assay that appends that count prints the same report, so the harness accepts it. The required line `phenotype reader: refused at edit 1 (ok)` was not printed. Edits 1–4 held, including `phase2/calculator.body: 1 ok, allele strip ok, 3 ok, 4 ok`.
```

```
Commit:     P4-07 be0ad43
Done-when:  cargo xtask assay --all → phase52/controls/missing_cell.body: instance orphan cell 4a37 was not supplied; acceptance is that cell in the cell map → MET
Suite:      cargo test --workspace --no-fail-fast → 193 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

## Last line of each command (end of Chunk A, on be0ad43)

- `cargo test --workspace --no-fail-fast` — `193 passed, 0 failed` (sum of every `test result:` passed count, including doc-test zeros; exit 0)
- `cargo xtask gate all` — `gate all wall milliseconds: 427079` (exit 0)
- `cargo xtask corpus verify` — `corpus verify: 42 hash(es) match; cells admitted`
- `cargo xtask vocab` — `vocab: ok`
- `cargo xtask modules` — `modules: ok (enforced 11 crate(s))`
- `cargo xtask assay agree` — `assay agree: 37 subject(s) agree; injected disagreement: refused as truth violation (ok)`

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

none (suite on be0ad43: 193 passed, 0 failed)

## CI

Newest run after the push of `be0ad43`, read with `Invoke-RestMethod` on `https://api.github.com/repos/joneseysinno/JoInn/actions/runs?branch=main&per_page=3` and that run's `jobs_url`.

```
head_sha: be0ad436c16a3c761955137f2e242f5768f7df94
status: completed
conclusion: success
check (windows-latest): success
check (ubuntu-latest): success
```

Run: https://github.com/joneseysinno/JoInn/actions/runs/36263309262

## Snags (all from the chunk)

| Commit | Snag |
|---|---|
| P4-05 | `phase52/controls/missing_cell.body` binds, and both derivations refuse `instance orphan cell 4a37 was not supplied; acceptance is that cell in the cell map`, so it has no agree line. `Chain::from_coeffs` now sums a repeated block, so a link inside one region is a cycle. |
| P4-06 | The neutral edit changes label text and not the number of labels (both counts printed `1`). The fake assay that appends that count was accepted. Required line `phenotype reader: refused at edit 1 (ok)` was not printed. Printed: `phenotype reader: accepted at edit 1; label counts 1 and 1`. |
