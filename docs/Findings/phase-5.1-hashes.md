# Phase 5.1 hashes

Date: 23 September 2026.

These coding hashes moved because the artifacts were corrected on purpose. This is not a rebless. No Phase 0–3 coding hash moved. `corpus verify` matches the new goldens.

| name | was | now | why |
|---|---|---|---|
| units | `556e785914ff720d908e14f10daa81ac797e0e0259b63a9647c68651a7703f57` | `2d5fcc96689b26df93fa86ace7525b4820d1bd7f68b46e9ce75ee77d7e2c9fda` | `units.body` is now the mul cell, feet to inches |
| echo | (path did not exist) | `556e785914ff720d908e14f10daa81ac797e0e0259b63a9647c68651a7703f57` | the Phase 5 units body, byte-for-byte, under `corpus/phase5/controls/echo.body` |
| universe.universe | `8008307276be08865c2c8fc991a01ddc6286c2416402caec08f902ef52ef9d9e` | `2ccbb067db0b150ca607d1e7469fd28ea46892142d6b7729f1075cb899c9d1ad` | `e0` is ordered and typed; `g0` is gone; units hash is the new body |
| ordered.universe | `7bc247e22c3782959f38ff8d857ddecfd8bbc3444d727422e4bffd875f93a3f4` | `2814282ad8711d6414a0d1004b0546cdc97400484de0ea30ac9fa790cd873cd7` | same typed shape, link id `path` |
| alone.universe | `3c9936c98b862a44f7486380662182363a5483397c9ba323d7354cd9c7518453` | unchanged | |
| adversary.universe | `00115de41ebed3250f1ca5dde861b4bcf0e33a2075c988fbeb1057495ea48fa9` | unchanged | left as the Phase 5 attempt; it now binds `echo.body` |

`transits.universe`, `two_systems.universe`, and `wrong_container.universe` were rebound to the new units hash. They are controls, not rows in `hashes.txt`.
