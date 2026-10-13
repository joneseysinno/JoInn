# Phase 8 — Strands, codex 1 and the tower: findings

P8-11. Strands are now real. They use four letters of two bits each, with the
partner given by the low bit and the partner strand being the partners
reversed. Identity is the `joinn.dna.v1` hash. Codex 1 names 14 codons in
seven partner pairs and reserves the other 50. The reader translates a strand
in frame from letter 0, and the partner strand reads the opposite proteins in
reverse order. The 2D Hilbert tower places codon k on segment k. Each floor is
the level filter of the floor below, and T1–T7 hold at every node and segment
of orders 1–6. The gene pool holds chromosomes by hash, reads nothing until
asked, and refuses a cycle on entry. The strand view and the fold view open
from `joinn-desktop`. Gate 8 passes 3/3, and `gate all` passes every phase.
Every prediction of §2.6 held. One claim from before 12 Oct, that the middle
codon is the only top-floor bridge, is corrected below.

Commits:
- P8-01 `7b965bb`
- P8-02 `4803b2c`
- P8-03 `50ac65a`
- P8-04 `17e3643`
- P8-05 `f25e025`
- P8-06 `eb1b81c`
- P8-07 `a6f9c60`
- P8-08 `7472072`
- P8-09 `a2ec2cc`
- P8-10 `64c3511`
- P8-11 (this commit)

Machine: Windows 10.0.26300, with adapters Microsoft Basic Render Driver (Dx12,
Cpu) and NVIDIA GeForce RTX 2080 (Dx12 and Vulkan). Every output below was run
from `joinn/` at P8-11, with cargo's Compiling, Finished and Running lines
dropped. None of the commands' code changed after the commit that introduced
it.

## What was built, by commit

- **P8-01, the plan.** Draft 1.0 at the Phase 8 path, with Draft 0.1 held as
  the Visual Truth Plan. Also: `AGENTS.md` per Appendix A; backlog R128–R133;
  `gate all` now compares the committed lock before writing.
- **P8-02, letters and strands (`joinn-strand`).**
  - A = 00, T = 01, G = 10, C = 11. The partner flips the low bit.
  - Four letters pack to a byte. Identity is the hash over the length and the
    packed bytes.
  - Strand files are `codex <n>` followed by the letters.
- **P8-03, codons and codex 1 (`joinn-codex`).**
  - Codons are six bits, with parity and partner codons. The table check
    covers the codon algebra.
  - The side comes from the head codon. The organelle list is printed.
  - New commands: `cargo xtask codex` and `cargo xtask strand <file>`.
- **P8-04, the reader.** START opens a protein and the next STOP closes it.
  Every other case is a refusal that names what, where, and what is accepted.
  `cargo xtask read` checks that the partner reads the opposites in reverse
  order.
- **P8-05, strand witnesses.** `empty`, `pair-split`, `twelve` and `reserved`,
  each with its `.proteins.txt` and `.partner.txt`. They are written once by
  `cargo xtask dna-witness` and hashed under `joinn.dna.v1`.
- **P8-06, the fold (`joinn-tower`).** The integer Hilbert map, derived
  directions, segment levels, the level filter, and T1–T7 checked at every
  node and segment.
- **P8-07, chromosomes and floors.** Codon k sits on segment k, and the order
  is derived. Silent segments are neither stored nor null. Gene references are
  kept per node, and floors come from the level filter.
- **P8-08, the gene pool.** `insert` refuses cycles. `expand` is the only read,
  and every read is counted.
- **P8-09, the strand view.** `StrandScene` draws a still double helix. Pick
  addresses are `<strand hash>.codon<k>`.
- **P8-10, the fold view.** `FoldScene`. Tab switches between the two views,
  `+`/`-` change the floor, and Enter opens a gene through the pool.
- **P8-11, gate 8 (this document).**

## The §2.6 predictions, marked

Every fact was checked by `cargo xtask tower` at orders 1–6, for every node and
segment, with nothing sampled. Gate 8 item 2 reruns the same facts.

| # | Prediction | Held? | Where it shows |
|---|---|---|---|
| T1 | node 0 at (0, 0); the last node at (2ⁿ − 1, 0) | **held** | e.g. order 6: `node 4095 at (63, 0)` |
| T2 | consecutive nodes one unit step apart | **held** | `4095 steps, each one unit` |
| T3 | `point(n, d) >> 1 == point(n − 1, d >> 2)` | **held** | `4096 nodes nest in order 5` |
| T4 | level of segment i = trailing 3s of i in base 4 | **held** | order 6: `levels 0 ×3072, 1 ×768, 2 ×192, 3 ×48, 4 ×12, 5 ×3` |
| T5 | the level-≥ 1 directions of order n are order n − 1's | **held** | `1023 segments of level ≥ 1 give order 5's 1023 directions` |
| T6 | reverse, reverse each, swap E↔W: unchanged | **held** | `mirror holds over 4095 directions` |
| T7 | exactly three segments at level n − 1, at 4ⁿ⁻¹·k − 1; the middle is the centre | **held** | order 3: `segments 15, 31, 47; the middle, 31, is the centre` |
| Order | smallest n with 4ⁿ − 1 ≥ codons | **held** | twelve: 14 codons, order 2 (15 segments); pair-split: 4 codons, order 2; empty: 2 codons, order 1 |
| Silent | segments past the last codon not stored, not null | **held** | twelve: `14 codons on 15 segments, 1 silent` |
| Identity | a chromosome's identity is its strand's hash | **held** | `twelve.chromosome` is `edf1529a6fc2…`, the same as `twelve.strand` |
| Zoom | floor f = codons on segments of level ≥ f | **held** | twelve floor 1: `3 CAC build, 7 CAA sense, 11 AAG grant` |
| Dimension | any `Dim` but 2 refused | **held** | `tower: dimension 4 is not built; acceptance is dimension 2` |

## The 12 Oct correction

Before 12 Oct, the claim was that the middle codon is the **unique** bridge on
the top floor. It is not. T7 shows that the top floor of order n
(level n − 1) has **exactly three** segments, at 4ⁿ⁻¹ − 1, 2·4ⁿ⁻¹ − 1 and
3·4ⁿ⁻¹ − 1. These are the three bridges between the four quarter-curves. The
middle one, 2·4ⁿ⁻¹ − 1, is the centre of the curve, but it is one of three
bridges, not the only one.

`twelve` shows this directly. Its floor 1 (order 2's top floor) holds three
codons, `3 CAC build`, `7 CAA sense` and `11 AAG grant`. The middle codon,
`sense` on segment 7, is the centre, and `build` and `grant` are the other two
bridges. The three are also the only codons the fold view draws at floor 1.

## Gate 8

| # | Item | Control artifact | Opposes |
|---|---|---|---|
| 1 | The reader holds the one table | `corpus/phase8/strands/twelve.proteins.txt` | `Replace`: twelve read by the codex with `AAA` and `TTT` swapped |
| 2 | Every order holds the one before it | `corpus/phase8/tower/twelve.floors.txt` | `Replace`: twelve's floors with T5 filtering at level ≥ 0 |
| 3 | The pool has no cycles | `corpus/phase8/pool/cycle.chromosome` | `Replace`: the cycle witness as a pool without the cycle check holds it |

The three printed opposites are constants in `gate_eight_items.rs`. A test
proves that each one is what its row claims:
- The swapped reading is `translate` of twelve with every `AAA` and `TTT`
  codon exchanged.
- The flat floors are twelve's floor 0 line repeated at every floor.
- The one-way pool is the cycle witness without its closing reference.

`cargo xtask gate 8`:

```
harness fixtures: ok
reader: codex 1 names 14 codons, one per meaning; 12 strand witnesses read back byte for byte
floors: 7 facts hold at orders 1 . 6; 6 tower witnesses derive byte for byte
pool: the cycle witness is refused at its second chromosome; pool: gene 8b126fce9ab6 contains itself through 8b126fce9ab6 → 8b126fce9ab6; acceptance is a pool without cycles
1 ok  The reader holds the one table
2 ok  Every order holds the one before it
3 ok  The pool has no cycles
item 1 control answered by check
item 2 control answered by check
item 3 control answered by check
phase 8: 3/3
```

`cargo xtask gate all` exits 0. Its summary:

```
phase 0: pass
phase 1: 4/4 legacy
phase 2: 4/4 legacy
phase 2.1: 8/8 legacy
phase 2.2: 9/9 legacy
phase 3: 8/8 legacy
phase 4: 4/4
phase 5: 8/8
phase 5.1: 4/4
phase 5.2: 3/3
phase 6: 3/3
phase 7: 3/3
phase 7.2: 3/3
phase 7.3: 3/3
phase 7.4: 3/3
phase 8: 3/3
```

`gates.lock` gained one line, `phase 8: 3/3`.

### Shown then reverted

Each break was made in the source, `cargo xtask gate 8` was run, and the file
was restored with `git checkout`.

**(a) `AAA` given the meaning `split`.** In `codex.rs`, row 2's `means` was set
to `Acid(Split)`. Exit 1. Item 1's control answers true on the real artifact,
because the reader no longer prints the witness. Item 2's control also fires,
because `twelve.floors.txt` names each codon's meaning (see snag 2 of the
P8-11 commit).

```
harness fixtures: ok
pool: the cycle witness is refused at its second chromosome; pool: gene 8b126fce9ab6 contains itself through 8b126fce9ab6 → 8b126fce9ab6; acceptance is a pool without cycles
gate item 1 (The reader holds the one table): control answered true on real subject
gate item 2 (Every order holds the one before it): control answered true on real subject
```

**(b) T5 filtering at level ≥ 0.** In `facts/all.rs`, `t5_filter(order,
nodes, below, 1)` was changed to `0`. Exit 1, and only item 2 fails.

```
harness fixtures: ok
reader: codex 1 names 14 codons, one per meaning; 12 strand witnesses read back byte for byte
floors: order 1 T5: tower: order 1 kept segment 0 (level ≥ 0) runs N; order 0 segment 0 runs nothing; acceptance is the level filter giving order 0's directions
floors: order 2 T5: tower: order 2 kept segment 0 (level ≥ 0) runs E; order 1 segment 0 runs N; acceptance is the level filter giving order 1's directions
floors: order 3 T5: tower: order 3 kept segment 0 (level ≥ 0) runs N; order 2 segment 0 runs E; acceptance is the level filter giving order 2's directions
floors: order 4 T5: tower: order 4 kept segment 0 (level ≥ 0) runs E; order 3 segment 0 runs N; acceptance is the level filter giving order 3's directions
floors: order 5 T5: tower: order 5 kept segment 0 (level ≥ 0) runs N; order 4 segment 0 runs E; acceptance is the level filter giving order 4's directions
floors: order 6 T5: tower: order 6 kept segment 0 (level ≥ 0) runs E; order 5 segment 0 runs N; acceptance is the level filter giving order 5's directions
pool: the cycle witness is refused at its second chromosome; pool: gene 8b126fce9ab6 contains itself through 8b126fce9ab6 → 8b126fce9ab6; acceptance is a pool without cycles
1 ok  The reader holds the one table
2 fail  Every order holds the one before it
3 ok  The pool has no cycles
item 1 control answered by check
item 2 control answered by check
item 3 control answered by check
phase 8: 2/3
```

**(c) A gene allowed to reference itself.** In `pool/cycle.rs`, a step from
the entering chromosome straight back to itself was skipped (`if next == me &&
path.len() == 1 { continue; }`). Exit 1, and only item 3 fails. The two-gene
cycle witness is still refused, because its path is longer than one step.

```
harness fixtures: ok
reader: codex 1 names 14 codons, one per meaning; 12 strand witnesses read back byte for byte
floors: 7 facts hold at orders 1 . 6; 6 tower witnesses derive byte for byte
pool: a gene naming itself entered; acceptance is a pool without cycles
1 ok  The reader holds the one table
2 ok  Every order holds the one before it
3 fail  The pool has no cycles
item 1 control answered by check
item 2 control answered by check
item 3 control answered by check
phase 8: 2/3
```

## The views

The window runs below were done under rule 67: only windows Cursor started,
and no mouse or keyboard input outside them. Each run printed this first tick
line:

- `joinn-desktop --strand corpus/phase8/strands/twelve.strand`:
  `tick: level 3 step 128 (k 12), anchor strand, rows 576`
- `joinn-desktop --fold corpus/phase8/tower/twelve.chromosome`:
  `tick: level 4 step 192 (k 28), anchor fold, rows 73`

The helix is drawn still: projected, not turning (P8-09 snag, V147). §6.2's
"watch the helix turn" will therefore show a fixed helix.

## Appendix: every command's output

### `cargo xtask strand <file>`, each strand witness

```
file      corpus/phase8/strands/empty.strand
codex 1
strand    AAT ATT
hash      994c4e6ed4098cbb1a24e6f61087f183845ffa7026adba78fa90d5254b9e9c65
partner   AAT ATT
hash      994c4e6ed4098cbb1a24e6f61087f183845ffa7026adba78fa90d5254b9e9c65
side      sense (head AAT 00 00 01 odd)
partner side sense (head AAT 00 00 01 odd)
partner from its tail TTA TAA: side partner (head TTA 01 01 00 even)
strand: 6 letters, 2 codons, side sense
```

```
file      corpus/phase8/strands/pair-split.strand
codex 1
strand    AAT AAA TTT ATT
hash      8b126fce9ab660a543562f38078cfe03e77aa2bc748ccea5af6e9b51f345ebb9
partner   AAT AAA TTT ATT
hash      8b126fce9ab660a543562f38078cfe03e77aa2bc748ccea5af6e9b51f345ebb9
side      sense (head AAT 00 00 01 odd)
partner side sense (head AAT 00 00 01 odd)
partner from its tail TTA TTT AAA TAA: side partner (head TTA 01 01 00 even)
strand: 12 letters, 4 codons, side sense
```

```
file      corpus/phase8/strands/twelve.strand
codex 1
strand    AAT AAA TTT CAC GTG AAC GTT CAA TTG GAA TTC AAG CTT ATT
hash      edf1529a6fc21715846486999b3252f1336462dc70b16f9edeb70eb96e617bca
partner   AAT AAG CTT GAA TTC CAA TTG AAC GTT CAC GTG AAA TTT ATT
hash      93e7a63fea770512acca5c04cd66958fb46b2d24905f4d9fe5486aa19f5f3a83
side      sense (head AAT 00 00 01 odd)
partner side sense (head AAT 00 00 01 odd)
partner from its tail TTA TTT AAA GTG CAC TTG CAA GTT AAC CTT AAG TTC GAA TAA: side partner (head TTA 01 01 00 even)
strand: 42 letters, 14 codons, side sense
```

```
file      corpus/phase8/strands/reserved.strand
codex 1
strand    AAT TAA ATT
hash      62ce1cb6359a29dade67078cfc4d7bae24b35a28e4533c7901950e2a4014e803
partner   AAT TTA ATT
hash      b90ccc1f665e6653587e39e26cf4c9497c4c9d76b1ddb49779eb6c3006bb6242
side      sense (head AAT 00 00 01 odd)
partner side sense (head AAT 00 00 01 odd)
partner from its tail TTA ATT TAA: side partner (head TTA 01 01 00 even)
strand: 9 letters, 3 codons, side sense
```

### `cargo xtask codex`

```
codex 1: codon bits parity means | partner bits parity means
AAT 00 00 01 odd   START  | ATT 00 01 01 even  STOP
AAA 00 00 00 even  pair   | TTT 01 01 01 odd   split
CAC 11 00 11 even  build  | GTG 10 01 10 odd   case
AAC 00 00 11 even  refer  | GTT 10 01 01 odd   return
CAA 11 00 00 even  sense  | TTG 01 01 10 odd   show
GAA 10 00 00 odd   keep   | TTC 01 01 11 even  recall
AAG 00 00 10 odd   grant  | CTT 11 01 01 even  revoke
reserved (50):
  ATA ATG ATC AGA AGT AGG AGC ACA ACT ACG
  ACC TAA TAT TAG TAC TTA TGA TGT TGG TGC
  TCA TCT TCG TCC GAT GAG GAC GTA GTC GGA
  GGT GGG GGC GCA GCT GCG GCC CAT CAG CTA
  CTG CTC CGA CGT CGG CGC CCA CCT CCG CCC
parity: AAT (START) 00 00 01 odd; ATT (STOP) 00 01 01 even; a codon and its partner differ in parity for 64 of 64
organelle screen: show
organelle input: sense
organelle store: keep, recall
organelle clock: sense
reader 8.0
codex 1
codex 1: 14 named, 50 reserved; every partner names the opposite
```

### `cargo xtask read <file>`, each strand witness

```
file      corpus/phase8/strands/empty.strand
codex 1 · reader 8.0
strand    AAT ATT
protein at letter 0: nothing
partner   AAT ATT
partner protein at letter 0: nothing
read: 1 proteins; partner reads the opposites
```

```
file      corpus/phase8/strands/pair-split.strand
codex 1 · reader 8.0
strand    AAT AAA TTT ATT
protein at letter 0: pair split
partner   AAT AAA TTT ATT
partner protein at letter 0: pair split
read: 1 proteins; partner reads the opposites
```

```
file      corpus/phase8/strands/twelve.strand
codex 1 · reader 8.0
strand    AAT AAA TTT CAC GTG AAC GTT CAA TTG GAA TTC AAG CTT ATT
protein at letter 0: pair split build case refer return sense show keep recall grant revoke
partner   AAT AAG CTT GAA TTC CAA TTG AAC GTT CAC GTG AAA TTT ATT
partner protein at letter 0: grant revoke keep recall sense show refer return build case pair split
read: 1 proteins; partner reads the opposites
```

`reserved.strand` is refused (exit 1):

```
corpus/phase8/strands/reserved.strand: reader: codon TAA at letter 3 names nothing in codex 1; acceptance is a codon codex 1 names
```

### `cargo xtask tower`

```
tower: dimension 2, orders 1-6
order 1: 4 nodes, 3 segments
  T1 node 0 at (0, 0); node 3 at (1, 0)
  T2 3 steps, each one unit
  T3 4 nodes nest in order 0
  T4 levels 0 ×3
  T5 0 segments of level ≥ 1 give order 0's 0 directions
  T6 mirror holds over 3 directions
  T7 top floor (level 0): segments 0, 1, 2; the middle, 1, is the centre
order 2: 16 nodes, 15 segments
  T1 node 0 at (0, 0); node 15 at (3, 0)
  T2 15 steps, each one unit
  T3 16 nodes nest in order 1
  T4 levels 0 ×12, 1 ×3
  T5 3 segments of level ≥ 1 give order 1's 3 directions
  T6 mirror holds over 15 directions
  T7 top floor (level 1): segments 3, 7, 11; the middle, 7, is the centre
order 3: 64 nodes, 63 segments
  T1 node 0 at (0, 0); node 63 at (7, 0)
  T2 63 steps, each one unit
  T3 64 nodes nest in order 2
  T4 levels 0 ×48, 1 ×12, 2 ×3
  T5 15 segments of level ≥ 1 give order 2's 15 directions
  T6 mirror holds over 63 directions
  T7 top floor (level 2): segments 15, 31, 47; the middle, 31, is the centre
order 4: 256 nodes, 255 segments
  T1 node 0 at (0, 0); node 255 at (15, 0)
  T2 255 steps, each one unit
  T3 256 nodes nest in order 3
  T4 levels 0 ×192, 1 ×48, 2 ×12, 3 ×3
  T5 63 segments of level ≥ 1 give order 3's 63 directions
  T6 mirror holds over 255 directions
  T7 top floor (level 3): segments 63, 127, 191; the middle, 127, is the centre
order 5: 1024 nodes, 1023 segments
  T1 node 0 at (0, 0); node 1023 at (31, 0)
  T2 1023 steps, each one unit
  T3 1024 nodes nest in order 4
  T4 levels 0 ×768, 1 ×192, 2 ×48, 3 ×12, 4 ×3
  T5 255 segments of level ≥ 1 give order 4's 255 directions
  T6 mirror holds over 1023 directions
  T7 top floor (level 4): segments 255, 511, 767; the middle, 511, is the centre
order 6: 4096 nodes, 4095 segments
  T1 node 0 at (0, 0); node 4095 at (63, 0)
  T2 4095 steps, each one unit
  T3 4096 nodes nest in order 5
  T4 levels 0 ×3072, 1 ×768, 2 ×192, 3 ×48, 4 ×12, 5 ×3
  T5 1023 segments of level ≥ 1 give order 5's 1023 directions
  T6 mirror holds over 4095 directions
  T7 top floor (level 5): segments 1023, 2047, 3071; the middle, 2047, is the centre
phase8/tower/twelve.chromosome
  chromosome edf1529a6fc21715846486999b3252f1336462dc70b16f9edeb70eb96e617bca order 2: 14 codons on 15 segments, 1 silent, 2 gene references
  floor 0 (level ≥ 0): 14 codons: 0 AAT START, 1 AAA pair, 2 TTT split, 3 CAC build, 4 GTG case, 5 AAC refer, 6 GTT return, 7 CAA sense, 8 TTG show, 9 GAA keep, 10 TTC recall, 11 AAG grant, 12 CTT revoke, 13 ATT STOP
  floor 1 (level ≥ 1): 3 codons: 3 CAC build, 7 CAA sense, 11 AAG grant
phase8/pool/cycle.chromosome
  chromosome 8b126fce9ab660a543562f38078cfe03e77aa2bc748ccea5af6e9b51f345ebb9 order 2: 4 codons on 15 segments, 11 silent, 1 gene references
  floor 0 (level ≥ 0): 4 codons: 0 AAT START, 1 AAA pair, 2 TTT split, 3 ATT STOP
  floor 1 (level ≥ 1): 1 codons: 3 ATT STOP
phase8/pool/cycle.chromosome
  chromosome 9be547e78b9ccb3226a19820365488f2a3264113ad439cf2c74d782e5073151b order 2: 4 codons on 15 segments, 11 silent, 1 gene references
  floor 0 (level ≥ 0): 4 codons: 0 AAT START, 1 CAC build, 2 GTG case, 3 ATT STOP
  floor 1 (level ≥ 1): 1 codons: 3 ATT STOP
dimension 4: tower: dimension 4 is not built; acceptance is dimension 2
tower: orders 1-6 hold T1-T7; every order holds the one before it
```

### `cargo xtask pool`

```
pool: phase8/strands/empty.strand 994c4e6ed409 order 1, 0 gene references
pool: phase8/strands/pair-split.strand 8b126fce9ab6 order 2, 0 gene references
pool: phase8/tower/twelve.chromosome edf1529a6fc2 order 2, 2 gene references (node 5 → 8b126fce9ab6, node 10 → 994c4e6ed409)
pool holds 3 chromosomes; reads 0 (nothing is read until asked)
expand twelve node 5 → 8b126fce9ab6: AAT AAA TTT ATT (reads 1)
expand twelve node 10 → 994c4e6ed409: AAT ATT (reads 2)
expand twelve node 3 → pool: node 3 holds no gene; acceptance is a node with a gene reference
expand twelve node 5 in a pool without pair-split → pool: node 5 names gene 8b126fce9ab6, which the pool does not hold
cycle: phase8/pool/cycle.chromosome
  enters 8b126fce9ab6
  refused: pool: gene 9be547e78b9c contains itself through 9be547e78b9c → 8b126fce9ab6 → 9be547e78b9c; acceptance is a pool without cycles
pool: no cycles; grows only when asked
```

### `cargo xtask dna-witness`

Every witness was already present with the same bytes, so none was rewritten.

```
phase8/strands/empty.strand 85b50b895a35f50f7a24d4f7a81c4da869a81358523d0df3dc5a07b166c42cae   # present
phase8/strands/empty.proteins.txt 25eb9999135b7778f5a705c8a660e9e9b59940a0854e4e16b4de9dcf9102afdf   # present
phase8/strands/empty.partner.txt d59c833f3a9364ac3e6d7ba0a97384f9c2657e14a4ee5f2f428ba50735b08a93   # present
phase8/strands/pair-split.strand e8962be0b75fa76fe899e8cf27cbd1b45e8c417fae1d7c145a4efae1425b926f   # present
phase8/strands/pair-split.proteins.txt 0522dbaf2c8cf8a72847c0ded1239e42a20d1adbeeba06466d7753a869e1b650   # present
phase8/strands/pair-split.partner.txt 4e37158fe7cc0f45390a8c60a4ddb707fd5304091e8801f3c95100efc87cb5b5   # present
phase8/strands/twelve.strand ed54905b20672d0ce6073dc5ffe10f910776a60dceae80cc64287a6978964084   # present
phase8/strands/twelve.proteins.txt 3f4f7ebda883af3f522b84ed92196ae268f292600a3752a1d2e8e0aef4949ee7   # present
phase8/strands/twelve.partner.txt 7a343906ebcbd373ec477baefe77686e7f745c7e306913ae9683cb5e81cbe6a9   # present
phase8/strands/reserved.strand a5f7c568c3048e905e62db691af806a5f5399cbb72f86420aeb3c569f7db7673   # present
phase8/strands/reserved.proteins.txt f5999bcd7c24e5308a8e30506f8c631b425baaf3f676b7beb67a596aa3f11d73   # present
phase8/strands/reserved.partner.txt 052d30087b3988e2e774752dcfe61b09f6e1d9699ca666e223ee285bbd1da8c8   # present
phase8/tower/order-1.txt c439c39d9b213b8f10eded32fa0a4a2ef48ead5fbc75214fa02f356c1bdda685   # present
phase8/tower/order-2.txt 11dfce02ed9e85505515b1d4b6d5d4c8874dc236fcdc74baa694f9bf000c0fe2   # present
phase8/tower/order-3.txt 4667528fd6ff0ca4ba087c956c6c4074736b80ab24ec51645a95bf22df2b7a20   # present
phase8/tower/order-4.txt a6e6552af2e89948923f3895be41705224f5915532ce0580162a7e5f387976a8   # present
phase8/tower/twelve.chromosome 2bc09f8ef4250e377e3fe1c9ee90465b0f0fbbc88430c719be55773c71b2206e   # present
phase8/tower/twelve.floors.txt a305442889206483885f40b0f0f0a419bad697b86f6751186bbcf39c1d615f00   # present
phase8/pool/cycle.chromosome c00684aa761f67969f18e4d537a048c0fff9f7909117e7b0cbbfa660016e7381   # present
```
