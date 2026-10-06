# JoInn Phase 7.3 Review

Claude · 6 Oct 2026 · against `docs/Plans/JoInn Phase 7.3 Implementation Plan.md` (Draft 0.2), `docs/Findings/phase-7.3-stop.md`, `docs/Findings/phase-7.3-links.md`, and the commit messages of P73-01 … P73-14

**Method.** Cloned GitHub `main` at `14110cc` (P73-stop: ledger line 3). This time everything was pushed, so nothing had to be copied in. Ran `cargo xtask stop-check` in the clone, then `cargo xtask gate all` on its own, and read the gate 7.3 code, its controls and the cross-gate checks.

What this review could not cover:

1. Linux, 2 cores, one adapter (lavapipe), **rustc 1.97 stable**. 1.95 still can't be downloaded from this session. AJ's machine has three adapters.
2. CI couldn't be read from this session, so the run on the stop push is unread.
3. The window wasn't run. That is AJ's §6.2 check.

---

## Verdict

**Phase 7.3 is accepted.** Links are drawn, they stay in the gutters, and every count the plan predicted came out exactly. The speed work did what it was for. `gate all` dropped from 1 082 s to 492 s on AJ's machine, and that is *with* gate 7.3 added. Every earlier phase line still equals `gates.lock`.

The review did find three real gaps. None of them is a wrong answer. Each one is a check that isn't checking what it claims to check. Two of them come from my plan. I propose fixing all three as the first commits of Phase 8 (P8-F1 … F3, below), the same way 7.2's fixes opened 7.3.

---

## Reproduced here

| Command | Here | Cursor |
|---|---|---|
| `cargo test --workspace --no-fail-fast` | **442 passed, 0 failed** | 442 passed, 0 failed |
| fmt, clippy, vocab, modules, layers, corpus verify | all ok, 44 hashes | the same |
| `agree`, `assay agree`, `forces`, `contact`, `power`, spike s8 | every last line identical | the same |
| `pick` / `regrow` | `1 adapter(s), 12 subject(s) agree; planted disagreement: refused` / `planted difference: refused` | the same with 3 adapters |
| `zoom` step → `links` | `links: 3 universes, crossings 0, each folded node touched once` | the same |
| `gate all` | exit 0; phases 0 … 7.2 equal `gates.lock`, **`phase 7.3: 3/3`**; 234 553 ms | the same; 492 425 ms |
| `stop-check` | every step ok except `lavapipe` (apt here can't reach one mirror; the driver was already installed) | ok |
| `git diff 0480723..HEAD -- joinn/corpus` | empty (T1 didn't fire) | empty |

---

## Snags settled

All 25 snags were read. Most are recording-level and accepted as written. The ones that matter:

| Commit | Snag | Settled |
|---|---|---|
| P73-01 | Committed without `AGENTS.md` / Appendix B; completed by P73-01a | Accepted. Paperwork only |
| P73-S1 | xtask kept at opt-level 0, because the optimizer merged four gate 2.1/2.2 checks with identical bodies, and a test compares check functions **by address** | Accepted for now. **Fix: P8-F1.** The four legacy rows really do check one fact (`agree_cached().is_ok()`). The test is right to notice that, but an address is a fragile way to notice it |
| P73-S2 | `joinn-cli` now compiles `joinn-prim`'s mutant register, which it never calls | Accepted. That's the cost of a single build, and it's small |
| P73-S4 | `stop-check`'s "every line containing fail" catches test names (`tests/fail/…`) and a gate item's name | Accepted. Tidy it in P8-F1: match `FAILED` and `failed,` instead of the bare word |
| P73-02 | F1 and F2 ran gate 7.2 only, not `gate all` | Accepted. P73-02's `gate all` covered both. It's a process lapse, and the scope table is clear |
| P73-10 | The window said `member 4` where the in-memory replay said `member 3` | **Not just a curiosity. Fix: P8-F3** (Found in review, 3) |
| P73-13 (1) | `DropMember` doesn't apply to a universe, so `FlipMark("e0", "units.scale@0")` was used per §6 | **My error**: I didn't check the closed catalogue against the artifact kind. Cursor followed §6 correctly |
| P73-13 (4), (5) | "Body pixel" defined by the CPU with links left out; one fade zoom per threshold picked by most pixels | Accepted. Both are sensible readings, and both are printed |

---

## Found in review

### 1. Gate 7.3 isn't in the cross-gate checks

Three checks keep a **hand-written list** of gate tables. Gate 7.3 was added to the gate registry (`gate_table.rs`) but **to none of these three lists**:

| Check | What it guards | Lists gates through |
|---|---|---|
| `check_distinct_opposition` (rule 41) | no two items share `(artifact, mutation)` | 7.2 |
| `g3_artifacts` (gate 3 item 8) | every control artifact resolves | 7.2 |
| `no_two_gate_items_share_a_check_or_control` | no two items share a function | 7.2 |

So rule 41 was **never checked for gate 7.3**. The plan said it held, and I checked by hand that it does: the three pairs are distinct from every other gate's. That only worked out because the pairs happened to be distinct. Phases 8 and 9 add gates the same way and would slip through the same gap.

### 2. Gate 7.3's oppositions are caught by admission, not by links

I mutated each control artifact the way gate 7.3 does and asked admission about it:

```
ordered.universe  WireAcross("path")             → refused: wire calc.sum@2 -> units.scale@0 spans bodies calc and units
ordered.universe  FlipMark("path","calc.sum@2")  → refused: link path member calc.sum@2 is head but direction is Out
universe.universe FlipMark("e0","units.scale@0") → refused: link e0 member units.scale@0 is tail but direction is In
```

Each control has two halves: "the subject is refused at admission" and the geometric half (crosses / arrow against marks / touched twice). **All three answer `true` through the first half.** The geometric halves never run against an opposed subject. In practice they repeat gate 5 and 5.1, which oppose `WireAcross("e0")` and `FlipMark("e0", "calc.sum@2")` on what is essentially the same file.

That isn't sloppiness on Cursor's part. It's built into what 7.3 claims. "A link never crosses" is true **by construction**: no universe that admission accepts *can* produce a crossing, because the streets never enter a building. So no change to the universe text can oppose it. The only thing that can is a change to the **code**, and that's what P73-04's demo did (a gutter moved 4 units gave 1305 crossings), along with P73-13's demos (b)–(d). But those demos were shown once and then reverted. Nothing keeps checking them.

`pick` and `regrow` already solved this in Phases 6 and 7.2 with a **standing plant**: every run plants a disagreement and must see it refused. `links` should do the same.

### 3. The same universe, two different pictures of its links

`print_universe` writes links sorted by id and the members of unordered links sorted, and the universe's hash is the hash of that text. `grow_grove` keeps links in the order it grew them. So the grove in memory and the grove read back from `target/grove.universe` have **the same hash** but different link slots and different member numbers. That's why the window said `member 4` where the replay said `member 3`.

In JoInn's terms, identity is the hash. Anything the user can see or click should be a function of the hash alone. Right now the link IDs and the `member i` a click prints depend on where the universe came from. That's harmless today because only the grove is grown in memory. In Phase 9 it won't be: the creator edits a universe in memory and saves it, and a click before and after saving would name different members.

### 4. Speed: as planned

| | 7.2 (P72-14) | 7.3 baseline (P73-01a) | 7.3 stop |
|---|---|---|---|
| `gate all`, AJ's machine | 1 237 857 ms | 1 082 128 ms | **492 425 ms** (gate 7.3 adds ~107 s) |
| `gate all`, here | 724 s | — | **235 s** |

What's left in the stop is `pick` (139 s), `test` (154 s), `agree` (112 s) and `regrow` (66 s). `pick` and `regrow` each run twice in a stop: once as their own CI step and once inside gate 7.2. That's worth a look later, not now. R108 (the engine's clone and re-hash per fire) is still the biggest single cost.

### 5. Routing cost: a finding, not a problem

0.63 s in release (1.0 s dev) to route the grove's 137 links at grow, about 4.6 ms per link. Nothing per tick depends on it, and a zoom writes 0 rows. Snapshots still wait (R105). The findings' observation is a good one: routes are already the right kind of snapshot (grown once, chosen by integers per tick), and doing the same for text per band would cut most of the 311 634 instances without caching a pixel.

---

## Proposed fixes: the first commits of Phase 8

| # | Fix | Why |
|---|---|---|
| **P8-F1 · One gate registry** | The cross-gate checks (rule 41, gate 3 item 8, the shared-function test, the contact-artifact test) read their tables from `gate_table` itself, so every gate that exists is covered automatically. The shared-function test stops comparing addresses: each item carries its check's name, and the four 2.1/2.2 rows that check one fact are written down as one legacy exception. That lets xtask go back to opt-level 1. `stop-check`'s fail filter matches `FAILED` / `failed,` | Gap 1, and the S1 snag |
| **P8-F2 · Standing plants for truths by construction** | `links` plants, on every run, (a) a gutter line moved 4 units into its bodies, (b) a second touch on a folded node, (c) a mid-path arrow on an unordered link. Each must be refused, printed like `pick`'s: `links: … planted crossing, double touch, false arrow: refused (ok)`. Gate grading also prints **which half** of a control answered (`admission` or `check`), so a gate whose every control answers by admission can be seen. Phases 8 and 9 follow one rule: a truth that no admitted artifact can break gets a standing code plant instead | Gap 2 |
| **P8-F3 · Canonical order at admission** | An admitted universe's coding is put in print order (links by id, unordered members sorted) before anything is laid out. A test proves that `parse(print(u))` and `u` give byte-identical Route and Segment rows and the same pick lines, for the grove and both corpus universes. No corpus file changes, and no hash changes (the hash already uses this order) | Gap 3 |

All three change only checks and order. No gate answer changes. `gates.lock` stays as it is.

---

## Exit conditions (§6.1)

1. `stop-check --fresh` exits 0 with `phase 7.3: 3/3` and no tripwire: **met** (AJ's machine; reproduced here).
2. `phase-7.3-links.md` with every prediction marked and the adversary answered: **met.** Every prediction is `as predicted`, and the routing cost is measured.
3. No open snag: **met.** The three gaps above are carried into Phase 8 by decision, and none blocks it.
4. AJ's window check (§6.2, about three minutes): **owed.** The CI run on the stop push is also unread.

**Phase 7.3 is closed** once AJ has done the window check and the CI run on `14110cc` is green.
