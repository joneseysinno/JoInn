# JoInn Coding Region Grammar

**P0-02 and P0-03 · Phase 1 reads this file**

Author: AJ · Draft 0.1 · September 17, 2026  
Status: **DECIDED** for Phase 1 (codex `1`)

This is the artifact Phase 1 parses, prints, canonicalizes and hashes. It is not a suggestion. A change here is a `codex` increment and a corpus rehash.

---

## 1. File shape

One file per cell. Encoding UTF-8, NFC. Source form may use CRLF or LF; canonical form is LF only.

Two regions in one file, separated by a hard delimiter line that is exactly three ASCII hyphens:

```
coding { … }

---

regulatory { … }

alleles { … }
```

The delimiter, the regulatory region and the alleles block are **not** part of the coding region and are **not** hashed. A creator must be able to see the split at a glance.

Comments: `#` to end of line, allowed in the source form only. They are stripped by canonicalization and **cannot be round-tripped**. `parse ∘ print` is identity on the *coding-region AST*, not on source bytes.

---

## 2. What the grammar can say

### 2.1 Frame reference

A name plus a version. Phase 1 names: `Text`, `ℤ`, `ℚ`. Source aliases `Z` → `ℤ`, `Q` → `ℚ`. Unknown names are a parse refusal.

Canonical: `ℤ 1`

### 2.2 Contract

Ordered port declarations `(position, direction, frame, required)`, a `join_policy`, and `retired_positions`.

```
port <position> <in|out> <frame> <required|optional>
join <refuse|latest|queue>
retired <position>*
require <position>: <formula>
ensure <position>: <formula>
```

Positions are unsigned decimal integers, no leading zeros (the position `0` is the exception). A retired position must not also be declared as a live port.

### 2.3 Require and ensure

Predicates attached to a port, in the formula language. Inside these formulas the reserved variable `port` denotes the value at that port.

### 2.4 Laws

Named, universally quantified formulas. Names are coding-region content (they appear in refusals). Port references inside laws are by **position**, never by display name.

### 2.5 Formula and term language

Exactly these term atoms:

| Atom | Source form | Meaning |
|---|---|---|
| variable | `a` | bound by `forall` |
| literal | `ℤ 1 5`, `Text 1 "a"`, `ℚ 1 2/3` | a frame-tagged canonicalizable value |
| frame op | `ℤ 1.zero`, `ℤ 1.succ(a)` | an operation from the named frame's signature |
| self at out-port | `self@2(0: a, 1: b)` | the cell under definition, read at that out-port |
| other cell at out-port | `cell:<64-hex>@1(0: n)` | another coding region, **by hash** |

Connectives: `=`, `¬` / `not`, `∧` / `and`, `∨` / `or`, `→` / `implies`, `∀` / `forall`.

Nothing else. No arithmetic sugar, no `let`, no recursion, no existentials, no primitive names (`int.add`, `text.parse_int`, …). A source file that writes a primitive name in the coding region is a **parse refusal**.

`self@N(...)` names the out-port position `N`. Arguments are a map from in-port position to a term.

### 2.6 Founding witnesses

Tuples of input values by port position to expected output values by port position:

```
witness {
  in 0: ℤ 1 2
  in 1: ℤ 1 3
  out 2: ℤ 1 5
}
```

Multi-out-port cells are representable. Phase 1 cells have one out-port.

### 2.7 Declarations

A list. Phase 1: must be empty. A non-empty list is a gate refusal with reason `declarations are not implemented`.

### 2.8 Lineage

Parent hash, zero or one: `lineage none` or `lineage <64-hex>`.

### 2.8.1 Turn (optional, Phase 2, `codex` stays `1`)

A coding region may declare `turn <out-position> from {<positions>}`. The block
prints only when present. A cell with no `turn` block prints byte-for-byte as it
did under Phase 1, and therefore hashes as it did. Absence means the same thing
in both formats: this cell declares no turn.

Canonical:

```
turn
0 from {1 2}
```

`turn` is identity: it is hashed. Adding a turn to an existing cell is an
evolution event (declare `lineage`).

### 2.9 Regulatory region

Display names, literals, prompts, styles, allele-selection rules. Never hashed.

```
regulatory {
  names { 0 a  1 b  2 sum }
  literals { prompt "a: " }
  styles { }
}
```

### 2.10 Alleles

Each allele: its frame, its body (`native "<registered name>"` in Phase 1), its own witness corpus. Alleles are payload, hashed on their own under `joinn.allele.v1`, never mixed into the cell hash.

---

## 3. What the grammar cannot say

- An allele inside the coding region.
- A primitive name anywhere in the coding region.
- A display name inside the coding region.
- A law that names an allele.

---

## 4. Decisions forced by the grammar

| # | Question | Decision |
|---|---|---|
| 1 | One file per cell, or per genome? | One file per cell. |
| 2 | Two files, two sections, or a delimiter? | One file, two sections, delimiter `---`. |
| 3 | Comments? | Source only; stripped; not round-tripped. |
| 4 | Unicode identifiers and frame names? | Yes. NFC mandatory on parse. |
| 5 | Integer literals | Canonical: optional ASCII minus, no plus, no underscores, no leading zeros except `0`, decimal only. Source may write `+5`; it canonicalizes to `5`. `"007"` is not an integer literal. |
| 6 | Are laws named? | Yes. Names are hashed. |
| 7 | Port reference in a law | Position. |
| 8 | Cell reference | Full 64-hex in canonical form. Source may write a shorter hex prefix only if a resolver is supplied; otherwise it is a parse refusal. |
| 9 | Multi-out-port witnesses | `in` / `out` maps by position. |
| 10 | Line endings, encoding, line length | UTF-8 NFC, LF in canonical form, no trailing whitespace, exactly one trailing newline, no maximum line length. |

---

## 5. Canonical form (P0-03)

The hash is taken over canonical text:

```
hash(object) = BLAKE3-256(  len_le(tag) ‖ tag ‖ len_le(bytes) ‖ bytes )
```

Length prefixes are `u64` little-endian. Bare concatenation of variable-length pieces is forbidden.

### 5.1 Domain tags

| Object | Tag |
|---|---|
| coding region (the cell's identity) | `joinn.cell.v1` |
| allele payload | `joinn.allele.v1` |
| a single witness, when hashed alone | `joinn.witness.v1` |
| body coding region | `joinn.body.v1` |

Output length: 32 bytes. Display in prose as the first four hex characters (`c19e`); **never truncate in storage**.

### 5.2 Member ordering

- Top-level coding fields: alphabetical by ASCII/UTF-8 byte order of the field keyword: `codex`, `contract`, `declarations`, `founding`, `frame`, `laws`, `lineage`, and optionally `turn`.
- Ports: by position.
- Retired positions: ascending.
- Require / ensure: by port position.
- Laws: by law name, byte-wise over NFC UTF-8.
- Witnesses: by the total order on their `(in-map, out-map)` — keys ascending, values by `Value`'s `Ord` (frame, then term).
- SelfAt / CellAt argument maps: by port position.
- `forall` binders: by variable name, byte-wise.
- Everything else: alphabetically, byte-wise over NFC UTF-8. No locale collation.

### 5.3 What is stripped

Comments, display names, all whitespace not required by this canonical form, the entire regulatory region, the alleles block, CRLF → LF, trailing spaces, extra blank lines.

### 5.4 Literal printing

Each frame owns the canonical printing of its own values. The DNA canonicalizer calls `Frame::print`. It does not format integers itself.

- `ℤ`: decimal, optional minus, no plus, no leading zeros except zero itself.
- `Text`: double-quoted, NFC, escapes for `"`, `\`, CR, LF, TAB; other bytes as UTF-8.
- `ℚ`: `numer/denom` in lowest terms, positive denominator, numer may be minus.

### 5.5 Codex

`codex` is a `u16` stored in the coding region, hashed, and currently `1`. Incrementing it rehashes the corpus. Rehash-on-format-change is accepted for Draft 0.1; goldens are never regenerated automatically.

### 5.6 Canonical text skeleton

```
codex 1
contract
ensure
join refuse
port 0 in ℤ 1 required
port 1 in ℤ 1 required
port 2 out ℤ 1 required
require
retired
declarations
founding
witness
in 0: ℤ 1 2
in 1: ℤ 1 3
out 2: ℤ 1 5
frame ℤ 1
laws
associative: forall a:ℤ 1 b:ℤ 1 c:ℤ 1. self@2(0: self@2(0: a, 1: b), 1: c) = self@2(0: a, 1: self@2(0: b, 1: c))
commutative: forall a:ℤ 1 b:ℤ 1. self@2(0: a, 1: b) = self@2(0: b, 1: a)
identity: forall a:ℤ 1. self@2(0: a, 1: ℤ 1.zero) = a
lineage none
```

Exactly one trailing newline. No trailing spaces. Keywords are ASCII. Frame names keep their Unicode form, NFC.

---

## 6. Hash rule (the one sentence)

> **hash(cell) = BLAKE3-256 of the length-prefixed domain tag `joinn.cell.v1` and the canonical coding-region text bytes.**

The regulatory region cannot participate: it does not implement `Genotype`.
