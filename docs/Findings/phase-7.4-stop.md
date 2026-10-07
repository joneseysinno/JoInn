# Phase 7.4 stop (Run 7.3–9 §4)

## HEAD

`git rev-parse HEAD` at the stop check: `aaf5fa5c3a3a17817154613ada3d3529498e57ae` (P74-15).

## Commits

- P74-01 45d868a plan, rules, docs: MET
- P74-F1 e8371f0 one gate registry: MET
- P74-F2 c9fab0c standing plants: MET
- P74-F3 562051a canonical order at admission: NOT MET (pick's grove lines changed where the plan predicted none; snag 1)
- P74-02 fa6e7cc the mutation catalogue first: MET
- P74-03 ab48797 grows: MET
- P74-04 626eaaa .system: MET
- P74-05 0428293 admission: MET
- P74-06 76f3328 growth and the engine: MET
- P74-07 0df0880 witness and evolution: MET
- P74-08 6ce1adb the corpus and grow: MET
- P74-09 3547079 the system layout and the lasso: MET
- P74-10 4d94c96 the system scene: MET
- P74-11 0596f31 picking the lasso: MET
- P74-12 cf20667 the shell grows a system: MET
- P74-13 dd11f95 findings: MET
- P74-14 ebbbaae gate 7.4: MET
- P74-15 aaf5fa5 docs and freeze: MET

## `cargo xtask stop-check --fresh`

```
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.46s
     Running `target\debug\xtask.exe stop-check --fresh`
stop-check: D:\JoInn\joinn\target\fresh\joinn at aaf5fa5 (fresh clone)
lavapipe: sudo apt-get update && sudo apt-get install -y mesa-vulkan-drivers → skipped (Linux only)
fmt: cargo fmt --all -- --check → exit 0 (6689 ms, information)
clippy: cargo clippy --workspace --all-targets -- -D warnings →     Finished `dev` profile [optimized + debuginfo] target(s) in 1m 05s (65980 ms, information)
test: cargo test --workspace --no-fail-fast → 486 passed, 0 failed (184265 ms, information)
vocab: cargo xtask vocab → vocab: ok (17829 ms, information)
modules: cargo xtask modules → modules: ok (enforced 14 crate(s)) (1147 ms, information)
layers: cargo xtask layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop) (731 ms, information)
corpus verify: cargo xtask corpus verify → corpus verify: 48 hash(es) match; cells admitted (780 ms, information)
agree: cargo xtask agree → injected disagreement: refused as truth violation (ok) (124960 ms, information)
assay agree: cargo xtask assay agree → assay agree: 38 subject(s) agree, 1 not measured; injected disagreements: 2 refused as truth violation (ok) (3694 ms, information)
pick: cargo xtask pick → pick: 3 adapter(s), 12 subject(s) agree; planted disagreement: refused as truth violation (ok) (93531 ms, information)
regrow: cargo xtask regrow → regrow: 3 adapter(s); planted difference: refused (ok) (72865 ms, information)
forces: cargo xtask forces → forces: 1 combine registered, 1 opposed; plants: difference refused, midpoint refused, max and plus1 order-blind but unregistered, unopposed refused (ok) (763 ms, information)
contact: cargo xtask contact → contact: 1 body(ies); two forms, one truth (1218 ms, information)
zoom: cargo xtask grove && cargo xtask zoom && cargo xtask links && cargo xtask grow → grow: 2 systems, every size true, counting witnesses every step; evolution holds; planted fold, lasso, hash: refused (ok) (7791 ms, information)
spike s8: cargo test --manifest-path spikes/s8-beam/Cargo.toml → 54 passed, 0 failed (7092 ms, information)
power: cargo xtask power → gate power: 20/20 (789 ms, information)
gate all: cargo xtask gate all → gate all wall milliseconds: 528347 (529259 ms, information)
stop-check: ok (1138540 ms, information)
```

stop-check prints only `gate all`'s last line. `gate all` in the main tree at P74-14 (`target/gate-all-14.txt`) printed phase 0 … phase 7.3 as `gates.lock` and `phase 7.4: 3/3`, with gate exit 0 and `gate all wall milliseconds: 522187`. The `says fail` line of the 7.3 stop does not appear.

## Predictions marked `differs`

None in the findings: every row of `phase-7.4-growth.md`'s Predictions is `as predicted` (or `(printed)` for `--measure`).

Outside the findings, one done-when prediction differed: P74-F3 predicted `pick` with every earlier line unchanged, and the grove's edge and link-owner counts changed (P74-F3 snag 1).

## Snags

- **P74-01**: Appendix B asks for decision rows G1–G19 from Part VII, but Part VII Draft 0.2 on disk lists G1–G16. G17–G19 are written from the 7.4 plan's own words, and each row's last column says so.
- **P74-F1 (1)**: the first check failed on fmt (exit 1); `cargo fmt --all`, then check ok. gate all ran before the fmt (whitespace only).
- **P74-F1 (2)**: the plan's "test asserts the cross-gate checks' tables equal gate_table's" is `cross_gate_tables_are_the_registry_s_tables` in `opposed_tables.rs`.
- **P74-F2 (1)**: gate 7.3's three controls all answer by admission (as the 7.3 review found). Links' standing plants oppose its truths, and the 7.4 findings name them, because 7.3's findings are closed.
- **P74-F2 (2)**: other items answered by admission: gate 4 item 4, gate 5 items 3, 4, 6, gate 5.1 items 2, 3, gate 5.2 item 1, gate 7 items 1, 2, gate 7.2 items 2, 3.
- **P74-F3 (1)**: pick's grove lines differ. Where links overlap, the link drawn last owns the pixel, and the last link is now the last by id. The plan predicted no change.
- **P74-F3 (2)**: shell_session's leg click: the leg of sys_g0s00 now serves member index 4 (was 3), as F3 says. Its expected `(976, 556, 4)` and "member 3" are now `(976, 556, 5)` and "member 4", still exact.
- **P74-F3 (3)**: gate 7.3 item 3 failed on the first run (`folded pick 960,540: cpu names link uni; acceptance is the leg's link, member index 4`). The check now picks a point on the longest member leg (its midpoint, a quarter point or an eighth point) that is more than 64 sixteenths of a unit from every other open piece. The failed run wrote `phase 7.3: 2/3` into gates.lock; `git checkout` restored it before the passing run.
- **P74-02 (1)**: the data types land here, ahead of the grammar (P74-03, P74-04). The tests apply each mutation directly. Until P74-04, reparse refuses a system: `a system has no canonical text yet; acceptance is a body, contact or universe`.
- **P74-02 (2)**: `Subject::System` carried `#[allow(dead_code)]` until P74-04's parse_subject built one, because clippy refused it as never constructed.
- **P74-03**: P74-02 was committed before its gate all finished, and rewriting it was refused. Its run, on fa6e7cc in a worktree: exit 0, every line as gates.lock, `gate all wall milliseconds: 522305`, nothing changed against P74-F3's run. From then on gate all ran before each commit.
- **P74-04**: the two trybuild fixtures `hash_regulatory.stderr` and `hash_body_regulatory.stderr` were regenerated (`TRYBUILD=overwrite`). With a fifth Genotype implementor, rustc prints a help list instead of the five impl sites. The error is unchanged (E0277).
- **P74-05 (1)**: check_system takes a `systems` map beside §2.4's arguments, because rule 7 needs the parent system itself.
- **P74-05 (2)**: rule 7's evolution refusal is tested by editing the child contact's grows cell in the struct, because the grammar admits only the input cell in Phase 7.4.
- **P74-06 (1)**: grow takes no frames argument; it admits with `FrameRegistry::phase1()`, as the corpus does.
- **P74-06 (2)**: the engine's BodyState cannot add instances. A growth step is therefore a delta on the grown state (grow_step), and the engine runs the lowered body of the state it reaches. Regrow equals it byte for byte.
- **P74-07 (1)**: check_evolution returns `Verdict<Evolution>` (the witnesses held and the gained input) instead of `Verdict<()>`, so `grow` can print its evolve line.
- **P74-07 (2)**: the refusal words name the accept word ("counting", "adding"), not a regulatory display name.
- **P74-07 (3)**: the first worktree run built stale: copied files kept older mtimes, so P74-06's tests ran. It was stopped and the files touched; every pasted line is from the rebuilt run.
- **P74-08 (1)**: a growing contact is a system's body, not a standalone contact. corpus_contacts and `cargo xtask contact` skip it (contact still reports 1 body), and system admission reads it through corpus_growing_contacts.
- **P74-08 (2)**: line forms: an empty transcript prints `grow <system> 0: (none) → …`, and a negative input prints `+-2`.
- **P74-08 (3)**: the planted lasso was printed `lasso —` in grow's last line until P74-09 planted it.
- **P74-09**: the lasso plant moves the whole outline, not one stroke. Drawn 2 units inward on counting grown by 1 1 1 (n 3), stroke 0 meets the surface (rule 1).
- **P74-10 (1)**: FORCE_TAG and STYLE_FORCE are defined here, not in P74-11, because the lasso's Stroke rows carry them.
- **P74-10 (2)**: a waiting box is a cell row flagged LATENT, with its in-port LIVE and not FILLED; no new flag.
- **P74-10 (3)**: the style-table test now expects 16 × 4 bytes; no earlier style moved.
- **P74-10 (4)**: the scene's only text is the count on the response's out-port.
- **P74-11 (1)**: `cut allows` on a system line is `SystemScene::allows`, not the universe cut, because a system is not in a universe until Phase 7.5. The owner count leaves the force out; the force has its own count.
- **P74-11 (2)**: seven_inputs moved from regrow's system pass into grow.rs, so pick and regrow grow by the same inputs.
- **P74-11 (3)**: pick's last line still counts 12 subjects. The system lines are checked and fail on their own.
- **P74-12 (1)**: text that is not an integer reaches the body as a Text value, so the body's own words refuse it; the shell adds no words.
- **P74-12 (2)**: after a growth the next waiting box is selected.
- **P74-12 (3)**: the camera stays where it was after a growth; the session test calls the view's home before each click. The message says "Home refits". The shell has no Home key; the key is `F`, which calls home. P74-15 corrected the findings' wording.
- **P74-12 (4)**: the window's first paint came after the posted input, so its one tick is the first upload, every row of the grown scene (38), not a delta.
- **P74-13 (1)**: the measure line's form is `grow <system> µs per step at n = 0, 6, 24, 96: <t…> (engine <e…>)`, not §3's `grow µs <t> per step at n = …`.
- **P74-13 (2)**: the engine is timed apart from the step, so at n = 96 its time can print a little above the step's.
- **P74-13 (3)**: the findings' dev and release tables come from main-tree runs before the commit's worktree run. They differ by noise (a few %).
- **P74-14 (1)**: demo (a) is refused by the mutation itself (`numbers already accepts any; acceptance is the other word`) before the control is asked.
- **P74-14 (2)**: demo (b) fails item 1 at grading (`control answered true on real subject`) before the check runs.
- **P74-14 (3)**: a failure line prints the CPU owner raw (`cpu Owned([1, 1, 0, 1]), gpu force count`); the line after it names the body (count) in words.
- **P74-14 (4)**: item 2's check makes "a counting with lineage and nothing new" with the catalogue mutation `Accepts("numbers", One)` on adding.system, the same edit its control opposes.
- **P74-15 (1)**: hashes.txt shows 5 insertions for four hashes; the fifth is the comment line above them, and no existing line changed.
- **P74-15 (2)**: the findings' camera paragraph was corrected at P74-15 (Home → `F`), after P74-13 committed it.

## CI

Read before this stop's push (`Invoke-RestMethod "https://api.github.com/repos/joneseysinno/JoInn/actions/runs?branch=main&per_page=3"`):

```
37428324308 14110cc completed success 10/06/2026 07:13:22
37392421907 0480723 completed success 10/06/2026 00:08:44
37391259727 a44e2d9 completed success 10/05/2026 23:55:53
```

- Run 37428324308 (`head_sha` 14110cc, P73-stop's ledger line): `check (windows-latest): completed success`, `check (ubuntu-24.04): completed success`.

P74-01 … P74-15 and this report are pushed at this stop. Their CI run is read once after the push, without waiting.

## Tripwires

None fired.

- `git diff --stat 45d868a..HEAD -- joinn/corpus` lists only the four phase74 files and `hashes.txt` (69 insertions, no line changed).
- `gate all` exits 0 with every earlier phase as `gates.lock` and `phase 7.4: 3/3`.
- The suite passes (486 passed, 0 failed).
- No `gate all` line other than timing lines changed from P74-13's run to P74-14's; the added lines are gate 7.4's.
- No snag text repeats across three consecutive commits.

AJ's window check (§6.2) waits for the review.

Ledger: `| 4 | 7.4 | aaf5fa5c3a3a17817154613ada3d3529498e57ae | phase 7.4: 3/3 | 486 passed, 0 failed | 43 | yes | none | 528347 |`

Next: review
