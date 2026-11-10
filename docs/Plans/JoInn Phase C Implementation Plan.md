# JoInn Phase C Implementation Plan

**Open the hypernode: the contact calculator drawn as blocks in contact, its surface ports, and its force as a lasso**

Draft 0.1 · 30 Oct 2026 · written from the [Hypernode Refactor Plan](JoInn%20Hypernode%20Refactor%20Plan.md) (Phase C) for AJ's review

> **Status: a proposal for AJ's review. Nothing in this plan is built yet.** Every decision below is tagged **DECIDED** (AJ already decided it in the refactor plan or in earlier theory) or **PROPOSED** (this draft's reading, open to AJ's veto). Phase C opens only after Phase B's stop (Phase B plan §6.1). When AJ has confirmed or changed each PROPOSED item, the plan becomes Draft 1.0 and runs as one unit with one stop; a step that can't be done as written follows §0.3 (Snags).

**Thesis.** The window draws the contact calculator as the form the engine runs: blocks that touch, the ports on its surface, and its force as a lasso pointing at `sum`, with every pixel still named by both pickers.

---

## For AJ: this plan in plain English

**What Phase C is.** Open `calculator.contact` in the window today and you see three boxes touching: `cli_a` above `cli_b`, and `sum` pressed against both. The force that turns two numbers into a sum isn't drawn at all. That was a deliberate decision in Phase 7 (V130: *a force owns no pixel*), made before forces had a look. Phase 7.4 then gave forces their look in systems: a lasso around what the force acts on, with an arrow at the result. Phase C brings that lasso into the calculator itself.

**What you'll see.**

```
  ┌──────────────────────────────────────────────┐   the hypernode's surface
  │ ╭──────────────╮                             │
  │ │ ○[  cli_a  ] │                ┌───────┐    │
  │ │  ──────────  │ ─────────────▶ │  sum  ○    │
  │ │ ○[  cli_b  ] │                └───────┘    │
  │ ╰──────────────╯                             │
  └──────────────────────────────────────────────┘
     ○ surface ports      the loop: combine, holding the two blocks in contact
```

- **Blocks.** `cli_a` and `cli_b` still touch along one edge. Touching *is* how contact is drawn: no line joins them.
- **Ports.** Only the hypernode's surface ports are drawn: the two in-ports on the left (`cli_a@0`, `cli_b@0`) and `sum`'s out-port on the right (`sum@2`). The ports the force uses inside are not drawn, as today.
- **The lasso.** A loop around `cli_a` and `cli_b`, clear of both, with a neck and an arrowhead that touches `sum`'s left side. Click it and the terminal prints `force sum`.
- **`sum` moves.** It no longer touches the two inputs; it sits at the lasso's point, like `count` in the counting system. The picture is wider (40 layout units instead of 32).

**Why this needs a correction on record.** V130 said a force owns no pixel in a contact picture, and gate 7 item 3 checks that picture. Phase C changes it on purpose, so it ships a dated finding, a new V row that supersedes V130, a changed gate 7 item 3 with a demo showing the changed check still refuses something, and its own gate item.

**What I decided (say if you disagree):**

- **`sum` moves away from the inputs** so the lasso can pass between them. The alternative, a lasso that crosses the shared edge, would break the lasso's first rule (it never meets a cell).
- **The lasso is drawn the way systems draw it**: the same strokes, style, ID tag and draw order (under the cells). No shader changes.
- **A click on the lasso prints `force sum`** (a force's name is its response's name, as `force count` is in the counting system).
- **Editing stays as it is**: you type into an in-port and press Enter. No block editor.
- **Gate C has one item.** Its mutant drops one of the force's members, which admission refuses; the lasso's truths are opposed by four standing plants, as gate 7.4's lasso item is.

**What you do at the end** (about three minutes, §6.2): open the calculator, look at the lasso, click it, type `2` and `3`.

---

## 0. How Cursor Works This Plan

| | |
|---|---|
| **Plan file** | `D:\JoInn\docs\Plans\JoInn Phase C Implementation Plan.md` |
| **Parent plan** | `docs/Plans/JoInn Hypernode Refactor Plan.md`, Phase C |
| **Code** | `D:\JoInn\joinn\`. No new crate, no new external dependency, no identifier renamed. `joinn-gpu` is predicted to need nothing (§2.6) |
| **Standing rules** | `AGENTS.md` as updated by PC-01 and PC-07 (Appendix A) |
| **Unit of work** | the phase: every commit of §4 in order, each its own git commit whose message starts with its id (`PC-04: …`). **One stop**, at the end |
| **End of the phase** | `cargo xtask stop-check --fresh`, the stop report (§0.2), commit, push, ledger line, print `phase C: stopped for review`, stop |
| **Who decides** | every decision is in §2. Cursor decides module layout, function bodies, private representations, the lasso's exact geometry within §2.3's rules, and error wording where §2 gives none |
| **How each commit is checked** | Run 7.3–9 §3 (AGENTS.md S1–S4). Any existing file in `joinn-visual`, `joinn-gpu` or `joinn-shell-desktop` means `gate 6`, `gate 7`, `gate 7.2`, `gate 7.3`, `gate 7.4`, `gate 8` and `gate B` |
| **Who checks** | Claude, at the phase stop, from a fresh clone. CI on the push, read once. AJ runs the window (§6.2) |
| **Input** | Never move the system mouse, click, or type outside a window Cursor started (rule 67) |

**Numbering used by this plan.** Highest at writing: **V173**, **R133**, rule **93**. Blocks in order: Phase B V174–V176, R134–R136, rule 94. **Phase C: V177–V180, R137–R139, rule 95.** Phase D: V181–V184, R140–R143, rule 96.

### 0.1 The commit report

Phase B's §0.1 block (`Commit`, `Done-when`, `Check`, `Gates`, `Snags`), every value copied from the terminal.

### 0.2 The stop report

Run 7.3–9 §4, the short form, at `docs/Findings/phase-C-stop.md`. Commit it as `PC-stop: stop report`.

### 0.3 Snags

As Phase 7.4's §0.3. Write the snag in the next commit report with the exact output, take the stated fallback, keep going. Predicted pixels and counts that differ are pasted beside the plan's value. Never widen the edge band (rule 57), never edit `gates.lock`, never move a corpus hash.

---

## 1. What Already Holds, and the Scope Fence

### 1.1 What already holds

| Fact | Where it is checked |
|---|---|
| The contact picture: `layout_contact` places `cli_a 4 4 12 6`, `cli_b 4 10 12 6`, `sum 16 4 12 12 response` in a `32 × 20` surface, with ports `cli_a@0`, `cli_b@0`, `sum@2` only | test `the_calculator_contact_prints_the_plan_block` (`joinn-visual/src/layout/layout_contact.rs`) |
| Gate 7 item 3, "The body is drawn as cells in contact": every probe names its owner, the seams `cli_a`/`sum` (columns 639 / 640) and `cli_a`/`cli_b` (rows 359 / 360) cross straight from cell to cell, `7/7` owners and no link, colours after each event, regrow equal, GPU dropped and regrown identical | `g7_picture`, `g7_probes`, `pick/contact_lines.rs` (`… owners n/total, links l`; any `WIRE_TAG` owner fails citing V130) |
| Gate 7 item 3's control: true when no response cell owns a pixel; opposes `DropForce("sum")` | `g7_picture_control` |
| A force in a system is a lasso: eight outline strokes, a neck and an arrowhead (`lasso_outline`, `check_lasso`, `LASSO_GAP 2`, `NECK 6`, `ARROW 2`, `LASSO_HALF_WIDTH` ½ unit); Stroke rows, style `STYLE_FORCE` (15), ID `[1, 0, FORCE_TAG, generation]` (`FORCE_TAG = 0x6000_0000`); printed `force count` | V164; gate 7.4 item 3; `joinn-visual/src/system_layout*`, `system_scene/write_rows.rs`, `system_scene/print_id.rs` |
| The GPU draws lasso strokes before every body, cell and port (`draw_at.rs`: `KIND_LASSO`, then the shapes, then `KIND_LINK`, ports, text) from Stroke rows whose style is `STYLE_FORCE` (`curve.wgsl`) | gate 7.4 item 3 on every adapter |
| `SystemScene::shapes()` gives the CPU the same order: lasso strokes, then `shapes_of_tables`, then text | V164 |
| The shell's run path on the contact writes rows `2, 1, 3` and keeps tables equal to regrow | V125; test `the_shell_run_path_keeps_tables_equal_to_regrow_on_the_contact_calculator` |
| The contact picture's source is the source the engine runs: `contact_surface(c) = surface(lower(c))` | V128; Phase B makes `calculator.contact` the one source |
| `pick: 3 adapter(s), 12 subject(s) agree; planted disagreement: refused as truth violation (ok)` | Phase 8 stop |

### 1.2 What V130 says, and what Phase C changes

V130 (`decisions.md`, Phase 7, `holds`): *A force owns no pixel; a contact picture has no link owner* — evidence "gate 7 item 3". In code, its second half is the `links` count in `contact_lines` (any `WIRE_TAG` owner fails, citing V130); its first half holds because the contact scene writes no row for a force, so `owners 7/7` has no room for one.

Phase C keeps the second half and reverses the first, for the contact picture: **a force owns the pixels of its lasso and nothing else.** That is rule 64's wording for systems, made true here too.

### 1.3 In scope

- The dated V130 finding and the superseding V177 (§2.1).
- The contact layout: members in contact, the response at the lasso's point, the lasso (§2.2, §2.3).
- The contact scene writes the lasso as Stroke rows; the body picker names it (§2.4, §2.5).
- Gate 7 item 3 amended (§2.7); the shell's existing contact tests moved to the new pixels.
- The shell: a click on the lasso prints `force sum` (§2.8).
- Gate C, one item, with standing plants in `cargo xtask pick` (§6). Findings `docs/Findings/phase-C-lasso.md`.

### 1.4 Out of scope

| Not now | Why it is tempting |
|---|---|
| A block editor, dragging a block into a lasso, or any new key | Text-backed editing is enough for this phase (refactor plan Phase C); the creator plan (Phase 9) is not built |
| Drawing the interior ports the force uses (`cli_a@1`, `cli_b@1`, `sum@0`, `sum@1`) | Rule 64: interior ports are not drawn. V134 already lets `present` name them |
| Drawing `lower`'s deliveries as wires | Rule 61: the compilation is never drawn |
| The contact inside a universe | Universes bind compiled bodies (rule 36); systems in universes is unbuilt |
| Chained forces, or a force whose members don't touch in one column | `layout_contact` refuses them today (R84); unchanged |
| One relationship type for link and force | Phase D |
| Changing any corpus file or hash | T1 |

---

## 2. Decisions

### 2.1 The V130 amendment (**DECIDED** that Phase C amends V130 on purpose, refactor plan Phase C; the record **PROPOSED**)

1. **A dated finding first (PC-02, before any code):** `docs/Findings/phase-C-v130.md`, dated, with: V130's words and evidence; why it was right in Phase 7 (a force had no look yet; Part VII, which drew forces as lassos, came after Phase 7.3); what changed (Part VII G6 and G7, decided; rule 64 since Phase 7.4; the refactor plan's Phase C); exactly what gate 7 item 3 changes (§2.7); the new row V177.
2. **The ledger:** V130's status becomes `reversed`, its `later` column `Findings/phase-C-v130.md; superseded by V177` (the ledger requires a `reversed` line to name a finding; `cargo xtask decisions` checks it).
3. **The superseding row, V177**, carries V130's surviving half: *In a contact picture a force owns the pixels of its lasso and nothing else, and the picture has no wire or link owner.*

### 2.2 What the window draws (**DECIDED**: blocks, ports, contact and the lasso, refactor plan Phase C; rules 56, 64; the layout **PROPOSED**)

| Thing | Drawn as | Owner, as a click prints it |
|---|---|---|
| The hypernode's surface | the surface rectangle, as today | `body surface` |
| A genome cell (`cli_a`, `cli_b`) | a cell rectangle; the members of one force stacked in one column, **sharing edges** | `body.cli_a`, `body.cli_b` |
| A force's response (`sum`) | a cell rectangle **one neck's length right of** the lasso, spanning its members' height, not touching them; latent until it holds a value, as today | `body.sum` |
| Surface ports: every port no force reaches, plus each response's out-port (rule 30) | port dots, as today: `cli_a@0`, `cli_b@0` in; `sum@2` out | `body.cli_a@0`, `body.cli_b@0`, `body.sum@2` |
| Contact between blocks | **shared edges only.** No stroke, wire or link row joins two cells | (the cells' own pixels) |
| The force | a **lasso**: the closed outline around its members, a neck, and a two-stroke arrowhead whose tip touches the response's in-port side | `force sum` |
| Interior ports, `lower`'s deliveries | **not drawn** (rules 61, 64) | — |

Predicted `print_layout` block for the calculator (layout units; Cursor may change the numbers only within §2.3's rules, and pastes the real block):

```
layout body
surface 0 0 40 20 r 3
cell cli_a 4 4 12 6 r 2
cell cli_b 4 10 12 6 r 2
cell sum 24 4 12 12 r 2 response
port cli_a@0 in 4 7 r 1
port cli_b@0 in 4 13 r 1
port sum@2 out 36 7 r 1
lasso sum outline 2 2 18 18 neck 18 10 tip 24 10
```

The `lasso` line is printed (by `print_layout`, and by `xtask`'s `layout_block` for `cargo xtask layout`) only when a layout has one, so every wired body's layout prints byte for byte as before (`cargo xtask layout corpus/phase2/calculator.body` and the existing `layout_text` test on it unchanged).

### 2.3 The contact lasso's rules (**PROPOSED**, adapted from plan 7.4 §2.8)

The geometry reuses `lasso_outline(&members_span, LASSO_GAP)` and the neck and arrowhead constants. It is checked exactly, in layout units, by a new `check_contact_lasso` (or by `check_lasso` generalised, if both callers keep their refusal words):

1. **Clear.** No lasso stroke, with its half-width, meets any cell rectangle or any drawn port dot; no outline stroke meets the response. Every stroke lies strictly inside the hypernode's surface.
2. **Holds its members, and only them.** Every member's cell lies strictly inside the outline; no other cell does.
3. **Points at its response.** The response lies outside the outline, and the arrowhead's tip is on its in-port side (`tip.x == response.x`, inside its height).
4. **Contact is touching.** The members of each force share edges in one column (as `layout_contact` already requires), and the response shares no edge with any member.

### 2.4 The scene writes the lasso (**PROPOSED**; drawing order **DECIDED** by rule 64 and plan 7.4 §2.8)

- `Scene::grow_contact` and `Scene::regrow_contact` write each force's lasso as Stroke rows, style `STYLE_FORCE`, half-width `LASSO_HALF_WIDTH`, owner `[body + 1, 0, FORCE_TAG | force slot]` with the row's generation. Force slots follow canonical force order (sorted by response name).
- A run writes **no** lasso row: `apply_run` writes cell and port rows only, so the shell's contact rows stay `2, 1, 3` (V180).
- A new `Scene::shapes()` gives the CPU pickers the GPU's order: lasso strokes, then `shapes_of_tables`, then text strokes, as `SystemScene::shapes()` does. Every contact picker (`contact_subjects`, `contact_lines`, `g7_picture`, `g7_probes`, the shell's click) uses it.

### 2.5 The owner (**PROPOSED**)

- `joinn_visual::Owner` gains `Force(String)` (the response name). `Scene::resolve` gains the `(0, FORCE_TAG)` arm; a stale generation is refused as other rows are.
- `print_owner` prints it **`force <name>`**, without the alias, exactly as `SystemScene::print_id` prints `force count`. So one force owner reads the same in a system and in a contact.
- No owner string that exists today changes.

### 2.6 The GPU (**PROPOSED**: nothing to change)

`curve.wgsl` already draws `KIND_LASSO` from Stroke rows with `STYLE_FORCE` and writes the row's owner to the ID target; `draw_at.rs` already draws it before every shape. Prediction: `git diff --stat <PC-01>..HEAD -- joinn/crates/joinn-gpu` prints nothing. If a stroke row needs a field the body scene doesn't set (its chart or band), that is a snag with the GPU line printed, and the fix goes in `joinn-visual`'s row, never in the shader's decision.

### 2.7 Gate 7 item 3, amended (**DECIDED** that it changes; the changes **PROPOSED**)

Its name (`The body is drawn as cells in contact`), check and control names, `control_artifact` (`corpus/phase7/calculator.contact`), `opposes` (`DropForce("sum")`) and the lock line `phase 7: 3/3` stay. What changes, all in one commit with the finding's reference:

| Where | Today | After Phase C |
|---|---|---|
| `g7_probes`'s `PROBES` (1280×720) | `(192, 104)` surface, `(448, 264)` cli_a, `(448, 456)` cli_b, `(832, 360)` sum, `(256, 264)` cli_a@0, `(256, 456)` cli_b@0, `(1024, 264)` sum@2, `(0, 0)` background | the same owners at the new layout's pixels (predicted below), plus `(668, 360)` **`force sum`** |
| `g7_colors`'s `PIXELS` (the same eight probes; `RGB` holds each step's colours) | the old pixels | the same owners at the new pixels; every colour in `RGB` unchanged; one new column, the neck at `(668, 360)`, `0xB46CE0` (`STYLE_FORCE`) at every step |
| `g7_probes`'s seams | `cli_a`/`sum` at columns 639 / 640; `cli_a`/`cli_b` at rows 359 / 360 | `cli_a`/`cli_b` at rows 359 / 360 only (that is the contact); the `cli_a`/`sum` seam is removed, because the response no longer touches its members (§2.3 rule 4) |
| `contact_lines`'s owner count | `owners n/total`, `total` = number of shapes | `total` = number of **distinct** owner IDs (eleven lasso strokes share one ID), so `owners 8/8` |
| `contact_lines`'s line | `…, owners n/total, links l` | `…, owners n/total, force owners f, links l` |
| `contact_lines`'s failure for a wire owner | `… link owner(s) in a contact picture; a force owns no pixel (V130)` | `… wire or link owner(s) in a contact picture; acceptance is none (V177)` |
| `g7_picture`'s doc comment | `7/7 owners and no link` | `8/8 owners, one force owner, no link` |

**Predicted pixels** at 1280×720. `fit` takes the largest multiple of 4 with `40·k ≤ 1248` and `20·k ≤ 688`, so `k = 28`, origin `(80, 80)`: a layout point `(x, y)` is pixel `(80 + 28x, 80 + 28y)`.

| Owner | Layout point | Pixel |
|---|---|---|
| `body surface` | (1, 1) | (108, 108) |
| `body.cli_a` | (10, 7) | (360, 276) |
| `body.cli_b` | (10, 13) | (360, 444) |
| `body.sum` | (30, 10) | (920, 360) |
| `body.cli_a@0` | (4, 7) | (192, 276) |
| `body.cli_b@0` | (4, 13) | (192, 444) |
| `body.sum@2` | (36, 7) | (1088, 276) |
| `force sum` | (21, 10), on the neck | (668, 360) |
| `background` | — | (0, 0) |
| seam `cli_a` / `cli_b` | y = 10 | rows 359 / 360, at three columns inside both cells (the cells span pixels 192 to 528; Cursor picks three, as today) |

The same old pixels are also written in a test in `joinn-visual/src/scene/regrow_contact.rs` and in `joinn-shell-desktop/src/session/enter.rs` (`CONTACT_SCRIPT`, whose comment reads "pixels (256, 264) and (256, 456) at k 32"); each moves to this table, and the comment to `k 28`.

**The demos that show the amended check still refuses something** (PC-04, shown then reverted with `gate 7 --item 3`): (a) the contact scene given one wire row `cli_a@1 → sum@0` (drawing the compilation, rule 61): the check fails with `links 1 … (V177)`; (b) `sum` moved back to touch `cli_a`: §2.3 rule 4 refuses the layout, so the contact has no picture and the item fails naming the rule.

### 2.8 The shell (**DECIDED**: text-backed editing, refactor plan Phase C; the lines **PROPOSED**)

What exists and stays: `joinn-desktop corpus/phase7/calculator.contact` opens the contact (`load_contact_file`, `Desktop::open` with the contact); clicking an in-port selects it (`selected body.cli_a@0`), typing fills the buffer, Enter runs the lowered body; `sum` lights when the second number arrives.

What changes: the pixels in the session tests that click (`CONTACT_SCRIPT` in `session/enter.rs`: `(256, 264)` and `(256, 456)` become `(192, 276)` and `(192, 444)`; `clicking_sum_in_the_contact_picture_names_the_response` clicks `(920, 360)`), and a click on the lasso prints `pick 668,360: force sum (cpu)` then the GPU confirm line, as `force count` does in a system. No new key.

### 2.9 Invariants

| # | Invariant | Commit |
|---|---|---|
| **V177** | In a contact picture a force owns the pixels of its lasso and nothing else, and the picture has no wire or link owner (supersedes V130) | PC-04 |
| **V178** | The contact picture is the form the engine runs: its cells are the genome's instances and each force's response; its ports are `contact_surface(c)`, which is `surface(lower(c))`; each lasso holds exactly its force's members and points at its response | PC-04, PC-06 |
| **V179** | Contact is drawn as touching, never as a line: a force's members share edges, the response touches none of them, and no stroke, wire or link joins two cells | PC-03 |
| **V180** | The lasso is written at grow and regrow alike and never by a run: tables after the script equal regrow, and each Enter writes the rows it wrote before Phase C | PC-04 |

---

## 3. What Runs

| Command | Prints | Exit 1 when |
|---|---|---|
| `cargo xtask layout corpus/phase7/calculator.contact` | §2.2's block | the layout is refused |
| `cargo xtask pick` | every earlier line, the contact lines now `calculator.contact <w>x<h>: agree …, edge …, disagree 0, owners 8/8, force owners 1, links 0`; the plants (§6); last line `pick: 3 adapter(s), 12 subject(s) agree; planted disagreement: refused as truth violation (ok); contact plants: lasso over cells, lasso inward, lasso unpicked, interior port: refused (ok)` | as before, or a plant admitted |
| `cargo xtask regrow` | every earlier line; the contact's `regrow contact …: color identical, ids identical` lines with the lasso drawn | as before |
| `cargo xtask gate C` | the item's row, `item 1 control answered by admission`, its standing plants named, `phase C: 1/1` | the item fails |

**Predictions** (marked in the findings, PC-08):

| # | Prediction |
|---|---|
| P1 | The layout block of §2.2, byte for byte |
| P2 | `fit` gives `k = 28`, origin `(80, 80)` at 1280×720; every pixel of §2.7's table names its owner on CPU and GPU, on every adapter |
| P3 | `owners 8/8, force owners 1, links 0, disagree 0` at every standard viewport on every adapter |
| P4 | The shell's contact rows stay `2, 1, 3` (V125, V180) |
| P5 | No file under `joinn/crates/joinn-gpu` changes |
| P6 | Every other line of `pick` and `regrow` (the wired calculator, grove, systems, strand) is unchanged: `Compare-Object` of the before and after output prints only the contact lines and the last line |
| P7 | `corpus verify: 67 hash(es) match; cells admitted`; `git diff --stat <PC-01>..HEAD -- joinn/corpus` prints nothing |
| P8 | `gate all` prints every earlier phase as `gates.lock`, then `phase C: 1/1` |

---

## 4. The Commits

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **PC-01** | **Plan and header** | This plan (Draft 1.0 after AJ's review); `AGENTS.md` header per Appendix A part 1; backlog R137–R139; `decisions.md` V177–V180 and R137–R139 (`open`) | `git show --stat HEAD` lists the plan, `AGENTS.md`, the backlog, `decisions.md`. `Check: docs only` |
| **PC-02** | **The V130 finding** | §2.1: `docs/Findings/phase-C-v130.md`; V130 → `reversed` naming it | `cargo xtask decisions` exits 0 (paste its counts). `Check: docs only` |
| **PC-03** | **The contact layout** | §2.2, §2.3: `layout_contact` places responses at the lasso's point and lays out each force's lasso; `Layout` carries the lassos; `print_layout` and `layout_block` print a `lasso` line only when present; `check_contact_lasso`; gate 7 item 3's probes, seams and colour pixels moved to the new geometry (§2.7, without the lasso's probe and colour column yet); the same pixels in the test of `joinn-visual/src/scene/regrow_contact.rs` and the shell's session tests (§2.8) moved; V179 | Tests: the plan block (P1) replaces the old one in `the_calculator_contact_prints_the_plan_block`; the four rules hold; a stroke moved 2 units inward is refused by rule 1; `sum` touching `cli_a` is refused by rule 4; the `layout_text` test on `phase2/calculator.body` unchanged. `gate 6`, `7`, `7.2`, `7.3`, `7.4`, `8`, `B` each full |
| **PC-04** | **The scene draws the lasso** | §2.4, §2.5, §2.7: Stroke rows in `grow_contact` and `regrow_contact`; `Owner::Force`; `resolve`; `print_owner`; `Scene::shapes()`; the contact pickers use it; `contact_lines` counts distinct owners and force owners, cites V177; the lasso's probe and colour column; V177, V180 | `cargo xtask pick` printed whole (paste): P3 on every adapter. Tests: tables after the script equal regrow; a run writes no lasso row; `print_owner` gives `force sum`. **Shown then reverted** with `gate 7 --item 3`: §2.7's demos (a) and (b). The same gate list as PC-03, each full |
| **PC-05** | **The shell names the lasso** | §2.8 | A session test (no window): click `(668, 360)` → `pick 668,360: force sum (cpu)`; the contact script still ends with `sum` filled and rows `2, 1, 3` (P4). Cursor runs the window once and pastes the lines, or writes `no display` |
| **PC-06** | **Gate C** | §6; `phase C` after `phase B` in `PHASE_LABELS`; `gate_c.rs`, `gate_c_items.rs`, `gc_lasso.rs`, `gc_lasso_control.rs`; the row in `gate_table`; the lock row; `only_contact_gates_name_a_contact_artifact` lists `phase C`; the four standing plants in `pick`; V178 | `cargo xtask gate all` exits 0 and prints `phase C: 1/1` and `item 1 control answered by admission`. `cargo xtask pick`'s last line names the four plants (paste). **Shown then reverted** with `gate C --item 1` only: (a) the lasso drawn after the cells (the arrowhead's tip pixels on `sum` become the force's): the item fails; (b) `Scene::shapes()` drops the lasso (the picture drifts from the tables): the item fails with a TRUTH VIOLATION line; (c) `sum@0` drawn: the item fails naming the interior port |
| **PC-07** | **Rule 64** | Appendix A part 2: rule 64 replaced, rule 95 appended | `git show --stat HEAD` lists `AGENTS.md` only. `Check: docs only` |
| **PC-08** | **Findings** | `docs/Findings/phase-C-lasso.md`: every command's output from PC-03 on; P1–P8 marked; *The adversary* (§7.1) answered; the amended cross-gate test list | Every prediction marked |
| **PC-09** | **Docs and freeze** | README; Guides 03 (`## The calculator opens (Phase C)` above Phase B's section; the Phase 7 record stays), 04 (the 30 Oct note: the force is now drawn as a lasso), 05 (Force, Lasso, Owner: the contact picture's lasso, `force sum`); `decisions.md` V177–V180 with status and evidence | Tier 1. `corpus verify` 67; `git diff --stat <PC-01>..HEAD -- joinn/corpus` prints nothing |
| — | **The phase stop** | `cargo xtask stop-check --fresh` (phases 0 … 8, B, C; exits 0); `phase-C-stop.md`; commit `PC-stop`, push, ledger; print `phase C: stopped for review`; stop | — |

Dependencies: in order. PC-03 depends on PC-02 (the record comes before the change); PC-05 and PC-06 on PC-04; PC-07 on PC-06.

**If something has to be cut for time:** the window run in PC-05 first (the session test stays). Never cut PC-02, PC-04, PC-06 or V177–V179.

---

## 5. Test Strategy

As Phase 7.4's §5. Picture tests compare bytes and owners; no test asserts only that a lasso "exists". Every predicted pixel is checked on CPU and GPU. Pixel numbers in existing tests change only in PC-03 and PC-04, each change listed in the finding.

---

## 6. Exit Gate C

One item in `xtask/src/fns/gate_c_items.rs`. Every GPU part runs on every adapter (rule 60).

| # | Item | `control_artifact` | `opposes` | Control answers `true` when |
|---|---|---|---|---|
| 1 | **The hypernode is drawn as the engine runs it** | `corpus/phase7/calculator.contact` | `DropMember("sum", "cli_b@1")` | the subject is refused at admission, or its layout breaks a rule of §2.3, or the owners its scene draws differ from what its coding implies: the genome's cells and each response, exactly the ports of `contact_surface`, and one lasso per force holding exactly that force's members |

- **On the subject** the control answers `false`: eight owners, as the coding implies.
- **On the mutant** it answers `true` **by admission**: `force sum has 1 members; its response takes 2. acceptance is 2 members (R84)` (the words `drop_member`'s test already asserts).
- **On the neutral edit** (`sum`'s label) it answers `false`: labels are regulatory and draw no owner.

The lasso's truths are true by construction for every admitted contact, so, as gate 7.4 item 3 does, the gate names its **standing plants** (rule 93), run by `cargo xtask pick` on every run:

| Plant | Must be refused by |
|---|---|
| (a) the lasso's rows drawn after the cells | the check: the force owns pixels of `sum` (rule 64, V177) |
| (b) one outline stroke moved 2 units inward | §2.3 rule 1: `lasso: stroke 0 meets cli_a; …` |
| (c) the CPU's shapes without the lasso rows | the pickers: a TRUTH VIOLATION at a lasso pixel (rules 56, 57) |
| (d) a port row for `sum@0` | V178: the drawn ports differ from `contact_surface` |

**Check (`gc_lasso`)**, on every adapter and standard viewport: `disagree 0`; owners exactly the eight of §2.7, with `force owners 1` and `links 0`; no cell, port or surface pixel owned by the force; the pick at `(668, 360)` names `force sum` on CPU and GPU; the drawn ports equal `contact_surface(c)` and `surface(lower(c))`; after the script `two`, `2`, `3` the lasso rows are untouched and the tables equal regrow; GPU state dropped and regrown draws identical colour and ID bytes; the four plants are refused.

This row does not restate gate 7 item 3 (rule 41): gate 7 item 3 judges cells in contact, colours and the run path; this row judges the force's lasso, its owner, and that the picture is the compiled form's surface.

### 6.1 Conditions for opening Phase D

1. `stop-check --fresh` exits 0 with `phase C: 1/1` and every earlier line as `gates.lock`; no tripwire fired.
2. `phase-C-lasso.md` exists with every prediction marked and the adversary answered.
3. Claude's review lists no open snag, or AJ has chosen to carry each one forward.
4. AJ has run the window (§6.2), or each thing that looked wrong is a snag Claude has settled.

### 6.2 AJ's window check (at the phase stop, about three minutes)

```
cd D:\JoInn\joinn
cargo run --release -p joinn-shell-desktop -- corpus/phase7/calculator.contact
```

1. You see `cli_a` above `cli_b`, touching, inside a loop; the loop's arrow points at `sum`, which stands apart and is dim.
2. Click the loop. The terminal prints `force sum`.
3. Click `cli_a`'s port on the left, type `2`, Enter; click `cli_b`'s port, type `3`, Enter. `sum` lights.
4. Click `sum`. The terminal prints `body.sum`.
5. Close the window. Tell Claude whether the loop reads as "these two combine into that one" at a glance.

---

## 7. Adversary, Risks and Tripwires

### 7.1 The adversary

**The picture drifts from the tables.** A lasso drawn by the GPU that the CPU's shapes don't hold (plant c), a lasso that owns a block's pixels (plant a), or a port the engine doesn't expose (plant d). The findings answer it with the four refusal lines and P3 on every adapter.

### 7.2 Risks

| Risk | What to do |
|---|---|
| The wider picture changes a pixel some other test or gate depends on | Each is listed by PC-03's gate runs; moved pixels are recorded in the finding. A gate other than 7 that fails is a snag with its line, never a widened check |
| The lasso's arrowhead overlaps `sum` by its half-width | Expected, as in 7.4: the lasso is drawn under the cells, so `sum` owns those pixels; plant (a) is exactly this order reversed |
| `Owner` gaining a variant breaks an exhaustive `match` elsewhere | Fix each match in the same commit; none may print a new string for an existing owner |
| A Stroke row in a body scene is hidden by a band or chart field | Snag; set the row's field in `joinn-visual` (§2.6) |
| The lasso, as straight strokes, looks crude | AJ judges at the window; exactness first (rule 70) |

### 7.3 Tripwires

Run 7.3–9 §7 applies (T1–T5). For T1, no file under `joinn/corpus/` changes in this phase.

---

## 8. Open Items

| ID | Topic | Question |
|---|---|---|
| **R137** | Several forces in one hypernode | With two forces (R84's chains, once admitted), does each get its own lasso, and may one lasso's response sit inside another lasso? |
| **R138** | The lasso as a depth | Is the set a lasso holds a hypernode of its own (the contraction Part VII names), so zooming into the lasso is Phase E's depth? |
| **R139** | Editing by contact | When editing leaves text, is "drag a block inside a lasso" the gesture that adds a member, and what refuses it (frame, receptor)? |

---

## Appendix A · `AGENTS.md` changes

**Part 1 (PC-01).** Replace the header paragraph Phase B wrote with:

```markdown
The active plan is docs/Plans/JoInn Hypernode Refactor Plan.md. Phases A and B
are closed. Phase C, open the hypernode, is built from docs/Plans/JoInn Phase C
Implementation Plan.md: the contact calculator is drawn as blocks in contact,
its surface ports, and its force as a lasso, and V130 is superseded by V177 on
purpose. Work the phase's commits in order, one git commit per numbered step,
check each by tier, and stop once at the phase's end. Never stop to ask; follow
the plan's Snags section instead.
```

**Part 2 (PC-07).** Replace rule 64 with:

```markdown
64. A FORCE IS SEEN AS A LASSO, IN A SYSTEM AND IN A CONTACT. It owns the
    pixels of its lasso and nothing else: no cell, port or surface pixel.
    Interior ports are not drawn, and a contact picture has no wire or link
    rows. A link and a force never look alike (Part VII G19).
```

Append:

```markdown
95. A HYPERNODE IS DRAWN AS THE ENGINE RUNS IT. A contact picture's cells are
    its genome and its responses, its ports are contact_surface (which is
    surface(lower)), and each lasso holds exactly its force's members. Contact
    is drawn as shared edges, never as a line.
```

Keep every other rule.

## Appendix B · Documents

- **Backlog**, after R136: `> **30 Oct 2026 · Hypernode refactor, Phase C.** R137–R139 come from the Phase C plan.` then R137–R139 (`open`) with §8's questions.
- **`decisions.md`**: V130 `reversed` (PC-02); V177–V180 (`open`; PC-09 sets them); R137–R139 (phase `C`, `open`).

---

*JoInn Phase C Implementation Plan, Draft 0.1 (30 Oct 2026), written from the Hypernode Refactor Plan's Phase C. AJ decided (refactor plan, 30 Oct; Part VII G6, G7, G19): the window shows the contact calculator as blocks, ports, contact and the lasso; editing stays text-backed; the picture is the canonical form the engine runs. Proposed, open to AJ's veto: moving `sum` to the lasso's point, the contact lasso's rules, `force sum` as the owner, the V130 record, the gate 7 item 3 changes, and one gate item with four standing plants.*
