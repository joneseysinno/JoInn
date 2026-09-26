# JoInn Phase 4 Implementation Plan

**The assay: measure first, keep it only if it sees something nothing else sees · a working plan for Cursor**

Author: AJ, with Claude · Draft 0.1 with Amendment A · September 26, 2026

> **Every decision in this plan is final.** Nothing here waits on AJ. Cursor never stops to ask AJ anything, and no file in this phase is typed by AJ. If a step can't be done the way the plan says, Cursor follows §0.3 (Snags) and keeps going.

---

## For AJ: this plan in plain English

**What Phase 4 is.** The theory says JoInn can measure the *shape* of an app the way topology measures a surface: count the loops, find the loops nothing closes, name them. The roadmap calls this the **assay** and says: build it, then test whether it catches anything the rest of JoInn can't. If it doesn't, cut it without sentiment.

**Your four decisions are built in:**

1. **R60 = Option A.** New grammar ships its list of test damages before any check may point at it. In this phase that means the new `declarations` line in `.body` and `.universe` files, and one new body damage (`AddWire`).
2. **Kill test first.** Chunk A builds the instrument and checks it on things we already know the answer to. Chunk B runs the "does it catch anything new?" test. Chunk C either wires the assay into JoInn (keep) or removes it (cut). Which one is decided by a rule written in this plan now, before anyone sees the result.
3. **Static counts.** The assay is kept if it names a defect between bodies that no existing check names without running the app on the right inputs.
4. **Test what exists.** The theory's strongest case (H¹, "every body is right but they can't all be right together") needs a way to say two values in different bodies are the same quantity. JoInn has no place to write that yet. Phase 4 writes that down as a finding and opens R65. No new grammar for it.

**What Claude found while writing this, which the plan fixes:**

- **The roadmap's calibration number is wrong.** It says deleting the round-trip law from `cli_input` raises the loop count from 0 to 1. In the real corpus both calculator inputs (`cli_a` and `cli_b`) are the *same* cell, so deleting the law opens **two** loops (0 → 2). To open exactly one, you swap one input for a copy of the cell without the law (0 → 1). The plan tests both.
- **The `lookup` body from Phase 5.2's adversary is two separate things.** Its `question` and `answer` cells aren't wired to each other. The assay will show that, and it explains the 5.2 finding structurally: the question/answer pairing only closes through the host.
- **The theory's H₂ reading contradicts the theory's own rule.** H₂ was meant to spot "specified but unimplemented" cells. But the theory also says an assay must be blind to implementations (alleles). Both can't hold. The plan makes the machine show this, and the H₂ reading is withdrawn.

**The honest prediction:** the assay probably survives on one thing. It names a **missing promise between two bodies**: a round trip across bodies where no law names its partner, even though every body passes on its own. A run can't see this if the two formatters behave the same today. That is exactly a "static counts" keep. If a check we already have catches it, the rule says cut.

**Your prompts to Cursor** (copy exactly, one per chunk):

- Chunk A: `Do chunk A of docs/Plans/JoInn Phase 4 Implementation Plan.md. Follow AGENTS.md.`
- Chunk B: `Do chunk B of docs/Plans/JoInn Phase 4 Implementation Plan.md. Follow AGENTS.md.`
- Chunk C: `Do chunk C of docs/Plans/JoInn Phase 4 Implementation Plan.md. Follow AGENTS.md.`

If Cursor runs out of room partway: `Continue chunk A of docs/Plans/JoInn Phase 4 Implementation Plan.md from the first commit not in git log. Follow AGENTS.md.` (Use the right letter.)

After each stop, tell Claude "Cursor finished chunk A" (or B, C).

---

## 0. How Cursor Works This Plan

| | |
|---|---|
| **Plan file** | `D:\JoInn\docs\Plans\JoInn Phase 4 Implementation Plan.md` |
| **Code** | `D:\JoInn\joinn\`. No new crates, no new dependencies |
| **Standing rules** | `AGENTS.md` as updated by P4-01 (Appendix A) |
| **Unit of work** | a **chunk** (A, B or C). Inside a chunk, do the commits in the order listed. Each commit is its own git commit, and its message starts with its id (`P4-05: …`) |
| **End of a chunk** | write the stop report (§0.2), commit it, `git push`, and stop. Do not start the next chunk |
| **Who decides** | every decision is in §2. Cursor decides only module layout (rule 25), function bodies, error strings, and Rust representation |
| **Who checks** | Claude, at each stop, from a fresh clone on Linux. GitHub CI runs on Windows and Linux on every push |

### 0.1 The commit report

Every commit ends with this block, in the commit message body **and** in the chunk's stop report. Every value is copied from the terminal, never summarised.

```
Commit:     P4-NN (hash in git log)
Done-when:  <the command> → <the line it printed>          MET | NOT MET
Suite:      cargo test --workspace --no-fail-fast → <N passed, M failed>
Scans:      vocab → <last line> · modules → <last line>
Snags:      none | <each thing that went differently from the plan, with the printed line>
```

The stop report carries the real short hash for each commit.

### 0.2 The stop report

At the end of each chunk Cursor writes `docs/Findings/phase-4-stop-<letter>.md` with:

1. `git rev-parse HEAD` (full hash).
2. Every commit report block from the chunk, in order, with real hashes.
3. The last line of each of: `cargo test --workspace --no-fail-fast`, `cargo xtask gate all`, `cargo xtask corpus verify`, `cargo xtask vocab`, `cargo xtask modules`, `cargo xtask assay agree` (from P4-05 on). Also every line of `gate all` that contains `fail`, and every failing test name.
4. **CI**: for the newest run after the push, read with PowerShell `Invoke-RestMethod "https://api.github.com/repos/joneseysinno/JoInn/actions/runs?branch=main&per_page=3"` and that run's `jobs_url`. Paste `head_sha`, `status`, `conclusion`, and each job's `name` and `conclusion`. If the run hasn't finished, wait and read again. A failed job is a snag, not a reason to change CI.
5. **Snags**: every snag from the chunk in one list, each tagged with its commit id. A snag says what the plan actually says.

Then commit (`P4-stop-<letter>: stop report`), push, and stop.

### 0.3 Snags

A snag is any point where a done-when can't be met as written, or a prediction in this plan turns out wrong.

- **Never** fake it, work around a scan, weaken a test, rebless a golden, or change the plan's meaning to make it pass.
- **A prediction that differs is not a failure.** Paste what the machine printed next to what the plan predicted, and continue. Claude settles it at the stop.
- Leave that piece in its honest state and **continue with the next commit**.
- If a later commit depends on the snagged one (§4), skip it and write `skipped: depends on P4-NN`.

---

## 1. Scope Fence

### In scope

- **Exact ranks** (H₀, H₁, H₂) with named cycles over the abstract complex in `joinn-assay`.
- **The derivation**: bodies and universes turned into a complex (§2.2–§2.4), in `joinn-link`.
- **Two derivations, one truth**: a fast one (regions) and a reference one (ports), cross-checked, plus Euler's identity.
- **The invariance harness**: the assay may not move under anything outside the hash boundary or under a lens change.
- **The decoration check**: the phase's adversary, three candidates, a verdict chosen by a rule written now (§2.8).
- **If kept**: declarations on bodies and universes, gate 4, the lock row, the freeze. **If cut**: the cut.
- **Five new corpus files** (§3.1). No existing hash moves.

### Out of scope (Cursor refuses these even when they look small)

| Not now | Why it is tempting |
|---|---|
| Holonomy, H¹ with values, any coefficient other than ℚ | The theory's headline case. Not expressible yet (§2.8 C2, R65) |
| A `laws` section in `.universe`, or any claim that spans bodies | That is R65, a later phase |
| Declarations in a `.cell` | A cell's own complex has no loops (§2.2), so a cell declaration could never refuse (rule 40). The gate's refusal of a non-empty cell declaration stays |
| An `AssayRegistry` in `joinn-gate` | Phase 5 §2.3 planned one for cell declarations. There are none (row above), so it has no caller. Declarations are checked where bodies and universes are admitted (§2.9) |
| A new file kind | None is needed. R60 still applies to the new grammar (§2.10) |
| Links with more than two members in the assay | None exist in the corpus. Such a link is printed `not measured` (R67) |
| Cycles with more than two frame changes | Printed `not measured` (R66) |
| Upgrading legacy gates 1–3 | R64 |
| Persistent or incremental homology, Mayer–Vietoris gluing | Not needed at this size |
| Any renderer, highlighted loop, or pixel | Phase 6+ |
| A new primitive, a native, a line in `grandfather.txt`, a reblessed golden | Never |

---

## 2. Decisions

All **DECIDED**.

### 2.1 Where things live

- **`joinn-assay`** gains `homology(&Complex) -> Verdict<Homology>`: exact ranks b₀, b₁, b₂ and canonical cycle representatives. It still depends on `joinn-frame` alone and names no DNA type. The Phase 5 scan `assay_names_no_homology` is replaced by `assay_names_no_dna` (bans `Body`, `Cell`, `Universe`, `Wire`, `Genome`, `joinn_dna`, `joinn_link` as identifiers in `joinn-assay`). `assay_depends_on_frame_alone` and `gate_names_no_assay_type` stay.
- **`joinn-link`** gains the derivation (`assay/`): it turns a bound universe into a `Complex`, calls `homology`, and returns an `AssayReport`. This corrects one sentence of the Phase 5 plan (§2.2 said the derivation would go in `joinn-assay`). That can't happen without a crate cycle, because `joinn-link` already depends on `joinn-assay`.
- **`xtask`** gains `cargo xtask assay <path>`, `cargo xtask assay --all`, `cargo xtask assay agree` and `cargo xtask decoration`. Rule 17 holds: xtask prints, and findings are written from what it printed.

**Arithmetic.** Ranks are computed over ℚ by fraction-free (Bareiss) elimination on `i128` with checked operations. Overflow is a refusal: `complex too large for exact elimination at <V> blocks; acceptance is a complex whose elimination fits i128`. No floats (rule 3). Ranks over ℚ are the torsion-free ranks; torsion isn't reported (the Klein bottle fixture pins this).

### 2.2 What the complex is made of (the fast derivation)

A body is assayed as the universe that holds only it, and its alias is `body`.

| Block | Made from | Why |
|---|---|---|
| **0-block: a region** | each connected piece of a body's inside (cells joined by wires, ignoring direction) | Inside a body, the engine is deterministic (G4), so two wire paths from the same value can't disagree. The inside is filled, and each connected piece shrinks to one point. The number of pieces per body is its **regions** count |
| **0-block: outside** | one block for the whole world beyond the universe's boundary | S1's `host` vertex. It is the cone on the boundary |
| **1-block: entry** | outside → region, one per in-port on the universe's boundary | a value the world hands in |
| **1-block: exit** | region → outside, one per out-port on the universe's boundary | a value the world reads |
| **1-block: link** | tail region → head region, one per two-member link | a hyperedge between bodies is a 1-block (k+1 = 2 ports) |
| **2-block: filling** | §2.3 | a law, or a frame, that closes a loop |

A port is on the universe's boundary when it is on ∂(body) (rule 30) and no link names it.

**Why this is the right instrument and not decoration.** Every loop in this complex either goes through the outside or crosses between bodies through links. A loop that stays inside one body was filled by determinism. So everything the assay reports is **at or across a membrane**, which is the only place Part III §9.6 said the assay could beat `require` and `ensure`. On a single body with no links, the assay can only say whether the body's round trips through the outside are promised. That is useful calibration, and chunk B treats it as calibration, not as the argument.

### 2.3 What fills a loop

A filling is a 2-block whose boundary is one specific loop, and it **names what made it**. There are exactly three rules:

1. **Frame identity.** An entry port *p* and an exit port *q* with a directed path from *p* to *q*, where *p* and *q* have the same frame. The loop outside → *p* ⇒ *q* → outside is filled `by frame <F>`. A value that goes in as ℤ and comes out as ℤ needs no conversion by the outside to close.
2. **A round trip at the outside.** The same, but *p* has frame *F_in* and *q* has frame *F_out* ≠ *F_in*. The outside has to convert *F_out* back to *F_in* to close the loop. The loop is filled when some cell instance in a region the path passes through carries a law of exactly one of these two shapes (quantifier over one variable `n`, nothing else):
   - `self@o(i: cell:h@o'(i': n)) = n`
   - `cell:h@o'(i': self@o(i: n)) = n`

   The law must be carried by **the cell instance that owns *p*, or the one that owns *q*** (checked in that order). A law on some other cell in the same region does not count, because the promise belongs to the cell that does the converting. The cell `h`'s in-port `i'` must have frame *F_out* and its out-port `o'` must have frame *F_in*. That is, the law promises that the conversion the outside does is undone. It is filled `by <alias>.<instance> <law name>`, naming the instance that carried it. `cli_input`'s `roundtrip` law, `self@1(0: format@1(0: n)) = n`, is the first shape with `h` = `format` (ℤ → Text), so it fills Text-in / ℤ-out loops. An implication such as `canonical` fills nothing.
3. **A loop between bodies.** A loop in the complex with the outside removed. Candidates are the fundamental cycles of a canonical spanning forest (§2.5). Along the loop, list each region's frame change from where the loop enters to where it leaves.
   - **No change:** filled `by frame <F>`.
   - A change happens where the loop crosses a region and the port it enters by has a different frame from the port it leaves by. The change is **done by** the instance that owns the entering port, and failing that the one that owns the leaving port (the same attribution as rule 2).
   - **Exactly two changes** (*F* → *G* done by cell instance *x*, then *G* → *F* done by cell instance *y*): filled when *x* carries a round-trip law of rule 2's shape whose `h` is **y's cell hash**, or *y* carries one whose `h` is **x's cell hash**. Filled `by <alias>.<instance> <law name>`. **The promise must name its partner by hash.**
   - **More than two changes:** printed `not measured` with the loop, and counted separately (R66).

Nothing else fills a loop. Two fillings with the same boundary both stay (that is what makes H₂).

**For a ★-pair with several directed paths**, the filling uses the canonical path (§2.5). The other paths differ from it by loops that rule 3 judges on their own.

### 2.4 What the report says

`AssayReport` is a value with a canonical printer. For `calculator.body`, Claude computed the exact expected output by hand from §2.2–§2.5:

```
assay body
regions: body 1
islands: 1
loops: 2
filled: outside →body.cli_a@0→ body →body.sum@2→ outside by body.cli_a roundtrip
filled: outside →body.cli_b@0→ body →body.sum@2→ outside by body.cli_b roundtrip
not measured: 0
H₀: 1
H₁: 0
H₂: 0
euler: V 2 − E 3 + F 2 = 1 = 1 − 0 + 0
```

- **regions**: one line per body alias, the count of its inside pieces. Where the count is more than 1, the line also lists, for each piece, its instances in name order, e.g. `regions: lookup 2 {question} {answer}`.
- **islands**: connected pieces of the complex with the outside removed.
- **loops**: rank of the cycle space (the dimension of Z₁).
- **filled** and **open**: one line per filling, then one line per H₁ representative (`open: …`, §2.5).
- **H₀** is computed *with* the outside, so it is 1 whenever anything touches the boundary. It is printed because Euler needs it; the readings that matter are regions and islands.
- **euler**: V − E + F on the left, b₀ − b₁ + b₂ on the right. They must be equal (V112). An inequality is a refusal, never a printed line.

A step `→x.y@n→` names the port by alias, instance and position. Display names and labels never appear (rule 9).

### 2.5 Canonical order (so the same universe always prints the same loops)

- **Edge key**: (kind: entry < exit < link, then body coding hash, then instance, then port position; ties between bodies with the same coding hash are broken by alias). Link ids never enter a key.
- **Spanning forest**: breadth-first from the outside, then from each remaining region in key order, taking edges in key order.
- **Fundamental cycles** in the order their non-tree edge appears.
- **H₁ representatives**: walk the fundamental cycles in order. Keep a cycle when it raises the rank of (the span of every filling boundary plus the cycles kept so far). The kept cycles print as `open:` lines. Their number is b₁.
- **Canonical path** from *p* to *q*: shortest by edge count, ties broken by the sequence of edge keys.

Under `RenameAlias` or `RenameLink` the printed report changes only in the renamed names (V114). A test shows it for `RenameAlias(units, meters)` and `RenameLink(e0, e1)` on `universe.universe`.

### 2.6 Two derivations, one truth (Law 6, V32)

§9.8 of Part III admits an assay only with a reference implementation, a corpus of known findings and a cross-check. The **reference derivation** keeps every port:

- 0-blocks: every port of every cell instance; one centre per cell instance; the outside.
- 1-blocks: port–centre (in-port → centre, centre → out-port); every wire; every two-member link; entries and exits at the boundary ports.
- 2-blocks: **determinism fillings**, one per fundamental cycle of each body's inside (the inside is filled, §2.2), plus §2.3's three rules applied to loops through the same boundary ports and links.

`cargo xtask assay agree` runs both derivations on every body and universe in the corpus that binds. It requires equal islands, equal regions, equal b₁ and b₂, and the same `open:` and `filled:` lines once each reference loop is shortened to its boundary steps (entries, exits, links). Euler must hold in both. It also runs one planted disagreement (the fast derivation with one filling removed) and must refuse it, printing `injected disagreement: refused as truth violation (ok)`. A disagreement on the corpus is a truth violation, not a test failure: it prints both reports and exits 1. CI gains an `assay agree` step after `agree`.

### 2.7 The assay is blind to what it must be blind to (V114)

For every subject that binds, the invariance harness requires the printed report to be **identical** under:

1. the kind's neutral edit (regulatory label changed);
2. every cell the subject uses stripped of **all** its alleles (alleles are outside the hash boundary, V19);
3. each `DropLens` that leaves at least one lens (a lens is a camera, Part I §10.2);
4. `RenameAlias` and `RenameLink`: identical once the renamed names are mapped back.

**Opposite fixture:** a fake assay that appends the count of regulatory labels to its report must be **refused** by the harness, naming edit 1. That proves the harness can see a phenotype reader. *(Corrected by Amendment A, P4-07c: a count cannot be seen by a text edit. The fake prints every name and label it was handed.)*

### 2.8 The decoration check (the adversary), and the rule that decides it

The roadmap's adversary for Phase 4: *does the assay catch anything `require` and `ensure` cannot?* Three candidates were claimed. Each is run or reasoned in chunk B, and **the verdict is chosen by this rule, written before anyone sees the output:**

> **KEEP** if, for candidate C1, the assay's answer on the subject differs from its answer on the mutant, and **no other check in the tree** does (every existing admission, typing, Law 4, lens and binding check, every declared `require`/`ensure`, and a run on the inputs in §2.8 C1). Catching it without a run counts (AJ, 26 Sep: *static counts*). **CUT** otherwise.

A check **distinguishes** only if it answers differently on subject and mutant. Refusing both, or admitting both, is not distinguishing (the Phase 5.2 control rule).

**C1 · A promise that doesn't name its partner (H₁ across bodies).** `phase4/loop.universe` (§3.1) joins the calculator to a formatter body in a ring: `calc.sum@2 → fmt → calc.cli_a@0`. The ring converts ℤ → Text in `fmt` and Text → ℤ in `cli_a`. `cli_input`'s `roundtrip` law names `format`'s hash, and `fmt` holds `format`, so rule 3 fills the ring. The mutant is `SwapBinding(fmt, <fmt_twin hash>)`. `fmt_twin` holds `format_twin`: the same cell with its one law renamed (`functionality` → `deterministic`). It behaves identically, passes the gate on its own, and is a different cell by content, so no law promises that `cli_input` undoes it.

- **Claude's prediction:** assay `H₁: 0` on the subject, `H₁: 1` on the mutant, with one `open:` line naming the ring through links `back` and `again`. Every other check answers the same on both. The run (test host, rounds `[[cli_a "2", cli_b "3"]]`) prints identical reports on both, because `format` and `format_twin` run the same native.
- **What it shows:** the assay names a missing promise between two bodies, statically, when every body is lawful on its own. It does not show a wrong number, because today there is none. That is the honest size of the claim.

**C2 · Every body right, the whole inconsistent (H¹ with values).** Not built. A value-consistency check needs (a) a factor stated in DNA, and (b) a coding-level claim that two values in different bodies are the same quantity. Today (a) comes from outside (`units` reads `factor: 12` from the host, as `universe.txt` shows), and (b) has no home except a `require` at one membrane that sees both values, which is local by definition. Cursor writes the finding from the evidence commands in P4-09. It opens **R65 · Laws that span bodies**.

**C3 · Laws around an empty inside (H₂ as "specified but unimplemented").** Withdrawn by the invariance rule. "Unimplemented" means "has no allele". The invariance harness (§2.7 edit 2) strips every allele and requires the report not to move. So an admissible assay **cannot** see implementation, and the theory's H₂ reading (Part III §9.5) contradicts Part III §9.8. H₂ stays as a number: it counts loops filled twice. Cursor writes the finding from P4-06's printed output for `calculator.body`.

### 2.9 Declarations (chunk C, only if KEEP)

- `.body` and `.universe` coding regions gain an optional section, printed **only when non-empty**, so no existing hash moves:

  ```
  declarations {
    assert H₁ = 0
  }
  ```

  `assert H₁ = 0` is the only sentence Phase 4 admits. Any other text is refused at parse time, naming the text and `acceptance is assert H₁ = 0`.
- **Where it is checked:** a body's declaration at `BodyStore::insert`, by assaying the body alone. A universe's declaration inside `assemble_universe`, after ∂∂. The refusal names the declaration and every `open:` loop.
- **An assay alone never refuses** (rule 49). Without a declaration the same loop is printed by `cargo xtask assay` and admission is unchanged.
- **The gate keeps four checks.** This answers R3's open question: a declaration is checked when a body or universe is admitted, not as a fifth gate check. A cell with a non-empty `declarations` is still refused by the gate, as today.

### 2.10 R60 decided: new grammar ships its mutants

**Decided 26 Sep 2026 (AJ): Option A.** A new file kind, or new grammar in an existing kind, ships with its parser, its canonical printer, the catalogue mutations its gate items oppose, and its neutral edit, each with a test that asserts what the mutant *does*, before any control may point at an artifact that uses it.

For Phase 4:

- **`DropDeclaration`** for `.body` and `.universe` (removes the `assert H₁ = 0` line; the section disappears when empty). Its downstream effect: the `fmt_twin` mutant of `loop_declared.universe` is refused, and after `DropDeclaration` the same universe assembles.
- **`AddWire(src, dst)`** for `.body`. Its downstream effect: on `asker.body`, `AddWire(question@2, answer@0)` makes the regions line `regions: body 1`.

No gate item in this phase opposes a `.cell` mutation. The calibration swaps a cell through the body (`SwapCell`), which already has a catalogue. `.cell` stays legacy (R64).

---

## 3. Architecture

```rust
// joinn-assay (depends on joinn-frame only)
pub struct Homology { pub b0: u32, pub b1: u32, pub b2: u32, pub open: Vec<Chain> }
impl Complex { pub fn homology(&self) -> Verdict<Homology>; }

// joinn-link
pub struct AssayReport { /* regions, islands, loops, filled, open, not_measured, b0, b1, b2, euler */ }
pub fn assay(u: &Universe, bound: &Bound) -> Verdict<AssayReport>;            // fast
pub fn assay_reference(u: &Universe, bound: &Bound) -> Verdict<AssayReport>;  // reference
pub fn print_assay(r: &AssayReport) -> String;                                 // canonical
```

`assay` never refuses because of what it finds (rule 49). It refuses only for its own failures: binding, arithmetic overflow, Euler inequality.

### 3.1 New corpus files (all under `corpus/phase4/`)

| File | What it is | Hash |
|---|---|---|
| `cli_input_open.cell` | `phase0/cli_input.cell` with the `roundtrip` law deleted, nothing else changed | new, recorded |
| `format_twin.cell` | `phase0/format.cell` with the law name `functionality` → `deterministic`, nothing else changed | new, recorded |
| `fmt.body` | `genome { cell:<format hash> as fmt }`, `grants { }`, `wires { }`, `budget { steps 100000 }`, `lineage none`; regulatory `names { fmt "Format" }` | new, recorded |
| `fmt_twin.body` | `fmt.body` with `<format_twin hash>` | new, recorded |
| `loop.universe` | text below | new, recorded |
| `loop_declared.universe` (P4-11, only if KEEP) | `loop.universe` plus `declarations { assert H₁ = 0 }` | new, recorded |

**`loop.universe`** (copy exactly; only `<fmt hash>` changes):

```
universe {
  codex 1
  bodies {
    body:b55fba1eff65942099f6daf84b8bc47d605be05f637d805d260d3fcfd8c3ebde as calc
    body:<fmt hash> as fmt
  }
  links {
    link back order none {
      calc.sum@2 tail
      fmt.fmt@0 head
    }
    link again order none {
      fmt.fmt@1 tail
      calc.cli_a@0 head
    }
  }
  lenses {
    lens function {
      galaxy app {
        system s { calc fmt }
      }
    }
  }
}
```

`corpus verify` goes from 37 to 42 hashes after P4-03 (43 after P4-11). **No existing hash may move.** A moved hash is a snag, and the commit is not continued past.

### 3.2 Claude's predictions (computed by hand from §2.2–§2.5)

| Subject | regions | islands | loops | fillings | H₁ | H₂ | Euler V − E + F |
|---|---|---|---|---|---|---|---|
| `phase2/calculator.body` | body 1 | 1 | 2 | 2 (`roundtrip` ×2) | 0 | 0 | 2 − 3 + 2 = 1 |
| same, `SwapCell(cli_b, <cli_input_open>)` | body 1 | 1 | 2 | 1 | **1**: `outside →body.cli_b@0→ body →body.sum@2→ outside` | 0 | 2 − 3 + 1 = 0 |
| same, both `cli_a` and `cli_b` swapped | body 1 | 1 | 2 | 0 | **2** | 0 | 2 − 3 + 0 = −1 |
| `phase5/universe.universe` | calc 1, units 1 | 1 | 3 | 3 (`roundtrip` ×2, `frame ℤ`) | 0 | 0 | 3 − 5 + 3 = 1 |
| `phase52/adversary/ask.universe` | lookup **2** `{question} {answer}`, units 1 | 1 | 4 | 4 (`frame ℤ` ×4) | 0 | 0 | 4 − 7 + 4 = 1 |
| `phase4/loop.universe` | calc 1, fmt 1 | 1 | 1 | 1 (`calc.cli_a roundtrip`) | 0 | 0 | 3 − 3 + 1 = 1 |
| same, `SwapBinding(fmt, <fmt_twin>)` | calc 1, fmt 1 | 1 | 1 | 0 | **1** | 0 | 3 − 3 + 0 = 0 |

`ask.universe` is worth reading closely. `lookup`'s two cells aren't wired together, so it has two regions, and the question → `units` → answer path is a path, not a loop. The loop closes only through the outside, in the host's rounds. That is Phase 5.2's adversary finding, now seen from structure alone.

Allele bodies (`phase21/*_ref.body`, `phase22/*`) are assayed and recorded, but not predicted here. The round-trip promise for `format` and `parse` lives in the cells that *carry* those alleles, not in the allele bodies, so an allele body that converts Text ↔ ℤ is expected to print an `open:` loop. That is R68, not a defect.

---

## 4. The Commits

Done-when is a command, and the command must be able to fail.

### Chunk A: the instrument, calibrated

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P4-01** | **Plan and rules** | Commit this plan. Update `AGENTS.md` per Appendix A. In `docs/Findings/decisions.md`: R60 row → `decided`, `Option A (AJ, 26 Sep): new grammar ships its mutants`. In the research backlog, R60 status → decided, with the same sentence | `git show --stat HEAD` lists the plan, `AGENTS.md`, `decisions.md` and the backlog, and nothing under `joinn/crates` |
| **P4-02** | **Exact ranks** | `Complex::homology` (§2.1, §2.5). `assay_names_no_homology` replaced by `assay_names_no_dna` | Tests on fixture complexes, each asserting (b₀, b₁, b₂) and Euler: filled triangle (1, 0, 0); hollow triangle (1, 1, 0) with its one `open` cycle named; tetrahedron surface (1, 0, 1); two points (2, 0, 0); a triangulated Klein bottle (1, 1, 0) over ℚ (the triangulation is written out in the test); an elimination sized to overflow `i128` → refused naming the block count. `cargo test -p joinn-assay` passes; paste its result line |
| **P4-03** | **Corpus files** | §3.1's first five files; `hashes.txt` and a new `docs/Findings/phase-4-hashes.md` | `cargo xtask corpus verify` → `42 hash(es) match`. `git diff --stat <P4-02>..HEAD -- joinn/corpus` shows only added files and `hashes.txt` additions. The gate admits `cli_input_open.cell` and `format_twin.cell` (paste the admission lines). `loop.universe` binds and assembles (paste the lines) |
| **P4-04** | **The derivation** | `assay`, `AssayReport`, `print_assay`; `cargo xtask assay <path>` | `cargo xtask assay` on each subject in §3.2 (mutants via a test that applies the catalogue mutation and prints the report). Paste every report. Each must match §3.2, or it is a snag with both versions pasted. `calculator.body` must print §2.4's block byte for byte |
| **P4-05** | **Two derivations, one truth** | `assay_reference`; `cargo xtask assay agree`; CI step | `cargo xtask assay agree` prints one line per subject and ends `assay agree: <n> subject(s) agree; injected disagreement: refused as truth violation (ok)`. Paste the whole output |
| **P4-06** | **Blind to what it must be blind to** | the invariance harness (§2.7) as `cargo xtask assay invariance`, with the phenotype-reader fixture | Output lists every subject with `ok` for edits 1–4, then `phenotype reader: refused at edit 1 (ok)`. Paste the whole output, including `calculator.body`'s allele-strip line (C3's evidence) |
| **P4-07** | **The corpus, assayed** | `cargo xtask assay --all`; `docs/Findings/assay-corpus.md`: the command, its full output pasted, and a section *Predictions* comparing §3.2 row by row, marked `as predicted` or `differs` with both values | Every §3.2 row is marked. The finding contains the sentence *"The roadmap's Phase 4 exit gate predicted 0 → 1 on deleting `roundtrip`. `cli_a` and `cli_b` are one cell, so the machine prints 0 → 2; swapping one instance prints 0 → 1."* followed by the two printed `H₁` lines |
| — | **Stop A** | `phase-4-stop-a.md` (§0.2), push, stop | — |

Dependencies: in order. P4-04 needs P4-02 and P4-03. P4-05 and P4-06 need P4-04.

### Amendment A (after Stop A, 26 Sep 2026): do these first in chunk B

Claude re-ran the tree at `9787be1` from a fresh clone on Linux (rustc 1.95.0). Every number in `phase-4-stop-a.md` reproduced: 193 passed, 0 failed; `gate all` exits 0 with every phase line as reported; `corpus verify` 42 hashes, and only the five new files and their `hashes.txt` lines were added; `vocab: ok`; `modules: ok (enforced 11 crate(s))`; `cargo fmt --check` and `cargo clippy -D warnings` clean; `assay agree` 37 subjects plus the plant; every §3.2 report matches by hand, including §2.4's block byte for byte. The instrument is right. The invariance harness is not yet able to fail, and these four commits fix that **before P4-08**. They are decided; nothing waits on AJ.

**Why.** (1) The P4-06 snag was right, and the error was Claude's: §2.7's fake assay appended the *count* of labels, and the neutral edit changes a label's *text*, so no harness could refuse it. (2) Worse, edit 1 never reaches the assay for a body. The harness wraps the edited body into a universe that carries only its hash and alias, then binds it against the store, which still holds the *original* body. The edited universe is the same value as the baseline, so edit 1 on every body compares a report with itself. (3) 26 of the 37 measured subjects have no name or label, so `neutral()` returns nothing and edit 1 is skipped, yet the line prints `1 ok`. The same goes for edit 3 (only `universe.universe` has two lenses) and edit 4 on bodies. That includes `loop.universe`, the subject gate 4 items 2 and 4 depend on.

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P4-07a** | **Amendment A** | Commit this plan file as it is on disk. Plan text only | `git show --stat HEAD` lists only this plan |
| **P4-07b** | **Every edit reaches the assay, and says when it didn't run** | In `xtask/src/fns/assay/invariance.rs`: **(1)** For a body subject, edit 1 inserts the edited body, with the cell map `bind` gave for the original, into a **fresh** `BodyStore`, and assays against that store (the way edit 2 already does), so `bind` hands the assay the edited body. **(2)** When `neutral()` returns nothing, the harness adds a name: for a body, `names { <first instance in name order> "neutral" }`; for a universe, `names { <first alias in name order> "neutral" }`, reprinted and reparsed as `neutral()` does. This lives in the harness only. `neutral()` is not changed, so no gate control moves. **(3)** Each line says what ran: edit 1 prints `1 ok` or `1 ok (added name)`; edit 2 prints `2 ok` (`allele strip ok` stays on `calculator.body`); edit 3 prints `3 ok` only when at least one lens was dropped, otherwise `3 n/a (one lens)` or `3 n/a (no lens)`; edit 4 prints `4 ok` for a universe and `4 n/a (body)` for a body. Never print `ok` for an edit that did not run | `cargo xtask assay invariance`, whole output pasted. It contains exactly `phase4/loop.universe: 1 ok (added name), 2 ok, 3 n/a (one lens), 4 ok`, `phase5/universe.universe: 1 ok, 2 ok, 3 ok, 4 ok` and `phase2/calculator.body: 1 ok, allele strip ok, 3 n/a (no lens), 4 n/a (body)`. A unit test asserts that, for `calculator.body`, the universe the harness assays after edit 1 binds to a body whose regulatory region differs from the original (the fix, asserted by what it does) |
| **P4-07c** | **The phenotype reader reads the phenotype** | Replace the fake assay: it prints the real report, then every name and label of every regulatory region it was handed, one per line in key order. That means each bound body's region, read from `Bound` (the same value the real assay receives), and the universe's own region. Run it on `phase2/calculator.body` and on `phase4/loop.universe`. The harness must refuse it at edit 1 on both. The command exits 1 if either is accepted | The output ends with the two lines `phenotype reader: refused at edit 1 on phase2/calculator.body (ok)` and `phenotype reader: refused at edit 1 on phase4/loop.universe (ok)`, and `cargo xtask assay invariance` exits 0. Also shown then reverted: undo P4-07b's fresh store (bodies bound against the original store again); the calculator line becomes `phenotype reader: accepted at edit 1 on phase2/calculator.body`. Paste both printed lines in full |
| **P4-07d** | **Agree names what it skips; two plants** | `cargo xtask assay agree`: a subject that parses but doesn't bind, or that both derivations refuse, prints `<rel>: not measured: <reason>` (one reason when both derivations print the same one, else both) instead of being skipped silently. Add a second plant on `calculator.body`: the fast report with `b1` raised by one and nothing else changed. Both plants must be refused. The last line becomes `assay agree: <n> subject(s) agree, <m> not measured; injected disagreements: 2 refused as truth violation (ok)` | Whole output pasted. It contains `phase52/controls/missing_cell.body: not measured: instance orphan cell 4a37 was not supplied; acceptance is that cell in the cell map` and ends `assay agree: 37 subject(s) agree, <m> not measured; injected disagreements: 2 refused as truth violation (ok)` |

**Changes to later commits and to the stop report:**

- **§2.7's opposite fixture** is P4-07c's fake from now on. Where this plan says `phenotype reader: refused at edit 1 (ok)`, read P4-07c's two lines.
- **§0.2 item 3** also lists the last line of `cargo xtask assay invariance`, and its exit code.
- **P4-09's C3 evidence** is the `calculator.body` line from P4-07b's output (`1 ok, allele strip ok, 3 n/a (no lens), 4 n/a (body)`).
- **P4-05's snag is accepted.** A link whose two members are in one region is a loop (its boundary is zero), so summing a repeated block in `Chain::from_coeffs` is right. `missing_cell.body` is not measured, because the body it binds names a cell nobody supplied. P4-07d prints that instead of skipping it.
- **Gate 4 (P4-13)**: "every item also passes §2.7's invariance on its subject" means P4-07b's edits, with `ok` or `n/a` as they print. An item whose subject prints `1 ok (added name)` is fine.

Dependencies for chunk B: P4-07a, then P4-07b, P4-07c, P4-07d in that order, then P4-08, P4-09, P4-10. P4-07c needs P4-07b.

### Chunk B: the decoration check

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P4-08** | **C1 run** | `cargo xtask decoration`: for `loop.universe` and its `SwapBinding(fmt, <fmt_twin hash>)` mutant, one line per check, `<check>: <subject answer> | <mutant answer> | same|differs`. Checks, in this order: gate admission of every cell in every bound body; `BodyStore::insert` of each body; `bind`; `assemble_universe`; `check_link_types`; `check_law4`; `check_lenses`; every `require` and `ensure` declared in any contract used (print `none declared` when the maps are empty); the test host run in rounds `[[("calc", cli_a "2"), ("calc", cli_b "3")]]` (print the reports); the assay (print `H₁`). The last line is `distinguishing: <comma-separated check names>` | Paste the whole output |
| **P4-09** | **C2 and C3 evidence** | No code. Cursor collects: (C2) `git grep -n "factor" -- joinn/corpus/transcripts joinn/corpus/phase5/units.body`, and `git grep -n "laws" -- joinn/corpus/phase5/*.universe joinn/corpus/phase4/*.universe` (expected: no lines); (C3) the allele-strip line for `calculator.body` from `cargo xtask assay invariance` | The three outputs pasted in the commit report |
| **P4-10** | **The verdict** | `docs/Findings/decoration-check.md`: the rule of §2.8 copied verbatim; P4-08's output pasted; the verdict **KEEP** if and only if the `distinguishing:` line is exactly `assay`, otherwise **CUT**; then the matching text from Appendix B for C1, and Appendix B's C2 and C3 texts with P4-09's outputs quoted. Backlog: add R65–R68 (§8); under R17, add *"Phase 4: decoration check → <KEEP/CUT>, see Findings/decoration-check.md"*. `decisions.md`: R65–R68 rows `open`; a row `P4-adv | decoration check | 4 | <held/fired> | Findings/decoration-check.md` (`held` for KEEP, `fired` for CUT) | The finding's verdict word matches the rule applied to P4-08's last line. `git grep -n "R65" -- docs` shows the backlog and `decisions.md` |
| — | **Stop B** | `phase-4-stop-b.md`, push, stop | — |

Dependencies: Amendment A's P4-07a–d come first (see above). Then in order. Chunk B needs chunk A's P4-04 and P4-06.

### Chunk C, if the verdict is KEEP: declarations, gate 4, freeze

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P4-11** | **Declarations** | §2.9: grammar, printer (omit when empty), parse refusal for other text, checks at `BodyStore::insert` and `assemble_universe`; `loop_declared.universe` and its hash | `corpus verify` → `43 hash(es) match`, no other hash moved. Test: `loop_declared.universe` assembles; its `SwapBinding(fmt, <fmt_twin>)` mutant is refused naming `assert H₁ = 0` and the `open:` loop; `loop.universe`'s same mutant assembles (an assay alone never refuses). A body with `assert H₂ = 0` is refused at parse naming the text. Paste the three printed lines |
| **P4-12** | **R60 for the new grammar** | `DropDeclaration` (`.body`, `.universe`) and `AddWire` (`.body`) in the catalogue, with neutral edits unchanged | One test per mutation asserting §2.10's downstream effect (never only that the mutant differs); `AddWire` on a missing instance is refused naming it |
| **P4-13** | **Gate 4** | the four items of §6, each with `opposes` per §6; the harness also checks §2.7's invariance on every gate 4 subject; the uniqueness check covers gates 4, 5, 5.1 and 5.2; lock row `phase 4: 4/4` printed between `phase 3` and `phase 5`; the phase label from the `const` in `gate_all.rs` | `cargo xtask gate all` exits 0 and prints `phase 4: 4/4`, and phases 5, 5.1, 5.2 still print 8/8, 4/4, 3/3. Giving item 4 the same `(artifact, opposes)` as item 2 refuses naming both, shown then reverted (paste both printed lines in full) |
| **P4-14** | **Docs and freeze** | `phase-4-hashes.md` final; README and `Guides/03-where-we-are.md` (the assay exists, what it measures, what it doesn't); `Guides/05-glossary.md` gains *assay, region, island, filling, declaration*; `decisions.md` rows V112–V118 `holds`; R3 note: *"declarations are checked at admission of bodies and universes; the gate keeps four checks (Phase 4)"*; the roadmap's Phase 4 section gains a dated note pointing at `assay-corpus.md` for the 0 → 2 correction | `cargo xtask gate all` from a fresh clone prints the fixtures, then phases 0, 1, 2, 2.1, 2.2, 3 (legacy), 4, 5, 5.1 and 5.2, and exits 0. `corpus verify` 43. `assay agree` and `assay invariance` pass. `phase-4-hashes.md` confirms no Phase 0–5.2 hash moved |
| — | **Stop C** | `phase-4-stop-c.md` with the CI read (§0.2 item 4), push, stop | — |

### Chunk C, if the verdict is CUT: the cut

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P4-11x** | **Cut the layer** | Delete `Complex::homology`, `joinn-link`'s `assay/`, and the xtask `assay` and `decoration` commands with their tests. Restore `assay_names_no_homology`. **Keep**: ∂, `assemble`, rule 40, the five corpus files (they stay valid DNA), every finding. CI's `assay agree` step removed | `git grep -n "homology" -- joinn/crates joinn/xtask` shows only the restored scan. `gate all` exits 0 with 8/8, 4/4, 3/3. `corpus verify` 42 |
| **P4-12x** | **Record the cut** | `decisions.md`: D7 applied, *"assay layer cut after Phase 4's decoration check; ∂∂ kept"*; the roadmap's Phase 4 section gains a dated note; Guides/03 updated; R17 status → `cut (Phase 4)` | `git grep -n "cut (Phase 4)" -- docs` prints the backlog line |
| — | **Stop C** | `phase-4-stop-c.md` with the CI read, push, stop | — |

If something has to be cut for time inside the KEEP path, cut P4-12's `AddWire` together with gate 4 item 3, and carry both forward with a line in `decisions.md`. Never cut P4-05, P4-06 or P4-10.

---

## 5. Test Strategy

| # | Invariant | Commit |
|---|---|---|
| **V112** | Ranks are exact over ℚ, and b₀ − b₁ + b₂ = V − E + F on every complex, or the assay refuses | P4-02 |
| **V113** | The fast and reference derivations agree on every corpus subject; an injected disagreement is a truth violation | P4-05 |
| **V114** | An assay report does not move under a neutral edit, an allele strip, a lens drop, or a rename (apart from the renamed names) | P4-06 |
| **V115** | An assay never refuses on its own; only a declaration refuses | P4-04, P4-11 |
| **V116** | Every filling names the law or frame that made it; nothing fills a loop except §2.3's three rules | P4-04 |
| **V117** | New grammar ships its mutants before a control points at it (R60) | P4-12 |
| **V118** | `joinn-assay` depends on `joinn-frame` alone and names no DNA type | P4-02 |

Carried forward and re-run on every commit: every earlier invariant. V16 (touch-only by ∂∂) is untouched: `assemble` is not changed by this phase except for the declaration step that runs after it.

**A test must never** assert only that a report printed, or that a mutant differs. It asserts the number or the named loop.

---

## 6. Exit Gate 4 (KEEP path)

`cargo xtask gate 4`, from a fresh clone, after the harness fixtures pass. Every item also passes §2.7's invariance on its subject.

- [ ] **1 · The instrument reads a known sample.** `phase2/calculator.body` prints `H₁: 0`, both loops filled by `roundtrip`. *Opposes* `SwapCell(cli_b, <cli_input_open hash>)`. The control answers `true` when `H₁ ≠ 0`.
- [ ] **2 · A promise names its partner.** `phase4/loop.universe` prints `H₁: 0`, the ring filled by `calc.cli_a roundtrip`. *Opposes* `SwapBinding(fmt, <fmt_twin hash>)`. The control answers `true` when the report has an `open:` line.
- [ ] **3 · A body that is two things is named.** `phase52/adversary/asker.body` prints `regions: body 2 {question} {answer}`. *Opposes* `AddWire(question@2, answer@0)`. The control answers `true` when regions is 1.
- [ ] **4 · A declaration refuses; an assay doesn't.** `phase4/loop_declared.universe` is admitted. *Opposes* `SwapBinding(fmt, <fmt_twin hash>)`. The control answers `true` when assembly refuses naming `assert H₁ = 0`.

### 6.1 Conditions for opening Phase 6

1. Gate 4 passes (KEEP) or the cut is recorded (CUT); `gate all` exits 0; CI is green on Windows and Linux (read, not assumed).
2. `assay-corpus.md`, `decoration-check.md` and the R65–R68 entries exist.
3. Claude's stop-C review lists no open snag, or AJ has chosen in conversation to carry each remaining one forward.

---

## 7. Risks

| Risk | What to do |
|---|---|
| The filling rules are written to the examples | They were written before any code, and the corpus sweep (P4-07) runs them on everything. A surprising `open:` line is recorded, not explained away |
| Rule 3's fundamental cycles depend on the spanning forest | Both derivations use the same rule, so Law 6 can't catch this. Canonical order (§2.5) makes it deterministic. A basis-dependent result found anywhere is a snag and becomes part of R66 |
| C1 is judged a "real bug" when behavior is identical | The finding says exactly what it shows: a missing promise, not a wrong number (Appendix B). AJ chose that static detection counts |
| An existing check turns out to distinguish C1 | Then the rule says CUT, and chunk C cuts. That is the phase working |
| Elimination overflows on a real body | It refuses with the block count. Report it; don't switch arithmetic |
| A hash moves | Only new files get hashes. Any other move stops the commit (rule 6) |
| `link again`'s head at `calc.cli_a@0` (a stdin-granted port) is refused by assembly or typing | Snag at P4-03 with the printed line. Then move the head to `calc.cli_b@0` everywhere in this plan and continue |

---

## 8. Open Items

| ID | Topic | Question |
|---|---|---|
| **R65** | Laws that span bodies | H¹ with values needs a coding-level claim that two values in different bodies are the same quantity, and a factor stated in DNA. Where does that claim live: a `laws` section in `.universe`, a typed quantity frame, or something else? |
| **R66** | Loops with many frame changes | Rule 3 judges loops with at most two frame changes. What pairs the conversions on a longer loop, and is the answer independent of the cycle basis? |
| **R67** | Hyperedges in the assay | A link with k + 1 members is a k-block in the theory. Phase 4 prints such a link `not measured`. What are its faces? |
| **R68** | Promises and allele bodies | The round-trip promise for an allele lives in the cell that carries it. Should an allele body be assayed in its owner's context? |
| **R60** | Every file kind ships its mutants | **Decided 26 Sep:** Option A (§2.10) |

---

## Appendix A · `AGENTS.md` changes

Replace the header paragraph with:

```markdown
# JoInn — standing rules

This repo is JoInn. Phase 4 is the assay: a measurement of loops over bodies and
universes. Its one idea is MEASURE FIRST, KEEP IT ONLY IF IT SEES SOMETHING
NOTHING ELSE SEES. The build plan is docs/Plans/JoInn Phase 4 Implementation
Plan.md. Work one CHUNK at a time (A, B or C), one git commit per numbered step,
and stop at the chunk's stop report. Every decision is in the plan. Never stop to
ask; follow the plan's Snags section instead.

There is no renderer, no holonomy, no universe-level law and no compiler in this
phase. Phase 6 runs after it.
```

Keep rules 1–45, 47 and 48 exactly as they are. Append:

```markdown
49. AN ASSAY NEVER REFUSES ON ITS OWN. It reports. Only a declaration
    (`assert H₁ = 0` in a body or universe) refuses, at admission. The gate
    keeps four checks and names no assay type.
50. AN ASSAY IS BLIND TO THE REGULATORY REGION, TO ALLELES AND TO LENSES. The
    invariance harness checks it on every subject. An assay that moves under any
    of them is inadmissible.
51. TWO DERIVATIONS, ONE TRUTH. The region derivation and the port derivation
    agree on every corpus subject, and Euler's identity holds in both. A
    disagreement is a truth violation, never a test failure.
52. NEW GRAMMAR SHIPS ITS MUTANTS (R60). No control points at an artifact that
    uses new grammar until the catalogue has that grammar's mutations, each
    tested by what the mutant does.
53. A FILLING NAMES WHAT MADE IT: a frame, or one law by alias, instance and
    name. Nothing else closes a loop.
```

## Appendix B · Text for P4-10

**If KEEP** (copy, then quote P4-08's output):

> **Held.** C1: the assay distinguished `loop.universe` from its `fmt_twin` mutant, and no other check did, including a run. Every body in the mutant is lawful on its own. What the assay found is a round trip across two bodies whose promise names a different partner. `cli_input` promises to undo `format`, and the ring converts with `format_twin`, a different cell by content. The two formatters behave the same today, so this is a missing promise, not a wrong number. Under Law 1, an operation whose opposition isn't declared is untrue by structure, and the assay is the only instrument in JoInn that sees it across a membrane without running anything.

**If CUT** (copy, then quote P4-08's output):

> **Fired.** C1: the check(s) named on the `distinguishing:` line told `loop.universe` from its `fmt_twin` mutant without the assay. The assay found nothing that JoInn could not already see. Per D7 the layer is cut; ∂ and the ∂∂ assembly stay, because the touch-only law (V16) is checked by them.

**C2** (always; then quote P4-09's C2 outputs):

> **Not expressible yet.** A value-consistency check (H¹ with values) needs a factor stated in DNA and a coding-level claim that two values in different bodies are the same quantity. Today the factor enters from outside (`factor: 12`), and no universe carries a law. A `require` could state the claim only at one membrane that sees both values, which makes it local. This opens R65.

**C3** (always; then quote P4-09's C3 output):

> **Withdrawn by the invariance rule.** Part III §9.5 read H₂ as laws closed around an empty inside: a specified but unimplemented cell. "Unimplemented" means "has no allele", and §9.8 requires an assay not to move when alleles change. The harness stripped every allele from the calculator's cells and the report did not move. An admissible assay cannot see implementation, so that reading of H₂ is withdrawn. H₂ stays as a count of loops filled twice.

---

*JoInn Phase 4 Implementation Plan, Draft 0.1 with Amendment A (26 Sep 2026). Opens after Phase 5.2's stop-E review. AJ decided: R60 Option A; kill test first; static detection counts; H¹ tested as written, not extended. Every other decision is made here. Cursor executes. Claude verifies at each stop.*
