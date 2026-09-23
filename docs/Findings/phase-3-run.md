# Phase 3 witness run (P5-00)

Date: 22 September 2026. Windows. Commands run from `joinn/`.

`gates.lock` was rewritten by `cargo xtask gate all`. It still reads
`phase 3: 9/9`. No phase dropped. The four items §0 of the Phase 5 plan
expects to be hollow are hollow: item 8 counts lines in `artifact_list.txt`
(F25), item 9 accepts any lock that contains the substrings `phase 0` and
`phase 2.2` (F26), item 1 is item 6 with a different `Signals` (F26), and
item 2 compares `describe` with itself under the name `cli_d` and never
touches `joinn-cli` (F27). They printed `ok`.

Last line of each command:

- `cargo xtask gate all` — `gate all wall milliseconds: 603384`
- `cargo xtask power` — `gate power: 20/20`
- `cargo xtask agree` — `injected disagreement: refused as truth violation (ok)`
- `cargo xtask corpus verify` — `corpus verify: 25 hash(es) match; cells admitted`
- `cargo xtask modules` — `modules: ok (enforced 9 crate(s))`
- `cargo xtask vocab` — `vocab: ok`

The lock `gate all` wrote, last line included:

```
phase 3: 9/9
```
