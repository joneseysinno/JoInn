# Phase 8 stop (plan 8 §0.2)

## HEAD

`git rev-parse HEAD` at the stop check: `5dcc2dd5a2056aeab0cb83b7a836c8dcb7fa9e12` (P8-12). The fresh clone ran on `2d3bbf7`, the same tree. P8-12's message was then amended to carry the result (P8-12 snag 3).

## Commits

- P8-01 7b965bb plan, rules, docs, gates.lock fix: MET
- P8-02 4803b2c letters and strands: MET. §2.2's "a strand and its partner have different hashes" does not hold for a strand that is its own partner (snag 1).
- P8-03 50ac65a codons and codex 1: MET
- P8-04 17e3643 the reader: MET
- P8-05 f25e025 strand witnesses and versions: MET
- P8-06 eb1b81c the fold: MET. The order-3 swap fails T2 and T5 as predicted, and T6 as well (snag 1).
- P8-07 a6f9c60 chromosomes and floors: MET
- P8-08 7472072 the gene pool: MET
- P8-09 a2ec2cc the 3D strand view: MET. The headless tests pass and the first tick line is pasted. The helix is drawn still, not turning (snag 2).
- P8-10 64c3511 the fold view: MET
- P8-11 d577550 gate 8 and findings: MET. `gate all` exits 0 with `phase 8: 3/3`. Demos (a), (b) and (c) each fail their item. Demo (a) also fails item 2 (snag 2).
- P8-12 5dcc2dd docs and freeze: MET. The corpus diff lists only `corpus/phase8/` and `hashes.txt`. `gate all` from a fresh clone is below.

## `cargo xtask stop-check --fresh`

```
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.58s
     Running `C:\Users\ajone\AppData\Local\Temp\cursor-sandbox-cache\3ef2016ed2da86c8ed384e4ccfe931cc\cargo-target\debug\xtask.exe stop-check --fresh`
stop-check: D:\JoInn\joinn\target\fresh\joinn at 2d3bbf7 (fresh clone)
lavapipe: sudo apt-get update && sudo apt-get install -y mesa-vulkan-drivers → skipped (Linux only)
fmt: cargo fmt --all -- --check → exit 0 (8269 ms, information)
clippy: cargo clippy --workspace --all-targets -- -D warnings →     Finished `dev` profile [optimized + debuginfo] target(s) in 1m 13s (74517 ms, information)
test: cargo test --workspace --no-fail-fast → 566 passed, 0 failed (196080 ms, information)
vocab: cargo xtask vocab → vocab: ok (18848 ms, information)
modules: cargo xtask modules → modules: ok (enforced 17 crate(s)) (1256 ms, information)
layers: cargo xtask layers → layers: ok (17 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop) (806 ms, information)
corpus verify: cargo xtask corpus verify → corpus verify: 67 hash(es) match; cells admitted (825 ms, information)
agree: cargo xtask agree → injected disagreement: refused as truth violation (ok) (128209 ms, information)
assay agree: cargo xtask assay agree → assay agree: 38 subject(s) agree, 1 not measured; injected disagreements: 2 refused as truth violation (ok) (3790 ms, information)
pick: cargo xtask pick → pick: 3 adapter(s), 12 subject(s) agree; planted disagreement: refused as truth violation (ok) (99337 ms, information)
regrow: cargo xtask regrow → regrow: 3 adapter(s); planted difference: refused (ok) (72230 ms, information)
forces: cargo xtask forces → forces: 1 combine registered, 1 opposed; plants: difference refused, midpoint refused, max and plus1 order-blind but unregistered, unopposed refused (ok) (858 ms, information)
contact: cargo xtask contact → contact: 1 body(ies); two forms, one truth (1326 ms, information)
zoom: cargo xtask grove && cargo xtask zoom && cargo xtask links && cargo xtask grow → grow: 2 systems, every size true, counting witnesses every step; evolution holds; planted fold, lasso, hash: refused (ok) (8009 ms, information)
dna: cargo xtask strand corpus/phase8/strands/twelve.strand && cargo xtask codex && cargo xtask read corpus/phase8/strands/empty.strand && cargo xtask read corpus/phase8/strands/pair-split.strand && cargo xtask read corpus/phase8/strands/twelve.strand && cargo xtask tower && cargo xtask pool → pool: no cycles; grows only when asked (5217 ms, information)
spike s8: cargo test --manifest-path spikes/s8-beam/Cargo.toml → 54 passed, 0 failed (6692 ms, information)
power: cargo xtask power → gate power: 20/20 (849 ms, information)
gate all: cargo xtask gate all → gate all wall milliseconds: 510416 (511357 ms, information)
stop-check: ok (1163615 ms, information)
```

stop-check prints only `gate all`'s last line. After the run, `git status` in the clone was empty. So `gate all`'s phases 0 … 8 equal the committed `gates.lock`, which ends `phase 7.4: 3/3`, `phase 8: 3/3`. In the main tree at P8-11, `gate all` printed every phase as in `gates.lock`, then `phase 8: 3/3`, and exited 0. Gate 8 took 7 ms (information).

## Predictions marked `differs`

None in the findings. Every row of `phase-8-strand-and-tower.md`'s §2.6 table is **held**. The one correction there, three top-floor bridges rather than one, is the 12 Oct correction the plan asked to be written down.

Five done-when or plan statements differed outside the findings:
- P8-02: a strand can be its own partner, with one hash (`GAATTC`; `pair-split` is one too).
- P8-03: §2.3's side rule cannot tell a START…STOP strand from its partner strand, because the partner of STOP is START.
- P8-06: the order-3 swap also fails T6.
- P8-09: the helix does not turn (V147).
- P8-11: demo (a) also fails item 2.

## Snags

- **P8-01 (1)**: Phase 7.4 already holds V160–V164 in decisions.md. Phase 8's V160–V168 are recorded as V165–V173, with the same order and words; each row names the plan's number.
- **P8-01 (2)**: git records the held plan as added and the Phase 8 path as modified, not as a rename, because Draft 1.0 replaced Draft 0.1 in the same commit. The held file is Draft 0.1 byte for byte.
- **P8-01 (3)**: Appendix A says "Append", so rules 82–86 follow rule 93.
- **P8-01 (4)**: Appendix B's three lines are ledger rows D-letters, D-held and D-accepted, because decisions.md is a table that `cargo xtask decisions` counts.
- **P8-02 (1)**: §2.2's different-hash claim fails for a strand that is its own partner (GAATTC). The first run failed with ``assertion `left != right` failed: n 4``. The fallback taken: the test states what holds. Identities differ exactly when the strands differ, and an odd-length strand is never its own partner.
- **P8-02 (2)**: `cargo xtask strand` prints the side, which needs codon parity, so it landed with P8-03.
- **P8-02 (3)**: joinn-strand sits in the layer table above the boundary, with one edge, joinn-frame.
- **P8-03 (1)**: §2.3's side rule cannot tell a protein strand from its partner strand. The partner of STOP (ATT) is START (AAT), so both open with an odd AAT. The rule taken: side is the parity of the head codon. The command also prints the partner read from its tail (TTA, even).
- **P8-03 (2)**: `cargo xtask strand` landed here, not at P8-02.
- **P8-03 (3)**: joinn-codex sits in the layer table above the boundary, with edges joinn-frame and joinn-strand. xtask depends on both new crates.
- **P8-04 (1)**: §0.1 asks for the Tower line from P8-04, but the command it runs was built at P8-06.
- **P8-04 (2)**: the reader meets four cases the plan doesn't word: a STOP with no START, an amino acid outside a protein, a START inside a protein, and letters past the last whole codon. Each is refused, in rule 35's wording.
- **P8-04 (3)**: the protein list text carries no reader version, so a version bump cannot move a witness byte.
- **P8-05 (1)**: the plan names no writer for "written once". `cargo xtask dna-witness` writes a witness that is absent and leaves one with the same bytes. It refuses one with other bytes and never rewrites.
- **P8-05 (2)**: hashes.txt names Phase 8 files by their path under corpus/. corpus verify hashes them as bytes under `joinn.dna.v1`.
- **P8-05 (3)**: twelve.strand holds the acids in table order, row by row. pair-split is its own partner strand.
- **P8-06 (1)**: the order-3 swap also breaks T6. All three refusals are pasted; T1, T3, T4 and T7 still hold.
- **P8-06 (2)**: T4 is checked against geometry-free arithmetic.
- **P8-06 (3)**: T5 takes its floor as a parameter, so gate 8's demo (b) can drop it to 0.
- **P8-06 (4)**: the order 1–4 witness files landed with the chromosome witnesses at P8-07.
- **P8-07 (1)**: a chromosome file holds one or more blocks separated by a blank line. Each block is a strand file, then `node <d> gene <hash>` lines sorted by d. Bad nodes and partial codons are refused.
- **P8-07 (2)**: twelve.chromosome's node 5 names pair-split, and node 10 names empty.
- **P8-07 (3)**: at order 2 the top floor is floor 1: segments 3, 7 and 11 (build, sense, grant). The middle, 7, is the centre.
- **P8-07 (4)**: the order 1–4 witnesses moved here from P8-06.
- **P8-08 (1)**: no existing store holds chromosomes, and joinn-tower may not depend on joinn-link. The pool takes the same shape inside joinn-tower: insert-only, and keyed by JoInn's Hash.
- **P8-08 (2)**: the pool prints hashes as their first twelve hex digits; files hold all 64.
- **P8-08 (3)**: the cycle witness's first chromosome shares pair-split's hash, so it enters a pool of its own.
- **P8-08 (4)**: the CI `dna` step runs `read` on the three readable strand witnesses only, because read exits 1 on reserved.strand by design. It runs on both runners.
- **P8-09 (1)**: the AGENTS header allows new crates only, but the views are additive changes to existing crates: joinn-visual, joinn-gpu (the style uniform grows to 40 entries), joinn-shell-desktop and xtask's layer table. Styles 0–15 keep their bytes, and no golden moved.
- **P8-09 (2)**: the helix does not turn. A turn owes a tick with no input, which V147 forbids. §10's fallback is taken in part: the helix is drawn still, projected from a little above.
- **P8-09 (3)**: the views read meaning through a new reader function, `joinn_codex::meanings`.
- **P8-09 (4)**: the stroke font has lowercase only, so names print lowercase. Reserved codons get no name.
- **P8-09 (5)**: the address prints the strand hash's first twelve hex digits.
- **P8-09 (6)**: the end cap has its own style (38). START and STOP share one tint (33).
- **P8-09 (7)**: the sandbox builds into its own CARGO_TARGET_DIR. `target\debug\joinn-desktop.exe` was a stale binary that read `--strand` as a path. The run pasted is the fresh binary.
- **P8-10 (1)**: floor zoom is on `+`/`-`, not the camera. In the fold, `-` climbs a floor and `+` comes back, and the wheel still zooms.
- **P8-10 (2)**: the shell's pool is every tower chromosome plus every strand whose hash it doesn't already hold. A `.strand` opens as the tower chromosome with its hash, so twelve.strand shows rings on nodes 5 and 10.
- **P8-10 (3)**: Enter with nothing selected opens the first ring not yet open.
- **P8-10 (4)**: the View trait gains two default methods and Key gains Tab. After Tab, every row of the other picture is owed.
- **P8-10 (5)**: the vocab scan refused a test name, so the test was renamed.
- **P8-10 (6)**: gene hashes are written at half label size.
- **P8-10 (7)**: the empty gene and pair-split draw as small curves.
- **P8-11 (1)**: item 3's opposite, "admission without the cycle check", is a change to the pool, and the catalogue damages artifacts. Item 3 therefore opposes `Replace(ONE_WAY)`, the cycle witness without its closing reference, and a test proves that constant. The pool-side opposite is demo (c).
- **P8-11 (2)**: demo (a) also fires item 2's control, because twelve.floors.txt names meanings.
- **P8-11 (3)**: grading runs every control before any check, so demo (a) stops at the controls.
- **P8-11 (4)**: items 1 and 2 oppose `Replace` with printed constants, each proved by a test. Text subjects have no neutral edit.
- **P8-11 (5)**: parse_subject gains `.chromosome`.
- **P8-11 (6)**: PowerShell garbled four characters in the findings' appendix capture. They were restored by hand.
- **P8-12 (1)**: Floor, Side and Organelle already had glossary meanings. The tower's floor and the strand's side are separate rows, and Organelle gains its codex sense.
- **P8-12 (2)**: V160–V168 are recorded as V165–V173.
- **P8-12 (3)**: "gate all from a fresh clone" is `stop-check --fresh`, run once on P8-12's tree.

## CI

Read before this stop's push (`Invoke-RestMethod "https://api.github.com/repos/joneseysinno/JoInn/actions/runs?branch=main&per_page=3"`):

```
37660372846 b510b90 completed success 10/07/2026 17:36:26
37563916398 16e2766 completed success 10/07/2026 02:50:38
37563804131 d672b5b completed success 10/07/2026 02:49:15
```

- Run 37660372846 (`head_sha` b510b90, P74-stop): `check (windows-latest): completed success`, `check (ubuntu-24.04): completed success`.

P8-01 … P8-12 and this report are pushed at this stop. Their CI run is read once after the push, without waiting.

## Tripwires

None fired.

- `git diff --stat 7b965bb..HEAD -- joinn/corpus` lists only the 19 files under `corpus/phase8/` and `hashes.txt` (736 insertions, no line changed).
- `gate all` exits 0, with every earlier phase as in `gates.lock` and `phase 8: 3/3`.
- The suite passes (566 passed, 0 failed, in the fresh clone).
- No golden hash moved, and no test was weakened. The P8-02 test states what holds in place of a claim that was false (snag 1), and the P8-10 rename keeps its assertions.
- No snag text repeats across three consecutive commits.

AJ's window check (§6.2) waits for the review. The helix in step 1 is drawn still (P8-09 snag 2).

Ledger: `| 5 | 8 | 5dcc2dd5a2056aeab0cb83b7a836c8dcb7fa9e12 | phase 8: 3/3 | 566 passed, 0 failed | 51 | yes | none | 510416 |`

Next: review
