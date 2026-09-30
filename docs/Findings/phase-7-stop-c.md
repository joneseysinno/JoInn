# Phase 7 Stop C: end of chunk C (P7-11a to P7-16)

Recorded by Cursor on 30 Sep 2026 from `D:\JoInn` on Windows (PowerShell).
Values below are copied from the terminal, not summarised.

`git rev-parse HEAD` after P7-16 (last commit of chunk C):

```
ab46298a79cce9e9f446b2eb949bc84be9804f5f
```

The stop-report commit that adds this file is `P7-stop-c` on `main` immediately after that.

## Commit reports (in order)

```
Commit:     P7-11a 6adb959b8331681f3e7f8f67c2e083ca1b711470
Done-when:  cargo test -p joinn-link check_contact → a_force_that_reaches_itself_is_refused ... ok (s → s; s → t → s), a_chain_of_forces_is_admitted_and_lowers ... ok          MET
            cargo xtask contact → the same lines as Stop B, byte for byte          MET
Suite:      cargo test --workspace --no-fail-fast → 315 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      none
```

```
Commit:     P7-12 834ff62ef687ebc21bebb959a9f9f5a4ade04011
Done-when:  cargo xtask layout phase7/calculator.contact → §2.8's block byte for byte (tests assert it)          MET
            cargo xtask pick → every contact line disagree 0, owners 7/7, links 0 (3 adapters × 4 viewports)          MET
            cargo xtask regrow → every contact event line ends tables equal regrow; rows 2, 1, 3          MET
            cargo xtask gate 6 → phase 6: 3/3          MET
Suite:      cargo test --workspace --no-fail-fast → 320 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      none
```

```
Commit:     P7-13 fe9bdf611191d4d11e02d543118f8c658e7a877e
Done-when:  cargo test -p joinn-shell-desktop → V125 passes on both files; calculator.contact rows 2, 1, 3          MET
            the window, click sum → pick 832,360: body.sum (cpu) / body.sum (gpu) · agree          MET
Suite:      cargo test --workspace --no-fail-fast → 322 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      the click was delivered as WM_MOUSEMOVE, WM_LBUTTONDOWN and WM_LBUTTONUP posted to the JoInn window at client (832,360), then WM_CLOSE; no hand was on the mouse. A first try that moved the system cursor landed on another window, because the JoInn window was not in front, and printed nothing. AJ's §6.2 check is still to do.
```

```
Commit:     P7-14 6dd6896f6780f00b29b2bf15d2d0924c6fa4826c
Done-when:  docs/Findings/phase-7-contact.md → every prediction in §2.3, §2.5, §2.7, §2.8, §2.9, §2.10 marked (all as predicted)          MET
Suite:      cargo test --workspace --no-fail-fast → 323 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      P7-14 is a findings commit, but §2.10's probes, seams and colors had no printed measurement before it, so it also adds one joinn-visual test that prints and asserts them. Its colors come from the tables and the shader's selection rule; the GPU color target at those probes is gate 7 item 3's check (P7-15).
```

```
Commit:     P7-15 e2093ce8ff56f705a897c203e402069a6365aec0
Done-when:  cargo xtask gate all → phase 6: 3/3, then phase 7: 3/3, exit 0          MET
            shown then reverted: (a) uniqueness refusal; (b) item 1 refused; (c) item 3 fails on the grow line's sum color (pasted in the commit and below)
Suite:      cargo test --workspace --no-fail-fast → 323 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      (b) differs from the plan (below, under Snags). mutate.rs's no_gate_row_names_a_contact_artifact_yet became
            only_gate_seven_names_a_contact_artifact: its premise ("yet") ends here. It still asserts no gate 1-6 row names a
            .contact, and now also that gate 7's three rows all name corpus/phase7/calculator.contact.
```

```
Commit:     P7-16 ab46298a79cce9e9f446b2eb949bc84be9804f5f
Done-when:  cargo xtask gate all, from `git clone D:\JoInn D:\JoInn-p7-clone` of 0d3fafd (this tree) → harness fixtures: ok; phase 0: pass … phase 6: 3/3; phase 7: 3/3; gate all wall milliseconds: 478502; exit 0          MET
            cargo xtask corpus verify (clone) → corpus verify: 44 hash(es) match; cells admitted          MET
            git diff --stat caeff0c..HEAD -- joinn/corpus → joinn/corpus/hashes.txt | 2 ++, joinn/corpus/phase7/calculator.contact | 23 +++ (P7-07's two files only)          MET
Suite:      cargo test --workspace --no-fail-fast → 323 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      decisions.md also gains V133 `holds` (Amendment B's invariant), beyond the plan's V125-V132 list. The guides'
            membrane → surface and Phase 7 → 7.1 wording fixes are outside the listed edits and change no meaning.
```

`0d3fafd` is P7-16 before its message was amended with the report; the tree is the same as `ab46298`.

## Gate 7 (P7-15)

`cargo xtask gate 7`, the table's lines:

```
harness fixtures: ok
calculator.contact 640x360: agree 230248, edge 152, disagree 0, owners 7/7, links 0
calculator.contact 1000x777: agree 776784, edge 216, disagree 0, owners 7/7, links 0
calculator.contact 1280x720: agree 921256, edge 344, disagree 0, owners 7/7, links 0
calculator.contact 1920x1080: agree 2073164, edge 436, disagree 0, owners 7/7, links 0
(the same four lines on each of the other two adapters)
1 ok  Two forms, one truth
2 ok  Combine is order-free, and it fits what it reaches
3 ok  The body is drawn as cells in contact
phase 7: 3/3
```

Item 2 also prints `cargo xtask forces`' seven lines (the register line, five plant lines, the summary).

Shown then reverted:

(a) item 2 given item 3's `opposes`, `cargo xtask gate 7`, exit 1:

```
distinct opposition: Combine is order-free, and it fits what it reaches and The body is drawn as cells in contact share (corpus/phase7/calculator.contact, DropForce("sum"))
```

(b) `lower` assigns members in reverse order, `cargo xtask gate 7`, exit 1:

```
gate item 1 (Two forms, one truth): control answered true on real subject
```

and `cargo xtask contact` under the same edit, exit 1 (the lines that differ from Stop B):

```
lowered differs from phase2/calculator.body at line 11: lowered `cli_a@1 -> sum@1`, phase2/calculator.body `cli_a@1 -> sum@0`
transcript differs from transcripts/calculator.txt at line 5:
a: two
   refused at membrane: "two" is not in ℤ
a: 2
b: 3
3 + 2 = 5

calculator.desc differs:
description {
  cell 6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39
  instance sum
  label "Sum"
  role cell
  port 0 in ℤ 1 "a" label "first addend" role input value 3
  port 1 in ℤ 1 "b" label "second addend" role input value 2
  port 2 out ℤ 1 "result" label "sum" role output value 5
}

surface equal (3 ports)
pairings: 2, sum@2 = 5 in each
contact: 3 failure(s)
```

(c) style 7 removed from the shader's `select` (`let latent = c.style;`), `cargo xtask gate 7`, exit 1:

```
grow pixel 832,360 adapter Microsoft Basic Render Driver · Dx12 · Cpu · vertex storage yes: sum 2F5D8AFF, want 39414DFF
event 1 "two" pixel 832,360 adapter Microsoft Basic Render Driver · Dx12 · Cpu · vertex storage yes: sum 2F5D8AFF, want 39414DFF
event 2 "2" pixel 832,360 adapter Microsoft Basic Render Driver · Dx12 · Cpu · vertex storage yes: sum 2F5D8AFF, want 39414DFF
grow pixel 832,360 adapter NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes: sum 2F5D8AFF, want 39414DFF
event 1 "two" pixel 832,360 adapter NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes: sum 2F5D8AFF, want 39414DFF
event 2 "2" pixel 832,360 adapter NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes: sum 2F5D8AFF, want 39414DFF
grow pixel 832,360 adapter NVIDIA GeForce RTX 2080 · Vulkan · DiscreteGpu · vertex storage yes: sum 2F5D8AFF, want 39414DFF
event 1 "two" pixel 832,360 adapter NVIDIA GeForce RTX 2080 · Vulkan · DiscreteGpu · vertex storage yes: sum 2F5D8AFF, want 39414DFF
event 2 "2" pixel 832,360 adapter NVIDIA GeForce RTX 2080 · Vulkan · DiscreteGpu · vertex storage yes: sum 2F5D8AFF, want 39414DFF
1 ok  Two forms, one truth
2 ok  Combine is order-free, and it fits what it reaches
3 fail  The body is drawn as cells in contact
phase 7: 2/3
```

After each revert, `git diff -- joinn/crates` printed nothing.

## Last line of each command (end of chunk C, on ab46298)

- `cargo test --workspace --no-fail-fast` — summed over every `test result:` line: `323 passed, 0 failed` (exit 0)
- `cargo xtask gate all` — `gate all wall milliseconds: 478502` (exit 0), from `git clone D:\JoInn D:\JoInn-p7-clone` of `0d3fafd` (the same tree), at `D:\JoInn-p7-clone\joinn`. In `D:\JoInn\joinn` after P7-15 it printed `gate all wall milliseconds: 469433` (exit 0).
- `cargo xtask corpus verify` — `corpus verify: 44 hash(es) match; cells admitted` (exit 0)
- `cargo xtask vocab` — `vocab: ok` (exit 0)
- `cargo xtask modules` — `modules: ok (enforced 14 crate(s))` (exit 0)
- `cargo xtask layers` — `layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)` (exit 0)
- `cargo xtask floor` — `floor: 16 members, paired` (exit 0)
- `cargo xtask assay agree` — `assay agree: 38 subject(s) agree, 1 not measured; injected disagreements: 2 refused as truth violation (ok)` (exit 0)
- `cargo xtask adapters` — `adapters: 3` (exit 0)
- `cargo xtask pick` — `pick: 3 adapter(s), 12 subject(s) agree; planted disagreement: refused as truth violation (ok)` (exit 0)
- `cargo xtask regrow` — `regrow: 3 adapter(s); planted difference: refused (ok)` (exit 0)
- `cargo xtask forces` — `forces: 1 combine registered, 1 opposed; plants: difference refused, midpoint refused, max and plus1 order-free but unregistered, unopposed refused (ok)` (exit 0)
- `cargo xtask contact` — `contact: 1 body(ies); two forms, one truth` (exit 0)
- `cargo xtask roles corpus/phase7/calculator.contact` — `roles: 3 cell(s); protect 2, carry 1, store 0, respond 0` (exit 0)

Phase lines from the fresh-clone `gate all` (the lock text it printed):

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

### Every `gate all` line containing `fail`

```
1 ok  references agree; a blind seal fails the item
```

### Every failing test name

none (suite on ab46298: 323 passed, 0 failed)

## CI

Newest run after the push of `ab46298` (P7-11a to P7-16, pushed together as `fe6ad5b..ab46298`), read with `Invoke-RestMethod` on `https://api.github.com/repos/joneseysinno/JoInn/actions/runs?branch=main&per_page=3` and that run's `jobs_url`. A first read just after the push and the reads at 15:56 and 16:06 printed `status in_progress` (at 16:06 the ubuntu job had finished `success`; windows was in `gate all`); this is the read at 16:12 after it finished.

```
head_sha: ab46298a79cce9e9f446b2eb949bc84be9804f5f
status: completed
conclusion: success
check (ubuntu-24.04): success
check (windows-latest): success
```

The `contact` and `gate all` steps passed on both jobs (`step contact: success`, `step gate all: success`); gate all there includes gate 7.

Run: https://github.com/joneseysinno/JoInn/actions/runs/36782019050

## Snags (all from the chunk)

- **P7-12**: none.
- **P7-13**: the window's click was delivered as posted messages (WM_MOUSEMOVE, WM_LBUTTONDOWN, WM_LBUTTONUP to the JoInn window at client 832,360, then WM_CLOSE), not by a hand on the mouse. The first try moved the system cursor and clicked while the JoInn window was behind AJ's browser, so **that click landed in AJ's browser window** (a YouTube page) and JoInn printed nothing. The posted click printed `pick 832,360: body.sum (cpu)`, `tick 1: rows 0, bytes 0`, `        body.sum (gpu) · agree`.
- **P7-14**: the findings commit also adds one joinn-visual test (`the_contact_picture_meets_the_probes_seams_and_colors`), because §2.10's probes, seams and colors had no printed measurement before it. Its colors come from the tables and the shader's selection rule; the GPU color target is gate 7 item 3's check.
- **P7-15 (b) differs from the plan.** Predicted: item 1 fails on the description and item 2 still passes. Actual: item 1's control compares the CLI transcript, which under reversed members prints `3 + 2 = 5` at line 5, so the control answers true on the real subject and grading refuses item 1 before its check runs. Gate 7 exits 1 on that line, and the table prints no ok/fail rows. Item 2's and item 3's checks ran and printed no failure. The description failure itself is shown by `cargo xtask contact` under the same edit (the same helpers as item 1's check): `calculator.desc differs` (port 0 value 3), while `pairings: 2, sum@2 = 5 in each` still holds. The gate was not changed to match the prediction.
- **P7-15**: mutate.rs's `no_gate_row_names_a_contact_artifact_yet` became `only_gate_seven_names_a_contact_artifact`. It still asserts that no gate 1–6 row names a `.contact`, and now also that gate 7's three rows all name `corpus/phase7/calculator.contact`.
- **P7-16**: `decisions.md` also gains V133 `holds` (Amendment B's invariant), beyond the plan's V125–V132 list. The guides' membrane → surface and Phase 7 → 7.1 wording fixes are outside the listed edits and change no meaning.
- **Stop C**: the first `git push origin main` was held for AJ's approval (a protected push) and went through after it: `fe6ad5b..ab46298  main -> main`.
- **Still to do: AJ's §6.2 window check.** Cursor ran the window once (P7-13) with a posted click; AJ has not yet run it.
- **All commits**: git normalises the report block's `Commit:`, `Done-when:`, `Suite:`, `Scans:`, `Snags:` lines as trailers, so `git log` shows them with one space after the colon. The blocks above restore §0.1's alignment; the values are unchanged.
