# Phase 2 review — are we following the plan?

Reviewer: Claude · September 18, 2026 · against `docs/Plans/JoInn Phase 2 Implementation Plan.md`

**Method and its limit.** This review is a source read of `D:\JoInn\joinn` at the
state of 2026-09-18. **No command was run**: not `cargo test`, not
`cargo xtask gate all`, not `power`, `agree`, `corpus verify` or `perf`. Every
claim below is about what the code says it does, never about what it printed.
Where the finding is that a check is vacuous, that is a claim about the check's
text and it stands without running it; where a check looks real, this review
says only that it looks real.

---

## Verdict

Structurally the phase is followed. Every commit P2-00 … P2-19 has an artifact,
the six crates exist with the dependency rule intact, `joinn-gate` holds no
arithmetic, the nine Phase 0 goldens are unmoved, and `gates.lock` records
`phase 2: 8/8`.

Substantively the phase's central claim is not true. **The reference alleles are
written in Rust, not in the minimal set, and the live engine cannot run a
DNA-backed allele at all.** V21 is unmet. The instrument the plan designed to
catch exactly this — mutant 14 — is a tautology that always reports "refused."

The Phase 0–1 plan named this failure in advance, in §2.1 of the Phase 2 plan's
own ancestor: of the three wrong fixes for the eliminator problem, *"write the
reference in Rust (kills it more quietly)."* It was killed quietly.

---

## What is genuinely there

Listed first because it is the larger part, and because the findings below are
worth acting on only if the foundation under them is sound.

| Commit | State |
|---|---|
| **P2-02** eliminators | Real. `Frame::case`, `Case::{Generator, Built}`, FO8 and FO9 in `conformance::check`, and a `BrokenFrame` carrying a lying `case` that the harness refuses **by obligation name**. |
| **P2-03** the floor | The strongest code in the phase. `matter::check_oppositions` walks the register; `check_one` verifies `pred(succ x) = x`, `split(pair(x,y)) = (x,y)`, `choose(eq(x,x),a,b) = a`, `eq(x,x) = 1`, and `case(zero) = Generator` on values drawn from **the frame's own generators**. An undeclared opposition refuses; a non-`hash` one-way declaration refuses; `oneway != 1` refuses. |
| **P2-04/05** the body form | `joinn.body.v1`, four spellings of `calculator.body` canonicalised and compared, a `trybuild` case for the body's regulatory seal, a wire to an unknown cell refused. |
| **P2-06 → P2-10** the engine | Explicit `BTreeSet` worklist keyed `(grant_epoch, wire_position, message_sequence, seq, dest, port)`; no Rust recursion; `CheckId::Budget` distinct from membrane refusals and asserted by value; determinism at 100 runs × 2 seeds; `Refuse`/`Latest`/`Queue` each with a test; ungranted read refused; `"two"` refused at the membrane with nothing crossing the out-port. |
| **P2-15** evolution | `sum_turn.cell` carries `lineage 6b32…`; `admit_sum_turn` refuses outright if `sum.cell`'s hash has moved, by comparing against the literal golden. |
| **P2-01** narrowing | `demos.rs` names mutants by string and takes the register as an argument; the alleles live in `joinn-prim`. The `vocab` lint bans `BigInt`/`BigRational` under `crates/joinn-gate`. |

---

## Findings, worst first

### F1 · The reference alleles are not references — and the engine cannot run one

`AddIntRef`, `ParseIntRef`, `MulInt`, `AddRatRef` (`joinn-prim/src/seals.rs:44`
and following) are Rust structs implementing `Oracle` with `loop`/`match` over
`Frame::case`. They have the right *shape* — case analysis, succ/pred, no Rust
recursion — but they are not cells, not bodies, and not expressible-in-the-
minimal-set in any sense a machine can check.

`Seal::reference` is typed `Hash` and documented, at `seals.rs:19`, as *"may be
a placeholder until a body is stored."* No body is stored.

And in the engine, `joinn-live/src/lib.rs:487`:

```rust
AlleleBody::Dna(_) => {
    ... format!("{name} Dna allele is run by agree, not by this fire path")
}
```

The live engine refuses DNA alleles. `AlleleBody::Dna` is a variant nothing
executes.

Against the plan:

- **P2-11 done-when** — "Reference allele **as a cell** (case + choose +
  succ/pred) **running on the live engine**." Not met.
- **P2-12 done-when** — "unfolding a sealed primitive in a test returns a cell
  **the engine can actually run**." Not met, by construction.
- **V21** — "every seal carries a reference allele expressible in the minimal
  set." Not met; there is no artifact in which the expressibility could be
  checked.
- **V33** — stated as a property over the corpus. There is no corpus of
  reference bodies to hold the property over.

This is the phase's substance. Everything in F2 and F3 is downstream of it.

### F2 · Mutant 14 is a tautology

The plan is unusually explicit here — mutant 14 is "a seal whose reference
allele is the sealed one in disguise," and it says *"`agree` passes — so this
one is caught by a rule, not a test: a reference allele that is not expressible
in the minimal set fails V33's structural check."*

`xtask/src/main.rs:628`:

```rust
fn v33_reference_not_sealed() -> bool {
    std::any::type_name::<joinn_prim::alleles::AddInt>()
        != std::any::type_name::<joinn_prim::seals::AddIntRef>()
}
```

Two distinct Rust types always have distinct type names. This returns `true`
unconditionally, and `power` counts it as a refused mutant. So `20/20` is
`19/20` with one tautology — and the tautology is sitting directly on top of F1,
which is the thing it was written to catch.

This is the surviving mutant. By §5.4's own rule it should be written into
`surviving-mutants.md` **before** it is fixed.

### F3 · `agree` never samples anything large

`seals.rs:458` and `seals.rs:469`:

```rust
fn small_int(seed: u64, i: u32) -> Value { ... (seed + i) % 65 - 32 ... }
fn small_int_tiny(seed: u64, i: u32) -> Value { ... (seed + i) % 7 - 3 ... }
```

The Int samplers in the agreement harness bypass `Frame::generate` entirely.
The Phase 0–1 plan §4.2 is direct about this: an ℤ generator must produce
values "well beyond `i64::MAX`… Write that in the frame, not in the tests." The
frame does; `agree` does not use it.

Consequence: the whole class of mutant 13 — a sealed allele correct except
beyond `i64` — is invisible to the differential harness. That is why
`wrapping_disagrees_beyond_i64()` exists as a hand-rolled single-point check at
`i64::MAX + 1` in `turns.rs`, standing in for the harness. As built, `agree`
could not find a jet mismatch above 32. Risk #5's instrument is blind in exactly
the region where jet mismatches live.

`Sample::Rat2` does use `rat.generate`. Fixing `Int2`, `Int2Small`, `Int1` and
`Text` to do the same is a small change with a large effect on what the harness
can see.

### F4 · Turn admission is outside the gate, and narrowed to one shape

The word "turn" does not appear anywhere in `joinn-gate`. `admit_turn` lives in
`joinn-prim/src/turns.rs` and is called directly from `xtask::gate_two`.
`Gate::admit_cell(&sum_turn, Some(&parent))` admits a cell carrying a `turn`
block **without ever asking whether a turn allele exists or agrees**.

The plan's §2.5: "A turn is admitted only with a turn allele… and **the gate
admits it** by generating the round-trip law and checking it against the forward
allele on samples." As built, a cell can declare `turn 0 from {1 2}`, be admitted
by the gate under check 4, and have no turn allele at all.

Three smaller things in the same file:

- `turns.rs:60` — `if turn.out != 0 || turn.from != [1,2] { refuse("Phase 2
  admits only turn 0 from {1 2}") }`. Turn admission is hard-coded to `Sum`'s
  shape. Nothing in the plan licenses that narrowing, and it means the
  generality of the mechanism is untested.
- The round-trip law samples with the same bespoke ±32 `small_int`, not the
  frame's generators. Mutant 19 (a turn wrong on negatives) is caught because
  ±32 straddles zero — by luck of the modulus, not by the generator discipline.
- The plan's "derivation for the free case" (`succ`/`pred` derived from the
  signature) is not implemented. `turn-annotations.md` asserts it is.

### F5 · R31's number is a literal, and the finding writes its own prose

`turns.rs:14`:

```rust
pub const HANDWRITTEN_TURN_ALLELES: u32 = 1;
```

A constant, not a count over the turn register. And `xtask::gate_two` writes the
entire body of `docs/Findings/turn-annotations.md` from a hardcoded template,
interpolating only that constant — including the sentences "There is one:
`int.add.turn0`…" and "The count is one, on the one cell that needs it."

R31 is the number that decides whether `Turn` survives to Phase 5. As built it
is an assertion that restates itself. It happens to be *correct* today — there
is exactly one hand-written turn allele — which is what makes it dangerous: it
will still say "1" on the day there are nine.

### F6 · P2-18's conclusion is in the template, and the honest number is unmeasurable by rule

`xtask::perf` interpolates three step counts into a markdown string whose final
paragraph is fixed text (`main.rs:756`):

> **Risk #3 has fired: no.** There is no superlinear blow-up…

That sentence is printed whatever the numbers are. The plan asked for "an
explicit yes/no on whether Risk #3 has fired"; what exists is an explicit "no"
with three numbers above it.

Underneath that is a real collision worth a decision rather than a workaround.
Risk #3 is "the live engine is too slow to build with" — a wall-clock question.
The findings file says wall time is not recorded because the clock API is
forbidden in this workspace, and the `vocab` lint bans `Instant` and
`SystemTime` under `xtask` as well as `crates`. So the rule that keeps the truth
core deterministic has also made the phase's stated adversary unmeasurable. The
determinism rule is right; its scope is wrong. `xtask` is a measuring
instrument, not a path that can reach a hash, a canonical form, a sample or a
refusal, and §2.9's ban was written for those paths.

### F7 · The runner never reads stdin, and ships the mutant register

`joinn-run/src/main.rs::calculator_session` pushes the literals `"two"`, `"2"`
and `"3"` into the output string and injects the same values into the engine.
There is no `std::io` in the crate at all.

To be fair to it: the refusal text is the engine's real refusal, and `2 + 3 = 5`
comes from a real `BodyState` run over the real parsed body and cells. It is not
fabricated output. But the session is a fixture, not a session, and P2-17's
"stdin/stdout at the membrane only" describes something that does not exist yet.
Exit gate 2's Milestone 0 should say what it is: the engine really runs; the
creator's half of the conversation is scripted.

Separately, `main.rs:115` loads `joinn_prim::natives_with_mutants()`. The
shipping binary registers `mutant.impostor`, `mutant.saturating` and the rest.
P2-01 said the mutants move to a *test-only* register; `natives_with_mutants` is
a plain `pub fn` used by the runner, by `perf`, and by every gate path.

### F8 · Smaller things, in order of how much they mislead

| Where | What |
|---|---|
| `seals.rs:632` | `pub fn format_parse_is_one_way() -> bool { true }`. P2-13's "a test asserts the declaration exists" asserts a constant. |
| `seals.rs:624` | `v23_holds` folds, unfolds and compares coding hashes — but coding hashes never include alleles (V19), so it holds for any cell whatever `fold` does. It re-tests V19 under V23's name. |
| `main.rs:692` | `one_delivery_path` asserts `delivery_count() == steps()`. A counting coincidence, not the structural claim V35 makes ("exactly one delivery path"). |
| `corpus/transcripts/calculator.txt` | Exit gate 2 specifies three-space indent on the refusal line; the file has none. "Prints, **exactly**" is the standard the plan set, and the transcript is a frozen witness. |
| `corpus/transcripts/calculator.trace` | Two records labelled `step 4`: the quiescent report reuses `self.step` without incrementing. Harmless today, confusing forever. |
| `xtask::vocab` / `line_has` | `_` is treated as a word character, so `fn calculator_body()` in `joinn-live` passes the V38 ban on "calculator". Also any line containing `allow(vocab)` is skipped wholesale — a silencer with no record. |
| `matter::register` | `hash` sits in the **matter** register; §2.4 puts it in **physics**. It is not in `sealed_natives()`, so no allele can call it, but the classification drifted from the frozen set. |
| `xtask::write_lock` | `gates.lock` is written from a hardcoded string including `phase 2: 8/8`. It runs only after the checks pass, so it is not false — but the file records the tool's intent rather than the checks' results, and `gates.lock` is supposed to be evidence. |

---

## What to do

**F1–F4 are one knot.** Untying it is a phase-scale decision, not a patch, and
there are two honest roads. Both are acceptable; drifting between them is not.

**Road A — finish P2-11 as written.**

1. Write the reference bodies as `.body` files in `corpus/phase2/`, hand-compute
   their hashes as P2-04 did, and store them as goldens.
2. Teach the engine to fire `AlleleBody::Dna` by running the reference body on a
   nested `BodyState`, charging its steps to the outer budget.
3. Make `Seal::reference` a real body hash and delete the placeholder comment.
4. Replace `v33_reference_not_sealed` with a structural check: the reference
   body's genome names only floor cells and other sealed cells' references,
   transitively, bottoming out in the minimal set. That is V33, and it is the
   only version of mutant 14 that can fail.
5. Re-run `agree` with the references executing on the engine rather than as
   Rust oracles.

**Road B — record the retreat.** Write into `docs/Findings/` that Phase 2's
references are Rust-shaped-like-the-floor rather than cells; that V21 and V33
are deferred; that `AlleleBody::Dna` is reserved and unrun; that mutant 14 is
unchecked. Then drop `gates.lock` to the number of gates actually met, and move
the reference-as-cell work to the head of Phase 3 with a named commit.

**Independent of that choice, and cheap:**

- Point `agree`'s `Int2`, `Int2Small`, `Int1` and `Text` sampling at
  `Frame::generate`. Then delete `wrapping_disagrees_beyond_i64` and let mutant
  13 be caught by `agree`, which is where the plan put it.
- Move turn admission into `joinn-gate` so that `admit_cell` refuses a `turn`
  block with no agreeing turn allele, and lift the `turn 0 from {1 2}`
  restriction or write down why it stands.
- Derive `HANDWRITTEN_TURN_ALLELES` from the turn register, and stop letting
  `perf` and `gate 2` author their own findings prose. A tool that writes its own
  conclusion is the Phase 2 adversary named on page one of the plan: an artifact
  that looks right.
- Scope §2.9's clock ban to `crates/` so `xtask perf` can report seconds, and
  answer Risk #3 with a measurement.
- Add `mutant.*` registration to the V38 lint list so the runner cannot load it,
  and give `natives_with_mutants` a `#[cfg(any(test, feature = "mutants"))]`
  gate or an equivalent fence.
- Fix the transcript indent and the duplicate step number, and re-freeze both
  witnesses deliberately, with a line in `canonical-form-changes.md`-style
  discipline even though no hash moves.

---

## One note on the shape of all this

Every finding above has the same shape: the artifact exists, the command exists,
the number prints, and the thing being checked is not the thing the plan named.
That is not carelessness — the code is careful, and the parts that are real
(the floor's oppositions, FO8/FO9, the engine's determinism) are more rigorous
than they had to be. It is the specific failure mode the Phase 2 plan predicted
in its §0: *"a transcript that looks right is the most convincing wrong artifact
this project will ever produce."*

The defence the plan proposed was that every done-when be a refusal, a
disagreement or a byte-identical replay. That defence held wherever the
done-when was a refusal. It failed wherever the done-when became a printed
number that the tool also authored — `power`'s 20/20, R31's 1, Risk #3's "no."

A rule worth adding to `AGENTS.md`, if any of this lands: **no instrument writes
its own finding.** `xtask` prints numbers; a human writes what they mean.
