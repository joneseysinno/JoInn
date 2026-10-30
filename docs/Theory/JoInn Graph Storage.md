# JoInn Graph Storage

**How a hypernode is stored so the graph stays canonical, opposed, and free of its picture**

*Draft · 30 October 2026*

> **Status.** Draft, 30 Oct 2026. AJ’s direction from the vision conversation. This paper records storage properties already demonstrated by Phase 8 and by Part VIII ([JoInn DNA, Codons and Amino Acids](JoInn%20DNA,%20Codons%20and%20Amino%20Acids.md)). It claims no new implementation.
>
> The product story lives in the [guides](../Guides/01-what-is-joinn.md). The work that follows is the [Hypernode Refactor Plan](../Plans/JoInn%20Hypernode%20Refactor%20Plan.md). Evidence: [phase-8-strand-and-tower.md](../Findings/phase-8-strand-and-tower.md).

DNA research is how JoInn stores a hypernode. The stored graph stays canonical, opposed, and free of its picture. A person builds on blocks and relationships. The letters are the storage.

Phase 8 checked the properties below. Where a sentence is a Part VIII decision about what may be added later, the sentence says so. The reader lists names and the picture tints by them. Running those names as the operations the live engine performs is later work.

---

## 1. The alphabet stores a choice

A letter is two bits. A = 00, T = 01, G = 10, C = 11. Four letters pack to a byte. A letter alone has no meaning.

Meaning lives in the reader table, the codex. A strand file names its codex (`codex 1`) and then stores letters. Codex 1 assigns meanings in `joinn-codex`. Gate 8’s reader line is: codex 1 names 14 codons, one per meaning.

Phase 8 checked this by reading the witnesses back byte for byte, and by swapping two codon meanings (`AAA` and `TTT`). The swapped reader no longer matches the witness.

## 2. One spelling, and the hash is identity

Codex 1 gives each named meaning one codon. A strand’s identity is the `joinn.dna.v1` hash of its length and its packed bytes. A chromosome’s identity is that same hash: `twelve.chromosome` hashes as `twelve.strand`.

One spelling means the bytes are the canonical form. A second spelling of the same meaning is not in the table. Part VIII decided this (no synonyms). Phase 8 checked the one-meaning table and the hash.

## 3. The opposite is in the bytes

A letter’s partner flips the low bit: A with T, G with C. The partner strand is every letter’s partner, in reverse order. The reader yields the opposite chain, in reverse order. `twelve.strand` reads `pair split build case refer return sense show keep recall grant revoke`. Its partner reads the opposites in reverse.

A reserved codon is a refusal. `reserved.strand` fails at `TAA`: the codon names nothing in codex 1, and the acceptance is a codon the codex names. The reader does not guess.

Phase 8 checked the partner reading on the strand witnesses, and checked that every codon and its partner differ in parity, 64 of 64.

## 4. Absence takes no stored null

Nothing is the reader’s silence.

The empty chain is start, then stop. `empty.strand` is `AAT ATT`, and the protein at letter 0 reads as nothing. The partner reads nothing as well. The file stores the start and the stop. The chain between them is empty, and there is no null value in its place.

A silent slot is a segment past the last codon. It is not stored, and it is not a null. Twelve places 14 codons on 15 segments, and the one segment left over costs no bytes. The fold view draws that segment thin and grey. Phase 8 checked both the empty witness and the silent segment.

## 5. References are hashes

A gene is a hash reference: a node of the curve names another chromosome. The pool holds chromosomes and reads none of them until `expand`. Every expansion is counted.

A chromosome that contains itself through its genes is refused on entry. Gate 8’s third item is that refusal. The cycle witness fails at its second chromosome, and the message names the hash that contains itself.

Containment follows those references, and containment may not cycle. A relationship among hypernodes is a different join. The product rule is that relationships may cycle. Phase 8 checked the pool’s containment rule. It did not check cycles among relationships. The window’s hyperedges are joins, and that check is separate.

## 6. The picture is derived

A chromosome is laid on a Hilbert curve, codon k on segment k. Positions, directions, and levels come from the segment index. Nothing about the fold is stored. Layout is not identity: the hash is the strand hash.

The curve filter is a **level**. Keeping the segments at or above a level leaves exactly the curve one order down (fact T5). The Phase 8 findings call this filter a floor. This paper calls it a level, so the word floor stays with the sealed opposed operations in [the-floor.md](../Findings/the-floor.md).

The order is the Hilbert size: the smallest curve with enough segments. Twelve’s 14 codons need order 2, which has 15 segments. Gate 8 checks that every order holds the one before it, at orders 1 through 6 (facts T1–T7).

### Three joints at the top level

Fact T7: the top level of order n (level n − 1) has exactly three segments. They sit between the curve’s four quarters. The middle one is the centre of the curve, and it is one of the three.

This paper calls those three segments the **joints**. For twelve, the top level holds `build`, `sense`, and `grant`. The middle joint, `sense`, is the centre. `build` and `grant` are the other two joints.

The Phase 8 findings, after the correction of 12 Oct, call these three segments bridges, and they record that the middle segment is one of three. The beam’s bridge is a different thing: an underivable law read from a pinned edition. The joints are the curve’s three top-level segments.

## 7. The reader version stands apart from the codex

The reader prints `reader 8.0` beside `codex 1`. The version is the reader’s. The codex is the table a strand names. The Phase 8 record states that changing the reader leaves what a codex-1 strand means unchanged, and every witness still reads byte for byte.

## 8. Room in the reserved codons

Codex 1 names 14 codons and reserves the other 50. Reading a reserved codon is the refusal in §3.

Part VIII (D18) leaves those reserved codons as room for a later opposed pair, admitted when the meaning cannot be derived from what is already stored. That is the same spirit as the floor rule in [the-floor.md](../Findings/the-floor.md): a new primitive is admitted only when it is irreducible and opposed, after an attempt to write it from what already exists has failed. This paper does not restate that rule’s membership. Phase 8 checked the refusal. It did not add a new named codon.

## 9. The names in the table are stored names of blocks

Codex 1’s named entries, besides start and stop, are six opposed pairs:

| Stored name | Opposite |
|-------------|----------|
| pair | split |
| build | case |
| refer | return |
| sense | show |
| keep | recall |
| grant | revoke |

These are the stored names of blocks. The reader lists them. `cargo xtask read` prints the chain. The strand view tints each codon by its name and prints the name underneath. A click on codon 7 names that codon in either view.

The codex also prints a device list: screen (show), input (sense), store (keep, recall), clock (sense). Those lines are names in the reader. The devices are not built.

Running these names as the floor operations the live engine already performs is later work. The [refactor plan](../Plans/JoInn%20Hypernode%20Refactor%20Plan.md) schedules it (Phase G). A protein is a slice of names between start and stop. Phase 8’s reader produces that slice. It does not run the calculator.

## 10. Four storage words, mapped

| Storage word | What Phase 8 stores |
|--------------|---------------------|
| **Amino acid** | A named table entry in the codex |
| **Protein** | The chain of names between a start and a stop |
| **Gene** | A hash reference, read on expand |
| **Chromosome** | A strand plus those references |

The picture, the labels under the codons, and the Hilbert layout stay out of the hash. Identity is the letters.

---

## Where to go next

- Product language: [Guides](../Guides/README.md), especially [what JoInn is](../Guides/01-what-is-joinn.md) and the [glossary](../Guides/05-glossary.md)
- What already runs: [Where we are](../Guides/03-where-we-are.md), Phase 8 section
- Next build: [Hypernode Refactor Plan](../Plans/JoInn%20Hypernode%20Refactor%20Plan.md)
- Decision record for the letters: [JoInn DNA, Codons and Amino Acids](JoInn%20DNA,%20Codons%20and%20Amino%20Acids.md)
- Measurements: [phase-8-strand-and-tower.md](../Findings/phase-8-strand-and-tower.md)
