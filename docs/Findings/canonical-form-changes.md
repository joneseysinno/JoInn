# Canonical form changes

A line is appended here only by `cargo xtask corpus rebless --i-changed-the-canonical-form`.

Reblessing a golden to make a test pass is forbidden. This file starts empty of change records because `codex` is 1 and the goldens in `joinn/corpus/hashes.txt` are the original Phase 0 hashes.

## 2026-09-18 · `mul.cell` law corrected (P22-11)

Not a rebless. The cell stated `identity: mul(a, 0) = a`, which is false, and
was a golden only because `corpus verify` compared hashes. The law is now
`annihilator: mul(a, 0) = 0`. Hash moved
`4a37…` → `12b6e5458891b14aa74e43cee34b1184be04547fe58e035b1882125467120fa7`.
See `docs/Findings/phase-2.2-hashes.md`.
