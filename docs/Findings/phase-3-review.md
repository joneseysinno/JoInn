# Phase 3 review — are we ready for Phase 4?

Reviewer: Claude · September 22, 2026 · against `docs/Plans/JoInn Phase 3 Implementation Plan.md`

**Method, and it is weaker than last time.** This review **did not run the
commands.** The desktop bridge in this session has no shell and no git history,
so `D:\JoInn` was read file by file: every source file under
`crates/joinn-host`, `crates/joinn-cli`, `crates/joinn-test-host`, the whole of
`xtask/src/fns` by listing and the Phase 3 files by content, `corpus/phase3/`,
`corpus/descriptions/`, `gates.lock`, `AGENTS.md`, `Cargo.toml`, `ci.yml`,
`hashes.txt`, and the five findings the phase produced. **No command was
executed, no experiment was planted, and nothing was reverted because nothing
was changed.** Every claim below is readable in the source; none is quoted from
a command's output.

That inversion matters and it is the first finding in spirit if not in
numbering. The Phase 2.2 review could say *"I tried to fool one and it refused
me."* This review can only say *"here is the code, and here is what it would do
if it ran."* Where I assert an item cannot fail, I am reading the branch, not
watching it. The claims are cheap to check and expensive to leave unchecked.

Five further limits:

1. `gates.lock` reads `phase 3: 9/9`. **That number was read from a file, not
   reproduced.** The whole of §2 below is about four of those nine ticks, and
   none of it depends on the run — but the other five are unverified in a way
   Phase 2.2's were not.
2. Commit boundaries and the P3-xx ordering are reported from the state of the
   tree and from mtimes, not from the log. The mtimes say the phase was built
   between 16:15 and 20:12 UTC on 19 September 2026, in one sitting, and has
   not been touched since.
3. `xtask/src/fns/vocab.rs` (8.5 KB) and `gate_three.rs` (9.7 KB) were read end
   to end. `joinn-dna/src/parse.rs`, `body.rs` and the Phase 1/2 gate tables
   were not.
4. The Windows machine's `cargo fmt`, `clippy` and `gate all` results are
   unknown. F19's fix is asserted by the presence of `modules` in `ci.yml`, not
   by a green run.
5. Findings continue the numbering. F19–F24 are Phase 2.2's. **F25 begins
   here.** (An earlier note in this session said F26; that was an off-by-one
   against a review that stops at F24.)

---

## Verdict

**The protocol landed. The instrument did not. Phase 3's one idea is real and
load-bearing; four of gate 3's nine items would print `ok` whether the code
worked or not. Do not open Phase 4 on top of it — and do not open Phase 3.1
either.**

§2.1 worked, and it worked at the level the plan claimed. `Description` is a
value in `joinn-host` with a hand-written canonical form and its own hash tag
(`joinn.description.v1`). `joinn-cli` does not format the universe; it renders a
description through `Host::present` → `fill_present`, substituting
`body.regulatory.present`. The five-line transcript that has been the repo's
witness since Phase 2 is now **produced from a description**, byte for byte, and
`joinn-run` is gone with rule 14 struck from `AGENTS.md` in the same breath. The
sentence *a cell describes itself and the host decides how it looks* is a type
now, three phases before the renderer that consumes it. That is what the phase
was for.

`vocab` is the best instrument the phase built and the best in the repo after
the `BLIND SEAL` message. It walks `crates/` and refuses `std::io`/`std::fs`
outside `/joinn-cli/`, `fn render` inside `/joinn-host/`, and `Description {`
outside `joinn-host` — by path, tree-scoped, `#[cfg(test)]`-aware, and wired
into CI ahead of `gate all`. Rule 15 was false for two phases and nothing
noticed; it is now enforced by a scan that can fail. F23 is closed properly:
`write_line.rs` and `TestimonyStore::at` are deleted, `corpus/testimony/` still
holds only `.gitkeep`, and R41 is untouched.

And `grandfather.txt` is still comments. Zero entries, four days after a rule
that guarantees files keep splitting. §2.8's ban held.

What did not land is the half of the phase that judges the other half. **Gate 3
item 8 — *"every control is an artifact"*, the item that enforces rule 24 across
every gate in the repo — counts non-comment lines in a text file the same commit
wrote, and passes at four.** It reads no gate table. Rule 24 was Phase 2.2's
answer to a failure species that has now changed address four times, and its
enforcement is a line count. That is F25, it is the headline, and it is the
reason the next phase opens with a correction block rather than with new work.

---

## What landed

| Commit | Landed |
|---|---|
| **P3-00/01** | Modularised tree carries `gate all`; `ci.yml` runs fmt, clippy `-D warnings`, test, `vocab`, `modules`, `agree`, `gate all`, `power`. `xtask_writes_findings` rescoped from one file to `xtask/src/**`. F19 addressed — unverified by a run. |
| **P3-02** | `corpus/phase22/drive_bound_zero.sealspec` exists; `nz()` returns a `Result`; `main.rs` carries `bound_zero_artifact_is_refused_by_name` over `p22_bound`/`p22_bound_control`. F20 closed. |
| **P3-03** | `counterfeit-strength.md`, hand-typed, five separating samples, the bound-1 experiment quoted, R39 opened with no date closed. `int_add_carry.body` separates at sample 9. F22 closed as evidence, not as a metric — which is what the finding itself says. |
| **P3-04** | `agree_cached.rs`; `p21_agree`, `p22_roundtrip`, `p22_see`, `p22_true` all read it. F21 closed. |
| **P3-05** | `joinn-host` with `Description`, `PortFace`, `Role`, `describe`, `hash_description`, `print_description`. `role` closed at four. The §3.3 canonical text is exactly what `corpus/descriptions/calculator.desc` holds. |
| **P3-06** | `Intent`, `Address`, `intent_set`, `check_intent`; the refusal names `sum@0`. See **F28**. |
| **P3-07** | `Signals`, `check_signals`, `corpus/phase3/environment.body`, `corpus/phase3/columns_reader.body`. The refusal names `columns`. See **F32**. |
| **P3-08** | `trait Host` with an associated `Raw`; `probe(&BodyState, Address)`. See **F32**. |
| **P3-09** | `joinn-test-host`: scripted `RawEvent`s, one description per fire, the refusal of `"two"` captured with `role refusal`. No IO in the crate. Built before the CLI, as the risk table asked. |
| **P3-10** | `joinn-cli` with `bin joinn`; template and prompts moved out of the protocol; `columns` declared. See **F31**. |
| **P3-11/12** | Spawn repointed to `joinn-cli`; `joinn-run` deleted; rule 14 removed from `AGENTS.md` in the same commit. The ordering held. |
| **P3-13** | `calculator.desc`, `calculator_refusal.desc`, both in `hashes.txt` under `joinn.description.v1`. See **F27**, **F29**. |
| **P3-14** | Rule 15 rewritten and enforced by path. `TestimonyStore::at` and `write_line.rs` deleted. |
| **P3-15** | `present-leaks.md`: the adversary fired and the upward wrap held. `describe` never consults `Signals`; a `Description` has no width. **This is the phase's best finding.** |
| **P3-16** | Four probes in `live-engine-performance.md`, hand-typed, with the honest note that the CLI's 301 ms is process spawn and not engine time. |
| **P3-17** | `decisions.md` and `cargo xtask decisions`; a `reversed` line with no linked finding fails the command. |
| **P3-18** | `phase-3-hashes.md`: no coding hash moved. `sum` and `calculator` unchanged. |
| **P3-19** | `gate 3`, `gates.lock` at `phase 3: 9/9`. See everything below. |

Twenty commits, twenty artifacts. Nothing in the plan was silently dropped.

---

## Findings

### F25 · Rule 24 has no enforcement, and the item that was supposed to provide it counts lines

`gate_three.rs:238–261`:

```rust
let list = fs::read_to_string(&list_path)?;
if list.trim().is_empty() {
    return Err("gate item 8 (every control is an artifact) control passed".into());
}
let mut listed = 0u32;
for line in list.lines() { if !line.is_empty() && !line.starts_with('#') { listed += 1; } }
if listed >= 4 { println!("8 ok  Every control is an artifact"); n += 1; }
```

`artifact_list.txt` has four entries. The item passes. It does not open a gate
table, does not resolve a single one of those four names against the filesystem,
and does not know that gates 1, 2, 2.1 and 2.2 exist. Exit gate 3's own text
says:

> **8 · Every control is an artifact.** Across gates 1, 2, 2.1, 2.2 and 3, every
> item's control points at a file, a body, a cell or a fixture. *Control:* a gate
> item whose control is a bare expression fails the build naming the item.

Neither half is implemented. Adding a tenth item to any gate with a predicate
control changes nothing. **V66 is asserted, not tested.**

This is the fourth address of one species, and the addresses are worth listing
in one place because the pattern is the finding:

| Phase | Address | Shape |
|---|---|---|
| 2.1 (F9) | `Drive::bound` | The instrument chose its own input |
| 2.1 | Several checks | Empty domains |
| 2.2 (F20) | Gate 2.2 item 3 | Control was a fact about `NonZeroU32` |
| **3 (F25)** | **Gate 3 item 8** | **Control is the length of a list the author wrote** |

Every previous fix was at the address. Rule 24 was the first attempt at the
mechanism, and this is the commit where the mechanism was supposed to acquire
teeth. `GateItem` already exists in `joinn-gate/src/gate.rs` as a struct with
`check: fn() -> bool` and `control: fn() -> bool`. The missing field is the
artifact's path, and the missing check is `run_gate_table` resolving it. Rule
15 forbids `std::fs` in `joinn-gate`, which is exactly right: the crate carries
the string, `xtask` opens it. The fix is small and it is structural.

There is a second reason item 8 could not have worked as written: **gate 3 is
not a table.** Gates 1, 2, 2.1 and 2.2 build `&[GateItem]` and hand it to
`run_gate_table`; `gate_three()` is 270 lines of inline `println!` and `n += 1`.
An item that scans gate tables cannot see gate 3 at all.

### F26 · Two of gate 3's nine items do not test their own claim

**Item 1 — "One body, two hosts"** (`gate_three.rs:18–40`) never runs the
calculator. It is:

```rust
let one = matches!(check_signals(&calc, &none), Verdict::Ok(()))
    && matches!(check_signals(&columns_reader, &none), Verdict::Refused(r) if r.reason.contains("columns"));
```

That is the *control* for item 1 as the plan wrote it, promoted to be the check.
The claim — *the identical calculator body, unchanged and un-recompiled, runs
under both hosts and produces the same results* — is the roadmap's Phase 3 exit
gate, the sentence the whole phase is named after, and it is not on the table.

**Item 6 — "Signals are declared"** (`211–223`) is the same two `check_signals`
calls with a second host added. Items 1 and 6 differ by one `Signals::new`. That
is the *same assertion written twice*, which is the phrase rule 24 uses for what
opposition is not.

**Item 9 — "The path of truth"** (`263–280`) reads `gates.lock` and passes on:

```rust
if live.contains("phase 0") && live.contains("phase 2.2")
```

`bad.lock` is compared to the live lock for inequality and then discarded. It
differs in exactly one character — `phase 3: 0/9` against `9/9` — and it would
pass the item unchanged if it were the live lock, because it contains both
strings. The item is named for the claim *"`gate all` writes the lock from
per-gate outcomes"* and checks that the lock contains two substrings. F24
flagged gate 2.2's item 9 as "the weakest item after item 3" for a softer
version of this; it has not improved by being renamed.

Nine items of which one is a duplicate and two check substrings is worse than
six honest items. The count is not the asset.

### F27 · V60 is true by construction, and item 2's "CLI side" is not the CLI

`describe` lives in `joinn-host` and both hosts call it. §2.1 requires that —
*neither host may build a `Description`* — and `vocab` enforces it. The
consequence is that **"two hosts produce byte-identical descriptions" compares
one function's output with itself.** There is no arrangement of the code
consistent with §2.1 under which item 2 can fail for the reason it names.

The plan's own risk table saw this:

> **New · the two hosts share a bug.** *Nothing, and that is the danger.* Two
> hosts written in one week by one agent from one plan will agree about anything
> they both get wrong.

The mitigation was ordering — build the degenerate host first — and the ordering
held. But the gate does not test the mitigation, and item 2 reads as though it
does.

Worse, item 2 does not use the CLI at all. `gate_three.rs:100–121` constructs a
`BodyState` inside `xtask`, injects, runs, and calls `describe` directly:

```rust
let cli_d = match describe(&state, "sum") { ... };
let cli_txt = print_description(&cli_d);
let two = test_txt == golden && cli_txt == golden;
```

The variable is named `cli_d`. Nothing in that path is `joinn-cli`. The binary's
description reaches the gate only through item 4's rendered transcript. The
honest version of item 2 either compares the capture against the CLI's own
`present` (which means the CLI must be able to emit a description, not only
text), or the item is renamed to what it checks: *the golden description is what
`describe` produces*.

What the item *does* establish is worth keeping: `first_desc_diff` names the
differing field rather than printing a boolean, and the gate asserts the field
is `value`. That is §5.2's third number working, and it is the right shape.

### F28 · `intent_set` is derived from grants, and is correct by coincidence

`joinn-host/src/intent/intent_set.rs`:

```rust
for insts in body.coding.grants.values() {
    for inst in insts {
        set.insert(Address { instance: inst.clone(), port: 0 });
    }
}
```

§2.2 of the plan says *one intent per **in-port** of an instance that the body
exposes at its membrane*. The code inserts port `0` for every granted instance,
whatever its contract says. For the calculator — where `cli_a` and `cli_b` each
have one in-port at position 0 — the two agree. For any exposed instance with
two in-ports, the set is short by one and the second port is unreachable by any
host, silently.

The check does not catch this because `intent_set.rs`'s test and
`gate_three.rs:194–203` both compare against a literal set carrying the same
hard-coded `0`. Check and control share the assumption. This is F9's species in
a derivation rather than in a knob: the domain is not empty, it is *one*, and
one is the only size at which the bug is invisible.

The artifact that would catch it is a body exposing an instance with two
in-ports. It does not exist in the corpus, and Phase 5's second body is the
natural place to put it.

`check_intent` itself is correct and its refusal names `sum@0`. It is exercised
in `tests/intent_set.rs` and by both hosts' `intend`. It is not on gate 3's
table at all — item 5 compares the set and stops.

### F29 · The description half of V61 has an empty domain

V61: *a regulatory edit moves a description and never a cell hash.*
`joinn-host/tests/desc_witness.rs`:

```rust
#[test]
fn regulatory_label_moves_description_not_cell_hash() {
    let mut cell = sum_cell();
    let before_cell = hash(&cell.coding);
    let before_label = cell.regulatory.labels.get(&0).cloned();
    cell.regulatory.labels.insert(0, "the first addend".into());
    assert_eq!(hash(&cell.coding), before_cell);
    assert_ne!(cell.regulatory.labels.get(&0), before_label.as_ref());
}
```

No description is built. The first assertion is V20 — regulatory text is not
hashed — restated one floor down, and the second asserts that a `BTreeMap`
insert changed the map. The clause the invariant is named for, *moves a
description*, is not tested. The test costs three extra lines: run the body,
`describe`, print, change the label, print again, assert the strings differ.

This matters more than it looks, because **Phase 4's invariance harness (V24) is
this test generalised** — *rename a port, restyle a cell, change a literal, swap
an allele; the answer must not move.* Phase 4 will inherit a template that skips
the half where the answer is computed.

### F30 · §2.7 broke inside the phase that wrote it

Decision 2.7 — *no check is scoped to a file* — was written because the hygiene
refactor moved code out from under a check without anyone choosing to. P3-00
rescoped `xtask_does_not_write_findings` to `xtask/src/**` and added
`source_checks_scan_a_tree` to keep it from happening again. Both landed.

And then it happened again, in the same phase, to the neighbouring check.

`xtask/src/main.rs`:

```rust
#[test]
fn write_lock_has_no_literal_gate_score() {
    let src = include_str!("fns/write_lock.rs");
    assert!(!src.contains("8/8") && !src.contains("9/9") && !src.contains("4/4"), ...);
}
```

`write_lock.rs` is clean; it formats `{n}/{total}` from its arguments. But the
literals now live in `gate_all.rs`:

```rust
("phase 2",   p2_ok,  if p2_ok  { 8 } else { 0 }, 8),
("phase 2.1", p21_ok, if p21_ok { 8 } else { 0 }, 8),
("phase 2.2", p22_ok, if p22_ok { 9 } else { 0 }, 9),
("phase 3",   p3_ok,  if p3_ok  { 9 } else { 0 }, 9),
```

Those are the gate scores, written by hand, in a file the check does not read —
and a gate that dropped an item would still write its full score, because the
score is a literal chosen by the caller rather than a count returned by the
gate. `run_gate_table` already computes `n` and `total`; it prints them and
throws them away.

And `source_checks_scan_a_tree`, the rule-26 enforcer, explicitly exempts the
offender:

```rust
if included.ends_with(".rs") && included != "fns/write_lock.rs" {
```

That exemption is honest — the check's subject really is that file — but it
means the one `include_str!` in the tree is the one whose subject moved. The
same test also scans only `xtask/src`; nothing checks `crates/` for
file-scoped checks at all.

### F31 · `joinn-cli` is `joinn-run` with a `Host` impl bolted on

Rule 14's letter is discharged. Its spirit is in
`joinn-cli/src/session/calculator_session.rs`, which reads exactly three lines,
names `cli_a`, `cli_b` and `sum` as string literals, sets `host.pending` by
hand before each `intend`, and returns `Err("first line must refuse at the
membrane")` if the calculator does not refuse `"two"`.

Two consequences.

First, `intend` does not derive an intent from a raw event; the session decides
the address and `intend` wraps it. The chain the roadmap asked for — *raw input
→ intent → address → message at a membrane* — is really *session → address →
intent → message*. The host-independent part of that chain is the part after the
address, which is the part that was never in doubt.

Second, exit gate item 1's *"runs unchanged and un-recompiled under
`joinn-cli`"* is true of the calculator because the CLI was written around the
calculator. Phase 5's second body will not run under it without a new session
function, and the roadmap's Phase 6 Visual Host cannot be written against this
shape at all. The generic CLI — read the body, derive the intent set, prompt for
each address in turn, present every fire — is maybe forty lines and it is the
thing that would have made item 1 testable.

This is not a defect the plan created; the Phase 2.2 review predicted it
precisely (*"Turning that script into raw input → intent → address → message is
the actual work of Phase 3"*). It is the part of the actual work that did not
get done.

### F32 · Smaller things, in order of how much they mislead

| Where | What |
|---|---|
| `joinn-host/src/probe.rs` | `probe(state, addr)` is `describe(state, &addr.instance)`. **`addr.port` is never read.** `probe` addresses a port and inspects an instance. It is a rename of `describe` with an unused field, and P3-08's done-when — *"a `probe` implementation that writes a slot does not compile"* — has no `trybuild` fixture behind it, while `joinn-dna` already uses `trybuild` for exactly this kind of claim. The budget half is genuinely tested. |
| `corpus/phase3/environment.body` | Hashed into `hashes.txt`, verified by `corpus verify`, and **read by nothing.** §2.3 says *the environment is a body; a host declares which of its out-ports it emits.* `Signals` is a `BTreeSet<String>` with no connection to the body, and the body is `prim:eq as columns` — a placeholder that parses. R47 asks whether the environment body is gated; the prior question is whether it is *used*. The mechanism R14 needed did arrive; the body did not. |
| gate 3 item 3 | The control artifact `plant_render.rs` lives in `corpus/phase3/controls/` and the scan walks `crates/joinn-host/src`. The control proves a string exists in a file the check never reads. `xtask/module_fixtures/` shows the right shape — fixtures inside the scanned tree, verified *before* the scan. The real opposition for this claim exists, in `vocab`, and item 3 does not call it; the item's "no IO" half is absent entirely. |
| gate 3 item 7 | `if toml.contains("joinn-run") { return Err("control passed") }` is the check dressed as the control. The stated control — *a planted member line fails* — would be a fixture `Cargo.toml`, not the live one. |
| `corpus/phase3/controls/` | Five files, four of which are consumed. `artifact_list.txt` is the fifth, and per F25 it is the only "control" in the phase that opposes nothing. |
| `README.md` | Still says *"Phase 2.1 is frozen when `gates.lock` records `phase 2: 8/8 · phase 2.1: 9/9`"* — the same stale sentence F24 flagged, now two phases behind, with 2.2 and 3 unmentioned and `joinn run calculator` listed under a binary that has changed crates. F24's row was not actioned. |
| `Role` | Closed at four (`cell`, `input`, `output`, `refusal`) as §3.3 intended, and `describe_refusal` synthesises a zero-hash `Description` when the instance is unknown. A refusal whose `cell` is 32 zero bytes is a description of nothing, and it is hashable. R46 is the right place for that, but the zero hash will end up in a corpus file eventually. |
| `Cargo.toml` clippy lints | `float_arithmetic`, `unwrap_used`, `expect_used`, `todo`, `dbg_macro` all `deny` at the workspace. Good — noted because it is the one place the repo's rules are enforced by the compiler rather than by a scan. |

---

## What Phase 3 proved, and it is not nothing

Strip the gate away and read the crates, and the phase's claim survives:

- A description is a value, built in one place, hashed like a cell, and
  compared byte for byte against a hand-computed golden.
- A host renders it and cannot construct it, and that is enforced by a scan
  across a tree, in CI, by path.
- A headless host with no IO runs the calculator end to end and captures one
  description per fire, including the membrane's refusal.
- The adversary the roadmap named for this phase was fired on purpose and did
  not survive: `present` needs nothing from its host.
- `joinn-run` is gone and the transcript moved first.

None of that depends on the four weak items. The protocol is sound; what is
missing is the apparatus that would let a stranger believe it without reading
the crates. That distinction is why the answer below is a correction block and
not a phase.

---

## Are we ready for Phase 4?

**No — and not because of Phase 3. Phase 4 is the wrong next phase on its own
merits.**

Phase 4's exit gate is: delete `parse(format n) = n` from `CliInput`'s DNA,
watch H₁ go 0 → 1, and name the cycle `host → cli_a → sum → host`. **That was
already done by hand in S1**, on the same complex, with the result written into
`docs/Findings/spikes/s1-assay.md`. S1's own negative note says why running it
again in Rust would not settle anything:

> The complex is small: two independent 3-cycles sharing the return edge
> `sum → host`. That is exactly the "shallow complexes" cost in Part III §14.
> The finding that decides Phase 4 is real at this scale; it does not prove the
> assay will earn its keep on deeper bodies.

Phase 4's adversary is the decoration check — *does the assay catch anything
`require` and `ensure` cannot?* — and Part III §9.6 says what the candidate is:
H¹ ≠ 0, a system where every law holds at every membrane and no consistent
global state exists. **That bug class is definitionally cross-membrane.** The
calculator is one body. There is no second membrane for the inconsistency to
hide between. Run Phase 4 now and the decoration verdict comes back
"inconclusive, complex too shallow", which is what the spike already said, at
the cost of a phase.

Phase 5 builds the second body. The roadmap's own dependency graph has
`P3 --> P5` as a hard edge and `P4 -.optional.-> P5`: **Phase 5 depends on
nothing from Phase 4.** Running 5 then 4 costs nothing structurally and gives
the decoration check a two-body universe to fire in. D7 already says Phase 4 is
cut without ceremony; a phase that can be cut can also be moved.

**One piece of Phase 4 should come forward.** ∂∂ = 0 is on the never-cut list
and Part III §9.4 says exactly what it is for:

> Touch-only law (I §5.2, II V16) | ∂(hyperedge) ⊆ ∪ ∂(cell): boundaries meet
> only at boundaries
>
> Merged, ∂∂ = 0 would hold by construction and therefore detect nothing.
> Derived, **the assembly can fail**, and failure is a finding — a link that is
> a second opening, a membrane that is not closed, a hyperedge that transits.
> V16 becomes a checkable output instead of a rule one hopes was followed.

V16 is a Phase 5 deliverable. Without ∂∂, Phase 5 checks the touch-only law with
a purpose-written predicate over the incidence list — and a purpose-written
predicate beside its own check is precisely what F25 is about. Pull ∂ and the
∂∂ assembly into Phase 5 as the instrument behind V16, and leave H₀, H₁, H₂,
the invariance harness and declarations in Phase 4.

### The correction block, concretely

Eight items. None is a design decision; all belong at the head of the Phase 5
plan the way P3-00…P3-04 carried Phase 2.2's debt.

1. **`GateItem` carries its control artifact.** Add `control_artifact:
   &'static str`, a repo-relative path. `run_gate_table` in `xtask` resolves it
   and refuses the run naming the item if it does not exist — `joinn-gate` never
   opens a file, so rule 15 holds. Delete `artifact_list.txt`. **(F25)**
2. **Convert `gate 3` to a table** so item 8 can see it, and rewrite item 8 as a
   scan over every phase's `&[GateItem]`. Its control is a fixture table with
   one item whose artifact does not resolve. **(F25)**
3. **Item 9 reads the lock it is named for.** Feed `bad.lock` through the same
   score parser as the live lock; a wrong score must fail the item. **(F26)**
4. **Item 1 runs the calculator under both hosts, or is deleted.** If the CLI
   cannot yet emit a description for comparison, delete item 1, renumber, and
   say so in the plan. Eight honest items beat nine. **(F26, F27)**
5. **Plant item 3's control where the scan looks** — a fixture directory inside
   `crates/joinn-host` on the `module_fixtures` pattern, checked before the scan
   — and fold `vocab`'s IO half into the item. **(F32)**
6. **`intent_set` iterates in-ports.** The artifact is a body exposing an
   instance with two in-ports, which Phase 5's second body supplies anyway.
   **(F28)**
7. **V61 builds a description.** Print, edit a regulatory label, print again,
   assert the description moved and the cell hash did not. This is Phase 4's
   V24 template; get it right here. **(F29)**
8. **Gate scores come from the gate.** Each gate returns `(n, total)`;
   `gate_all` passes them through; the literals leave `gate_all.rs`; the
   `write_lock.rs` exemption leaves `source_checks_scan_a_tree`. **(F30)**

F31 — the generic CLI — is **not** in the correction block. Phase 5's exit gate
needs a host that can run a two-body universe, so the generic session is Phase 5
work under its own commit, not debt repayment.

And one item that is neither: **run `gate all` on the Windows machine and write
down what it printed.** Nine ticks have been sitting in a file for three days
without a witness. Whatever the correction block does to items 1, 3, 8 and 9, a
recorded run of the tree as it stands is the baseline that says whether the
other five ever worked.

---

## A note on the species, and whether rule 24 was the right answer

Four phases, four addresses, and rule 24 was the first fix aimed at the
mechanism rather than the instance. It was aimed correctly. What F25 shows is
that a rule written into `AGENTS.md` and a rule the build can enforce are
different objects, and the gap between them is exactly one struct field.

The deeper lesson is about where the rule was put. Rule 24 says *a control is an
artifact the item points at*. The word doing the work is **points**: the item
must carry the pointer, in data, where something can follow it. Written as
prose in `AGENTS.md` it is a habit. Written as `control_artifact: &'static str`
on `GateItem` it is a type, and a type is what the repo has been converting
habits into since Phase 2.1 — `Drive::bound` to `NonZeroU32`, `Seal::reference`
to a `BodyRef`, `present` to a `Description`. The same move works here, and it
is the third time the answer has been *make it a value*.

R43 is advanced again by `first_desc_diff`: a refusal that names `port 2 value`
rather than "descriptions differ" knows what its acceptance would have looked
like. Two instruments now have that shape. It is worth writing down as a rule
before a third one is built without it.

---

*Phase 3 review, September 22, 2026. Continues the F-numbering at F25. Written
against the tree as of `gates.lock` mtime 2026-09-19 20:12 UTC, by reading the
source only: **no command was run and no experiment was planted.** Every claim
is checkable by `cargo xtask gate 3` and by opening the files named.*
