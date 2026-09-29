# The Floor

**The primitives JoInn is written in · admitted by declaration, not by the gate**

Supersedes `minimal-primitives.md`. Amended September 18, 2026 — see §5.

---

> **29 Sep 2026 · re-read, not amended.** Part V's reading of the Floor as
> forces was tried by this document's own rule (R78). The attempts are recorded
> in §5. No pair left the Floor.

## 1. The rule

There is no target count. A count is an aesthetic; this is a test.

> **A primitive is admitted only if it is irreducible and opposed.**
>
> **Irreducible** — no body over the rest of the floor computes it. If one does, the candidate is not a primitive; it is a cell, and it belongs above the floor with everything else.
>
> **Opposed** — it names an opposite that is also in the floor, and that opposite names it back. Naming is symmetric or neither is admitted.

Two consequences follow, and both are checkable rather than aspirational.

**The floor's size is even.** Not by design — by arithmetic. If every member is one half of a symmetric pair, the count cannot be odd. So **an odd count is a proof that an opposition is missing**, and the pairing lint that reports it is not a style check; it is Law 1 applied to the floor's own membership. This is the honest version of what the old document called "minimal": the floor is not as small as we could make it, it is as small as opposition permits.

**Irreducibility is mechanically refutable.** You cannot prove a thing is irreducible, but you can refute it, and that is the direction that matters:

> **To petition for a primitive, first try to write it as a reference body over the existing floor. If `agree` succeeds, the petition is refused — the candidate is reducible and you have just written the cell that replaces it.**

The instrument already exists. This is the agreement harness pointed at the floor instead of at a seal, and it answers R33: the petition procedure is not paperwork, it is an attempted reduction that must fail.

## 2. Vocabulary

The floor is **the floor**. It is not "the minimal set," not "the minimal primitive set," and it is never named by its size — "the eleven," "the twelve," "the sixteen." A count in prose goes stale the first time an opposition is found, and a document that names its own size invites the reader to check the number instead of the rule. The count lives in one place, `xtask vocab`, which derives it and asserts it is even.

A **register** classifies a pair by what kind of thing it is. It does not affect membership.

| Register | What it is | Who may call it |
|---|---|---|
| **matter** | evaluable, pure, frame-parametric | any allele |
| **space** | body construction — grammar, not functions | nothing calls it; a body *is* it |
| **physics** | services the body bus offers | the engine only; never an allele |

## 3. The floor

Eight pairs. Each entry states the pair, its opposition law, and **why it cannot be built from the others** — the irreducibility argument is the admission, so it is written down rather than assumed.

### matter

#### `build` / `case`

`build(frame, tag, parts)` makes a value at the frame's `tag`-th declared constructor. `case(value)` returns the tag and the parts it was built from, with tag `0` meaning "this is a generator."

> **Opposition:** `case(build(f, t, p)) = (t, p)` and `build(f, case(v)) = v`. This is FO8 and FO9, already checked in the conformance harness.

**Irreducible.** `build` is the only way to produce a frame value that was not given as a literal; `case` is the only way to reach inside one. Nothing else comes close: `eq` can test a value against a literal but cannot make a new one or recover a part, and `pair`/`split` move values without entering a frame at all.

**What this replaces.** `zero`, `succ` and `pred` are *not* floor primitives. They are ℤ's first three constructors — `build(ℤ,0,[])`, `build(ℤ,1,[x])`, `build(ℤ,2,[x])` — and a thing constructible from `build` is by the rule not a primitive. They remain **frame vocabulary**: nameable in a law under §2.3 of the Phase 0–1 plan, and part of ℤ's signature. The floor stopped naming them; the frame did not.

This is why the floor is frame-parametric and why it is a *floor* at all. A floor with `succ` in it privileges one frame and has nothing to say about `Text` or `ℚ`.

#### `pair` / `split`

The only product. `pair(x, y)` makes one value of two; `split` recovers both.

> **Opposition:** `split(pair(x, y)) = (x, y)`.

**Irreducible**, and the argument is worth stating precisely because `build` looks like it should subsume this: `build` makes a value **in a frame**, and it must be told which. **A pair is the only value that belongs to no frame.** It is how two values cross one wire. Ask which frame a pair of a ℤ and a `Text` lives in and the question has no answer, which is the proof.

*Consequence, recorded rather than hidden:* `Value` is frame-tagged today. A frameless product is a change to the value model, and it is **R38**.

#### `eq` / `choose`

`eq(a, b)` makes a distinction. `choose(d, a, b)` spends one.

> **Opposition:** a decision made and a decision acted on. The stated law is `choose(eq(x, x), a, b) = a`.

**Irreducible, with a proof for once.** Any structural equality written over `case` must descend both values and compare the tags it recovers — and comparing tags *is* equality. **`eq` cannot be constructed without `eq`.** And `choose` is the only way for a value to depend on a decision; without it nothing a body produces can vary with a test.

**Why `case`'s tag is not a second way to branch.** A tag is a value like any other, and the only thing done with it is compare it: `eq(tag, 0)`. `case` recovers structure; `eq` decides; `choose` acts. Three jobs, and the middle one has exactly one owner. A floor with two independent ways to branch would have two places where Law 1 lives, and they would eventually disagree.

### space

#### `bound` / `fill`

`bound` declares a membrane — an inside, an outside, and holes. `fill` occupies a declared hole.

> **Opposition:** `fill` occupies what `bound` declared.

**Irreducible.** Nothing evaluable declares a boundary. A cell *is* a `bound` with its ports as the holes; this is grammar, and there is no `fn bound(…)` anywhere in the workspace.

#### `bind` / `unbind`

Incidence. `bind` attaches a port to a port; `unbind` severs.

> **Opposition:** `unbind(bind(p, q))` restores the prior wiring.

**Irreducible.** `bound` declares boundaries; only `bind` relates two of them. A body file is `bound` and `bind` applied, which is what the `.body` grammar means.

### physics

#### `hash` / `resolve`

`hash(v)` is content address: value → identity. `resolve(h)` is identity → value, and it is **partial** — it refuses when the store does not hold it.

> **Opposition:** `resolve(hash(v)) = v` when the store holds `v`, and a refusal otherwise.

**Irreducible.** Content addressing cannot be computed from the rest of the floor, and `resolve` cannot either — it is a lookup, not a calculation.

**This restates R32's hole more truly.** The old document called `hash` one-way and let it stand as the floor's single exception. That was never quite right: `hash` has an opposite and it is used constantly — `resolve` is what turns `cell:6b32…` in a genome into a cell. The honest statement is not *hash has no inverse*; it is:

> **`hash` is total and `resolve` is partial, and the gap between them is the store.**

One-wayness is a property of the *store's* contents, not of the operation. That is a narrower and more useful hole than the one R32 was written against, and it is where the trust actually sits.

#### `grant` / `revoke`

Capability movement. Whoever holds the grant may use the port.

> **Opposition:** `revoke` returns what `grant` moved.

**Irreducible.** No evaluable primitive can move authority; if one could, an allele could widen its own capabilities.

#### `join` / `fan`

`join` fires when the required in-ports hold messages. `fan` sends one message to many.

> **Opposition:** `fan` splits what `join` gathered.

**Irreducible.** Message arity at a membrane is not computable from inside it.

## 4. What the floor is not

**It is not gated.** `build` has no coding region to be judged against, because `build` is part of what a coding region is written in. Judging it would mean judging it against laws stated in terms of itself. **The floor is a declared axiom set, and this is where the trust is.**

What it gets instead of a gate is the thing a gate cannot give it: **each pair's opposition ships as an executable property test**, run against values drawn from the frames' own generators. Those tests are Law 1 at the bottom of the stack, and they are the only thing standing under it.

Nothing else is trusted this way. Everything above the floor is refused until it earns admission.

## 5. Amendment record

Every change to the floor is dated here, with its reason and its refuted reduction.

### 2026-09-18 · `build` / `case` admitted; `zero`, `succ`, `pred` removed; `resolve` admitted

**Why.** Phase 2 discovered that the floor could not write its own reference alleles. ℤ's constructors could build a value and `eq` could compare one against a literal, but nothing could take a value apart, so `add`'s recursion could not ask which shape its argument had. §2.1 of the Phase 2 plan named it exactly: *"constructors without a destructor is opposition missing at the bottom of the stack."* The eliminator was added to the `Frame` trait as a Rust method, which Rust could call and a body could not — and the reference alleles were written in Rust instead. That is F1 of `phase-2-review.md`, and this amendment is the correction.

**What changed.**

| | Before | After |
|---|---|---|
| Construction | `zero`, `succ`, `pred` — ℤ only | `build(frame, tag, parts)` — any frame |
| Destruction | *(absent)* | `case(value)` |
| `hash` | one-way, the stated exception | opposed by `resolve`, partial |
| `eq`'s opposite | itself | `choose` |
| `choose`'s opposite | itself | `eq` |
| The rule | a frozen count | irreducible **and** opposed |

**The refuted reductions.** `zero`, `succ` and `pred` were reduced: each is `build` at a fixed tag, so each failed the irreducibility clause and left the floor for ℤ's signature. `build`, `case` and `resolve` were attempted as bodies over the remaining floor and could not be written, which is the admission.

**What this cost.** `Frame::constructors()` is now ordered and that order is part of the frame's identity, because `build(ℤ, 1, [x])` means "successor" only while ℤ's constructor list says so. Appending a constructor stays conservative; reordering or removing one moves the frame version. That is **R34**.

**Standing rule from here.** The floor grows or shrinks only through an entry in this section, with a date, a reason, and a refuted reduction. A change with no refuted reduction is not an amendment; it is a convenience.

### 2026-09-29 · R78 attempted: the Floor re-read as forces · no amendment

**Why.** Part V (*Cells, Bodies and Forces*) proposed that the Floor sorts into
two opposed forces (combine / separate, carry / release), distinction, and the
membrane. The rule here says nothing changes until each reduction is tried.

**What was tried, and how it came out.**

| Pair | Tried as | Result |
|---|---|---|
| `build` / `case` | combine / separate in a frame | A regrouping, not a reduction. Stays |
| `pair` / `split` | combine / separate with no frame | Reduces only if a frameless combine exists (R38). Not attempted in code. Stays (R86) |
| `join` / `fan` | combine's completion rule; carry to many | `join` reads as when a combine completes; `fan` as carry of a value that is copied, not used up. Not attempted in code until carry exists. Stays (R86) |
| `eq` / `choose` | distinction; carry one, release the other | **Kept as a pair (AJ, 29 Sep).** `choose` is where distinction meets a force: a decision made and a decision acted on. Taking `choose` away would leave `eq` unopposed and the count odd |
| `grant` / `revoke` | carry / release of authority | Does not fit: release leaves a thing where it is, revoke brings it back. Stays as authority |
| `bind` / `unbind` | moves to systems | Stays in the Floor, in the system's register: systems are written in the Floor too |
| `bound` / `fill` | the membrane | Stays |
| `hash` / `resolve` | distinction by identity, and the store | Stays |

**Contact needs no new pair.** Cells that fill one boundary with no `bind`
between them are in contact. Phase 7's `.contact` grammar has no way to write a
`bind`, which is the test of this.

**Forces are opposed at the frame, not in the Floor.** Combine on ℤ is admitted
only with its separate, a turn of its response (Phase 7, `joinn-prim::forces`).
Separate for a lossy combine like sum is a turn given one part back; for a
lossless combine like `build` it is `case` (R82).

Standing rule unchanged: the Floor grows or shrinks only through an entry here
with a refuted reduction.
