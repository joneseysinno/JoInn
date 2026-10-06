# JoInn Phase 7.4 Implementation Plan

**Her counting app, part 1: the counting system · growth by DNA · counting as the witness · evolution to adding · the lasso**

Author: AJ, with Claude · Draft 0.1 · October 6, 2026

> **Every decision in this plan is final.** Nothing here waits on AJ. Cursor never stops to ask AJ anything, and no file in this phase is typed by AJ. If a step can't be done the way the plan says, Cursor follows §0.3 (Snags) and keeps going. This phase runs inside `docs/Plans/JoInn Run 7.3-9.md` (Draft 0.3) as **one unit with one stop**; that run's checks (§3), stop report (§4) and tripwires (§7) apply.

---

## For AJ: this plan in plain English

**What Phase 7.4 is.** It builds the first piece of the girl's counting app from Part VII (*Systems, Forces and Growth*): one **counting system** you can open in the window and watch grow.

**What you'll see.** Open `counting.system`. There's one empty box waiting, and a lasso around the body pointing to the count, which shows **0**. Type `1` and press Enter: the box fills, a new empty box appears, the lasso stretches around it, and the count shows **1**. Do it again and the count shows 2. Type `3` and it's refused: *counting grows by one*. Then open `adding.system`, which is counting **evolved**. It starts the same way (0, with two empty boxes), but now `3` is accepted, and `2`, `3`, `4` gives **9**.

**What is true underneath.**

| Part VII says | Phase 7.4 builds |
|---|---|
| The calculator is a system holding forces (G4) | a new `.system` file: the body it holds, and its combine force |
| The DNA says how a body grows (G8) | a `grows` section in the body's `.contact`: what cell it grows, and what it accepts (`one` for counting, `any` for adding) |
| A body can grow to any size; 2 is only a starting shape (G9, G10) | every size from 0 up is true; how many empty boxes show is presentation |
| Growth is not evolution; identity follows the DNA (G11, G12) | the system's hash never changes as it grows; the numbers typed are state, never stored |
| Counting is the witness (G14) | every count is checked by counting it out one step at a time |
| The answer is a cell at the lasso's point; the inputs stay (G15) | the count cell sits outside the body, at the lasso's arrow |
| Evolution keeps every old witness (Part V C14) | `adding` names `counting` as its parent, and every counting transcript gives the same answer on adding |
| A force is a lasso; things must be visually clear (G6, G7, G19) | the lasso is drawn and clickable: `force count` |

**What I decided (say if you disagree):**

- **7.4 is one system, in its own window; 7.5 puts systems into universes.** Universes, the lens as the view (G17), links that show their ends, room to grow inside a universe, and each level having its own look are **Phase 7.5**, planned after this phase's review. Doing both at once would be the largest phase so far. This way you see growth and the lasso first.
- **The lasso goes around the whole body**, not around single cells, because the force reaches every number in it. It never touches a cell, and its arrow ends at the count.
- **Counting accepts exactly `1`; adding accepts any whole number.** That is the whole difference between them: the evolution.
- **Typing is how she counts for now.** A tap or a button is Phase 8 (the keyboard) or the creator.
- **The three fixes from the 7.3 review open this phase** (P74-F1 … F3), as planned for Phase 8.

**What you do at the end** (about three minutes, §6.2): open counting, count to three, try a 3, then open adding and add 2, 3 and 4.

---

## 0. How Cursor Works This Plan

| | |
|---|---|
| **Plan file** | `D:\JoInn\docs\Plans\JoInn Phase 7.4 Implementation Plan.md` |
| **Code** | `D:\JoInn\joinn\`. No new crate. No new external dependency |
| **Standing rules** | `AGENTS.md` as updated by P74-01 (Appendix A) |
| **Unit of work** | **the phase**: every commit of §4 in order, each its own git commit whose message starts with its id (`P74-04: …`). Parts 0, F, A, B and C group the work; **they are not stops** |
| **End of the phase** | one stop: `cargo xtask stop-check --fresh`, the stop report (§0.2), commit, push, ledger line, then print `phase 7.4: stopped for review` and stop |
| **Who decides** | every decision is in §2. Cursor decides only module layout, function bodies, private representations, the lasso's exact geometry within §2.8's rules, the engine mechanism within §2.6's rules, and error wording where §2 gives none |
| **How each commit is checked** | Run 7.3–9 §3: the done-when, `cargo xtask check`, and the gates scoped to the files changed. Demos run `gate <phase> --item <n>` only |
| **Who checks** | Claude, at the phase stop, from a fresh clone of what was pushed. CI on the push (read once, never waited for). AJ runs the window (§6.2) at the stop |
| **Input** | Never move the system mouse, click, or type outside a window Cursor started (rule 67) |

### 0.1 The commit report

As Phase 7.3's §0.1. From P74-08 on, `cargo xtask check` also prints `Grow:` (`cargo xtask grow → <last line>`).

### 0.2 The stop report

Run 7.3–9 §4, the short form, at `docs/Findings/phase-7.4-stop.md`. Commit it as `P74-stop: stop report`.

### 0.3 Snags

As Phase 7.3's §0.3. Predictions that differ are pasted beside the plan's value and work continues.

---

## 1. Scope Fence

### In scope

- The three fixes from the 7.3 review (§2.12).
- **`grows`** in the `.contact` grammar (§2.2).
- **The `.system` file kind**: bodies, forces, lineage (§2.3).
- **Admission** of a system and its growing bodies (§2.4).
- **Growth**: a grown state from a system and its inputs, exact at every size (§2.5, §2.6).
- **Counting as the witness** (§2.7) and **evolution checked** (§2.9).
- **The lasso**: layout, strokes, owner, pick (§2.8).
- Four corpus files under `corpus/phase74/` and their hashes (§2.10).
- **The shell** opens a `.system` and grows it by typing (§2.11).
- `cargo xtask grow`, gate 7.4, findings `docs/Findings/phase-7.4-growth.md`, the freeze.

### Out of scope

| Not now | Why it is tempting |
|---|---|
| Systems inside universes; the lens as the view (G17); rule 33's change | Phase 7.5. Universes and their grammar don't change in this phase |
| Links that show their ends; each level's own look; room to grow in the universe grid | Phase 7.5 |
| The UI and saves systems of her app | Later; the window is the UI for now |
| Subtraction, multiplication, fractions | Further evolutions of counting; each is its own step on the path of truth |
| A tap or button to count | Phase 8 (keyboard) or Phase 9 (creator) |
| Changing `calculator.contact`, any corpus file, or any existing hash | T1. A contact without `grows` prints exactly as before |
| Changing the engine's speed (R108) | A truth-core change of its own |
| Changing Phase 6, 7.2 or 7.3 row layouts | Byte for byte, as 7.3 left them |

---

## 2. Decisions

All **DECIDED**.

### 2.1 Where things live

| Crate | Gains |
|---|---|
| `joinn-frame` | `TAG_SYSTEM = b"joinn.system.v1"` |
| `joinn-dna` | `grows` in `ContactCoding` (parse, print); the `.system` grammar (`System`, `SystemCoding`, parse, print) |
| `joinn-link` | `check_system`; `grow` (a grown state) and `lower_grown` (the body the engine runs); `count_witness`; `check_evolution` |
| `joinn-visual` | `layout_system`, the lasso, `SystemScene` (grow, apply growth, regrow), `FORCE_TAG` in the pick and cut |
| `joinn-gpu` | nothing new: the lasso is Stroke rows (§2.8) |
| `joinn-shell-desktop` | opens `.system`; Enter on the waiting box grows the body |
| `xtask` | `grow`, the `.system` mutation catalogue, gate 7.4, the 7.3 review fixes |

No new workspace edge. If `layers` refuses an edge this needs, that is a snag with the edge printed.

### 2.2 `grows`: how a body grows (body DNA)

A `.contact` may carry one `grows` section, after `forces` and before `budget`:

```
grows {
  cell:<64 hex> as numbers accepts one
}
```

| Part | Means |
|---|---|
| `cell:<hash>` | the cell each growth adds. In this phase it must be the input cell `c4a0a132…` (the calculator's `cli_a`) |
| `as numbers` | the **growth name**. Grown instances are `numbers.0`, `numbers.1`, … in growth order |
| `accepts one` | counting: an input is accepted only if its value is exactly `1` in ℤ |
| `accepts any` | adding: any value in ℤ |

- **Printed only when present.** A contact without `grows` prints byte for byte as before, so every existing contact hash is unchanged (a test asserts `calculator.contact`'s hash).
- A grown instance holds the `stdin` grant by being grown; `grants` names no grown instance.
- **A contact with `grows` has no `forces` section** (rule 91: forces belong to systems). Its parser refuses one: `contact: forces in a growing body; acceptance is forces in the system that holds it`.
- `accepts` takes exactly `one` or `any`. Anything else is refused naming the two words.
- The genome of a growing contact may be empty (both corpus contacts' genomes are).

### 2.3 The `.system` file kind

```
system {
  codex 1
  bodies {
    contact:<64 hex> as numbers
  }
  forces {
    combine ℤ 1 cell:<64 hex> as count on numbers
  }
  lineage none
}
---
regulatory {
  names   { count "Count" numbers "Numbers" }
  present { count "{0}" }
  waiting { numbers 1 }
}
```

- **`bodies`**: one or more contacts, each by hash and alias. In this phase a system holds exactly one body; two or more is refused: `system: <n> bodies; acceptance is one body in Phase 7.4`.
- **`forces`**: one or more forces. A force names its kind, frame, pinned response, response name, and `on <body alias>`. It reaches **every grown instance** of that body through its receptor (frame ℤ 1), at the input cell's out-port. A force names no instance: the lasso holds the body.
- **`lineage`**: `none` or `<64 hex>`, the parent system's hash (as a contact writes its lineage).
- **Regulatory** (never hashed): `names`, `present` as for a contact, and **`waiting`**: how many empty boxes the shell shows for the body (counting `1`, adding `2`). This is G9: the starting shape is presentation.
- Hashed as `TAG_SYSTEM` over the canonical print. Canonical print sorts bodies by alias and forces by response name.

### 2.4 Admission

`check_system(system, contacts, cells, frames) → Verdict<()>`, in this order, each refusal naming its acceptance:

1. Every bound contact is supplied and passes `check_contact`.
2. Every bound contact has a `grows` section (in this phase a system holds growing bodies only).
3. At least one force: `system: no force; acceptance is a force (a system is the bodies its forces reach)`.
4. Every force's `on` names a bound alias.
5. Receptor: the grown cell's out-port frame equals the force's frame.
6. The response is the register's response for that force and frame (rule 62, unchanged).
7. A `lineage` parent, if named, is supplied, and every one of its bodies' `grows` cells equals the child's (evolution may change what a body accepts, not what it grows).

### 2.5 Growth

`grow(system, contacts, cells, inputs: &[Value]) → Verdict<Grown>`.

- A **grown state** is the admitted system plus the ordered list of accepted inputs. It is never hashed, never printed into a coding region, and never written under `corpus/` (rule 88).
- Each input is checked against the body's `accepts` before it grows anything. Counting refuses a value other than `1`: `counting: <value> is not one; acceptance is 1 (counting grows by one)`. The words are the body's growth name and the system's display name; the acceptance is fixed.
- An accepted input grows instance `numbers.<n>` and fills it.
- **Every size is true** (G9, G10): with 0 inputs the response is combine's **identity** (0 in ℤ); with 1 it is that input; with n it is the combine of all n.
- **Identity follows the DNA** (G12): `hash_system` of the system is the same before and after any growth, and `Grown` has no hash of its own.

### 2.6 The engine

`lower_grown(grown, cells, frames) → Verdict<Body>` derives the body the engine runs, as `lower` does for a contact (rule 61: never written, described or drawn).

- **n ≥ 2:** the response's binary cell is applied as a left fold over the members in growth order. This is exact because the register's combine is order-blind and associative, which `order_blind` already checks (rule 63).
- **n = 1 and n = 0:** the response presents the input, or the identity. Cursor chooses the mechanism. It must not add a cell to the corpus, and it must not add a register row.
- A growth step is a **delta**: the engine state and the scene take the new input. **Regrow** (from the system and the whole input list) must equal the deltas, byte for byte (rule 58).
- No float, no wall clock, no unseeded order (rules 3, 4, 69).

### 2.7 Counting is the witness (G14)

`count_witness(inputs) → Value` computes the response **by counting**: start at 0, and for each input `v` take `|v|` steps of +1 (or −1 if `v < 0`). It uses no sum cell, no `+` on values, and no register; only the next and previous integer.

- After **every** growth step, the engine's response must equal `count_witness` of the inputs so far. A difference is a truth violation, printed with the inputs.
- `count_witness` refuses an input with `|v| > 1 000 000` (`witness: too many steps; acceptance is |v| ≤ 1000000`). Every transcript in this phase is far below it. It is a witness, not the engine (Part VII §5.2).

### 2.8 The lasso

A force is drawn as a **lasso** (G6, G7). Rule 64 changes to say so (Appendix A).

**Layout.** `layout_system(system, contacts, cells, grown, waiting) → Verdict<SystemLayout>`, integers in layout units:

- The body's cells sit in rows of 6, filled cells in growth order, then the waiting boxes. Each grown cell and each waiting box has the input cell's size from `layout_contact`. The body's surface wraps them with Phase 6's margins.
- The **response cell** sits to the right of the body, vertically centred on it, outside the lasso.
- The **lasso** is a closed outline around the body's surface, at least 2 units outside it, with rounded corners made of straight strokes (no arc; rule 70). A **neck** leaves the outline's right side at the body's vertical centre and ends at the response's in-port side, with an **arrowhead** (two strokes) pointing into the response.

**Rules the geometry must meet** (checked by `cargo xtask grow`, exactly, in layout units):

1. No lasso stroke (with its half-width) meets any cell rectangle or the body's surface.
2. Every grown cell and every waiting box lies inside the outline.
3. The response cell lies outside the outline, and the arrowhead's tip touches its in-port side.
4. After a growth step, rules 1–3 hold again (the lasso stretches; it is never left behind).

**Drawing.** The lasso is **Stroke rows** (Phase 7.2's table and pipeline, unchanged layout), style **15 force**, half-width ½ unit, colour chosen by Cursor to differ from every other style. Drawing order: frames → link segments → **lasso** → bodies, cells, ports → text. The lasso lies under nothing it surrounds.

**Owner.** `FORCE_TAG = 0x6000_0000` in blue. A lasso pixel's ID is `[force slot + 1, 0, FORCE_TAG, generation]`. Owners print `force count`. Picks print `pick …: force count (cpu)` then the GPU confirm line.

### 2.9 Evolution, checked

`check_evolution(parent, child, contacts, cells) → Verdict<()>`: the child names the parent in `lineage`, and:

1. **Every old witness holds.** A parent's **witness** is a §3 transcript of the parent that it accepts whole. On each, the child accepts every input and gives the same response after every step. (A transcript the parent refuses part of is not a witness; on its accepted part the child must still agree.)
2. **It gains something.** Some input the parent refuses is accepted by the child: `evolution: adding accepts nothing counting refuses; acceptance is a new ability (otherwise it is an edit)`.

For the corpus, `adding` evolves `counting`: counting accepts `one`, adding accepts `any`, and both grow the same cell under the same force.

### 2.10 The corpus

Four new files under `corpus/phase74/`. Their hashes are appended to `corpus/hashes.txt` (new lines only; no existing line changes). T1 treats these exactly as Phase 8 treats its witness files: new files and appended lines are not changes.

| File | Holds |
|---|---|
| `counting.contact` | empty genome, `grows { cell:c4a0a132… as numbers accepts one }`, no forces, `lineage none` |
| `counting.system` | binds `counting.contact` as `numbers`; `combine ℤ 1 cell:6b3271… as count on numbers`; `lineage none`; `waiting { numbers 1 }` |
| `adding.contact` | as counting, `accepts any`, `lineage <counting.contact's hash>` (the contact grammar's existing lineage form) |
| `adding.system` | binds `adding.contact`; the same force; `lineage <counting.system's hash>`; `waiting { numbers 2 }` |

**Predicted:** `corpus verify` → **`48 hash(es) match`** (44 + 4).

### 2.11 What the shell shows

`joinn-desktop corpus/phase74/counting.system` (or `adding.system`):

- The system's scene: body, filled cells, waiting boxes, the lasso, the count.
- **Clicking a waiting box** selects it (as an input cell is selected today). Typing and Enter **grow the body**: `intent numbers.<n> "1"`, then `grew numbers.<n>; count 1` and the tick line. A refusal prints `refused: counting: 3 is not one; acceptance is 1 (counting grows by one)`, grows nothing, and keeps the selection.
- Clicking the lasso prints `pick …: force count (cpu)` and the GPU confirm line.
- No new keys. The camera is Phase 7.2's.

### 2.12 Carried from Phase 7.3 (the 7.3 review)

**F1 · One gate registry.** The cross-gate checks (`check_distinct_opposition` (rule 41), `g3_artifacts` (gate 3 item 8), `no_two_gate_items_share_a_check_or_control`, `only_gate_seven_names_a_contact_artifact`) read their tables from `gate_table`, so every gate that exists is covered with no list to edit. Gate 7.3 is covered at once (the review found it in none of the first three).

- The shared-function test stops comparing addresses. Each `GateItem` gains a `check_name: &'static str` and a `control_name: &'static str` (the function's own name); the test compares names. The four 2.1/2.2 rows that check one fact (`agree_cached().is_ok()`) are listed as **one legacy exception**, with that sentence.
- With that, xtask goes back to the workspace profile: delete `[profile.dev.package.xtask] opt-level = 0`.
- `stop-check`'s fail filter matches `FAILED`, `failed,` and `: fail` instead of the bare word `fail`.

**F2 · Standing plants for truths by construction.**

- `cargo xtask links` plants three faults on every run, and each must be refused: (a) one system gutter line moved 4 units into its bodies (a crossing), (b) a second touch on a folded node, (c) a mid-path arrowhead on an unordered link. Last line: `links: 3 universes, crossings 0, each folded node touched once; planted crossing, double touch, false arrow: refused (ok)`.
- Gate grading prints which half of each control answered on the opposed subject: `item <n> control answered by admission` or `… by check`. A gate whose controls all answer by admission names its standing plants in its findings.
- **The rule for Phases 7.4, 8 and 9:** a truth that no admitted artifact can break gets a standing plant in its command (rule 93).

**F3 · Canonical order at admission.** An admitted universe's coding is put in print order (links by id; members of unordered links sorted) before anything lays it out. A test proves `parse(print(u))` and `u` give byte-identical Route and Segment rows and identical `pick` owner lines, for the grove and both Phase 5 universes. No corpus file changes and no hash changes (the hash already uses this order). A grove leg's `member <i>` is then the same from memory and from `target/grove.universe`.

### 2.13 Invariants

| # | Invariant | Commit |
|---|---|---|
| **V160** | A body grows by its DNA, and every size is true: identity at 0, the input at 1, the combine at n | P74-06 |
| **V161** | Counting is the witness: after every growth step the response equals `count_witness` | P74-07 |
| **V162** | Growth is not evolution: the system's hash is unchanged by growth; grown state is never hashed or stored | P74-06 |
| **V163** | Evolution keeps every old witness and gains an ability | P74-07 |
| **V164** | A force is seen as a lasso: it surrounds what it reaches, never meets a cell, and points to its response; CPU and GPU name its pixels alike | P74-11 |

---

## 3. What runs

| Command | Prints | Exit 1 when |
|---|---|---|
| `cargo xtask grow` | per system and transcript, one line per step: `grow <system> <step>: +<v> → cells <n>, count <c>, witness <c>, lasso ok, hash <first 12 hex>`; refusals as `grow <system> <step>: +<v> refused: <reason>`; then `evolve counting → adding: <k> witnesses hold; adding accepts 3, counting refuses it`; then the plants; last line `grow: 2 systems, every size true, counting witnesses every step; evolution holds; planted fold, lasso, hash: refused (ok)` | any difference, any refused plant not refused |
| `cargo xtask grow --measure` | per system: `grow µs <t> per step at n = 0, 6, 24, 96` (release advised; never decides) | — |
| `cargo xtask pick` | every earlier line unchanged; plus per adapter, per system at sizes 0, 3 and 7: `system <name> n <k>: … disagree 0, owners <n> (cut allows <n>), force owners 1 (cut allows 1)` | as before, or a force owner differs |
| `cargo xtask regrow` | every earlier line unchanged; plus `system <name>: deltas equal regrow at n = 0 … 7` | as before |
| CI | the `zoom` step gains `cargo xtask grow` | the step fails |

**Witness transcripts** (fixed, seeded nowhere because they are written here):

| System | Transcript | Predicted |
|---|---|---|
| counting | (none) | count **0**, cells 0 |
| counting | `1 1 1` | counts **1, 2, 3** |
| counting | `1 1 3` | **1, 2**, then `3` refused; cells stay 2, count 2 |
| counting | `1` × 12 | count **12** |
| adding | (none) | count **0** |
| adding | `2 3 4` | counts **2, 5, 9** |
| adding | `5 -2 0 7` | counts **5, 3, 3, 10** |
| adding | every counting transcript above | the same counts as counting, except `1 1 3` → **1, 2, 5** |

**Predicted:** the hash printed on every step of a system is that system's hash in `corpus/hashes.txt`, unchanged. `evolve counting → adding: 3 witnesses hold` (the three counting transcripts with no refusal).

**Plants** (standing, every run; F2's rule): (a) the fold drops its last member: the witness refuses at the first n ≥ 2; (b) the lasso drawn 2 units inward: lasso rule 1 refuses; (c) a hash computed over the grown state: V162 refuses at the first growth.

---

## 4. The Commits

### Part 0: the plan

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P74-01** | **Plan, rules, docs** | Commit this plan, `docs/Plans/JoInn Run 7.3-9.md` (Draft 0.3), `docs/Theory/JoInn Systems, Forces and Growth.md` (Part VII, already on disk), `docs/Findings/phase-7.3-review.md` (already on disk). `AGENTS.md` per Appendix A. Appendix B. No code | `git show --stat HEAD` lists this plan, the run plan, Part VII, the 7.3 review, `AGENTS.md`, the backlog, `decisions.md`, the roadmap. `Check: docs only` |

### Part F: carried from Phase 7.3 (§2.12)

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P74-F1** | **One gate registry** | §2.12 F1 | `cargo xtask check` passes with xtask at the workspace profile (paste the `Cargo.toml` diff). A test asserts the cross-gate checks' tables equal `gate_table`'s, gate 7.3 included. **Shown then reverted:** gate 7.3 item 3 given gate 5.1's `(artifact, opposes)` pair: rule 41's check refuses it naming both items (paste). `gate all` (shared harness) → every line as `gates.lock` |
| **P74-F2** | **Standing plants** | §2.12 F2 | `cargo xtask links` last line as §2.12 (paste). `cargo xtask gate 7.3` prints the three `answered by` lines (paste). **Shown then reverted:** plant (a) not planted: `links` exits 1 naming the missing plant |
| **P74-F3** | **Canonical order at admission** | §2.12 F3 | The round-trip test passes (paste its name and `ok`). `cargo xtask pick` and `regrow` print every earlier line unchanged (`Compare-Object` prints nothing, paste). `gate all` → as `gates.lock` |

### Part A: the truth (no picture)

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P74-02** | **The mutation catalogue first** (rule 52) | `Mutation` gains, at least: `Accepts(&str, Accept)`, `DropForce(&str)`, `DropLineage(&str)`, `ForceOn(&str, &str)`, each with its refusal words for a subject of the wrong kind. The `.system` subject kind and its parse in the harness | Tests: each mutation applies to a hand-written system or contact and is refused on a universe naming the kind |
| **P74-03** | **`grows`** | §2.2 in `joinn-dna` | Tests: parse and print round-trip; `accepts` other than `one`/`any` refused; forces in a growing contact refused; **`calculator.contact`'s hash unchanged** (asserted against `hashes.txt`); `corpus verify` → `44 hash(es) match` |
| **P74-04** | **`.system`** | §2.3; `TAG_SYSTEM` | Tests: parse, print, hash; canonical order; two bodies refused; regulatory never hashed (renaming `count` keeps the hash) |
| **P74-05** | **Admission** | §2.4 | One test per refusal of §2.4, each asserting its words |
| **P74-06** | **Growth and the engine** | §2.5, §2.6; V160, V162 | Tests: sizes 0, 1, 2, 3, 12 on hand-written counting and adding systems give 0, the input, and the combine; counting refuses `3` with §2.5's words; `hash_system` unchanged by growth; deltas equal regrow at every size |
| **P74-07** | **The witness and evolution** | §2.7, §2.9; V161, V163 | Tests: `count_witness` of every §3 transcript; the engine agrees at every step; `check_evolution(counting, adding)` ok; `check_evolution(counting, counting')` (a counting with lineage and nothing new) refused with §2.9's words |
| **P74-08** | **The corpus and `grow`** | §2.10; `cargo xtask grow` (without the lasso lines until P74-09: print `lasso —`); `check` gains `Grow:` | The four files committed, hashes appended. `corpus verify` → **48** (paste). `cargo xtask grow` printed whole (paste). `git diff <P74-01>..HEAD -- joinn/corpus` lists only the four files and `hashes.txt` (paste) |

### Part B: the picture

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P74-09** | **The system layout and the lasso** | §2.8 layout and its four rules | `grow` prints `lasso ok` on every step (paste). Tests: the four rules at sizes 0 … 13 (both row wraps); a lasso stroke moved inward is refused by rule 1 |
| **P74-10** | **The system scene** | `SystemScene`: grow, apply growth as a delta, regrow; Stroke rows for the lasso; style 15 | Tests: a growth step writes only the rows it changes; tables after 7 deltas equal regrow; Phase 6, 7.2, 7.3 row layouts unchanged. `cargo xtask regrow` gains the `system …` lines (paste) |
| **P74-11** | **Picking the lasso** | §2.8 owner; `FORCE_TAG` in `cpu_pick`, the cut and the shader's owner path; `pick` gains the system lines; V164 | Whole `pick` output pasted; every line `disagree 0`, force owners equal. `gate 6`, `gate 7`, `gate 7.2`, `gate 7.3` each full |
| **P74-12** | **The shell grows a system** | §2.11 | A session test (no window) on `counting.system`: select the waiting box, type `1`, Enter, three times → `count 3`; type `3`, Enter → §2.5's refusal and no rows written; a pick on the lasso prints `force count`. The same on `adding.system` with `2 3 4` → `count 9`. Cursor runs the window once and pastes the lines, or writes `no display` |

### Part C: measurement, gate 7.4, freeze

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P74-13** | **Findings** | `grow --measure`; `docs/Findings/phase-7.4-growth.md`: every command's output from P74-06 on; *Predictions* marked `as predicted` / `differs`; **The adversary** (§7) answered with the measured µs; **What 7.5 needs** (observed) | Every prediction marked |
| **P74-14** | **Gate 7.4** | §6 items; `phase 7.4` after `phase 7.3` in `PHASE_LABELS`; lock row | `cargo xtask gate all` exits 0, prints `phase 7.4: 3/3`, and three `answered by` lines. **Shown then reverted** with `gate 7.4 --item <n>` only, pasting each failure: (a) give item 2 item 1's `opposes`; (b) counting's `accepts` read as `any` in `grow` (item 1 must fail); (c) `check_evolution`'s gain test removed (item 2 must fail on its control); (d) the lasso drawn after bodies (item 3 must fail on a cell pixel owned by the force) |
| **P74-15** | **Docs and freeze** | README; `Guides/03-where-we-are.md` (a system grows; counting witnesses; adding evolved from counting; the lasso); glossary: *system file, grows, growth name, accepts, grown state, waiting box, count witness, evolution check, lasso, neck*; `decisions.md` V160–V164 with status; roadmap 7.4 note points at the findings | Tier 1. `corpus verify` 48; `git diff --stat <P74-01>..HEAD -- joinn/corpus` lists only the four files and `hashes.txt` |
| — | **The phase stop** | `cargo xtask stop-check --fresh` (prints phases 0 … 7.4, exits 0); `phase-7.4-stop.md` (Run 7.3–9 §4); commit `P74-stop`, push, ledger; print `phase 7.4: stopped for review`; stop | — |

Dependencies: in order. P74-F2 and F3 depend on P74-F1; P74-03 … 05 on P74-02; P74-06 on P74-05; P74-07 on P74-06; P74-08 on P74-07; P74-09 on P74-08; P74-10 on P74-09; P74-11 and P74-12 on P74-10; P74-14 on P74-08, P74-11, P74-12.

**If something has to be cut for time:** cut `--measure` first, then the window run in P74-12 (the session test stays). Never cut V160–V164, P74-F1, P74-F2 or P74-F3.

---

## 5. Test Strategy

As Phase 7.3's §5. Growth tests assert exact counts, cell counts and refusal words, never "it grew". Picture tests compare bytes. No test asserts only that a lasso "exists"; each asserts the four rules or owners.

---

## 6. Exit Gate 7.4

Three items in `xtask/src/fns/gate_seven_four_items.rs`, built like gate 7.3's. Every GPU part runs on every adapter (rule 60). **Items 1 and 2 are answered by their checks, not by admission:** their mutants are admitted, and the control finds the wrong behaviour.

| # | Item | `control_artifact` | `opposes` | Control answers `true` when |
|---|---|---|---|---|
| 1 | **A body grows by its DNA, and every size is true** | `corpus/phase74/counting.system` | `Accepts("numbers", Any)` | the subject is refused at admission, or growing it by `1 1 3` accepts the `3`, or any response differs from `count_witness` |
| 2 | **Evolution keeps every old witness, and gains** | `corpus/phase74/adding.system` | `Accepts("numbers", One)` | the subject is refused at admission, or `check_evolution(counting, subject)` refuses |
| 3 | **A force is seen as a lasso; growth is not identity** | `corpus/phase74/counting.system` | `DropForce("count")` | the subject is refused at admission, or a lasso rule (§2.8) fails at any size 0 … 13, or the hash changes with growth |

Item 3's mutant is refused at admission (a system needs a force), so it is answered by admission. Its truths are by construction and are opposed by `grow`'s standing plants (b) and (c) (§3), as F2's rule requires.

**Checks:**

1. Every §3 transcript on both systems gives its predicted counts and refusals, and `count_witness` agrees at every step. Counting at 0 shows 0.
2. `check_evolution(counting, adding)` holds with 3 witnesses; a counting with lineage and nothing new is refused with §2.9's words.
3. On every adapter, at sizes 0, 3 and 7 of both systems: the lasso's four rules hold; CPU and GPU agree on every pixel (`disagree 0`); no cell, port or body pixel is owned by the force; a pick on a lasso pixel names `force count`; and `hash_system` is the same at every size.

Rule 41 holds through `gate_table` (F1): the three pairs are new. If the uniqueness test refuses one, Cursor uses the next unused mutation of the same artifact from the catalogue, prints it, and records a snag.

### 6.1 Conditions for opening Phase 7.5

1. `stop-check --fresh` exits 0 with `phase 7.4: 3/3` and every earlier line as `gates.lock`; no tripwire fired.
2. `phase-7.4-growth.md` exists with every prediction marked and the adversary answered.
3. Claude's review of the phase stop lists no open snag, or AJ has chosen to carry each one forward.
4. AJ has run the window (§6.2), or each thing that looked wrong is a snag Claude has settled.

### 6.2 AJ's window check (at the phase stop, about three minutes)

```
cd D:\JoInn\joinn
cargo run --release -p joinn-shell-desktop -- corpus/phase74/counting.system
```

1. You see one empty box inside a lasso, and the lasso's arrow pointing at a count of **0**.
2. Click the empty box, type `1`, press Enter. The box fills, a new empty box appears inside the lasso, and the count is **1**. Do it twice more: **3**.
3. Click the new empty box, type `3`, press Enter. The terminal says counting grows by one. Nothing changes.
4. Click the lasso. The terminal prints `force count`.
5. Close it, and open adding:

```
cargo run --release -p joinn-shell-desktop -- corpus/phase74/adding.system
```

6. Two empty boxes and a count of **0**. Type `2`, `3` and `4` (Enter after each). The count is **9**, and the lasso holds all three.
7. Close the window. Tell Claude what you saw, and whether the lasso reads as a force at a glance.

---

## 7. Risks

| Risk | What to do |
|---|---|
| **The adversary: re-lowering on every growth.** If each step re-derives and replays the whole body, growth costs grow with n | Measured in P74-13 at n = 0, 6, 24, 96. A slow number is a finding, not a workaround |
| The input cell's layout size makes a wide row of 6 too wide for the window at frame | Presentation: the camera frames the system. Never shrink a cell |
| The n = 0 and n = 1 responses need a mechanism the engine doesn't have | §2.6 lets Cursor choose, within its limits. If neither limit can be kept, a snag with both tries printed |
| The lasso, as straight strokes, looks crude | AJ judges at the window. Its exactness comes first (rule 70) |
| `layers` refuses an edge | A snag with the edge printed; never move code to dodge it (rule 42) |
| A predicted count differs | Paste both; exact counts decide at the review |

---

## 8. Open Items

| ID | Topic | Question |
|---|---|---|
| **R110a** | Carry, the force | From Part VII: is *bears on* a link or a force? Not needed until the beam |
| **R111** | What is consumed | From Part VII: if combine keeps its inputs, is "consumed" a property of carry? |
| **R115** | Removing a number | Growth only adds. Can a body shrink (she erases a number), and is that growth backwards, or an edit? |
| **R116** | Where state lives | A grown state is the input list. When the saves system exists, it stores that list. Is the list itself testimony with lineage (her teacher sees *what* she counted and *when*, R114)? |
| **R117** | Systems of several bodies | A system holds one body here. With two (the beam and its column), does one lasso hold both, or one lasso per body? |

---

## Appendix A · `AGENTS.md` changes

Replace the header paragraph with:

```markdown
# JoInn — standing rules

This repo is JoInn. Phase 7.4 is her counting app, part 1: one counting system
that grows by its DNA, witnessed by counting, evolved into adding, with its force
drawn as a lasso. Its one idea is A BODY GROWS BY ITS DNA, AND EVERY SIZE IS
TRUE. The build plan is docs/Plans/JoInn Phase 7.4 Implementation Plan.md, run
inside docs/Plans/JoInn Run 7.3-9.md. Work the phase's commits in order, one git
commit per numbered step, check each by tier, and stop once at the phase's end.
Never stop to ask; follow the plan's Snags section instead.

New grammar in this phase: `grows` in .contact, and the .system file kind. Their
mutants ship first (rule 52). Universes do not change (Phase 7.5).
```

Replace rule 64 with:

```markdown
64. A FORCE IS SEEN AS A LASSO. It owns the pixels of its lasso and nothing
    else: no cell, port or body pixel. Interior ports are not drawn; a contact
    picture has no link rows. A link and a force never look alike (Part VII G19).
```

Keep every other rule. Append (rules 77 … 86 stay reserved for Phases 8 and 9):

```markdown
87. A BODY GROWS BY ITS DNA, AND EVERY SIZE IS TRUE. A growing body's `grows`
    section says what it grows and what it accepts. Size 0 is the force's
    identity, size 1 the input, size n the combine.
88. GROWTH IS NOT EVOLUTION. A system's hash covers its DNA and forces, never
    what grew. A grown state is never hashed and never written under corpus/.
89. EVOLUTION KEEPS EVERY OLD WITNESS, AND GAINS. A child names its parent; every
    parent transcript gives the same answers on the child; the child accepts
    something the parent refuses.
90. COUNTING IS THE WITNESS. After every growth step the response equals the
    count taken one step at a time. A difference is a truth violation.
91. FORCES BELONG TO SYSTEMS. A growing contact has no forces section; its
    system holds them.
92. A SYSTEM IS THE BODIES ITS FORCES REACH. A system with no force is refused.
93. A TRUTH BY CONSTRUCTION GETS A STANDING PLANT. When no admitted artifact can
    break a truth, its command plants the fault on every run and must refuse it.
```

## Appendix B · Documents

- **Backlog** (`docs/Theory/JoInn Research Backlog.md`): after R108, `> **6 Oct 2026 · Part VII and Phase 7.4.** R109–R117 come from *Systems, Forces and Growth* (Part VII) and the 7.4 plan.` then R109 (answered: G17), R110 (answered: G18), R110a, R111–R114 (from Part VII §12, open), R115–R117 (from §8, open).
- **`docs/Findings/decisions.md`**: rows G1–G19 (Part VII, `decided`); R109–R117 as in the backlog; V160–V164 (`open`; P74-15 sets them).
- **Roadmap**: under Phase 7.3, add `#### Phase 7.4 · Her counting app, part 1` with: `> **6 Oct 2026.** Planned in Plans/JoInn Phase 7.4 Implementation Plan.md after Part VII: a counting system grows by its DNA, counting witnesses every step, adding evolves from counting, and the force is drawn as a lasso. Phase 7.5 (systems in universes, the lens as the view, links that show their ends, each level's own look) follows; Phases 8 and 9 keep their numbers.`

---

*JoInn Phase 7.4 Implementation Plan, Draft 0.1 (6 Oct 2026). Written after Part VII (Systems, Forces and Growth, Draft 0.2), which AJ accepted whole the same day. AJ decided: the calculator is a system holding forces; a force is a directed hyperedge drawn as a lasso; a system starts as a seed whose DNA grows its bodies; all basic math is counting; links transfer data while forces stimulate; a lens is the view. Claude decided, open to AJ's veto: one system in its own window now, universes in 7.5; the lasso around the whole body; counting accepts exactly one; typing to count; the 7.3 review's fixes first.*
