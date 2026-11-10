# JoInn Phase B Implementation Plan

**One running hypernode: the contact file is the calculator, `lower` compiles it, the wired body witnesses the compilation**

Draft 0.1 · 30 Oct 2026 · written from the [Hypernode Refactor Plan](JoInn%20Hypernode%20Refactor%20Plan.md) (Phase B) for AJ's review

> **Status: a proposal for AJ's review. Nothing in this plan is built yet.** Every decision below is tagged **DECIDED** (AJ already decided it in the refactor plan or in earlier theory) or **PROPOSED** (this draft's reading, open to AJ's veto). When AJ has confirmed or changed each PROPOSED item, the plan becomes Draft 1.0 and runs the way Phase 7.4 ran: one unit, one stop, no question asked mid-phase; a step that can't be done as written follows §0.3 (Snags).

**Thesis.** The calculator has one source, the contact file; `lower` is its compilation into the live runner; the wired `.body` is kept only as the declared witness that the compilation is right.

---

## For AJ: this plan in plain English

**What Phase B is.** Today the calculator exists twice. `corpus/phase7/calculator.contact` is the hypernode: two blocks in contact and a force pointing at the result. `corpus/phase2/calculator.body` is the older wired form. Gate 7 already proves they are the same calculator: `lower` turns the contact into exactly the wired file, byte for byte. But the project still tells two stories. `joinn run calculator` runs the wired file, though the guides (since 30 Oct) already call the wired file the witness of `lower`'s compilation. And `joinn/AGENTS.md` has two rules that disagree: rule 32 says *inside a body is a wire*, rule 61 says *a body has no wires*.

Phase B makes it one story, without deleting anything:

| Today | After Phase B |
|---|---|
| `joinn run calculator` runs the wired file | `joinn run calculator` runs the contact file, compiled by `lower`. The transcript is the same, byte for byte |
| The wired file is found because it shares the stem `calculator` | The wired file is **declared** as the witness of `lower` for the contact, in one table |
| Nothing refuses a `lower` that drifts | `cargo xtask contact` plants a drifting `lower` on every run and must refuse it |
| Rules 32 and 61 contradict each other | One wording: inside a hypernode, blocks touch and the file has no wires; `lower` compiles contact into deliveries; between hypernodes, a link is a hyperedge |
| Guides say the wired file is the witness, and that `joinn run calculator` still runs it | Guides say `joinn run calculator` runs the contact, and the wired file is the declared witness |

**What you'll see.** Nothing new on screen. In a terminal, `joinn run calculator` prints the same lines it always has (`"two"` refused, then `2 + 3 = 5`), and `cargo xtask contact` prints one new line naming the witness and one new line saying a planted bad compilation was refused.

**What I decided (say if you disagree):**

- **Keep the wired file, and keep every gate that uses it.** Gates 2 and 3 are legacy; gates 4, 5 and 6 check facts about the compiled form (its assay, its surface, its picture). They stay exactly as locked. The wired file stops being the *default*, not stops existing.
- **Declare the witness in a table in `xtask`**, not by a file-name coincidence. Where witnesses should finally live is already a research question (R118); this is the small step.
- **`joinn run calculator` prefers the contact.** A stem that names a contact (one that doesn't grow) runs the contact; otherwise the `.body`, as today. `units` and every other stem behave as before.
- **The wired file can still be run by path** (`joinn run corpus/phase2/calculator.body`), so anyone can see the two forms agree.
- **Change the refusal words for a `wires` section.** Today they say "wires belong to systems", which Phase 7.4 made untrue (rule 91: *forces* belong to systems). This is the one PROPOSED change you might prefer to leave; §2.7 gives the fallback.
- **One gate item**, `phase B: 1/1`. Its subject is the contact; its mutant renames `sum`, which the witness must catch.

**What you do at the end** (about two minutes, §6.2): run `joinn run calculator`, type `two`, `2`, `3`, and read `cargo xtask contact`'s witness line.

---

## 0. How Cursor Works This Plan

| | |
|---|---|
| **Plan file** | `D:\JoInn\docs\Plans\JoInn Phase B Implementation Plan.md` |
| **Parent plan** | `docs/Plans/JoInn Hypernode Refactor Plan.md`, Phase B |
| **Code** | `D:\JoInn\joinn\`. No new crate. No new external dependency. No identifier renamed |
| **Standing rules** | `AGENTS.md` as updated by PB-01 and PB-07 (Appendix A) |
| **Unit of work** | the phase: every commit of §4 in order, each its own git commit whose message starts with its id (`PB-04: …`). **One stop**, at the end |
| **End of the phase** | `cargo xtask stop-check --fresh`, the stop report (§0.2), commit, push, ledger line, print `phase B: stopped for review`, stop |
| **Who decides** | every decision is in §2. Cursor decides module layout, function bodies, private representations, and error wording where §2 gives none |
| **How each commit is checked** | Run 7.3–9 §3 (AGENTS.md S1–S4): the done-when, `cargo xtask check`, and the gates scoped to the files changed |
| **Who checks** | Claude, at the phase stop, from a fresh clone. CI on the push, read once. AJ at the stop (§6.2) |
| **Input** | Never move the system mouse, click, or type outside a window Cursor started (rule 67). This phase opens no window |

**Numbering used by this plan.** At writing, the highest invariant in `docs/Findings/decisions.md` is **V173**, the highest research item in `docs/Theory/JoInn Research Backlog.md` is **R133**, and the highest standing rule in `joinn/AGENTS.md` is **93**. The three refactor phases take non-overlapping blocks in order: **Phase B: V174–V176, R134–R136, rule 94.** Phase C: V177–V180, R137–R139, rule 95. Phase D: V181–V184, R140–R143, rule 96. (Phase 9's older plan still names V160–V166 and R111–R114, which are taken; it renumbers when it runs, as Phase 8 did.)

### 0.1 The commit report

Every commit ends with Phase 7.3's block in its message, every value copied from the terminal:

```
Commit:     PB-NN (hash in git log)
Done-when:  <the command> → <the line it printed>          MET | NOT MET
Check:      <the block cargo xtask check prints, whole>  |  docs only
Gates:      <each scoped gate run → its phase line>  |  none
Snags:      none | <each thing that went differently from the plan, with the printed line>
```

From PB-02 on, `cargo xtask check` also prints `Contact:` (`cargo xtask contact → <last line>`).

### 0.2 The stop report

Run 7.3–9 §4, the short form, at `docs/Findings/phase-B-stop.md`. Commit it as `PB-stop: stop report`.

### 0.3 Snags

As Phase 7.4's §0.3. Write the snag in the next commit report with the exact output, take the plan's stated fallback if there is one, and keep going. A prediction that differs is pasted beside the plan's value. Never widen a check to make it pass, never edit `gates.lock` by hand, never move a corpus hash (rule 6).

---

## 1. What Already Holds, and the Scope Fence

### 1.1 What already holds (Phase B does not re-prove it)

| Fact | Where it is checked |
|---|---|
| `lower(calculator.contact)` prints exactly as `phase2/calculator.body`, hash `b55fba1e…` | Gate 7 item 1 (`g7_forms`, V128); test `the_calculator_lowers_to_the_wired_calculator_byte_for_byte` in `joinn-link/src/lower.rs` |
| Both forms print `corpus/transcripts/calculator.txt` and give the same descriptions and surface (`cli_a@0`, `cli_b@0`, `sum@2`) | Gate 7 item 1; `cargo xtask contact` (`contact: 1 body(ies); two forms, one truth` at the Phase 8 stop) |
| A `.contact` refuses a `wires` section | V126; gate 7 item 1 |
| Member order is never hashed; combine is order-blind | V127, V129; gate 7 item 2 |
| The shell's run path keeps tables equal to regrow on both forms (rows `2, 3, 4` wired, `2, 1, 3` contact) | V125 |
| A link with every member in one body is refused by Law 4, whose acceptance is a wire inside that body | `check_law4` in `joinn-link`; gate 5 item 6 (`WireAcross("e0")`) |
| `corpus verify: 67 hash(es) match; cells admitted` | Phase 8 stop |
| `gates.lock`: phases 0 … 8 as locked, phase 7 `3/3` | `gates.lock` |

### 1.2 What still treats the wired calculator as primary

Found by searching the tree for `calculator.body` and `phase2/calculator` (30 Oct 2026):

| Place | What it does | Phase B |
|---|---|---|
| `joinn run calculator` (`joinn-cli/src/load/load_body.rs`, by stem) | runs `phase2/calculator.body` | **changes** (PB-04): runs the contact |
| `cargo xtask contact`'s `lowered_line` (`xtask/src/fns/contact/lowered_line.rs`) | finds the witness by the stem `calculator.body` | **changes** (PB-02): reads the declared witness |
| `joinn/README.md` line 7; `docs/Guides/01-what-is-joinn.md` line 58; `docs/Guides/02-how-parts-connect.md` section 5 | call the wired file the witness of `lower`'s compilation (fixed 30 Oct) and say `joinn run calculator` still runs it | **changes** (PB-09): the "today … Phase B makes the contact the default" sentence becomes "`joinn run calculator` runs the contact" |
| `docs/Guides/04-try-the-calculator.md` lines 24, 30 and 78 ("the CLI only knows `joinn run calculator` right now"); `joinn/README.md` lines 26 and 34; Guides 02 lines 125, 158 and 189; Guides 03 lines 38 and 45 | tell a reader `joinn run calculator` and, separately, `joinn run corpus/phase7/calculator.contact` | **changes** (PB-09): one command, which runs the contact; the path form is named as the way to run the witness |
| Gates 2 and 3 (legacy): `g2_bodies`, `g3_intent` name `corpus/phase2/calculator.body` | legacy checks (rule 24) | **stay**; their checks must still pass |
| `xtask/src/fns/run_calculator_bin.rs` runs `cargo run -p joinn-cli --offline --quiet -- run calculator` (stdin `two`, `2`, `3`) for gate 3's host and transcript checks; `load_calculator.rs` reads the wired file for them | legacy evidence | **stays**; after PB-04 the same command runs the contact, and P1 predicts the same bytes |
| Gate 4 item 1 (`SwapCell("cli_b", …)`), gate 5 item 2 (`DropWire("cli_a@1", "sum@0")`), gate 6 items 1–3 (`DropWire("cli_b@1", "sum@1")`, `DropGenome("sum")`, `DropGenome("cli_b")`) | facts about the compiled form, artifact `corpus/phase2/calculator.body` | **stay**: their subject is now named the witness, and their locked pairs do not move |
| 13 corpus universes bind `body:b55fba1e…` (`phase4/loop.universe`, `phase4/loop_declared.universe`, `phase5/universe.universe`, `phase5/ordered.universe`, `phase5/adversary.universe`, `phase5/alone.universe`, and seven under `phase5/controls/`) | a universe binds compiled bodies by hash (rule 36) | **stay**; V176 checks that hash is `lower`'s |
| `phase2/variants/calculator_{b,c,d}.body` | spellings that share the one hash (gate 2) | **stay**; they are not witnesses |
| `xtask` `pick`, `regrow`, `g72_zoom`, `layout` use `phase2/calculator.body` as the Phase 6 body picture | the compiled form's picture | **stay** |
| Fixtures and tests reading the wired file: `joinn-visual/src/fixtures/calculator.rs`, `joinn-gpu/src/fixtures.rs`, `joinn-shell-desktop/src/session/fixtures.rs`, `joinn-test-host/tests/calculator.rs` and `one_face.rs`, `joinn-host/tests/*`, `joinn-link/tests/{assemble,law4,adversary,crossing}.rs` | tests of the compiled form | **stay** |

### 1.3 In scope

- Rules 32 and 61 rewritten as one statement, after the code has one source (§2.1).
- The witness of `lower` declared in one table (§2.3), and a standing plant for a drifting `lower` (§2.6).
- `joinn run calculator` runs the source; the witness runs by path (§2.4).
- The refusal words for a `wires` section (§2.7, PROPOSED).
- Gate B, one item (§6). Findings `docs/Findings/phase-B-one-source.md`. The guides and README tell one story (§2.9).

### 1.4 Out of scope

| Not now | Why it is tempting |
|---|---|
| Renaming `body`, `contact`, `lower` or any identifier | A rename is its own phase (refactor plan header) |
| Deleting or moving `calculator.body`, or any corpus file | The wired file is the witness through Phase B (refactor plan, "What this plan leaves alone"); rule 6 |
| Re-pointing gates 4, 5, 6 at the contact | Their `(artifact, opposes)` pairs are locked; a re-pointed row would restate gate 7 (rule 41) |
| Serving universes from `lower` instead of the witness file | R134. Touches every universe path and the grove |
| Drawing the force as a lasso in the contact picture | Phase C |
| One relationship type for link and force | Phase D |
| A shell default (the window opens only the path it is given) | `joinn-desktop` has no default file today; nothing to change |

---

## 2. Decisions

### 2.1 Rules 32 and 61: one statement (**DECIDED** that Phase B resolves it; wording **PROPOSED**)

The refactor plan schedules the rule edit for this phase, "when the code has one source of truth". The two rules are both true of different files:

- Rule 61 is true of the **source**: a `.contact` has no wires, and its parser refuses a `wires` section.
- Rule 32 is true of the **compiled form**: the deliveries `lower` derives are wires, a wired `.body` is that form, and a universe binds compiled bodies by hash. Law 4's refusal (`link … has all members in body …; acceptance is a wire inside body …`) is about universes, which bind compiled bodies.

The resolution says which statement holds where: **inside a hypernode, blocks are in contact and the file has no wires; the engine compiles contact to deliveries with `lower`; between hypernodes, relationships are hyperedges.** Appendix A gives the text. It is applied at PB-07, after the code has one source (PB-02, PB-04) and gate B holds it (PB-06), not at PB-01.

### 2.2 One source, one compilation, one witness (**DECIDED**, refactor plan Phase B)

| Role | File or function | Rule |
|---|---|---|
| Source | `corpus/phase7/calculator.contact` | what entry points run |
| Compilation | `joinn_link::lower(contact, cells, frames) → Verdict<Body>` | never written to `corpus/` or `docs/`; printed only to be compared |
| Witness | `corpus/phase2/calculator.body` | declared; compared byte for byte; never edited to match |

### 2.3 The witness is declared (**PROPOSED**)

A new leaf, `xtask/src/fns/contact/witnesses.rs`, holds one table of `(source, witness)` corpus-relative paths. In this phase it has one row: `("phase7/calculator.contact", "phase2/calculator.body")`.

- `lowered_line` takes the declared witness instead of searching for `<stem>.body`. Its printed line becomes `witness phase2/calculator.body = lower(phase7/calculator.contact) (b55fba1e…)` when the canonical texts are byte-identical, and keeps its existing first-differing-line refusal otherwise (`lowered differs from <witness> at line <n>: …`).
- Every non-growing corpus contact has exactly one row. A contact with none is refused: `contact: <rel> declares no witness; acceptance is one (source, witness) row`. A row whose file is missing is refused naming it.
- A growing contact (`grows`, Phase 7.4) is a system's body and has no witness, as `cargo xtask contact` already skips it.
- Where witnesses finally live (a corpus file with lineage, as Phase 8's witness files are) is R118's question and stays open.

### 2.4 The CLI runs the source (**PROPOSED**)

`joinn-cli` resolves a bare name in this order: the unique non-growing `<name>.contact` under `corpus/`, else the unique `<name>.body`, else today's refusal (`no body file named <name>.body`). Two contacts with one stem are refused naming both, as two bodies are today.

- `joinn run calculator` therefore runs `lower(calculator.contact)`. `lower`'s body has the wired body's coding and the contact's regulatory region, which equals the wired file's (test `the_contact_calculator_loads_as_the_wired_calculator` in the shell asserts both), so the transcript and the trace are predicted byte-identical.
- `universe` still goes to `run_universe` first (`joinn-cli/src/session/run_named.rs`), before any stem search.
- `units`, and every stem with no contact, resolves as today. `counting` and `adding` (growing contacts) are skipped by the contact search and refused as today.
- The resolution is one new leaf in `joinn-cli/src/load/` returning the corpus-relative path it chose, so a test (and gate B's check, through `xtask`'s existing `joinn-cli` dependency) can name the file without parsing output.
- **The witness runs by path:** `joinn run <path>.body` loads that file, as `joinn run <path>.contact` does today. The usage line becomes `usage: joinn run <name> | joinn run <path>.contact | joinn run <path>.body`.

### 2.5 What `cargo xtask contact` prints (**PROPOSED**)

Per non-growing contact, in this order: `<rel>: hash <hex>, admitted`; the `--- lowered ---` block (stdout only, as today); the witness line (§2.3); `transcript equal` (the source, as today); **`witness transcript equal`** (new: the witness run by path); the description, surface, pairing and mutant lines as today. Last line:

```
contact: 1 body(ies); two forms, one truth; planted diverging lower: refused (ok)
```

### 2.6 A drifting `lower` is a standing plant (**DECIDED** by rule 93; the plant **PROPOSED**)

No admitted artifact can make `lower` itself drift, so `cargo xtask contact` plants it on every run (rule 93): `lower` with each force's members fed to the response's in-ports in **reverse** canonical order (the existing `lower_paired` helper already permutes in-ports). The witness comparison must refuse it, naming the first differing line (`cli_a@1 -> sum@1` where the witness has `cli_a@1 -> sum@0`). The plant's sum is still `5` (combine is order-blind), which is the point: only the witness sees this drift.

### 2.7 The refusal words for a `wires` section (**PROPOSED**; AJ may decline)

Today (`joinn-dna/src/body/contact/contact_coding.rs`, mirrored in `xtask/src/fns/g7_forms.rs`'s `WIRES` and a test in `joinn-dna/src/body/contact.rs`):

```
contact: wires belong to systems; a body's cells touch. acceptance is a forces section
```

Proposed:

```
contact: a contact has no wires; its cells touch, and lower derives the deliveries. acceptance is a forces section
```

Code names stay (`contact`, `cells`, `lower`), as the refactor plan requires. No hash moves (a refusal string is not hashed). Gate 7 item 1's check changes its `WIRES` constant in the same commit; its `(artifact, opposes)` pair and score do not change. **Fallback if AJ declines:** PB-05 is not made, Appendix A's rule 61 quotes today's words, and the finding records the mismatch with rule 91 as open.

### 2.8 What stays as it is (**DECIDED**: rules 6, 24, 41; refactor plan "What this plan leaves alone")

Everything in §1.2 marked **stay**. In particular: no `gates.lock` line other than the new `phase B` row; no corpus byte; gates 1–3 stay legacy and their checks must pass; gate 7's three pairs stay.

### 2.9 One explanation in the guides (**DECIDED**, refactor plan Phase B exit)

At PB-09, the wording everywhere is: *The contact file is the source. `lower` compiles it into the deliveries the live engine runs. The wired `.body` is the compilation witness: a declared file the compilation must equal, byte for byte.* §1.2 lists the lines. The historical phase record in `03-where-we-are.md` stays as written; Phase B adds its own section.

### 2.10 Invariants

| # | Invariant | Commit |
|---|---|---|
| **V174** | The contact file is the calculator's one source: `joinn run calculator` resolves to `phase7/calculator.contact` and runs `lower` of it | PB-04 |
| **V175** | The wired body is a declared witness of `lower`: for every declared pair, `lower(source)` prints byte for byte as the witness and hashes alike; a source that diverges and a `lower` that diverges are both refused | PB-02, PB-06 |
| **V176** | A universe that binds the calculator binds the hash `lower` derives from the source: all 13 corpus universe files that name `b55fba1e…` name `hash(lower(calculator.contact))` | PB-06 |

---

## 3. What Runs

| Command | Prints | Exit 1 when |
|---|---|---|
| `cargo xtask contact` | §2.5's lines; last line `contact: 1 body(ies); two forms, one truth; planted diverging lower: refused (ok)` | any inequality, a contact with no declared witness, or the plant admitted |
| `cargo run -p joinn-cli -- run calculator` | `corpus/transcripts/calculator.txt`, byte for byte, under the script `two`, `2`, `3` | — |
| `cargo run -p joinn-cli -- run corpus/phase2/calculator.body` | the same transcript | — |
| `cargo xtask gate B` | the item's row, `item 1 control answered by check`, `phase B: 1/1` | the item fails |
| `cargo xtask check` | as today, plus `Contact: cargo xtask contact → <last line>` | as today |

**Predictions** (each marked `as predicted` or `differs` in the findings, PB-08):

| # | Prediction |
|---|---|
| P1 | `joinn run calculator`'s transcript after PB-04 equals `corpus/transcripts/calculator.txt` byte for byte; `transcript_matches_golden` in `joinn-cli` passes unchanged |
| P2 | The trace bytes `run_session` returns for `calculator` are the same before and after PB-04 |
| P3 | `cargo xtask contact` prints `witness phase2/calculator.body = lower(phase7/calculator.contact) (b55fba1e…)` |
| P4 | The diverging-lower plant is refused at line 11 of the printed body, the first line inside `wires { … }`: lowered `cli_a@1 -> sum@1`, witness `cli_a@1 -> sum@0` (Cursor pastes the real line) |
| P5 | Gate B's mutant `RenameAlias("sum", "total")` is admitted, and the control answers by its check at the genome line `cell:6b3271… as total` |
| P6 | 13 universe files bind `b55fba1e…`, and each equals `hash(lower(calculator.contact))` |
| P7 | `corpus verify: 67 hash(es) match; cells admitted`, unchanged at every commit; `git diff --stat <PB-01>..HEAD -- joinn/corpus` prints nothing |
| P8 | `gate all` prints every earlier phase as `gates.lock`, then `phase B: 1/1` |

---

## 4. The Commits

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **PB-01** | **Plan and header** | This plan (Draft 1.0 after AJ's review). `AGENTS.md` header per Appendix A part 1 only. Appendix B: backlog R134–R136; `decisions.md` V174–V176 and R134–R136 (`open`). No code | `git show --stat HEAD` lists this plan, `AGENTS.md`, the backlog, `decisions.md`. `Check: docs only` |
| **PB-02** | **The declared witness and the plant** | §2.3: `contact/witnesses.rs`; `lowered_line` reads it (and `g7_forms`'s call passes the declared witness); §2.5 witness line; §2.6 plant; `check` gains `Contact:` | `cargo xtask contact` printed whole (paste): the witness line, and the last line as §2.5. Tests: every non-growing corpus contact has exactly one row; a contact with none is refused with §2.3's words. **Shown then reverted:** the plant not planted: `contact` exits 1 naming the missing plant. `gate 7` → `phase 7: 3/3` |
| **PB-03** | **The witness runs by path** | §2.4's `<path>.body` form in `joinn-cli`; the usage line; `contact` prints `witness transcript equal` | Test `witness_transcript_matches_golden` (new, in `run_session.rs`) passes. `cargo xtask contact` prints `witness transcript equal` (paste). `gate all` (joinn-cli is a foundation crate for S1) → every line as `gates.lock` |
| **PB-04** | **`calculator` is the source** | §2.4's resolution leaf; V174 | Tests: the resolver names `phase7/calculator.contact` for `calculator`, `phase5/units.body` for `units`, and refuses `counting` as today; `transcript_matches_golden` passes unchanged (P1); the trace is unchanged (P2, a test comparing the bytes from both resolutions). `gate all` → every line as `gates.lock` |
| **PB-05** | **The refusal words** (only if AJ confirms §2.7) | §2.7 in `contact_coding.rs`, its test in `contact.rs`, and `g7_forms`'s `WIRES` | `cargo test -p joinn-dna` passes with the new words asserted. `gate 7 --item 1` passes (paste). `gate all` → every line as `gates.lock` |
| **PB-06** | **Gate B** | §6; `phase B` after `phase 8` in `PHASE_LABELS`; `gate_b.rs`, `gate_b_items.rs`, `gb_source.rs`, `gb_source_control.rs`; the row in `gate_table`; the lock row; the cross-gate test `only_gate_seven_names_a_contact_artifact` becomes `only_contact_gates_name_a_contact_artifact` with the list `phase 7`, `phase B` (finding in PB-08); V175, V176 | `cargo xtask gate all` exits 0 and prints `phase B: 1/1` and `item 1 control answered by check`. **Shown then reverted**, with `gate B --item 1` only, pasting each failure: (a) give item 1 the pair of gate 7 item 3 (`calculator.contact`, `DropForce("sum")`): rule 41's check refuses naming both; (b) the control compares regulatory regions too: the neutral edit flips it and the harness refuses the item; (c) the resolver searches `.body` first: the check fails naming `phase2/calculator.body` |
| **PB-07** | **Rules 32 and 61** | Appendix A part 2: rules 32 and 61 replaced, rule 94 appended | `git show --stat HEAD` lists `AGENTS.md` only. `Check: docs only` |
| **PB-08** | **Findings** | `docs/Findings/phase-B-one-source.md`: every command's output from PB-02 on; *Predictions* P1–P8 marked; *The adversary* (§7.1) answered; the dated amendment of the cross-gate test (why gate B names a contact artifact); §2.7's outcome | Every prediction marked |
| **PB-09** | **Docs and freeze** | README line 7; Guides 01, 02 and 04 per §2.9; Guides 03: a `## One source (Phase B)` section above Phase 8's; glossary: *source*, *compilation witness*, *declared witness*; `decisions.md` V174–V176 with status and evidence | Tier 1. `corpus verify` 67; `git diff --stat <PB-01>..HEAD -- joinn/corpus` prints nothing |
| — | **The phase stop** | `cargo xtask stop-check --fresh` (prints phases 0 … 8, B; exits 0); `phase-B-stop.md`; commit `PB-stop`, push, ledger; print `phase B: stopped for review`; stop | — |

Dependencies: in order. PB-03 and PB-04 depend on PB-02; PB-06 on PB-04 (and on PB-05 if it is made); PB-07 on PB-06.

**If something has to be cut for time:** PB-05 first (its fallback is in §2.7), then the `witness transcript equal` line of PB-03 (the path form stays). Never cut PB-02, PB-04, PB-06 or PB-07.

---

## 5. Test Strategy

As Phase 7.4's §5. Every comparison is bytes. No test asserts only that a witness "exists"; each names the file and the first differing line, or asserts equality. The CLI resolution is tested by the path it returns, never by parsing a transcript.

---

## 6. Exit Gate B

One item in `xtask/src/fns/gate_b_items.rs`, built like gate 7.4's. It is answered **by its check, not by admission**: the mutant is admitted (`rename_alias_changes_only_that_name` in `xtask/src/fns/mutate/rename_contact_alias.rs` already shows it), and the control finds the drift.

| # | Item | `control_artifact` | `opposes` | Control answers `true` when |
|---|---|---|---|---|
| 1 | **The contact is the source; the wired body witnesses `lower`** | `corpus/phase7/calculator.contact` | `RenameAlias("sum", "total")` | the subject is refused at admission, or `print_body(lower(subject).coding)` differs from the declared witness's `print_body`, or their hashes differ |

- **On the subject** the control answers `false`: `lower` gives the witness byte for byte.
- **On the mutant** it answers `true`: `lower` names `total` where the witness names `sum`. The printed reason is the witness line's refusal, e.g. ``lowered differs from phase2/calculator.body at line 4: lowered `cell:6b3271… as total`, phase2/calculator.body `cell:6b3271… as sum` `` (line number pasted from the run).
- **On the neutral edit** (one regulatory label) it answers `false`: the control compares coding regions only.

**Check (`gb_source`):**

1. For every declared `(source, witness)`: `lower(source)` prints byte for byte as the witness, and the hash is `b55fba1eff65942099f6daf84b8bc47d605be05f637d805d260d3fcfd8c3ebde`.
2. `joinn run calculator` resolves to `phase7/calculator.contact` (the resolver's returned path) and prints `transcripts/calculator.txt`.
3. `joinn run corpus/phase2/calculator.body` prints the same transcript.
4. Every corpus universe that binds `b55fba1e…` binds `hash(lower(calculator.contact))`: 13 files (V176).
5. The §2.6 plant is refused by (1)'s comparison (rule 93), and the gate prints `planted diverging lower: refused`.

This row does not restate gate 7 item 1 (rule 41). Gate 7 item 1's control runs the *source* against the golden transcript and never reads the wired file. This row's control reads the *declared witness*, and its check adds what gate 7 cannot see: which file the product's default runs, and a drifting `lower`.

### 6.1 Conditions for opening Phase C

1. `stop-check --fresh` exits 0 with `phase B: 1/1` and every earlier line as `gates.lock`; no tripwire fired.
2. `phase-B-one-source.md` exists with every prediction marked and the adversary answered.
3. Claude's review lists no open snag, or AJ has chosen to carry each one forward.
4. AJ has run §6.2.

### 6.2 AJ's check (at the phase stop, about two minutes)

```
cd D:\JoInn\joinn
cargo run -p joinn-cli -- run calculator
```

1. Type `two`, Enter: the refusal. Type `2` and `3`: `2 + 3 = 5`. This is the contact file running.
2. `cargo xtask contact`: find `witness phase2/calculator.body = lower(phase7/calculator.contact)` and the last line ending `planted diverging lower: refused (ok)`.
3. Tell Claude whether the guides now read as one story.

---

## 7. Adversary, Risks and Tripwires

### 7.1 The adversary

**The two forms diverge.** Two ways: the source changes and the witness does not (gate B's mutant), or `lower` changes and the source does not (the standing plant). Both must be refused by the same comparison. The findings answer it with the two refusal lines.

### 7.2 Risks

| Risk | What to do |
|---|---|
| The trace from the contact differs from the wired trace (P2 fails) | A snag with both byte counts and the first differing offset. Fallback: PB-04 keeps `calculator` on the `.body`, gate B's check (2) is removed from the plan by AJ, and V174 stays `open`. Never edit `calculator.trace` |
| A legacy gate (1–3) check runs `joinn run calculator` and depends on the `.body` path | It must still pass (rule 24). If it fails, a snag with its line; same fallback as above |
| `gate_cmd` or the lock parser refuses a letter label (`phase B`) | A snag with the refusal; extend the parser in PB-06 (the label is a string everywhere the test `every_label_after_phase_0_has_one_gate_and_its_argument_is_the_label` reads it). Never renumber an earlier phase |
| Amending `only_gate_seven_names_a_contact_artifact` reads as weakening a test | It is replaced by a test with an explicit list, recorded in the finding with its date and reason (rule 7 forbids weakening to go green; this is a deliberate scope change of what the test guards) |
| `layers` refuses an edge | A snag with the edge printed; never move code to dodge it (rule 42) |

### 7.3 Tripwires

Run 7.3–9 §7 applies (T1–T5; T6 does not). For T1, **no** file under `joinn/corpus/` changes in this phase, and `hashes.txt` gains no line.

---

## 8. Open Items

| ID | Topic | Question |
|---|---|---|
| **R134** | Universes served by the compilation | A universe binds `b55fba1e…`, and today the store reads those bytes from the witness file. Should a `BodyStore` be filled from `lower(source)` instead, so the witness is only ever compared? |
| **R135** | When the witness retires | The refactor plan keeps the wired file through Phase B. What exit lets it retire as a second file: Phase G's strand round trip, Phase H's second engine, or never (a witness is cheap)? |
| **R136** | Witnesses as testimony | The declared table lives in `xtask`. Should a witness pair be a corpus artifact with lineage, so a person's own hypernode can declare one? Joins R118 |

---

## Appendix A · `AGENTS.md` changes

**Part 1 (PB-01).** Replace the header's third paragraph ("The active plan is … The next implementation phase is B …") with:

```markdown
The active plan is docs/Plans/JoInn Hypernode Refactor Plan.md. Phase A (the
language) is closed. Phase B, one running hypernode, is built from
docs/Plans/JoInn Phase B Implementation Plan.md: the contact file is the
calculator's one source, `lower` compiles it, and the wired .body is its
declared witness. Work the phase's commits in order, one git commit per
numbered step, check each by tier, and stop once at the phase's end. Never stop
to ask; follow the plan's Snags section instead. Phase 8's plan is finished
work (gate 8 passed); rules 82–86 remain the storage rules.
```

**Part 2 (PB-07).** Replace rule 32 with:

```markdown
32. INSIDE A HYPERNODE, BLOCKS TOUCH; BETWEEN HYPERNODES, A LINK IS A
    HYPEREDGE. A .contact (the source) has no wires. Wires exist only in the
    compiled form: `lower` derives them, and a wired .body is that form. A
    universe binds compiled bodies by hash, so Law 4's acceptance for a link
    inside one body is a wire inside that body. Each other form is refused
    naming the container and the method it requires.
```

Replace rule 61 with (the quoted words follow §2.7's outcome):

```markdown
61. A CONTACT HAS NO WIRES. A .contact file has no wires section and its
    parser refuses one ("a contact has no wires; its cells touch, and lower
    derives the deliveries"). The engine compiles a contact with `lower`, from
    the contact, its cells and the force register only. The compilation is
    never written to corpus/ or docs/ and never drawn; it is printed only to be
    compared with its declared witness.
```

Append:

```markdown
94. A WITNESS IS DECLARED, NEVER A SOURCE. A file a derivation must equal
    (the wired calculator is `lower`'s) is named beside its source in one
    declared table. Entry points run the source. A witness that differs from
    its derivation is a refusal naming the first differing line, and a
    witness is never edited to match.
```

Keep every other rule.

## Appendix B · Documents

- **Backlog**, after R133: `> **30 Oct 2026 · Hypernode refactor, Phase B.** R134–R136 come from the Phase B plan.` then R134–R136 (`open`) with §8's questions.
- **`decisions.md`**: V174–V176 (`open`; PB-09 sets them); R134–R136 (phase `B`, `open`).
- **Refactor plan**: no edit beyond the link to this plan.

---

*JoInn Phase B Implementation Plan, Draft 0.1 (30 Oct 2026), written from the Hypernode Refactor Plan's Phase B. AJ decided (refactor plan, 30 Oct): the contact file is the source, `lower` is compilation, the wired body is the compilation witness and stays through Phase B, the rules contradiction is resolved in this phase, and code names stay. Proposed, open to AJ's veto: the declared witness table, the CLI resolution order and the path form, the refusal words, the plant, and one gate item whose mutant renames `sum`.*
