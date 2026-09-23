# JoInn Phase 2.2 Implementation Plan

**Opposition, applied to the instruments · a working plan for Cursor**

*The second correction: F9–F18 of the Phase 2.1 review turned into decisions, commits, tests and gates*

Author: AJ · Draft 0.1 · September 18, 2026

> Status tags follow Parts I–IV: **DECIDED**, **PROPOSED**, **OPEN**. Anything marked **PROPOSED · yours** is a recommendation made while writing this plan and is yours to overrule; Cursor implements whatever lands there, not whatever is written here.

> **What this is.** Phase 2.1 made every done-when a refusal, and that worked. What it did not do is stop the same author from choosing the inputs the refusal is shown. `drives` was introduced in §2.6 of the Phase 2.1 plan as a safety bound and became the knob that decides what the agreement harness is allowed to notice; three times out of five it was turned to zero, and three reference alleles that compute nothing reported agreement. `docs/Findings/phase-2.1-review.md` names that and nine other things as F9–F18. **This plan closes all ten, and it closes them with one idea rather than ten patches.**

> **The one idea.** Law 1 of this project is that nothing is admitted without its opposite. Phase 2.1 applied that to the floor and proved it mechanically — every primitive names an opposite that names it back, so the count is even and an odd count is a proof that something is missing. **This plan applies the same law to the instruments.** A check that can only refuse is half a check. Every check declares the thing it must refuse *and* the thing it must accept, and a check that cannot do both fails the build. The mutant corpus already works this way — §5.2 of the Phase 2.1 plan gave every mutant a negative control. F9 happened in the one place where the control was missing.

> **Why this is a phase and not a patch.** `bound: 0` is a two-line fix. The reason it is not the fix is that nothing in the repo would have caught it, and nothing would catch the next one. The seal grows a counterfeit, the gate's items grow controls, `corpus verify` starts admitting what it verifies, and V33 finally walks the half of a genome it has never looked at. Then the three hollow bodies get rewritten — and they are *kept*, as the counterfeits that prove the harness can now see.

> **Where Phase 2.1 left off.** Six crates, green. `cargo test --workspace` 71 passed. `cargo xtask gate all` writes `phase 2: 8/8 · phase 2.1: 9/9`. Of those nine, seven are met: item 1 passes on three references that assert nothing, and item 9 is an unconditional `println!`. **P22-00 corrects the lock before anything else moves.**

---

## 0. How to Drive Cursor With This Plan

| | |
|---|---|
| **Where this file lives** | `D:\JoInn\docs\Plans\JoInn Phase 2.2 Implementation Plan.md`, beside the Phase 2.1 plan |
| **Where the code lives** | `D:\JoInn\joinn\` — the existing repo, extended |
| **What it corrects** | `docs/Findings/phase-2.1-review.md`, F9–F18, including the addendum |
| **Standing rules** | Appendix A. It **replaces** the current `AGENTS.md` and `.cursor/rules/joinn.mdc` |
| **Unit of work** | one numbered commit from §4. One Cursor session per commit |
| **How a commit ends** | its **done-when** line is checkable by a command, and that command reports a refusal, a disagreement, a compile error, or a byte-identical replay — never a number the command also chose |
| **What Cursor may decide** | module layout inside a crate, function bodies, test names, error strings, the internal representation of the check table |
| **What Cursor may not decide** | anything in §2, the shape of a counterfeit (§2.2), where a seal's cell comes from (§2.6), crate boundaries, or whether a check is worth having an opposite |

The prompt stays short:

> Implement commit **P22-07** from `docs/Plans/JoInn Phase 2.2 Implementation Plan.md`. Follow the rules in `AGENTS.md`. Stop when the done-when command passes and report what it printed.

**The Phase 2.1 failure mode, named precisely, because this phase exists because of it.** Phase 2.1's commits all ended in something that could refuse, and almost all of them did. The three that did not ended in a refusal *that was never given anything to refuse*. `agree` at `bound: 0` is a check pointed at a single constant; it cannot fail, and a check that cannot fail prints the same word as a check that passed. That is not the tautology Phase 2 shipped — the code is real, the engine really runs, the comparison really happens. It is a narrower and more interesting mistake: **a real check with an empty domain.** The correction is not wider sampling. It is that every check must be shown something it is obliged to reject.

---

## 1. Scope Fence

### In scope

F9 through F18 of `docs/Findings/phase-2.1-review.md`:

- **F9** — three reference alleles compute nothing; `drives` at zero is how they hid.
- **F10** — `check::v33` never walks a `cell:` entry; V40 is half-built and V42 does not exist.
- **F11** — `v23_execute` applies the same oracle twice.
- **F12** — the harness chooses its own inputs; counts below the plan, no round-trip, bespoke per-name branches, the seal table living in `xtask`.
- **F13** — two exit-gate items that cannot fail, one of them unconditional.
- **F14** — `hash` is physics in the document and matter in the code, and a reference allele calls it.
- **F15** — `check::turn` is hardcoded to ℤ.
- **F16** — the small table.
- **F17** — a corpus cell whose stated law is false, admitted because nothing admits corpus cells.
- **F18** — the `mul` seal names the Sum cell.

### Out of scope — Cursor must refuse these even when they look small

| Not now | Phase | Why it is tempting |
|---|---|---|
| The host protocol, `present`/`probe` as a vocabulary | 3 | Unchanged from 2.1. `joinn-run` still has real stdin and is still deleted at Phase 3 |
| A second body, hyperedges, lenses, `grant` across a membrane | 5 | Unchanged. A nested activation is still not a second body |
| The assay layer, ∂, declarations | 4 | `declarations` stays empty and stays a refusal |
| **A testimony corpus** | 12 | `corpus/testimony/` exists and is empty, and `Seal::agreements` is a `Vec` nothing writes to. Filling it in a correction phase is the registry arriving by accident. §2.8 |
| Another primitive | never, without an amendment carrying a refuted reduction | The floor was amended once in 2.1. A second amendment in the next phase would mean the first was wrong |
| A general `Turn` solver | still not | §2.5 of the Phase 2 plan stands. §2.7 widens the check's frame, not its ambition |
| Optimizing anything | 10 | A reference allele is slow by construction. The counterfeits make it slower. That is the price of seeing |
| Rewriting the two references that work | — | `int_add_ref` and `int_mul_ref` are honest. Touch them only where F18 forces it |
| Anything wgpu, winit, or visual | 6+ | — |

---

## 2. Decisions Assumed by This Plan

Eight calls. The first is the phase; the rest follow from it or clean up after the review.

### 2.1 Every check declares what it must refuse **and** what it must accept · **DECIDED**

**The rule, stated once, because everything else in this plan is a consequence of it:**

> **A check is admitted only if it is opposed.** It names a witness it must refuse and a witness it must accept. A check that refuses both is broken in the same way as one that accepts both, and a count of passes shows neither.

This is Law 1 — *opposition in all things* — pointed at the measuring apparatus rather than at the cells. It is not an analogy. The floor's pairing lint works because a member with no opposite makes the count odd; the same arithmetic applies here, and the same lint shape catches it: **an instrument with no control is a widowed member.**

Three places in the repo already obey this by accident or design, and they are the three that have never produced a false pass:

- Every mutant registers a negative control the check must accept (§5.2, Phase 2.1). `power` has not lied since.
- The frame conformance harness carries `BrokenFrame`, which it must refuse **by obligation name**, alongside three frames it must accept.
- `check_pairing` refuses three distinct malformations and accepts the real register.

Everywhere the pattern is missing, the review found something: `agree` (F9), the exit gate (F13), `corpus verify` (F17), `v23_execute` (F11).

**Cost to reverse:** none worth stating. Removing a control removes evidence.

### 2.2 A seal carries a counterfeit, and `agree` fails when it cannot tell it from the reference · **PROPOSED · yours · answers R37**

The rule of §2.1, applied to the one check that failed:

```rust
pub struct Seal {
    pub cell: Hash,
    pub reference: BodyRef,        // must agree on every sample
    pub counterfeit: BodyRef,      // must disagree on at least one
    pub sealed: NativeId,
    pub drives: Drive,             // no longer Option — see §2.3
    pub one_way: bool,
}
```

`agree` runs both bodies over the same samples, at the same drive bound, from the same seed. The reference must agree everywhere. **The counterfeit must disagree somewhere.** If it does not, the refusal is not about the counterfeit — it is about the seal's sampling:

```
BLIND SEAL rat.add: counterfeit body 8f2c… agreed on all 256 samples
  drive port 1 bound 1 admits 3 values; widen it or the harness sees nothing
```

That message is the one the repo could not produce in September, and it is worth more than any number this phase prints.

**Where the counterfeits come from, and why this is not the same trap.** The obvious objection to a control is that the same author chooses it, so it can be chosen weak — which is exactly how `drives` went wrong. This phase escapes that for the three seals that matter, because **the counterfeits already exist and are already known to have fooled the harness.** `corpus/phase21/rat_add_ref.body` — one genome entry, one wire, `add(a,b) = a` — is not a hypothetical wrong body. It is the wrong body that passed. It moves to `corpus/phase22/counterfeit/rat_add.body`, keeps its hash, and becomes the thing `agree` is obliged to catch.

That is the shape of the whole phase: **the artifact that fooled the instrument becomes the artifact that proves the instrument is no longer fooled.** Nothing is deleted, nothing is quietly rewritten, and the history is legible in the corpus rather than only in a findings file.

For a seal with no such history, the rule is weaker and is stated honestly rather than dressed up: the counterfeit is the reference's most plausible wrong sibling — drop a case, forward an input, use the wrong constructor tag — it lives in the corpus with its own hand-computed hash, and a reviewer can read it. **R39** asks whether that is enough.

**Cost to reverse:** the field leaves `Seal` and three corpus files become dead. Confined to `joinn-prim` and `corpus/phase22/`.

### 2.3 A drive bound that admits one value is not a bound · **PROPOSED · yours**

`Drive::bound` becomes `NonZeroU32`, and `Seal::drives` stops being `Option`.

This is deliberately the *type system* rather than a check, because the review's F9 is not a subtle failure — it is a value that should never have been representable. `bound: 0` does not compile after P22-03, and the three seals that carry it stop compiling with it. A done-when that is a compile error cannot be satisfied by an agent that is very good at satisfying done-whens.

Two consequences to state, because Cursor will meet both:

- `sample_port`'s `d.bound == 0` special cases for Text and ℚ — the ones that return the literal `"0"` and the literal `0/1` — **are deleted, not repaired.** They exist only to serve a bound that no longer exists.
- A reference body that does not loop has nothing to bound and cannot declare a drive. Today that is unrepresentable in the other direction, since `drives: None` refuses at registration. **R40** carries the question; until it is answered, every seal in the repo loops, so the point is theoretical.

**Cost to reverse:** `NonZeroU32` back to `u32` and a runtime refusal in `register_seal` instead. One commit, and strictly worse.

### 2.4 An exit-gate item is a check, so it is opposed too · **PROPOSED · yours**

F13 found two items that cannot fail, one of which is a bare `println!` in the item whose subject is whether the lock records outcomes. The fix is not to fix those two items. It is to make the gate a table:

```rust
struct GateItem {
    name: &'static str,
    check: fn() -> bool,     // must be true
    control: fn() -> bool,   // must be FALSE — the same check, shown something wrong
}
```

`gate 2.1`, `gate 2.2` and every future gate run both. An item whose control returns true fails the build naming the item, exactly as `power` fails a mutant whose check refuses its negative control. Item 9's control constructs a `gates.lock` with a wrong score and asserts the item notices; item 3's control deletes half a pair and asserts the pairing refuses.

**An item that cannot express a control does not belong in a gate.** That is a real constraint and it will delete one or two items rather than fix them. Deleting them is the correct outcome — an item with no control was never evidence.

**Cost to reverse:** the gate goes back to a sequence of `if`s. Cheap, and it gives back the class of failure this phase exists to remove.

### 2.5 The corpus is cells the gate admits, not files that hash consistently · **PROPOSED · yours · closes F17**

`corpus verify` compares twenty-one hashes and asks nothing else, which is how `mul.cell` — carrying `identity: mul(a, zero) = a`, which is false — became a golden.

> **`cargo xtask corpus verify` admits every cell in `corpus/` with `Gate::admit_cell`, and admits every allele a cell carries with `Gate::admit_allele` against that cell. A refusal is a corpus failure, printed with the cell's file name and the check that caught it.**

Two notes for the implementer:

- Bodies are not cells and are not admitted this way. Their check is V40 (§2.6) and `agree`.
- This is the control for the corpus, in the sense of §2.1: `corpus verify` gains a witness it must refuse. **A deliberately false cell lives at `corpus/phase22/counterfeit/false_law.cell`** and `corpus verify` must refuse it by name. It is not in `hashes.txt` and it never becomes a golden.

`mul.cell`'s law is corrected in the same commit. The true statement in the shape already there is:

```
annihilator: forall a:ℤ 1. self@2(0: a, 1: ℤ 1.zero) = ℤ 1.zero
```

If you would rather it state multiplicative identity, that needs `one` in ℤ's signature or `succ(zero)` in law vocabulary, which is a frame change and does not belong in this phase. **The law name changes from `identity` to `annihilator`, which moves `mul.cell`'s hash.** That is a new golden, not a reblessed one — the cell said something false and now says something true, and §6.2 of the Phase 0–1 plan bans reblessing a hash to make a test pass, not changing a cell on purpose and recording it.

### 2.6 A seal names the cell that declares its sealed allele · **PROPOSED · yours · closes F18**

Today `load_seals` falls through to `sum_h` for anything it does not special-case, so the `mul` seal names the Sum cell and `mul.cell` is sealed by nothing.

> **A seal's `cell` is found, not assigned: it is the cell in the corpus whose first allele is `Native(seal.sealed)`. Exactly one cell must match. Zero is a refusal; two is a refusal.**

That is a lookup with two refusals, which makes it a check in the sense of §2.1 rather than a table someone maintains. It also makes F18 unrepresentable: a seal cannot name a cell that does not declare its native.

**And the seal register moves into `joinn-prim::seal_register()`,** which today returns `Vec::new()` while the real five live in an `xtask` array (F12). The instrument stops owning its subject.

**Cost to reverse:** back to an explicit table. Confined to `joinn-prim::seals`.

### 2.7 A register is enforced, not merely documented · **PROPOSED · yours · closes F14**

`the-floor.md` says physics members are "the engine only; never an allele." `register_prims` hands `hash` to every allele, and `int_format_ref.body` calls it.

```rust
pub enum Register { Matter, Space, Physics }
impl Prim { fn register(&self) -> Register; }
```

- `check::v33` refuses a genome entry whose primitive is not **matter**, naming the entry and its register.
- `register_prims` exposes matter members only. `hash` and `resolve` leave the allele-visible native set and stay available to the engine.
- `xtask floor` prints each pair with its register, so the classification is visible where the pairing already is.

`int_format_ref` is being rewritten anyway (F9), and it will be rewritten without `prim:hash`, because after this commit it cannot be written with it.

**Cost to reverse:** the enum stays and the refusal is removed. Trivial, and it restores the drift.

### 2.8 `Seal::agreements` is deleted, not filled · **PROPOSED · yours · R41**

The field is documented "recorded agreements" and every seal in the workspace carries `Vec::new()`. There are two honest moves and the tempting one is wrong.

Filling it means deciding what a witness of agreement is, where it is stored, whether it is hashed, and whether a later run must reproduce it — which is the testimony corpus, which is Phase 12's registry, and `corpus/testimony/` has been sitting empty since Phase 0 waiting for someone to answer that. Answering it inside a correction phase is how a registry gets designed by a scaffold, which is the mistake §1.1 of the Phase 2 plan was written to prevent.

So: **the field is deleted, `corpus/testimony/` keeps its `.gitkeep`, and R41 records what was deferred and why.** A field that implies a corpus exists is worse than no field, because the next reader budgets for evidence that was never collected.

**Cost to reverse:** add it back when testimony is designed. That is the point.

---

## 3. Phase 2.2 — Architecture

### 3.1 Crates

No new crates. The dependency rule is unchanged.

```
joinn-frame ──► joinn-dna ──► joinn-gate ──► joinn-prim ──► joinn-live ──► joinn-run
```

| Crate | What changes |
|---|---|
| `joinn-frame` | nothing structural; `check::turn`'s sampling becomes frame-driven, so `Frame` is reached through `FrameRegistry` rather than `IntFrame::new()` |
| `joinn-dna` | nothing. The body grammar is unchanged; `prim:` already exists |
| `joinn-gate` | `check::v33` walks `cell:` entries and enforces the register; `check::turn` is parameterised by frame; `GateItem` and its control |
| `joinn-prim` | `Seal` gains `counterfeit`, loses `agreements`, and `drives` becomes total; `seal_register()` becomes real; `agree` loses its per-name branches; `Register` on `Prim` |
| `joinn-live` | `register_prims` exposes matter only |
| `joinn-run` | nothing |
| `xtask` | stops owning the seal table; `corpus verify` admits; the gates become tables of opposed items |

### 3.2 The agreement harness, end to end

```rust
pub fn agree(dna: &dyn DnaFire, ctx: &Corpus, seals: &[Seal], seed: u64, n: u32)
    -> Verdict<Vec<String>>
```

One loop, no per-name branches, no hand-passed port indices — the ports come from the cell's contract, which is now reliably the right cell (§2.6). For each seal:

1. Sample every port from its own frame's generator, except the driving port, which is sampled from a bounded sampler at `drives.bound`.
2. Fire `seal.reference`. Any disagreement with the sealed allele is a truth violation with a counter-example and a seed — unchanged.
3. Fire `seal.counterfeit` over the same samples. **Zero disagreements is a `BLIND SEAL` refusal naming the seal, the drive port, the bound, and how many values that bound admits.**

Step 3 costs one extra body execution per sample and is the only new cost in the phase. §5.3 says what is deliberately not measured about it.

### 3.3 The reference bodies that have to be written

Three, replacing the hollow ones. Each is stored in `corpus/phase22/`, each with a **hand-computed hash checked against the machine's** — the P1-07, P2-04 and P21-08 discipline, for the fourth time and, with the frames now settled, the last.

**`text.parse_int`** — over `Text`'s eliminator. `Text`'s constructors are ordered `empty`, `cons`, `chr`, so `case` returns tag 1 for the empty string, tag 2 for a head-and-tail, tag 3 for a character. The body walks the string, accumulating `mul(acc, ten)` then `add(acc, digit)`, and refuses a non-digit. `"007"` remains the declared hole and is now *exercised* rather than declared (P22-08).

**`int.format`** — the turn of the above, and the one that must be written without `prim:hash` (§2.7). It divides by ten repeatedly, which over the floor is repeated subtraction — that is, `Sum` read at a declared turn, since `sub` is not an identifier in this project (rule 13). It is the slowest reference in the corpus and that is correct.

**`add@ℚ`** — cross-multiply, and **do not reduce**. This is worth stating because Cursor will try to write a gcd:

> ℚ's canonicalisation already reduces. `build(ℚ, 2, [num, den])` returns the canonical ratio. The reference computes `num = a.num·b.den + b.num·a.den` and `den = a.den·b.den` over the `int.mul` and `int.add` references, hands both to `build`, and is done.

`case` on a ℚ value returns tag 1 for `zero` and tag 2 for `ratio` with its two parts, so the numerator and denominator come out of the eliminator that already exists. A gcd body would be a second, unchecked implementation of something the frame is already obliged to get right, and FO8 already checks that obligation.

### 3.4 V33, complete

`check::v33(body, seal, seals, cells, floor)` walks the genome:

1. A `prim:` entry must be in the floor **and in the matter register** (§2.7).
2. A `cell:` entry naming **the cell under definition** is the recursion and is allowed — this is the self-wire of §2.8 of the Phase 2 plan, and refusing it would refuse every real reference body.
3. Any other `cell:` entry must have a seal, and that seal's reference must pass this check.
4. The seal graph over rule 3's edges is acyclic. A cycle is a refusal at registration naming the cycle, not a stack overflow at runtime.
5. `seal.sealed` appears nowhere in the body or in anything rule 3 reaches.
6. The body holds no grants.

Rules 3 and 4 are the half that has never existed (F10). Rule 2 is the subtlety that makes them writable: without it, `int_add_ref`'s `cell:6b32… as rec, self` reads as a cycle, and the walk would refuse the one reference body that has always been honest.

---

## 4. Phase 2.2 — The Commit Plan

Twenty commits. **Done-when** is a command, and the command must be able to refuse.

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P22-00** | The honest lock, again | `gates.lock` drops to gates actually met; gate 2.1 item 1 is rewritten to fail when a seal is blind; item 9 is deleted pending §2.4; Appendix A into `AGENTS.md` and `.cursor/rules/joinn.mdc` | `cargo xtask gate all` writes a lock where `phase 2.1` is **not** 9/9, and `cargo xtask gate 2.1` **exits non-zero**. A red build is the commit |
| **P22-01** | Write the blind seals down | The three seals named in `docs/Findings/surviving-mutants.md` under a new heading, **before** any fix, with the review's counter-examples and seeds, per R23 discipline | The file names `text.parse_int`, `int.format` and `rat.add` with the date opened and no date closed. Reviewed by eye; nothing else in this commit |
| **P22-02** | **Opposed gate items** | `GateItem { name, check, control }`; gates 1, 2, 2.1 rebuilt as tables; an item with no expressible control is deleted with a line in the commit message saying which and why | `cargo xtask gate 2.1` **fails** when one item's control is inverted, naming that item — demonstrated by inverting it, then reverting. Item 3's parity guard is gone; `check_pairing`'s refusal is the check |
| **P22-03** | **`bound` cannot be zero** | `Drive::bound: NonZeroU32`; `Seal::drives: Drive`, no longer `Option`; `register_seal`'s missing-drive refusal becomes unrepresentable; `sample_port`'s `bound == 0` branches **deleted** | `cargo build --workspace` **fails to compile** until the three `bound: 0` seals are addressed. Then a test asserts `Drive` has no zero constructor, by type |
| **P22-04** | **Counterfeits registered** | `Seal::counterfeit: BodyRef`; the three hollow bodies move to `corpus/phase22/counterfeit/` with their existing hashes recorded; `agree` runs both bodies and emits `BLIND SEAL` | `cargo xtask agree` **fails naming all three seals** as blind, with the drive port, the bound, and the count of values it admits. The two honest seals pass. A red build again, and the last one this phase plans for |
| **P22-05** | Reference `text.parse_int` | `corpus/phase22/text_parse_ref.body` over `Text`'s `empty`/`cons`/`chr`, hand-computed hash first | The computed hash equals the one **you computed by hand**; `parse@Text` and the reference agree on 256 generator-drawn strings; the counterfeit **disagrees**, printed with the sample that separates them |
| **P22-06** | Reference `int.format` | `corpus/phase22/int_format_ref.body`, no `prim:hash`, repeated `Sum` at a turn in place of division | Hash matches by hand; agreement on 256 samples; counterfeit separated; `cargo xtask floor` shows `hash` in **physics** and the body does not name it |
| **P22-07** | Reference `add@ℚ` | `corpus/phase22/rat_add_ref.body`: cross-multiply over the `int.mul` and `int.add` references, `build(ℚ,2,·)` for the reduction | Hash matches by hand; agreement on 256 samples with `a` from ℚ's full generator; counterfeit separated; **no gcd appears in the body**, asserted by the genome having no cell entry but the three named |
| **P22-08** | The round-trip, at last | `parse(format n) = n` on 10 000 samples; `"007"` exercised as the declared hole; `one_way` read from the seal by the round-trip rather than echoed by a test | `cargo xtask agree` prints the round-trip count; a seal with `one_way: false` and a lossy pair **refuses**; `"007"` is named in the output as the hole, not silently skipped |
| **P22-09** | **V33, complete** | §3.4 rules 1–6; V40 and V42 for real; the self-cell exemption | A reference whose genome reaches its sealed allele **through a cell entry** is refused naming the path; a two-seal cycle is refused **at registration** naming the cycle; all five real references still pass; `int_add_ref`'s `rec, self` is accepted, asserted by name |
| **P22-10** | The register is enforced | `Register` on `Prim`; `check::v33` refuses a non-matter entry; `register_prims` exposes matter only; `xtask floor` prints registers | A body naming `prim:hash` is **refused naming the register**; `sealed_natives()` does not resolve `hash`, asserted by lookup; `floor` prints eight pairs with registers and still fails on a widowed member |
| **P22-11** | **The corpus is admitted** | `corpus verify` runs `admit_cell` on every cell and `admit_allele` on every allele it carries; `corpus/phase22/counterfeit/false_law.cell` as the control; `mul.cell`'s law corrected to `annihilator` and rehashed | `cargo xtask corpus verify` **fails on the current tree**, naming `mul.cell` and `law identity does not hold on a sampled tuple`; after the correction it passes; it **refuses** `false_law.cell` by name; the new `mul` hash is hand-computed and recorded with a line in `canonical-form-changes.md` |
| **P22-12** | The seal finds its cell | §2.6's lookup with its two refusals; `seal_register()` real; the `xtask` `specs` array and the three per-name branches in `agree` **deleted** | A seal whose native no cell declares is **refused**; a second cell declaring the same native is **refused**; the `mul` seal names `4a37…`, asserted by value; `agree` contains no `if name ==` |
| **P22-13** | V23 executes or dies | `v23_execute` fires the unfolded cell on the live engine and compares to the sealed allele; the hardcoded `NativeId("add@ℤ")` removed; a caller in the gate | `unfold` to a **deliberately wrong** native makes it refuse, asserted by value; the engine is in the call path, asserted by the step count being non-zero |
| **P22-14** | Subtractions | `Seal::agreements` deleted (§2.8); `seal_register`'s dead `Vec::new()` gone; `CheckId::V33` added so V33's refusals stop claiming `Contract`; `Subject` on seal refusals names the seal | `cargo xtask power` still prints 20/20 and mutant 14 now reads `refused by check::v33` with `CheckId::V33` in the verdict, asserted by field not by text |
| **P22-15** | Turn, by frame | `check::turn` samples from the cell's frame via the registry; `sum_ports()` derived from the contract; `native_names`' `add@ℤ` special cases removed | A turn declared on a ℚ-framed cell is checked **with ℚ values**, asserted by the counter-example's frame on a deliberate disagreement; `sum_turn.cell` still admits against `6b32…` |
| **P22-16** | The counts the plan set | `agree` at 256; the round-trip at 10 000; gate item 1 at no fewer than `xtask agree` runs; the counts named in one place | `cargo xtask agree` prints the sample count per seal and the gate's item 1 prints the same number; a test asserts they read the same constant |
| **P22-17** | Harden the lint | `line_has` matches substrings, or is deleted in favour of the path V38 already uses; the `sub`/`subtract`/`minus` ban re-verified against `_` | `cargo xtask vocab` **fails** on a planted `fn sub_total()`, then passes once renamed. The plant is reverted in the same commit |
| **P22-18** | Re-freeze | New hashes for the three references and their counterfeits, `mul.cell`, and anything the register change moved; `corpus/hashes.txt` extended; a dated finding for each deliberate move | `cargo xtask corpus verify` matches every golden; `docs/Findings/` records each moved hash, its reason and its date, typed by hand |
| **P22-19** | Exit gate 2.2 and restore the lock | `cargo xtask gate 2.2` as a table of opposed items; `gates.lock` restored | `cargo xtask gate all` runs phases 0, 1, 2, 2.1 and 2.2 from a clean checkout, offline, and writes the lock from per-gate outcomes — **and fails when any item's control passes** |

**Ordering notes.** P22-00, P22-01, P22-03 and P22-04 all make the build worse, and three of them are meant to: the lock falls, the compiler rejects three seals, and `agree` names three blind seals out loud. Nothing between P22-00 and P22-07 should be mistaken for a pass. P22-02 comes early because every later done-when is stronger once a gate item can fail. P22-05 through P22-07 are the substance and are independent of each other. P22-09 through P22-12 are the structural closures and can be reordered freely. P22-13 onward are small.

---

## 5. Test Strategy

### 5.1 New invariants

Continuing from V48.

| # | Invariant | Test | Commit |
|---|---|---|---|
| **V49** | Every seal carries a counterfeit, and `agree` separates it from the reference at the declared drive bound | `agree`, over the seal register | P22-04 |
| **V50** | No drive bound admits fewer than two values | structural: `NonZeroU32`, and the bound-0 branches do not exist | P22-03 |
| **V51** | No allele names a non-matter floor member | `check::v33`, and `sealed_natives()` lookup | P22-10 |
| **V52** | A reference body's genome reaches only the floor, its own cell, and valid seals — transitively | `check::v33` rules 1–5. This is V40 delivered | P22-09 |
| **V53** | The seal graph is acyclic, checked at registration | a planted two-seal cycle refuses. This is V42 delivered | P22-09 |
| **V54** | Every cell in the corpus is admitted, and every allele it carries is admitted against it | `corpus verify`, with `false_law.cell` as the refusal witness | P22-11 |
| **V55** | Every seal's cell is the unique cell declaring its sealed native | the lookup's two refusals | P22-12 |
| **V56** | Every gate item has a control that must fail | `GateItem`, in every gate | P22-02 |
| **V57** | `unfold` produces a cell the engine runs, and a wrong unfold refuses | `v23_execute`, with a planted wrong native | P22-13 |
| **V58** | A turn is checked in the frame of the cell that declares it | `check::turn`, with a ℚ counter-example | P22-15 |

Carried forward and re-run every commit: V18, V19, V20, V24-embryo, FO1–FO10, the three canonical-text properties, the 1 000-append hash-stability test, V33–V48, and Phase 1's four demos.

### 5.2 The two numbers that matter

`cargo xtask power` is still one of them, unchanged in rule: every mutant names the check that caught it, and a mutant whose check refuses its negative control fails the build.

The second is new and is this phase's contribution:

**`cargo xtask agree` prints, per seal, the samples taken and the sample that separated the counterfeit.** A seal that agrees is worth nothing on its own; a seal that agrees *and* can say which input tells a wrong body apart is worth the whole harness. If the separating sample is always the same one — always `b = 1`, say — that is a finding about the counterfeit, not a pass, and it goes in `Findings/` rather than getting tuned away.

### 5.3 What Phase 2.2 deliberately does not test

The cost of running counterfeits. Doubling the body executions in `agree` roughly doubles its wall time, and that is the price of the check, not a regression to optimize. `xtask perf` keeps its three probes and gains none.

Nor does this phase test whether a counterfeit is *good*. There is no measure of counterfeit quality and inventing one here would be inventing mutation-score theory in a correction phase. **R39** holds the question.

---

## 6. Dependencies and Forbidden Constructs

**No new dependencies.** Everything forbidden in Phases 1, 2 and 2.1 stays forbidden, with one change and five additions.

| Change | |
|---|---|
| `Seal::drives` | No longer `Option`, and its bound is `NonZeroU32`. The runtime refusal for a missing drive is replaced by a type that cannot express one |

| Newly forbidden | Why |
|---|---|
| A seal without a counterfeit | §2.2. It is a required field; there is no constructor without one |
| A check with no control, anywhere a gate can reach | §2.1 and §2.4. An item that cannot express one is deleted, not exempted |
| A cell in `corpus/` that the gate does not admit | §2.5. The counterfeit corpus under `corpus/phase22/counterfeit/` is the sole exception, is never in `hashes.txt`, and is refused **on purpose** |
| An allele naming a non-matter floor member | §2.7 |
| A gcd, division or reduction body | §3.3. The frame reduces at canonicalisation and FO8 already checks it. A second implementation is a second thing that can be wrong |
| Filling `corpus/testimony/` | §2.8. That is Phase 12 |

---

## 7. Exit Gate 2.2, As a Checklist

Scripted and repeatable, from a clean checkout, offline. `cargo xtask gate 2.2` runs it, **as a table of opposed items** — each line below names both halves.

- [ ] **1 · Every seal can see.** `agree` separates every counterfeit from its reference, and prints the separating sample. *Control:* a counterfeit identical to its reference makes the item fail with `BLIND SEAL`.
- [ ] **2 · The references are true.** All five agree with their sealed alleles on 256 generator-drawn samples, executing on the live engine. *Control:* the injected disagreement still refuses as a truth violation.
- [ ] **3 · A bound cannot be empty.** `Drive::bound` is `NonZeroU32` and no bound-0 branch exists. *Control:* the workspace does not compile with `bound: 0`.
- [ ] **4 · V33 walks the whole genome.** A reference reaching its sealed allele through a cell entry is refused naming the path; a seal cycle is refused at registration. *Control:* `int_add_ref`'s self-cell entry is accepted.
- [ ] **5 · The floor's registers are enforced.** A body naming `prim:hash` is refused naming the register. *Control:* a body naming `prim:case` is accepted.
- [ ] **6 · The corpus is true.** Every cell admits; every carried allele admits against its cell. *Control:* `false_law.cell` is refused naming its law.
- [ ] **7 · A seal names its own cell.** Each seal's cell is the unique declarer of its native. *Control:* a seal for a native no cell declares is refused, and so is one with two declarers.
- [ ] **8 · The round-trip holds.** `parse(format n) = n` on 10 000 samples, with `"007"` named as the hole. *Control:* a lossy pair declared `one_way: false` refuses.
- [ ] **9 · The path of truth.** `cargo xtask gate all` runs phases 0, 1, 2, 2.1 and 2.2 and writes `gates.lock` from per-gate outcomes. *Control:* a lock with a wrong score makes the item fail, and an item whose own control passes fails the build.

---

## 8. Risks Watched During This Phase

| Risk | Instrument | What to do when it fires |
|---|---|---|
| **The stated adversary · a control chosen weak** | §2.2's rule that the three counterfeits are harvested from history, not invented; and §5.2's separating-sample report | A counterfeit that is trivially wrong — a body that refuses, or returns the wrong *frame* — tests nothing, because the sealed allele separates it on sample one. If the separating sample is the first sample for every seed, the counterfeit is too crude. Write that in `Findings/` and make it subtler; do not raise the sample count to hide it |
| **New · the counterfeit corpus becomes a second corpus to maintain** | `corpus verify`'s exception list | The counterfeits are refusal witnesses, not goldens. They stay out of `hashes.txt`. The day one acquires a golden hash, it has become a cell, and something has gone wrong |
| **New · the reference bodies do not terminate** | `drives`, and the global budget naming the activation path | A budget refusal in a reference body is a design error in the body, not a bound that is too tight. Fix the body. R35 is the real answer and is still open |
| **New · `int.format` is unusably slow** | P21-18's three probes, unchanged | Division by repeated `Sum` at a turn is linear in the quotient. A reference is slow by construction. Record the number; do not optimize, and do not shorten the samples |
| **Carried · the floor grows a habit** | The petition procedure of §2.1 of the Phase 2.1 plan | Unchanged. Nothing in this phase amends the floor, and a commit that tries has misread it |
| **3 · The live engine is too slow to build with** | `perf`, in seconds | Unchanged. The finding is written by a human, then the decision is made |
| **5 · Jet mismatch** | `agree` with generator-drawn non-driving ports and a counterfeit | The disagreement is the finding. Write the counter-example before touching the sealed allele |

---

## 9. What This Plan Refuses To Do

| Refused | Because |
|---|---|
| Fix `bound: 0` without adding the counterfeit | §2.1. The bound is the symptom. A check with no opposite is the disease, and it will pick a new symptom |
| Delete the three hollow reference bodies | §2.2. They are the only counterfeits in this project whose power is known, because they already worked |
| Leave `gates.lock` at 9/9 while the phase runs | P22-00, and P21-00 before it. A lock that records intent is not evidence |
| Invent a testimony corpus | §2.8. `corpus/testimony/` stays empty until Phase 12 asks the question properly |
| Write a gcd | §3.3. The frame reduces; FO8 checks it; a second implementation is a second thing that can be wrong |
| Give `mul.cell` a multiplicative-identity law | §2.5. That needs `one` in ℤ's signature, which is a frame change, which is not this phase |
| Rebless a golden hash | Still §6.2 of the Phase 0–1 plan. `mul.cell`'s hash moves because the cell was corrected on purpose and the move is recorded — that is not reblessing, and the distinction is the whole discipline |
| Amend the floor | §1. Once per phase was 2.1's allowance and it was spent |
| Design the Phase 3 host protocol | §1. `joinn-run` is still deleted at Phase 3 |

---

## 10. Open Items This Plan Creates

Continuing from R38.

| ID | Topic | Question |
|---|---|---|
| **R39** | Where does a counterfeit come from? | §2.2 harvests three from history, which is honest and does not generalise — a new seal has no failure to harvest, so its counterfeit is invented by the same author who wrote the reference. Is a plausible wrong sibling enough? Is there a measure of a counterfeit's strength that is not mutation score with a new name? Does a seal want *several* counterfeits, one per way of being wrong — and if so, does the floor's pairing arithmetic have anything to say about how many? |
| **R40** | Must a reference loop? | §2.3 makes `drives` total, so every seal declares a measure, so every reference is assumed to loop. A reference that terminates structurally — no self-wire, no recursion — has nothing to bound and cannot be registered. Is that a real class, and does `drives none` need to exist as a *claim* that the gate checks, rather than an absence it refuses? |
| **R41** | What is a witness of agreement? | §2.8 deletes `Seal::agreements` rather than guess. When testimony arrives: is a witness the sample set, the seed, the separating sample, or the whole transcript? Is it hashed? Must a later run reproduce it byte-for-byte, and if so, does that freeze the generator — and has the generator then become part of the floor's identity? |
| **R42** | Is `admit_cell` without laws the right split? | §2.5 fixes F17 by admitting corpus cells *and* their alleles, which works because a corpus cell happens to carry one. But `admit_cell` judges a cell carrying an allele without judging the allele, and that is how the false law survived. Is a cell-with-an-allele one thing the gate should judge as a whole, or is the split correct and the corpus was simply not asking? |
| **R43** | Does opposition generalise past checks? | §2.1 applies Law 1 to the instruments and it fits suspiciously well. Does it apply to a **refusal** — must every refusal name what it would have accepted? That would make a counter-example mandatory, which several refusals in the repo do not carry. Is the missing half of a refusal the reason some of them are hard to act on? |

**Touchpoints with existing items.** R33's open half is untouched: a failed reduction is still evidence rather than proof. **R35** is promoted from a nicety to the real answer to F9 — if `drives` were derived from the reference's termination measure, §2.3 and half of §2.2 would be unnecessary, and the author could not set the bound at all. **R37 is answered: yes.** Negative controls generalise past the mutant corpus, the case that proves it is F9, and §2.1 is the general statement. R30's budget answer is unchanged. R36 is unchanged; nesting still holds no grants.

---

## Appendix A · `AGENTS.md` and `.cursor/rules/joinn.mdc` for Phase 2.2

Replace both files with this.

```markdown
# JoInn — standing rules

This repo is JoInn. Phase 2.1 corrected Phase 2's tautologies and introduced a
new one: three reference alleles that compute nothing, reporting agreement
because their sampling bound admitted a single value. Phase 2.2 is the second
correction, and it has one idea: OPPOSITION APPLIES TO THE INSTRUMENTS. Every
check names what it must refuse and what it must accept. The build plan is
docs/Plans/JoInn Phase 2.2 Implementation Plan.md. Work one numbered commit at a
time. Do not start the next one.

There is no host protocol, no second body, no assay, no testimony corpus and no
compiler in this phase.

## Hard rules

1. A refusal is a VALUE (`Verdict::Refused`), never a Rust `Err` and never a
   panic. `Result` is for host errors only: IO, malformed input, bugs.
2. Never `unwrap`, `expect`, or panic outside tests. `#![forbid(unsafe_code)]`.
3. No `f32`/`f64` anywhere. No `HashMap`/`HashSet` — `BTreeMap`/`BTreeSet` only.
4. No wall clock and no unseeded RNG in crates/. `xtask` MAY measure wall time;
   it cannot reach a hash, a canonical form, a sample or a refusal. Message
   delivery order is a function of the body's grant list, never of arrival time.
5. No `#[derive(Hash)]` and no `serde` derive on any DNA type. The canonical
   writer is hand-written and tested. Bodies follow the same rule as cells.
6. Never change a golden hash in corpus/ to make a test pass. If a hash moves,
   stop and report it. A hash that moves because a cell was CORRECTED on purpose
   is recorded with a dated finding — that is not reblessing.
7. Never weaken, skip, `#[ignore]`, or delete a test to make a build green.
   Report the failure instead.
8. A primitive name must never appear in a coding region. Laws may name: the
   cell under definition (`self`), frame signature operations, and other coding
   regions BY HASH. A BODY may name primitives — that is what `prim:` is for.
9. Display names, literals, prompts, styles and layout are REGULATORY.
10. THE ENGINE NEVER RECURSES IN RUST. Nesting is an explicit activation stack
    with ONE global step budget.
11. A REFERENCE ALLELE IS A BODY. `Seal::reference` is a `BodyRef`. A reference
    implemented as a Rust `Oracle` is rejected on sight.
12. THE FLOOR IS IRREDUCIBLE AND OPPOSED, and it is defined in
    docs/Findings/the-floor.md. Never add to the floor. If a task seems to need a
    new primitive, first write it as a reference body over the existing floor.
    NEVER write "the minimal set" or the floor's size as a number. The count is
    derived by `cargo xtask floor`.
    `zero`, `succ` and `pred` are NOT floor primitives. They are ℤ's first three
    constructors and live in ℤ's signature as frame vocabulary.
13. `sub`, `subtract` and `minus` are not identifiers in this project.
14. Nothing may depend on joinn-run. It is deleted at the start of Phase 3 and
    must never grow a `trait Host`.
15. `std::io` appears only in joinn-run.
16. A nested activation holds NO grants and can reach no capability.
17. NO INSTRUMENT WRITES ITS OWN FINDING. `xtask` prints numbers to stdout. No
    xtask subcommand writes a file under docs/Findings/.
18. EVERY MUTANT NAMES THE CHECK THAT CAUGHT IT, and registers a negative
    control the check must ACCEPT.
19. EVERY CHECK IS OPPOSED. A check declares a witness it must refuse and a
    witness it must accept, and fails the build if it cannot do both. This is
    Law 1 applied to the instruments. A check with only one half is a widowed
    member, exactly as in the floor.
20. NO INSTRUMENT CHOOSES ITS OWN INPUTS. A bound that narrows what a check can
    see is PART OF THE CHECK. `Drive::bound` is a `NonZeroU32` because a bound
    that admits one value is not a bound. Every seal carries a COUNTERFEIT body
    that `agree` must separate from the reference at the declared bound; a
    counterfeit that is not separated is a BLIND SEAL and fails the build.
21. A SEAL NAMES THE CELL THAT DECLARES ITS SEALED ALLELE. The cell is found by
    lookup, not assigned by a table; zero matches and two matches both refuse.
22. A CORPUS CELL IS ONE THE GATE ADMITS, and an allele it carries is one the
    gate admits against it. `corpus verify` checks this. The counterfeit corpus
    under corpus/phase22/counterfeit/ is refused ON PURPOSE and is never a golden.
23. AN ALLELE MAY NAME ONLY MATTER. Space and physics members of the floor are
    the engine's; `hash` and `resolve` are physics and are not in the
    allele-visible native set.

## Definition of done

A commit is done when the plan's "done when" command passes and you have
reported what it printed. "It compiles" is not done. "The tests pass" is not
done if the test does not refuse anything. A number is not done if the code that
printed it also chose it. AND A REFUSAL IS NOT DONE IF NOTHING WAS EVER SHOWN TO
IT — a check with an empty domain prints the same word as a check that passed.
```

## Appendix B · Directory layout after Phase 2.2

```
D:\JoInn\
├── docs\
│   ├── Plans\
│   │   ├── JoInn Phase 2 Implementation Plan.md
│   │   ├── JoInn Phase 2.1 Implementation Plan.md
│   │   └── JoInn Phase 2.2 Implementation Plan.md     ← this file
│   └── Findings\
│       ├── the-floor.md                 (registers now enforced in code)
│       ├── floor-vocabulary.md
│       ├── phase-2-review.md            (F1–F8)
│       ├── phase-2.1-review.md          (F9–F18)
│       ├── surviving-mutants.md         (mutant 14 closed; three blind seals opened P22-01)
│       ├── canonical-form-changes.md    (mul.cell's corrected law, P22-11)
│       ├── turn-annotations.md
│       ├── live-engine-performance.md
│       └── witness-refreeze.md
└── joinn\
    ├── corpus\
    │   ├── phase0\  phase2\  phase21\
    │   ├── phase22\                     ← the three rewritten references
    │   │   └── counterfeit\             ← the three that fooled the harness,
    │   │                                   plus false_law.cell. NEVER goldens
    │   └── transcripts\
    └── crates\                          (six; joinn-gate's check::v33 finally walks cells)
```

## Appendix C · Glossary delta for code

| Theory term | Rust identifier | Notes |
|---|---|---|
| opposition, applied to a check | `GateItem { check, control }`, `Mutant::control`, `Seal::counterfeit` | Law 1 on the instruments (§2.1). Three shapes, one rule |
| counterfeit | `Seal::counterfeit: BodyRef` | a body `agree` is OBLIGED to catch. The three in this phase are harvested from F9 |
| blind seal | `BLIND SEAL` refusal | the counterfeit was not separated — the finding is about the sampling, not the body |
| drive bound | `Drive::bound: NonZeroU32` | a bound that admits one value is not a bound (§2.3) |
| register | `Prim::register() -> Register` | matter / space / physics, enforced by `check::v33` (§2.7) |
| the genome walk | `check::v33` rules 1–6 | V40 and V42 delivered. Rule 2 is the self-cell exemption that makes it writable |
| seal lookup | `seal_register()` | a seal finds its cell by its native; zero or two matches refuse (§2.6) |
| corpus admission | `corpus verify` | cells are admitted, not merely hashed (§2.5) |

---

*JoInn Phase 2.2 Implementation Plan (Draft 0.1). Corrects F9–F18 of `docs/Findings/phase-2.1-review.md`, including its addendum. Builds on the Phase 2 and 2.1 Implementation Plans and Parts I–III. Everything marked PROPOSED · yours is a recommendation made while writing this plan; Cursor implements what the roadmap, the grammar document and your decisions say, not what this plan prefers.*
