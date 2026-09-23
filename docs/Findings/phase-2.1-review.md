# Phase 2.1 review — did the correction correct?

Reviewer: Claude · September 18, 2026 · against `docs/Plans/JoInn Phase 2.1 Implementation Plan.md`

**Method, and its limit.** Unlike the Phase 2 review, **this one ran the commands.**
A copy of `D:\JoInn\joinn` as of 2026-09-18 was built and exercised:
`cargo build --workspace --locked`, `cargo test --workspace --locked`,
`cargo xtask gate all`, `gate 2.1`, `power`, `agree`, `floor`, `vocab`, `perf`.
Where this review says a check is vacuous, it says so either from the check's
text — which stands without running it — or from an **experiment**: a named
edit to a copy, the command re-run, and the refusal it then produced quoted with
its seed. Every experiment was reverted. **No file on the build machine was
modified.**

Three limits, stated so they are not mistaken for coverage:

1. The build ran on Linux in a container, not on the Windows machine. Exit gate
   2.1 item 9's *"from a clean checkout, offline"* was **not** reproduced —
   dependencies were fetched from crates.io against the committed `Cargo.lock`.
2. `joinn-live/src/lib.rs` (65 KB) and `joinn-dna/src/parse.rs` (39 KB) were
   searched, not read end to end. The activation stack is reported here as
   *behaving* correctly, not as *audited*.
3. Findings continue the Phase 2 review's numbering, so an F-number is unique
   across the project. F9 begins here. **F17 and F18 were added later the same
   day**, while the Phase 2.2 plan was being drafted; they are in an addendum
   after F16 rather than sorted into place, so that a number written down stays
   put.

---

## Verdict

**The structural half of the phase landed. The substantive half landed for two
of five seals.**

F1 is closed in form and open in fact. `Seal::reference` is a `BodyRef`, there
is no reference `Oracle` left in `joinn-prim`, `AlleleBody::Dna` fires, and five
reference bodies live in `corpus/phase21/` with hand-computed hashes that match.
Then three of those five bodies compute nothing, and `agree` reports that they
agree, because the `drives` declaration that was supposed to bound one port was
set to zero — which does not bound a port, it pins it to a single value.

`gates.lock` reads `phase 2.1: 9/9`. That number is the new `phase 2: 8/8`.

Two further findings — a corpus cell whose stated law is false, and a seal that
names the wrong cell — are in the **Addendum** after F16.

The phase's stated adversary was *"a correction phase that corrects the
appearance."* The plan's §8 said to reject a commit that prints a pass. Eight of
the nine exit-gate items print passes that mean what they say. Item 1 does not,
and item 9 prints a pass without checking anything at all.

---

## What is genuinely there

Listed first because it is the larger part, and because the findings below are
worth acting on only if the foundation under them is sound. Everything in this
table was run, not read.

| Commit | State |
|---|---|
| **P21-02** the floor rule | Done, and done well. `zero`/`succ`/`pred` are out of the floor and in ℤ's signature; `prim:succ` in a genome is refused **by name** (`joinn-live` test `prim_succ_is_refused_by_name`). `Opposition::OneWay` is deleted. `check_pairing` walks the register and refuses an undeclared opposition, an opposite outside the floor, and an opposite that names someone else — three distinct refusals, none vacuous. `cargo xtask floor` prints eight pairs: `bind↔unbind`, `bound↔fill`, `build↔case`, `choose↔eq`, `fan↔join`, `grant↔revoke`, `hash↔resolve`, `pair↔split`. |
| **P21-03/04** eliminator | `Frame::constructors()` ordered and in identity; FO10 present; `floor_oppositions_hold_on_10000_samples` passes against the frames' own generators. |
| **P21-05** `prim:` genome | Four spellings canonicalise identically; `prim:nonesuch` refuses naming it; **all fourteen Phase 0/2 goldens still match**, so §2.3's "no golden moves" held. `corpus verify` reports 21 hashes matching. |
| **P21-06** activation stack | `calculator.trace` replays byte-for-byte. Depth omitted at depth 0. One global budget. No Rust recursion in `fire`. |
| **P21-13** agreement past `i64` | Real for the seals that have a real reference. `wrapping_disagrees_beyond_i64` is gone and `power` still refuses mutant 13 **by `check::agree`**. Verified: non-driving ℤ ports come from `IntFrame::generate`, whose case table reaches `i64::MAX`, `i64::MIN`, and deliberately beyond both. |
| **P21-14** turn in the gate | `check::turn` is a genuine generated round-trip and it is the gate's fifth check. An unwitnessed `turn` block is refused by `admit_cell`; `turn 1 from {0 2}` admits with its own allele. See F15 for what is still narrow. |
| **P21-15** turn count | Derived from the register. `handwritten_count_moves_when_register_grows` passes. `turn-annotations.md` is hand-typed. |
| **P21-16/17** runner | Real stdin; `echo "two\n2\n3"` produces the five-line transcript with the plan's indent. Mutants are `cfg`-gated and `sealed_natives_do_not_contain_mutants` asserts their absence by lookup. |
| **P21-18** perf | `xtask perf` prints three probes with milliseconds and **writes no file**. `live-engine-performance.md` is hand-typed. |
| **P21-00/§2.8** instruments | `xtask_does_not_write_findings` and `write_lock_has_no_literal_gate_score` both pass. This rule took. |
| **P21-19** vocab | V38 now matches substrings, `calculator_body` is gone from `joinn-live`, and `allow(vocab)` requires a trailing reason. |
| **P21-20** witnesses | Both re-freezes are real and `witness-refreeze.md` records them, dated, with reasons, no hash moved. |
| **§5.2** negative controls | Implemented. `power` prints `refused but negative control also refused` and fails when a check refuses everything. |

Totals as printed: `cargo test --workspace` 71 passed, 0 failed. `power` 20/20,
each mutant naming its check. `vocab` ok.

---

## Findings, worst first

### F9 · Three of the five reference alleles compute nothing, and `drives` is where they hide

`Drive` was specified in §2.6 to bound the one port that drives the reference's
loop, so the reference could finish while every other port drew from the frame's
full generator. Three seals declare `bound: 0` (`xtask/src/main.rs:486`):

```
("text_parse_ref", "parse@Text", Some(Drive { port: 0, bound: 0 }), true),
("int_format_ref", "format@ℤ",   Some(Drive { port: 0, bound: 0 }), false),
("rat_add_ref",    "add@ℚ",      Some(Drive { port: 1, bound: 0 }), false),
```

A bound of zero does not bound a port. It **pins it to one value**.
`sample_port` (`crates/joinn-prim/src/seals.rs:136`) returns the literal `"0"`
for Text at `d.bound == 0`, the literal `0/1` for ℚ, and `bounded_int(…, 0)`,
whose span is 1, is always `0`. `text.parse_int` has one input port, and it is
the driving port — so that seal is tested at exactly one point, thirty-two
times. `int.format` likewise. `rat.add` varies `a` and holds `b` at zero.

The bodies are what that permits them to be. `corpus/phase21/rat_add_ref.body`
is one genome entry and one wire:

```
genome
cell:fc5403…  as self
wires
self@0 -> self@2
```

That is `add(a, b) = a`. It agrees with `add@ℚ` because `b` is always zero.
`text_parse_ref.body` computes `eq(a, a)`, pushes the result through `case`, and
wires the parts to its out-port; it does not parse. `int_format_ref.body` wires
`prim:hash`'s output into `build`'s **frame** port, which is not a formatting
algorithm under any reading (and see F14).

**Experiment.** Raising each bound in a copy and re-running `cargo xtask agree`,
one at a time, seed 1 unchanged:

| Seal | Bound raised to | Result |
|---|---|---|
| `text.parse_int` | 4 | `TRUTH VIOLATION` — expected `-31`, got `0` |
| `int.format` | 4 | `TRUTH VIOLATION` — expected `"-3"`, got `"0"` |
| `rat.add` | 4 | `TRUTH VIOLATION` — expected `-65853234582920539119026175/7139822`, got `1/7139822` |

**The two that are real.** `int_add_ref` (drive port 1, bound 32) and
`int_mul_ref` (drive port 1, bound 8) are genuine reference bodies. `int.mul`
carries a bespoke branch in `agree` that overrides *both* ports with
`bounded_int` at the drive bound, contrary to §2.6's one-driving-port rule;
deleting that branch so `a` comes from the full ℤ generator, and lowering the
bound to 6, the seal **still agrees**. So the branch is unnecessary as well as
wrong, and `int_mul_ref` is honest work.

This is F3 with the sign flipped. F3 said `agree` never samples anything large.
The correction introduced a per-seal bound and then set three of those bounds so
low that the harness samples nothing at all. §8 of the plan anticipated the
mechanism exactly — *"a harness that runs fast because it stopped looking is F3
again"* — and named the wrong instrument to catch it.

---

### F10 · `check::v33` never walks a `cell:` entry, so V40 and V42 do not exist

§3.4 specified a transitive walk with four clauses. The implementation
(`crates/joinn-gate/src/check/v33.rs:36`) is:

```rust
GenomeTarget::Cell(_) => {}
```

Clause 1 (a cell entry's own seal passes this check), clause 2's *transitivity*,
and clause 3 (the seal graph is acyclic) are absent. **V42 is not implemented
anywhere** — no registration-time walk, no cyclic-pair refusal. V40 is
implemented only for the `prim:` half of a genome.

What survives is clause 4 (no grants) and a name comparison against `prim:`
entries. That comparison is what refuses mutant 14, and it is thinner than it
looks: the mutant body must spell the sealed native `add@ℤ` as a **floor entry**,
and `floor.contains("add@ℤ")` would have refused it one line later regardless.
The teeth close on a name that could never be a legitimate floor member. A
reference body that reaches its own sealed allele through a `cell:` entry —
which is the realistic shape of the mistake — passes.

Concretely: `rat_add_ref.body`'s genome names `cell:fc5403…`, which is the very
cell the seal seals, and nothing objects.

The negative control for mutant 14 is a body naming `prim:eq` and `prim:case`.
It is accepted, so the check is not degenerate. It is just narrow.

---

### F11 · `v23_execute` is the Phase 2 tautology under a new name

F8 of the Phase 2 review reported `v23_holds` comparing coding hashes that never
contain alleles. P21-12's done-when was *"V23 is tested as `fold` then `unfold`
then **execute**, not as a hash comparison that holds vacuously."* The
implementation (`crates/joinn-prim/src/seals.rs:113`) folds, unfolds, compares
two coding hashes — and then:

```rust
match (native.apply(inputs), native.apply(inputs)) {
    (Verdict::Ok(a), Verdict::Ok(b)) if a == b => Verdict::Ok(()),
    …
    _ => Verdict::Refused(refuse("V23: unfolded allele disagreed with itself")),
}
```

It applies the **same oracle twice** and checks that a deterministic function is
deterministic. The engine is never invoked. The unfolded cell is never run. The
function hardcodes `NativeId("add@ℤ")` regardless of which cell it was handed,
and its only caller in the workspace is its own unit test. V41 is asserted
structurally by the type system, which is fine and is not this.

---

### F12 · The harness still chooses its own inputs

2.1's governing insight was that an instrument must not choose the number it
prints, and that rule took — `write_lock`, `perf` and the turn count all obey it.
The rule it did not reach is the one underneath: **an instrument must not choose
the inputs its refusal sees.** Every seal's reference body and its drive bound
were authored together, under a done-when that says "`agree` passes." That is the
same failure at one remove, and F9 is what it produced.

Supporting detail, all of it in the same direction:

- **Sample counts below what the plan set.** `cargo xtask agree` runs `n = 32`.
  Exit gate 2.1 item 1 runs `n = 8` (`xtask/src/main.rs:1009`). P21-08 specified
  256 for `int.add`; P21-10 specified 10 000 for the round-trip.
- **The round-trip does not exist.** There is no `parse(format n) = n` check at
  any sample count anywhere in the workspace.
- **`one_way` is data that nothing reads.** P21-10 asked for "a data declaration
  on the seal that a test reads." The test constructs a `Seal` with
  `one_way: true` and asserts `seal.one_way`. It reads a literal it just wrote.
- **Bespoke per-name branches are back.** `agree` contains `if name ==
  "int.mul"`, `if name == "text.parse_int"` and `if name == "rat.add"`, the last
  two of which `inputs.clear()` and rebuild the map. F3 was a bespoke helper for
  one seal; this is three of them inside the general function.
- **The seal register is not in the crate.** `seal_register()` returns
  `Vec::new()` (`seals.rs:90`). The five real seals are a `specs` array in
  `xtask`. The measuring instrument owns the thing being measured.

---

### F13 · Exit gate 2.1 contains two items that cannot fail, and item 1 passes on hollow references

| Item | What it prints | What it checks |
|---|---|---|
| **9** | `9 ok  gate all records per-gate outcomes` | **Nothing.** `xtask/src/main.rs:1135` is an unconditional `println!` followed by `n += 1`. There is no condition, no comparison, no read of `gates.lock`. This is P21-00's finding — a lock that records intent rather than outcome — reintroduced by P21-21, in the item whose subject is that exact question. |
| **3** | `3 ok  floor paired (8 pairs)` | The guard is `pairs.len() * 2 % 2 == 0` (`main.rs:1031`), which is true for every integer. Harmless in effect, because `check_pairing` has already refused anything unpaired before the guard is reached — but it is a tautology standing inside the anti-tautology phase, and the next reader will not know it is load-bearing or not without checking. |
| **1** | `1 ok  references are bodies; agree executed each on the live engine` | True as written and false as understood. Three of the five execute and assert nothing (F9). |
| **8** | `8 ok  xtask source does not write docs/Findings` | A substring search of `xtask/src/main.rs` for `join("Findings")`. Weak as evidence, but the unit test `xtask_does_not_write_findings` is the real check and it is fine. |

Because item 1 and item 9 pass regardless, `gates.lock`'s `phase 2.1: 9/9`
records nine gates of which seven are met.

---

### F14 · `hash` changed register in the document and not in the code — and an allele now calls it

The Phase 2 review's F8 noted `hash` sitting in **matter** while §2.4 put it in
**physics**. `the-floor.md` §3 now states the physics register plainly:
*"services the body bus offers · the engine only; never an allele."* The code
did not follow. `register_prims` is documented at `floor.rs:371` as exposing
"evaluable matter + physics lookup members," and `sealed_natives()` calls it,
so `hash` is reachable from any allele.

What was a classification drift in Phase 2 is a live instance in Phase 2.1:
`corpus/phase21/int_format_ref.body` contains `prim:hash as hx`. A reference
allele — the most scrutinised kind of allele in the project — names a physics
member, and no check refuses it. Nothing in `check::v33` knows what a register
is; it asks only `floor.contains(name)`, and the floor contains all sixteen.

---

### F15 · `check::turn` is general in shape and hardcoded in frame

F4 is closed for the thing it named: `turn k from S` is handled generally, the
`{1 2}` restriction is gone, and the check lives in `admit_cell`. What remains
narrow is the frame. `check::check` draws every sample from `IntFrame::new()`
(`crates/joinn-gate/src/check/turn.rs`), `sum_ports()` supplies a Sum-shaped
default, and `native_names` carries `if forward == "add@ℤ"` special cases. A turn
declared on a ℚ- or Text-framed cell cannot be checked by this code; it will
either refuse for the wrong reason or sample values from the wrong frame.

This is a smaller problem than F9 and it is the same species: the check is
general along the axis the finding named and specific along the axis it did not.

---

### F16 · Smaller things, in order of how much they mislead

| Where | What |
|---|---|
| `seals.rs:38` / `xtask:526` | `Seal::agreements: Vec<Witness>` is documented as "recorded agreements." Nothing ever pushes to it. Every seal in the workspace carries `Vec::new()`. A witness corpus that is never written is worse than none, because the field implies one exists. |
| `seals.rs:90` | `seal_register()` returns `Vec::new()` and is publicly exported. Dead, and misleadingly named. |
| `check/v33.rs:9` | V33's refusals carry `CheckId::Contract` while their text says `check::v33`. `power` prints the text, so the mutant table reads correctly and the verdict's own field does not. |
| `seals.rs` `refuse()` | Every refusal from the seal layer is built with `CheckId::Laws` and `Subject::Allele(String::new())` — an empty subject. A refusal that does not name its subject is half a refusal. |
| `xtask::line_has` | Still splits on `!is_ascii_alphanumeric() && c != '_'`, so `_` is a word character. P21-19's done-when described fixing this; what was actually fixed was V38, which now uses `line.contains` on a separate path. `line_has` survives for the `sub`/`subtract`/`minus` ban, where word-boundary matching is defensible — but `fn sub_total()` would pass it. |
| `gate 2.1` item 4 | Uses `wrapping_caught_by_agree`, which is `#[cfg(any(test, feature = "mutants"))]`-gated and calls `agree_one` with hand-passed port indices. Correct today; it is the shape that became F3. |

---

## Addendum — two findings made while drafting the Phase 2.2 plan

Added later the same day, September 18, 2026. Numbered after F16 rather than
sorted into the worst-first order above, so that an F-number stays stable once it
is written down. By severity F17 belongs between F10 and F11, and F18 beside it.

### F17 · The corpus contains a cell whose stated law is false, and `corpus verify` cannot tell

`corpus/phase21/mul.cell` was added by P21-11 and carries:

```
laws {
  identity: forall a:ℤ 1. self@2(0: a, 1: ℤ 1.zero) = a
}
```

For multiplication that law is false: `a × 0 = 0`, not `a`. The cell also carries
its own allele, `native mul@ℤ`, and its hash `4a37b2e1…` is a golden in
`corpus/hashes.txt`.

**Probe.** Parsing the file and putting it to the gate:

```
PROBE: mul.cell ADMITTED
PROBE: mul allele REFUSED: law identity does not hold on a sampled tuple
PROBE: rat_sum.cell ADMITTED
PROBE: rat_sum allele ADMITTED
```

So the gate *can* see it, and nothing asks. `Gate::admit_cell`
(`crates/joinn-gate/src/gate.rs:68`) runs contract, extension and turns; laws and
witnesses are judged only by `admit_allele`, and no code path puts `mul.cell`'s
allele to `admit_allele`. `corpus verify` compares twenty-one hashes and asks
nothing about admissibility, so a cell is a golden on the strength of hashing
consistently — which a false cell does as reliably as a true one.

The phase's own standard makes this worse than a typo. A law is the cell's claim
about itself, and `mul.cell` is the first cell in the project whose claim is
untrue. It was checked in under a done-when that asked for a `.cell` file to
exist.

### F18 · The `mul` seal names the wrong cell

`load_seals` maps each seal's sealed native to a cell hash
(`xtask/src/main.rs:505`), and the arm is:

```rust
let cell_h = match *sealed {
    "parse@Text" => cli_h,
    "format@ℤ"   => format_h,
    "add@ℚ"      => rat_h,
    _            => sum_h,          // ← add@ℤ *and* mul@ℤ
};
```

So `Seal { sealed: mul@ℤ, cell: 6b32… }` — the **Sum** cell. `int_mul_ref.body`
agrees: its genome opens `cell:6b32… as rec, self`, declaring the multiplication
reference to be an allele of Sum. `mul.cell` is parsed, hashed, inserted into the
`cells` map, and sealed by nothing.

Today this is invisible, because `agree` compares against `MulIntSealed` directly
and the products match. It stops being invisible the moment anything structural
is built on `Seal::cell`: `fold`/`unfold` for the mul seal rewrite Sum's allele,
and the transitive genome walk F10 asks for would start from the wrong cell and
conclude the wrong thing. It is also the reason F17 went unnoticed — the only
cell that declares `mul@ℤ` is the one cell in `corpus/phase21/` that nothing
points at.

---

## What to do

**F9 and F12 are one knot, and it is smaller than F1–F4 were.** Two rules close
it mechanically, and neither requires new theory:

1. **`bound: 0` is a refusal at seal registration, exactly like `drives: None`.**
   `register_seal` already refuses a missing drive; a drive that admits one value
   is not a bound, it is a blind spot, and it should refuse in the same line with
   the same kind of message. This alone turns F9 into three red builds.

2. **Generalise §5.2's negative control from mutants to `agree` itself.** Every
   seal registers a deliberately broken reference alongside its real one, and the
   harness fails the build if it cannot tell them apart **at the declared drive
   bound**. A body that ignores `b` is indistinguishable from a correct one at
   `b = 0`, so this test refuses `rat_add_ref`'s bound the day the bound is
   written, without anyone having to notice that the body is one wire long.

   This answers **R37** empirically: yes, the practice generalises, and the case
   that proves it is in this repo.

**R35 is the better long-term form of the same fix** and has been promoted from
a nicety by F9. If `drives` is derived from the reference body's termination
measure, the author cannot set it, and a reference body that cannot state its
measure is one that might not terminate — which is the thing the bound existed
to prevent in the first place.

**Independent of that, and cheap:**

- Implement §3.4's clauses 1–3 in `check::v33`: walk `cell:` entries
  transitively, refuse a genome that reaches its own sealed allele by any path,
  and refuse a cycle in the seal graph at registration. That is V40 and V42.
- Make `v23_execute` run the unfolded cell on the live engine and compare against
  the sealed allele, or delete it. A function whose only caller is its own test,
  comparing a value to itself, should not carry an invariant number.
- Write the `parse(format n) = n` round-trip, at the sample count P21-10 named,
  and make `one_way` a property the round-trip's declared hole is read from
  rather than a bool a test echoes.
- Raise `agree` to the counts the plan set — 256 for `int.add`, and the gate's
  item 1 to at least what `xtask agree` runs, rather than 8.
- Move the seal table out of `xtask` into `joinn-prim::seal_register()` so the
  instrument stops owning its subject, and delete the three per-name branches in
  `agree` in the same commit.
- Refuse an allele that names a physics-register member, and move `hash` out of
  `register_prims`' allele-visible set. Then fix `int_format_ref`, which will
  stop parsing.
- Make exit gate item 9 read `gates.lock` and compare it against the outcomes
  just computed, or delete the item. Replace item 3's parity guard with the
  refusal `check_pairing` already produces.
- Parameterise `check::turn` by the cell's frame.

**And before any of it: drop the lock.** `gates.lock` should fall to the gates
actually met, and exit-gate item 1 should fail, before the first fix lands. P21-00
established the discipline — *"the repo should be honest before it is fixed, so
that nothing in the middle of this phase can be mistaken for a pass"* — and it is
the one part of Phase 2.1's method that is unambiguously worth repeating.

---

## One note on the shape of all this

Phase 2 failed because its done-whens ended in numbers the tools chose. Phase 2.1
fixed that, and the fix held: `write_lock` records outcomes, `perf` prints and
concludes nothing, the turn count is derived, no instrument writes into
`Findings/`, and every mutant names its check and carries a control. Those are
real gains and they are not going to have to be made again.

What 2.1 did not see is that a refusal is only as strong as the inputs it is
shown. The plan moved authority over the *verdict* away from the instrument and
left authority over the *sample* exactly where it was — with the same author, in
the same commit, under a done-when that rewards agreement. `drives` was
introduced as a safety bound and became the knob that decides what the harness is
allowed to notice. Three times out of five it was turned to zero.

That is not a worse failure than Phase 2's. It is a more specific one, which is
what a correction phase is supposed to produce. The next rule, if any of this
lands, belongs beside rule 17 in `AGENTS.md`:

**No instrument chooses its own inputs.** A bound that narrows what a check can
see is part of the check, and it needs a control that fails when the bound is
too tight — or it needs to be derived from something the author does not hold.

---

*Phase 2.1 review. Commands run and quoted; experiments named and reverted.
Findings F9–F16 continue the numbering of `phase-2-review.md`. Exit gate 2.1
item 9 was not reproduced offline from a clean checkout; see Method.*
