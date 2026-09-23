# JoInn Phase 3 Implementation Plan

**A host is proved by there being two · a working plan for Cursor**

*Bootstrap 0's last phase: the thing the calculator note left undefined, defined, and proved by having two of them*

Author: AJ · Draft 0.1 · September 19, 2026

> Status tags follow Parts I–IV: **DECIDED**, **PROPOSED**, **OPEN**. Anything marked **PROPOSED · yours** is a recommendation made while writing this plan and is yours to overrule; Cursor implements whatever lands there, not whatever is written here.

> **What this is.** Phases 2, 2.1 and 2.2 built a universe that is true and cannot be seen. Something has been showing it anyway — `joinn-run`, a hard-coded runner with `print!` in it, which rule 14 has promised to delete since Phase 2. This phase defines what that thing actually is. **A host is a host because there are two of them and the body cannot tell which one it is under.**

> **The one idea.** Phase 2.1's was *a reference allele is a body*. Phase 2.2's was *opposition applies to the instruments*. This one: **a description is a value, not text.** Today `joinn-run::present` reads `body.regulatory.present`, substitutes `{0} + {1} = {2}`, and returns a `String`. That is not a cell describing itself; it is a CLI format string wearing the name of a protocol. If `present` keeps returning text, then "two hosts produce the same cell-level descriptions" is a test of `String` equality between two callers of the same substitution, and the exit gate tests the CLI twice. A `Description` that is a **value** — comparable, hashable, capturable — is what makes the gate mean anything, what makes the headless test host possible at all, and what the Visual Host turns into organelles in Phase 6 without renegotiating.

> **Why this is a phase and not a patch.** The roadmap's Phase 3 names three crates and a protocol. The protocol is the part with a decision in it, and the decision is §2.1. Everything else — two hosts, an intent vocabulary, an environment body, deleting `joinn-run` — falls out of it or is bookkeeping. A phase that only moved `present` into a trait would be a refactor; a phase that makes a cell's self-description into a value the gate can judge is the last brick in Bootstrap 0.

> **Where Phase 2.2 left off, honestly.** `gates.lock` reads `phase 2: 8/8 · phase 2.1: 8/8 · phase 2.2: 9/9`, and on the tree as of 2026-09-18 20:18 that was reproduced end to end: build clean, `cargo test --workspace` 77 passed, `power` 20/20, `agree` five seals × 256 + 10 000 round-trip, `corpus verify` 21, `gate all` 1086 s writing a byte-identical lock. **Then the tree was modularised** — `joinn-live/src/lib.rs` went 68 KB → 255 bytes across ~35 files, `xtask/src/main.rs` 59 KB → 2 KB across ~95, and every crate moved to `module.rs` + `module/`. **That tree has not been re-verified.** `corpus/hashes.txt` and every corpus file are untouched by mtime, which is the right sign, and `xtask modules` is already built and opposed, which is better than the plan would have asked for. But the refactor is the largest single change in the repo's history and it has not been run against the gates. **P3-00 is that run, and it is the first commit for the same reason P21-00 and P22-00 were: the repo is made honest before it is extended.**

---

## 0. How to Drive Cursor With This Plan

| | |
|---|---|
| **Where this file lives** | `D:\JoInn\docs\Plans\JoInn Phase 3 Implementation Plan.md`, beside the Phase 2.2 plan |
| **Where the code lives** | `D:\JoInn\joinn\` — the existing repo, extended by three crates and shrunk by one |
| **What it closes** | The roadmap's Phase 3; F19–F25 of `docs/Findings/phase-2.2-review.md`; R14 and R15 begin |
| **Standing rules** | Appendix A. It **replaces** the current `AGENTS.md` and `.cursor/rules/joinn.mdc` |
| **Unit of work** | one numbered commit from §4. One Cursor session per commit |
| **How a commit ends** | its **done-when** line is checkable by a command, and that command reports a refusal, a disagreement, a compile error, or a byte-identical replay — never a number the command also chose |
| **What Cursor may decide** | module layout inside a crate (subject to rule 25), function bodies, test names, error strings, the internal representation of a capture |
| **What Cursor may not decide** | anything in §2, the shape of a `Description` (§2.1), where an intent comes from (§2.2), crate boundaries, the ordering of P3-11 before P3-12, or whether a check is worth having an artifact for an opposite |

The prompt stays short:

> Implement commit **P3-07** from `docs/Plans/JoInn Phase 3 Implementation Plan.md`. Follow the rules in `AGENTS.md`. Stop when the done-when command passes and report what it printed.

**The failure mode this phase must not have.** Phase 2 shipped checks that could not fail. Phase 2.1 shipped checks with empty domains. Phase 2.2 shipped a control that was a fact about the standard library. The species is the same each time and it changes address every phase. **Phase 3's address is the description.** If `Description` is allowed to be a `String`, or to be built by asking the host how wide the terminal is, or to be compared by rendering both sides through the same formatter, then exit-gate item 2 will print a pass and mean nothing, and this plan will have been written for nothing. Watch that one line.

---

## 1. Scope Fence

### In scope

- **The host protocol** — `present`/`probe` outward, raw event → intent → address → message inward, as a vocabulary with no renderer behind it.
- **The CLI host**, properly built, replacing `joinn-run`.
- **The headless test host** — the witness-capture harness of Part II §17.3, built years before the pixels it will compare.
- **The environment body** (R14), as a signal emitter, with the CLI's signal set nearly empty.
- **F19–F25** of the Phase 2.2 review, which are small and are folded into the commits they touch.
- **The module rule**, stated as a standing rule and enforced in CI.

### Out of scope — Cursor must refuse these even when they look small

| Not now | Phase | Why it is tempting |
|---|---|---|
| Any renderer, any pixel, any `wgpu`, `winit`, `lyon` | 6+ | A `Description` is *exactly* what a renderer consumes. Building one makes drawing it look like the obvious next step. It is not; it is three phases away and the intervening phases change what is drawn |
| A second body, hyperedges, lenses, `grant` across a membrane | 5 | Two hosts is not two bodies. The calculator stays one body under both |
| The assay layer, ∂, declarations | 4 | `declarations` stays empty and stays a refusal |
| A testimony corpus | 12 | `corpus/testimony/` stays empty. §2.8 of the Phase 2.2 plan still governs, and P3-14 makes it *harder* to fill by accident, not easier |
| Golden pixels, perceptual diffing, AccessKit | 8 | The a11y **face** captured here is a value in a `Description`, not a tree handed to a platform API |
| Expression rules, thermal signals, safe-area insets | 11 | R14 *begins* here with a mechanism and one signal. It does not get content |
| A compiler, or optimising the live engine | 10 | Unchanged |
| Another primitive | never, without an amendment carrying a refuted reduction | The floor's amendment allowance was spent in 2.1 and was not renewed in 2.2. A host is not a reason to reopen it |
| Adding to `grandfather.txt` | never in this phase | §2.8. A grandfather list is how a rule dies, and this phase is where the rule is written down |

---

## 2. Decisions Assumed by This Plan

Nine calls. The first is the phase; the rest follow from it, resolve the Phase 2.2 review, or write down what the hygiene refactor already did.

### 2.1 A description is a value, not text · **PROPOSED · yours · this is the phase**

> **`present` produces a `Description`. A `Description` is a value: it names the cell by hash, the instance by name, each port by position, each port's frame, each port's value in canonical form, and the regulatory face — display name, label, role. It contains no formatting, no layout, no widths, and no host knowledge. Turning a `Description` into text is the CLI host's business. Turning it into organelles is the Visual Host's business, in Phase 6.**

Sketch, so the shape is not in doubt:

```rust
pub struct Description {
    pub cell: Hash,
    pub instance: String,
    pub ports: Vec<PortFace>,
    /// Regulatory face of the instance itself.
    pub label: String,
    pub role: Role,
}

pub struct PortFace {
    pub position: u32,
    pub direction: Direction,
    pub frame: FrameRef,
    /// Canonical printed form of the value, or None when the slot is empty.
    pub value: Option<String>,
    /// Regulatory display name (G1: names are regulatory, positions are structure).
    pub name: String,
    pub label: String,
    pub role: Role,
}
```

**Three things this buys, and they are the reason it is the phase.**

1. **The exit gate becomes a byte comparison.** "Both hosts produce the same cell-level descriptions" is `desc_cli == desc_test`, over a value with a canonical form and a hash — the same machinery as a cell. Compare that to comparing two `String`s produced by the same `format!`, which is what the current code would give you.
2. **The test host becomes possible.** A headless host has nothing to print into. It captures. You cannot capture a format string's output without deciding what the format is, which means a headless host built on text is a *second formatter*, and V32 would then demand the two formatters agree — an obligation invented by a bad decision.
3. **Phase 6 does not renegotiate.** Part II's upward wrap says a cell describes itself and the host decides how it looks. This is that sentence as a type, arriving three phases before the renderer, which is exactly when the roadmap wanted it: *build the universe first and the renderer falls out.*

**What this deletes.** `joinn-run/src/present/present_ports.rs` — the `{0} + {1} = {2}` substitution — does not move into `joinn-host`. It moves into `joinn-cli`, where it belongs, and `body.regulatory.present` becomes a **CLI-host template**, read by the CLI host from the regulatory region, not part of the protocol. `prompt()` moves the same way.

**Cost to reverse:** `Description` collapses to `String`, the test host becomes a formatter, and the Phase 3 exit gate loses its meaning. One commit to do, three phases to regret.

### 2.2 An intent is declared by the body, never invented by the host · **PROPOSED · yours · R14/V9**

Inward, the roadmap's chain is raw input → **intent** → address → message at a membrane. The load-bearing question is where the intent vocabulary comes from, and the tempting answer is "the host knows what its users can do," which makes every host's vocabulary different and makes V9 — one intent vocabulary across three platforms — unfalsifiable until Phase 11.

> **The intent set of a body is computed from the body: one intent per in-port of an instance that the body exposes at its membrane. A host may only emit an intent in that set; emitting one outside it is a refusal naming the intent. An `Intent` carries an address — `(instance, port)` — and a raw `Term` that the membrane will check like any other message.**

So the calculator's intent set is derived, not written: `{ (cli_a, 0), (cli_b, 0) }`. The CLI host maps "a line of stdin while prompting for `cli_a`" to the first. A future Visual Host maps "a click in that cell's input port" to the same one. **The same intent, from a keystroke and from a click**, which is V9 becoming checkable in Phase 3 instead of Phase 11.

Note what this does *not* do: it does not check the value. The intent carries a `Term`; the membrane refuses `"two"` exactly as it does today. The host is not a validator and must not become one — the refusal that prints in the transcript is the membrane's, and it stays the membrane's.

**Cost to reverse:** intents become host-declared, V9 goes back to being a Phase 11 promise, and the test host and CLI stop being comparable on anything but output. Confined to `joinn-host`.

### 2.3 The environment is a body, it declares its signals, and the two hosts disagree about them · **PROPOSED · yours · R14 begins**

The roadmap wants the environment body as a signal emitter and notes that the CLI's signal set is "nearly empty — which is a useful degenerate case." Nearly empty is right. **Exactly empty is a trap**, and it is this project's own trap: a signal set with no members is a check with an empty domain, and a host protocol whose signal mechanism is never exercised prints a pass for the same reason `agree` did at `bound: 0`.

> **The environment is a body. A host declares which of its out-ports it emits. A body reading a signal the host does not declare is refused, naming the signal. The CLI host declares exactly one signal; the test host declares none.**

One signal, and it should be the one a CLI genuinely has and a headless capture genuinely does not. `columns : ℤ` is the honest candidate. It is not used by the calculator — which is the point:

- The calculator reads no signals, so it runs under both hosts unchanged. That is exit-gate item 1.
- A second, deliberately signal-reading body runs under the CLI and is **refused under the test host, naming `columns`**. That is item 6's control, and it is an artifact — a `.body` file in the corpus — not a predicate.

R14 thereby *begins* with a mechanism and one member, and Phase 11 fills it without inventing the mechanism under deadline.

**Cost to reverse:** signals become a struct handed to the host and the refusal disappears. Cheap now; it re-opens as Phase 11's whole surface.

### 2.4 `probe` is read-only and returns a `Description` · **PROPOSED · yours**

`present` is the host asking a cell to describe itself for display. `probe` is the host asking a cell to describe itself for **inspection** — what the creator will use in Phase 9 and what an accessibility tree is built from in Phase 8.

> **`probe(address) -> Verdict<Description>` reads and never writes. It holds no grants, takes no step budget from the running body, and cannot cause a fire. A `probe` that would change a slot is a refusal, not a warning.**

The reason to decide it now rather than in Phase 9 is that a `probe` which can fire is a second delivery path, and V35 says there is exactly one. Deciding it late means discovering that the creator's inspector is mutating the universe it inspects.

**Cost to reverse:** none worth stating. A writing `probe` is a bug with a nice name.

### 2.5 The transcript moves to its new host **before** the old one is deleted · **DECIDED**

Not a design decision — an ordering one, and the plan states it because getting it wrong is a day lost and a re-freeze nobody wanted.

`xtask::fns::run_calculator_bin` spawns `cargo run -p joinn-run -- run calculator` and pipes `"two\n2\n3\n"` at it. **Gate 2's transcript item and gate 2.1 item 7 both go through that spawn.** Rule 14 says `joinn-run` is deleted at the start of Phase 3. V28 says the transcript passes at every later gate, forever, byte for byte.

> **P3-11 repoints the spawn at `joinn-cli` and proves the five lines are byte-identical. P3-12 deletes `joinn-run`. In that order, in separate commits, with `gate all` green between them.**

A single commit that does both cannot tell you whether the CLI host reproduced the transcript or whether the test simply stopped running.

### 2.6 A control is an artifact, not a predicate · **PROPOSED · yours · closes F20, answers R43 in part**

From the Phase 2.2 review. Every control that is a file, a body, a cell or a planted malformation constrained something; every control that is a boolean expression written beside its check constrained nothing.

> **A gate item's control is an artifact the item points at — a file in the corpus, a body with a hash, a fixture directory — and a reviewer can read it. A control that is an expression evaluated beside the check is not opposition; it is the same assertion written twice.**

Two things make this the right phase for it rather than a correction phase of its own. First, it is already happening: `xtask/module_fixtures/` holds `accept_one_function.rs`, `refuse_two_functions.rs` and a planted `refuse_mod_layout/mod.rs`, and `check_opposed` refuses to run the scan if any of the three stops being what it claims. That is rule 24 invented independently, in the hygiene refactor, which is good evidence it is the natural shape. Second, **this phase manufactures artifacts by the dozen** — every captured `Description` is one — so the rule arrives with the machinery that makes obeying it free.

`gate 2.2` item 3 is the one existing violation and P3-02 fixes it.

**Cost to reverse:** gate items go back to paired predicates and F9's species finds a fourth address.

### 2.7 No check is scoped to a file · **PROPOSED · yours · new, and the refactor is why**

The hygiene refactor did something the plan for it could not have predicted. `xtask/src/main.rs` used to be the whole instrument, 59 KB of it, and the test that enforces rule 17 read:

```rust
let src = include_str!("main.rs");
assert!(!src.contains("join(\"Findings\")"));
```

After the split, `main.rs` is 2 KB and the test reads `include_str!("fns/write_lock.rs")` — **one file out of about a hundred.** The check kept its name, kept passing, and lost almost all of its domain. Nothing was done wrong; the code moved out from under a check that was scoped by file.

> **A check that scans "the source" scans a directory tree, not a file. `include_str!` of a single file is admissible only when the check's subject *is* that file.**

This is not a one-off fix. Rule 25 guarantees files will keep splitting, for the whole life of the project, so a file-scoped check is a check with a shrinking domain by construction. It is F9 arriving through the build system instead of through a knob, and it is worth a standing rule precisely because no one chose it.

**Cost to reverse:** none. This is a subtraction of a failure mode.

### 2.8 The module rule, written down · **DECIDED, AJ, Sept 19**

The hygiene refactor converted every crate and built `cargo xtask modules` to enforce it. This section states the rule so it is a rule and not a habit. It is what the scanner already does; nothing here asks for new behaviour.

> **`module.rs` beside `module/`. Never `mod.rs`. A leaf file holds at most one named production function.**

| Shape | Rule |
|---|---|
| `mod.rs`, anywhere | **Refused.** Use `foo.rs` + `foo/` |
| **Leaf** — a `.rs` with no sibling directory of its name | At most **one** named production function |
| **Capsule root** — `foo.rs` with `foo/` beside it | Types, `mod` declarations, `pub use`, and getters. It is the capsule's face, not its work |
| **`lib.rs`** | A facade: **zero** production functions |
| **`main.rs`** | `main` only |
| `#[cfg(test)]` | Does not count toward the one-function rule, at any depth |
| One `impl Trait for T` | Its required methods may share a leaf. A trait's shape is not the author's to split |
| `allow(modules): <reason>` | Silences one file. A bare silencer with no reason is a build failure, exactly as in `vocab` |
| `tests/` at crate level | Exempt from the one-function rule (trybuild and integration tests are not capsules). **Still refused a `mod.rs`** |
| `grandfather.txt` | Repo-relative paths permitted to violate while their crate is enforced. **A stale entry — a file that already obeys — fails the build.** |

Three notes for the implementer, because they are where this rule will be argued with:

- **`grandfather.txt` is how this rule dies.** It is currently empty, which is the best state it will ever be in. **No commit in this phase may add a line to it.** If a Phase 3 file cannot obey the rule, that is a finding about the rule, written into `docs/Findings/`, not a line in a list.
- **The fixtures are the control** (§2.6). `check_opposed` runs *before* the scan and fails if `refuse_two_functions.rs` stops having two functions, if the planted `mod.rs` goes missing, or if `accept_one_function.rs` stops having exactly one. Keep that ordering; a scan that runs before its own control is a scan that can be quietly blinded.
- **The new crates are born compliant.** `joinn-host`, `joinn-cli` and `joinn-test-host` go on `enforced.txt` in the commit that creates them, not in a cleanup commit afterwards.

**Cost to reverse:** the scanner is deleted and the tree drifts back. The conversion is already paid for; reversing it wastes that and nothing else.

### 2.9 `std::io` and `std::fs` live in host crates and `xtask` · **PROPOSED · yours · closes F23**

Rule 15 currently says *"`std::io` appears only in `joinn-run`."* It has been false since Phase 2.1: `joinn-gate/src/testimony/write_line.rs` opens, creates and appends to `corpus/testimony/<hash>.log`, and `xtask vocab` does not check rule 15, so nothing noticed. Phase 3 both makes the rule urgent — two new crates, one of which must do IO and one of which must not — and makes it cheap to fix.

> **`std::io` and `std::fs` appear only in host crates (`joinn-cli` today) and in `xtask`. `joinn-host` is protocol and does none. `joinn-test-host` captures into memory and does none. `joinn-frame`, `joinn-dna`, `joinn-gate`, `joinn-prim` and `joinn-live` do none. `xtask vocab` enforces this by path.**

And the second half, which is the part §2.8 of the Phase 2.2 plan was protecting: `TestimonyStore::at` — a file-backed writer aimed at the directory Phase 12 is supposed to design — **has no caller anywhere in the workspace.** It is deleted. `TestimonyStore::memory()` stays, `corpus/testimony/.gitkeep` stays, R41 stays open. The field was removed in 2.2; this removes the drawer.

**Cost to reverse:** add the writer back when testimony is designed, which is the point.

---

## 3. Phase 3 — Architecture

### 3.1 Crates

Three added, one deleted. The dependency rule is unchanged: each crate depends only on those to its left.

```
joinn-frame ─► joinn-dna ─► joinn-gate ─► joinn-prim ─► joinn-live ─► joinn-host ─┬─► joinn-cli
                                                                                   └─► joinn-test-host
```

| Crate | Owns | IO |
|---|---|---|
| `joinn-host` | The protocol. `Description`, `PortFace`, `Role`, `Intent`, `Signals`, `trait Host`, `describe`, `probe`, the intent-set derivation, the environment body's contract | **none** |
| `joinn-cli` | The CLI host. `bin joinn`. stdin/stdout, the regulatory template substitution, prompts, the `columns` signal | yes |
| `joinn-test-host` | The headless host. Captures descriptions, a11y faces and intent sets into memory. Declares no signals | **none** |
| ~~`joinn-run`~~ | Deleted at P3-12 | — |

`joinn-host` depends on `joinn-live` because a host drives the engine: it injects at a membrane and reads reports. It does **not** depend on `joinn-prim`; the native register is handed to it, exactly as `joinn-live` is handed one today.

### 3.2 The protocol, end to end

```rust
pub trait Host {
    /// Outward: show a description. The host decides how; the description decides what.
    fn present(&mut self, d: &Description) -> Verdict<()>;

    /// Inward: turn a host-specific raw event into a declared intent, or refuse it.
    fn intend(&mut self, raw: RawEvent) -> Verdict<Option<Intent>>;

    /// Which environment signals this host emits (§2.3).
    fn signals(&self) -> &Signals;
}

/// Read-only inspection. Holds no grants, takes no budget, cannot fire (§2.4).
pub fn probe(state: &BodyState, addr: Address) -> Verdict<Description>;

/// The intent set a body admits, derived from the body (§2.2).
pub fn intent_set(body: &Body) -> BTreeSet<Address>;
```

`RawEvent` is the one host-specific type in the protocol, and it is deliberately opaque to `joinn-host`: a CLI's raw event is a line, a test host's is a scripted tuple, a Visual Host's will be a pointer position. Everything downstream of `intend` is host-independent, which is the boundary the exit gate tests.

### 3.3 The description's canonical form

A `Description` is compared and stored, so it needs a canonical form — and it already has all the machinery. It is written and hashed exactly as a cell is: the hand-written canonical writer of `joinn-dna`, the length-prefixed tag rule of P0-03, no `serde`, no `#[derive(Hash)]`.

```
description {
  cell 6b3271631abf…
  instance sum
  label "Sum"
  role cell
  port 0 in ℤ 1 "a" label "first addend" role input value 2
  port 1 in ℤ 1 "b" label "second addend" role input value 3
  port 2 out ℤ 1 "result" label "sum" role output value 5
}
```

Stored in `corpus/descriptions/`, with the **hand-computed hash checked against the machine's** — the P1-07 / P2-04 / P21-08 / P22-05 discipline, for the fifth and final new file kind in Bootstrap 0.

**The a11y face is `label` and `role`, and nothing more, in this phase.** Both are regulatory. `role` is a small closed enum — `cell`, `input`, `output`, `refusal` — chosen now precisely so that Phase 8 inherits a vocabulary rather than inventing one under the pressure of making AccessKit work.

### 3.4 What the two hosts must and must not share

| | `joinn-cli` | `joinn-test-host` |
|---|---|---|
| Builds a `Description` | no — `joinn-host::describe` does | no — same |
| Renders it | to text, via the regulatory template | not at all; stores the value |
| `RawEvent` | a line of stdin | a scripted `(Address, Term)` |
| Signals declared | `columns` | none |
| `std::io` | yes | **no** |
| Captures | nothing | descriptions, a11y faces, intent sets |

The one line to watch, restated from §0: **neither host may build a `Description`.** If `joinn-cli` ever constructs one, it can construct one that renders nicely, and item 2 compares the CLI's idea of the universe with the CLI's idea of the universe.

---

## 4. Phase 3 — The Commit Plan

Twenty commits. **Done-when** is a command, and the command must be able to refuse.

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P3-00** | **The refactored tree, verified** | `gate all`, `power`, `agree`, `corpus verify`, `modules`, `vocab` run on the modularised tree; `gates.lock` rewritten from per-gate outcomes; the four stray `.py` scripts in `crates/joinn-dna/` and `crates/joinn-frame/` and `xtask/extract_xtask.py` deleted; `xtask_does_not_write_findings` rescoped from one file to `xtask/src/**` (§2.7) | `cargo xtask gate all` writes a lock from outcomes on **this** tree; the rescoped Findings check **fails** on a planted `join("Findings")` in any `xtask/src` file, then passes when reverted; `corpus verify` still matches 21 goldens. If any gate dropped, the lock records the drop and the commit is red — **that is the commit** |
| **P3-01** | **CI is green** | `cargo fmt --all`; `&& false` at `floor/check_one.rs` deleted; `manual_is_multiple_of` fixed; `cargo xtask modules` added to `ci.yml` after `vocab` | `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo xtask modules` and `cargo xtask gate all` **all exit 0** from a clean checkout, offline |
| **P3-02** | Rule 24 on the one violator | `gate 2.2` item 3's control becomes a `SealSpec` with `drive_bound: 0` refused by name; the two `NonZeroU32::new(0)` stdlib assertions deleted; `joinn-prim::seals::nz` returns a `Verdict` or is deleted | `gate 2.2` **fails naming item 3** when the seal-loading refusal is removed, demonstrated then reverted; no test in the workspace asserts a property of `NonZeroU32` |
| **P3-03** | Counterfeit strength | `docs/Findings/counterfeit-strength.md`, hand-typed: five separating samples, the bound-1 experiment, R39 opened with no date closed; one **subtler** counterfeit for `add@ℤ` — correct except on one carry — replacing the identity-forward body | `cargo xtask agree` prints a separating sample **greater than 8** for `add@ℤ`; the old counterfeit stays in the corpus as a second, cruder one; the finding names both |
| **P3-04** | The gate stops repeating itself | `gate 2.2` items 1, 2 and 8 read one `agree()` result per gate run instead of calling it three times | `cargo xtask gate all` prints its wall time and the `agree` harness output appears **twice** in the whole run, not four times; the three items still fail independently when their own controls pass |
| **P3-05** | **`joinn-host`: the description** | The crate; `Description`, `PortFace`, `Role`; `describe(state, address) -> Verdict<Description>`; the canonical writer and hash; `enforced.txt` gains `joinn-host` | `joinn-host` contains **no `String`-returning render function**, asserted by a scan, and **no `std::io`/`std::fs`**, asserted by `vocab`; `describe` on the calculator's `sum` instance produces the §3.3 text; **the hash matches the one you computed by hand** |
| **P3-06** | Intents come from the body | `Intent`, `Address`, `intent_set(body)`; an intent outside the set is refused naming it | `intent_set(calculator)` is exactly `{(cli_a,0), (cli_b,0)}`, asserted by value; an `Intent` addressed at `sum@0` is **refused naming the address**; the control is the two declared intents, accepted |
| **P3-07** | The environment body | `Signals`; the environment as a `.body` in `corpus/phase3/`; a host declares its emitted set; reading an undeclared signal refuses naming it; `corpus/phase3/columns_reader.body` as the artifact control | The calculator reads no signal and runs under a host declaring none; `columns_reader.body` runs under a host declaring `columns` and is **refused naming `columns`** under one that does not |
| **P3-08** | `trait Host` | The trait; `probe` read-only per §2.4; no grants, no budget draw | A `probe` implementation that writes a slot **does not compile** (the signature takes `&BodyState`); `probe` on a running body leaves `steps()` unchanged, asserted by value |
| **P3-09** | **`joinn-test-host`** | The headless host: scripted `RawEvent`s, captures descriptions, a11y faces and the intent set; declares no signals; `enforced.txt` gains it | The calculator runs end to end under the test host with **no IO in the crate**, asserted by `vocab`; the capture holds one `Description` per fire; the refusal of `"two"` is captured as a `Description` with `role refusal` |
| **P3-10** | **`joinn-cli`** | The CLI host: `bin joinn`, stdin/stdout, the regulatory template and prompts moved here from `joinn-run` (not into `joinn-host`), the `columns` signal | `echo "two\n2\n3" \| joinn run calculator` prints the five lines **byte-identical** to `corpus/transcripts/calculator.txt`, indent included; `joinn-cli` builds a `Description` nowhere, asserted by a scan for `Description {` outside `joinn-host` |
| **P3-11** | **Move the witness** | `xtask::fns::run_calculator_bin` spawns `joinn-cli`; `joinn-run` is untouched and still present | `cargo xtask gate 2` and `gate 2.1` pass with the spawn repointed; the transcript and `calculator.trace` both still replay byte-for-byte. **`joinn-run` still exists at the end of this commit** (§2.5) |
| **P3-12** | **Delete `joinn-run`** | The crate removed from the workspace and from disk | `cargo xtask gate all` passes with the crate gone; `grep -r joinn-run` finds nothing outside `docs/`; rule 14 is retired from `AGENTS.md` in the same commit, because it has been discharged |
| **P3-13** | **Description witnesses** | `corpus/descriptions/calculator.desc` and `calculator_refusal.desc`, hand-computed hashes; `corpus/hashes.txt` extended; both hosts compared | Both hosts produce **byte-identical** descriptions for every fire of the calculator; the hashes match the hand-computed ones; changing a regulatory label moves the description and **not** the cell hash, asserted by value |
| **P3-14** | IO, in its place | Rule 15 rewritten per §2.9; `TestimonyStore::at` and `testimony/write_line.rs` **deleted**; `vocab` enforces IO by crate path | `cargo xtask vocab` **fails** on a planted `std::fs` in `joinn-gate`, then passes when reverted; `corpus/testimony/` still holds only `.gitkeep`; `TestimonyStore::memory()` still backs `Gate::new` |
| **P3-15** | **The adversary, on purpose** | A body written to need a host fact for its own `present` — a cell whose description would want a width | Either it is **refused naming the missing signal** (the upward wrap holds, and that refusal is the finding), or it cannot be, and `docs/Findings/present-leaks.md` records what `present` turned out to need, hand-typed, with the roadmap's Phase 3 adversary marked **fired** |
| **P3-16** | Perf, two hosts | `xtask perf` gains a fourth probe: the calculator under each host, steps and milliseconds | `cargo xtask perf` prints four probes and **writes no file**; the conclusion is typed by a human into `live-engine-performance.md` |
| **P3-17** | The decision ledger · **PROPOSED · yours, cuttable** | `docs/Findings/decisions.md`, hand-typed: one line per D/V/R — id, statement, phase that set it, status, and for each later move a kind (`narrowed`, `extended`, `deferred`, `reversed`); `cargo xtask decisions` prints the counts | `cargo xtask decisions` prints a reversal count and **writes no file**; a planted `reversed` line with no linked finding **fails** the command |
| **P3-18** | Re-freeze | New goldens for the description witnesses, the environment body, the subtler counterfeit and anything P3-01's formatting moved; a dated finding per deliberate move | `cargo xtask corpus verify` matches every golden; `docs/Findings/` records each moved hash, its reason and its date, typed by hand |
| **P3-19** | Exit gate 3 and the lock | `cargo xtask gate 3` as a table of **artifact-opposed** items (§2.6); `gates.lock` restored | `cargo xtask gate all` runs phases 0, 1, 2, 2.1, 2.2 and 3 from a clean checkout, offline, writes the lock from per-gate outcomes, **and fails when any item's control passes** |

**Ordering notes.** P3-00 and P3-01 come first and P3-00 may well be red — the modularised tree has not been run, and if the refactor moved something, the honest state of the repo is a dropped gate, not a plan that pretends otherwise. P3-02 through P3-04 are the Phase 2.2 review's debt and are independent of everything after them; they can be cut to a later phase if the host work is going well, **except P3-02**, which must precede P3-19 because gate 3's item 8 checks rule 24 across every gate. P3-05 is the substance and everything from P3-06 to P3-10 depends on it. **P3-11 before P3-12 is not negotiable** (§2.5). P3-13 is the exit gate in embryo. P3-15 is the phase's adversary and is scheduled *before* the re-freeze so that a finding can still change the shape.

---

## 5. Test Strategy

### 5.1 New invariants

Continuing from V58.

| # | Invariant | Test | Commit |
|---|---|---|---|
| **V59** | A `Description` is a value with a canonical form and a hash; no host constructs one | structural scan + hand-computed hash | P3-05 |
| **V60** | Two hosts produce byte-identical descriptions for the same body and the same inputs | the capture comparison | P3-13 |
| **V61** | A regulatory edit moves a description and never a cell hash | V20's sibling, one floor up | P3-13 |
| **V62** | A host may only emit an intent in the body's derived intent set | `intent_set`, with the refusal | P3-06 |
| **V63** | A body reading a signal its host does not declare is refused naming the signal | `columns_reader.body` as the artifact control | P3-07 |
| **V64** | `probe` writes nothing, fires nothing, and draws no budget | `steps()` unchanged; the signature | P3-08 |
| **V65** | No crate but a host crate and `xtask` names `std::io` or `std::fs` | `vocab`, by path | P3-14 |
| **V66** | Every gate item's control is an artifact the item points at | gate 3 item 8, over every gate's table | P3-19 |
| **V67** | No check is scoped to a single file unless that file is its subject | the rescoped Findings check; a scan for `include_str!` in check positions | P3-00 |
| **V68** | Every `.rs` under `crates/` and `xtask/src` obeys the module rule, and `grandfather.txt` holds no stale entry | `cargo xtask modules`, with its three fixtures | P3-01 |
| **V69** | The Phase 2 transcript is produced by whichever host currently exists, byte for byte | gate 2, after the spawn moves | P3-11 |

Carried forward and re-run every commit: V18, V19, V20, V24-embryo, FO1–FO10, the three canonical-text properties, the 1 000-append hash-stability test, V33–V58, and Phase 1's four demos.

### 5.2 The numbers that matter

Three now, and the third is this phase's contribution.

1. **`cargo xtask power`** — unchanged in rule. Every mutant names the check that caught it; a mutant whose check refuses its negative control fails the build.
2. **`cargo xtask agree`** — unchanged, plus P3-03's obligation: the separating sample is printed per seal, and a sample of 0 or 1 for every seal is a finding about the counterfeits, written into `Findings/`, not tuned away.
3. **The description diff.** `cargo xtask gate 3` prints, per fire, whether the two hosts' descriptions matched, and on a mismatch prints **the first differing field** — not "descriptions differ." A diff that names `port 2 value` is a bug report; a boolean is a shrug. This is the `BLIND SEAL` message's lesson applied to the new instrument: a refusal that knows what its own acceptance would look like.

### 5.3 What Phase 3 deliberately does not test

**How a description looks.** There is no golden text, no width, no wrapping, no colour. The CLI's rendering is tested only by the five-line transcript, which already exists and which V28 already protects. Inventing a rendering test here would be inventing Phase 8's perceptual-tolerance problem two bootstraps early, and Phase 8 has already pre-agreed that golden pixels may be dropped entirely.

**Whether the intent vocabulary is *good*.** V9 says one vocabulary across platforms; this phase proves two hosts can share one, not that the one they share is the right one. That is R21's territory and a first-grader's.

---

## 6. Dependencies and Forbidden Constructs

**No new third-party dependencies.** Everything forbidden in Phases 1, 2, 2.1 and 2.2 stays forbidden, with one change and six additions.

| Change | |
|---|---|
| Rule 15 | Was *"`std::io` appears only in joinn-run."* Becomes §2.9: `std::io` and `std::fs` appear only in **host crates** and `xtask`. Enforced by `vocab`, by path, which is what made the old rule fail silently |

| Newly forbidden | Why |
|---|---|
| A `Description` constructed outside `joinn-host` | §2.1. A host that can build one can build one that flatters it |
| A `String`-returning render function in `joinn-host` | §2.1. The moment the protocol can format, the protocol has a look |
| An intent a host invents | §2.2. The vocabulary is the body's |
| A `probe` that writes, fires, or draws budget | §2.4. V35 has exactly one delivery path |
| A gate item whose control is a predicate written beside its check | §2.6 |
| A new line in `grandfather.txt` | §2.8. The list is empty; this phase keeps it that way |
| `mod.rs`, anywhere | §2.8 |
| A check scoped to one file that is not its subject | §2.7 |

---

## 7. Exit Gate 3, As a Checklist

Scripted and repeatable, from a clean checkout, offline. `cargo xtask gate 3` runs it, **as a table of artifact-opposed items** — each line names both halves, and each control is a file.

- [ ] **1 · One body, two hosts.** The calculator runs unchanged and un-recompiled under `joinn-cli` and `joinn-test-host` and produces the same results. *Control:* `corpus/phase3/columns_reader.body` runs under one host and is refused under the other, naming the signal.
- [ ] **2 · The descriptions agree.** Every fire produces byte-identical `Description` values under both hosts, matching `corpus/descriptions/calculator.desc`. *Control:* a planted description with one altered port value is refused, and the failure **names the differing field**.
- [ ] **3 · A description is a value.** `joinn-host` has no render function and no IO. *Control:* a planted `fn render(&self) -> String` in `joinn-host` fails the scan; a planted `std::fs` fails `vocab`.
- [ ] **4 · The transcript survived the move.** `joinn run calculator` — now `joinn-cli` — prints the five lines byte-identically, indent included, and `calculator.trace` replays. *Control:* the pre-P21-20 transcript, with no indent, is refused.
- [ ] **5 · The intent set is the body's.** A host emitting an intent outside `intent_set(body)` is refused naming the address. *Control:* the two declared intents are accepted.
- [ ] **6 · Signals are declared.** A body reading an undeclared signal is refused naming it. *Control:* under the host that declares `columns`, the same body runs.
- [ ] **7 · `joinn-run` is gone.** The crate does not exist and nothing references it. *Control:* a planted `joinn-run = { path = … }` in any `Cargo.toml` fails the check.
- [ ] **8 · Every control is an artifact.** Across gates 1, 2, 2.1, 2.2 and 3, every item's control points at a file, a body, a cell or a fixture. *Control:* a gate item whose control is a bare expression fails the build naming the item.
- [ ] **9 · The path of truth.** `cargo xtask gate all` runs phases 0, 1, 2, 2.1, 2.2 and 3 and writes `gates.lock` from per-gate outcomes. *Control:* a lock with a wrong score makes the item fail, and an item whose own control passes fails the build.

**End of Bootstrap 0.** When gate 3 passes, JoInn is a correct, verifiable, host-independent platform with a text editor for a face. Nothing about it is exciting to look at and everything about it is true.

---

## 8. Risks Watched During This Phase

| Risk | Instrument | What to do when it fires |
|---|---|---|
| **The stated adversary · `present` needs to know its host** | P3-15, run on purpose, before the re-freeze | This is the roadmap's own Phase 3 adversary and it is *supposed* to be tested, not avoided. If a cell cannot describe itself without knowing what will draw it, write `present-leaks.md`, mark the adversary fired, and stop — the negotiation step it implies is unbudgeted and belongs in a plan, not in a commit |
| **New · the description becomes a format string with a struct around it** | §0's one line; item 2's control; the scan in P3-05 | A `Description` that carries a template, a width, an alignment or a pre-rendered field has already lost. Delete the field; do not make it optional |
| **New · the refactor moved something and P3-00 finds it** | `gate all` on the modularised tree | A dropped gate is a finding, not a merge conflict. Record which gate, which item, and whether a golden moved, **before** touching anything. A hash that moved during a mechanical refactor is the most serious kind of hash move there is |
| **New · `grandfather.txt` starts filling** | `xtask modules`, and §2.8's ban | A file that cannot obey the rule is evidence about the rule. Write the finding; do not write the line |
| **New · the two hosts share a bug** | Nothing, and that is the danger | Two hosts written in one week by one agent from one plan will agree about anything they both get wrong. The test host's value is that it is *degenerate* — no IO, no width, no formatting — so build it **first** (P3-09 before P3-10) and let the CLI be the one that has to fit |
| **Carried · a control chosen weak** | §2.6, and P3-03's separating sample | R39 is still open. An artifact control is readable, which is most of the defence |
| **Carried · the floor grows a habit** | The petition procedure; the amendment record | Unchanged. A host is not a reason to add a primitive, and a commit that tries has misread §1 |
| **3 · The live engine is too slow to build with** | `perf`, now four probes | Unchanged. The finding is written by a human, then the decision is made |

---

## 9. What This Plan Refuses To Do

| Refused | Because |
|---|---|
| Let `present` return a `String` | §2.1. It is the whole phase |
| Build a renderer, or anything that draws | §1. Phase 6, and the intervening phases change what is drawn |
| Delete `joinn-run` before the transcript runs under `joinn-cli` | §2.5. V28 is not a thing you re-establish afterwards |
| Let a host declare its own intents | §2.2. V9 would go back to being unfalsifiable until Phase 11 |
| Ship an empty signal set | §2.3. A mechanism never exercised prints the same word as one that works |
| Fill `corpus/testimony/` | Still Phase 12. P3-14 removes the writer that made it easy |
| Add a line to `grandfather.txt` | §2.8 |
| Amend the floor | §1. The allowance was spent in 2.1 and was not renewed |
| Design the Visual Host's organelles because `Description` looks close | §1. It is close. That is why the fence is here |
| Rebless a golden hash | Still §6.2 of the Phase 0–1 plan. P3-18's moves are deliberate corrections, recorded and dated — which is not reblessing, and the distinction is the whole discipline |

---

## 10. Open Items This Plan Creates

Continuing from R43.

| ID | Topic | Question |
|---|---|---|
| **R44** | What is in a description? | §2.1 fixes cell, instance, ports, frames, values, names, labels, roles. A renderer will want more — extent, ordering, grouping, which port is "primary." Are those regulatory fields of the description, or are they the host's own layout decisions computed *from* the description? The answer decides whether Phase 6's cut reads descriptions or replaces them |
| **R45** | Is `role` a closed enum? | §3.3 freezes four roles so Phase 8 inherits a vocabulary. A creator will want a fifth within a week of Phase 9. Is `role` a closed set like the floor — irreducible and opposed — or is it a frame, with a signature and an admission rule? If the latter, does a role have laws? |
| **R46** | Does a host owe a refusal a description? | P3-09 captures the membrane's refusal of `"two"` as a `Description` with `role refusal`. That makes a refusal a *thing that can be shown*, which is Part II's promise and also a new obligation: must every refusal be describable? A refusal carrying a counter-example the host cannot render is a refusal that cannot be shown, and G7's thirty minutes probably turn on exactly that |
| **R47** | Is the environment body admitted by the gate? | §2.3 makes the environment a body. A body has DNA, and DNA is admitted. Is the environment body gated like any other — laws, witnesses, a hash — or is it declared, like the floor? If gated, what are a signal's laws? If declared, the platform has a second declared thing and R32's question about what witnesses a declaration applies to it too |
| **R48** | What does `probe` cost at scale? | §2.4 makes `probe` free of budget because it cannot fire. At Phase 7, the cut probes thousands of bodies per tick. A free operation that is called a million times is not free, and the snapshot budget of R10 may need to cover probing as well as drawing |

**Touchpoints with existing items.** **R14 begins** at P3-07 with a mechanism and one signal; Phase 11 fills it. **R15 begins** at P3-09 — the witness-capture harness exists, years before the pixels it will compare, which is what the roadmap asked for. **R39** is advanced but not closed by P3-03: one subtler counterfeit is evidence, not a measure. **R41** is unchanged and `corpus/testimony/` stays empty. **R43** is answered in part by §5.2's third number: a refusal that names the first differing field is a refusal that knows what acceptance would have looked like. **R35** is untouched and is the one open item that could retroactively unmake a Phase 2.2 decision; it is not chased here.

---

## Appendix A · `AGENTS.md` and `.cursor/rules/joinn.mdc` for Phase 3

Replace both files with this. **Rule 14 is discharged at P3-12 and removed in that commit.**

```markdown
# JoInn — standing rules

This repo is JoInn. Phases 2–2.2 built a universe that is true and cannot be
seen, and a temporary runner has been showing it anyway. Phase 3 defines what
that thing is: A HOST IS PROVED BY THERE BEING TWO. Its one idea is that A
DESCRIPTION IS A VALUE, NOT TEXT. The build plan is
docs/Plans/JoInn Phase 3 Implementation Plan.md. Work one numbered commit at a
time. Do not start the next one.

There is no renderer, no second body, no assay, no testimony corpus and no
compiler in this phase.

## Hard rules

1. A refusal is a VALUE (`Verdict::Refused`), never a Rust `Err` and never a
   panic. `Result` is for host errors only: IO, malformed input, bugs.
2. Never `unwrap`, `expect`, or panic outside tests. `#![forbid(unsafe_code)]`.
3. No `f32`/`f64` anywhere. No `HashMap`/`HashSet` — `BTreeMap`/`BTreeSet` only.
4. No wall clock and no unseeded RNG in crates/. `xtask` MAY measure wall time;
   it cannot reach a hash, a canonical form, a sample or a refusal. Message
   delivery order is a function of the body's grant list, never of arrival time.
5. No `#[derive(Hash)]` and no `serde` derive on any DNA type, and none on a
   Description. The canonical writer is hand-written and tested.
6. Never change a golden hash in corpus/ to make a test pass. If a hash moves,
   stop and report it. A hash that moves because an artifact was CORRECTED on
   purpose is recorded with a dated finding — that is not reblessing.
7. Never weaken, skip, `#[ignore]`, or delete a test to make a build green.
   Report the failure instead.
8. A primitive name must never appear in a coding region. Laws may name: the
   cell under definition (`self`), frame signature operations, and other coding
   regions BY HASH. A BODY may name primitives — that is what `prim:` is for.
9. Display names, literals, prompts, styles, labels, roles and layout are
   REGULATORY.
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
14. Nothing may depend on joinn-run. It is deleted at P3-12. REMOVE THIS RULE IN
    THAT COMMIT.
15. `std::io` AND `std::fs` APPEAR ONLY IN HOST CRATES AND xtask. joinn-host is
    protocol and does none. joinn-test-host captures into memory and does none.
    joinn-frame, joinn-dna, joinn-gate, joinn-prim and joinn-live do none.
    `cargo xtask vocab` enforces this by path.
16. A nested activation holds NO grants and can reach no capability.
17. NO INSTRUMENT WRITES ITS OWN FINDING. `xtask` prints numbers to stdout. No
    xtask subcommand writes a file under docs/Findings/.
18. EVERY MUTANT NAMES THE CHECK THAT CAUGHT IT, and registers a negative
    control the check must ACCEPT.
19. EVERY CHECK IS OPPOSED. A check declares a witness it must refuse and a
    witness it must accept, and fails the build if it cannot do both.
20. NO INSTRUMENT CHOOSES ITS OWN INPUTS. `Drive::bound` is a `NonZeroU32`.
    Every seal carries a COUNTERFEIT body that `agree` must separate from the
    reference at the declared bound; a counterfeit that is not separated is a
    BLIND SEAL and fails the build.
21. A SEAL NAMES THE CELL THAT DECLARES ITS SEALED ALLELE, found by lookup;
    zero matches and two matches both refuse.
22. A CORPUS CELL IS ONE THE GATE ADMITS, and an allele it carries is one the
    gate admits against it. The counterfeit corpus under
    corpus/phase22/counterfeit/ is refused ON PURPOSE and is never a golden.
23. AN ALLELE MAY NAME ONLY MATTER. `hash` and `resolve` are physics and are not
    in the allele-visible native set.
24. A CONTROL IS AN ARTIFACT, NOT A PREDICATE. A gate item's control is a file,
    a body, a cell or a fixture directory that the item points at, and a
    reviewer can read it. A control that is an expression evaluated beside its
    check is not opposition; it is the same assertion written twice.
25. MODULE.RS BESIDE MODULE/, NEVER MOD.RS. A leaf file holds at most one named
    production function. A capsule root (`foo.rs` with `foo/` beside it) holds
    types, `mod` declarations, `pub use` and getters. `lib.rs` is a facade with
    zero production functions; `main.rs` holds `main` only. `#[cfg(test)]` does
    not count. One `impl Trait for T`'s required methods may share a leaf.
    `allow(modules): <reason>` silences one file and a bare silencer fails.
    Crate-level `tests/` is exempt from the one-function rule but not from the
    mod.rs ban. `cargo xtask modules` enforces this, running its three fixtures
    BEFORE the scan. NEVER ADD A LINE TO grandfather.txt — a file that cannot
    obey the rule is evidence about the rule, and it goes in docs/Findings/.
26. NO CHECK IS SCOPED TO ONE FILE unless that file is its subject. A check that
    scans "the source" scans a directory tree. Rule 25 guarantees files keep
    splitting, so a file-scoped check has a shrinking domain by construction.
27. A DESCRIPTION IS A VALUE, NOT TEXT. `Description` is constructed only in
    joinn-host. It carries no template, width, alignment or pre-rendered field.
    A host renders a description; it never builds one. There is no
    `String`-returning render function in joinn-host.
28. A HOST NEVER INVENTS AN INTENT. The intent set is derived from the body.
    A host emitting an intent outside it is refused naming the address.

## Definition of done

A commit is done when the plan's "done when" command passes and you have
reported what it printed. "It compiles" is not done. "The tests pass" is not
done if the test does not refuse anything. A number is not done if the code that
printed it also chose it. A refusal is not done if nothing was ever shown to it.
AND A CONTROL IS NOT DONE IF A REVIEWER CANNOT READ IT — if the control is an
expression rather than a file, the item is asserted twice, not opposed.
```

## Appendix B · Directory layout after Phase 3

```
D:\JoInn\
├── docs\
│   ├── Plans\
│   │   ├── JoInn Phase 2 Implementation Plan.md
│   │   ├── JoInn Phase 2.1 Implementation Plan.md
│   │   ├── JoInn Phase 2.2 Implementation Plan.md
│   │   └── JoInn Phase 3 Implementation Plan.md        ← this file
│   └── Findings\
│       ├── the-floor.md            floor-vocabulary.md
│       ├── phase-2-review.md       phase-2.1-review.md
│       ├── phase-2.2-review.md     phase-2.2-hashes.md
│       ├── surviving-mutants.md    canonical-form-changes.md
│       ├── counterfeit-strength.md      ← new, P3-03 (R39)
│       ├── present-leaks.md             ← new IF the adversary fires, P3-15
│       ├── decisions.md                 ← new, P3-17 (cuttable)
│       ├── turn-annotations.md     live-engine-performance.md
│       └── witness-refreeze.md
└── joinn\
    ├── corpus\
    │   ├── phase0\  phase2\  phase21\  phase22\
    │   ├── phase3\                      ← environment body, columns_reader.body
    │   ├── descriptions\                ← NEW file kind, hashes hand-computed
    │   ├── testimony\                   (still only .gitkeep)
    │   └── transcripts\
    ├── crates\                          ← eight: joinn-run out, three hosts in
    │   ├── joinn-frame  joinn-dna  joinn-gate  joinn-prim  joinn-live
    │   ├── joinn-host\                  ← protocol. No IO. Builds descriptions
    │   ├── joinn-cli\                   ← bin joinn. IO. Renders descriptions
    │   └── joinn-test-host\             ← headless. No IO. Captures descriptions
    └── xtask\
        ├── module_fixtures\             ← the opposed fixtures for rule 25
        └── src\ ( fns\  modules\ )
```

## Appendix C · Glossary delta for code

| Theory term | Rust identifier | Notes |
|---|---|---|
| a cell describing itself | `joinn_host::Description` | a VALUE with a canonical form and a hash (§2.1). Built only in `joinn-host` |
| the face of a port | `PortFace` | position, direction, frame, value, name, label, role. Names and labels are regulatory |
| the accessibility face | `label`, `role` | regulatory; `Role` is closed at four in this phase (R45) |
| what a host shows | `Host::present(&Description)` | the host decides how; the description decides what |
| inspection | `probe(&BodyState, Address)` | read-only, no budget, cannot fire (§2.4) |
| raw input → intent | `Host::intend(RawEvent) -> Verdict<Option<Intent>>` | `RawEvent` is the one host-specific type in the protocol |
| the intent vocabulary | `intent_set(&Body)` | derived from the body, never declared by a host (§2.2) |
| the environment body | `corpus/phase3/environment.body`, `Signals` | a body, not a struct. The host declares which out-ports it emits (§2.3) |
| the degenerate host | `joinn-test-host` | no IO, no width, no formatting. Built BEFORE the CLI, on purpose |
| the module rule | `cargo xtask modules`, `module_fixtures/` | rule 25. Fixtures run before the scan |
| the decision ledger | `cargo xtask decisions`, `docs/Findings/decisions.md` | Law 2 applied to decisions. Prints a reversal count; writes no file |

---

*JoInn Phase 3 Implementation Plan (Draft 0.1). Delivers the roadmap's Phase 3 and closes F19–F25 of `docs/Findings/phase-2.2-review.md`. Builds on the Phase 2, 2.1 and 2.2 Implementation Plans and Parts I–IV. Everything marked PROPOSED · yours is a recommendation made while writing this plan; Cursor implements what the roadmap, the grammar document and your decisions say, not what this plan prefers.*
