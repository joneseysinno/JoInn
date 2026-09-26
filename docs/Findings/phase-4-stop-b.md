# Phase 4 Stop B: end of Chunk B (P4-07a to P4-10)

Recorded by Cursor on 26 Sep 2026 from `D:\JoInn` on Windows (PowerShell).
Values below are copied from the terminal, not summarised.

`git rev-parse HEAD` after P4-10 (last commit of Chunk B):

```
25226fac81002f3ba0e600277af18e7f8d89d364
```

The stop-report commit that adds this file is `P4-stop-b` on `main` immediately after that.

## Commit reports (in order)

```
Commit:     P4-07a f151ff4
Done-when:  git show --stat HEAD → 1 file changed, 27 insertions(+), 4 deletions(-) → MET
Suite:      cargo test --workspace --no-fail-fast → 193 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

```
Commit:     P4-07b 2b1532c
Done-when:  cargo xtask assay invariance → the lines below (exit 1) → MET
phase2/calculator.body: 1 ok, allele strip ok, 3 n/a (no lens), 4 n/a (body)
phase2/variants/calculator_b.body: 1 ok, 2 ok, 3 n/a (no lens), 4 n/a (body)
phase2/variants/calculator_c.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase2/variants/calculator_d.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase21/int_add_ref.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase21/int_format_ref.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase21/int_mul_ref.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase21/rat_add_ref.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase21/text_parse_ref.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase22/int_format_ref.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase22/int_mul_ref.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase22/rat_add_ref.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase22/text_parse_ref.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase3/columns_reader.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase3/environment.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase4/fmt.body: 1 ok, 2 ok, 3 n/a (no lens), 4 n/a (body)
phase4/fmt_twin.body: 1 ok, 2 ok, 3 n/a (no lens), 4 n/a (body)
phase4/loop.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phase5/adversary.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phase5/alone.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phase5/bus.body: 1 ok, 2 ok, 3 n/a (no lens), 4 n/a (body)
phase5/controls/alias_is_local.universe: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 ok
phase5/controls/echo.body: 1 ok, 2 ok, 3 n/a (no lens), 4 n/a (body)
phase5/controls/frame_mismatch.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phase5/controls/no_such_port.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phase5/controls/transits.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phase5/controls/two_systems.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phase5/controls/wrong_container.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phase5/controls/wrong_direction.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phase5/ordered.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phase5/two_in_ports.body: 1 ok, 2 ok, 3 n/a (no lens), 4 n/a (body)
phase5/units.body: 1 ok, 2 ok, 3 n/a (no lens), 4 n/a (body)
phase5/universe.universe: 1 ok, 2 ok, 3 ok, 4 ok
phase52/adversary/ask.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phase52/adversary/asker.body: 1 ok, 2 ok, 3 n/a (no lens), 4 n/a (body)
phase52/adversary/lookup.body: 1 ok, 2 ok, 3 n/a (no lens), 4 n/a (body)
phase52/adversary/lookup.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phenotype reader: accepted at edit 1; label counts 1 and 1
Suite:      cargo test --workspace --no-fail-fast → 194 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      Reprint sorts genome instances, so the body in the fresh store keeps the coding that was assayed and takes the regulatory region from the reprint. Inserting the reprinted coding moved asker.body from `regions: body 2 {question} {answer}` to `regions: body 2 {answer} {question}`. The command exits 1 because the phenotype reader still appends a label count; P4-07c replaces that reader.
```

```
Commit:     P4-07c b62bd85
Done-when:  cargo xtask assay invariance → exit 0, ending
phenotype reader: refused at edit 1 on phase2/calculator.body (ok)
phenotype reader: refused at edit 1 on phase4/loop.universe (ok)
→ MET
Whole output:
phase2/calculator.body: 1 ok, allele strip ok, 3 n/a (no lens), 4 n/a (body)
phase2/variants/calculator_b.body: 1 ok, 2 ok, 3 n/a (no lens), 4 n/a (body)
phase2/variants/calculator_c.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase2/variants/calculator_d.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase21/int_add_ref.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase21/int_format_ref.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase21/int_mul_ref.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase21/rat_add_ref.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase21/text_parse_ref.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase22/int_format_ref.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase22/int_mul_ref.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase22/rat_add_ref.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase22/text_parse_ref.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase3/columns_reader.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase3/environment.body: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 n/a (body)
phase4/fmt.body: 1 ok, 2 ok, 3 n/a (no lens), 4 n/a (body)
phase4/fmt_twin.body: 1 ok, 2 ok, 3 n/a (no lens), 4 n/a (body)
phase4/loop.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phase5/adversary.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phase5/alone.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phase5/bus.body: 1 ok, 2 ok, 3 n/a (no lens), 4 n/a (body)
phase5/controls/alias_is_local.universe: 1 ok (added name), 2 ok, 3 n/a (no lens), 4 ok
phase5/controls/echo.body: 1 ok, 2 ok, 3 n/a (no lens), 4 n/a (body)
phase5/controls/frame_mismatch.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phase5/controls/no_such_port.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phase5/controls/transits.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phase5/controls/two_systems.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phase5/controls/wrong_container.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phase5/controls/wrong_direction.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phase5/ordered.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phase5/two_in_ports.body: 1 ok, 2 ok, 3 n/a (no lens), 4 n/a (body)
phase5/units.body: 1 ok, 2 ok, 3 n/a (no lens), 4 n/a (body)
phase5/universe.universe: 1 ok, 2 ok, 3 ok, 4 ok
phase52/adversary/ask.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phase52/adversary/asker.body: 1 ok, 2 ok, 3 n/a (no lens), 4 n/a (body)
phase52/adversary/lookup.body: 1 ok, 2 ok, 3 n/a (no lens), 4 n/a (body)
phase52/adversary/lookup.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok
phenotype reader: refused at edit 1 on phase2/calculator.body (ok)
phenotype reader: refused at edit 1 on phase4/loop.universe (ok)
Shown then reverted (bodies bound against the original store): phenotype reader: accepted at edit 1 on phase2/calculator.body
Suite:      cargo test --workspace --no-fail-fast → 194 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

```
Commit:     P4-07d 7abb095
Done-when:  cargo xtask assay agree → exit 0
phase2/calculator.body: agree
phase2/variants/calculator_b.body: agree
phase2/variants/calculator_c.body: agree
phase2/variants/calculator_d.body: agree
phase21/int_add_ref.body: agree
phase21/int_format_ref.body: agree
phase21/int_mul_ref.body: agree
phase21/rat_add_ref.body: agree
phase21/text_parse_ref.body: agree
phase22/int_format_ref.body: agree
phase22/int_mul_ref.body: agree
phase22/rat_add_ref.body: agree
phase22/text_parse_ref.body: agree
phase3/columns_reader.body: agree
phase3/environment.body: agree
phase4/fmt.body: agree
phase4/fmt_twin.body: agree
phase4/loop.universe: agree
phase5/adversary.universe: agree
phase5/alone.universe: agree
phase5/bus.body: agree
phase5/controls/alias_is_local.universe: agree
phase5/controls/echo.body: agree
phase5/controls/frame_mismatch.universe: agree
phase5/controls/no_such_port.universe: agree
phase5/controls/transits.universe: agree
phase5/controls/two_systems.universe: agree
phase5/controls/wrong_container.universe: agree
phase5/controls/wrong_direction.universe: agree
phase5/ordered.universe: agree
phase5/two_in_ports.body: agree
phase5/units.body: agree
phase5/universe.universe: agree
phase52/adversary/ask.universe: agree
phase52/adversary/asker.body: agree
phase52/adversary/lookup.body: agree
phase52/adversary/lookup.universe: agree
phase52/controls/missing_cell.body: not measured: instance orphan cell 4a37 was not supplied; acceptance is that cell in the cell map
assay agree: 37 subject(s) agree, 1 not measured; injected disagreements: 2 refused as truth violation (ok)
→ MET
Suite:      cargo test --workspace --no-fail-fast → 194 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

```
Commit:     P4-08 ca19ef2
Done-when:  cargo xtask decoration → exit 0
gate admission: admitted | admitted | same
insert: admitted | admitted | same
bind: admitted | admitted | same
assemble: admitted | admitted | same
check_link_types: admitted | admitted | same
check_law4: admitted | admitted | same
check_lenses: admitted | admitted | same
require/ensure: none declared | none declared | same
run: description {   cell c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e   instance cli_a   label "cli_a"   role cell   port 0 in Text 1 "line" label "line" role input value "5"   port 1 out ℤ 1 "value" label "value" role output value 5 }  || description {   cell c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e   instance cli_b   label "cli_b"   role cell   port 0 in Text 1 "line" label "line" role input value "3"   port 1 out ℤ 1 "value" label "value" role output value 3 }  || description {   cell 6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39   instance sum   label "Sum"   role cell   port 0 in ℤ 1 "a" label "first addend" role input value 2   port 1 in ℤ 1 "b" label "second addend" role input value 3   port 2 out ℤ 1 "result" label "sum" role output value 5 }  || description {   cell 3b0ab2bca11406a7e3bb3c1c4fd78e95af213f82fe2c1c5c6ffc0d4ec76e9c11   instance fmt   label "Format"   role cell   port 0 in ℤ 1 "n" label "n" role input value 5   port 1 out Text 1 "text" label "text" role output value "5" }  || description {   cell c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e   instance cli_a   label "cli_a"   role cell   port 0 in Text 1 "line" label "line" role input value "5"   port 1 out ℤ 1 "value" label "value" role output value 5 }  | description {   cell c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e   instance cli_a   label "cli_a"   role cell   port 0 in Text 1 "line" label "line" role input value "5"   port 1 out ℤ 1 "value" label "value" role output value 5 }  || description {   cell c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e   instance cli_b   label "cli_b"   role cell   port 0 in Text 1 "line" label "line" role input value "3"   port 1 out ℤ 1 "value" label "value" role output value 3 }  || description {   cell 6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39   instance sum   label "Sum"   role cell   port 0 in ℤ 1 "a" label "first addend" role input value 2   port 1 in ℤ 1 "b" label "second addend" role input value 3   port 2 out ℤ 1 "result" label "sum" role output value 5 }  || description {   cell fcc1ba589d125d8b490c631510dcec4d86d65a9e1f032ecb8d254b692999b85d   instance fmt   label "Format"   role cell   port 0 in ℤ 1 "n" label "n" role input value 5   port 1 out Text 1 "text" label "text" role output value "5" }  || description {   cell c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e   instance cli_a   label "cli_a"   role cell   port 0 in Text 1 "line" label "line" role input value "5"   port 1 out ℤ 1 "value" label "value" role output value 5 }  | differs
assay: H₁: 0 | H₁: 1 | differs
distinguishing: run, assay
→ MET
Suite:      cargo test --workspace --no-fail-fast → 194 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      The plan predicted identical run reports and a distinguishing line of exactly assay. The descriptions differ in the fmt cell: subject 3b0ab2bca11406a7e3bb3c1c4fd78e95af213f82fe2c1c5c6ffc0d4ec76e9c11, mutant fcc1ba589d125d8b490c631510dcec4d86d65a9e1f032ecb8d254b692999b85d. Port values on both sides are 5 and "5". The last line is `distinguishing: run, assay`.
```

```
Commit:     P4-09 c15dea6
Done-when:  the three outputs below → MET
C2 factor:
joinn/corpus/phase5/units.body:15:  prompts { scale "factor: " }
joinn/corpus/transcripts/universe.txt:6:factor: 12
C2 laws:
(no lines; git grep exit 1)
C3:
phase2/calculator.body: 1 ok, allele strip ok, 3 n/a (no lens), 4 n/a (body)
Suite:      cargo test --workspace --no-fail-fast → 194 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      none
```

```
Commit:     P4-10 25226fa
Done-when:  git grep -n "R65" -- docs shows docs/Findings/decisions.md:57 and docs/Theory/JoInn Research Backlog.md:297; Verdict: CUT → MET
Suite:      cargo test --workspace --no-fail-fast → 194 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 11 crate(s))
Snags:      The plan's prediction was KEEP. The distinguishing line is `run, assay`, so the rule says CUT.
```

## Last line of each command (end of Chunk B, on 25226fa)

- `cargo test --workspace --no-fail-fast` — `194 passed, 0 failed` (sum of every `test result:` passed count, including doc-test zeros; exit 0)
- `cargo xtask gate all` — `gate all wall milliseconds: 350960` (exit 0)
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

none (suite on 25226fa: 194 passed, 0 failed)

## CI

Newest run after the push of `25226fa`, read with `Invoke-RestMethod` on `https://api.github.com/repos/joneseysinno/JoInn/actions/runs?branch=main&per_page=3` and that run's `jobs_url`.

```
head_sha: 25226fac81002f3ba0e600277af18e7f8d89d364
status: completed
conclusion: success
check (ubuntu-latest): success
check (windows-latest): success
```

Run: https://github.com/joneseysinno/JoInn/actions/runs/36268593340

## Snags (all from the chunk)

| Commit | Snag |
|---|---|
| P4-07b | Reprint sorts genome instances, so the body in the fresh store keeps the coding that was assayed and takes the regulatory region from the reprint. Inserting the reprinted coding moved asker.body from `regions: body 2 {question} {answer}` to `regions: body 2 {answer} {question}`. The invariance command exited 1 with `phenotype reader: accepted at edit 1; label counts 1 and 1`. P4-07c replaces that reader. |
| P4-08 | The plan predicted identical run reports and a distinguishing line of exactly `assay`. The descriptions differ in the fmt cell: subject `3b0ab2bca11406a7e3bb3c1c4fd78e95af213f82fe2c1c5c6ffc0d4ec76e9c11`, mutant `fcc1ba589d125d8b490c631510dcec4d86d65a9e1f032ecb8d254b692999b85d`. Port values on both sides are 5 and "5". The last line is `distinguishing: run, assay`. |
| P4-10 | The plan's prediction was KEEP. The distinguishing line is `run, assay`, so the rule says CUT. |
