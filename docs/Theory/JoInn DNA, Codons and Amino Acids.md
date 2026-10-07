# JoInn DNA, Codons and Amino Acids

**Part VIII: four letters, one reader, and everything built from them**

*Theory only · no implementation implied*

Author: AJ, with Claude · Draft 0.4 · October 7, 2026

> Status tags follow Part I: **DECIDED** (AJ said it), **PROPOSED** (Claude's reading, open to AJ's veto), **OPEN**. Decisions are numbered D1–D30 in §13. Research items R118–R127 are in §14.

> **Thesis.** JoInn's DNA is a strand of four letters, A, T, G and C, that mean nothing on their own. Read three at a time, they make codons. One fixed table, held by the reader and nowhere else, turns each codon into an amino acid, a start or a stop. Chains of amino acids make proteins, proteins make cells, and cells make everything above them. **The platform knows the letters, the table, the reader and the laws. Everything else is written in DNA.** That is how JoInn stops hard-coding.

---

## For AJ: this note in plain English

- **Why this note exists.** Phase 7.4 worked, but it showed the problem: every phase built a new *reader* (a new file kind, a new Rust rule) instead of writing new *DNA*. Number kinds live in Rust. The force register knows ℤ. Counting's `accepts one` is a keyword. You said it plainly: *add is add, whether it's a number, a string or a sheep.*
- **The fix is the one nature uses.** Four letters, one universal reader, a fixed table, and physics doing the shaping. All life (bacteria, plants, animals) uses the same four letters and nearly the same table.
- **The letters mean nothing alone.** A, T, G, C are just "one of four" (2 bits each). Meaning appears only when three are read together as a codon and looked up in the table.
- **The table is the only place meaning lives.** If something isn't in the table, JoInn can't express it. That's the guard against hard-coding.
- **12 amino acids, in 6 opposite pairs.** Put together / take apart, make / read, go in / come back, sense / show, keep / recall, grant / revoke. Plus start and stop. Anything that can be derived (add, equal, count, compare) is a protein, not an amino acid.
- **The table is built on opposition.** Every codon's partner, read backwards on the other strand, names its opposite amino acid. So **reading a protein's partner strand gives its opposite protein**: the undo. Opposition in all things, built into the letters themselves.
- **Nothing isn't stored.** There's no letter for nothing and no "null." The empty chain (start, then stop) is nothing.
- **Switches** are stretches of DNA that turn proteins on and off. They're how one app works on a phone and a desktop: the device is the surroundings, and the switches respond to it.
- **Two sets:** the **laws** (JoInn's physics: the reader, the table, the forces, exactness), and **DNA** (everything users build, including engineering knowledge).
- **You build at the chain.** A 3D builder where you snap amino acids onto a chain. You never type A, T, G, C. The first thing you'll see is the strand itself, in 3D.
- **What happens to what's built.** It's kept, in new crates beside today's code. Today's cells become witnesses the new proteins must agree with. Phase 7.5 is paused.

---

## 1. Where JoInn Stands: a Review

### 1.1 What the build has proven (Phases 0–7.4)

| Built | Phase | Status under Part VIII |
|---|---|---|
| Identity by hash; canonical form; lineage | 0–1 | **Kept.** A strand is hashed the same way |
| Laws, sampling, witness replay, the four checks, refusals | 1–2 | **Kept.** Every amino acid and protein is admitted by them |
| The Floor: `pair`/`split`, `build`/`case`, `eq`/`choose`, each with a declared opposition | 2 | **Becomes the amino acids** (§5). Four of the six are already there |
| Reference bodies (integer add built from the Floor), seals, fast alleles that must agree | 2.1–2.2 | **Kept.** This is exactly how fast Rust stays honest under DNA |
| The assay layer (incidence combinatorics) | 4 | **Kept** |
| Bodies, universes, links, grants | 5–5.2 | **Witnesses**, then retired as grammars unify |
| GPU drawing, exact picking, zoom, bands, the stroke font, links in gutters, the lasso | 6–7.4 | **Kept**, and extended to 3D |
| The gate harness, `gates.lock`, standing plants, `stop-check` | all | **Kept**. Old phases stay as legacy witnesses |

The trust machinery (identity, laws, opposition, witnesses, admission) is the best-built part of JoInn. It's the immune system, built before the organism. None of it is wasted.

### 1.2 What drifted

| Where | What the platform knows that it shouldn't |
|---|---|
| `joinn-frame` | Every kind is Rust: `IntFrame`, `RatFrame`, `TextFrame`. Nobody can add sheep without a programmer |
| `joinn-prim` natives | Every cell runs a Rust function: `add@ℤ`, `mul@ℤ`, `format@ℤ`, `parse@Text`, `add@ℚ` |
| The force register | A Rust table: combine on ℤ → the sum cell |
| `joinn-dna` | Five grammars: `.cell`, `.body`, `.contact`, `.system`, `.universe`. Each new idea got a new reader |
| Phase 7.4 | `accepts one \| any` is a two-word keyword; the witness transcripts are a Rust table |

**Why it happened.** Each phase needed something visible and exactly checkable, and the cheapest proof is one concrete example. The gates pinned that example byte for byte. Nothing ever asked: *could a stranger do this with DNA alone?*

### 1.3 The rule that would have caught it (D1, **DECIDED**)

> **Add is add.** A force works on anything that can answer its questions: numbers, text, sheep. The platform may know forces and laws. Only DNA may know kinds.

---

## 2. Two Sets: the Laws and the DNA

AJ asked whether there are two sets: one for the living and one for the nonliving (D2, **DECIDED**).

In nature, DNA contains no physics. It records the order of a chain, and chemistry folds it. Bone is stiff because of mechanics, not because stiffness is encoded. The living is **written against** physics, and **listens** to it through receptors, but never contains it.

| | **Laws** (nonliving) | **DNA** (living) |
|---|---|---|
| Is | how JoInn's world behaves | everything built in that world |
| Holds | the letters, the code table, the reader, the forces (gather/separate, carry/release), opposition, exactness, the device's senses, screen and storage | strands, proteins, cells, bodies, systems, kinds, regulation, UX |
| Written by | the platform, once | anyone, endlessly |
| Changes | almost never (a new code table is a new `codex`) | grows, evolves, branches |
| Knows the other? | no: the laws never know what a sheep is | yes: DNA is written to use the laws |

**One rule at the boundary** (D3, **DECIDED**): engineering knowledge about the real world (beam bending, steel strength, soil bearing) is **DNA**, not law. It describes real physics, but it is testimony that grows, and a design must be able to stay pinned to the code it was built to. JoInn's own physics (its forces, opposition, exact arithmetic at the bottom) is law.

---

## 3. The Letters

### 3.1 Four letters, no meaning (D4, D5)

JoInn's DNA is written in **A, T, G and C** (D4, **DECIDED**). The names are kept on purpose: they aren't words, so nothing can be read into them.

A letter carries exactly one thing: **which of four it is** (2 bits). **A letter alone means nothing** (D5, **DECIDED**). Meaning appears only when letters are grouped and read.

### 3.2 A strand

A **strand** is a row of letters with a direction: it has a **head** end and a **tail** end, and it is read head to tail (D6, **DECIDED**). In nature the ends are called 5′ and 3′.

### 3.3 Partners (D7)

Every letter has one partner: **A pairs with T, and G with C** (D7, **DECIDED**, as in nature). Every strand has a **partner strand**: each letter replaced by its partner, running the other way.

```
strand          head  A A A  C A C  tail      (pair, build)
partner strand  tail  T T T  G T G  head      (read head to tail: G T G  T T T = case, split)
```

What the partner is *for* in JoInn is in §6.3. It's the most important property of the table.

### 3.4 Storage and identity (D8)

A strand is stored packed, four letters to a byte. **Its identity is the hash of the strand** (§4.4), the same way every JoInn identity works today (D8, **DECIDED**).

---

## 4. Codons and the Code

### 4.1 Codons (D9)

The reader reads letters **three at a time**. Three letters is a **codon**, and there are 4 × 4 × 4 = **64** of them (D9, **DECIDED**). Two letters would give only 16, too few to hold the amino acids, their opposites, start, stop and room to grow.

### 4.2 The reading frame (D10)

The same letters grouped differently say different things. The reader finds a **start** codon and counts in threes from there until it reaches a **stop** (D10, **DECIDED**):

```
… G G A A T  A A A  C A C  G A G  A T T  C C …
            start pair build case  stop
```

Shift the grouping by one letter and the meaning changes completely, so a chain begins only at a start.

### 4.3 The code table lives in the reader (D11)

> **The code table is the only place meaning is assigned, and it lives in the reader, not in the DNA.** (D11, **DECIDED**)

In nature, nothing about the chemistry of `GCA` makes it mean alanine. Small adapter molecules (tRNAs) make the match: one end fits the codon, the other end carries the amino acid. The table is in the reader's adapters.

JoInn does the same. The table is part of the laws (§2). It is fixed, universal (every JoInn universe reads codons the same way, so a protein from one user's universe works in anyone's), and versioned. **The `codex 1` line every JoInn file already carries becomes the table's version** (D12, **DECIDED**).

### 4.4 Every letter counts: no synonyms (D13)

In nature several codons name the same amino acid (synonyms), so many single-letter copying errors change nothing. Nature needs that because it copies by chemistry, which makes mistakes.

**JoInn does not use synonyms. All three letters of a codon carry meaning, so the table can hold 64 different things** (D13, **DECIDED**, revised 7 Oct). AJ's reason: using only two letters gives 16 meanings, and JoInn already uses 14; growing past 16 would force more and more exceptions where the middle letter suddenly matters. With all three letters, every codon is its own entry and nothing needs an exception.

What this gives:

- **One protein, one strand.** With no synonyms, a protein can be written exactly one way, so the strand *is* the canonical form and its hash is its identity (D8).
- **Room to grow:** 64 codons make 32 partner pairs (§6.3). Start/stop use one pair and the twelve amino acids six, leaving **25 pairs** for the future.

What it costs, honestly: a single changed letter now always changes the meaning. JoInn doesn't rely on synonyms for safety; it copies digitally and checks every copy twice: against the hash, and against the partner strand (§3.3). A changed letter is caught before anything reads it (R125).

---

## 5. The Amino Acids

### 5.1 The rule for what qualifies (D14)

AJ's rule from 1 Oct decides: **anything that can be derived is not an amino acid** (D14, **DECIDED**). An amino acid must be either:

1. **irreducible**: it can't be built from the others, or
2. **a touch on the outside world**: the device's senses, screen and storage, which no amount of DNA can derive.

And opposition in all things: **every amino acid comes with its opposite**. They're added in pairs or not at all (D15, **DECIDED**).

### 5.2 The twelve (D16)

| Pair | Amino acid | Does | Its opposite | Does | Today's Floor |
|---|---|---|---|---|---|
| **Together** | **pair** | puts two things together as one | **split** | takes a pair apart into its two | `prim:pair` / `prim:split` |
| **Making** | **build** | makes a thing marked with what it is | **case** | reads what a thing is marked as, and goes that way | `prim:build` / `prim:case` |
| **Going** | **refer** | goes into another protein (or itself) by its hash | **return** | comes back out with the result | recursion in reference bodies |
| **Senses** | **sense** | takes a signal in from the device (a key, a tap, a clock, the screen size) | **show** | puts something out to be seen | the hosts (Phases 5, 6) |
| **Memory** | **keep** | stores a thing under a name | **recall** | reads it back | (new) |
| **Permission** | **grant** | opens the membrane to something | **revoke** | closes it | `grant` / `revoke` (Phase 5.2) |

That is the whole list (D16, **DECIDED**). Twelve is far under the ceiling of 64 AJ noted. The rest of the room is the point (§6.2).

### 5.3 What is *not* an amino acid, and why

| Not an amino acid | Because it is |
|---|---|
| **eq** (are these the same?) | a protein: `case` both, compare marks, `refer` to itself on the parts |
| **choose** (pick one by yes/no) | `case` on a thing built as yes or no |
| **add, count, compare, multiply** | proteins on chains of tallies (§7) |
| **turn** (direction, negative numbers) | `case` on a direction mark, then `build` the other |
| **swap, copy, drop** | `split` and `pair` in a new order |
| **gather, separate, carry, release** | **laws**, not amino acids. Forces act on bodies from systems, like chemistry acting on a protein. Proteins don't call them |
| **parallel** | not needed: proteins run at the same time unless one waits on another (§9) |
| **nothing** | not stored at all (§8) |

`eq` and `choose` are in today's Floor. Under D14 they become the first two proteins, and today's Rust versions become their witnesses.

---

## 6. The Table

### 6.1 The table (D17)

Every codon is read whole, all three letters (D13). Codex 1 uses 14 of the 64 codons (D17, **DECIDED**, revised 7 Oct):

| Codon | Means | Its partner (read backwards) | Means |
|---|---|---|---|
| `AAT` | **START** | `ATT` | **STOP** |
| `AAA` | pair | `TTT` | split |
| `CAC` | build | `GTG` | case |
| `AAC` | refer | `GTT` | return |
| `CAA` | sense | `TTG` | show |
| `GAA` | keep | `TTC` | recall |
| `AAG` | grant | `CTT` | revoke |

Every other codon (50 of them) is **reserved**: it names nothing in codex 1.

These are the same codons the first draft used as each amino acid's first spelling, so nothing chosen so far moves. Each pair sits on a codon and its partner, so §6.3's property holds.

### 6.2 Reserved codons are room to grow (D18)

The 50 reserved codons name **no amino acid in codex 1**. Reading one is a refusal, not a guess: *codon `TAA` names nothing in codex 1*. They form **25 partner pairs**, and each can later hold **one opposite pair** of amino acids, added only when something is found that truly can't be derived (D18, **PROPOSED**).

### 6.3 The partner strand reads the opposite protein (D19)

The table was laid out so that **every codon's partner, read backwards, names its opposite amino acid** (D19, **DECIDED**):

| Codon | Partner, read backwards | |
|---|---|---|
| `AAA` pair | `TTT` split | |
| `CAC` build | `GTG` case | |
| `AAC` refer | `GTT` return | |
| `CAA` sense | `TTG` show | |
| `GAA` keep | `TTC` recall | |
| `AAG` grant | `CTT` revoke | |
| `AAT` start | `ATT` stop | |

No codon is its own partner, so this works for all 64. Claude checked it by program: every codon, every partner.

**What it gives.** A protein is a chain read head to tail: start, *a₁, a₂ … aₙ*, stop. Its partner strand, read head to tail, is start, *opposite(aₙ) … opposite(a₁)*, stop: **the opposites, in reverse order.** That is exactly how you undo a chain of steps: undo the last one first.

- For the structure pairs (pair/split, build/case, refer/return), the partner protein **is the undo**.
- For the world pairs (sense/show, keep/recall, grant/revoke), it is the **mirror**: what came in goes out.

So every protein carries its own opposite, written in the same letters. The creator's undo (Phase 9's "undo is a real inverse") stops being a feature to build; it's the partner strand. This is **opposition in all things in the material itself.**

**An honest note.** Nature's partner strand does *not* encode opposite proteins; the two strands of real DNA usually hold unrelated genes. This is JoInn's own design, borrowing nature's pairing for a purpose nature doesn't use it for.

### 6.4 What a single changed letter does

Claude counted every possible single-letter change to the 14 codons in use (126 changes):

| Lands on | Count |
|---|---|
| a reserved codon (a refusal: visible) | **80** |
| a different amino acid | 44 |
| its opposite | 2 (start ↔ stop) |

With no synonyms, no change is harmless, which is why every copy is checked against its hash and its partner strand before it is read (§3.3, R125). While most of the table is reserved, most changes also fall on a reserved codon and are refused.

---

## 7. Proteins

### 7.1 A protein is a chain (D20)

A **protein** is a chain of amino acids from a start to a stop (D20, **DECIDED**).

**How the reader runs a chain** (D21, **DECIDED**): each amino acid takes what it needs from the things the chain is holding and leaves its result for the next one, in order. Chaining two proteins is **putting one strand after the other**. No wiring language is needed. This is how the "concatenative" languages work (Forth, Joy, Factor), and it is the natural reading of a chain.

### 7.2 A protein's shape is derived (D22)

A real protein's shape is its job, and nobody writes the shape down: physics folds it from the chain.

**A JoInn protein's shape (its ports: how many things it takes in, how many it gives out) is derived from its chain** by the reader (D22, **DECIDED**). Nobody declares ports. That matches Part V's rule that a body's outline is derived.

A protein also carries **laws** (its truth) and **witnesses**, admitted by the gate exactly as cells are today.

### 7.3 Kinds are built, not coded (D23)

A kind is what a chain is built *as* (D23, **DECIDED**):

| Kind | Built as |
|---|---|
| a **count** | a chain of tallies: 3 is tally, tally, tally (all math is counting, made literal) |
| a **whole number** | a count marked with a direction (up or down) |
| a **fraction** | a pair: a top and a bottom, with the law "the bottom is never nothing" |
| **text** | a chain of letters, each marked as a letter, in order |
| a **sheep** | a name, marked "sheep" |
| a **flock** | a chain of sheep, marked "flock" |

**Add is a protein** that walks one chain onto another. It doesn't look inside the beads, so it adds counts, joins text, and gathers sheep into a flock. Whether order matters, and what the result contracts into, are the kind's laws (Part VII's three questions), written in the kind's DNA.

Fractions are built *from* whole numbers, so "3 is 3/1" is not a hand-written rule; it follows from how the fraction is built. That answers R119 (§14).

### 7.4 Fast alleles stay honest (D24)

A count of a million is a million tallies, which is slow. JoInn already has the answer: **a fast Rust allele may run instead, but only if it agrees with the protein, and the protein stays the truth** (D24, **DECIDED**). Today's natives (`add@ℤ`, `mul@ℤ`) become exactly that, checked by the existing seal machinery. Phase 10 (the compiler) is where speed is earned in general.

---

## 8. Nothing (D25)

DNA has no letter for nothing, and no blank space. **Nothing is something the reader recognizes, never something stored** (D25, **PROPOSED**). Nature shows three kinds:

| Nothing in nature | In JoInn |
|---|---|
| A stop codon: no adapter fits, so the chain is released | `STOP` |
| A stretch the protein reader skips (it's a switch, or not for this reader) | regions read by other readers (§10) |
| A gene switched off: no protein is made | a switch that isn't bound |

**The empty chain**, start followed directly by stop, is nothing. It is the empty count (0), empty text, and an empty pasture, all at once, so kinds don't have to declare their own nothing.

**There is no null.** Absence is never a value someone can forget to check; it's the reader's silence. A reserved codon is a refusal, not a null.

One caution: the empty chain is "changes nothing" for **gather** only. For multiply, "changes nothing" is 1. That belongs to the force (a law), not the kind.

---

## 9. Running Together

In a cell, many proteins work at once, and many ribosomes read the same message at the same time.

**Proteins run at the same time unless one waits on another** (D26, **PROPOSED**). Two parts of a chain that don't share anything may run in parallel; the reader can see that from the chain. The amino acids that touch the world (sense, show, keep, recall, grant, revoke) happen in strand order, so a run is the same every time.

**Why this is safe in JoInn when it isn't in most systems:** the gather force is already required to be **order-blind**, and the gate checks it (Part V, Phase 7). Gathering on 8 cores or a GPU gives the same answer as gathering on 1. The guarantee came first.

---

## 10. Switches (D27)

Not all DNA is codons. Some stretches are **switches**: short patterns placed before a protein's start, meaning *make this protein when this signal is present*.

**Switches solve the device problems** (D27, **DECIDED**). The device is the surroundings. The platform reports its signals (touch or mouse, phone or desktop, screen size, a strong or weak GPU) as short letter patterns. A switch whose pattern matches a present signal **binds**, and its protein is made. One cell's DNA works everywhere; only what is switched on changes.

```
switch[touch]  start … tap-handling protein … stop
switch[mouse]  start … click-handling protein … stop
```

The same mechanism carries **growth** (Part VII): *make another number cell when a number arrives*. Phase 7.4's `accepts one` stops being a keyword and becomes a switch and a protein that checks the arrival.

**One strand, several readers** (D28, **PROPOSED**): the protein reader reads codons; the switch reader reads switches. That is how one strand holds both the recipes and the instructions for when to use them, and **why five grammars become one**: a cell, a body, a system and a universe are all strands, read by the same readers, at different zoom.

---

## 11. Building It: the Chain Builder

### 11.1 You build at the chain (D29)

**AJ asked for a 3D visual builder at the chain layer** (D29, **DECIDED**). Three views of one strand, joined by the semantic zoom JoInn already has:

| View | Shows | You do |
|---|---|---|
| **Strand** (closest) | the double helix: letters as coloured rungs grouped in threes, each codon tinted by its amino acid, the partner strand beside it. A mismatched pair glows | look and check. Nobody types A, T, G, C |
| **Chain** | amino acids as beads on a string, each with its own shape and colour; start and stop as end caps | **build**: snap beads on, run the chain, watch a value travel bead to bead |
| **Folded** (zoomed out) | the protein's derived shape: its ports | snap proteins into a cell |

The builder writes the codons for you. You design the protein; the letters follow, as in nature.

**Gentle 3D.** The helix turns on its own axis, chains lie mostly on a working surface, and the camera moves along fixed paths. Free orbit can come later; it loses beginners.

### 11.2 The first thing you'll see (D30, **DECIDED**)

The first visible proof is **the strand view in 3D**: a real strand, read by the real reader through the real table, drawn as a helix with its partner, codons grouped, amino acids named, and a mismatch glowing. Then the chain view, then the calculator built from proteins.

---

## 12. What Happens to What Is Built

**Where:** new crates, in the same repository, beside today's code (**DECIDED**). Nothing is deleted to make room.

| Today | Under Part VIII |
|---|---|
| Hashing, canonical form, lineage, the gate, sampling, witnesses, refusals, seals | **kept**; they admit strands and proteins |
| The Floor's `pair`, `split`, `build`, `case` | **become four amino acids**; their Rust stays as the reader's implementation |
| `eq`, `choose` | become the first proteins; today's Rust is their witness |
| `IntFrame`, `RatFrame`, `TextFrame`; the natives | **witnesses** the new kinds and proteins must agree with, then fast alleles (D24) |
| The five grammars and their admission code | **witnesses**, retired as each is rebuilt as strands |
| The force register in Rust | retired; a kind's laws say how it answers a force |
| Phase 7.4's `accepts`, transcripts, `.system` | witnesses; rebuilt as switches and proteins |
| GPU, picking, zoom, the stroke font, the lasso | **kept**; extended to 3D for the builder |
| `gates.lock` and every past gate | **kept** as legacy witnesses; they must keep passing until each is retired on purpose |

**Phase 7.5 is paused** (**DECIDED**). Of the 7.4 review's fixes, the `gates.lock` fix (compare before writing) still applies and opens the first Part VIII phase. The evolution plant is folded into how evolution is rebuilt.

**The test for every phase from now on: the stranger's kind test** (**PROPOSED**). Every new mechanism must work on at least two unlike kinds (numbers and sheep), with the second written only in DNA and no Rust change. **Did we write new DNA, or build a new reader?** New DNA is progress. A new reader is hard-coding, unless it is a law.

---

## 13. Decisions

| # | Decision | Status |
|---|---|---|
| D1 | Add is add: forces work on any kind that answers their questions. The platform knows forces and laws; only DNA knows kinds | **DECIDED** |
| D2 | Two sets: laws (nonliving: JoInn's physics) and DNA (living: everything built) | **DECIDED** |
| D3 | Engineering knowledge about the real world is DNA (testimony), not law | **DECIDED** |
| D4 | The letters are A, T, G, C | **DECIDED** |
| D5 | A letter alone means nothing; it is one of four | **DECIDED** |
| D6 | A strand has a head and a tail and is read head to tail | **DECIDED** |
| D7 | A pairs with T, G with C; every strand has a partner strand | **DECIDED** |
| D8 | A strand's identity is the hash of the strand | **DECIDED** |
| D9 | Letters are read three at a time: 64 codons. The codon and amino acid layers are kept | **DECIDED** |
| D10 | A chain begins at a start codon and ends at a stop | **DECIDED** |
| D11 | The code table is the only place meaning is assigned, and it lives in the reader | **DECIDED** |
| D12 | `codex` names the table's version | **DECIDED** |
| D13 | All three letters of a codon carry meaning; no synonyms; 64 possible meanings (revised 7 Oct) | **DECIDED** |
| D14 | Anything derivable is not an amino acid; an amino acid is irreducible or touches the world | **DECIDED** |
| D15 | Amino acids come in opposite pairs, added in pairs or not at all | **DECIDED** |
| D16 | Twelve amino acids: pair/split, build/case, refer/return, sense/show, keep/recall, grant/revoke | **DECIDED** |
| D17 | The table of §6.1: 14 codons in use, each pair on a codon and its partner (revised 7 Oct) | **DECIDED** |
| D18 | The 50 reserved codons are room for 25 future pairs; reading one is a refusal | PROPOSED |
| D19 | Every codon's partner, read backwards, names its opposite; a protein's partner strand is its opposite protein (undo, or mirror) | **DECIDED** |
| D20 | A protein is a chain of amino acids from start to stop | **DECIDED** |
| D21 | The reader runs a chain in order, each amino acid taking from and leaving for the next; chaining is putting strands end to end | **DECIDED** |
| D22 | A protein's shape (its ports) is derived from its chain, never declared | **DECIDED** |
| D23 | Kinds are built (counts as chains of tallies, fractions as pairs, a sheep as a name marked "sheep"); add is one protein for all | **DECIDED** |
| D24 | A fast Rust allele may run only if it agrees with the protein; the protein is the truth | **DECIDED** |
| D25 | Nothing is never stored; the empty chain is nothing; there is no null | PROPOSED |
| D26 | Proteins run at the same time unless one waits on another; world touches happen in strand order | PROPOSED |
| D27 | Switches solve device problems: the platform reports signals, matching switches bind, their proteins are made | **DECIDED** |
| D28 | One strand, several readers; the five grammars become one | PROPOSED |
| D29 | A 3D builder at the chain layer: strand, chain and folded views joined by zoom | **DECIDED** |
| D30 | The first visible proof is the 3D strand view; new crates in the same repo; Phase 7.5 paused | **DECIDED** |

---

## 14. Open Questions

| ID | Topic | Question | Status |
|---|---|---|---|
| **R118** | Where witnesses live | Phase 7.4's witnesses are a Rust table. Under Part VIII they are strands with lineage that travel with the parent. Her own counts become the witnesses for the next evolution (joins R116) | open |
| **R119** | Evolution that changes the kind | Answered by D23: a fraction is built from whole numbers, so the step from one to the other comes with its own proof | **answered** |
| **R120** | Literal marks | How does a chain write a constant (the mark "sheep", the direction "up")? It must be written so the partner strand still reads the exact opposite (D19) | open |
| **R121** | Switch patterns | How long is a switch pattern, how does the platform name its signals, and can a switch respond to a *protein's* output (a cell switching on its neighbour) as well as to the device? | open |
| **R122** | The world pairs | Are sense/show, keep/recall, grant/revoke truly irreducible, or can one be derived from another (is storage just a device you sense and show to)? | open |
| **R123** | The partner of a world touch | For structure pairs, the partner protein is the undo. For world pairs, it's a mirror. Is the mirror useful, or should undo of a world touch be refused? | open |
| **R124** | One strand at every level | What does a body, a system or a universe look like as a strand? Where do links and forces sit in it? | open |
| **R125** | Copy and repair | When a strand is copied (shared, saved, loaded), is it checked against its partner every time, and is a changed letter refused, or repaired from the partner strand and reported? | open |
| **R126** | Speed of the reader | How slow is a count of tallies before fast alleles take over, and where is the line? | open |
| **R127** | Codex 2 | How would a new table ever be introduced without breaking every strand written under codex 1? | open |

---

## 15. Prior Art

| Idea | Source | Lesson |
|---|---|---|
| Four letters, three-letter codons, one near-universal table | The genetic code (Nirenberg, Khorana, Crick, 1960s) | §3–§4 |
| The third letter of a codon "wobbles" | Crick's wobble hypothesis (1966) | §4.4: nature's free letter; JoInn chose not to use one |
| The real code is unusually good at making errors harmless | Freeland & Hurst, "The genetic code is one in a million" (1998) | §6.4: nature needs synonyms because it copies by chemistry |
| Adapters (tRNAs) carry the table, not the DNA | Crick's adaptor hypothesis (1955) | §4.3: the table lives in the reader |
| Genes switched on by proteins binding near them | Jacob & Monod, the lac operon (1961) | §10: switches |
| Programs as chains; composing is putting one after another | Forth (Moore), Joy (von Thun), Factor | §7.1: how a chain runs |
| Running a program backwards; every step has an inverse | Reversible computing (Bennett, 1973; the Janus language) | §6.3: the partner strand as undo |
| A number is "do something n times" | Peano; Church numerals | §7.3: counts as chains of tallies |
| "God made the integers; all else is the work of man" | Kronecker | Only the bottom is base |

---

## 16. Glossary

| Term | Definition |
|---|---|
| **Letter** | A, T, G or C. One of four; means nothing alone |
| **Strand** | A row of letters with a head and a tail |
| **Partner strand** | Each letter replaced by its partner (A↔T, G↔C), running the other way |
| **Codon** | Three letters read together; 64 in all |
| **Code table** | The one table, held by the reader, that turns each codon (all three letters) into an amino acid, a start or a stop. Its version is the `codex` |
| **Amino acid** | One of the twelve basic operations; irreducible or a touch on the world; always one of an opposite pair |
| **Mark** | A label `build` puts on a thing, saying what it is; `case` reads it. A mark is never a unit of counting |
| **Tally** | One unit of a count; 3 is three tallies |
| **Reserved codon** | A codon naming nothing in this codex; reading one is a refusal |
| **Protein** | A chain of amino acids from start to stop. Its shape (ports) is derived |
| **Reader** | The platform part that reads strands through the table and runs proteins (nature's ribosome) |
| **Switch** | A stretch of strand before a protein that makes it when a matching signal is present |
| **Laws** | JoInn's physics: letters, table, reader, forces, opposition, exactness, the device's senses |
| **Stranger's kind test** | A new mechanism must work on a second, unlike kind written only in DNA, with no Rust change |

---

## 17. Start Here (for the next planning chat)

**Where things stand, 7 Oct 2026.**

- **Phase 7.4 is accepted** (`docs/Findings/phase-7.4-review.md`). AJ's window check and the CI run on `16e2766` are still owed. The review's fixes: the `gates.lock` fix carries forward; the evolution plant is folded into the rebuild.
- **Phase 7.5 is paused.** AJ decided JoInn must build its DNA from the beginning: four letters, codons, amino acids, proteins, then cells.
- **Decided today:** every decision except D18, D25, D26 and D28, which are still proposed.

**The order to work in:**

1. ~~AJ settles D16, D17, D19~~ Done 7 Oct. AJ settles the remaining proposals (D18, D25, D26, D28).
2. The roadmap is redrawn from Part VIII.
3. The first Part VIII phase is planned: the letters, the table, the reader, the 3D strand view, and the `gates.lock` fix.

**Documents to read with this one:** Part V (*Cells, Bodies and Forces*), Part VII (*Systems, Forces and Growth*), *The Floor*, the *Phase 7.4 Review*, and the *Research Backlog*.

---

*JoInn Architecture and Theory, Part VIII. Draft 0.1, 7 Oct 2026, from AJ's conversation with Claude after the Phase 7.4 review. AJ said: we keep hard-coding instead of using DNA and composition; add is add, whether a number, a string or a sheep; there are two sets, the living and the nonliving; mimic DNA's four letters, A, T, G, C, and don't skip the codon and amino acid layers; switches will solve device problems; build it as a 3D visual builder at the chain layer; an app is simpler than an octopus, so start at the beginning and get the ability to build the cell, so users become the creators of their universe. Claude proposed the twelve amino acids, the table layout, and the partner strand as the opposite protein, and checked the table by program. Draft 0.2 (same day): AJ accepted D3 (engineering knowledge is DNA) and D5 (a letter alone means nothing). Draft 0.3: AJ accepted D6, D8, D10–D13, D15–D17, D19–D24 and rejected none; §6.1 now says where the middle letter matters, and a count is a chain of tallies (a mark is a label, never a unit). Draft 0.4: AJ revised D13 and D17: all three letters carry meaning (64 possible, no synonyms), because two letters cap the table at 16 and JoInn is already at 14.*
