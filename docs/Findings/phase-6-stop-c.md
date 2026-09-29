# Phase 6 Stop C: end of chunk C (P6-11a to P6-14)

Recorded by Cursor on 28 Sep 2026 from `D:\JoInn` on Windows (PowerShell).
Values below are copied from the terminal, not summarised.

`git rev-parse HEAD` after P6-14 (last commit of chunk C):

```
9c2fc1f10e385a84750adae347eaf9926ff15868
```

The stop-report commit that adds this file is `P6-stop-c` on `main` immediately after that.

## Commit reports (in order)

```
Commit:     P6-11a dd37349e1e0d27ce3c28bbf92b4fb80633100017
Done-when:  git show --stat HEAD → 3 files changed, 42 insertions(+), 4 deletions(-); the plan, decisions.md and the backlog, nothing under joinn/crates or joinn/xtask          MET
Suite:      cargo test --workspace --no-fail-fast → 248 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      none
```

```
Commit:     P6-12 6d33697bee785b157c1daaa26ed9b8b66d27f38f
Done-when:  cargo build -p joinn-shell-desktop → Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.16s          MET locally
Suite:      cargo test --workspace --no-fail-fast → 252 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      P6-12's done-when also asks for the build on both OSes. This commit records the local build. CI is read at Stop C.
```

Window, click on the sum cell (from the commit; a display was available):

```
adapter: NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes
surface: Bgra8Unorm
pick 920,276: body.sum (cpu)
        body.sum (gpu) · agree
```

```
Commit:     P6-13 fde7dddd3a35cc487ef203547df13f6bd935fa84
Done-when:  cargo xtask gate all → phase 5.2: 3/3 / phase 6: 3/3 (exit 0)          MET
Suite:      cargo test --workspace --no-fail-fast → 252 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      none
```

Shown then reverted. The terminal printed:

```
distinct opposition: What the engine computes is a row the picture shows and A click names an address; a stale click is refused share (corpus/phase2/calculator.body, DropGenome("sum"))
```

```
no GPU adapter; acceptance is at least one adapter (on Linux, install mesa-vulkan-drivers for lavapipe)
1 fail  Every pixel has one owner, and both pickers name it
2 fail  What the engine computes is a row the picture shows
3 fail  A click names an address; a stale click is refused
phase 6: 0/3
```

```
event grow pixel 192,220 adapter Microsoft Basic Render Driver · Dx12 · Cpu · vertex storage yes: F2B134FF C9CED6FF
2 fail  What the engine computes is a row the picture shows
```

```
Commit:     P6-14 9c2fc1f10e385a84750adae347eaf9926ff15868
Done-when:  cargo xtask gate all → harness fixtures: ok; phase 5.2: 3/3; phase 6: 3/3; exit 0          MET
            cargo xtask corpus verify → corpus verify: 43 hash(es) match; cells admitted          MET
            git diff --stat 0ec763d..HEAD -- joinn/corpus → (no output)          MET
Suite:      cargo test --workspace --no-fail-fast → 252 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      none
```

## Command tails

- `cargo test --workspace --no-fail-fast` — last `test result:` line: `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s` (the last doc-test binary); summed over every `test result:` line: `252 passed, 0 failed` (exit 0)
- `cargo xtask gate all` — `gate all wall milliseconds: 372405` (exit 0). Before that the lock ends `phase 5.2: 3/3` then `phase 6: 3/3`. The run opens `harness fixtures: ok`.
- `cargo xtask corpus verify` — `corpus verify: 43 hash(es) match; cells admitted`
- `cargo xtask vocab` — `vocab: ok`
- `cargo xtask modules` — `modules: ok (enforced 14 crate(s))`
- `cargo xtask layers` — `layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)`
- `cargo xtask assay agree` — `assay agree: 38 subject(s) agree, 1 not measured; injected disagreements: 2 refused as truth violation (ok)`
- `cargo xtask adapters` — `adapters: 3`
- `cargo xtask pick` — `pick: 3 adapter(s), 12 subject(s) agree; planted disagreement: refused as truth violation (ok)`
- `cargo xtask regrow` — `regrow: 3 adapter(s); planted difference: refused (ok)`
- `git diff --stat 0ec763d..HEAD -- joinn/corpus` — no output

### Every `gate all` line containing `fail`

```
1 ok  references agree; a blind seal fails the item
```

### Every failing test name

none (suite on 9c2fc1f: 252 passed, 0 failed)

## CI

Newest run after the push of `9c2fc1f` (P6-11a through P6-14), read with `Invoke-RestMethod` on `https://api.github.com/repos/joneseysinno/JoInn/actions/runs?branch=main&per_page=3` and that run's `jobs_url`.

```
head_sha: 9c2fc1f10e385a84750adae347eaf9926ff15868
status: completed
conclusion: success
check (ubuntu-24.04): success
check (windows-latest): success
```

Run: https://github.com/joneseysinno/JoInn/actions/runs/36508892897

## Snags (all from the chunk)

- **P6-12**: the done-when also asks for `cargo build -p joinn-shell-desktop` on both OSes. The commit recorded the local Windows build. This stop's CI run built the workspace, including that binary, on ubuntu-24.04 and windows-latest, and both jobs concluded success.
- **P6-13**: the uniqueness refusal on the terminal was `DropGenome("sum")`. The commit message stored `DropGenome(""sum"")` because the PowerShell here-string doubled the quotes. The tree was reverted to `DropGenome("cli_b")` before the commit. The empty-adapter run printed the refusal line once per item, so items 2 and 3 failed as well (`phase 6: 0/3`); item 1 failed and did not skip.
