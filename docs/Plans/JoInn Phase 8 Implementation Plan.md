# JoInn Phase 8 Implementation Plan

**The strand and the tower: four letters, one reader, and a fold that is free**

Author: AJ, with Claude · Draft 1.0 · October 12, 2026 · replaces Draft 0.1 (visual truth), which is held (§9)

> **Every decision in this plan is final.** Nothing here waits on AJ. Cursor never stops to ask AJ anything, and no file in this phase is typed by AJ. If a step can't be done the way the plan says, Cursor follows §0.3 (Snags) and keeps going. The phase runs as **one unit with one stop** (AJ, 5 Oct).

---

## For AJ: this plan in plain English

**What Phase 8 is.** The first phase of the DNA base from Part VIII, rebuilt around what we worked out on 12 Oct. It makes three things real and lets you see them:

1. **The letters.** A, T, G, C as two bits each, with pairing built into the bits.
2. **The reader.** It reads letters three at a time through the one code table (codex 1) and names the amino acids. It is versioned on its own; the table is frozen.
3. **The tower.** A strand's codons sit on the segments of a Hilbert curve. The curve is the fold: it puts every codon at an exact place, for free, and every order of the curve holds the one before it. The nodes between segments can point to other genes: the gene pool.

**What you'll see at the end** (about five minutes, §6.2): the strand as a turning 3D helix with its partner strand, codons grouped and named, a mismatched letter glowing; then the same strand folded flat on its Hilbert curve, where zooming out shows the order below it, and a node opens into the gene it points to.

**What it does not do yet.** It doesn't run proteins, build cells, form membranes, or rebuild the calculator. Those are the next phases (§8). Phase 8 makes the material and the fold true first.

**The bit layout (today's decision, now in the plan):**

| Letter | Bits | 1st bit: strength | 2nd bit: shape |
|---|---|---|---|
| A | 00 | weak | big (purine) |
| T | 01 | weak | small (pyrimidine) |
| G | 10 | strong | big |
| C | 11 | strong | small |

A letter's partner is the same letter with its **second bit flipped**. So A↔T and G↔C, exactly as Part VIII says, and the big/small question from 8 Oct is answered without extra bits.

**Two facts I checked by program before writing them here:**

- **A codon and its partner always have opposite parity**, for all 64 codons, and no codon is its own partner. So the reader can tell which strand it's holding just by counting ones. That's why the opposite protein can't be read by mistake.
- **The tower properties hold through order 6:** every order's strand, filtered to its upper floors, is exactly the order below; every curve is its own mirror; every order nests inside the next.

**One correction to what I told you this morning.** I said the middle codon of a strand is "the single highest-floor bridge." That's wrong. Every order has **three** top-floor bridges (at ¼, ½ and ¾ of the strand), and the middle codon is the centre one of the three. It's still a natural centre, just not unique. The plan uses the corrected fact.

**What I decided (say if you disagree):**

- **2D only in this phase.** Even dimensions (4D, 6D) come later; the tower code is written so dimension is one number, but only 2 is built and tested.
- **A chromosome uses the smallest order that holds its codons.** Slots past the last codon are **silent**: not stored, not null, the reader simply finds nothing there. This follows D25 (nothing is never stored); this plan treats D25 as accepted.
- **Start and stop stay as codons too.** The curve's first and last nodes are the chromosome's ends; START and STOP codons still mark where each protein begins and ends inside it. Both, not one or the other.
- **Nodes hold at most one gene reference each**, by hash. A gene can't contain itself, directly or through others; that's refused when it enters the pool. Nodes open only when asked (zoom or request), never all at once.
- **The reader holds exactly one codex (1).** Reading a strand that names another codex is refused. Holding several codices side by side (the mitochondria case) is a later decision.
- **Organelles are listed, not built.** The runtime declares what it inherits (screen, input, storage, clock); a strand may only touch an organelle the runtime lists. No new organelle is written in this phase.
- **Membranes and roles from the fold** (closed folds, faces-out from geometry) are written down as research, not built. They need the reader running proteins first.

---

## 0. How Cursor Works This Plan

| | |
|---|---|
| **Plan file** | `D:\JoInn\docs\Plans\JoInn Phase 8 Implementation Plan.md` |
| **Theory it builds** | `docs/Theory/` Part VIII *JoInn DNA, Codons and Amino Acids* (D1–D30), with the 12 Oct additions in §2 here |
| **Code** | `D:\JoInn\joinn\`. **Three new crates** beside today's code: `joinn-strand`, `joinn-codex`, `joinn-tower`. The views go in `joinn-visual` and `joinn-shell-desktop`. Nothing existing is deleted |
| **New external dependencies** | none |
| **Standing rules** | `AGENTS.md` as updated by P8-01 (Appendix A) |
| **Unit of work** | the whole phase; commits `P8-01` … `P8-12`; **one stop**, at the end |
| **Who decides** | every decision is in §2. Cursor decides module layout, private representations, function bodies, colours inside the style table, and error wording where §2 gives none |
| **Who checks** | Claude, from a fresh clone, after the stop; CI on every push; AJ at the end (§6.2) |
| **Speed** | run only the affected crates' tests per commit (`cargo test -p <crate>`); `gate all` and the full workspace build run **once**, at P8-11. Use the existing `stop-check` |

### 0.1 The commit report

As Phase 7.4, plus from P8-04 on: `Tower:      cargo xtask tower → <last line>`.

### 0.2 The stop report

At `docs/Findings/phase-8-stop.md`: every command in §3 with its last line, `gate all`, `corpus verify`, CI status, and any snags.

### 0.3 Snags

As Phase 7.4: write the snag in the stop report with the exact output, take the plan's stated fallback if there is one, and keep going. Never widen a check to make it pass.

---

## 1. Scope Fence

### In scope

- **The `gates.lock` fix** from the Phase 7.4 review (compare before writing). Opens the phase.
- **Letters and strands** (`joinn-strand`): the bit layout, partners, partner strands, packing, hashing.
- **The codex and the reader** (`joinn-codex`): codex 1 exactly as Part VIII §6.1, reading frames, translation into amino-acid chains, refusals, the runtime/codex version split.
- **The tower** (`joinn-tower`): the 2D Hilbert fold, segment levels, the level filter, mirror and nesting checks, chromosomes, silent slots, the gene pool and node expansion.
- **The organelle list**: a declared, read-only list in the runtime.
- **Two views**: the 3D strand view (D30, first) and the flat fold view.
- **Witnesses** for all of the above, written once under `corpus/phase8/`.
- **Gate 8**, findings, freeze.

### Out of scope

| Not now | Why it is tempting |
|---|---|
| Running proteins (executing amino acids) | It's the next step; Phase 8 only reads and names them |
| Membranes, roles, cells from proteins | Need running proteins first (§8) |
| Rebuilding the calculator from a strand | The milestone after that (§8) |
| Dimensions other than 2 | Written as a parameter, built and tested at 2 only |
| Moore curve or other fold variants | Research (R130) |
| A second codex | Refused by the reader (§2.5) |
| Switches (D27) | Their own phase |
| Changing any existing grammar, crate or corpus file | New crates and new corpus files only |
| The visual-truth work (old Draft 0.1) | Held (§9) |

---

## 2. Decisions

All **DECIDED**.

### 2.1 Letters

| Letter | Bits |
|---|---|
| A | `00` |
| T | `01` |
| G | `10` |
| C | `11` |

- **Partner** = flip the low bit. `partner(partner(x)) = x`; no letter is its own partner.
- **Strength** = the high bit (A, T weak; G, C strong). Pairing never changes strength.
- **Shape** = the low bit (A, G big; T, C small). Pairing always swaps it.
- A letter alone means nothing (D5). No API gives a letter a name other than A, T, G, C.

### 2.2 Strands

- A strand is a row of letters with a head and a tail, read head to tail (D6).
- **Partner strand** = every letter's partner, in reverse order (Part VIII §3.3).
- **Packing**: four letters to a byte, first letter in the two high bits; a strand's length in letters is stored beside it so a last partial byte is exact.
- **Identity** = the existing JoInn hash over the packed bytes plus the length (D8). A strand and its partner have different hashes.
- **Text form** (for corpus files): the letters on one line, grouped in threes by single spaces from the first letter, `\n` at the end. Line 1 of every strand file is `codex 1`.

### 2.3 Codons and parity

- A codon is three letters, 6 bits, 64 in all (D9). All three letters carry meaning (D13).
- **Parity** of a codon = the number of 1 bits in its 6 bits, mod 2.
- **The partner codon** (the three letters' partners, reversed) **always has the opposite parity.** No codon is its own partner. (Checked for all 64 by program, 12 Oct.)
- The reader reports a strand's **side**. `AAT` (START) is `00 00 01`: one 1 bit, **odd**. `ATT` (STOP) is `00 01 01`: two, **even**. So a strand whose START reads odd is `sense`; one whose START reads even is `partner`. P8-03 prints both parities from code as a check on this text.

### 2.4 Codex 1

Exactly Part VIII §6.1, unchanged:

| Codon | Means | Partner | Means |
|---|---|---|---|
| `AAT` | START | `ATT` | STOP |
| `AAA` | pair | `TTT` | split |
| `CAC` | build | `GTG` | case |
| `AAC` | refer | `GTT` | return |
| `CAA` | sense | `TTG` | show |
| `GAA` | keep | `TTC` | recall |
| `AAG` | grant | `CTT` | revoke |

All other 50 codons are **reserved**: reading one is a refusal, never a guess (D18 treated as accepted). The table lives in `joinn-codex` and nowhere else (D11). A test asserts that for every row, the partner of the codon names the opposite amino acid (D19), and that every reserved codon's partner is reserved.

### 2.5 The reader and its versions

- `translate(strand) → Verdict<Vec<Protein>>`: find each START in frame order, read in threes until STOP; a protein is the list of amino acids between them (D10, D20).
- Refusals (rule 35 wording: what was found, where, what acceptance looks like):
  - reserved codon: `reader: codon TAA at letter 12 names nothing in codex 1; acceptance is a codon codex 1 names`
  - START without STOP: `reader: start at letter 0 has no stop; acceptance is a stop in frame`
  - wrong codex: `reader: strand names codex 2; this reader holds codex 1`
- **Two version tracks.** The strand file's `codex 1` line pins the table. The reader's own version (`reader 8.0`) is separate and printed by `cargo xtask codex`. Changing the reader never changes what a codex-1 strand means; a test reads every Phase 8 strand witness and compares to its witnessed protein list byte for byte.
- **The partner reading.** `translate(partner(s))` for a strand of one protein gives the opposite protein: the opposites in reverse order (D19). Tested on every protein witness.

### 2.6 The tower (2D)

**Backbone.** At order *n* the curve has 4ⁿ nodes and 4ⁿ − 1 segments. Node *d* (0 … 4ⁿ − 1) is at integer point `point(n, d)`, the standard Hilbert map. All integer, no floats.

**Facts the code must witness** (all checked by program for orders 1–6, 12 Oct):

| # | Fact |
|---|---|
| T1 | node 0 is at (0, 0); the last node is at (2ⁿ − 1, 0) |
| T2 | consecutive nodes are one unit step apart |
| T3 | **nesting**: `point(n, d) >> 1 == point(n − 1, d >> 2)` for every *d* |
| T4 | **level** of segment *i* = the number of trailing 3s in *i* written in base 4 |
| T5 | **level filter**: the directions of order *n*'s segments with level ≥ 1, in order, are exactly order *n − 1*'s directions |
| T6 | **mirror**: reverse the direction list, reverse each direction, swap E↔W: the list is unchanged |
| T7 | **top floor**: exactly three segments have level *n − 1*, at indices 4ⁿ⁻¹·k − 1 for k = 1, 2, 3; the middle segment is the centre one |

**Direction** of a segment is one of N, E, S, W, derived, never stored.

**Chromosome.** A chromosome is a strand of codons placed on a tower:

- codon *k* (counting from the strand's head, in frame from letter 0) sits on segment *k*;
- its **order** is the smallest *n* with 4ⁿ − 1 ≥ the codon count (order 0 holds nothing: the empty chromosome);
- segments past the last codon are **silent**: not stored and not null; the reader returns nothing for them;
- a chromosome's identity is its strand's hash; the order is derived, never stored.

**Zoom out = level filter.** The chromosome at floor *f* is the codons on segments with level ≥ *f*. `cargo xtask tower` prints each floor of every witness chromosome.

**Dimension** is a parameter in the API (`Dim(2)`); any other value is refused: `tower: dimension 4 is not built; acceptance is dimension 2`.

### 2.7 The gene pool

- Each **node** of a chromosome may hold **one gene reference**: the hash of another chromosome in the pool. A node with no reference holds nothing (silent).
- References are written in a chromosome's file as lines after the strand: `node <d> gene <hash>`, sorted by *d*.
- The **pool** is a content-addressed store of chromosomes, using the existing hash store.
- **No self-containment.** A gene that reaches itself through references is refused when it enters the pool: `pool: gene <h> contains itself through <h1> → <h2> → <h>; acceptance is a pool without cycles`.
- **Grow on demand.** `expand(chromosome, node)` returns the referenced chromosome; nothing is read from the pool until asked. A test counts pool reads and asserts zero for an unexpanded chromosome.
- A reference to a hash not in the pool is refused at expansion: `pool: node 5 names gene <h>, which the pool does not hold`.

### 2.8 Organelles

`joinn-codex` (the runtime side) holds a fixed list:

| Organelle | Used by amino acids |
|---|---|
| `screen` | show |
| `input` | sense |
| `store` | keep, recall |
| `clock` | sense |

The list is printed by `cargo xtask codex`. Phase 8 only declares it; a test asserts every world amino acid (sense, show, keep, recall) names at least one listed organelle, and every organelle is named by at least one amino acid. Grant/revoke act on membranes, not organelles.

### 2.9 Witnesses

Under `corpus/phase8/`, written once (rule 79 style: an existing file is refused, never rewritten):

| File | Holds |
|---|---|
| `strands/empty.strand` | `AAT ATT` (the empty protein: nothing, D25) |
| `strands/pair-split.strand` | `AAT AAA TTT ATT` |
| `strands/twelve.strand` | START, all twelve amino acids in table order, STOP |
| `strands/reserved.strand` | START, `TAA`, STOP (must be refused) |
| `strands/*.proteins.txt` | the reader's output for each, or its refusal line |
| `strands/*.partner.txt` | the partner strand and its proteins |
| `tower/order-1.txt` … `order-4.txt` | each order's nodes, segment directions and levels |
| `tower/twelve.chromosome` | `twelve.strand` placed on its tower, with two gene references |
| `tower/twelve.floors.txt` | the chromosome at every floor |
| `pool/cycle.chromosome` | two chromosomes referencing each other (must be refused) |

Each file goes in `corpus/hashes.txt` under a dated comment with the tag `joinn.dna.v1` and in `corpus verify`. Cursor prints the new `corpus verify` count.

### 2.10 The views

**Strand view (3D, first; D30).** A double helix turning slowly on its own axis: one rung per letter pair, the four letters in four style colours, rungs grouped in threes with a small gap, each codon tinted by its amino acid (reserved codons grey), START and STOP as end caps, the amino acid's name in the stroke font beside its codon. A **mismatched pair** (a letter whose partner is wrong) **glows**. The camera moves along a fixed path only (Part VIII §11.1, gentle 3D): arrow keys move along the strand, `+`/`-` move in and out. No free orbit.

**Fold view (2D).** The chromosome drawn on its Hilbert curve: each segment coloured by its codon's amino acid, silent segments as thin grey, nodes with a gene reference drawn as a small ring. Zoom out shows floor by floor (the level filter, §2.6); Enter on a ringed node opens its gene in place (§2.7). Exact integers throughout; the existing zoom and pick machinery is reused.

Both views draw from the reader's and the tower's output; neither computes meaning itself (rule 72 in spirit: one truth, two pictures). Picking a codon in either view returns the same address: `<strand hash>.codon<k>`.

`joinn-shell-desktop` opens them with `--strand <file>` and `--fold <file>`; Tab switches between the two views of the same file.

### 2.11 Invariants

| # | Invariant | Commit |
|---|---|---|
| **V160** | Partner is the low bit: `partner` is an involution with no fixed letter, and never changes strength | P8-02 |
| **V161** | A codon and its partner have opposite parity; no codon is its own partner | P8-03 |
| **V162** | The table lives in the reader only; every codon's partner names its opposite; reserved stays reserved | P8-03 |
| **V163** | The reader's version never changes a codex-1 strand's meaning | P8-05 |
| **V164** | The fold is free and exact: T1–T7 hold for orders 1–6 | P8-06 |
| **V165** | Every order holds the one before it (T3, T5) | P8-06 |
| **V166** | Nothing is stored for nothing: silent slots and empty nodes cost no bytes | P8-07 |
| **V167** | The pool has no cycles, and grows only when asked | P8-08 |
| **V168** | Two views, one truth: strand and fold views pick the same codon address | P8-10 |

---

## 3. What Runs

| Command | Prints | Exit 1 when |
|---|---|---|
| `cargo xtask strand <file>` | the strand, its partner, side (`sense`/`partner`), hash; last line `strand: <n> letters, <c> codons, side <s>` | a letter that isn't A, T, G, C |
| `cargo xtask codex` | the table, the organelle list, `reader 8.0`, `codex 1`; last line `codex 1: 14 named, 50 reserved; every partner names the opposite` | any table check fails |
| `cargo xtask read <file>` | the proteins, then the partner's proteins; last line `read: <p> proteins; partner reads the opposites` | refusal |
| `cargo xtask tower` | T1–T7 per order 1–6, then every witness chromosome by floor; last line `tower: orders 1-6 hold T1-T7; every order holds the one before it` | any fact fails |
| `cargo xtask pool` | the pool, each expansion, the cycle refusal; last line `pool: no cycles; grows only when asked` | otherwise |
| CI | new step `dna` after `zoom` on both jobs: `strand`, `codex`, `read`, `tower`, `pool` | the step fails |

---

## 4. The Commits

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P8-01** | **Plan, rules, docs, `gates.lock` fix** | First `git mv` the existing Draft 0.1 plan to `docs/Plans/JoInn Visual Truth Plan (held).md` unchanged (§9), then commit this plan at the Phase 8 path; `AGENTS.md` per Appendix A; Appendix B; `gates.lock` compares before writing (7.4 review) | `git show --stat HEAD` lists the plan, `AGENTS.md`, backlog, `decisions.md`, roadmap, the lock fix. A test: writing an unchanged lock leaves the file's bytes and timestamp untouched |
| **P8-02** | **Letters and strands** | `joinn-strand`; §2.1, §2.2; V160 | Tests: all four partners; involution; strength unchanged; pack/unpack round trip for lengths 0–64; partner strand of the partner is the strand |
| **P8-03** | **Codons and codex 1** | `joinn-codex`; §2.3, §2.4; V161, V162 | `cargo xtask codex` printed whole (paste it). Tests: parity flips for all 64; table pairs; reserved partners reserved; the parity of `AAT` and `ATT` printed |
| **P8-04** | **The reader** | §2.5 translate and refusals | `cargo xtask read` on every strand witness (paste the last lines); each refusal pasted |
| **P8-05** | **Strand witnesses and versions** | §2.9 strands; V163 | Witness files committed; `corpus verify` count printed. Test: bump the reader's version string; every witness still reads byte-identically |
| **P8-06** | **The fold** | `joinn-tower`; §2.6 backbone, T1–T7; V164, V165 | `cargo xtask tower` printed whole. **Shown then reverted**: swap two node positions at order 3; T2 and T5 fail; paste both refusals |
| **P8-07** | **Chromosomes and floors** | §2.6 chromosome, silent slots, floors; V166 | Test: a 20-codon strand gets order 3 and stores 20 codons, not 63; floors of `twelve.chromosome` printed; the empty strand is order 0 |
| **P8-08** | **The gene pool** | §2.7; V167 | `cargo xtask pool` printed whole; the cycle refusal pasted; the zero-reads test passes |
| **P8-09** | **The 3D strand view** | §2.10 strand view | Cursor runs `--strand corpus/phase8/strands/twelve.strand` once and pastes the first tick line, or writes `no display`. A headless test: the drawn rows hold 3 × codons rungs per strand side, one tint per codon, one glow when one letter of the partner is flipped |
| **P8-10** | **The fold view** | §2.10 fold view; V168 | Headless tests: segment count drawn = codons + silent; floor zoom draws exactly the level-filtered segments; picking codon 7 in both views returns the same address |
| **P8-11** | **Gate 8 and findings** | §6; `docs/Findings/phase-8-strand-and-tower.md` with every command's output, the predictions of §2.6 marked, and the 12 Oct correction (the middle codon is one of three top-floor bridges) | `gate all` exits 0 and prints `phase 8: 3/3`. **Shown then reverted**: (a) give `AAA` the meaning `split` (item 1 fails); (b) drop T5's level filter to level ≥ 0 (item 2 fails); (c) let a gene reference itself (item 3 fails) |
| **P8-12** | **Docs and freeze** | README; `03-where-we-are.md` (the strand is read, the fold is free); glossary: *letter bits, strength bit, shape bit, partner strand, side, codex, reader version, tower, order, floor, level filter, chromosome, silent slot, gene pool, organelle*; `decisions.md` V160–V168; roadmap note | `gate all` from a fresh clone prints phases 0 … 8, exits 0; `git diff --stat <P8-01>..HEAD -- joinn/corpus` lists only `corpus/phase8/` and `hashes.txt` |
| — | **Stop** | `phase-8-stop.md`, push, ledger | — |

**If something has to be cut for time:** the fold view's node expansion (the pool stays, tested headless), then the strand view's glow. Never cut V161, V162, V164, V165, V167.

---

## 5. Test Strategy

As Phase 7.4. Every witness test compares bytes. No test asserts only that something exists. Tower facts are checked for every *d* at orders 1–6, not sampled.

---

## 6. Exit Gate 8

Three items in `xtask/src/fns/gate_eight_items.rs`. Witness files are the controls.

| # | Item | `control_artifact` | `opposes` | Control answers `true` when |
|---|---|---|---|---|
| 1 | **The reader holds the one table** | `corpus/phase8/strands/twelve.proteins.txt` | the codex with `AAA` and `TTT` swapped (printed) | the reader's output differs from the artifact |
| 2 | **Every order holds the one before it** | `corpus/phase8/tower/twelve.floors.txt` | T5 filtering at level ≥ 0 (printed) | the derived floors differ from the artifact |
| 3 | **The pool has no cycles** | `corpus/phase8/pool/cycle.chromosome` | admission without the cycle check | the cycle is admitted |

### 6.1 Conditions for the next phase

All of §4 done, gate 8 passing, every earlier gate passing (V28), CI green.

### 6.2 AJ's check (end of phase, about five minutes)

```
cd D:\JoInn\joinn
cargo run --release -p joinn-shell-desktop -- --strand corpus/phase8/strands/twelve.strand
```

1. Watch the helix turn. You should see two strands, rungs in groups of three, and the amino acid names beside them.
2. Arrow keys move along it. Find START at one end and STOP at the other.
3. Press **Tab**. The same strand lies flat on its Hilbert curve, coloured by amino acid.
4. Press **-** a few times. The curve simplifies floor by floor; each floor is the order below.
5. Find a small ring on a node and press **Enter**. The gene it points to opens there.
6. Close the window. Tell Claude what you saw and what felt wrong.

---

## 8. What Comes After (for the roadmap, not this phase)

| Next | What it proves |
|---|---|
| **8.1 Proteins run** | The reader executes chains (D21) using the Floor's Rust for pair/split/build/case; the partner protein undoes |
| **8.2 Fold to membrane** | Closed folds become boundaries; faces-out/in from geometry, holds/reacts from the amino acid (R128, R129) |
| **8.3 The cell from a strand** | A protein chain inside a derived membrane is a cell, drawn by today's host |
| **8.4 The calculator from a strand** | The counting calculator regrown from DNA, with Phase 7.4's hand-built version as its witness |

---

## 9. What Happens to Draft 0.1 (Visual Truth)

The 3 Oct plan (owners, reader tree, intents witnessed; keyboard; AccessKit) is **held unchanged** as `docs/Plans/JoInn Visual Truth Plan (held).md`. Its work is still wanted, but its subjects (`calculator.body`, `calculator.contact`) are the bodies Part VIII will rebuild from strands, so it waits until 8.4 and is then replanned against the strand-built calculator. Its open items R108–R110 stay open.

---

## 10. Risks

| Risk | What to do |
|---|---|
| The Hilbert map has an off-by-one in rotation that only shows above order 3 | T1–T7 are checked for every node at orders 1–6, and the order-1 … 4 witnesses pin the exact output |
| 3D in the existing host is more work than expected | Snag: ship the strand view as a flat ladder (same rows, no turn) and note it; the glow and tints still ship |
| Packing order disagrees between platforms | Rule 48: one byte order, defined in §2.2; a witness differing between Windows and Linux is a truth problem |
| The silent-slot rule tempts a "blank codon" | There is none. A test asserts no API returns a codon for a silent slot |
| The gene pool's cycle check is slow on deep pools | Phase 8 pools are tiny; note the depth in findings for later |

---

## 11. Open Items

| ID | Topic | Question |
|---|---|---|
| **R128** | Membrane from the fold | Is a membrane a closed fold (a Moore curve)? How does a closed loop sit inside an open tower? |
| **R129** | Roles from the fold | Does faces-out/in come from geometry (edge vs inside of the occupied region) while holds/reacts comes from the amino acid? Then "faces out" is never a base kind |
| **R130** | Fold variants | May a gene choose its sub-curve's orientation (mirror, rotation, Moore), so sequence shapes the fold? |
| **R131** | Higher dimensions | 4D and 6D towers: in 6D one floor is exactly one codon. Which dimension suits app structure: deep (2D) or wide (6D)? |
| **R132** | Several codices | Should the reader ever hold two codices side by side, as cells do with mitochondria? |
| **R133** | Tower security and transfer | Merkle hashing up the floors; a key for floor *n* derived from floor *n − 1*; sending only the order plus codons |

---

## Appendix A · `AGENTS.md` changes

Replace the header paragraph with:

```markdown
# JoInn — standing rules

This repo is JoInn. Phase 8 builds the DNA base: four letters (A, T, G, C as
two bits each), the one code table read by a versioned reader, and the tower
(a 2D Hilbert curve) that folds a strand for free. Its one idea is THE FOLD IS
FREE AND EXACT. The build plan is docs/Plans/JoInn Phase 8 Implementation
Plan.md. The phase is one unit with one stop. Never stop to ask; follow the
plan's Snags section instead.

New crates only: joinn-strand, joinn-codex, joinn-tower. No existing crate,
grammar or corpus file changes except gates.lock (P8-01).
```

Keep every rule. Append:

```markdown
82. A LETTER IS TWO BITS. A=00, T=01, G=10, C=11. Partner is the low bit.
    No letter has another name or meaning.
83. THE TABLE LIVES IN THE READER. Meaning is assigned only in joinn-codex.
    A reserved codon is a refusal, never a guess. The reader's version never
    changes what a codex-1 strand means.
84. THE FOLD IS FREE AND EXACT. Positions, directions and levels are derived
    from the segment index with integers. Nothing derived is stored.
85. NOTHING IS STORED FOR NOTHING. Silent slots and empty nodes have no bytes
    and no null.
86. GENES GROW WHEN ASKED. The pool is read only on expansion, and a gene that
    contains itself is refused at admission.
```

## Appendix B · Documents

- **Backlog**: after the last R-item, `> **12 Oct 2026 · Phase 8 (Draft 1.0).** R128–R133 come from the strand-and-tower plan.` then R128–R133 (`open`) with §11's questions.
- **`decisions.md`**: V160–V168 (`open`; P8-12 sets them); R128–R133 (phase `8`, `open`); and the lines: `Letters: A=00 T=01 G=10 C=11; partner is the low bit (12 Oct 2026).` `Phase 8 Draft 0.1 (visual truth) held until 8.4 (12 Oct 2026).` `D18 and D25 treated as accepted by Phase 8.`
- **Roadmap**, under Phase 8: `> **12 Oct 2026.** Replanned as the first Part VIII phase: letters, codex 1 and its reader, the Hilbert tower and gene pool, the 3D strand view and the fold view. Visual truth is held until the calculator is regrown from a strand (8.4).`

---

*JoInn Phase 8 Implementation Plan, Draft 1.0 (12 Oct 2026), from AJ's conversation with Claude that day. AJ decided: keep nature's four letters with pairing (00↔01, 11↔10); codons can't be derived from other codons; amino acids as many as the irreducibility rule allows; the partner strand reads the opposite protein; all 64 codons meaningful; the Hilbert tower determines the fold, with codons on segments and nodes as a gene pool; the reader is the versioned runtime; inherited machinery lives in the runtime; Phase 8 becomes the first DNA phase. Claude decided, open to AJ's veto: 2D only; smallest order that holds the codons, with silent slots; START/STOP kept as codons; one gene per node; one codex held; organelles listed not built; membranes and roles deferred to research. Claude checked by program: parity flips for all 64 codons; T1–T7 for orders 1–6; and corrected the earlier claim that the middle codon is the unique top-floor bridge.*
