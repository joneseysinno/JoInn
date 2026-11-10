# JoInn Phase D Implementation Plan

**One relationship: a link and a force are the same connector, chosen and pointed differently, and still drawn apart**

Draft 0.1 · 30 Oct 2026 · written from the [Hypernode Refactor Plan](JoInn%20Hypernode%20Refactor%20Plan.md) (Phase D) for AJ's review

> **Status: a proposal for AJ's review. Nothing in this plan is built yet.** Every decision below is tagged **DECIDED** (AJ already decided it in the refactor plan or in earlier theory), **PROPOSED** (this draft's reading, open to AJ's veto) or **OPEN** (left for research). Phase D opens only after Phase C's stop (Phase C plan §6.1); it reads the contact picture's lasso that Phase C builds. When AJ has confirmed or changed each PROPOSED item, the plan becomes Draft 1.0 and runs as one unit with one stop; a step that can't be done as written follows §0.3 (Snags).

**Thesis.** A link and a force are both relationships, one connector that differs only in how it chooses its members and how it points, and both pickers name either kind's pixels alike while the screen keeps the lasso and the routed link apart.

---

## For AJ: this plan in plain English

**What Phase D is.** JoInn has two kinds of connector today, built in different phases and kept in different places:

- A **link** joins bodies in a universe. You write its members by name, and you may mark tails and heads and declare an order. The window routes it along the streets between bodies. A click prints `link e0` or `link e0 member 1`.
- A **force** acts on cells. In a contact you name its members; in a system it reaches every number the body grows. It always points at its result. The window draws it as a lasso. A click prints `force count` (and, after Phase C, `force sum`).

They are the same idea: a relationship that can touch many things at once. Part VII already said a force is a directed hyperedge (G6). Phase D makes that true in one place in the code, without changing a file format, a hash, or anything you see.

**What changes.**

| Layer | After Phase D |
|---|---|
| What a relationship *is* | **One type** in `joinn-link`: a name, a form (link or force), how its members are chosen (named ports, or a whole body's grown cells), its order, and its head (marked members, or a response) |
| How a click names it | **One function** prints every relationship owner. The words stay exactly as today: `link <id>`, `link <id> member <i>`, `force <name>` |
| The corpus | One new witness file lists every relationship in the corpus, one line each, so the "one view" can be checked byte for byte |
| The grammar, the engine, the drawing | **Unchanged.** Links still deliver between bodies and forces still compile with `lower` (G18: links transfer data, forces stimulate). Links still route in gutters; forces are still lassos (G19: a link and a force never look alike) |

**What waves need from this.** A wave (Phase F) is a relationship whose members are *derived*: every receptor that accepts the signal. Phase D's member choice already has two kinds, and one of them, the system force's "every grown cell of this body", is already derived. Phase F adds the third kind. Phase D writes the place down and builds nothing for it.

**What I decided (say if you disagree):**

- **Unify the description and the naming, not the files.** The grammar stays three sections in three file kinds. Rewriting them would move hashes and is a rename-sized job.
- **Keep every owner word.** `link` and `force` are what clicks print and what the gates have pasted for months. The ID tags stay apart too, because the tag is how both pickers know which kind they hit.
- **One gate item, gate-8 style.** Its subject is the new witness table. Its mutant is the table a careless unification would print, with every force named `link`, and it must be refused.
- **R110 stays where it is.** Whether a link is "a lasso of kind carry" is not decided by this phase (§2.8).

**What you do at the end** (about three minutes, §6.2): click a link in one window and a lasso in another; read the relationship table.

---

## 0. How Cursor Works This Plan

| | |
|---|---|
| **Plan file** | `D:\JoInn\docs\Plans\JoInn Phase D Implementation Plan.md` |
| **Parent plan** | `docs/Plans/JoInn Hypernode Refactor Plan.md`, Phase D |
| **Code** | `D:\JoInn\joinn\`. No new crate, no new external dependency, no identifier renamed, no grammar changed |
| **Standing rules** | `AGENTS.md` as updated by PD-01 and PD-07 (Appendix A) |
| **Unit of work** | the phase: every commit of §4 in order, each its own git commit whose message starts with its id (`PD-05: …`). **One stop**, at the end |
| **End of the phase** | `cargo xtask stop-check --fresh`, the stop report (§0.2), commit, push, ledger line, print `phase D: stopped for review`, stop |
| **Who decides** | every decision is in §2. Cursor decides module layout, private representations, function bodies, and error wording where §2 gives none |
| **How each commit is checked** | Run 7.3–9 §3 (AGENTS.md S1–S4). `joinn-link` is a foundation crate: `gate all`. Any existing file in `joinn-visual`: gates 6, 7, 7.2, 7.3, 7.4, 8, B and C |
| **Who checks** | Claude, at the phase stop, from a fresh clone. CI on the push, read once. AJ at the stop (§6.2) |
| **Input** | Never move the system mouse, click, or type outside a window Cursor started (rule 67) |

**Numbering used by this plan.** Highest at writing: **V173**, **R133**, rule **93**. Blocks in order: Phase B V174–V176, R134–R136, rule 94; Phase C V177–V180, R137–R139, rule 95. **Phase D: V181–V184, R140–R143, rule 96.**

### 0.1 The commit report

Phase B's §0.1 block, every value copied from the terminal. From PD-04 on, `cargo xtask check` also prints `Relationships:` (`cargo xtask relationships → <last line>`).

### 0.2 The stop report

Run 7.3–9 §4, the short form, at `docs/Findings/phase-D-stop.md`. Commit it as `PD-stop: stop report`.

### 0.3 Snags

As Phase 7.4's §0.3. Write the snag in the next commit report with the exact output, take the stated fallback, keep going. Never change an owner string to make a comparison pass; a string that must change is a dated finding first.

---

## 1. What Already Holds, the Inventory, and the Scope Fence

### 1.1 What already holds

| Fact | Where it is checked |
|---|---|
| A link touches and never crosses; routes live in gutters; order is drawn only when declared; a folded node is touched once | V148–V151; gate 7.3 items 1–3; `cargo xtask links` → `links: … crossings 0, each folded node touched once; planted crossing, double touch, false arrow: refused (ok)` |
| Two pickers name every link pixel: `link <id>` and `link <id> member <i>` | V152; `pick`'s grove lines `… link owners <n> (cut allows <n>)` |
| A force is a lasso in a system; two pickers name it `force count` | V164; gate 7.4 item 3; `pick`'s system lines `… force owners <f> (cut allows <f>)` |
| A force is a lasso in the contact calculator; two pickers name it `force sum`; a contact picture has no wire or link owner | V177, V178 (Phase C); gate C item 1 |
| A force is a directed hyperedge (G6); drawn as a lasso hugging its tails, cinched to an arrow at its head (G7); links transfer data while forces stimulate (G18); a link and a force never look alike (G19) | Part VII; `decisions.md` G6, G7, G18, G19 (`decided`) |

### 1.2 Inventory: the two kinds today

| Layer | Link | Force in a contact | Force in a system |
|---|---|---|---|
| **Text** | `.universe`: `link e0 order ordered { calc.sum@2 tail  units.scale@0 head }`; `grants { e0: units }` | `.contact`: `combine ℤ 1 cell:6b3271… as sum from cli_a@1, cli_b@1` | `.system`: `combine ℤ 1 cell:6b3271… as count on numbers` |
| **Rust type** | `joinn_link::Link { id, order: Order, members: Vec<joinn_link::Member> }`; `Member { body, instance, port, mark: Mark }`; `Order { None, Ordered }`; `Mark { None, Tail, Head }` (`joinn-link/src/universe.rs`) | `joinn_dna::Force { kind: ForceKind, frame: FrameRef, response: Hash, name, members: Vec<joinn_dna::Member> }`; `Member { instance, port }`; `ForceKind { Combine }` (`joinn-dna/src/body/contact.rs`) | `joinn_dna::SystemForce { kind, frame, response, name, on }` (`joinn-dna/src/body/system.rs`) |
| **Members chosen** | named ports on body surfaces (rule 31); canonical print sorts unordered members | named out-ports of genome instances; order never hashed (V127) | **derived**: every grown instance of the body `on` names, at the grown cell's out-port (plan 7.4 §2.3) |
| **Direction** | tails are out-ports, heads in-ports, frames equal (rule 36); a spine only when `order ordered` (rule 76) | members → the response, whose in-ports `0 … k−1` they feed in canonical order (`lower`) | members → the response (`lower_grown`) |
| **Admission** | `assemble_universe`, `check_link_types`, `check_law4`, `bind`, `check_capability` | `check_contact` | `check_system` |
| **Engine** | `UniverseState` delivers across links; the far side gets a `LinkRefusal` (rule 39); incidence as `Csr` / `CsrMember` (`csr_from_universe`, never hashed) | compiled by `lower` into wires inside one body | compiled by `lower_grown` |
| **Rows** | `RouteRow`, `SegmentRow` (laid out by `routes` and `form_of`, with the leaves `route_link.rs`, `knot.rs` and `stub.rs` in `joinn-visual/src/routes/`); styles `STYLE_REGION` 12, `STYLE_HUB` 13, `STYLE_SPINE` 14 | Stroke rows, `STYLE_FORCE` 15 (Phase C) | Stroke rows, `STYLE_FORCE` 15 (`system_scene/write_rows.rs`) |
| **GPU** | `segment.wgsl` writes `[link + 1, member, LINK_TAG \| form, generation]` | `curve.wgsl`, `KIND_LASSO` | `curve.wgsl`, `KIND_LASSO` |
| **ID tag** | `LINK_TAG = 0x5000_0000`, form 0 … 3 in the low bits; green 0 for trunk, knot, spine or region, member + 1 for a leg or stub | `FORCE_TAG = 0x6000_0000` \| force slot (Phase C) | `[1, 0, FORCE_TAG, generation]` |
| **Printer** | `UniverseScene::print_id` → `link <id>`, `link <id> member <i>` | `print_owner` on `Owner::Force` → `force <name>` (Phase C) | `SystemScene::print_id` → `force <name>` |
| **Mutations** | `DropLink`, `FlipMark`, `ShiftPort`, `CopyMember`, `WireAcross`, `DropGrant`, `RenameLink`, `RenameAlias` | `DropForce`, `SwapResponse`, `ShiftMember`, `DropMember`, `RenameAlias` | `DropForce`, `ForceOn`, `Accepts`, `DropLineage` |

Not a relationship, and staying apart: a **wire** of the compiled form (`LinkRow` of `WIRE_KIND` in a body scene, tag `WIRE_TAG`, printed `body wire <src> -> <dst>`). It is a delivery `lower` derives (rule 61), not something a person builds. The code name collision (`tables.link` holds wires) is left for the rename phase.

No shared type or trait exists today between `Link`, `Force` and `SystemForce`.

### 1.3 In scope

- The relationship view: one type in `joinn-link` and three read-only functions over the existing codings (§2.2).
- One owner vocabulary: one function prints every relationship owner; strings unchanged (§2.4).
- `cargo xtask relationships` and its witness `corpus/phaseD/relationships.txt` (§2.5).
- `pick` shows both pickers name both kinds alike and the kinds stay apart, with standing plants (§2.6).
- Rule 96 and rule 76 widened (Appendix A). Gate D, one item. Findings `docs/Findings/phase-D-relationships.md`. Guides.

### 1.4 Out of scope

| Not now | Why it is tempting |
|---|---|
| A unified grammar (`relationships { … }`) or any file format change | Moves every universe, contact and system hash (rule 6); R140 |
| Unifying the engine (links lowered, or forces delivered across bodies) | G18: links transfer data, forces stimulate. A link with a response law is R142 |
| Drawing a link as a lasso, or a force as a route | G19, rule 64 |
| One ID tag for both kinds | The tag is how both pickers tell the kinds apart; the adversary |
| Wave membership | Phase F. Phase D only names its place (§2.7) |
| A link and a force in one picture | Needs systems or contacts inside universes (unbuilt; Phase E's depth). Phase D checks each kind where it is drawn today |
| Renaming `Link`, `Force`, `link`, `force` | The rename phase |

---

## 2. Decisions

### 2.1 What a relationship is (**DECIDED**, refactor plan Phase D; Part VII G6)

A relationship is one connector that can touch many members. It has a way of choosing its members and a way of carrying direction. A force is its directed form: it surrounds what it acts on and points to what it produces.

### 2.2 One representation (**PROPOSED**)

A new capsule `joinn-link/src/relationship.rs` with read-only leaves. Public names (Cursor may add private ones):

```
Relationship {
    name: String,               // a link's id, or a force's response name
    form: RelationshipForm,     // Link | Force { kind: ForceKind, frame: FrameRef, response: Hash }
    members: MemberSelection,   // Named(Vec<RelationshipPort>) | Body(String)
    order: Order,               // joinn_link::Order: None | Ordered
    head: RelationshipHead,     // Marked (heads are members marked Head) | Response(String)
}
RelationshipPort { body: Option<String>, instance: String, port: u32, mark: Mark }
```

| Source | `form` | `members` | `order` | `head` |
|---|---|---|---|---|
| a universe `Link` | `Link` | `Named`, one port per member, with its body alias and mark | the link's `order` | `Marked` |
| a contact `Force` | `Force { … }` | `Named`, its members (no body; marked `Tail`), in canonical (instance, port) order | `None` | `Response(name)` |
| a `SystemForce` | `Force { … }` | `Body(on)`: every grown instance of that body | `None` | `Response(name)` |

Three functions build it: `relationships_of_universe(&joinn_link::Universe)`, `relationships_of_contact(&joinn_dna::Contact)`, `relationships_of_system(&joinn_dna::System)` (not `joinn_link::System`, which is a lens's grouping), each in canonical order (links by id, as F3 admits them; forces by response name). They never refuse an admitted artifact, and they are never hashed, printed into a coding region, or written under `corpus/` except as §2.5's witness text.

### 2.3 What is unified at which layer

| Layer | Unified? | Tag |
|---|---|---|
| Product word and guides | **Yes**: link and force are two forms of a relationship | DECIDED (refactor plan Phase A, Phase D) |
| Data model (what a relationship is) | **Yes**: one read-only view type over the three codings | PROPOSED |
| Grammar and hashes | **No** | PROPOSED (rule 6; R140) |
| Admission and engine | **No**: links deliver, forces compile with `lower` | DECIDED by G18 for meaning; PROPOSED for code |
| Tables and rows | **No**: Route and Segment rows for links; Stroke rows, style 15, for lassos | DECIDED by G19, rule 64 |
| Shaders and ID tags | **No**: `segment.wgsl` and `LINK_TAG`; `curve.wgsl` `KIND_LASSO` and `FORCE_TAG` | PROPOSED (the adversary) |
| Owner naming | **Yes**: one function prints every relationship owner; the words are kept | PROPOSED |
| Pick decoding | **Shared naming, separate arms**: each scene decodes its own tag, then names through the one function | PROPOSED |
| Member selection | **Two kinds now** (named, a whole body); a third (accepting) is Phase F's | DECIDED (refactor plan Phase D, F) for the place; PROPOSED for the two kinds |

### 2.4 One owner vocabulary, words kept (**PROPOSED**)

A new leaf in `joinn-visual` prints a relationship owner from a `Relationship` and an optional member index: `link <id>`, `link <id> member <i>`, `force <name>`. `UniverseScene::print_id`, `SystemScene::print_id` and `print_owner` (for Phase C's `Owner::Force`) call it. No printed string changes, so no finding is needed and every pasted line in the findings of Phases 7.3, 7.4 and C stays valid. Prediction: `pick`, `regrow` and `links` print every earlier line unchanged.

### 2.5 The relationship table and its witness (**PROPOSED**)

`cargo xtask relationships` prints one line per relationship of every admitted corpus `.universe`, non-growing `.contact` and `.system`, in path order then canonical order:

```
<rel> link <id>: named <k>, order <none|ordered>, tails <t>, heads <h>; owner link <id>
<rel> force <name>: named <k>, combine ℤ 1 → <name>; owner force <name>
<rel> force <name>: body <alias>, combine ℤ 1 → <name>; owner force <name>
```

Last line: `relationships: <n> file(s), <l> link(s), 3 force(s); one view, two looks; planted link-named force, shared style, erased member: refused (ok)`.

Known lines (Cursor pastes the whole table; other universes' lines are printed, not predicted):

```
phase5/ordered.universe link path: named 2, order ordered, tails 1, heads 1; owner link path
phase5/universe.universe link e0: named 2, order ordered, tails 1, heads 1; owner link e0
phase7/calculator.contact force sum: named 2, combine ℤ 1 → sum; owner force sum
phase74/adding.system force count: body numbers, combine ℤ 1 → count; owner force count
phase74/counting.system force count: body numbers, combine ℤ 1 → count; owner force count
```

`cargo xtask relationships --witness` writes the table (every line but the last) once to `corpus/phaseD/relationships.txt`, as `cargo xtask dna-witness` writes Phase 8's witnesses: absent → written; same bytes → left; other bytes → refused, never rewritten. The file is appended to `corpus/hashes.txt` under a dated comment and verified as bytes, as Phase 8's witness texts are.

### 2.6 Both pickers, both kinds, kinds apart (**PROPOSED**; G19 **DECIDED**)

`cargo xtask pick` gains one line per adapter after its scene lines:

```
relationships: links <n> named alike, forces <f> named alike, kinds apart
```

`links` counts link owners named alike by CPU and GPU over the grove views and `ordered.universe`'s galaxy; `forces` counts force owners named alike over both systems at sizes 0, 3, 7 and the contact calculator's standard viewports. **Kinds apart** means: every pixel whose ID carries `LINK_TAG` names a `Link` in the view and every `FORCE_TAG` pixel a `Force`; link rows use styles 12–14 through `segment.wgsl`, lassos style 15 through `KIND_LASSO`; and `STYLE_TABLE`'s colours for 12, 13, 14 differ from 15's. A test asserts the style and colour part on its own.

Standing plants (rule 93), refused on every run:

| Plant | Refused by |
|---|---|
| (a) a force's owner printed with the link word (`link sum`) | the view: the printed name has no `Link` of that name in its scene |
| (b) a lasso row written with `STYLE_HUB` | kinds apart: a force row in a link style |
| (c) a leg's member index dropped (`link <id> member <i>` printed `link <id>`) | the pickers: the member owners the cut allows exceed those named |

### 2.7 The place for waves (**DECIDED** that D keeps a place, refactor plan Phase D; the form **PROPOSED**)

`MemberSelection` documents three kinds: `Named` and `Body` (built), and **accepting** (Phase F: every receptor in the hypernode that accepts the signal; a receptor may refuse). Phase D adds no variant, no grammar and no check for it. The table's `members` word list is `named <k>`, `body <alias>`; Phase F adds `accepting <signal>`. A system force's `Body` selection is already derived from growth, which makes it the nearest existing thing to a wave; Phase F's plan starts from it.

### 2.8 R110 (**OPEN**, unchanged)

Part VII §3.3 asked whether "a link may be a lasso of kind carry: one directed hyperedge, in one of four kinds, for everything outside a body" (R110). Today the record disagrees with itself: `decisions.md` and the backlog say R110 is `answered` by G18 (links transfer data while forces stimulate), and Part VII's own table still says `open, first`. Phase D does not decide it and depends on neither answer:

- It unifies what a relationship **is** and how a click **names** it.
- It does not make a link look like a lasso (G19), does not give a link a response law, and does not add a `carry` kind to `ForceKind`.
- `RelationshipForm` keeps `Link` and `Force` apart, so R110 can later be answered either way by changing what a `Link` maps to, not by changing the view's shape.

R110a (is *bears on* a link or a force?) stays open.

### 2.9 Invariants

| # | Invariant | Commit |
|---|---|---|
| **V181** | A link and a force are both relationships: one view lists every corpus link and force with its member selection, order and head, and its table equals `corpus/phaseD/relationships.txt` byte for byte | PD-02, PD-04 |
| **V182** | One owner name per relationship pixel: on every adapter, the CPU and GPU pickers name every link and force pixel alike, through one printer, with the words `link <id>`, `link <id> member <i>`, `force <name>` | PD-03, PD-05 |
| **V183** | A link and a force never look alike: their styles, colours, pipelines and ID tags are disjoint, and no pixel of one kind names the other (G19) | PD-05 |
| **V184** | Direction is drawn only when declared, for both kinds: a link's spine and mid-path arrowheads only when ordered, a tail's stub arrow only on a tail; a force's arrowhead always, because its response is always declared | PD-05 |

---

## 3. What Runs

| Command | Prints | Exit 1 when |
|---|---|---|
| `cargo xtask relationships` | §2.5's table and last line | an admitted artifact's relationships can't be listed, or a plant is admitted |
| `cargo xtask relationships --witness` | `relationships: wrote corpus/phaseD/relationships.txt` or `… unchanged` | the file exists with other bytes |
| `cargo xtask pick` | every earlier line unchanged; per adapter §2.6's `relationships:` line; the plants | as before, a relationship named differently, or kinds not apart |
| `cargo xtask gate D` | the item's row, `item 1 control answered by check`, `phase D: 1/1` | the item fails |

**Predictions** (marked in the findings, PD-08):

| # | Prediction |
|---|---|
| P1 | The five known lines of §2.5's table appear as written; `3 force(s)` |
| P2 | After PD-03, `pick`, `regrow` and `links` print every earlier line unchanged (`Compare-Object` prints nothing) |
| P3 | `relationships: links <n> named alike, forces <f> named alike, kinds apart` on all three adapters, with `<n>` equal to the sum of the grove lines' and galaxy line's `link owners`, and `<f>` equal to the sum of the system lines' and contact lines' `force owners` |
| P4 | `corpus verify: 68 hash(es) match; cells admitted` after PD-04 (67 + 1); `git diff --stat <PD-01>..HEAD -- joinn/corpus` lists only `phaseD/relationships.txt` and `hashes.txt` |
| P5 | No file under `joinn/crates/joinn-gpu` and no grammar file under `joinn/crates/joinn-dna` changes |
| P6 | `gate all` prints every earlier phase as `gates.lock`, then `phase D: 1/1` |

---

## 4. The Commits

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **PD-01** | **Plan and header** | This plan (Draft 1.0 after AJ's review); `AGENTS.md` header per Appendix A part 1; backlog R140–R143; `decisions.md` V181–V184 and R140–R143 (`open`) | `git show --stat HEAD` lists the plan, `AGENTS.md`, the backlog, `decisions.md`. `Check: docs only` |
| **PD-02** | **The relationship view** | §2.2 in `joinn-link`; §2.7's documented place | Tests: `relationships_of_universe` on `universe.universe` gives one `Link` `e0`, `Named` 2, `Ordered`, `Marked`; on the contact, one `Force` `sum`, `Named` 2, `Response("sum")`; on `counting.system`, one `Force` `count`, `Body("numbers")`; an unordered link's members come out sorted (F3); reordering a contact force's members gives the same view (V127). `gate all` → every line as `gates.lock` |
| **PD-03** | **One owner vocabulary** | §2.4: the printer leaf; the three printers call it | Tests: every existing owner-string test passes unchanged. `cargo xtask pick` and `cargo xtask regrow` printed whole; `Compare-Object` against the PD-01 output prints nothing (P2, paste). Gates 6, 7, 7.2, 7.3, 7.4, 8, B, C each full |
| **PD-04** | **The table and its witness** | §2.5: `cargo xtask relationships`, `--witness`; `corpus/phaseD/relationships.txt`; `hashes.txt` line; `check` gains `Relationships:`; V181 | `cargo xtask relationships` printed whole (paste). `corpus verify` → **68** (paste). `git diff <PD-01>..HEAD -- joinn/corpus` lists only the new file and `hashes.txt`. `gate all` → every line as `gates.lock` |
| **PD-05** | **Both pickers, both kinds** | §2.6: the `relationships:` line, kinds apart, the style test, the three plants; V182, V183, V184 | `cargo xtask pick` printed whole (paste): P3 on every adapter; the plants refused. **Shown then reverted:** plant (a) not planted: `pick` exits 1 naming the missing plant |
| **PD-06** | **Gate D** | §6; `phase D` after `phase C` in `PHASE_LABELS`; `gate_d.rs`, `gate_d_items.rs`, `gd_relationships.rs`, `gd_relationships_control.rs`; the `gate_table` row; the lock row; the `Replace` text const and the test that builds it (as gate 8's `SWAPPED_READING`) | `cargo xtask gate all` exits 0 and prints `phase D: 1/1`. **Shown then reverted**, with `gate D --item 1` only, pasting each failure: (a) give the item gate 8 item 1's `opposes` text: the control does not flip, and the harness refuses the item; (b) the view maps a `SystemForce` to `Link`: the derived table differs from the witness; (c) the system printer prints `link count`: the check fails on plant (a)'s rule |
| **PD-07** | **Rules** | Appendix A part 2: rule 96 appended; rule 76 widened | `git show --stat HEAD` lists `AGENTS.md` only. `Check: docs only` |
| **PD-08** | **Findings** | `docs/Findings/phase-D-relationships.md`: every command's output from PD-02 on; P1–P6 marked; *The adversary* (§7.1) answered; R110's two records quoted side by side (§2.8) | Every prediction marked |
| **PD-09** | **Docs and freeze** | Guides: 01 (the code table's *Relationship, drawn* and *Directed relationship* rows point at one view), 02 (a short *One relationship* section), 03 (`## One relationship (Phase D)` above Phase C's), 05 (Relationship: two forms today, link and force; a wave is the third, ahead); `decisions.md` V181–V184 with status and evidence | Tier 1. `corpus verify` 68 |
| — | **The phase stop** | `cargo xtask stop-check --fresh` (phases 0 … 8, B, C, D; exits 0); `phase-D-stop.md`; commit `PD-stop`, push, ledger; print `phase D: stopped for review`; stop | — |

Dependencies: in order. PD-03 and PD-04 depend on PD-02; PD-05 on PD-03 and PD-04; PD-06 on PD-05; PD-07 on PD-06.

**If something has to be cut for time:** the `--witness` flag's "unchanged" path first (writing once is enough). Never cut PD-02, PD-03, PD-05, PD-06 or V181–V183.

---

## 5. Test Strategy

As Phase 7.4's §5. Every owner string is compared exactly; no test asserts only that a relationship "exists". The view is tested on every corpus artifact that has a link or a force, and on hand-written ones for each `MemberSelection` and `RelationshipHead`.

---

## 6. Exit Gate D

One item in `xtask/src/fns/gate_d_items.rs`, built like gate 8's: a witness file is the control artifact, and the mutant is the text a wrong derivation would print. It is answered **by its check**.

| # | Item | `control_artifact` | `opposes` | Control answers `true` when |
|---|---|---|---|---|
| 1 | **A link and a force are both relationships, and both pickers name them** | `corpus/phaseD/relationships.txt` | `Replace(LINK_NAMED_FORCES)`: the table with each force line's `force <name>` (both the kind word and the owner) printed `link <name>`, as a unification that erased the owner distinction would print it | the table the relationship view derives from the corpus differs from the subject |

- **On the subject** the control answers `false`: the derived table equals the witness.
- **On the mutant** it answers `true`: the derived table names `force sum` and `force count` where the mutant names `link sum` and `link count`.
- Text has no neutral edit (`neutral` returns `None` for text), as gate 8's items.

**Check (`gd_relationships`)**, on every adapter:

1. The derived table equals `corpus/phaseD/relationships.txt` byte for byte (V181).
2. Every relationship owner a CPU pick names, in the grove views, `ordered.universe`'s galaxy, both systems at sizes 0, 3, 7 and the contact calculator, is named alike by the GPU (`disagree 0`) and resolves to exactly one row of the view (V182).
3. Kinds apart (§2.6) (V183).
4. Arrows exactly where declared: no spine or mid-path arrowhead on an unordered link (V150's count), a tail stub arrow only on tails, and one arrowhead per lasso (V184).
5. The three plants of §2.6 are refused, and the gate names them.

This row does not restate V150 or V152 or V164 (rule 41): those judge each kind alone. This row judges that both kinds are one view, named by one printer, and never confused.

### 6.1 Conditions for opening the next phase

1. `stop-check --fresh` exits 0 with `phase D: 1/1` and every earlier line as `gates.lock`; no tripwire fired.
2. `phase-D-relationships.md` exists with every prediction marked and the adversary answered.
3. Claude's review lists no open snag, or AJ has chosen to carry each one forward.
4. AJ has run §6.2.

### 6.2 AJ's check (at the phase stop, about three minutes)

```
cd D:\JoInn\joinn
cargo xtask relationships
cargo run --release -p joinn-shell-desktop -- corpus/phase5/ordered.universe
cargo run --release -p joinn-shell-desktop -- corpus/phase7/calculator.contact
```

1. Read the table: links and forces in one list, each with its owner word.
2. In the universe window, click the link: `link path` (or `link path member <i>` on a leg).
3. In the calculator window, click the lasso: `force sum`.
4. Tell Claude whether the two still look plainly different, and whether the table reads as one idea.

---

## 7. Adversary, Risks and Tripwires

### 7.1 The adversary

**Unifying them erases a pixel owner, and the pickers disagree on that pixel.** The three plants are the three ways it happens: a force named as a link, a force drawn in a link's style, a leg's member index lost. Each must be refused on every run, and the gate's mutant is the first of them written into the witness. The findings answer it with the refusal lines and P3 on every adapter.

### 7.2 Risks

| Risk | What to do |
|---|---|
| A printer refactor changes a string by one character | P2's `Compare-Object` catches it; revert the change, never re-paste the findings |
| `corpus verify` has no tag for a non-DNA witness text | A snag; verify it as Phase 8's witness texts are verified (bytes, by path), and say so in the finding |
| The view's canonical order for an ordered link differs from the routes' member numbering | The view keeps declaration order for an ordered link (rule 76, F3); a test compares `link <id> member <i>` from the view with the routes' incidence order on `ordered.universe` |
| The relationship type tempts a shared admission function | Out of scope (§1.4); admission stays per kind |
| Rule 41 refuses the `(artifact, opposes)` pair | It can't collide: the artifact is new. If the uniqueness test still refuses, a snag with both items printed |

### 7.3 Tripwires

Run 7.3–9 §7 applies (T1–T5). For T1, the only corpus changes allowed are the new `phaseD/relationships.txt` and its appended `hashes.txt` line.

---

## 8. Open Items

| ID | Topic | Question |
|---|---|---|
| **R140** | One grammar for relationships | Should a later grammar write links and forces in one section, and does that belong to the rename phase or to Phase G's strand storage, where Graph Storage already writes hyperedges? |
| **R141** | What a wave accepts | For Phase F's derived selection: is "accepts the signal" a receptor's frame (rule 36's equal frames), a law the receptor carries, or both? |
| **R142** | A link with a response | Could a link ever carry a response law (a carry force), and would that be R110 answered yes? Joins R110a |
| **R143** | Links and lassos in one picture | When contacts or systems are drawn inside a universe (Phase E's depth), do a lasso and a routed link share gutters, and which is drawn over the other? |

---

## Appendix A · `AGENTS.md` changes

**Part 1 (PD-01).** Replace the header paragraph Phase C wrote with:

```markdown
The active plan is docs/Plans/JoInn Hypernode Refactor Plan.md. Phases A, B and
C are closed. Phase D, one relationship, is built from docs/Plans/JoInn Phase D
Implementation Plan.md: a link and a force are both relationships in one view,
named by one printer with the words link and force, and still drawn apart.
Work the phase's commits in order, one git commit per numbered step, check each
by tier, and stop once at the phase's end. Never stop to ask; follow the plan's
Snags section instead.
```

**Part 2 (PD-07).** Replace rule 76 with:

```markdown
76. DIRECTION IS DRAWN ONLY WHEN DECLARED. A spine or a mid-path arrowhead
    appears only on an ordered link; a tail's stub arrow only on a tail. A
    force's arrowhead is always drawn, because its response is always
    declared.
```

Append:

```markdown
96. ONE RELATIONSHIP, TWO LOOKS. A link and a force are both relationships:
    one view gives each its members (named, or a whole body) and its head
    (marked members, or a response), and one printer names their pixels
    `link <id>`, `link <id> member <i>` and `force <name>`. They never share a
    style, a pipeline or an ID tag. A derived member set is a wave and waits
    for Phase F.
```

Keep every other rule.

## Appendix B · Documents

- **Backlog**, after R139: `> **30 Oct 2026 · Hypernode refactor, Phase D.** R140–R143 come from the Phase D plan.` then R140–R143 (`open`) with §8's questions.
- **`decisions.md`**: V181–V184 (`open`; PD-09 sets them); R140–R143 (phase `D`, `open`). R110 and R110a unchanged.

---

*JoInn Phase D Implementation Plan, Draft 0.1 (30 Oct 2026), written from the Hypernode Refactor Plan's Phase D. AJ decided (refactor plan, 30 Oct; Part VII G6, G7, G18, G19): link, force and later wave are one connector; forces stay directed hyperedges drawn as lassos; a link and a force never look alike; wave membership is Phase F. Proposed, open to AJ's veto: a read-only view rather than a grammar change, owner words kept, one printer, a witness table with one gate item, and the place left for waves. Left open: R110.*
