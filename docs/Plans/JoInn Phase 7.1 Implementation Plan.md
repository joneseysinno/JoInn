# JoInn Phase 7.1 Implementation Plan

**The beam computes, stage 1: worked examples · counting, where and side · formulas derived, libraries witness**

Author: AJ, with Claude · Draft 0.1 · October 1, 2026

> **Every decision in this plan is final.** Nothing here waits on AJ. Cursor never stops to ask AJ anything, and no file in this phase is typed by AJ. If a step can't be done the way the plan says, Cursor follows §0.3 (Snags) and keeps going.

---

## For AJ: this plan in plain English

**What Phase 7.1 is.** Part VI (*JoInn Dimension*) changed how JoInn thinks about quantities. A dimension is *where* a quantity lives. The base is counting, where, and side. Formulas are derived, and libraries are witnesses. You said working examples would help most, so this phase works examples. It does **not** add new JoInn grammar yet. It builds a **spike**: a small separate program, like S2 was for the GPU, that works five real beams exactly and shows its work. What it finds decides the grammar of stage 2.

**The five beams.** All W12x26, A992. Every answer is an exact fraction, and you'll see the decimal next to it.

| | Beam | What JoInn must get by itself |
|---|---|---|
| E1 | 24 ft simple span, 1.2 kip/ft | R = 14.4 kip each end · M = 86.4 kip·ft · δ = 1.514 in |
| E2 | 24 ft simple span, 10 kip at midspan | M = 60 kip·ft · δ = 0.841 in |
| E3 | 24 ft simple span, 10 kip at 6 ft | R = 7.5 and 2.5 kip · M = 45 kip·ft |
| E4 | 10 ft cantilever, 5 kip at the tip | M = 50 kip·ft **hogging** at the wall · δ = 0.487 in |
| E5 | E1 and E3 together | M = 116.4 kip·ft at midspan · δ = 2.093 in |

E5 is the "case no table lists". JoInn derives it directly. Its witness is the sum of two table cases, which is how you'd check it by hand.

**How JoInn gets them** (your idea, made exact):
1. **Balance first, with no material.** The reactions come from the forces summing to zero, and so do the turning effects. No E, no I, nothing from a library. The program shows it read no bridge.
2. **Add up along the span.** Shear is the load added up, and moment is the shear added up. In fractions this is exact, so wL²/8 comes out on its own.
3. **Cross the mirror once.** Deflection needs E and I, the only testimony. They come from one small pinned list: A992's E = 29,000 ksi and W12x26's Ix = 204 in⁴.
4. **Then the witnesses.** Every answer is compared with the formula AISC Table 3-23 and Roark Table 8.1 state. A difference would be a truth violation.
5. **Refine and check again.** Cut the beam into 1-ft pieces and every answer must come out the same. If the method is exact, the pieces can't change it.

**Five planted mistakes the spike must catch.** These answer the three challenges in Part VI §11:
- **Mixed order.** One moment taken as r × F and another as F × r. Caught because the moment at the far support isn't zero.
- **Rectangle rule.** A sloppy adding-up that skips the curve. Caught because cutting the beam finer changes the answer.
- **Wrong witness.** wL²/12 used in place of wL²/8. Caught as a disagreement.
- **Moment plus work.** Both are kip·in, which unit exponents can't tell apart. JoInn refuses to add them because they live in different places.
- **Deflection without a bridge.** Refused, while balance and moment work fine with no bridge at all.

**The first commits clean up Phase 7:**
- The window refuses ports it doesn't know again.
- "Order-free" becomes **order-blind** everywhere in code.
- A new standing rule says Cursor never moves your mouse or types outside a window it opened itself.
- The copied cell-loading rule becomes one helper.

**Choices Claude made while writing this. Say if you disagree before Cursor starts:**
- **A spike, not grammar.** The beam is worked in `joinn/spikes/s8-beam`, its own small program, outside the gated workspace, so nothing in the truth core moves while we learn. Stage 2 brings what works into JoInn's own files (cells, forces, tags), with a gate. This stage has **no gate**: a spike is measured, not gated, as S1 and S2 were.
- **The beam lives in its elevation plane** (Part VI K13). Span along x, loads along y. A moment is a plane quantity: lever arm wedge force.
- **Signs.** Up is positive and sagging is positive inside the program. "Down", "sagging" and "hogging" are printed words, which is presentation (K13a). A consistent flip of every sign is a different convention, not an error. Only *mixed* order is an error.
- **Moment versus work stands in for stress versus pressure.** Part VI §11's first challenge asked JoInn to tell stress from pressure. That needs a cross-section, which is 3D. In the elevation plane the same challenge is moment versus work: both kip·in, one oriented (a plane), one not. It's the classic torque-versus-energy confusion, and it's testable now.
- **Witnesses are formulas, written once.** The spike states each witness formula from AISC Table 3-23 and Roark Table 8.1 as an exact expression and never uses it to derive anything.

**What you do at the end (Stop C), about three minutes.** Run the spike and read E1 as an engineer: does 86.4 kip·ft and 1.51 in match your hand calc? Then pick one beam from your own work for stage 2. The steps are in §6.2.

**Your prompts to Cursor** (copy exactly, one per chunk):

- Chunk A: `Do chunk A of docs/Plans/JoInn Phase 7.1 Implementation Plan.md. Follow AGENTS.md.`
- Chunk B: `Do chunk B of docs/Plans/JoInn Phase 7.1 Implementation Plan.md. Follow AGENTS.md.`
- Chunk C: `Do chunk C of docs/Plans/JoInn Phase 7.1 Implementation Plan.md. Follow AGENTS.md.`

If Cursor runs out of room partway: `Continue chunk A of docs/Plans/JoInn Phase 7.1 Implementation Plan.md from the first commit not in git log. Follow AGENTS.md.` (Use the right letter.)

After each stop, tell Claude "Cursor finished chunk A" (or B, C).

---

## 0. How Cursor Works This Plan

| | |
|---|---|
| **Plan file** | `D:\JoInn\docs\Plans\JoInn Phase 7.1 Implementation Plan.md` |
| **Code** | `D:\JoInn\joinn\`. Chunk A changes the workspace; no new crate, no new external dependency in the workspace. Chunk B adds `joinn/spikes/s8-beam/`, its own workspace (like `spikes/s2-gpu`) |
| **Standing rules** | `AGENTS.md` as updated by P71-01 (Appendix A) |
| **Unit of work** | a **chunk** (A, B or C). Inside a chunk, do the commits in the order listed. Each commit is its own git commit, and its message starts with its id (`P71-05: …`) |
| **End of a chunk** | write the stop report (§0.2), commit it, `git push`, and stop. Do not start the next chunk |
| **Who decides** | every decision is in §2. Cursor decides only module layout, function bodies, private type representation, and error-string wording where §2 gives none |
| **Who checks** | Claude, at each stop, from a fresh clone on Linux. GitHub CI runs on Windows and Linux on every push. AJ runs the spike at Stop C |
| **Input** | **Never move the system mouse, click, or type outside a window Cursor itself started** (rule 67). A window check uses messages posted to that window, or writes `no display` |

### 0.1 The commit report

Every commit ends with this block, in the commit message body **and** in the chunk's stop report. Every value is copied from the terminal, never summarised.

```
Commit:     P71-NN (hash in git log)
Done-when:  <the command> → <the line it printed>          MET | NOT MET
Suite:      cargo test --workspace --no-fail-fast → <N passed, M failed>
Scans:      vocab → <last line> · modules → <last line> · layers → <last line>
Snags:      none | <each thing that went differently from the plan, with the printed line>
```

From P71-05 on, add one line: `Spike:      cargo test --manifest-path spikes/s8-beam/Cargo.toml → <N passed, M failed>`.

### 0.2 The stop report

At the end of each chunk Cursor writes `docs/Findings/phase-7.1-stop-<letter>.md` with:

1. `git rev-parse HEAD` (full hash).
2. Every commit report block from the chunk, in order, with real hashes.
3. The last line of each of: `cargo test --workspace --no-fail-fast`, `cargo xtask gate all`, `cargo xtask corpus verify`, `cargo xtask vocab`, `cargo xtask modules`, `cargo xtask layers`, `cargo xtask floor`, `cargo xtask forces`, `cargo xtask contact`, `cargo xtask pick`, `cargo xtask regrow`; from P71-05 on, the last line of `cargo run --manifest-path spikes/s8-beam/Cargo.toml`. Also every line of `gate all` that contains `fail`, and every failing test name.
4. **CI**: for the newest run after the push, read with PowerShell `Invoke-RestMethod "https://api.github.com/repos/joneseysinno/JoInn/actions/runs?branch=main&per_page=3"` and that run's `jobs_url`. Paste `head_sha`, `status`, `conclusion`, and each job's `name` and `conclusion`. If the run hasn't finished, wait and read again. A failed job is a snag, not a reason to change CI.
5. **Snags**: every snag from the chunk in one list, each tagged with its commit id.

Then commit (`P71-stop-<letter>: stop report`), push, and stop.

### 0.3 Snags

A snag is any point where a done-when can't be met as written, or a prediction in this plan turns out wrong.

- **Never** fake it, work around a scan, weaken a test, rebless a golden, or change the plan's meaning to make it pass.
- **A prediction that differs is not a failure.** Paste what the machine printed next to what the plan predicted, and continue. Claude settles it at the stop.
- Leave that piece in its honest state and **continue with the next commit**.
- If a later commit depends on the snagged one (§4), skip it and write `skipped: depends on P71-NN`.

---

## 1. Scope Fence

### In scope

- **Phase 7's carry-forwards**: `present` refuses unknown ports again; the order-blind rename; one helper for the keep-first-with-alleles rule; the mouse rule.
- **Docs**: Part VI committed; backlog R87–R97; `decisions.md` rows; the roadmap renumbering (the beam is 7.1, Visual Host II is 7.2).
- **Spike S8** (`joinn/spikes/s8-beam`): tagged exact quantities, the elevation complex, balance, derivation by adding up, the bridge edition, the witnesses, refinement, five plants, a golden output, and a CI step.
- **Findings**: `docs/Findings/phase-7.1-beam-examples.md`, with every prediction marked and the three answers to Part VI §11.

### Out of scope (Cursor refuses these even when they look small)

| Not now | Why it is tempting |
|---|---|
| Any new grammar: a tag in `.cell`, a beam file kind, a quantity frame | Stage 2, planned from this phase's findings |
| ℚ's turn, `mul@ℚ`, a multiply register row | Stage 2. The spike multiplies inside itself |
| Using the spike from any workspace crate, or the workspace from the spike | The spike is its own workspace and depends on nothing in `crates/` |
| Statically indeterminate beams (propped cantilever, fixed–fixed, continuous) | They need compatibility with a bridge to find reactions. Stage 2, R98 |
| Shear deformation, lateral-torsional buckling, φMn, a ratio check | Stage 2 and later |
| Drawing the beam, or any GPU work | Phase 7.2 |
| Library links, *bears on*, the column | *The beam connects*, after stage 2 |
| Changing any corpus file, hash, golden, `.desc`, `grandfather.txt` or `gates.lock` row | Never. No gate changes in this phase |
| Rewriting history: `phase-7-contact.md`, the Phase 7 plan and reviews keep "order-free" | They record what was true when written |

---

## 2. Decisions

All **DECIDED**.

### 2.1 Carry-forwards

**`present` refuses a port it does not know** (Phase 7 Stop C, found item 1). A `Scene` records the set of **interior addresses**: for a scene grown from a contact, every member port and every response in-port (`contact_surface`'s complement over the contact's lowered body); for a wired scene, none. `present` skips a face whose address is interior, and refuses any other face with no port row:

`present: port <address> is not in this scene; acceptance is a port of body <alias>` (Phase 6's wording, restored).

Tests: the wired calculator presenting `sum@9` is refused with that wording; the contact calculator presenting `sum`'s description (faces `sum@0`, `sum@1`, `sum@2`) is admitted, and only `sum@2` writes a row. Gate 6 and gate 7 still pass 3/3, and every regrow line is unchanged.

**Order-blind** (Part VI K15). Every identifier, printed word and rule text that says *order-free* or `order_free` in `crates/`, `xtask/`, `AGENTS.md`, `README.md` and `docs/Guides/` becomes *order-blind* / `order_blind`. Gate 7 item 2's title becomes `Combine is order-blind, and it fits what it reaches`. `gates.lock` does not change (it records scores, not titles). Files under `docs/Findings/phase-7-*`, `docs/Plans/JoInn Phase 7 *` and `docs/Theory/` keep their words.

**Predicted `cargo xtask forces` after the rename** (only the words change; every number, counterexample and order stays):

```
combine ℤ 1 by cell:6b32…: order-blind (64 pairs, 64 triples, seed 7), opposed by separate cell:6fcb… (turn 0 from {1 2})
planted: order_blind on mutant.difference: refused (ok): not order-blind: f(a, b) = 9223372039002259455 but f(b, a) = -9223372039002259455 at a = 9223372036854775807, b = -2147483648; acceptance is a response whose result does not depend on member order
planted: order_blind on mutant.midpoint: refused (ok): not order-blind: f(f(a, b), c) = -1535576763092620387 but f(a, f(b, c)) = -3841419772306314346 at a = -9223372036854775837, b = 3081064984484294291, c = 0; acceptance is a response whose result does not depend on member order
planted: order_blind on mutant.max: order-blind (ok), not registered
planted: order_blind on mutant.plus1: order-blind (ok), not registered
planted: check_register with separate = response: refused (ok): combine on ℤ 1 is unopposed: no separate; acceptance is a turn of cell:6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39
forces: 1 combine registered, 1 opposed; plants: difference refused, midpoint refused, max and plus1 order-blind but unregistered, unopposed refused (ok)
```

**One helper for shared coding regions** (Phase 7 Stop B and C, item 4). `joinn_dna::keep_first_with_alleles(map: &mut BTreeMap<Hash, Cell>, hash: Hash, cell: Cell)` inserts `cell` unless the map already holds a cell for `hash` that carries alleles, or the new cell carries none. That is the one rule, written once. The eight copies (`joinn-cli` and `joinn-shell-desktop` `load_cells`, xtask `perf`, `load_phase5_bodies`, `forces::corpus_cells`, and the tests `joinn-test-host/tests/universe_far_side.rs`, `joinn-link/tests/crossing.rs`, `joinn-link/tests/contact.rs`) call it. No new workspace edge: every caller already depends on `joinn-dna`. A unit test covers the four cases (empty map; first without alleles then with; first with then without; both with: first kept).

### 2.2 The spike's shape

`joinn/spikes/s8-beam/`: `Cargo.toml` with its own `[workspace]` (as `spikes/s2-gpu`), edition 2024, dependencies `num-bigint = "0.4"`, `num-rational = "0.4"`, `num-traits = "0.2"` only. One binary (`cargo run`) and its tests (`cargo test`). The spike follows the spirit of the standing rules even though no scan reaches it: **no `f32`/`f64`, no `unwrap`/`expect` outside tests, no randomness, refusals as values.** Module layout is Cursor's (rule 25's style).

Units inside are **kip and inch**. Inputs and printed lengths are in **feet** and converted exactly (12 in = 1 ft). Moments print in kip·ft (kip·in ÷ 12, exact).

### 2.3 Tags (Part VI §8, in miniature)

```rust
enum Side { Placement, Source, Energy }     // Energy: a source · placement product, the shared currency
enum Axis { None, X, Y, Plane }             // the elevation plane: x along the span, y up; Plane = x∧y
struct Tag { side: Side, length: i32, axis: Axis }
struct Q { value: BigRational, tag: Tag }
```

A tag prints as `<side> · length <n> · <axis>`, all lowercase: `source · length 1 · plane`. Its **unit** prints from side and length: Source and Energy give `kip`, then `·in` per positive length or `/in` per negative (`kip·in`, `kip/in`); Placement gives `in`, `in²`… or `1` at length 0.

| Quantity | Tag |
|---|---|
| x, L, a lever arm, a segment length ℓ | placement · length 1 · x |
| P, R, V | source · length 0 · y |
| w (as the oriented q = −w) | source · length −1 · y |
| M | source · length 1 · plane |
| work (force · displacement along y) | energy · length 1 · none |
| θ | placement · length 0 · plane |
| v | placement · length 1 · y |

**Operations** (the only ones; every printed quantity is made by them):

- `add(a, b)`: tags must be equal. Otherwise refused: `add: <tag a> and <tag b> differ, though both are <unit>; acceptance is two quantities on one piece, side and pair` when the units print the same, and without the `, though both are <unit>` clause when they don't.
- `scale(a, n)`: by a plain count n in ℚ (½, ⅙, the 12 of a unit). Tag unchanged.
- `wedge(a, b)`: x ∧ y → plane, value a·b; y ∧ x → plane, value −a·b (**order-signed**). Placement ∧ Source and Source ∧ Placement → Source; Placement ∧ Placement → Placement; Source ∧ Source refused. Lengths add.
- `dot(a, b)`: same axis (x·x or y·y) → axis none, value a·b (**order-blind**). Source with Placement → Energy; Placement with Placement → Placement; Source · Source refused. Lengths add.
- `total(a, ℓ)`: a density added up over a line (ℓ: placement · length 1 · x). Tag of a, length + 1. This is combine along the line.
- `bridge`: the only way across the mirror (§2.5).

### 2.4 The elevation complex, balance, and adding up

**The complex.** Points at both ends, at each point load, and at each printed section, sorted by x; one line between each pair of neighbors. `refined` adds a point at every whole foot.

**Supports.** *Simple*: pin at x = 0, roller at x = L, reactions R_A and R_B along y. *Cantilever*: fixed at x = 0, reaction R_A and moment M_A; free at x = L.

**Balance, with no bridge.** ΣF = 0 along y, and ΣM = 0 about A, with every moment taken as `wedge(r, F)`, r from A to the force. Two equations, two unknowns, solved exactly. The uniform load enters as its total over the span, acting at mid-span. The spike counts bridge reads; balance must read **zero**.

For the cantilever, balance gives the wall's couple C_A from ΣM = 0 about A (C_A = −Σ wedge(x_i, F_i) over every load). The internal moment just right of the wall is **−C_A**, and that is what prints as `M_A` with its sense (E4: C_A = +600 kip·in, so `M_A 50 kip·ft hogging`).

**Adding up along the span.** Walk the points left to right with V and M. Before the first point, V = 0 and M = 0 (simple) or −C_A (cantilever). At each point, first step across the line from the previous point (length ℓ, oriented load q on it):

```
M ← M + wedge(ℓ, V) + wedge(ℓ/2, total(q, ℓ))      the load's moment: lever arm ℓ/2 wedge its total
V ← V + total(q, ℓ)
```

then add the point's reaction and point load to V. **M at the far end must be exactly 0** (the closure check). That is balance seen from the other end, and it reads no bridge.

### 2.5 The bridge edition and deflection

The **bridge edition** is one small file in the spike, `bridges.edition`, read at start:

```
edition AISC Manual, 16th ed.
A992 E 29000 ksi
W12x26 Ix 204 in4
```

It is the only testimony the spike reads. `bridge(M)` turns a moment into curvature, κ = M / (E·I) (placement · length −1 · plane), and counts one read. Without the edition, any call to it is refused: `<what> needs the bridge E·I; acceptance is a pinned edition`.

**Rotation and deflection** are the exact integrals of κ along each line (κ is quadratic on a line, so the integrals are exact in ℚ):

```
θ ← θ + (M·ℓ + V·ℓ²/2 + q·ℓ³/6) / EI
v ← v + θ_prev·ℓ + (M·ℓ²/2 + V·ℓ³/6 + q·ℓ⁴/24) / EI      using M, V before the step
```

Simple span: θ at A is the unknown that makes v = 0 at B (one exact linear solve). Cantilever: θ = v = 0 at A. How Cursor tags the inside of these two lines is Cursor's choice, but every printed v and θ carries its tag from §2.3.

### 2.6 The witnesses

Each witness is an exact expression of the example's inputs, written once in the spike, with its source. **No witness is ever used to derive.** A witness compares to the derived value's magnitude, with the case's sense (sagging, hogging, down).

| Example | Witness (AISC Table 3-23 · Roark Table 8.1) |
|---|---|
| E1 | wL²/8 at mid-span; 5wL⁴/384EI at mid-span |
| E2 | PL/4 at mid-span; PL³/48EI at mid-span |
| E3 | Pab/L at the load; Pa²b²/3EIL at the load |
| E4 | PL at the wall (hogging); PL³/3EI at the tip |
| E5 | E1 + E3 (superposition of the two table cases) for M and v at 6 ft and 12 ft |

E5's deflection witnesses use the stated formulas: uniform load, v(x) = wx(L³ − 2Lx² + x³)/24EI; point load at a, for x ≤ a: Pbx(L² − b² − x²)/6EIL, and for x > a: Pa(L − x)(2Lx − a² − x²)/6EIL.

### 2.7 The plants (rule 19: every check opposed)

| Plant | What it does | Must print |
|---|---|---|
| mixed order | in the walk, the load's moment is taken `wedge(total(q, ℓ), ℓ/2)` (F ∧ r) while the reactions stay r ∧ F | the closure check refuses |
| rectangle rule | the walk omits the load's moment term | refinement refuses: the 2-line and 24-line answers differ |
| wrong witness | E1's moment witness replaced by wL²/12 | a disagreement, counted as a truth violation |
| moment + work | `add(M(12 ft) of E1, dot(P, v(12 ft)) of E2)` | `add` refuses: different tags, the same kip·in |
| no bridge | E1 run with the edition absent | v refused; balance and M succeed and read no bridge |

### 2.8 The predicted output

`cargo run --manifest-path spikes/s8-beam/Cargo.toml` prints exactly this (Claude computed it from §2.3–§2.7 with an independent script in exact fractions; every witness value was checked against its closed form separately). Decimals: a value that isn't a whole number also prints its decimal in parentheses, to at most 4 places with trailing zeros dropped when exact, or rounded to exactly 4 places (half up) with `~` when not.

```
s8 beam · exact in ℚ · kip and inch inside, feet shown
bridges: A992 E 29000 ksi · W12x26 Ix 204 in⁴ (pinned: AISC Manual, 16th ed.)
witnesses: AISC Manual Table 3-23 · Roark Table 8.1 (stated, compared, never used to derive)
E1 simple span, uniform load · L 24 ft · w 6/5 kip/ft down
  complex: 3 points, 2 lines (elevation plane)
  balance: R_A 72/5 kip up, R_B 72/5 kip up · ΣF 0 · ΣM 0 · M at end 0 · no bridge read
  M(12 ft) 432/5 kip·ft sagging (86.4)
  v(12 ft) 93312/61625 in down (1.5142~)
  witness wL²/8 = 432/5 kip·ft: agree
  witness 5wL⁴/384EI = 93312/61625 in: agree
  refined to 24 lines: equal at every point of the 3
E2 simple span, point load at midspan · L 24 ft · P 10 kip down
  complex: 3 points, 2 lines (elevation plane)
  balance: R_A 5 kip up, R_B 5 kip up · ΣF 0 · ΣM 0 · M at end 0 · no bridge read
  M(12 ft) 60 kip·ft sagging
  v(12 ft) 10368/12325 in down (0.8412~)
  witness PL/4 = 60 kip·ft: agree
  witness PL³/48EI = 10368/12325 in: agree
  refined to 24 lines: equal at every point of the 3
E3 simple span, point load at 6 ft · L 24 ft · P 10 kip down
  complex: 3 points, 2 lines (elevation plane)
  balance: R_A 15/2 kip up, R_B 5/2 kip up · ΣF 0 · ΣM 0 · M at end 0 · no bridge read
  M(6 ft) 45 kip·ft sagging
  v(6 ft) 5832/12325 in down (0.4732~)
  witness Pab/L = 45 kip·ft: agree
  witness Pa²b²/3EIL = 5832/12325 in: agree
  refined to 24 lines: equal at every point of the 3
E4 cantilever, point load at the tip · L 10 ft · P 5 kip down
  complex: 2 points, 1 line (elevation plane)
  balance: R_A 5 kip up, M_A 50 kip·ft hogging · ΣF 0 · ΣM 0 · M at end 0 · no bridge read
  M(0 ft) 50 kip·ft hogging
  v(10 ft) 240/493 in down (0.4868~)
  witness PL = 50 kip·ft: agree
  witness PL³/3EI = 240/493 in: agree
  refined to 10 lines: equal at every point of the 2
E5 simple span, uniform load and point load at 6 ft · L 24 ft · w 6/5 kip/ft · P 10 kip
  complex: 4 points, 3 lines (elevation plane)
  balance: R_A 219/10 kip up, R_B 169/10 kip up · ΣF 0 · ΣM 0 · M at end 0 · no bridge read
  M(6 ft) 549/5 kip·ft sagging (109.8)
  M(12 ft) 582/5 kip·ft sagging (116.4)
  v(6 ft) 478224/308125 in down (1.5520~)
  v(12 ft) 128952/61625 in down (2.0925~)
  witness E1 + E3 at 6 ft = 549/5 kip·ft: agree
  witness E1 + E3 at 12 ft = 582/5 kip·ft: agree
  witness E1 + E3 at 6 ft = 478224/308125 in: agree
  witness E1 + E3 at 12 ft = 128952/61625 in: agree
  refined to 24 lines: equal at every point of the 4
plant mixed order: refused (ok): balance: E1 M at end 1728/5 kip·ft, want 0; acceptance is one order for every moment
plant rectangle rule: refused (ok): refinement: E1 M(12 ft) 864/5 kip·ft with 2 lines, 468/5 kip·ft with 24 lines; acceptance is a derivation the complex cannot change
plant witness wL²/12: refused (ok): E1 witness wL²/12 = 288/5 kip·ft, derived 432/5 kip·ft; a disagreement is a truth violation
plant moment + work: refused (ok): add: source · length 1 · plane and energy · length 1 · none differ, though both are kip·in; acceptance is two quantities on one piece, side and pair
plant no bridge: refused (ok): v(12 ft) needs the bridge E·I; acceptance is a pinned edition. balance and M read no bridge
s8: 5 example(s), 12 witness(es) agree, 0 disagree; refinement equal in 5; plants: 5 refused (ok)
```

The spike writes **no file**. A test, `the_run_prints_the_plan_block`, compares the run's output with `spikes/s8-beam/expected.txt`, which P71-05 creates holding exactly the block above, typed from this plan (rule 17: the instrument does not write its own finding).

### 2.9 Invariants

| # | Invariant | Commit |
|---|---|---|
| **V134** | `present` refuses a port the scene does not hold, unless it is a contact's interior port | P71-02 |
| **V135** | Balance and moment read no bridge; deflection reads exactly the bridge edition | P71-07, P71-08 |
| **V136** | Every derived value equals every witness that states it, exactly | P71-09 |
| **V137** | Refining the complex changes no derived value at any shared point | P71-07 |
| **V138** | `add` refuses unequal tags, even when their units print the same | P71-05 |
| **V139** | Every moment is taken in one order; mixed order fails the closure check | P71-06 |
| **V140** | Cursor sends no input outside a window it started (rule 67) | P71-01 onward |

---

## 3. What runs

| Command | Prints | Exit 1 when |
|---|---|---|
| `cargo xtask forces` | §2.1's seven lines | as Phase 7 |
| `cargo run --manifest-path spikes/s8-beam/Cargo.toml` | §2.8's block | any witness disagrees outside the planted one, any refinement differs outside the planted one, any closure is not 0 outside the planted one, or any plant is not refused |
| `cargo test --manifest-path spikes/s8-beam/Cargo.toml` | the spike's tests, including `the_run_prints_the_plan_block` | any test fails |
| CI | new step `spike s8` after `contact`, on both jobs: `cargo test --manifest-path spikes/s8-beam/Cargo.toml` | the step fails |

---

## 4. The Commits

Done-when is a command, and the command must be able to fail.

### Chunk A: carry-forwards and words

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P71-01** | **Plan, rules, docs** | Commit this plan. `AGENTS.md` per Appendix A. Commit the doc edits already on disk unchanged (`docs/Theory/JoInn Dimension.md` and any other), **then** apply Appendix B. No code | `git show --stat HEAD` lists this plan, `AGENTS.md`, `docs/Theory/JoInn Dimension.md`, the backlog, `decisions.md` and the roadmap (paste `git status` first). Nothing under `joinn/` except `AGENTS.md` |
| **P71-02** | **V134** | §2.1's `present` rule, the interior set, its two tests | Both tests pass; `cargo xtask gate all` exits 0 with every phase line as before; `cargo xtask regrow` and `cargo xtask pick` print the same lines as Phase 7 Stop C (paste a `Compare-Object` that prints nothing) |
| **P71-03** | **Order-blind** | §2.1's rename | `cargo xtask forces` prints §2.1's seven lines byte for byte. `grep -rni "order.free" crates xtask joinn/AGENTS.md joinn/README.md docs/Guides` prints nothing. `cargo xtask gate 7` prints `2 ok  Combine is order-blind, and it fits what it reaches` |
| **P71-04** | **One helper** | §2.1's `keep_first_with_alleles` and its test; the eight callers | `grep -rn "alleles.is_empty() && !cell.alleles.is_empty()" crates xtask` prints only the helper's line. `cargo test --workspace` passes; `cargo xtask contact` and `cargo xtask forces` print the same lines as after P71-03 |
| — | **Stop A** | `phase-7.1-stop-a.md` (§0.2), push, stop | — |

### Chunk B: spike S8

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P71-05** | **Tags** | The spike's skeleton (§2.2), `Q`, `Tag` and the six operations (§2.3), `expected.txt` (§2.8), and tests: each operation's rule, `add` refusing moment + work with §2.7's words (V138), `wedge` order-signed (`wedge(x, y) = −wedge(y, x)`), `dot` order-blind | `cargo test --manifest-path spikes/s8-beam/Cargo.toml` passes every test except `the_run_prints_the_plan_block` (it fails until P71-09; paste its first differing line) |
| **P71-06** | **Complex and balance** | §2.4's complex, supports, balance and closure; the mixed-order plant (V139) | Tests: each example's reactions as in §2.8; closure 0 in all five; balance's bridge-read count 0; the mixed-order plant refused with §2.8's line |
| **P71-07** | **Adding up** | §2.4's walk for V and M; `refined`; the rectangle plant (V137) | Tests: every M in §2.8; refinement equal in all five; the rectangle plant refused with §2.8's line |
| **P71-08** | **The bridge** | `bridges.edition`, `bridge`, §2.5's rotation and deflection; the no-bridge plant (V135) | Tests: every v in §2.8; the bridge-read count is 0 for balance and M and positive for v; the no-bridge plant refused with §2.8's line |
| **P71-09** | **Witnesses and the run** | §2.6's witnesses; the wrong-witness plant; `main` printing §2.8; the CI step (§3) | `cargo run --manifest-path spikes/s8-beam/Cargo.toml` prints §2.8's block byte for byte (paste it whole). `the_run_prints_the_plan_block` passes. CI's `spike s8` step passes on both OSes |
| — | **Stop B** | `phase-7.1-stop-b.md`, push, stop | — |

Dependencies: in order. P71-05 to P71-09 touch only `spikes/s8-beam/` and, in P71-09, the CI file.

### Chunk C: findings

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P71-10** | **Findings** | `docs/Findings/phase-7.1-beam-examples.md`: the spike's full output; a *Predictions* section marking every line of §2.8 and §2.1's seven lines `as predicted` or `differs` with both values; and **Part VI §11's three answers**, each with the line that answers it: (1) moment versus work told apart (the moment + work plant); (2) truth separated from testimony (the bridge-read counts, the no-bridge plant); (3) a sign that depends on order caught (the mixed-order plant). Then a list, **What stage 2 needs**, of what the spike had to invent that JoInn's grammar doesn't have yet, one line each, observed, not designed | Every prediction is marked; the three answers each quote a printed line |
| **P71-11** | **Docs** | `Guides/03-where-we-are.md`: the beam worked exactly, in a spike; what isn't built. `Guides/05-glossary.md` gains *where, side, pair, bridge, bridge edition, witness library, order-blind, order-signed, order-bound, chaos, elevation plane, refinement*. `decisions.md` rows V134–V140 with their status. The roadmap's Phase 7.1 note points at the findings | `cargo xtask gate all` from a fresh clone exits 0 with phases 0 … 7 as before. `git diff --stat <P71-01>..HEAD -- joinn/corpus joinn/gates.lock` prints nothing |
| — | **Stop C** | `phase-7.1-stop-c.md` with the CI read, push, stop | — |

**If something has to be cut for time:** cut E5's deflection witnesses first, then E4. Never cut balance with no bridge, the closure check, refinement, or the moment + work plant.

---

## 5. Test Strategy

Every earlier invariant is re-run on every commit; gates 0 … 7 still pass, and no score moves. **A test must never** assert only that something ran. It asserts the exact value, the tag, the count, or the refusal's words.

The spike's tests assert exact rationals, never decimals; the decimal is presentation and is checked only by the golden.

---

## 6. Exit

This stage has **no gate**. A spike is measured, not gated (as S1 and S2 were). The gate comes with stage 2, when the beam enters JoInn's own files.

### 6.1 Conditions for opening stage 2

1. `gate all` exits 0 with phases 0 … 7 as before; CI is green on both OSes, the `spike s8` step included (read, not assumed).
2. `phase-7.1-beam-examples.md` exists with every prediction marked and the three answers quoted.
3. AJ has run the spike (§6.2) and said the numbers are right, or each thing that looked wrong is a snag Claude has settled.
4. Claude's Stop C review lists no open snag, or AJ has chosen to carry each one forward.

**Stage 2** (planned after this): the tag in JoInn's own grammar, ℚ's turn and multiply, balance and adding-up as forces in a contact body, the bridge edition as a pinned source, witnesses as a checked corpus, and AJ's own example from §6.2.

### 6.2 AJ's check (Stop C, about three minutes)

In PowerShell:

```
cd D:\JoInn\joinn
cargo run --manifest-path spikes/s8-beam/Cargo.toml
```

1. Read **E1**. Does R = 14.4 kip, M = 86.4 kip·ft and δ = 1.514 in match a 24 ft span at 1.2 kip/ft, W12x26? Every line says `agree`.
2. Read **E4**. The wall moment says **hogging**, and the tip goes **down**.
3. Read **E5**. JoInn derived it directly; the witness is E1 + E3, and it agrees.
4. Read the five `plant` lines. Each says `refused (ok)`.
5. Last line: `s8: 5 example(s), 12 witness(es) agree, 0 disagree; refinement equal in 5; plants: 5 refused (ok)`.

Tell Claude "numbers are right" or which line looks wrong. **Then name one beam from your own work** (any span, supports and loads) for stage 2.

---

## 7. Risks

| Risk | What to do |
|---|---|
| **Tags are decoration** (Part VI §11's adversary) | If the moment + work plant could be caught without tags, or no step in the walk ever needed a tag to refuse, write that in the findings. Stage 2 then falls back to unit exponents |
| A value differs from §2.8 | A prediction that differs, not a failure. Paste both. Exact fractions decide; never adjust the golden to the output |
| The decimal rule differs at a rounding edge | A snag with both strings. The fraction is the truth; the decimal is presentation |
| The deflection integrals need more than §2.5 states | Write what was needed in *What stage 2 needs*. Never call the witness to fill a gap |
| The spike wants to share code with `crates/` | Refuse. Copying ℚ helpers into the spike is allowed; depending on the workspace is not |
| AISC or Roark state a case differently from §2.6 | The witness is the formula as written in §2.6; record the difference as a snag for Claude |

---

## 8. Open Items

| ID | Topic | Question |
|---|---|---|
| **R98** | Indeterminate beams | A propped cantilever needs compatibility (placement side) and a bridge to find its reactions. Is "balance reads no bridge" then only true for determinate beams, and does the order of the mirror change (placement first)? |
| **R99** | Where a section property lives | Ix is the W12x26's section, a 2D body one zoom level in (Part VI §9). Is it a bridge (testimony from the shapes table) or derived from the section's own complex, with the table as its witness? |
| **R100** | The decimal | A printed decimal is presentation. Who chooses its places: the host, the quantity's tolerance, or the edition's precision? |
| R87–R97 | From Part VI and Phase 7 Stop C | Stay open; stage 2 answers R87 (presentation names parts) with the beam's part names |

---

## Appendix A · `AGENTS.md` changes

Replace the header paragraph with:

```markdown
# JoInn — standing rules

This repo is JoInn. Phase 7.1 is the beam computes, stage 1: worked examples in
a spike (spikes/s8-beam), exact in ℚ, with formulas derived and libraries as
witnesses. Its one idea is NOTHING DERIVED IS A BASE. The build plan is
docs/Plans/JoInn Phase 7.1 Implementation Plan.md. Work one CHUNK at a time (A,
B or C), one git commit per numbered step, and stop at the chunk's stop report.
Every decision is in the plan. Never stop to ask; follow the plan's Snags
section instead.

No new grammar in this phase. The spike is its own workspace and depends on
nothing in crates/.
```

Rule 63 becomes:

```markdown
63. COMBINE IS ORDER-BLIND, AND EVERY FORCE IS OPPOSED. Order is always real and
    recorded; a registered combine's result does not depend on it (order-blind),
    which the order sample checks. Every registered combine has a registered
    separate that is a turn of its response. Member order is never hashed.
```

Keep every other rule exactly as it is. Append:

```markdown
67. THE MOUSE AND KEYBOARD ARE AJ'S. Never move the system cursor, click, or type
    outside a window you started. A window check posts messages to that window
    only, or writes `no display`.
68. FORMULAS ARE DERIVED; LIBRARIES WITNESS. A value JoInn can derive from
    connection and counting is derived, never looked up. A library formula is a
    witness, compared exactly; a disagreement is a truth violation. Only bridges
    (material laws, section properties, constants) are read from an edition.
69. NOTHING IS RANDOM. Every sample is seeded, every order is canonical, and
    every run replays exactly. "Chaos" means only deterministic and sensitive.
```

---

## Appendix B · Documents (P71-01 applies these after committing the on-disk edits)

### B1 · The research backlog (`docs/Theory/JoInn Research Backlog.md`)

Append, after R86:

```markdown
> **1 Oct 2026 · Part VI.** R87–R97 come from *JoInn Dimension* (Part VI) and the
> Phase 7 Stop C review. Part VI answers R88 and R89 and partly answers R92.

## R87 — Presentation names parts  ·  status: open
## R88 — Base kinds  ·  status: answered (Part VI K18–K19: no base kinds; counting, where, side)
## R89 — Orientation  ·  status: answered (Part VI K13a: part of where; a sign convention is presentation)
## R90 — Is carry the mirror?  ·  status: open
## R91 — Time as pieces  ·  status: open
## R92 — The continuum  ·  status: partly answered (Part VI K20: polynomial loads derive exactly)
## R93 — Beyond linear bridges  ·  status: open
## R94 — Tags and the gate  ·  status: open
## R95 — Sensitive problems  ·  status: open
## R96 — Kinds as bodies  ·  status: open (murky; go forward)
## R97 — Witness libraries  ·  status: open
## R98 — Indeterminate beams  ·  status: open
## R99 — Where a section property lives  ·  status: open
## R100 — The decimal  ·  status: open
```

Each heading is followed by its question, copied from Part VI §13 or this plan's §8. Also set **R83** to `decided` with: `AJ 30 Sep: multiply is combine across dimensions; Part VI K9–K10 (stacking, order-signed)`.

### B2 · `docs/Findings/decisions.md`

Set R83 to `decided` with the note above. Add rows R87–R100 with phase `7.1` and the status from B1. Add rows V134–V140 with status `open` (P71-11 sets them).

### B3 · The roadmap (`docs/Plans/JoInn Build Roadmap.md`)

- After the 30 Sep note, add:

```markdown
> **1 Oct 2026 · Phase 7.1 is the beam.** Part VI (*JoInn Dimension*) defines a
> dimension as where a quantity lives, with counting, where and side as the base.
> Phase 7.1 is *the beam computes*: stage 1 works five beams exactly in a spike,
> with formulas derived and AISC and Roark as witnesses
> (`Plans/JoInn Phase 7.1 Implementation Plan.md`); stage 2 brings it into JoInn's
> grammar. Visual Host II is now **Phase 7.2**. Every later phase keeps its number.
```

- In the 30 Sep note, change `Next is the steel beam, then Phase 7.1.` to `Next is the steel beam (Phase 7.1), then Phase 7.2.`
- Retitle `#### Phase 7.1 · Visual Host II: charts, zoom, bands, links` to `#### Phase 7.2 · Visual Host II: charts, zoom, bands, links`.
- In the diagram, `P7["Phase 7.1<br/>visual host II"]` becomes `P7["Phase 7.2<br/>visual host II"]`.

---

*JoInn Phase 7.1 Implementation Plan, Draft 0.1 (1 Oct 2026). Opens after Phase 7's Stop C review and Part VI (JoInn Dimension, Draft 0.2). AJ decided on 30 Sep: the beam is Phase 7.1, split into computes and connects; multiply is combine across dimensions. On 1 Oct: order-blind; nothing derived is a base; counting, where and side; formulas derived and libraries as witnesses; and work examples to find the way. Claude decided, open to AJ's veto before chunk A: a spike rather than grammar, no gate this stage, the elevation plane, the sign rule, moment versus work as the first adversary test, and the five examples. Cursor executes. Claude verifies at each stop. AJ runs the spike at Stop C.*
