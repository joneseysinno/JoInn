# JoInn Phase 8 Implementation Plan

**Visual truth: what a body means on screen is witnessed · the screen-reader tree from the same cut · every intent by keyboard**

Author: AJ, with Claude · Draft 0.1 · October 3, 2026

> **Every decision in this plan is final.** Nothing here waits on AJ. Cursor never stops to ask AJ anything, and no file in this phase is typed by AJ. If a step can't be done the way the plan says, Cursor follows §0.3 (Snags) and keeps going. This phase runs inside `docs/Plans/JoInn Run 7.2B-9.md`; its tripwires apply.

---

## For AJ: this plan in plain English

**What Phase 8 is.** JoInn's gate already protects what a cell *computes*. Phase 8 protects what it *means on screen*: who you can click, what a screen reader says, and what you can do. Those three things are recorded once as **witnesses**, and from then on any change to a body must keep them, or it is refused. Style is free: colours, names and exact pixel positions may change.

**Three witnesses, three questions.**

| Witness | Question it answers | Truth (checked) | Style (free) |
|---|---|---|---|
| **Owners** | Who owns pixels in this view? | the set of addresses you can click | where exactly, what colour |
| **Reader** | What does a screen reader hear? | roles, values, order, what can be done | names and labels |
| **Intents** | What can you do here? | the list of inputs you can fill | how you reach them |

**Three cuts, one truth.** The picture, the clicking, and the screen-reader tree all come from the same cut of the universe. Zooming in, for a screen reader, is opening a group. Zooming out is closing it.

**Keyboard only.** The calculator becomes fully usable with no mouse: Tab moves between inputs, you type, Enter sends it. Enter on a lens tile zooms into it; Backspace zooms back out. If the focused thing is off screen, the camera brings it into view.

**Enter on an empty box** (left open since Phase 6): does nothing and the terminal says so. Nothing is sent.

**What I decided (say if you disagree):**
- **Names aren't witnessed.** A screen reader saying "Total" instead of "Sum" is a rename, which JoInn allows silently (names are regulatory since Phase 0). Saying "6" instead of "5", or losing the result, is refused.
- **Golden pixel images are dropped as witnesses.** Comparing pictures across graphics cards needs a tolerance that is either too tight or too loose. The roadmap said to pre-agree this fallback, and owners already catch everything that matters. Pixels are still compared exactly inside one run (regrow, rebase).
- **JoInn builds its own reader tree**, exact and testable; the real screen reader (Windows Narrator) gets it through AccessKit, the one new library in this run.

**What you do at the end** (about four minutes, §6.2): run the calculator with the keyboard only, and, if you like, turn on Narrator and hear it read the sum.

---

## 0. How Cursor Works This Plan

| | |
|---|---|
| **Plan file** | `D:\JoInn\docs\Plans\JoInn Phase 8 Implementation Plan.md` |
| **Code** | `D:\JoInn\joinn\`. No new crate. New external dependencies: `accesskit` and `accesskit_winit`, in `joinn-shell-desktop` only (§2.9) |
| **Standing rules** | `AGENTS.md` as updated by P8-01 (Appendix A) |
| **Unit of work** | a **chunk** (A, B or C); commit ids `P8-NN` |
| **End of a chunk** | stop report (§0.2), commit, push, ledger, tripwires, continue per the run plan |
| **Who decides** | every decision is in §2. Cursor decides module layout, function bodies, private representations, the focus ring's shape, and error wording where §2 gives none |
| **Who checks** | Claude after the run, from a fresh clone; CI on every push; AJ at the end of the run |
| **Input** | rule 67 |

### 0.1 The commit report

As Phase 7.2's §0.1, plus from P8-05 on: `Visual:     cargo xtask visual → <last line>`.

### 0.2 The stop report

As Phase 7.2's §0.2, at `docs/Findings/phase-8-stop-<letter>.md`, adding `cargo xtask grove`, `zoom`, `links`, and from P8-05 on `cargo xtask visual`, from P8-08 on `cargo xtask inverse`. Then the run plan's §5 lines.

### 0.3 Snags

As Phase 7.2's §0.3.

---

## 1. Scope Fence

### In scope

- **The reader tree** (`A11yTree`) from the CPU cut, exact, in `joinn-visual` (§2.2).
- **Roles** grow from four to six (§2.3).
- **Witnesses**: owners, reader, intents, for three subjects, written once under `corpus/phase8/visual/` (§2.4–§2.6).
- **`check_visual`**: a body or universe against its witnesses (§2.7).
- **The inverse contract** for every row kind (V3, §2.8), including text: point ↔ character index.
- **Keyboard**: focus, Tab order, typing, Enter, expand and collapse by zoom (§2.9).
- **The AccessKit bridge** in the shell (§2.10).
- **Parity**: the test host and the desktop session produce identical witnesses (V10).
- **Gate 8**, findings `docs/Findings/phase-8-visual-truth.md`, the freeze.

### Out of scope

| Not now | Why it is tempting |
|---|---|
| Golden pixels as witnesses, perceptual tolerance | Dropped, as the roadmap pre-agreed (§2.1) |
| A fifth gate check | Visual truth is checked by `check_visual`, beside the gate. The gate keeps four checks (rule 49, R3) |
| Changing what Phase 3's descriptions hold | They are compared against, not changed |
| Mobile, web, touch intents | Phase 11 |
| Editing anything by keyboard beyond filling in-ports | The creator, Phase 9 |
| Speech output of JoInn's own | The platform's screen reader speaks; JoInn supplies the tree |
| Any grammar change; changing any existing corpus file | No new grammar. New files only in `corpus/phase8/visual/` |

---

## 2. Decisions

All **DECIDED**.

### 2.1 Truth and style

| Witness | Truth (hashed, checked) | Style (free) |
|---|---|---|
| **Owners** | per view, the sorted set of owner addresses with at least one non-edge pixel | pixel positions, colours, stroke shapes |
| **Reader** | per view, the tree's shape, order, roles, values, actions, `expanded` | names and labels |
| **Intents** | per environment, the ordered intent set | key bindings |
| **Golden pixels** | — (not a witness) | everything |

**Why owners and not the pick map.** A label's strokes belong to their cell, so a relabel moves pixels without changing who owns them. The owner set is what *means*; the map is where.

### 2.2 The reader tree

`a11y(&scene, &camera) → A11yTree`, in `joinn-visual`, built **from the CPU cut** (rule 72 extended: three cuts, one truth). A node is in the tree when its owner can own a pixel in the cut, or it is an ancestor of one.

| Thing | Node |
|---|---|
| universe, open galaxy, open system | `group`, `expanded yes`, children in layout order |
| lens node (folded system or galaxy) | `group`, `expanded no`, no children |
| body below Summary | `group`, `expanded no`, no children |
| body at Summary or Full | `group`, `expanded yes`, its cells as children. A link is never a body's child: it sits after the bodies of the lowest open level that contains all its members |
| cell | `cell`; children its surface ports in position order |
| in-port on the surface | `input`, `value <printed>` or `value -`, `actions focus set` |
| out-port on the surface | `output`, `value <printed>` or `value -` |
| refused cell | `refusal` |
| link (Phase 7.3) | `relation`; unordered: `members <n>`; ordered: `order 1 of n` … on each member reference |

Children are ordered as the layout orders them: galaxies, systems, slots, then cells in layout order, then ports by position. Links come after the bodies at their level, in link-id order.

**Text form.** One node per line, two spaces of indent per depth:

```
<role> <address> [value <v>] [actions <a,b>] [expanded yes|no] [members <n>] [order <i> of <n>]
```

Addresses print as Phase 7.2's owners: in a single-body scene `sum@2`, in a universe `<alias>.sum@2` (`calc.sum@2`, `b0000.sum@2`), plus `system g0s00`, `galaxy g0`, `link e0`. The **truth face** is exactly this text; the **label face** is the same lines with ` label "<label>"` appended and is never hashed or witnessed.

### 2.3 Roles

`joinn_host::Role` grows from four to six: `Cell`, `Input`, `Output`, `Refusal` (unchanged, same strings) plus **`Group`** and **`Relation`**. Phase 3 descriptions never produce the new two, and their hashes do not move (a test asserts `calculator.desc` still hashes to its golden). Rule 65 still holds: this is the accessibility role, not a cell's role.

### 2.4 Witness views and states

| Subject | States | Views |
|---|---|---|
| `corpus/phase2/calculator.body` | `fresh`; `summed` (after `a = "2"`, `b = "3"`, sum 5); `refused` (after `a = "two"`) | the four Phase 6 standard viewports (fit); and at 1920 × 1080, focus the body's centre, pin `(960, 540)`, at level −4 (`dot`), −1 (`glyph`), 1 (`summary`), 3 (`full`); the standard viewports are named as Phase 6 names them |
| `corpus/phase7/calculator.contact` | the same three | the same eight |
| `corpus/phase5/universe.universe` | fresh | its frame at 1920 × 1080; level −4 and level 3 at the centre of `calc` |

**Predicted bands** for a 40 × 24 body (size 40) at the four zooms: level −4 `s = 5/2` → **dot**; level −1 `s = 20` → **glyph**; level 1 `s = 80` → **summary**; level 3 `s = 320` → **full**. None sits in a fade window (`10s` vs `11T`: 25 < 44; 200 vs 44 and < 352; 800 vs 352 and < 2640; 3200 ≥ 2640).

### 2.5 Witness files

Under `corpus/phase8/visual/`, three per subject (`calculator_body`, `calculator_contact`, `universe`):

| File | Holds |
|---|---|
| `<subject>.owners.txt` | for each state then view, a header line `view <state> <view>` and one owner per line, sorted |
| `<subject>.reader.txt` | for each state then view, `view <state> <view>` and the tree's truth face |
| `<subject>.intents.txt` | for each environment (`test-host`, `desktop`), `environment <name>` and one intent address per line, in Tab order |

Bytes with `\n` line ends, UTF-8, no trailing spaces. `cargo xtask visual print <subject> --out <path>` writes a file **only when it does not exist** and only under `corpus/phase8/visual/`; an existing file is refused (`visual: <path> exists; acceptance is a new witness. A witness is written once.`). Each file is added to `corpus/hashes.txt` under a dated comment with the tag `joinn.visual.v1`, and to `corpus verify`. **`corpus verify` then prints 53** (44 + 9). This is the run's only corpus addition before Phase 9, and T1 ignores it.

Witness text files are artifacts of the existing **transcript** kind for the mutation catalogue (`DropLine`, `SwapLines`), so no new grammar and no new mutation (rule 52 does not apply).

### 2.6 Intents and Tab order

The intent set is Phase 3's (rule 28: one address per in-port of ∂(body)); for a universe, the union over bodies in layout order. **Tab order is the intent set's order.** A witness lists it per environment; V10 requires the two lists equal.

### 2.7 `check_visual`

`check_visual(subject, witnesses) → Verdict<()>` in `joinn-visual`: derive all three witnesses for the subject and compare with the files. A difference refuses, naming the file, the view and the first differing line, and what acceptance looks like (rule 35):

```
visual: calculator_contact.owners.txt view summed full: missing sum@2; acceptance is every witnessed owner
```

It lives beside the gate, not in it (rule 49). `cargo xtask visual` runs it over the three subjects, and over each subject's **neutral edit** (a label change) and a **style plant** (every style colour rotated by one), both of which must pass, then prints its last line.

### 2.8 The inverse contract (V3)

Every row kind in the tables declares its inverse; `cargo xtask inverse` checks each, over the calculator (both forms), `universe.universe` and the grove at Phase 7.2's §2.12 views:

| Row kind | Inverse | Checked |
|---|---|---|
| Body, cell, port, frame, lens node (shapes) | CPU inside test | already: two pickers agree |
| Stroke (text) | `index_at(point) → character index` and `caret(index) → point`, exact | round trip for every index of every drawn string |
| Chart | rebase back is identity | every chart row: `rebase(A→B→A)` = identity |
| Segment (link) | CPU inside test | already: Phase 7.3 picking |
| **Every row** | its owner has a node in the reader tree | every row, every view in which it can own a pixel |

A row whose owner has no reader node is refused: `inverse: <row kind> row <n> owns pixels for <owner>, which has no reader node; acceptance is a face for every owner`. **Shown then reverted** in P8-08: a stroke row whose owner is removed from the tree.

### 2.9 Keyboard

Focus is a session value: an address in the intent set, or a group node, or none.

| Key | Does |
|---|---|
| Tab / Shift+Tab | next / previous intent in Tab order (wraps) |
| typing | fills the focused input's box (Phase 6 typing) |
| Enter on an input with text | sends it, as Phase 6 |
| **Enter on an empty input** | sends nothing; prints `enter: empty box at <address>; acceptance is a value` |
| Escape | clears the box |
| Enter on a folded node or collapsed body | frames it (zoom in = expand) |
| Backspace with an empty box | frames the parent (zoom out = collapse) |
| `F`, `+`, `-`, arrows | as Phase 7.2 |

**Focus follows into view:** when focus moves to an address not visible, the camera frames that body (Phase 7.2's `frame`); otherwise the camera does not move. A **focus ring** (style 11) is drawn around the focused port; it is owned by that port, so owners do not change.

### 2.10 The AccessKit bridge

`joinn-shell-desktop` maps the `A11yTree` to AccessKit nodes after any tick that changed the tree, through `accesskit_winit`. Roles map: group → `Group`, cell → `GenericContainer`, input → `TextInput`, output → `Label` with value, refusal → `Alert`, relation → `List` (ordered) or `Group`. Labels go to the node name; values to the value. An AccessKit action (`Focus`, `SetValue`, `Default`) becomes the same intent the keyboard would send. Versions: the newest `accesskit_winit` that supports the workspace's `winit`; if none does, it is a snag: the tree, the witnesses and the keyboard still ship, and the bridge goes to the findings. `cargo xtask layers` gains `accesskit in joinn-shell-desktop` and refuses it anywhere else.

### 2.11 Parity (V10)

The headless test host (Phase 3) and the desktop session (no window) each produce the three witnesses for every subject and state. They must be byte-identical. For the full-band view of a single body, the reader tree's cells and ports must also agree with Phase 3's `Description` roles and values (two derivations, one truth).

### 2.12 Invariants

| # | Invariant | Commit |
|---|---|---|
| **V153** | Three cuts, one truth: picture, pick and reader tree come from one cut | P8-03 |
| **V154** | Meaning is witnessed, style is not: a relabel or restyle passes `check_visual`; losing an owner, a value, a role or an intent is refused | P8-06 |
| **V155** | A witness is written once; a different derivation later is a refusal, never a rewrite | P8-05 |
| **V156** | Every intent is reachable by keyboard; hover carries nothing (V9) | P8-09 |
| **V157** | Every row has an inverse and a face (V3) | P8-08 |
| **V158** | Two environments, one meaning: the test host and the desktop produce identical witnesses (V10) | P8-07 |
| **V159** | Zooming in is expanding a group; zooming out is collapsing it | P8-02 |

---

## 3. What runs

| Command | Prints | Exit 1 when |
|---|---|---|
| `cargo xtask reader <subject> [--state S] [--view V] [--labels]` | the tree's truth face (or label face) | refusal |
| `cargo xtask visual` | per subject: `visual <subject>: <n> views, owners ok, reader ok, intents ok; neutral ok; style ok`; last line `visual: 3 subjects witnessed; meaning held under relabel and restyle` | any refusal |
| `cargo xtask visual print <subject> --out <path>` | writes once (§2.5) | the file exists, or the path is elsewhere |
| `cargo xtask inverse` | per row kind `inverse <kind>: <n> rows, inverse ok, face ok`; last line `inverse: every row has an inverse and a face` | a row without one |
| `cargo xtask keys` | the keyboard script on both calculators (§4, P8-09), one line per key; last line `keys: calculator by keyboard: 2 + 3 = 5; every intent reached` | otherwise |
| CI | new step `visual` after `zoom` on both jobs: `cargo xtask visual`, `inverse`, `keys` | the step fails |

---

## 4. The Commits

### Chunk A: the reader tree

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P8-01** | **Plan, rules, docs** | Commit this plan; `AGENTS.md` per Appendix A; Appendix B | `git show --stat HEAD` lists this plan, `AGENTS.md`, backlog, `decisions.md`, roadmap |
| **P8-02** | **Roles and the tree** | §2.2, §2.3; V159 | Tests: `calculator.desc` golden unchanged; the grove at level −4 has 128 `group … expanded no` system nodes and no body nodes; at level −3 every system is `expanded yes`; framing a folded node (the Enter rule) turns its node `expanded yes` |
| **P8-03** | **Three cuts, one truth** | the tree from the cut; V153 | Test over every Phase 7.2 §2.12 view: the set of tree addresses equals the CPU cut's owners plus their ancestors. `cargo xtask reader calculator_contact --state summed --view full` printed whole (paste it) |
| **P8-04** | **Labels** | the label face from the regulatory region | Test: renaming `sum` changes the label face and not the truth face (bytes compared) |
| — | **Stop A** | `phase-8-stop-a.md`, push, ledger, continue | — |

### Chunk B: witnesses and the inverse contract

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P8-05** | **Witness files** | §2.4–§2.6; `visual print`; the nine files; hashes; V155 | The nine files committed; `corpus verify` prints `53 hash(es) match` (paste it). Test: `visual print` on an existing file is refused with §2.5's words. `git diff --stat <run start>..HEAD -- joinn/corpus` lists only the nine new files and `hashes.txt` |
| **P8-06** | **`check_visual`** | §2.7; `cargo xtask visual`; V154 | `cargo xtask visual` printed whole (paste it). Tests: neutral and style plant pass; a plant that stops drawing `sum@2`'s port is refused naming `sum@2`; a plant where `b = "4"` (value 6) is refused naming the output's line; dropping the `cli_b@0` intent is refused |
| **P8-07** | **Parity** | §2.11; V158 | Test: test host and desktop session witnesses byte-equal for every subject and state; the full-band reader nodes agree with `Description` roles and values |
| **P8-08** | **The inverse contract** | §2.8; text `index_at` / `caret`; `cargo xtask inverse`; V157 | `cargo xtask inverse` printed whole (paste it). **Shown then reverted:** the faceless stroke row; paste the refusal |
| — | **Stop B** | `phase-8-stop-b.md`, push, ledger, continue | — |

### Chunk C: keyboard, the bridge, gate 8, freeze

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P8-09** | **The keyboard** | §2.9; focus ring; `cargo xtask keys`; V156 | `cargo xtask keys` printed whole: on both calculators, Tab, `2`, Enter, Tab, `3`, Enter prints the transcript line `2 + 3 = 5`; Enter on an empty box prints §2.9's line and sends nothing; every intent reached by Tab. A session test (no window) on the grove at level −4: Tab from nothing focuses `b0000.cli_a@0` and frames `b0000` (the tick line is pasted) |
| **P8-10** | **The AccessKit bridge** | §2.10 | `cargo xtask layers` passes with the accesskit line (paste it). A test maps the calculator's tree to AccessKit nodes and back to the same truth face. Cursor runs the window once and pastes the first tick line, or writes `no display`. If no compatible version exists: snag, and this commit delivers the `layers` refusal test only |
| **P8-11** | **Findings** | `docs/Findings/phase-8-visual-truth.md`: every command's output from P8-02 on; *Predictions* (§2.4's bands, `corpus verify 53`) marked; **The adversary** (§7); **Golden pixels**: why they are not witnesses, in the roadmap's words | Every prediction marked |
| **P8-12** | **Gate 8** | §6 items; `phase 8` after `phase 7.3`; lock row | `gate all` exits 0 and prints `phase 8: 3/3`. **Shown then reverted**, pasting each failure: (a) item 2 given item 3's `opposes`; (b) the reader tree built from all bodies instead of the cut (item 2 must fail at the level −4 view); (c) Tab skipping `cli_b@0` (item 3 must fail) |
| **P8-13** | **Docs and freeze** | README, `03-where-we-are.md` (meaning is witnessed; the calculator by keyboard); glossary: *witness, owners witness, reader tree, truth face, label face, intent set, Tab order, focus, inverse contract, three cuts one truth*; `decisions.md` V153–V159; roadmap Phase 8 note | `gate all` from a fresh clone prints phases 0 … 8, exits 0; `corpus verify` 53; `git diff --stat <P8-01>..HEAD -- joinn/corpus` lists only the nine witnesses and `hashes.txt` |
| — | **Stop C** | `phase-8-stop-c.md` with CI, push, ledger, tripwires, continue | — |

Dependencies: in order. P8-06 depends on P8-05; P8-07 on P8-06; P8-12 on P8-06, P8-08, P8-09.

**If something has to be cut for time:** the AccessKit bridge first (the tree and keyboard stay), then `--labels`. Never cut V153, V154, V155, V158.

---

## 5. Test Strategy

As Phase 7.2's §5. Witness tests compare bytes. No test asserts only that a tree "exists"; each asserts lines.

---

## 6. Exit Gate 8

Three items in `xtask/src/fns/gate_eight_items.rs`. The witness files are the controls: a control parses its transcript-kind artifact and answers whether the **subject's derivation differs from it**.

| # | Item | `control_artifact` | `opposes` | Control answers `true` when |
|---|---|---|---|---|
| 1 | **Who owns the screen is witnessed** | `corpus/phase8/visual/calculator_contact.owners.txt` | `DropLine(n)`, `n` = the line of `sum@2` in view `summed full` (Cursor computes it and prints it) | the derived owners differ from the artifact |
| 2 | **What the reader hears is witnessed** | `corpus/phase8/visual/calculator_contact.reader.txt` | `SwapLines(a, b)`, the `cli_a@0` and `cli_b@0` input lines in view `fresh full` (printed) | the derived truth face differs from the artifact |
| 3 | **Every intent, by keyboard, everywhere** | `corpus/phase8/visual/calculator_contact.intents.txt` | `DropLine(m)`, the first `cli_b@0` line (printed) | the derived intent set, in either environment, differs from the artifact |

The transcript kind has no neutral edit (as for `corpus/transcripts/`); the control must answer false on the subject and true on the mutant.

**Checks:**

1. On every adapter, every subject, state and view: GPU non-edge owners = CPU cut owners = the witness. Relabel and restyle pass; the `sum@2` plant is refused.
2. Every subject's reader truth face equals its witness in both environments; the `b = "4"` plant is refused naming the line; the tree's addresses equal the cut's owners plus ancestors at every Phase 7.2 view of the grove.
3. `keys` on both calculators prints `2 + 3 = 5` with no pointer event; every intent is reached by Tab; the `cli_b@0` drop is refused.

### 6.1 Conditions for opening Phase 9

The run plan's tripwires.

### 6.2 AJ's check (end of run, about four minutes)

```
cd D:\JoInn\joinn
cargo run --release -p joinn-shell-desktop -- corpus/phase7/calculator.contact
```

1. Don't touch the mouse. Press **Tab**. A ring appears on the first input.
2. Type `2`, press **Enter**. Press **Tab**, type `3`, **Enter**. The sum shows **5**.
3. Press **Tab** until the ring is on an empty box and press **Enter**. Nothing happens; the terminal says the box is empty.
4. *(Optional)* Turn on Windows Narrator (Ctrl+Win+Enter), Tab through, and listen: it should read the inputs and the sum's value. Turn it off the same way.
5. Close the window. Tell Claude what worked.

---

## 7. Risks

| Risk | What to do |
|---|---|
| **The adversary: witnesses too tight or too loose.** The roadmap's fear was pixels; here it is owners. If a harmless restyle changes the owner set (a label's strokes vanish at some band), owners are too tight; if dropping a port still passes, too loose | Both plants test it. A plant that passes or fails the wrong way is a snag with its line; never loosen `check_visual` |
| AccessKit and winit versions don't meet | Snag; bridge to findings (§2.10) |
| Phase 3's description hash moves when `Role` grows | T1 would fire. The new variants must not touch `print_description` for existing roles |
| A witness differs between Windows and Linux | A truth problem (line endings, sort order). Rule 48. Never write two witnesses |

---

## 8. Open Items

| ID | Topic | Question |
|---|---|---|
| **R108** | Three cuts, one truth | R101 asked if "two cuts, one truth" is the rule for every derived view. Phase 8 adds a third. Is every future view (print, PDF, a calc package) a cut too? |
| **R109** | What a reader says for a force | A force owns no pixel (rule 64), so the reader never names `combine`. Should a person be able to ask *why* the sum is 5? |
| **R110** | Witnesses in DNA | Visual witnesses live in `corpus/`, beside the body. Should a body's founding visual witnesses be part of its coding region, like founding witnesses? |

---

## Appendix A · `AGENTS.md` changes

Replace the header paragraph with:

```markdown
# JoInn — standing rules

This repo is JoInn. Phase 8 is visual truth: what a body means on screen —
who owns pixels, what a screen reader hears, what can be done — is witnessed
once and checked forever, and the calculator works by keyboard alone. Its one
idea is MEANING IS WITNESSED, STYLE IS NOT. The build plan is
docs/Plans/JoInn Phase 8 Implementation Plan.md, run inside
docs/Plans/JoInn Run 7.2B-9.md. Work one CHUNK at a time, one git commit per
numbered step, write each stop report, and continue unless a tripwire fires.
Never stop to ask; follow the plan's Snags section instead.

No new grammar in this phase. The only new dependency is AccessKit, in the shell.
```

Keep every rule. Append:

```markdown
77. MEANING IS WITNESSED, STYLE IS NOT. Owners, the reader tree's truth face and
    the intent set are witnesses. Names, labels, colours and pixel positions are
    style. No witness is ever widened to make a check pass.
78. THREE CUTS, ONE TRUTH. The picture, the pick and the reader tree come from
    one cut. A node in the tree that the cut does not allow, or an owner the tree
    does not hold, is a truth violation.
79. A WITNESS IS WRITTEN ONCE. `visual print` refuses an existing file. A later
    derivation that differs is a refusal, never a rewrite.
80. EVERY INTENT HAS A KEY. Everything a pointer can do to a port, a key can do.
    Hover carries nothing.
81. EVERY ROW HAS AN INVERSE AND A FACE. A row kind declares its inverse; every
    row's owner has a reader node.
```

## Appendix B · Documents

- **Backlog**: after R107, `> **3 Oct 2026 · Phase 8.** R108–R110 come from the visual-truth plan.` then R108–R110 (`open`) with §8's questions.
- **`decisions.md`**: R108–R110 (phase `8`, `open`); V153–V159 (`open`; P8-13 sets them). Also the line: `Golden pixels: not a witness (Phase 8 plan §2.1), the roadmap's pre-agreed fallback.`
- **Roadmap**, under Phase 8: `> **3 Oct 2026.** Planned in Plans/JoInn Phase 8 Implementation Plan.md: owners, reader and intents witnessed; golden pixels dropped as witnesses (the pre-agreed fallback); the calculator by keyboard.`

---

*JoInn Phase 8 Implementation Plan, Draft 0.1 (3 Oct 2026). Written with the run plan while AJ was away. Claude decided, open to AJ's veto at the run review: owners instead of the pick map; names not witnessed; golden pixels dropped; JoInn's own reader tree with AccessKit as the bridge; Enter on an empty box does nothing and says so.*
