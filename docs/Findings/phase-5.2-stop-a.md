# Phase 5.2 Stop A: baseline after Sitting A

Recorded by Claude on 24 Sep 2026 from a fresh `git clone` of
`github.com/joneseysinno/JoInn` on Linux (Rust 1.95.0 stable). The commands were
run from `joinn/`.

Tree: `7093897` (Checkpoint A evidence: Carried into Sitting B filled after P52-02a).

## Last line of each command

- `cargo test --workspace --no-fail-fast`: 148 passed, 1 failed (`error: 1 target failed`)
- `cargo xtask gate all`: `gate all: a phase failed` (exit 1)
- `cargo xtask corpus verify`: `corpus verify: 33 hash(es) match; cells admitted`
- `cargo xtask vocab`: `vocab: ok`
- `cargo xtask modules`: `modules: 1 hit(s)` (exit 1)

## Failures

| Command | Item or test | What it printed | Cause | Fixed by |
|---|---|---|---|---|
| cargo test | `joinn-cli` `universe_transcript_matches_golden` | output starts `> two` … `ignored-in-hash` instead of `a: two` … `2 + 3 = 5` | the CLI picks `calculator_c.body` (same coding hash as `calculator.body`, different labels) because Linux lists files in a different order | P52-02b (§2.10, §2.11) |
| gate all | 5.1 · Two hosts, one universe | `2 fail` | same cause as the test above | P52-02b |
| gate all | 3 · The path of truth | `9 fail` | the item certifies the previous run's lock (F54) | P52-06 retires it |
| gate all | 5 · A refusal stays home | `8 fail` | expected (F49) | P52-13 |
| gate all | 5.1 · A refusal stays home | `7 fail` | expected (F49); the row is a wrapper | P52-07 deletes it |
| gate all | phases 2, 2.1, 2.2 | `0/0`, `control does not read its artifact: …` | the byte-damage rule refuses their controls | P52-03 (legacy, §2.13) |
| modules | `xtask/src/modules/scan.rs` | `leaf has 8 production fn(s); want at most 1` | on AJ's disk an empty folder `modules/scan/` makes the scan treat the file as a capsule root; git doesn't store empty folders | P52-02b (§2.11) |

## Lock written by this run (not committed)

```
phase 0: pass
phase 1: 4/4
phase 2: 0/0
phase 2.1: 0/0
phase 2.2: 0/0
phase 3: 8/9
phase 5: 7/8
phase 5.1: 7/9
```

## Also found

- `joinn/.github/workflows/ci.yml` has never run, because GitHub only reads `.github/` at the repository root. Fixed by P52-02b (§2.12).
- `.cursor/rules/joinn.mdc` still carries the Phase 5 rules and header, while `AGENTS.md` carries 5.1's. Fixed by P52-02b (Appendix B).
