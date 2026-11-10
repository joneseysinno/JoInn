# JoInn Workspace Sketch Plan

**A creator workspace AJ can see and judge: one nestable cell, zoom as the third dimension, time as the fourth, and a first analysis worksheet**

Draft 1.0 · 10 Nov 2026 · every decision below is settled by AJ in the 10 Nov conversation. Cursor executes; nothing here waits on AJ until the stop.

> **Status.** AJ's direction, 10 Nov 2026: *"I need to be able to see a product to know if I like it. We need to get an app creator workspace created so I can see the product. We can then worry about how to store and create the binary for the apps."*
>
> This plan is now the active plan. Phases B, C and D of the Hypernode Refactor Plan are **parked** (not cancelled). DNA, storage, the compiler and the optimizer come after AJ has judged the workspace.
>
> This is a **sketch**: a spike outside the gates, like `spikes/s8-beam`. Its only question is "does AJ like it". It proves nothing and no gate measures it.

---

## For AJ: this plan in plain English

**What gets built.** A window with a canvas in the middle, a palette of five base cells on the left, a details panel on the right, and a time slider along the bottom.

**What you can do in it at the first stop:**

1. Drag a cell from the palette onto the canvas, name it, and give it a value.
2. Drag cells inside other cells, as deep as you like.
3. Zoom with the wheel from the whole worksheet down into one cell and back. A trail at the top always says where you are; Home takes you to the top.
4. Type a formula into a Result (`w * L^2 / 8`) and see the answer, with lines drawn from the cells it uses.
5. Drag the time slider back and watch the worksheet un-build, step by step.
6. Open the example worksheet, **Simple beam**: a 24 ft span under 1.2 kip/ft. It must show reactions of 14.4 kip and a moment of 86.4 kip·ft.

**What you do at the stop** (about ten minutes): run one command Cursor prints, try the six things above, and tell Claude what you like and don't like. That is the whole review.

**What is deliberately missing:** exact pixels, hashes, gates, DNA, real unit handling (units are labels for now), and the GPU renderer. Those come back once you know you like the product.

---

## 0. How Cursor works this plan

| | |
|---|---|
| **Plan file** | `D:\JoInn\docs\Plans\JoInn Workspace Sketch Plan.md` |
| **Code** | `D:\JoInn\joinn\spikes\s9-workspace\`: its own Cargo workspace with its own `Cargo.lock`. It is **not** a member of `joinn/Cargo.toml` and no crate under `joinn/crates/` may depend on it or be changed by it |
| **Unit of work** | Sitting 1 (§4): every commit in order, each its own git commit whose message starts with its id (`W1-03: …`). **One stop**, at the end of Sitting 1 |
| **Who decides** | Every product decision is in §2. Cursor decides module layout, function bodies, styling details and wording where §2 gives none |
| **Never stop to ask** | A step that can't be done as written: write the snag in the commit message with the exact output, take the stated fallback if there is one, otherwise the simplest thing that keeps AJ's six actions working, and keep going |
| **AJ supplies** | Nothing during the run. No hashes, no pasted output, no filled-in files |

### 0.1 What the sketch is exempt from

The spike is outside JoInn's truth core, so these do **not** apply inside `spikes/s9-workspace/`: the gates and `gates.lock`, `cargo xtask check`, `stop-check`, the vocabulary scan, the modules and layers checks, corpus hashes, the mutation catalogue, the no-float rule in screen code, the `BTreeMap`-only rule, and the workspace clippy denials. The sketch is **not** added to `stop-check` or CI.

What still applies: no `unsafe`; no panics in ordinary use (a bad formula is shown, never a crash); a refusal is a value the screen shows.

### 0.2 What must not change

Nothing under `joinn/crates/`, `joinn/xtask/`, `joinn/corpus/`, `joinn/gates.lock` or `.github/`. `git diff --stat` for the run lists only `joinn/spikes/s9-workspace/` and the documents named in W1-01 and W1-09. If the tree has uncommitted changes when the run starts, Cursor lists them in the W1-01 commit message and leaves them alone.

---

## 1. The idea in one paragraph

The workspace is four-dimensional. **x and y** are where a cell sits inside its parent. **Depth** is the third dimension: how far in you have zoomed, so zooming is travelling along it. **Time** is the fourth: everything that happens is recorded in order and can be scrubbed. There is one kind of thing, the **cell**, and it nests. A worksheet, a page, a section and a single number are all cells at different depths. The workspace itself is the outermost cell.

---

## 2. Decisions (all settled)

### 2.1 The stack

- **Dioxus, stable 0.7 line, desktop target** on Windows. Web is not built in this sketch.
- Two crates in the spike's workspace:
  - `workspace-model`: cells, nesting, relationships, formulas, the time log. **No screen code and no Dioxus dependency.** It is the part that survives if the renderer is later replaced by JoInn's wgpu window.
  - `workspace-app`: the Dioxus program. It only draws the model and sends it edits.
- Fallback if Dioxus desktop won't build or open on AJ's machine: the same app on the Dioxus web target served locally, with the run command in the stop report. Never switch UI library.

### 2.2 The anatomy of a cell

Every cell has exactly four parts:

| Part | What it is |
|---|---|
| **Name** | What it's called. Unique among its siblings |
| **Inside** | Either one piece of data, or other cells |
| **Face** | What it shows its parent when it is too small to open |
| **Ports** | Where relationships attach. In Sitting 1 a cell has one port, itself |

### 2.3 The five base cells

No sixth is added in this plan.

| Base cell | Inside | Face when small |
|---|---|---|
| **Number** | A value and a unit label | `24 ft` |
| **Text** | Words: a heading, a note, a reference | Its first line |
| **Choice** | A list of options and the one picked | The pick |
| **Result** | A formula. It holds no value of its own; it responds | The answer and its unit label; `pass` or `fail` if the formula is a comparison |
| **Group** | Other cells, with an arrangement | Its name, then the faces of the children AJ has pinned to its face |

- A worksheet, a page and a section are Groups. They are not extra kinds.
- A Group's arrangement is **stack** (top to bottom, like a calc sheet; the default for a new Group) or **free** (cells sit where they are dropped; the root cell is free).
- For a Group, each child has a "show on face" tick in the details panel. A new Result is ticked by default; everything else is unticked.

### 2.4 Nesting

- Any cell can be dragged into any Group at any depth, and back out.
- A cell has exactly one parent. Dropping a Group into itself or its own descendant is refused with a visible message; nothing changes.
- Deleting a Group deletes what is inside it. Because of §2.8, scrubbing back brings it all back.

### 2.5 Zoom is the third dimension

- Wheel zooms about the cursor. Dragging empty canvas pans.
- **The camera always lives in one cell, the current cell.** The breadcrumb trail shows the path from the root to it.
- **Passing through a boundary.** When a child Group grows to fill most of the window, it becomes the current cell and the camera's numbers are re-based to it. When the current cell shrinks to a small part of the window, its parent becomes current. Zoom is therefore unlimited in both directions with ordinary numbers. Thresholds are Cursor's choice; there must be a gap between them so the view never flickers between two cells.
- Double-click a Group: enter it. Esc: go up one. Home: go to the root and frame everything.
- **Size decides how a cell is drawn**, in three bands by its height on screen: a **dot** (a small coloured bar with no text), its **face**, and **open** (its children drawn). Cursor picks the pixel thresholds; a short crossfade between bands is welcome but not required.
- At most two depths below the current cell are drawn open; anything deeper shows its face. Cells off screen are not drawn.
- Fallback if smooth boundary-passing can't be made stable: zoom is clamped inside the current cell, and entering and leaving are by double-click, Esc and the breadcrumb only. Record it as a snag.

### 2.6 Formulas and relationships

- A formula is **typed into the Result**, as in Mathcad. The workspace **draws the relationship automatically**: a line from each cell the formula uses to the Result, with an arrowhead at the Result.
- Formula language: numbers, cell names, `+ - * /`, `^` with a whole-number exponent, parentheses, and one optional comparison (`<= >= < >`) at the top, which makes the Result a check that shows `pass` or `fail`.
- **Finding a name.** A name is looked for among the Result's siblings first, then its parent's siblings, and so on outward; the nearest wins. `Inputs.L` reaches into a named Group. Names with spaces are written in quotes.
- **Arithmetic is exact.** Values are exact fractions; typing `1.2` means 6/5. A value is shown as a decimal; when the cell is selected the details panel shows the exact fraction as well.
- **Units are labels in this sketch.** A Number and a Result each carry a typed unit label. Labels are shown, never checked or converted.
- **A refusal is shown, never a crash.** A Result whose formula can't be read, names a cell that doesn't exist, names a cell that isn't a number, divides by zero, or depends on itself shows a short plain-English refusal in place of its answer, and is drawn in the refusal colour. Every other cell keeps working.
- **Two looks, never confused.** A formula's relationship is a coloured line with an arrowhead. A **link** is a plain grey line with no arrowhead that AJ draws by hand between any two cells at any depth (select one, press L, click the other); a link carries nothing in Sitting 1 and is there so AJ can see how joined cells read on screen.
- A line to a cell that is drawn as a face or a dot ends at that cell's edge. A line to a cell hidden inside a folded Group ends at the folded Group's edge, once, however many cells inside it are involved.

### 2.7 The layout of the window

| Region | Holds |
|---|---|
| Top bar | The breadcrumb trail; the worksheet's name; New, Open, and Open example |
| Left | The palette: the five base cells, dragged onto the canvas or into a Group. A Group highlights when a drop would land in it |
| Centre | The canvas |
| Right | Details of the selected cell: name, value and unit label, options and pick, formula, arrangement, show-on-face ticks, the exact fraction, the refusal if any, and Delete |
| Bottom | The time slider |

Select by click. Rename by double-clicking a name or in the details panel. Move by dragging. Delete with the Delete key or the button.

Look: clean and calm, light background, one accent colour, readable at a glance. It should look like a product, not a developer tool. Cursor's choice within that.

### 2.8 Time is the fourth dimension

- **One log.** Every change is appended to one ordered log. The workspace on screen is always "the log, replayed up to a moment". There is no other state.
- **Two kinds of entry**, both recorded from Sitting 1: an **edit** (add, remove, move, rename, change a formula, change an arrangement, pin to a face, draw a link) and a **value** (a Number's value or a Choice's pick changes).
- **The slider.** Dragging it back shows the workspace as it was at that entry. While in the past the workspace is read-only and a banner says so, with two buttons: **Return to now** and **Restore this moment**. Restore appends entries that bring the workspace back to that moment; nothing is ever erased from the log.
- Left and right arrow keys step one entry when the slider has focus.
- There is no separate undo in Sitting 1. Scrub and Restore are undo.
- **The log is the file.** A worksheet is saved as its log, as JSON, in `spikes/s9-workspace/saves/`, written after every entry. Open reads a log and replays it. This is a convenience so AJ doesn't lose work; it is not JoInn's storage and decides nothing about it.

### 2.9 The example worksheet

`Open example` loads **Simple beam**, shipped as a log so it can be scrubbed from empty to finished:

| Cell | Kind | Content |
|---|---|---|
| Simple beam | Group (stack) | the worksheet |
| · Title | Text | `Simple beam, uniform load` |
| · Inputs | Group (stack) | |
| · · L | Number | `24`, label `ft` |
| · · w | Number | `1.2`, label `kip/ft` |
| · · Ma | Number | `92.8`, label `kip·ft`, typed by hand as the allowable moment |
| · · Steel | Choice | `A992`, `A36`; picked `A992` |
| · Results | Group (stack) | |
| · · R | Result | `w * L / 2`, label `kip` → **14.4** |
| · · M | Result | `w * L^2 / 8`, label `kip·ft` → **86.4** |
| · · Check | Result | `M <= Ma` → **pass** |

These answers agree with the exact beam spike (`72/5` and `432/5`).

---

## 3. Sittings

| Sitting | What AJ can do at its end | Built by this plan |
|---|---|---|
| **1** | Everything in "For AJ" above | **Yes. Stop here** |
| **2** | **Use mode**: a second slider track. In Build mode you change the worksheet; in Use mode you only change inputs, as the app's user would. Each build moment holds its own run, so zooming the slider into one edit shows the app running as it was built then. **Blueprints**: save any cell as a blueprint and stamp it again | No. Written after AJ's Sitting 1 review |
| **3** | Real units, a picture cell, the lasso look for a formula's relationship, a printable page | No |

Sitting 1 records value entries separately from edit entries (§2.8) so Sitting 2's second track needs no change to saved worksheets.

---

## 4. The commits (Sitting 1)

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **W1-01** | **Plan and pointers** | This plan in `docs/Plans/`. `joinn/AGENTS.md` header's "active plan" paragraph replaced per Appendix A. One dated line at the top of the Hypernode Refactor Plan's status: B, C and D parked 10 Nov 2026, see this plan. No code | `git show --stat` lists only those three files |
| **W1-02** | **The model: cells and nesting** | `workspace-model`: the cell with its four parts, the five base cells, add, remove, move, rename; the refusal for a Group dropped into itself | Tests: build a tree three deep; move a cell between Groups; the self-drop is refused and the tree is unchanged |
| **W1-03** | **The model: the log** | Every change is a log entry of kind edit or value; state is the replay of the log up to an index; Restore; save and load as JSON | Tests: replay to every index of a 30-entry log equals the state recorded at that index; Restore to index k gives a state equal to index k and a longer log; save then load gives an equal log |
| **W1-04** | **The model: formulas** | §2.6: the formula reader, name finding, exact fractions, the comparison, every refusal; the list of cells each Result uses | Tests: the Simple beam log gives `72/5`, `432/5` and `pass`; changing `w` to `2` gives `24` and `144` and `fail`; each refusal in §2.6 is produced and names its cause; a Result that depends on itself is refused and its neighbours still answer |
| **W1-05** | **The window** | `workspace-app`: the five regions of §2.7; palette drag onto the canvas and into Groups; select, rename, move, delete; the details panel; stack and free arrangements | The app opens. A cell can be added, named, given a value, nested three deep, moved out, and deleted |
| **W1-06** | **Zoom and depth** | §2.5: wheel zoom about the cursor, pan, the three bands, the current cell, boundary-passing with re-basing, breadcrumb, double-click, Esc, Home | From the root, zooming into a cell four deep and back out returns to the same view; the breadcrumb is right at every step |
| **W1-07** | **Formulas on screen** | Typing a formula into a Result; answers and refusals shown; relationship lines with arrowheads; hand-drawn links; lines ending at folded Groups | The Simple beam built by hand in the window shows 14.4, 86.4 and pass; folding Inputs leaves one line end on its edge |
| **W1-08** | **Time on screen** | §2.8: the slider, the read-only past and its banner, Return to now, Restore this moment, arrow-key stepping; autosave; New, Open, Open example | Open example, drag the slider from the right end to the left: the worksheet un-builds to empty, and back |
| **W1-09** | **Example, notes and stop** | The Simple beam log in `saves/`; a short `README.md` in the spike with the run command and the keys; `docs/Findings/workspace-sketch-stop.md` (§5); a "Workspace sketch" section at the top of `docs/Guides/03-where-we-are.md` in plain words | `cargo test` in the spike passes; the app opens the example from a clean build |

Order: as listed. W1-05 to W1-08 depend on W1-02 to W1-04.

**If something has to be cut for time**, cut in this order: band crossfades; hand-drawn links; smooth boundary-passing (take §2.5's fallback); Restore this moment. Never cut nesting, zoom into a Group, typed formulas, the slider, or the example.

---

## 5. The stop

`docs/Findings/workspace-sketch-stop.md`, short and in plain words:

1. The one command that opens the workspace, on its own line.
2. The six things AJ can try, as a numbered list (from "For AJ").
3. What was cut or took a fallback, each in one sentence.
4. Snags, each in one sentence.
5. The spike's test count.

Commit it, push, print `workspace sketch: stopped for AJ's look`, and stop.

### 5.1 AJ's look (about ten minutes)

1. Open example. Zoom from the whole worksheet into `M` and back out.
2. Change `w` to `2`. Watch `M` and `Check` change.
3. Drag the slider to the far left and back.
4. Start a new worksheet. Make three cells, put two inside a Group, and add a Result that uses them.
5. Tell Claude: what felt right, what felt wrong, and whether this is the product.

### 5.2 What the look decides

| AJ says | Next |
|---|---|
| This is the direction | Claude writes Sitting 2 with AJ's notes folded in |
| Close, but change these things | Claude revises this sketch; no new sitting yet |
| Not this | The sketch is kept as a record and the conversation returns to what the product should be |

---

## 6. Risks

| Risk | What to do |
|---|---|
| The webview slows down with many cells | Draw only what is on screen and big enough (§2.5). The sketch needs hundreds of cells, not thousands. Record the count at which it feels slow |
| Zoom jitters or loses its place when passing a boundary | §2.5's fallback |
| Drag-and-drop into nested Groups is unreliable in the webview | Fallback: select a cell, press M, click the Group it should move into. Record as a snag |
| The sketch quietly becomes the product | It may not be depended on by `joinn/crates/`. What carries forward is `workspace-model`'s ideas and AJ's verdict, through a plan of its own |

---

## 7. Open, for after AJ's look

- How a cell here maps to the hypernode, block and relationship of the product language, and to Phase E's one tree of any depth.
- Whether the formula's relationship should be drawn as the lasso.
- Real units and the dimension work (Part VI).
- Whether the time log is the same thing as edit, edition, evolution and growth (Part VII).
- Moving the drawing from the webview to JoInn's wgpu window.
- The paper trial of whether the twelve amino acids can spell the calculator (still worth doing before any storage work resumes).

---

## Appendix A · `AGENTS.md` header

Replace the paragraph beginning "The active plan is" with:

```markdown
The active plan is docs/Plans/JoInn Workspace Sketch Plan.md (10 Nov 2026):
a creator workspace AJ can see and judge, built as a spike in
spikes/s9-workspace with Dioxus. It is outside the gates; the plan's §0.1
lists what it is exempt from. Nothing under crates/, xtask/ or corpus/
changes. Work the commits in order, one git commit per numbered step, and
stop once at the end of Sitting 1. Never stop to ask; follow the plan's
fallbacks. The Hypernode Refactor Plan's Phases B, C and D are parked until
AJ has judged the workspace. Phase 8's plan is finished work (gate 8
passed); rules 82–86 remain the storage rules.
```

Keep every other line of `AGENTS.md`.

---

*JoInn Workspace Sketch Plan, Draft 1.0 (10 Nov 2026). AJ decided: see a product first; a 4D workspace with nesting as the third dimension and time as the fourth; one nestable cell; both kinds of time; Dioxus for the sketch; five base cells built toward an analysis worksheet; formulas typed with the relationship drawn automatically; units as labels first.*
