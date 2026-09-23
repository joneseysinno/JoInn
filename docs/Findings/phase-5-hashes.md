# Phase 5 hash record

Date: 22 September 2026.

No Phase 0–3 coding-region hash moved. Every golden that existed before
this phase — phase0 cells, calculator bodies, phase 2.1 reference bodies,
phase 3 descriptions and environment bodies — is byte-identical in
`corpus/hashes.txt` to its prior value.

New artifacts, hashed at P5-23 (bodies with `joinn.body.v1`, universes
with `joinn.universe.v1` over canonical coding text):

- `corpus/phase5/units.body` — `556e785914ff720d908e14f10daa81ac797e0e0259b63a9647c68651a7703f57`
- `corpus/phase5/two_in_ports.body` — `8c9f1cb62fe42e1502f58279692dec0cf909089487a6663233c81ef9d8d62900`
- `corpus/phase5/bus.body` — `fa812abd9ab0b1b6a3aef72b3c8ca1e2641c6f307350a8cc86ebea56187b8123`
- `corpus/phase5/universe.universe` — `8008307276be08865c2c8fc991a01ddc6286c2416402caec08f902ef52ef9d9e` (610 payload bytes)
- `corpus/phase5/alone.universe` — `3c9936c98b862a44f7486380662182363a5483397c9ba323d7354cd9c7518453`
- `corpus/phase5/ordered.universe` — `7bc247e22c3782959f38ff8d857ddecfd8bbc3444d727422e4bffd875f93a3f4`
- `corpus/phase5/adversary.universe` — `00115de41ebed3250f1ca5dde861b4bcf0e33a2075c988fbeb1057495ea48fa9`

The calculator body hash remains
`b55fba1eff65942099f6daf84b8bc47d605be05f637d805d260d3fcfd8c3ebde`.
The sum cell hash remains
`6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39`.

P5-09's generic CLI session did not move any coding hash; it only changed
how a host loads a body by stem.
