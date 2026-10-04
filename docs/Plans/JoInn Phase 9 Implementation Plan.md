# JoInn Phase 9 Implementation Plan

**The creator: build bodies and universes by hand, and get the same bytes a person would type**

Author: AJ, with Claude · Draft 0.1 · October 3, 2026

> **Every decision in this plan is final.** Nothing here waits on AJ. Cursor never stops to ask AJ anything, and no file in this phase is typed by AJ. If a step can't be done the way the plan says, Cursor follows §0.3 (Snags) and keeps going. This phase runs inside `docs/Plans/JoInn Run 7.2B-9.md`; its tripwires apply.

---

## For AJ: this plan in plain English

**What Phase 9 is.** The first time JoInn is built *in* JoInn. You open the creator, drag cells from drawers onto a bench, apply a force, link bodies into systems, and save. What it saves is exactly the text file a person would have typed, byte for byte, with the same hash. Two editors, one truth.

**What you can build in it.** Contact bodies (cells in contact, with forces, like the calculator) and universes (bodies in systems and galaxies, joined by hyperedges). Not new cells with new laws yet: writing laws is its own hard problem (R23) and gets its own phase once you've used this.

**How it feels.**

- **Drawers** down the left side: *Cells*, *Forces*, *Grants*, *Words* for a body; *Bodies*, *Links*, *Lenses*, *Grants*, *Words* for a universe.
- **The bench** is the thing you're building, drawn by the same renderer as everything else, so it zooms.
- **Snapping checks as you go.** Drag a number's port onto a text port and it doesn't snap: the refusal is drawn right there, saying what would have worked.
- **Undo is a true opposite.** Every gesture knows its exact inverse; undo applies it. Undo everything and the bench is empty; redo everything and the bytes come back identical.
- **A session is a script.** Every gesture prints one line in the terminal. Replaying those lines rebuilds the same file. So your session is also a test.
- **Keyboard too.** Everything the mouse does has a key path (Phase 8's rule).
- **Run it.** Press R to try the body you just built: type 2 and 3, see 5.
- **Unfold.** Press U on a sealed cell to see the body it was folded from; Escape returns exactly where you were.

**What I decided (say if you disagree):** bodies and universes only, no cell authoring; tails and heads come from port direction (you never choose them); the creator saves only outside `corpus/` and `docs/`; unfold is a separate view, not deeper zoom.

**What you do at the end** (about five minutes, §6.2): build the calculator from the drawers and compare the hash. **Then the first-grader test** (§6.3), another day, with a real person who can't program.

---

## 0. How Cursor Works This Plan

| | |
|---|---|
| **Plan file** | `D:\JoInn\docs\Plans\JoInn Phase 9 Implementation Plan.md` |
| **Code** | `D:\JoInn\joinn\`. **One new crate: `joinn-creator`** (exact, no float, no IO). No new external dependency |
| **Standing rules** | `AGENTS.md` as updated by P9-01 (Appendix A) |
| **Unit of work** | a **chunk** (A, B or C); commit ids `P9-NN` |
| **End of a chunk** | stop report, commit, push, ledger, tripwires; at Stop C the run ends |
| **Who decides** | every decision is in §2. Cursor decides module layout, function bodies, private representations, drawer and tile drawing, and error wording where §2 gives none |
| **Who checks** | Claude after the run; CI; AJ at the end, then the first-grader test |
| **Input** | rule 67 |

### 0.1 The commit report

As Phase 7.2's §0.1, plus from P9-04 on: `Create:     cargo xtask create → <last line>`.

### 0.2 The stop report

As Phase 7.2's §0.2, at `docs/Findings/phase-9-stop-<letter>.md`, adding `grove`, `zoom`, `links`, `visual`, `inverse`, `keys`, and from P9-04 on `cargo xtask create`. Then the run plan's §5 lines; Stop C's `Next:` is `run finished`.

### 0.3 Snags

As Phase 7.2's §0.3.

---

## 1. Scope Fence

### In scope

- **`joinn-creator`**: the draft, edits with exact inverses, undo and redo, the gesture script, replay and its inverse `script_of` (§2.2–§2.5).
- **The bench and drawers** drawn through `joinn-visual`, with a screen chart for the drawers (§2.6).
- **Gestures**: pointer and keyboard to edits; frame-match at snap; refusals drawn where they happen (§2.7, §2.8).
- **Text fields** for names and words, with the Phase 8 caret (§2.9).
- **Run mode** and **unfold view** (§2.10, §2.11).
- **The shell's creator mode**: `--create` and `--edit`, save (§2.12).
- **Gate 9**, findings `docs/Findings/phase-9-creator.md`, the freeze, and the first-grader protocol for AJ.

### Out of scope

| Not now | Why it is tempting |
|---|---|
| Authoring a `.cell`: laws, witnesses, alleles, frames | Law authoring is R23 and its own phase |
| Choosing a link member's mark by hand | Marks follow direction: out is tail, in is head |
| `.body` (wired, legacy) authoring | Wires belong to systems (rule 61); the legacy form is read-only here |
| Editing the regulatory `present` template beyond typing its text | Presentation is the host's (R76) |
| A plugin, macro or scripting system for gestures | The gesture script is a transcript, not a language |
| Collaborative or multi-window editing | Later; R12 first |
| Writing under `corpus/` or `docs/` from the creator | Refused (rule 86) |
| Any change to existing grammar, corpus files or hashes | None. The creator emits through the existing canonical writers |

---

## 2. Decisions

All **DECIDED**.

### 2.1 Where things live

| Crate | Gains |
|---|---|
| **`joinn-creator`** (new; depends on `joinn-frame`, `joinn-dna`, `joinn-link`, `joinn-prim`, `joinn-visual`) | `Draft`, `Edit`, `Session`, undo/redo, the gesture script, `replay`, `script_of`, gesture → edit, the bench and drawer scene |
| `joinn-visual` | chart kind 4 **screen** (§2.6), `layout_draft` (§2.6), refusal text placement |
| `joinn-shell-desktop` | creator mode, run mode, unfold view, save |
| `xtask` | `create`, gate 9; `layers` gains the crate's row |

`joinn-creator` holds no `std::io`, `std::fs`, float or wgpu (rules 15, 54, 55). `cargo xtask layers` table: `joinn-creator` may depend on the five crates above and nothing else; `joinn-shell-desktop` may depend on it.

### 2.2 The draft and its edits

A **draft** is the DNA value being built: a `Contact` or a `Universe`, exactly the types the parsers produce, with `codex 1`, `budget { steps 100000 }` and `lineage none` by default.

| Edit (contact) | Edit (universe) |
|---|---|
| `AddCell { hash, instance }` / `RemoveCell { instance }` | `AddBody { hash, alias }` / `RemoveBody { alias }` |
| `Rename { from, to }` | `RenameAlias { from, to }` |
| `AddForce { word, frame, response, name }` / `RemoveForce { name }` | `AddLink { id, order }` / `RemoveLink { id }` |
| `AddMember { force, member }` / `RemoveMember { force, member }` | `AddLinkMember { link, member }` / `RemoveLinkMember { link, member }` (mark from direction) |
| `Grant { capability, instance }` / `Ungrant { … }` | `GrantLink { link, alias }` / `UngrantLink { … }` |
| `SetWord { section, key, text: Option<String> }` | `AddLens`, `AddGalaxy`, `AddSystem`, `Place { lens, system, alias }`, their removals; `SetWord` |
| `SetBudget { steps }` | — |

- **Every edit has an exact inverse**, computed from the edit and the draft before it: `inverse(e, d)` such that `apply(inverse(e, d), apply(e, d)) = d`, compared by canonical text. A removal's inverse carries everything it removed, including its position where canonical order depends on it (V161).
- **Undo applies the inverse; redo re-applies the edit.** No snapshot is stored or restored (rule 83).
- A force's `response` is the registered response for `(word, frame)` in `joinn_prim::forces`; the creator offers only registered pairs. A `separate` tile appears wherever its `combine` does (rule 63).
- `AddCell` with a hash already in the genome adds the instance to that entry; canonical text decides the bytes.

### 2.3 Admission while editing

After every edit the session runs the existing admission: `check_contact` for a contact; `bind`, `check_link_types`, `check_law4`, `check_lenses` for a universe. The draft is **admitted** or **refused with a reason**. A refused draft is kept (it is usually only unfinished), drawn with the refusal (§2.8), and cannot be saved.

**Snap-time refusals are different:** a member whose frame differs from the force's or link's, an in-port offered as a force member, or an out-port offered as a link head, is **not applied**. The draft does not change; the refusal is drawn at the target until the next gesture. This is frame-match at snap: the cheapest opposition.

### 2.4 The gesture script

One gesture per line, UTF-8, `\n`. The vocabulary (contact, then universe):

```
new contact | new universe
cell <name|cell:hash> as <instance>          remove cell <instance>
rename <from> <to>
force <combine|separate> <frame> as <name>    remove force <name>
member <force> <instance@port>                remove member <force> <instance@port>
grant <capability> <instance>                 remove grant <capability> <instance>
word <prompts|present|names|labels> <key> "<text>"     remove word <section> <key>
budget <n>
body <name|body:hash> as <alias>              remove body <alias>
link <id> <none|ordered>                      remove link <id>
member <link> <alias.instance@port>           remove member <link> <alias.instance@port>
grant <link> <alias>
lens <name> · galaxy <lens> <name> · system <lens> <galaxy> <name> · place <lens> <system> <alias>
undo · redo · save <path>
```

Names resolve through the corpus catalogue (`corpus/hashes.txt` names to hashes; `cell:`/`body:` hashes are always accepted). A line that does not parse is refused naming the line and what acceptance is.

- **`replay(script) → Session`** applies each line as a gesture.
- **`script_of(draft) → script`** is replay's inverse: a canonical script (sections in canonical order, no undo) such that `replay(script_of(d))` has canonical text equal to `d`'s, for every corpus `.contact` and `.universe` (V163).
- **A session is a script:** the shell prints each gesture's line as it happens (`gesture: member sum cli_a@1`). Copying those lines into a file and replaying them yields the same bytes (rule 85).

The three gate scripts live in `xtask/gate_fixtures/creator/`: `calculator_contact.gestures`, `universe.gestures`, `ordered.gestures`, each written by Cursor by hand as a person would gesture (not by `script_of`), in an order different from canonical order. **Phase 9 adds no corpus file.**

### 2.5 Predicted bytes

| Script | Must equal | Coding hash |
|---|---|---|
| `calculator_contact.gestures` | `corpus/phase7/calculator.contact`, byte for byte | **`868e79b293c8d35625f514274eb0bf04d228898ed994a795b23b942a530b6209`** |
| `universe.gestures` | `corpus/phase5/universe.universe` | **`0b784f4c5219fd0bc77076dd6562e53ae5562884637c68b81486458f585bb828`** |
| `ordered.gestures` | `corpus/phase5/ordered.universe` | **`ded37f4f0d3fba33085fb0cb6037083b507bba138fa05acc69d52ca66e0a667e`** |

### 2.6 The bench and the drawers

- **Chart kind 4, screen.** A chart pinned to the viewport: origin at pixel `(0, 0)`, one layout unit = one pixel, unaffected by zoom and pan. Its rows are written only on resize and on drawer changes. Phase 7.2's rules hold: exact, integer.
- **Drawers**: a column 240 px wide at the left of the screen chart; one tile per item, 32 px tall, the name in the stroke font (Phase 7.2 §2.8, Full-band size at k = 1 scaled to 12 px cap). Contact mode: *Cells* (every corpus cell, by catalogue name), *Forces* (every registered `(word, frame)`), *Grants* (`stdin`), *Words*. Universe mode: *Bodies* (every corpus `.body` and `.contact`), *Links* (`none`, `ordered`), *Lenses*, *Grants*, *Words*. One drawer open at a time; its tiles list in catalogue order.
- **The bench** is the draft, drawn in world charts as any body or universe. `layout_draft` lays out a draft whether admitted or not by the same rules as `layout_contact` and `layout_universe`; **for an admitted draft it equals them exactly** (test).
- Tiles, fields and refusal text own pixels: a tile is `[0, 0, DRAWER_TAG | item, generation]` with `DRAWER_TAG = 0x6000_0000`; owners print `drawer cells cli_input`. The reader tree (Phase 8) gains the drawers as a `group` with one `cell` node per tile, so the creator itself is witnessed by Phase 8's rules.

### 2.7 Gestures

| Pointer | Key path | Edit |
|---|---|---|
| Drag a tile onto the bench, release | `D` cycles drawers, arrows select a tile, `Enter` places it | `AddCell` / `AddBody` / `AddForce` / `AddLink` …, then a **name field** opens (§2.9) |
| Drag from a port onto a force's response cell | Tab to the port, `M`, then Tab to the force, `Enter` | `AddMember` |
| Drag from a port to another body's port (universe) | Tab, `L`, Tab, `Enter` | `AddLink` (a field asks its id; `O` toggles ordered before `Enter`) with both members |
| Drag a port onto a selected link | Tab, `M`, Tab to the link, `Enter` | `AddLinkMember` |
| Drag `stdin` onto a cell | `G` on the focused cell | `Grant` |
| Drag a body onto a system tile | Tab, `P`, Tab to the system, `Enter` | `Place` |
| Select, then `Delete` | the same | the matching removal |
| `W` on a selected cell, body or link | the same | opens its word fields |
| `Ctrl+Z` / `Ctrl+Shift+Z` | the same | undo / redo |
| `Ctrl+S` | the same | save |

A drag starting on a tile, a port or a body is a creator drag; a drag starting on empty bench pans (Phase 7.2). A press that moves less than 4 px is a click (select). Every gesture becomes exactly one script line; a refused snap prints `gesture: … refused: <reason>` and changes nothing. `cargo xtask create --keys` replays every gate script through the key paths only and must give the same bytes (V165).

### 2.8 Refusals drawn where they happen

The **target** of a gesture is the address it acted on (the instance placed, the member added, the link joined). A refusal (snap or admission) is drawn at its target: a ring in the refusal style around the target's owner, and the refusal's text in the stroke font beside it (Full band; at lower bands the ring only). The refusal text owns its pixels for the target. Undoing the gesture removes the drawing. While any admission refusal stands, `save` is refused: `save: the draft is refused at <target>: <reason>; acceptance is an admitted draft`.

### 2.9 Text fields

A field is a strip on the bench or in the drawer column, holding a string drawn in the stroke font, with a caret placed by Phase 8's `caret(index)` and moved by `index_at(point)` on click. Typing inserts at the caret; Backspace deletes before it; `Enter` commits (one edit); `Escape` cancels (no edit). A character outside the stroke set is refused at the keystroke with Phase 7.2's wording and not inserted. Instance names and aliases must match the grammar's identifier rule; a name field refuses others with the parser's words.

### 2.10 Run mode

`R` toggles between **build** and **run**. Run mode needs an admitted draft (else `run: the draft is refused at …; acceptance is an admitted draft`). It runs the draft exactly as the shell runs a saved file (Phase 7 for contacts, Phase 5 for universes); inputs are filled as Phase 6 and 8 (pointer or keyboard). Leaving run mode returns to the draft unchanged. Run mode never edits.

### 2.11 Unfold view

`U` on a cell that is the declared cell of a seal (rule 21) opens its **reference body** as a scene of its own; the camera frames it; `Escape` returns to the bench with the camera **exactly** as it was (V166). On any other cell: `unfold: <instance> is not sealed; acceptance is a sealed cell`. The view is read-only.

### 2.12 The shell

```
cargo run -p joinn-shell-desktop -- --create <contact|universe> <out path>
cargo run -p joinn-shell-desktop -- --edit <path> [--out <out path>]
```

`--create` starts an empty draft; `--edit` opens a `.contact` or `.universe` as a draft (`script_of` then replay, so its first lines print as gestures). Save writes canonical text with `\n` to the out path, and the session's gesture lines (undo and redo included) to `<out path>.gestures` beside it. **An out path under `corpus/` or `docs/` is refused** (rule 86): `save: <path> is under corpus/ or docs/; acceptance is a path outside them`. The tick line is unchanged; an idle creator draws nothing (V147).

### 2.13 Invariants

| # | Invariant | Commit |
|---|---|---|
| **V160** | Two editors, one truth: gestures produce the typed file's bytes and hash | P9-05 |
| **V161** | Every edit has an exact inverse; undo applies it and stores no snapshot | P9-02 |
| **V162** | Gesture order is blind: two orders of the same gestures give the same bytes | P9-05 |
| **V163** | `replay(script_of(d)) = d` for every corpus contact and universe | P9-04 |
| **V164** | A refusal is drawn at its target; a refused snap changes nothing | P9-07 |
| **V165** | Every pointer gesture has a key path giving the same edit | P9-08 |
| **V166** | Unfold and return leave the camera and the bytes exactly as they were | P9-11 |

---

## 3. What runs

| Command | Prints | Exit 1 when |
|---|---|---|
| `cargo xtask create [--keys]` | per gate script: `create <script>: <n> gestures, admitted, bytes equal <file>, hash <hex>`; then `create undo: <n> gestures undone to empty, redone to hash <hex>` per script; `create sweep: 1000 sequences, every gesture undone exactly`; `create round trip: <n> corpus drafts, replay(script_of) exact`; last line `create: 3 scripts, two editors, one truth` | any difference |
| `cargo xtask create --replay <script>` | the session's lines and the final canonical text | the script refuses |
| CI | new step `create` after `visual` on both jobs: `cargo xtask create`, `cargo xtask create --keys` | the step fails |

---

## 4. The Commits

### Chunk A: the creator's brain (headless)

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P9-01** | **Plan, rules, docs** | Commit this plan; `AGENTS.md` per Appendix A; Appendix B | `git show --stat HEAD` lists this plan, `AGENTS.md`, backlog, `decisions.md`, roadmap |
| **P9-02** | **The crate, drafts and contact edits** | `joinn-creator`; §2.2 contact edits and inverses; §2.3; V161 | `cargo xtask layers` passes with the crate's row (paste it). Tests: every contact edit kind applied then inverted gives the same canonical text; undo stores no draft (a test asserts the session's history holds edits only); a refused draft is kept and not saved |
| **P9-03** | **Universe edits** | §2.2 universe edits and inverses | Tests as P9-02 for every universe edit kind; `AddLinkMember` marks from direction; a Text port beside a ℤ link is a snap refusal and the draft is unchanged |
| **P9-04** | **The gesture script** | §2.4: parse, print, `replay`, `script_of`; the three fixture scripts; `cargo xtask create --replay`; V163 | Tests: `replay(script_of(d))` exact for every corpus `.contact` and `.universe` (count printed); every malformed line refused with its words |
| **P9-05** | **Two editors, one truth** | `cargo xtask create`; the undo sweep (SplitMix64, seed 7); V160, V162 | `cargo xtask create` printed whole; the three hashes as §2.5 (paste the lines). Test: each script in a second gesture order gives the same bytes |
| — | **Stop A** | `phase-9-stop-a.md`, push, ledger, continue | — |

### Chunk B: the creator's face

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P9-06** | **Screen chart, drawers, bench** | §2.6; `layout_draft` | Tests: `layout_draft` equals `layout_contact` / `layout_universe` on every admitted corpus draft; screen-chart rows are not written by a pan or zoom; tile owners print as §2.6; Phase 6, 7.2, 7.3 row layouts unchanged; `gate 6` … `gate 8` full |
| **P9-07** | **Pointer gestures and refusals** | §2.7 pointer column; §2.8; V164 | A session test (no window) performs `calculator_contact.gestures` by pointer events at computed pixels and saves the same bytes; a drag of `calc.sum@2` onto `bus.listen@0` draws a refusal owner at `bus.listen@0`, changes nothing, and prints the refused gesture line |
| **P9-08** | **Fields and the key paths** | §2.9; §2.7 key column; `create --keys`; V165 | `cargo xtask create --keys` printed whole: same three hashes. Tests: a character outside the stroke set is refused at the keystroke; Escape commits nothing |
| **P9-09** | **Run mode** | §2.10 | Session test: build the calculator by gestures, `R`, fill 2 and 3 by keyboard, the transcript line `2 + 3 = 5` prints; `R` again, the draft's bytes unchanged |
| — | **Stop B** | `phase-9-stop-b.md`, push, ledger, continue | — |

### Chunk C: the shell, unfold, gate 9, freeze

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P9-10** | **The shell's creator mode** | §2.12 | Session test: `--edit corpus/phase7/calculator.contact --out target/c.contact`, save, bytes equal the corpus file; `--create contact corpus/x.contact` refused with §2.12's words. Cursor runs the window once with `--create contact target/try.contact` and pastes the first gesture line, or writes `no display` |
| **P9-11** | **Unfold** | §2.11; V166 | Test: unfold a sealed response, Escape, camera and color/ID bytes identical; a non-sealed cell prints §2.11's refusal |
| **P9-12** | **Findings** | `docs/Findings/phase-9-creator.md`: every command's output from P9-04 on; *Predictions* (§2.5's three hashes) marked; **The adversary** (§7) with the concept count; **Ready for the first grader**: the exact §6.3 protocol | Every prediction marked; the concept count printed |
| **P9-13** | **Gate 9** | §6 items; `phase 9` after `phase 8`; lock row | `gate all` exits 0, prints `phase 9: 3/3`. **Shown then reverted**, pasting each failure: (a) item 2 given item 3's `opposes`; (b) undo implemented as restoring a stored draft (item 2's history test must fail); (c) a snap refusal that still applies the edit (item 3 must fail) |
| **P9-14** | **Docs and freeze** | README, `03-where-we-are.md` (JoInn builds bodies and universes in itself; what isn't built: cell authoring, law help); glossary: *creator, draft, edit, inverse edit, gesture, gesture script, replay, script_of, drawer, bench, snap, frame-match, run mode, unfold view*; `decisions.md` V160–V166; roadmap Phase 9 note | `gate all` from a fresh clone prints phases 0 … 9 and exits 0; `corpus verify` 53; `git diff --stat <P9-01>..HEAD -- joinn/corpus` prints nothing |
| — | **Stop C** | `phase-9-stop-c.md` with CI, push, ledger; `run: finished` | — |

Dependencies: in order. P9-03 on P9-02; P9-05 on P9-04; P9-07 on P9-06; P9-08 on P9-07; P9-13 on P9-05, P9-07, P9-08.

**If something has to be cut for time:** unfold first (R113 carries it), then run mode, then the drawers' *Lenses* tile (systems added by script only, said so). Never cut V160, V161, V163, V164, V165.

---

## 5. Test Strategy

As Phase 7.2's §5. Creator tests compare canonical bytes and hashes, never "an edit happened". Session tests drive pointer and key events into the session with no window and assert the printed gesture lines and the saved bytes.

---

## 6. Exit Gate 9

Three items in `xtask/src/fns/gate_nine_items.rs`.

| # | Item | `control_artifact` | `opposes` | Control answers `true` when |
|---|---|---|---|---|
| 1 | **Two editors, one truth** | `corpus/phase7/calculator.contact` | `DropGenome("cli_b")` | the coding hash of `replay(calculator_contact.gestures)` differs from the subject's |
| 2 | **Every gesture has its inverse** | `corpus/phase5/universe.universe` | `RenameAlias("units", "unit")` | replaying `universe.gestures`, undoing to empty and redoing all gives a coding hash different from the subject's |
| 3 | **A refusal is drawn where it happened** | `corpus/phase5/ordered.universe` | `ShiftPort("path", "units.scale@0", 2)` | `replay(script_of(subject))` draws a refusal owner |

**Checks:**

1. The three scripts give §2.5's bytes and hashes, by pointer and by keys; a second gesture order gives the same; `replay(script_of(d))` exact on every corpus draft.
2. The seeded sweep: every gesture undone exactly; undo-all is the empty draft; redo-all returns the hash; the session holds no stored draft.
3. The mutant's refusal is drawn at `units.scale@2` with its text, on every adapter, owned by that target; the Text-onto-ℤ snap is refused and the draft's bytes are unchanged.

If a pair collides under rule 41, Cursor takes the next unused mutation of the same artifact, prints it, and records a snag.

### 6.1 After Phase 9

The run ends. Claude reviews every stop report of the run. Then AJ's checks.

### 6.2 AJ's creator check (end of run, about five minutes)

```
cd D:\JoInn\joinn
cargo run --release -p joinn-shell-desktop -- --create contact target/mine.contact
```

1. Open the **Cells** drawer. Drag **cli_input** onto the bench; type `cli_a`, Enter. Do it again as `cli_b`.
2. Open **Forces**. Drag **combine ℤ 1** onto the bench; type `sum`, Enter.
3. Drag from `cli_a`'s right port onto `sum`; then from `cli_b`'s right port onto `sum`.
4. Try dragging `cli_a`'s **left** port onto `sum`. It refuses and says why. Nothing changed.
5. Open **Grants**; drag `stdin` onto `cli_a` and onto `cli_b`.
6. Open **Words**; give `cli_a` the prompt `a: `, `cli_b` the prompt `b: `, `sum` the present text `{0} + {1} = {2}`, name and label `Sum`.
7. Press **Ctrl+Z** a few times, then **Ctrl+Shift+Z** the same number. Everything comes back.
8. Press **R**, type 2 and 3. It shows `2 + 3 = 5`. Press **R** again.
9. Press **Ctrl+S**. Close the window. Then:

```
cargo xtask create --replay target/mine.contact.gestures
```

   *(The shell writes the session's gesture lines beside the saved file.)* The last line's hash should be `868e79b2…`, the same as the calculator you typed by hand in Phase 7.

Tell Claude what worked.

### 6.3 The first-grader test (G7, R21)

The roadmap's only gate that can't be automated. Another day, about forty minutes.

- **Who:** someone who can't program. A player you coach is perfect.
- **Setup:** `cargo run --release -p joinn-shell-desktop -- --create contact target/add.contact`, the window full screen.
- **The only instruction, read exactly:** *"Make something that asks for two numbers and shows their sum. When you think it works, press R and try it."*
- **Rules for you:** no hints, no pointing, no rescuing. Write down where they get stuck and for how long.
- **Pass:** in under thirty minutes, unaided, they press R, enter two numbers, and see their sum.
- **After:** tell Claude the time, each place they got stuck, and what they said. Those notes are the specification for the next round (the roadmap expects the first several to fail).

---

## 7. Risks

| Risk | What to do |
|---|---|
| **The adversary: the first-grader test.** Everything aims at those thirty minutes. Its early warning is the number of concepts a first body needs | P9-12 counts them: drawers opened, tile kinds used, fields typed, for the calculator. That number is the budget R21 asks for |
| Canonical bytes differ from the typed file in one place (spacing, section order) | A truth problem in the draft or the writer, never in the corpus. Paste the first differing line |
| `script_of` cannot express a corpus draft | A snag with the draft named; the gesture vocabulary is closed in §2.4 |
| Undo needs a snapshot for some edit | A snag naming the edit; never store a draft |
| Drag versus pan feels wrong | Presentation; AJ judges at the end |

---

## 8. Open Items

| ID | Topic | Question |
|---|---|---|
| **R111** | Law authoring (R23 lands here) | The creator builds bodies from cells that exist. How does a non-expert make a new cell, and how does the creator help them write its laws, suggest them from witnesses, or propose the inverse? |
| **R112** | Incomplete versus wrong | A half-built body is refused the same way as a wrong one. Should the creator draw "not finished yet" differently from "untrue"? |
| **R113** | Unfold as zoom | Unfold is a separate view. Should zooming into a sealed cell simply continue into its reference body's chart? |
| **R114** | The creator built from cells | Part II's self-hosting test: can the creator's own drawers and bench be shown as cells and bodies? |

---

## Appendix A · `AGENTS.md` changes

Replace the header paragraph with:

```markdown
# JoInn — standing rules

This repo is JoInn. Phase 9 is the creator: contact bodies and universes are
built by gestures and saved as the same bytes a person would type. Its one idea
is TWO EDITORS, ONE TRUTH. The build plan is
docs/Plans/JoInn Phase 9 Implementation Plan.md, run inside
docs/Plans/JoInn Run 7.2B-9.md. Work one CHUNK at a time, one git commit per
numbered step, write each stop report, and continue unless a tripwire fires.
Never stop to ask; follow the plan's Snags section instead.

No new grammar in this phase. One new crate, joinn-creator. No cell authoring.
```

Keep every rule. Append:

```markdown
82. THE CREATOR IS A VIEW OVER TEXT (V27). It emits through the existing
    canonical writers and never has a writer of its own. Its draft is the
    parser's own type.
83. EVERY GESTURE IS AN EDIT WITH AN INVERSE. Undo applies the inverse. No
    snapshot of a draft is ever stored to undo or redo.
84. A REFUSAL IS DRAWN WHERE IT HAPPENED. A refused snap changes nothing. A
    refused draft is kept, drawn at its target, and cannot be saved.
85. A SESSION IS A SCRIPT. Every gesture prints its script line; replaying the
    lines gives the same bytes. replay(script_of(d)) = d.
86. THE CREATOR NEVER WRITES UNDER corpus/ OR docs/. Save refuses those paths.
```

## Appendix B · Documents

- **Backlog**: after R110, `> **3 Oct 2026 · Phase 9.** R111–R114 come from the creator plan. R111 carries R23.` then R111–R114 (`open`) with §8's questions.
- **`decisions.md`**: R111–R114 (phase `9`, `open`); V160–V166 (`open`; P9-14 sets them).
- **Roadmap**, under Phase 9: `> **3 Oct 2026.** Planned in Plans/JoInn Phase 9 Implementation Plan.md: the creator builds contact bodies and universes (not cells); undo as inverse; a session is a script. The first-grader test follows AJ's window check.`

---

*JoInn Phase 9 Implementation Plan, Draft 0.1 (3 Oct 2026). Written with the run plan while AJ was away. AJ decided earlier: the creator is Phase 9, after 7.2, 7.3 and 8; bodies like the beam are built in JoInn once the creator exists. Claude decided, open to AJ's veto at the run review: bodies and universes only; marks from direction; unfold as a view; the drawers; refused drafts kept but unsavable; gesture scripts as fixtures, not corpus.*
