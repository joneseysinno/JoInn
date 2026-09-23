# Phase 2.2 review — are we ready for Phase 3?

Reviewer: Claude · September 18, 2026 · against `docs/Plans/JoInn Phase 2.2 Implementation Plan.md`

**Method.** This review ran the commands. A copy of `D:\JoInn\joinn` as of
2026-09-18 20:18 (the `gates.lock` mtime) was built and exercised on Linux:
`cargo build --workspace`, `cargo test --workspace`, `cargo fmt --all -- --check`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo xtask gate all`,
`vocab`, `floor`, `agree`, `power`, `corpus verify`, `perf`, and
`joinn run calculator` via the gate. Three **experiments** were run — a planted
`fn sub_total()`, every `drive_bound` set to `1`, and two unknown `xtask`
subcommands — each reverted immediately. Every claim below is either quoted from
a command's output or readable in the source without running it.

Five limits, stated so they are not mistaken for coverage:

1. Linux container, not the Windows machine. Exit gate 2.2 item 9's *"from a
   clean checkout, offline"* was reproduced only in its second half:
   dependencies were fetched once against the committed `Cargo.lock`, then every
   command ran `--offline`.
2. **No git history was available.** The desktop bridge in this session has no
   shell, so the repo was read file-by-file. Commit boundaries, commit messages
   and the P22-xx ordering are reported from the *state of the tree*, not from
   the log. Where the plan required a commit message to name something (P22-02's
   deleted items), that could not be checked.
3. `joinn-live/src/lib.rs` (68 KB), `joinn-dna/src/parse.rs` (39 KB) and
   `joinn-dna/src/body.rs` (35 KB) were searched, not read end to end.
4. `corpus/mutants/` and `crates/joinn-gate/src/natives/` listed as empty
   directories through the bridge; if they hold ignored files this review did not
   see them.
5. Findings continue the numbering. F9–F18 are Phase 2.1's. **F19 begins here.**

---

## Verdict

**Phase 2.2 landed, and it landed for the right reason. The gates are real this
time — I tried to fool one and it refused me. Ship Phase 3 after one short
hygiene commit, not before.**

The one idea worked. `agree` no longer reports agreement on a body that computes
nothing, and it does not merely *say* so — setting every `drive_bound` to `1` and
re-running produced:

```
BLIND SEAL format@ℤ: counterfeit body f9d6dbb5… agreed on all 256 samples
  drive port 0 bound 1 admits 3 values; widen it or the harness sees nothing
```

That is the message the repo could not produce in September, produced on demand,
naming the seal, the port, the bound and the size of the domain it admits. It is
the best refusal in this codebase and it is worth more than the nine ticks on
gate 2.2.

`gates.lock` reads `phase 2: 8/8 · phase 2.1: 8/8 · phase 2.2: 9/9`, and unlike
Phase 2.1's `9/9`, **this review could not find a ninth tick that means nothing.**
It found one tick that means less than it says (F20), three that say the same
thing three times (F21), and a risk the plan itself predicted arriving on
schedule and unrecorded (F22). None of those is the Phase 2 or Phase 2.1
failure. The phase did not correct the appearance.

What is not ready is duller and more annoying: **the repo's own CI would be red.**
`cargo fmt --all -- --check` reports 46 diffs and `cargo clippy --workspace
--all-targets -- -D warnings` fails with two errors, one of which is a `&& false`
sitting inside the floor's opposition property check. `ci.yml` runs both *before*
it runs `gate all`. So the phase whose standing rule says *"a number is not done
if the code that printed it also chose it"* shipped a green `gates.lock` on a
tree where the first two CI steps never pass. That is F19, it is twenty minutes
of work, and it should be fixed before a new phase adds three crates to the
surface it applies to.

---

## What is genuinely there

Run, not read. This is the larger part.

| Commit | State |
|---|---|
| **P22-00** honest lock, `AGENTS.md` | Done. `AGENTS.md` and `.cursor/rules/joinn.mdc` are Appendix A verbatim, 23 hard rules, `.mdc` differing only by front-matter. Gate 2.1's item 9 is gone; 2.1 is now an 8-item table. |
| **P22-01** blind seals written down | `surviving-mutants.md` opens all three under their own heading with drive port, bound, seed 1, and the exact counter-examples (`expected -31, got 0`; `expected "-3", got "0"`; the ℚ pair). Opened and closed the same day, which is honest and matches the tree. |
| **P22-02** opposed gate items | Real. `GateItem { name, check: fn() -> bool, control: fn() -> bool }` in `joinn-gate/src/gate.rs`; `run_opposed` runs the **control first** and returns `Err("gate item N (name) control passed")`, aborting the whole gate. Gates 1, 2.1 and 2.2 are tables. |
| **P22-03** `bound` cannot be zero | `Drive::bound: NonZeroU32`, `Seal::drives: Drive` (no longer `Option`), and `sample_port`'s `bound == 0` branches are gone — the literal `"0"` and `0/1` special cases do not exist in `seals.rs`. See F20 for the half that did not land. |
| **P22-04** counterfeits | `Seal::counterfeit: BodyRef` is a required field. `agree` fires both bodies over the same samples and emits `BLIND SEAL` naming the drive port, the bound, and `admits N values`. Verified live by the bound-1 experiment above. |
| **P22-05/06/07** three real references | `corpus/phase22/text_parse_ref.body` (12 KB, walks `empty`/`cons`/`chr`), `int_format_ref.body` (2.1 KB, **no `prim:hash`**, repeated `Sum` at a turn), `rat_add_ref.body` (cross-multiply, genome names only the `int.mul` and `int.add` references and `build` — **no gcd**). All five references agree with their sealed alleles on 256 generator-drawn samples at seed 1. |
| **P22-08** round-trip | `parse(format n) = n` on 10 000 samples, printed, with `hole "007" named (one_way read from the parse seal)`. |
| **P22-09** V33 complete | `check::v33` walks `cell:` entries: unknown cell refused, a cell that names the sealed native refused **with the path**, a cell without a justifying seal refused, the seal graph checked for cycles, grants refused. The self-cell exemption (`if *h == self_cell { continue }`) is there, which is what makes the walk writable at all. V52 and V53 delivered. |
| **P22-10** registers enforced | `Register::{Matter, Space, Physics}` on `Prim`. `cargo xtask floor` prints eight pairs *with* registers: `build↔case matter`, `hash↔resolve physics`, `bind↔unbind space`, … A body naming `prim:hash` is refused; a body naming `prim:case` is accepted. |
| **P22-11** corpus admitted | `cargo xtask corpus verify` → `refused false_law.cell by name` / `21 hash(es) match; cells admitted`. `mul.cell`'s law is now `annihilator`, and the hash move `4a37… → 12b6e545…` is recorded in **two** dated findings with the reason. That is the rule 6 distinction honored rather than quoted. |
| **P22-12** seal finds its cell | `seal_register()` lives in `joinn-prim` and returns five real `SealSpec`s. `find_cell_for_native` refuses on zero matches and on two. The `mul` seal names `12b6…`, asserted by prefix in gate item 7. F18 is closed. |
| **P22-13** V23 executes | `v23_execute_dna` fires the body on the live engine, refuses on `last_steps() == 0`, and compares DNA output to the sealed oracle. The hardcoded `NativeId("add@ℤ")` is gone from the check; a wrong unfold refuses. |
| **P22-14** subtractions | `Seal::agreements` is **deleted** — `grep -rn agreements crates xtask` returns nothing. `CheckId::V33` exists and `power` prints `14 mutant.sealed_as_reference refused by check::v33`. `corpus/testimony/.gitkeep` is still the only thing in that directory. |
| **P22-15** turn by frame | `check::turn::check(..., frame: &dyn Frame)` — every sample is drawn from the passed frame. No `add@ℤ` string appears in `turn.rs`. F15's named axis is closed. |
| **P22-17** vocab lint | **Experiment:** appended `fn sub_total() {}` to `joinn-live/src/lib.rs`; `cargo xtask vocab` → `1 hit(s) … turn-ident sub`, exit 1. Reverted. |
| **P22-18/19** re-freeze and gate | `phase-2.2-hashes.md` records five moved hashes with old value, new value, commit and reason. `cargo xtask gate all` runs phases 0, 1, 2, 2.1, 2.2 offline and writes a `gates.lock` **byte-identical to the one in the tree**. |

**Numbers as printed.** `cargo test --workspace` → **77 passed, 0 failed, 0 ignored**
(71 at Phase 2.1). `power` → **20/20**, each mutant naming its check. `agree` →
five seals × 256 samples + 10 000 round-trip samples + the injected disagreement
refused. `corpus verify` → 21. `floor` → 16 members, 8 pairs, registers printed.
`perf` → calculator 5 steps / 2 ms, 50-cell 101 steps / 2 ms, reference
`add(0, 64)` 64 steps / 41 ms. `gate all` → **1086 seconds**.

---

## Findings, worst first

### F19 · The repo's own CI is red, and no gate item looks at it

`.github/workflows/ci.yml` runs, in order: `fmt`, `clippy`, `test`, `vocab`,
`agree`, `gate all`, `power`. Steps 3–7 pass. **Steps 1 and 2 do not.**

```
$ cargo fmt --all -- --check      → exit 1, 46 diffs
$ cargo clippy --workspace --all-targets -- -D warnings → exit 101
error: this boolean expression contains a logic bug
   --> crates/joinn-prim/src/floor.rs:616:20
616 |                 if i as u32 >= n * 3 && false {
error: manual implementation of `.is_multiple_of()`
   --> crates/joinn-prim/src/floor.rs:505:8
```

The formatting diffs are spread over `joinn-dna/src/body.rs`,
`joinn-prim/src/seals.rs`, `joinn-prim/src/lib.rs` and `xtask/src/main.rs` — the
files Phase 2.2 touched most. `use crate::model::{Cell};` at `body.rs:2` is the
flavour.

The clippy error is the one that matters, and not because clippy said so.
`floor.rs:616` sits inside `check_one`'s `"build" | "case"` arm — the property
check that proves `build(frame, case(v)) = v` for the floor's own eliminator
pair. The guard `if i as u32 >= n * 3 && false { break; }` is a loop bound that
**can never fire**. It is harmless in effect (the iterator ends at `n * 3`
anyway), and that is exactly what makes it worth naming: *it is a condition with
an empty domain, inside the floor's own opposition check, in the phase whose
single idea was that a check with an empty domain prints the same word as a check
that passed.* F9 was a sampling bound pinned to one value; this is a break
condition pinned to `false`. Same species, one floor down.

`overly_complex_bool_expr` is deny-by-default, so this fails on any recent
clippy, not only the container's 1.95. Whatever ran during Phase 2.2, `cargo
clippy -- -D warnings` was not it.

**What to do.** `cargo fmt --all`; delete the `&& false`; fix the modulo. Then
decide whether `gate all` should refuse to write the lock when `fmt`/`clippy`
have not been run — the honest version of that is a gate item whose *control* is
a deliberately misformatted file in a scratch directory, and it is probably not
worth the machinery. The cheaper answer is that CI is the instrument, and CI must
actually be green before a lock is believed.

---

### F20 · Gate item 3 is opposed by a fact about the standard library

Item 3 is named *"a bound cannot be empty."* Here it is, whole:

```rust
fn p22_bound() -> bool {
    std::num::NonZeroU32::new(0).is_none()
        && joinn_prim::seal_register().iter().all(|s| s.drive_bound != 0)
}
fn p22_bound_control() -> bool {
    std::num::NonZeroU32::new(0).is_some()
}
```

The control is `false` for every Rust program ever compiled. It cannot notice
anything about JoInn. The unit test that backs the item —
`seals.rs::drive_bound_is_nonzero_by_type` — asserts
`NonZeroU32::new(0).is_none()` and `NonZeroU32::new(1).is_some()`, which is the
same two facts about the standard library, written twice.

The plan's stated control was *"the workspace does not compile with `bound: 0`."*
That is not what is built. `SealSpec::drive_bound` is a plain `u32`, so a `0` in
the register **compiles**; it is refused at runtime by `xtask::nz` →
`"drive bound 0 is not a bound"` when `load_seals` builds the `Drive`. That is a
real refusal and it is fine — but it is a *runtime* refusal, and item 3 neither
exercises it nor opposes it. The check's second conjunct reads the literals in
the table; the control reads the stdlib. Nothing in gate 2.2 would notice if
`xtask::nz` stopped refusing.

Second half, smaller: `joinn-prim` carries its own private

```rust
fn nz(n: u32) -> NonZeroU32 { match NonZeroU32::new(n) { Some(v) => v, None => NonZeroU32::MIN } }
```

which silently launders `0` into `1`. Today it is only ever called with literals
(`nz(32)`, `nz(1)`), so nothing is wrong in fact. It is a zero-to-one conversion
living in the crate whose phase existed to ban one, and it will be reached for
the first time someone plumbs a variable through it.

**What to do.** Make the control an artifact: a `SealSpec` with `drive_bound: 0`
handed to the seal-loading path, refused, naming the seal. Delete the stdlib
assertions or replace them with that refusal. Either delete `joinn-prim::nz` or
make it return `Verdict`.

---

### F21 · Three of gate 2.2's nine items are the same check, and the gate now takes eighteen minutes

```rust
fn p22_see()       -> bool { agree().is_ok() }
fn p22_true()      -> bool { agree().is_ok() }
fn p22_roundtrip() -> bool { agree().is_ok() }
```

Items 1, 2 and 8. Nine items, **six distinct checks.** Their *controls* do differ
— counterfeit ≡ reference, injected disagreement admitted, a lossy pair declared
`one_way: false` — so the opposition is genuinely three-fold and this is not a
tautology. What misleads is the count: `phase 2.2: 9/9` reads as nine
independent confirmations and is six.

The cost is measurable. `cargo xtask gate all` took **1086 seconds** — a little
over eighteen minutes — and the log shows the 256-sample harness printed **four
times** (once for gate 2, three times inside gate 2.2), plus each item's control
run. This is the number to watch, not because eighteen minutes is unaffordable
but because Phase 3 adds a second host to every transcript item and the honest
answer to *"did I break V28?"* has to stay a command someone will actually run.

**What to do.** Collapse 1, 2 and 8 into one item with three controls, or compute
`agree()` once per gate invocation and let the three items read the result.
Either way, say in the item name what the item is opposing, since that is now
the only thing distinguishing them.

---

### F22 · Every counterfeit is the same counterfeit, and the plan's own alarm has fired unrecorded

Four of the five counterfeit bodies are, in full:

```
genome  cell:<the cell> as self
wires   self@0 -> self@2
```

Forward the first input. `int_add.body`, `int_mul.body`, `rat_add.body` and
(modulo two extra prims) `text_parse.body` are the same trick. Only
`int_format.body` is different, and only because it is the harvested
`prim:hash` body.

`agree` prints the separating sample, and it is `0` or `1` for all five:

```
add@ℤ:    counterfeit separated at sample 1
parse@Text: counterfeit separated at sample 1
format@ℤ:  counterfeit separated at sample 0
mul@ℤ:     counterfeit separated at sample 0
add@ℚ:     counterfeit separated at sample 0
```

§8 of the plan, the row headed *"The stated adversary · a control chosen weak"*:

> If the separating sample is the first sample for every seed, the counterfeit is
> too crude. Write that in `Findings/` and make it subtler; do not raise the
> sample count to hide it.

It is the first sample for every seal. **No such finding exists in
`docs/Findings/`.** The sample count was not raised to hide it — nothing was
done at all, which is the milder failure but the same one: the instrument
reported the condition its own plan said to write down, and the report was not
read.

**Experiment, to say how much this costs.** Every `drive_bound` in
`seal_register()` set to `1` — the narrowest bound the type permits — and `agree`
re-run: **four of five seals still separate their counterfeit.** Only `format@ℤ`
goes `BLIND SEAL`. So for four seals out of five, the counterfeit places
essentially no constraint on the bound the author chose. The knob F9 was about is
still, for 80% of the register, the author's.

This is **R39** arriving on schedule and it is not a failure of the phase — the
mechanism is right, it is the only mechanism in the repo that could ever catch
this, and it did catch `format@ℤ`. What is missing is the finding, and after it,
one subtler counterfeit per seal: a body that is correct except on a carry, or on
a negative, or on the empty string. A counterfeit separated at sample 200 would
be evidence the bound is doing something. A counterfeit separated at sample 0 is
evidence the sealed allele returns a different frame.

**What to do.** Write `docs/Findings/counterfeit-strength.md` with the five
separating samples and the bound-1 experiment, opened, no date closed, under
R39. Then one subtler counterfeit for `add@ℤ` as the proof of concept, before
Phase 3 rather than during it.

---

### F23 · `joinn-gate` writes files, and the testimony drawer outlived the field that implied it

Standing rule 15: *"`std::io` appears only in `joinn-run`."*
`grep -rln "std::io" crates xtask` returns three files, and one of them is
`crates/joinn-gate/src/testimony.rs`:

```rust
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

fn write_line(dir: &Path, cell: &Hash, record: &str) -> io::Result<()> {
    fs::create_dir_all(dir)?;
    let mut f = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(f, "{record}")
}
```

`cargo xtask vocab` does not check rule 15, so nothing noticed.

The second half is the interesting one. §2.8 deleted `Seal::agreements` on the
grounds that *"a field that implies a corpus exists is worse than no field"* and
that designing testimony inside a correction phase is how a registry gets built
by a scaffold. That deletion happened. Meanwhile `TestimonyStore::at(dir)` — a
file-backed, append-only writer aimed at `corpus/testimony/<hash>.log`, the exact
path §2.8 said must stay empty until Phase 12 — is already written, already
compiled, has **no caller anywhere in the workspace** (`Gate::new` uses
`TestimonyStore::memory()`), and has no test. The field was removed and the
drawer was left open.

**What to do.** One of: move `write_line` and `TestimonyStore::at` out of
`joinn-gate` and into `xtask` until Phase 12 asks the question properly; or
amend rule 15 to name the two crates that may do IO and give `vocab` a check for
it. Not both, and not neither — three crates doing IO with a rule that says one
is how the rule stops being read.

---

### F24 · Smaller things, in order of how much they mislead

| Where | What |
|---|---|
| `xtask/src/main.rs` dispatch | An **unrecognised subcommand prints usage and exits 0.** `cargo xtask nonesuch` → exit 0. `cargo xtask gate 2.3` correctly exits 1, so the hole is only at the top level — but `agre` for `agree` in a CI file is a silently skipped step and a green build. One line. |
| `README.md` | Still says *"Phase 2.1 is frozen when `gates.lock` records `phase 2: 8/8 · phase 2.1: 9/9`."* 2.1 is `8/8` now, by design (P22-00 deleted item 9). Phase 2.2 is not mentioned at all. The README is where a stranger reads what is true. |
| gate 2.2 item 9 | Named *"the path of truth."* What it checks is `run_opposed` on a synthetic two-line probe — real, but it is a self-test of the runner, not the item's own claim that `gate all` *"writes the lock from per-gate outcomes."* That claim is covered by observation and by `write_lock_has_no_literal_gate_score`, not by item 9. Better than 2.1's unconditional `println!`; still the weakest item after item 3. |
| `seals.rs` round-trip | `agree` still branches on `s.sealed.0 == "parse@Text"` and `== "format@ℤ"`. P22-12's done-when was *"`agree` contains no `if name ==`."* Defensible — the round-trip needs a named pair — but it is the last per-name branch, and the pair wants to be declared on the seal rather than found by string. |
| `check/v33.rs` register message | The refusal names the register by re-deriving it from a name list: `if name == "hash" \|\| name == "resolve" { "physics" } else { "space" }`. Everywhere else the register is `Prim::register()`. The caller passes only the flattened `matter` set, so the check cannot ask. A third space member added later will be refused as "space" by luck. |
| `seals.rs::refuse` | Still builds `CheckId::Laws` with `Subject::Allele(String::new())` — F16's fourth row. `refuse_seal` now names the subject, and V33 now carries `CheckId::V33`, so this is the residue rather than the whole finding. |
| `v23_execute` | Ends with `native.apply(inputs)` whose result is discarded; `v23_execute_dna` then applies the same oracle again for `want`. F11 is genuinely closed — the comparison is now DNA-vs-native with a nonzero step count — but the vestigial apply is a leftover of the tautology and reads like one. |
| `corpus/mutants/`, `crates/joinn-gate/src/natives/` | Both list as empty directories. `natives.rs` is a single file and needs no `natives/`. Empty directories in a repo that content-addresses everything are noise. |

---

## Are we ready for Phase 3?

**Yes — after a hygiene commit, and with three inherited facts written into the
Phase 3 plan before it starts.**

The roadmap's Phase 3 is the host protocol, `joinn-host`, `joinn-cli`,
`joinn-test-host`, and an exit gate that says *the identical calculator body,
unchanged and un-recompiled, runs under both hosts and produces the same results
and the same cell-level descriptions.* Its preconditions are met:

- Phase 2's gate passes, verified, including the byte-exact transcript and the
  `calculator.trace` replay.
- **V28 holds.** `gate all` runs phases 0, 1, 2, 2.1 and 2.2 in one command and
  every earlier gate still passes. No golden moved except the five recorded on
  purpose.
- **G4 (determinism)** is enforced, not asserted: `power` refuses
  `mutant.arrival_order` by `check::grant` and `mutant.ungranted` by the same.
- **G5 (join)** is in the contract and hashed: `join refuse` appears in the cell
  text, and `mutant.drop_second` is refused by `check::join`.
- The floor is closed, registered and paired; the engine does not recurse; the
  budget is global; nesting holds no grants.

Three things Phase 3 inherits that nobody has written down yet:

**1. The Phase 2 exit gate is wired to the binary Phase 3 deletes.**
`xtask::run_calculator_bin` spawns `cargo run -p joinn-run -- run calculator` and
pipes `"two\n2\n3\n"` at it. Gate 2's transcript item and gate 2.1's item 7 both
go through it. Rule 14 says `joinn-run` is deleted at the *start* of Phase 3.
V28 says the transcript passes at every later gate, forever. So **Phase 3's first
commit is moving that spawn to `joinn-cli` with byte-identical output**, and the
transcript must be re-checked before `joinn-run` is removed, not after. Written
in the wrong order this is a day lost and a re-freeze nobody wanted.

**2. The host protocol already exists, badly, in 30 lines of `joinn-run`.**
`prompt()`, `present()` and `calculator_session()` are the embryo. The good news
is what they touch: `present` reads `body.regulatory.present` and substitutes
`{0} {1} {2}`, and `prompt` reads `body.regulatory.prompts` — **regulatory region
only, no coding region, no frame knowledge.** That is the upward wrap behaving
correctly before anyone designed it, and it is evidence for Phase 3's exit gate
rather than against it. The bad news is `calculator_session`, which hard-codes the
whole dialogue: read a line, inject at `cli_a`, run, *require* a refusal, read two
more, inject both, run, present. Turning that script into raw input → intent →
address → message is the actual work of Phase 3, and the repo has no notion of an
intent today.

**3. Phase 3's adversary can be tested on day one, for free.** The roadmap's
stated adversary is *"`present` turns out to need to know something about its
host after all."* The current `present` needs exactly one thing the CLI happens to
supply: a template string with positional holes. A headless test host that
captures descriptions rather than text will find out immediately whether that
template is a host-independent description or a CLI format string in disguise.
Build `joinn-test-host` **before** `joinn-cli` is finished, not after — the
degenerate host is the one that exposes the assumption, and the roadmap already
says as much about the environment body's nearly-empty signal set.

### The hygiene commit, concretely

Six items, none of them a design decision. Call it P22-20…P22-25 or Phase 2.3; it
does not need a plan document.

1. `cargo fmt --all`. Delete `&& false` at `floor.rs:616`. Fix
   `manual_is_multiple_of` at `floor.rs:505`. Get `cargo clippy --workspace
   --all-targets -- -D warnings` to exit 0. **(F19)**
2. Give gate 2.2 item 3 an artifact control: a `SealSpec` with `drive_bound: 0`
   refused by the seal-loading path, naming the seal. Delete the two stdlib
   assertions. **(F20)**
3. Write `docs/Findings/counterfeit-strength.md`: the five separating samples,
   the bound-1 experiment, R39 opened with no date closed. Then one subtler
   counterfeit for `add@ℤ`. **(F22)**
4. Collapse gate 2.2 items 1/2/8 into one item with three controls, or memoize
   `agree()` per gate run. **(F21)**
5. Decide rule 15 and enforce whichever way it goes in `xtask vocab`. **(F23)**
6. Unknown `xtask` subcommand exits non-zero; README updated to 2.1 `8/8` and
   Phase 2.2. **(F24)**

Items 1, 2 and 6 are an evening. Item 3 is the one that matters for the long run
and it is mostly typing. Items 4 and 5 can slip into Phase 3's first commits if
they must.

---

## One note on the shape of all this

Phase 2.1's failure was *a check with an empty domain.* Phase 2.2's answer was
*every check declares a control.* The thing this review found is that **a control
is a check, and it has a domain too** — so the answer regresses one level, and
the hole moves up rather than closing.

Look at which controls worked and which did not, and the pattern is not about
care:

| Item | Its control | Did it constrain anything? |
|---|---|---|
| 1 · every seal can see | a **body** whose counterfeit equals its reference | yes — and it caught `format@ℤ` under experiment |
| 6 · the corpus is true | a **file**, `false_law.cell`, refused by name | yes |
| 4 · V33 walks the genome | a **body** whose genome reaches its sealed native | yes |
| 3 · a bound cannot be empty | a **boolean expression**, `NonZeroU32::new(0).is_some()` | no |
| 9 · the path of truth | a **synthetic probe** built in the item | barely |

Every control that is an *artifact* — a file in the corpus, a body with a hash, a
planted malformation — constrains something. Every control that is a *predicate
written beside the check* constrains nothing, because the same hand that wrote the
check chose it, which is F9's disease with the symptom changed exactly as the
plan warned it would be. The floor's pairing lint works for the same reason:
`check_pairing` refuses three malformations *of the register*, not three boolean
expressions.

So the standing rule worth adding for Phase 3 is narrower than rule 19 and does
more work:

> **A control is an artifact, not a predicate.** It is a file, a body, a cell or a
> value the gate item points at, and it lives in the corpus where a reviewer can
> read it. An item whose control is an expression written beside its check is not
> opposed; it is asserted twice.

That rule would have caught F9 and F20 with the same reading, and it costs one
line in `AGENTS.md`.

And a partial answer to **R43**, *"must every refusal name what it would have
accepted?"* — the best refusal in this repo does exactly that:

```
drive port 0 bound 1 admits 3 values; widen it or the harness sees nothing
```

It names the failure, the measurement, and the direction of the fix. Compare
`"V33: reference body holds grants"`, which names the failure and leaves the
reader to infer the other two. The difference is not severity. It is that the
first refusal knows what its own acceptance would look like. That is worth
promoting from an accident of one message to a shape all refusals take, and it is
cheap to try on the seal layer in the hygiene commit.

---

*Phase 2.2 review, September 18, 2026. Continues the F-numbering at F19. Written
against the tree as of `gates.lock` mtime 2026-09-18 20:18 UTC. Every experiment
was reverted; no file on the build machine was modified.*
