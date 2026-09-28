# Phase 4 hashes

Date: 26 September 2026.

No Phase 0–5.2 coding hash moved. The rows already in `corpus/hashes.txt` are unchanged. This is not a rebless.

New artifacts, hashed at P4-03 (a cell with `joinn.cell.v1`, a body with `joinn.body.v1`, a universe with `joinn.universe.v1`, each over canonical coding text):

- `corpus/phase4/cli_input_open.cell` — `eb8479f42622365a12d5eb6ae3f35386d782cb9679144582774c49ea83945678`
- `corpus/phase4/format_twin.cell` — `fcc1ba589d125d8b490c631510dcec4d86d65a9e1f032ecb8d254b692999b85d`
- `corpus/phase4/fmt.body` — `5ed25b02b183bdb22709917bd7a5bb19d2bbf6e31d83ba3617a0ad3b05defed0`
- `corpus/phase4/fmt_twin.body` — `f1b05fd17e9eeaa284a03a6ea586073b7b328e377c169628fa347cdec239b0cf`
- `corpus/phase4/loop.universe` — `bf5f0c4c7a9690a31a86193e791b31e02480ae76e167d535b551f1db12fc2ace`

Added at P4-11 (28 September 2026), the same way:

- `corpus/phase4/loop_declared.universe` — `bd054c47fb60263b1d9d6142d86347e7dde0375ccf82bbd3159a0ad17e867e15`

The `declarations` section prints only when it is non-empty, so every earlier body and universe prints and hashes as before.

## Final (P4-14, 28 September 2026)

`cargo xtask corpus verify` → `corpus verify: 43 hash(es) match; cells admitted`.

`git diff 4b47cf6..HEAD -- joinn/corpus/hashes.txt` (from P4-02, the last commit before the Phase 4 corpus, to P4-13) prints only added lines: the two comment lines and the six Phase 4 hashes above. No line was removed or changed, so no Phase 0–5.2 hash moved.

The calculator body hash remains
`b55fba1eff65942099f6daf84b8bc47d605be05f637d805d260d3fcfd8c3ebde`.
The format cell hash remains
`3b0ab2bca11406a7e3bb3c1c4fd78e95af213f82fe2c1c5c6ffc0d4ec76e9c11`.
