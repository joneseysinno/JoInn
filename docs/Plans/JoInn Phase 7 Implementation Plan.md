# JoInn Phase 7 Implementation Plan

**Contact bodies: a body with no wires inside · the combine force · a working plan for Cursor**

Author: AJ, with Claude · Draft 0.1 · September 29, 2026

> **Every decision in this plan is final.** Nothing here waits on AJ. Cursor never stops to ask AJ anything, and no file in this phase is typed by AJ. If a step can't be done the way the plan says, Cursor follows §0.3 (Snags) and keeps going.

---

## For AJ: this plan in plain English

**What Phase 7 is.** Part V says a body is cells touching, and that an operation like sum is a force from outside, not a cell. Phase 7 makes that real for the calculator. There is a new file kind, `.contact`. It lists the cells and one force, `combine`, and it has **no wires**. The parser refuses a wires section with the words *wires belong to systems*. The contact calculator must print the same five-line transcript, refusal of `"two"` included. In the window it draws as cells pressed against each other, with nothing between them.

**Your decisions are built in:**

1. **R76 is decided as proposed.** Combine inside a body doesn't care about order. Minus and divide are combine read at a turn. "2 + 3 = 5" is the host's job. Anything else that needs order is wired between bodies.
2. **R78 is recorded, not rushed.** The Floor gets a dated entry listing each reduction that was tried and how it came out. **No pair leaves the Floor in this phase.** `eq`/`choose` stay a pair, as you chose. Contact needs no new pair: it is cells filling one boundary with no `bind`. The `.contact` grammar proves this by having no way to write a `bind`.
3. **R81 is decided: two opposites, four roles.** A cell faces out or in, and it holds or reacts. That gives protect, carry, store and respond. Roles are **worked out from the body, never written in it**. In code, a role is two yes/no values, so a fifth role can't be written.
4. **Phase 6's three carry-forwards are the first work:** a permanent test of the window's own run path, wgpu's real device-lost signal, and Enter on an empty box.

**Choices Claude made while writing this. Say if you disagree before Cursor starts:**

- **Numbering.** This is Phase 7. The old Phase 7 (charts, zoom, bands, links) becomes **Phase 7.1**, so every later phase keeps its number and older documents stay right.
- **Combine keeps its inputs.** Part V §6.2 said combine *consumes* 2 and 3 and leaves only 5. That was Claude's proposal, and on a closer look it is wrong. Sum throws information away: 5 could be 2 + 3 or 1 + 4. So its opposite, *separate*, can only work if you're handed one part back, and that is exactly a **turn** (5 − 3 = 2). If the inputs were destroyed, sum would have no working opposite. So the inputs stay, and the result appears beside them.
- **Every force must have its opposite on file.** The ℤ combine (the old sum cell's laws) is only allowed because its separate exists: the turn cell `sum_turn`, already in the corpus. ℚ can't combine in a contact body yet, because it has no turn. That is a real finding, not a gap in the plan.
- **The response law lives with the number type, and the file pins it.** "How ℤ responds to combine" is kept in one place for the frame ℤ. The contact file names that law by its hash, the way a pinned link records the code edition it was built to. If the two don't match, the body is refused.
- **The engine is allowed to do its own plumbing.** To run a contact body, the engine works out the deliveries itself. That work is never written to a file, drawn, or addressable. Claude's prediction: what it works out is the Phase 2 wired calculator **byte for byte**, same hash. That shows the wires were always the engine's business, never the body's.
- **A force owns no pixel.** You only see a force through what it did. The result cell is drawn dim (*latent*) until combine fills it, then it turns blue like any other cell.
- **Only the ports on the surface are drawn.** Inside a body, cells touch, so there is nothing to click between them.
- **Enter on an empty box does nothing.** It prints `(empty: nothing sent)`. A host never invents an intent, and an empty box is not one.

**One correction from our chat.** I said the calculator's integers are *stores* and its result *responds*. With the rule made exact, it comes out differently. `cli_a` and `cli_b` face the host and hold their number, so they are **protectors**. That fits: they are what refuse `"two"`. `sum` faces the host and reacts, so it **carries**. Store and respond first appear in the beam, where the material cell sits inside and the moment responds inside.

**The honest adversary.** *Contact is just wires with the names hidden.* This phase measures that directly. If running or drawing a contact body needs any fact that isn't in the `.contact` file or the frame's response law, contact is a naming convention and Part V's C2 needs rework. Examples: a port pairing that order-freedom can't justify, a hand-placed position, or a hidden member order.

**What you do at the end (Stop C), about two minutes.** Open the contact calculator in the window and click around. The exact steps are in §6.2. You report "it worked" or what looked wrong. You never copy output.

**Your prompts to Cursor** (copy exactly, one per chunk):

- Chunk A: `Do chunk A of docs/Plans/JoInn Phase 7 Implementation Plan.md. Follow AGENTS.md.`
- Chunk B: `Do chunk B of docs/Plans/JoInn Phase 7 Implementation Plan.md. Follow AGENTS.md.`
- Chunk C: `Do chunk C of docs/Plans/JoInn Phase 7 Implementation Plan.md. Follow AGENTS.md.`

If Cursor runs out of room partway: `Continue chunk A of docs/Plans/JoInn Phase 7 Implementation Plan.md from the first commit not in git log. Follow AGENTS.md.` (Use the right letter.)

After each stop, tell Claude "Cursor finished chunk A" (or B, C).

---

## 0. How Cursor Works This Plan

| | |
|---|---|
| **Plan file** | `D:\JoInn\docs\Plans\JoInn Phase 7 Implementation Plan.md` |
| **Code** | `D:\JoInn\joinn\`. No new crate. No new external dependency |
| **Standing rules** | `AGENTS.md` as updated by P7-01 (Appendix A) |
| **Unit of work** | a **chunk** (A, B or C). Inside a chunk, do the commits in the order listed. Each commit is its own git commit, and its message starts with its id (`P7-05: …`) |
| **End of a chunk** | write the stop report (§0.2), commit it, `git push`, and stop. Do not start the next chunk |
| **Who decides** | every decision is in §2. Cursor decides only module layout (rule 25), function bodies, private type representation, error-string wording where §2 gives none, and WGSL function bodies |
| **Who checks** | Claude, at each stop, from a fresh clone on Linux (llvmpipe). GitHub CI runs on Windows and Linux on every push. AJ runs the window at Stop C |

### 0.1 The commit report

Every commit ends with this block, in the commit message body **and** in the chunk's stop report. Every value is copied from the terminal, never summarised.

```
Commit:     P7-NN (hash in git log)
Done-when:  <the command> → <the line it printed>          MET | NOT MET
Suite:      cargo test --workspace --no-fail-fast → <N passed, M failed>
Scans:      vocab → <last line> · modules → <last line> · layers → <last line>
Snags:      none | <each thing that went differently from the plan, with the printed line>
```

### 0.2 The stop report

At the end of each chunk Cursor writes `docs/Findings/phase-7-stop-<letter>.md` with:

1. `git rev-parse HEAD` (full hash).
2. Every commit report block from the chunk, in order, with real hashes.
3. The last line of each of: `cargo test --workspace --no-fail-fast`, `cargo xtask gate all`, `cargo xtask corpus verify`, `cargo xtask vocab`, `cargo xtask modules`, `cargo xtask layers`, `cargo xtask floor`, `cargo xtask assay agree`, `cargo xtask adapters`, `cargo xtask pick`, `cargo xtask regrow`, and from P7-06 on `cargo xtask forces`, from P7-10 on `cargo xtask contact`, from P7-11 on `cargo xtask roles corpus/phase7/calculator.contact`. Also every line of `gate all` that contains `fail`, and every failing test name.
4. **CI**: for the newest run after the push, read with PowerShell `Invoke-RestMethod "https://api.github.com/repos/joneseysinno/JoInn/actions/runs?branch=main&per_page=3"` and that run's `jobs_url`. Paste `head_sha`, `status`, `conclusion`, and each job's `name` and `conclusion`. If the run hasn't finished, wait and read again. A failed job is a snag, not a reason to change CI.
5. **Snags**: every snag from the chunk in one list, each tagged with its commit id. A snag says what the plan actually says.

Then commit (`P7-stop-<letter>: stop report`), push, and stop.

### 0.3 Snags

A snag is any point where a done-when can't be met as written, or a prediction in this plan turns out wrong.

- **Never** fake it, work around a scan, weaken a test, rebless a golden, or change the plan's meaning to make it pass.
- **A prediction that differs is not a failure.** Paste what the machine printed next to what the plan predicted, and continue. Claude settles it at the stop.
- Leave that piece in its honest state and **continue with the next commit**.
- If a later commit depends on the snagged one (§4), skip it and write `skipped: depends on P7-NN`.

---

## 1. Scope Fence

### In scope

- **Phase 6's carry-forwards**: the shell's run path tested against regrow; wgpu's device-lost callback; Enter on an empty buffer.
- **The surface rename**: a body's derived outline is its *surface* in code and output; *membrane* is a cell's only.
- **The force register** (`joinn-prim::forces`): combine and its separate for ℤ, the order-free check, the opposition check, `cargo xtask forces`.
- **The `.contact` file kind** (`joinn-dna::contact`): model, parser, printer, hash tag, and the corpus file `corpus/phase7/calculator.contact`.
- **Its mutation catalogue first** (R60).
- **Admission, surface and derivation** (`joinn-link`): `check_contact`, `contact_surface`, `lower`.
- **Running it**: `joinn-cli`, the test host, and the shell accept `.contact`; `cargo xtask contact`.
- **Roles**, derived: `joinn-link::body_roles`, `cargo xtask roles`.
- **The picture**: contact layout, the scene for a contact body, the latent style, and `pick`/`regrow`/`layout` extended to `.contact`.
- **Gate 7** and the freeze.
- **Docs**: Part V's decisions recorded, the Floor's R78 entry, the backlog, `decisions.md`, the roadmap note, guides and glossary.

### Out of scope (Cursor refuses these even when they look small)

| Not now | Why it is tempting |
|---|---|
| Writing `separate`, `carry` or `release` in a `.contact` file | Only combine is written this phase. Separate already exists as a turn; carry/release belong with systems and the beam |
| A combine with more members than its response has in-ports (a fold) | The beam needs it; R84 |
| One port feeding two forces | That is carry to many (fan); R85 |
| Removing any Floor pair, or adding one | Rule 12. R78 is recorded, not enacted |
| Converting any `.body` file to `.contact`, or changing any existing corpus file, hash, golden or `grandfather.txt` | Never. `.body` is legacy and keeps its 43 rows |
| Changing the `Frame` trait or any frame's version | The response register lives in joinn-prim, keyed by frame |
| ℚ in the register | ℚ has no turn yet, so its combine has no separate (§2.2) |
| Text on screen, zoom, charts, a second body in a picture, hyperedge forms | Phase 7.1 |
| Drawing a force, or drawing interior ports | V130: a force owns no pixel; inside a body cells touch |
| The steel beam, part names, source links, relationships | The next phase, once this one holds |
| A `layout` or `roles` section in any file kind | Layout stays computed (R71); roles are derived (R81) |
| Upgrading gates 1–3 (R64); describe-at-firing-time (R69) | Carried forward |

---

## 2. Decisions

All **DECIDED**.

### 2.1 Where things live

No new crate. The Phase 6 layer table (`cargo xtask layers`) is unchanged: no new workspace edge, no new external dependency.

| Piece | Crate · module | Depends on |
|---|---|---|
| `Contact`, `ContactCoding`, `Force`, `ForceKind`, `parse_contact`, `print_contact`, `hash` of a contact | `joinn-dna::contact` (beside `body`) | frame |
| `TAG_CONTACT = b"joinn.contact.v1"` | `joinn-frame::hash` (beside `TAG_BODY`) | — |
| The force register, `order_free`, `check_register` | `joinn-prim::forces` | dna, frame, gate |
| `check_contact`, `contact_surface`, `lower`, `body_roles` | `joinn-link` (beside `instance_ports`, `surface`) | dna, prim, … (existing) |
| `layout_contact`, `Scene::grow_contact`, `Scene::regrow_contact` | `joinn-visual` | link (existing) |
| Latent style in the shape organelle | `joinn-gpu` | — |
| Loading `.contact` | `joinn-cli`, `joinn-test-host`, `joinn-shell-desktop` | link (existing) |

`joinn-host`'s `Role` (cell, input, output, refusal) is the **accessibility** role and is not touched: `.desc` goldens hash it. Part V's role is a different thing, named **`BodyRole`** in code (§2.7).

### 2.2 The force register (joinn-prim::forces)

A force's response law lives in what it acts on (Part V C10). For a number, that is its frame. The register is **frame vocabulary for forces**, the way ℤ's constructors are frame vocabulary for `build`. It is one short table, keyed by (force, frame), and it is not a list of kinds (C18).

```rust
pub enum ForceKind { Combine }                       // closed; Phase 7 writes combine only
pub struct RegisterRow {
    pub force: ForceKind,
    pub frame: (&'static str, u16),                  // ("ℤ", 1)
    pub response: Hash,                              // the coding region that says how the frame responds
    pub separate: Hash,                              // its opposite: a turn of `response`
}
pub fn register() -> &'static [RegisterRow];
pub fn response(force: ForceKind, frame: &FrameRef) -> Option<Hash>;
pub fn order_free(oracle: &dyn Oracle, frame: &dyn Frame, seed: u64, bound: NonZeroU32) -> Verdict<()>;
pub fn check_register(rows: &[RegisterRow], cells: &BTreeMap<Hash, Cell>, natives: &NativeRegistry, frames: &FrameRegistry) -> Verdict<()>;
```

**The one row:**

| force | frame | response | separate |
|---|---|---|---|
| combine | ℤ 1 | `6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39` (the sum cell) | `6fcb1e7397454543fad3b22d4b1c310932985e44d1ed64ae94142ab82339e9cf` (`sum_turn`: lineage `6b32…`, `turn 0 from {1 2}`) |

The sum cell's coding region was always ℤ's response to combine. Its laws (identity, commutative, associative), its founding witness (2, 3) → 5, and its allele `add@ℤ` are unchanged. Nothing is rehashed.

**`order_free`.** It draws `bound` pairs and `bound` triples from the frame's generators, with a seeded RNG, and checks both laws through the oracle: `f(a, b) = f(b, a)` and `f(f(a, b), c) = f(a, f(b, c))`. The first counterexample is a refusal:

`not order-free: f(a, b) = <x> but f(b, a) = <y> at a = <a>, b = <b>; acceptance is a response whose result does not depend on member order` (and the same shape for associativity, naming `a, b, c`).

The register uses `bound = 64` and seed `7`. Both are declared constants (rule 20).

**`check_register`**, for every row:

1. The response cell exists, has in-ports `0 … k−1` and one out-port `k` all in the row's frame, and has an allele for that frame whose native exists.
2. `order_free` passes on that native.
3. **Opposed** (C23): the separate cell exists, its `lineage` is the response's hash, and it declares a turn. Otherwise: `combine on <frame> is unopposed: no separate; acceptance is a turn of cell:<response>`.

**Plants** (rule 19), all run by `cargo xtask forces` against `natives_with_mutants()`:

| Plant | Must print |
|---|---|
| `order_free` on `mutant.difference` | refused, commutativity counterexample |
| `order_free` on `mutant.midpoint` | refused, associativity counterexample (it is commutative) |
| `order_free` on `mutant.max` and on `mutant.plus1` | each `order-free (ok), not registered`. Both are wrong answers that don't depend on order (`plus1` is `a + b + 1`, which is commutative and associative). Order-free is necessary, not sufficient: a pin to anything but the registered response is refused by `check_contact` |

**`mutant.midpoint` is new** (P7-06). It computes `⌊(a + b) / 2⌋` and goes into `natives_with_mutants()` only. No existing mutant is commutative but not associative: `plus1`, `max` and `times` are order-free, and `saturating` breaks associativity only near the i64 bounds, where sampling may never reach. Its counterexample: `f(f(0, 0), 4) = 2` but `f(0, f(0, 4)) = 1`.
| `check_register` on a copy of the row with `separate` set to the response's own hash | refused as unopposed |

### 2.3 The `.contact` file kind (joinn-dna::contact)

**Source form** (`corpus/phase7/calculator.contact`, written by P7-07 exactly as below):

```
contact {
  codex 1
  genome {
    cell:c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e as cli_a, cli_b
  }
  grants {
    stdin: cli_a, cli_b
  }
  forces {
    combine ℤ 1 cell:6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39 as sum from cli_a@1, cli_b@1
  }
  budget { steps 100000 }
  lineage none
}

---

regulatory {
  prompts { cli_a "a: "  cli_b "b: " }
  present { sum "{0} + {1} = {2}" }
  names { sum "Sum" }
  labels { sum "Sum" }
}
```

The section parser is `.body`'s: sections in any order, `#` comments, optional commas, braces optional. The regulatory region is `BodyRegulatory`, unchanged, and a response name (`sum`) may appear in it like any instance.

**Canonical form** (`print_contact`, hashed with `TAG_CONTACT`):

```
budget
steps <n>
codex <n>
genome
cell:<hex> as <instances>          one line per hash, hashes ascending, instances sorted, joined ", "
grants
<capability> <instances>           capabilities ascending, instances in grant order, joined " "
read                               only when non-empty, then one name per line
lineage none | lineage <hex>
forces
combine <frame> <version> cell:<hex> as <response> from <members>
```

Forces are sorted by response name. **Members are sorted by (instance, port) and joined by a space, so member order is never hashed** (V127). `forces` comes last, where `.body` has `wires`.

**Claude's prediction** for `calculator.contact`, computed by hand from the form above:

```
budget
steps 100000
codex 1
genome
cell:c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e as cli_a, cli_b
grants
stdin cli_a cli_b
lineage none
forces
combine ℤ 1 cell:6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39 as sum from cli_a@1 cli_b@1
```

**Hash: `868e79b293c8d35625f514274eb0bf04d228898ed994a795b23b942a530b6209`.** (The same script reproduces `calculator.body`'s `b55fba1e…` from its canonical text, so the method is checked.)

**Parse refusals** (each names its acceptance, rule 35):

| Input | Refusal |
|---|---|
| a `wires` section | `contact: wires belong to systems; a body's cells touch. acceptance is a forces section` |
| a force word other than `combine` | `contact codex 1 writes combine only; <word> is not written here (separate is combine read at a turn). acceptance is combine` |
| a `prim:` genome entry | `contact: a genome entry is a cell; prim:<name> belongs to .body. acceptance is cell:<hash>` |
| a `declarations` section | `contact codex 1 has no declarations; acceptance is a .body or .universe for an assert` |
| a response name that is also a genome instance | `contact: <name> is both a cell and a force's response; acceptance is a new name` |
| a member written twice in one force | `contact: <member> is written twice in force <name>` |

**The four-spellings test** (like `.body`'s): the source above; a second with members `from cli_b@1 cli_a@1`, sections reordered and comments added; a third on one line; a fourth changing only the regulatory region. All print identical canonical text and hash `868e79b2…`. A fifth with grant order `stdin: cli_b, cli_a` hashes differently (grant order is hashed, G4).

### 2.4 Admission: `check_contact` (joinn-link)

`check_contact(contact, cells, frames) -> Verdict<()>` returns the first refusal, in this order:

1. **Cells.** Every genome hash is supplied (the `check_body` refusal, reused).
2. **Grants.** Every granted instance exists.
3. **Members exist.** Every member names a genome instance and a port it has: `member <m> names no port; acceptance is a port of <instance>`.
4. **Receptor** (Part V §3.1). The member port's frame equals the force's frame: `receptor: <m> is <frame>, the force is combine <frame'>; acceptance is a member in <frame'>`. This comes before the direction check, because a membrane admits by what a thing *is* first.
5. **Direction.** Every member is an out-port: `member <m> is an in-port; acceptance is an out-port`.
6. **One force per port.** `<m> is reached by forces <a> and <b>; acceptance is one force per port (R85)`.
7. **Pin.** `forces::response(Combine, frame)` equals the pinned hash: `combine on <frame> responds by cell:<registered>; force <name> pins cell:<pinned>. acceptance is the registered response`. Unregistered frame: `combine on <frame> has no registered response; acceptance is a frame in the force register`.
8. **Arity.** Member count equals the response's in-port count: `force <name> has <n> members; its response takes <k>. acceptance is <k> members (R84)`.
9. **The response cell is supplied** (it is fetched by hash like any genome cell).

`check_contact` does not resample order-freedom. That was settled when the register was admitted (`check_register`, run by `cargo xtask forces` and by gate 7 item 2).

### 2.5 Derivation: `lower` (joinn-link)

```rust
pub fn lower(contact: &Contact, cells: &BTreeMap<Hash, Cell>, frames: &FrameRegistry) -> Verdict<Body>;
```

`lower` runs `check_contact` first. It then derives the internal deliveries the engine needs, and **its inputs are exactly the contact, the cells, and the register** (the signature enforces it). The derived `Body`:

- **genome** = the contact's genome, plus `cell:<pinned response> as <response>` for each force;
- **grants, reads, budget, codex, lineage** = the contact's;
- **wires** = for each force, its members in canonical order: member *i* → `<response>@i`;
- **declarations** = none; **regulatory** = the contact's, unchanged.

**The derivation is the engine's business** (rule 61):

- No `.contact` file can express it.
- No description or picture shows it. The contact scene has no link rows.
- No xtask command writes it to disk. `cargo xtask contact` prints its canonical text to stdout for comparison only.

**Claude's prediction** (gate 7 item 1 asserts it): `print_body(lower(calculator.contact).coding)` is **byte-identical** to `print_body` of `corpus/phase2/calculator.body`, and its hash is `b55fba1eff65942099f6daf84b8bc47d605be05f637d805d260d3fcfd8c3ebde`.

**Pairing doesn't matter, and that is tested.** A test lowers with the members assigned in every order (2 orders for the calculator) and runs the script (§2.12) under each. `sum@2` holds `5` every time. The *present* line differs (`3 + 2 = 5`), and that is exactly R76: presentation is the host's, and canonical order is what makes it stable.

### 2.6 The surface of a contact body

`contact_surface(contact, cells) -> Verdict<Vec<SurfacePort>>`, derived and stored nowhere (rule 30):

- every port of every genome cell that is **not a member** of a force;
- every response port that is **not fed by a member**, meaning each response's out-port.

For the calculator: `cli_a@0 in Text 1`, `cli_b@0 in Text 1`, `sum@2 out ℤ 1`.

**Two derivations, one truth** (rule 51): `contact_surface(c)` equals `surface(lower(c))` (the renamed ∂(body), §2.11) on every `.contact` in the corpus, as a set of (instance, port, direction, frame). The host's intent set is still one address per surface in-port (rule 28), so the contact calculator's intent set is `{cli_a@0, cli_b@0}`, the same as the wired one's.

### 2.7 Roles (joinn-link::body_roles)

```rust
pub enum Facing { Out, In }        // Out: the cell has at least one port on the surface
pub enum Holding { Holds, Reacts } // Reacts: the cell is a force's response
pub struct BodyRole { pub facing: Facing, pub holding: Holding }
pub fn body_roles(contact: &Contact, cells: &BTreeMap<Hash, Cell>) -> Verdict<BTreeMap<String, BodyRole>>;
```

| | holds | reacts |
|---|---|---|
| **faces out** | protect | carry |
| **faces in** | store | respond |

The four words are *printed* from the pair (`BodyRole::word()`). **There is no role list and no role enum with four variants.** A fifth role cannot be written (rule 65). Roles are never written in any file kind.

**Claude's prediction**, printed by `cargo xtask roles corpus/phase7/calculator.contact`:

```
roles calculator.contact
cli_a protect (faces out, holds)
cli_b protect (faces out, holds)
sum carry (faces out, reacts)
roles: 3 cell(s); protect 2, carry 1, store 0, respond 0
```

**Tests:**

- **Invariance (V131).** Roles are unchanged under the kind's neutral edit and under a regulatory rewrite.
- **Store and respond.** A test-only contact built in memory chains two forces over three inputs: `combine … as s1 from a@1, b@1` and `combine … as s2 from c@1, s1@2`. It prints `s1 respond (faces in, reacts)`. A test-only cell with a single out-port that a force reaches prints `store (faces in, holds)`. These fixtures are code, not corpus files.

### 2.8 Contact layout (joinn-visual)

Same units and constants as Phase 6 §2.3, except that **cells touch: there is no gap** (`GAP_X = GAP_Y = 0`).

1. **Column**: a genome cell is column 0. A response is `1 + max(column of its members' instances)`.
2. **Column 0**: cells in name order, stacked from `top = MARGIN`, touching. Height = `4 × max(surface in-ports, surface out-ports, 1) + 2`.
3. **A response spans its members.** Its top is its first member's top and its bottom is its last member's bottom, so it touches every cell it combined. Two refusals:
   - members that aren't adjacent in one column: `layout: force <name>'s members do not touch in one column; acceptance is members stacked together (R84)`;
   - a response too short for its surface ports: `layout: response <name> needs height <h>, its members span <s>`.
4. **x**: `left = MARGIN + column × CELL_W`.
5. **Surface**: the rectangle `(0, 0)` to `(W, H)`. `W = 2·MARGIN + columns·CELL_W`, and `H = MARGIN + the lowest bottom`.
6. **Only surface ports are drawn.** In-ports go on the left edge, out-ports on the right edge, with `y = top + 3 + 4k` for the k-th surface port on that side. Member ports and response in-ports are not drawn.
7. There are no wires and no link rows.

**Claude's prediction** for `cargo xtask layout phase7/calculator.contact` (P7-12 asserts it byte for byte):

```
layout body
surface 0 0 32 20 r 3
cell cli_a 4 4 12 6 r 2
cell cli_b 4 10 12 6 r 2
cell sum 16 4 12 12 r 2 response
port cli_a@0 in 4 7 r 1
port cli_b@0 in 4 13 r 1
port sum@2 out 28 7 r 1
camera 640x360: k 16, origin 64 20
camera 1000x777: k 28, origin 52 108
camera 1280x720: k 32, origin 128 40
camera 1920x1080: k 52, origin 128 20
```

### 2.9 The contact scene

- `Scene::grow_contact(alias, &Contact, cells)` and `Scene::regrow_contact(alias, &Contact, cells, &BodyState)` build the same `Tables` types as Phase 6. They have no link rows and no incidence rows, and an empty table binds as one zeroed row, as today. `present`, `apply_run`, `take_pending`, `resolve` and `print_owner` are shared with the wired scene, unchanged.
- **Latent.** A cell row gains flag **bit 2, latent**. It is set on a response whose out-port holds no value. `present` clears or sets it from the description. `regrow_contact` sets it from `BodyState`.
- **Styles.** Style **7, response latent, `#39414D`** is appended to the style table. The shape shader picks the style in this order: refused (3), else latent (7), else the row's style. The wired calculator never sets bit 2, so gate 6's colors don't move.
- **The delta bound (V121)** is the same rule, counting drawn ports only: `Σ (1 + drawn ports(i))` over the instances a run touched, plus 1 when it clears a refused bit it didn't touch. A port with no row is ignored by `present`.
- **Regrow is the test of the delta (rule 58).** After every scripted event, the tables `grow_contact` + deltas build equal `regrow_contact`'s, byte for byte.

**Claude's prediction** for the script (§2.12) on the contact calculator:

| Event | Touched | Rows (bound) | What changed |
|---|---|---|---|
| 1 `cli_a@0 ← "two"` | cli_a | 2 (2) | cli_a refused, cli_a@0 filled |
| 2 `cli_a@0 ← "2"` | cli_a, sum | 1 (4) | cli_a no longer refused |
| 3 `cli_b@0 ← "3"` | cli_b, sum | 3 (4) | cli_b@0 filled, sum no longer latent, sum@2 filled |

### 2.10 The picture's predictions (1280×720: `k 32`, origin `128 40`)

**Probes.** A layout point `(u, v)` probes pixel `(128 + 32u, 40 + 32v)`.

| Layout point | Pixel | Owner printed |
|---|---|---|
| (2, 2) | 192, 104 | `body surface` |
| (10, 7) | 448, 264 | `body.cli_a` |
| (10, 13) | 448, 456 | `body.cli_b` |
| (22, 10) | 832, 360 | `body.sum` |
| (4, 7) | 256, 264 | `body.cli_a@0` |
| (4, 13) | 256, 456 | `body.cli_b@0` |
| (28, 7) | 1024, 264 | `body.sum@2` |
| — | 0, 0 | `background` |

**Seams: the cells touch.** Two checks, both with nothing else between the cells:

- Pixel column 639 is `body.cli_a` and column 640 is `body.sum`, at pixel rows 232, 264 and 296.
- Pixel row 359 is `body.cli_a` and row 360 is `body.cli_b`, at pixel columns 320, 448 and 576.

The same checks on the wired calculator would cross 8 layout units of membrane. Here they cross none.

**Counts.** Claude computed these with §2.5's exact integer rule, written independently in a script. The same script reproduces Phase 6's `edge 312` on the wired calculator.

| Viewport | agree | edge | owners |
|---|---|---|---|
| 640×360 | 230248 | 152 | 7/7 |
| 1000×777 | 776784 | 216 | 7/7 |
| 1280×720 | 921256 | 344 | 7/7 |
| 1920×1080 | 2073164 | 436 | 7/7 |

**Colors at the probes** (Phase 6 Amendment B2's method, on every adapter). A `.` means unchanged from the line above. Every event: surface `22262E`, pixel `0,0` `15171C`, alpha `FF`.

| After | cli_a | cli_b | sum | cli_a@0 | cli_b@0 | sum@2 |
|---|---|---|---|---|---|---|
| grow | `2F5D8A` | `2F5D8A` | `39414D` | `C9CED6` | `C9CED6` | `C9CED6` |
| event 1 `"two"` | `B03A2E` | . | . | `F2B134` | . | . |
| event 2 `"2"` | `2F5D8A` | . | . | . | . | . |
| event 3 `"3"` | . | . | `2F5D8A` | . | `F2B134` | `F2B134` |

### 2.11 The surface rename (C8)

A body's derived outline is its **surface**. The word *membrane* is kept only where it means a cell's boundary. In code and output:

| Before | After |
|---|---|
| `joinn_link::membrane` (∂(body)) | `joinn_link::surface` |
| `Owner::Membrane`, printed `body membrane` | `Owner::Surface`, printed `body surface` |
| `membrane` in `print_layout` | `surface` |
| rule 30 *A MEMBRANE IS ∂(BODY)*, rule 31 *…PORTS ON MEMBRANES ONLY* | Appendix A's wording |
| xtask identifiers and messages that mean a body's outline | `surface` |

**Kept exactly:**

- `refused at membrane: "two" is not in ℤ` in the transcript and `.desc` goldens. That refusal happens at `cli_a`'s membrane, which is a cell's, so it was right all along.
- `self@n` grammar.
- Every corpus byte.

**The only test expectations that may change** are strings where `membrane` named a body's outline. The Phase 6 layout block's first line and the four `"body membrane"` probe strings (joinn-visual, joinn-gpu, joinn-shell-desktop, xtask `g6_click`) become `surface`. P7-05 lists each one in its commit report. Anything else that changes is a snag.

### 2.12 The script

The same three events as Phase 6 §2.12, on `calculator.contact`, run through check, inject, run on the lowered body, then `apply_run` (or the refusal presentation):

1. `cli_a@0` ← `"two"`
2. `cli_a@0` ← `"2"`
3. `cli_b@0` ← `"3"`

`cargo xtask regrow` also runs Phase 6 Amendment A4's two events on a fresh contact calculator (`cli_b@0` ← `"x"`, then `cli_a@0` ← `"2"`). The second event's line must show the `+ 1`.

### 2.13 Phase 6's carry-forwards

- **V125: the shell's run path keeps V122.** Make Claude's Stop C scratch test permanent, in `joinn-shell-desktop`'s tests. It drives the shell's own leaves (click `cli_a@0`, type, Enter) for `two`, `2`, `3`, and after each Enter asserts that the session's tables equal `Scene::regrow` of its `BodyState`, byte for byte, with rows 2, 3, 4. After P7-13 the same test also runs on `calculator.contact`, with rows 2, 1, 3.
- **V132: device loss comes from wgpu.**
  - `joinn-gpu` registers `Device::set_device_lost_callback` when it opens a device. The callback sets a shared flag, and `Gpu::lost()` reads it.
  - The shell checks the flag at the start of every redraw. When it is set, the shell regrows: drop the renderer, reopen the adapter, `upload_all`, and print `regrow: device lost (<reason>), tables re-uploaded`.
  - The "two `Lost` surfaces in a row" guess is removed. `Lost` and `Outdated` only reconfigure the surface.
  - A joinn-gpu test opens every adapter, calls `device.destroy()`, polls, and asserts `lost()` is true with reason `Destroyed`. That half of R9 is closed.
- **Enter on an empty buffer does nothing.** With an address selected and an empty buffer, Enter prints `  (empty: nothing sent)`. No intent is made, no run happens, no tick is requested, and the selection stays. A unit test on the leaf covers it.

### 2.14 The catalogue for `.contact` (R60, rule 52)

The catalogue grows before any control points at a `.contact` file. `Subject::Contact(Contact)` joins `Subject`, with `parse_subject`, `reprint` and `reparse` for the kind.

| Mutation | Applies to | Does |
|---|---|---|
| `DropForce(response)` | Contact | removes the force |
| `SwapResponse(response, hash)` | Contact | re-pins the force's response to `hash` |
| `ShiftMember(response, member, port)` | Contact | moves one member to another port of the same instance |
| `DropMember(response, member)` | Contact | removes one member |
| `DropGenome(instance)` | Body, **Contact** | extended to contacts |
| `RenameAlias(from, to)` | Universe, **Contact** | extended: renames an instance or response everywhere in the file |

**Neutral edit** for Contact: one regulatory label (or name) gains ` (neutral)`, as for Body.

**Each mutation is tested by what the mutant does** (rule 52):

- `DropForce("sum")`: the body is admitted, and `sum` no longer exists.
- `SwapResponse("sum", mul)` (`12b6e545…`): refused at the pin.
- `ShiftMember("sum", "cli_a@1", 0)`: refused at the receptor, naming `cli_a@0`.
- `DropMember("sum", "cli_b@1")`: refused at arity.
- `DropGenome("cli_b")`: refused, member names no instance.
- `RenameAlias("sum", "total")`: admitted, and the canonical text changes only in that name. It hashes differently, because instance names are coding, as in `.body`.

### 2.15 Gate 7

Three items, in `xtask/src/fns/gate_seven_items.rs`, built like `gate_six_items.rs`. `"phase 7"` goes into `PHASE_LABELS` after `"phase 6"`, and `gates.lock` gains `phase 7: 3/3`, written by the lock writer. GPU parts run on every adapter and fail on none (rule 60).

| # | Item | `control_artifact` | `opposes` | Control answers `true` when |
|---|---|---|---|---|
| 1 | **Two forms, one truth** | `corpus/phase7/calculator.contact` | `SwapResponse("sum", "12b6e5458891b14aa74e43cee34b1184be04547fe58e035b1882125467120fa7")` | the subject is refused at admission, or its CLI transcript differs from `calculator.txt`, or `sum@2` after the script is not `5`. (Descriptions are compared in the item's checks, not in the control, because the kind's neutral edit changes `sum`'s label and must still answer `false`.) |
| 2 | **Combine is order-free, and it fits what it reaches** | `corpus/phase7/calculator.contact` | `ShiftMember("sum", "cli_a@1", 0)` | `check_contact` refuses the subject with a receptor refusal naming `cli_a@0` |
| 3 | **The body is drawn as cells in contact** | `corpus/phase7/calculator.contact` | `DropForce("sum")` | no pixel of the rendered ID image (1280×720) is owned by a force's response |

**Checks:**

1. **Two forms, one truth.**
   - `print_body(lower(c).coding)` equals `calculator.body`'s canonical text, and its hash is `b55fba1e…` (§2.5).
   - `joinn run corpus/phase7/calculator.contact` prints `corpus/transcripts/calculator.txt` byte for byte.
   - The test host's descriptions after the script equal `calculator.desc` (sum, after event 3) and `calculator_refusal.desc` (after event 1), byte for byte and by hash.
   - `contact_surface(c)` equals `surface(lower(c))`, and the two intent sets are equal (§2.6).
   - A contact with a `wires` section is refused with §2.3's wording.
2. **Combine is order-free, and it fits what it reaches.**
   - `check_register` passes on the register.
   - `order_free` refuses `mutant.difference` and `mutant.midpoint`, each with its counterexample, and passes `mutant.max` and `mutant.plus1`.
   - Every member→port order of `lower` gives `sum@2 = 5`.
   - The four spellings of §2.3 share hash `868e79b2…`.
   - The unopposed-register plant is refused.
3. **The body is drawn as cells in contact.** On every adapter, at every standard viewport:
   - every non-edge pixel's GPU owner equals `cpu_pick`, and `cpu_pick` equals `cpu_pick_reference`;
   - all 7 owners own a pixel, and **no pixel is owned by a link**;
   - §2.10's probes and seams hold at 1280×720;
   - the script, through `apply_run`, keeps tables equal to `regrow_contact` after every event, within the bound;
   - §2.10's colors hold after each event;
   - dropping and regrowing the device gives identical color and ID bytes.

Rule 41 holds: the three `(control_artifact, opposes)` pairs are new, and the uniqueness check extends to gate 7. Gate 6 still passes 3/3 on `calculator.body` (V28).

---

## 3. What xtask prints

| Command | Prints | Exit 1 when |
|---|---|---|
| `cargo xtask forces` | `combine ℤ 1 by cell:6b32…: order-free (64 pairs, 64 triples, seed 7), opposed by separate cell:6fcb… (turn 0 from {1 2})`; then one line per plant (§2.2); last line `forces: 1 combine registered, 1 opposed; plants: difference refused, midpoint refused, max and plus1 order-free but unregistered, unopposed refused (ok)` | the register fails, or any plant answers wrong |
| `cargo xtask contact` | for each `.contact` in the corpus: `<rel>: hash <hex>, admitted`; then the lowered canonical text between `--- lowered ---` and `--- end ---`; then `lowered = phase2/calculator.body (b55fba1e…)` or the first differing line; then `transcript equal`, `calculator.desc equal`, `calculator_refusal.desc equal`, `surface equal (3 ports)`, `pairings: 2, sum@2 = 5 in each`; then one line per catalogue mutant with its refusal or `admitted`; last line `contact: <n> body(ies); two forms, one truth` | any inequality, or a mutant answering against §2.14 |
| `cargo xtask roles <path>` | §2.7's block | the path isn't a `.contact`, or it is refused |
| `cargo xtask layout <path>` | accepts `.contact` (§2.8); `--all` includes `.contact` files | as Phase 6 |
| `cargo xtask pick` | adds, per adapter and viewport, `calculator.contact <w>x<h>: agree <a>, edge <e>, disagree 0, owners 7/7, links 0` | as Phase 6, or any link owner in a contact picture |
| `cargo xtask regrow` | adds the contact script: `contact event <n> <address> <term>: touched <instances>, rows <r> (bound <b>), bytes <y>, tables equal regrow`, the A4 pair, and per adapter `regrow contact <adapter>: color identical, ids identical` | as Phase 6 |

`cargo xtask floor` is unchanged: the Floor's pairs and count don't move.

---

## 4. The Commits

Done-when is a command, and the command must be able to fail.

### Chunk A: carry-forwards, the rename, the force register

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P7-01** | **Plan, rules, Part V** | Commit this plan. `AGENTS.md` per Appendix A. Commit the theory and doc edits already on disk, unchanged, **then** apply Appendix B on top: the Floor's R78 entry, backlog entries, `decisions.md` rows, the Part V status notes, and the roadmap note. No code | `git show --stat HEAD` lists this plan, `AGENTS.md`, `docs/Theory/JoInn Cells, Bodies and Forces.md`, `docs/Findings/the-floor.md`, `docs/Findings/decisions.md`, the backlog and the roadmap, plus any other doc `git status` showed as already edited on disk (paste that `git status` first). Nothing under `joinn/` except `AGENTS.md` |
| **P7-02** | **V125** | The shell run-path test (§2.13), on `calculator.body` | The test passes and prints rows `2, 3, 4`. Shown then reverted: skip `apply_run` in the shell's Enter leaf; paste the test's failure |
| **P7-03** | **V132** | Device-lost callback (§2.13) | The joinn-gpu test passes on every adapter; paste its line per adapter. `grep -rn "Lost" crates/joinn-shell-desktop/src` shows only the reconfigure path |
| **P7-04** | **Empty Enter** | §2.13's third item | Unit test passes: selected `cli_a@0`, empty buffer, Enter → `  (empty: nothing sent)`, no intent, no pending delta |
| **P7-05** | **Surface** | §2.11's rename | `cargo test --workspace` passes. `cargo xtask gate all` exits 0 with every phase line as before. `grep -rn "body membrane" crates xtask` prints nothing. `grep -rn "refused at membrane" corpus` still prints its 4 lines. The commit report lists every changed test expectation |
| **P7-06** | **Force register** | `joinn-prim::forces` (§2.2); `cargo xtask forces` with its plants; CI step `forces` after `regrow` | Whole output of `cargo xtask forces` pasted; last line as in §3 |
| — | **Stop A** | `phase-7-stop-a.md` (§0.2), push, stop | — |

Dependencies: in order. P7-02 to P7-04 touch only the shell and joinn-gpu. P7-06 needs P7-01 only.

### Chunk B: the file kind, its mutants, admission, running it, roles

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P7-07** | **`.contact`** | `joinn-dna::contact` (§2.3); `TAG_CONTACT`; `corpus/phase7/calculator.contact` exactly as §2.3; `hashes.txt` gains `# Phase 7. The calculator as a contact body. New file, no earlier hash moved.` and `calculator.contact 868e79b293c8d35625f514274eb0bf04d228898ed994a795b23b942a530b6209`; corpus walkers and `corpus_artifact` learn `phase7` and `.contact` | The four-spellings test passes. `cargo xtask corpus verify` prints 44 and ends ok. Each §2.3 parse refusal has a test. `git diff --stat <P7-01>..HEAD -- joinn/corpus` lists only `corpus/phase7/calculator.contact` and `corpus/hashes.txt` (one comment line and one row added) |
| **P7-08** | **Catalogue first** | §2.14: `Subject::Contact`, the four new mutations, the two extensions, neutral edit, each tested by what the mutant does | Tests pass. A test asserts no gate row names a `.contact` artifact yet |
| **P7-09** | **Admission and derivation** | `check_contact`, `contact_surface`, `lower` (§2.4–§2.6) | Tests: each §2.4 refusal with its wording; `print_body(lower(calculator.contact))` equals `calculator.body`'s canonical text and hash `b55fba1e…`; surface equality on every `.contact` in the corpus |
| **P7-10** | **Run it** | `joinn-cli`, `joinn-test-host` and the shell's loader accept `.contact` (parse → check → lower → the existing run path); the pairing test (§2.5); `cargo xtask contact` (§3); CI step `contact` after `forces` | Whole output of `cargo xtask contact` pasted; last line as in §3. `cargo run -p joinn-cli -- run corpus/phase7/calculator.contact` with the script's input on stdin prints the five transcript lines (paste them) |
| **P7-11** | **Roles** | `body_roles` (§2.7); `cargo xtask roles` | `cargo xtask roles corpus/phase7/calculator.contact` prints §2.7's block byte for byte (a test asserts it). The invariance, respond and store tests pass |
| — | **Stop B** | `phase-7-stop-b.md`, push, stop | — |

Dependencies: P7-07, then P7-08, then P7-09. P7-10 and P7-11 need P7-09. P7-09 needs P7-06.

### Chunk C: the picture, gate 7, freeze

| # | Commit | Delivers | Done when |
|---|---|---|---|
| **P7-12** | **Contact picture** | `layout_contact`, `grow_contact`, `regrow_contact`, latent bit and style 7 (§2.8–§2.9); `layout`, `pick` and `regrow` extended (§3) | `cargo xtask layout phase7/calculator.contact` prints §2.8's block byte for byte (a test asserts it). Whole outputs of `cargo xtask pick` and `cargo xtask regrow` pasted. Every contact line `disagree 0 … owners 7/7, links 0`, and every event line ends `tables equal regrow`. Gate 6 still 3/3 |
| **P7-13** | **The shell opens `.contact`** | `cargo run -p joinn-shell-desktop -- corpus/phase7/calculator.contact`; V125 extended to the contact file | V125 passes on both files, contact rows `2, 1, 3`. Cursor runs the window once on Windows, clicks the `sum` cell, and pastes the two pick lines (or writes `no display`) |
| **P7-14** | **Findings** | `docs/Findings/phase-7-contact.md`: every command's full output from P7-06 to P7-13, and a *Predictions* section marking each prediction in §2.3, §2.5, §2.7, §2.8, §2.9, §2.10 `as predicted` or `differs`, with both values | Every prediction is marked |
| **P7-15** | **Gate 7** | §2.15's three items; uniqueness through gate 7; `phase 7` label; lock row | `cargo xtask gate all` exits 0 and prints `phase 7: 3/3` after `phase 6: 3/3`. Shown then reverted, pasting each printed failure: (a) give item 2 item 3's `opposes` (uniqueness refusal); (b) make `lower` assign members in reverse order (item 1 must fail on the description; item 2 must still pass: order-free is about the result); (c) remove style 7 from the shader's `select` (item 3 must fail on the grow line's `sum` color) |
| **P7-16** | **Docs and freeze** | README and `Guides/03-where-we-are.md` (a body can be cells in contact; the calculator in both forms; what isn't built yet); `Guides/02-how-parts-connect.md` (contact inside a body, wires between bodies); `Guides/05-glossary.md` gains *contact body, force, combine, response, latent, surface port, role (protect, carry, store, respond), force register, lower*; `decisions.md` rows V125–V132 `holds`; the roadmap's Phase 7 note points at `phase-7-contact.md` | `cargo xtask gate all` from a fresh clone prints phases 0 … 6 and 7, and exits 0. `corpus verify` 44. `git diff --stat <P7-01>..HEAD -- joinn/corpus` lists only P7-07's two files |
| — | **Stop C** | `phase-7-stop-c.md` with the CI read, push, stop | — |

Dependencies: P7-12, then P7-13. P7-14 needs both. P7-15 needs P7-12. P7-16 needs P7-14 and P7-15.

**If something has to be cut for time:**

- Cut P7-11 (roles) and the chained fixture first, and carry R81's build forward with a line in `decisions.md`.
- Never cut P7-09's derivation check, gate 7 items 1 and 2, or P7-12's pick agreement.

---

## 5. Test Strategy

| # | Invariant | Commit |
|---|---|---|
| **V125** | The shell's own run path keeps V122 (tables equal regrow after every Enter) | P7-02, P7-13 |
| **V126** | A `.contact` file holds no wire: the parser refuses a wires section | P7-07 |
| **V127** | Member order is never hashed | P7-07 |
| **V128** | `lower` is a function of the contact, its cells and the register only, and `contact_surface(c) = surface(lower(c))` | P7-09 |
| **V129** | Every registered combine is order-free and opposed by a registered separate (a turn of its response) | P7-06 |
| **V130** | A force owns no pixel; a contact picture has no link owner | P7-12 |
| **V131** | Roles are derived and blind to the regulatory region | P7-11 |
| **V132** | Device loss is reported by wgpu's callback, and the shell regrows from it | P7-03 |

Carried forward and re-run on every commit: every earlier invariant. V28: every earlier gate still passes, and the wired calculator still draws with wires under gate 6.

**A test must never** assert only that something parsed, rendered, or ran. It asserts the bytes, the hash, the owner, the count, or the refusal's words.

---

## 6. Exit Gate 7

`cargo xtask gate 7`, from a fresh clone, after the harness fixtures pass. It needs at least one GPU adapter.

- [ ] **1 · Two forms, one truth.** *Opposes* `SwapResponse(sum, mul)`.
- [ ] **2 · Combine is order-free, and it fits what it reaches.** *Opposes* `ShiftMember(sum, cli_a@1, 0)`.
- [ ] **3 · The body is drawn as cells in contact.** *Opposes* `DropForce(sum)`.

### 6.1 Conditions for opening the next phase

1. Gate 7 passes; `gate all` exits 0; CI is green on Windows and Linux (read, not assumed).
2. `phase-7-contact.md` exists with every prediction marked.
3. AJ has run the window (§6.2) and said it worked, or each thing that looked wrong is a snag Claude has settled.
4. Claude's Stop C review lists no open snag, or AJ has chosen in conversation to carry each one forward.

The next phase is **the steel beam** (Part V §9): part names, source links (pinned), one *bears on* relationship, and the first store and respond cells. After that comes Phase 7.1 (charts, zoom, bands, links), replanned around bodies, forces and systems.

### 6.2 AJ's window check (Stop C, about two minutes)

In PowerShell:

```
cd D:\JoInn\joinn
cargo run -p joinn-shell-desktop -- corpus/phase7/calculator.contact
```

1. A window opens. On the left, two blue cells are stacked and touching. On the right, a taller **grey** cell touches both of them. There are no lines between the cells.
2. Click the middle of the grey cell. The terminal prints a `pick …: body.sum (cpu)` line, then a `body.sum (gpu) · agree` line.
3. Click the top-left cell's left port. It prints `selected body.cli_a@0`. Press Enter without typing. It prints `(empty: nothing sent)` and nothing changes.
4. Type `two` and press Enter. The top-left cell turns red and its port fills in.
5. Type `2` and press Enter. The red goes away. The grey cell stays grey, because it has only one number so far.
6. Click the bottom-left cell's left port, type `3`, and press Enter. The grey cell turns blue, and its right port fills in.
7. Resize the window. The picture re-centres, and the tick line says `rows 0`.
8. Close the window.

Tell Claude "window worked", or which step looked wrong.

---

## 7. Risks

| Risk | What to do |
|---|---|
| **Contact is wires in disguise** (the phase's adversary) | V128 bounds what `lower` may read, and the pairing test shows pairing doesn't matter. If `lower` needs anything else (a hand-given pairing, a position, an order), that is a finding for Part V C2, written in `phase-7-contact.md`, not a workaround |
| `lower` doesn't reproduce `calculator.body` byte for byte | A prediction that differs, not a failure. Paste the first differing line. The transcript and description checks still decide item 1 |
| `order_free` passes something it shouldn't, or the bound is too small to catch `midpoint` | Report the plant's line. Never raise the bound silently. A change to the bound is Claude's decision at the stop |
| wgpu's device-lost callback doesn't fire on some adapter after `destroy()` | A snag naming the adapter. Never fall back to the surface guess |
| The rename touches more expectations than §2.11 lists | Each extra one is a snag with the file and line. Never change a corpus byte or a `.desc` golden |
| A contact layout refuses something reasonable | That is R84, recorded, not fixed |
| The latent style changes a gate 6 color | A truth problem in the shader's style choice. Gate 6's colors must not move |
| Canonical text differs from §2.3's prediction in form (spacing, section order) | A snag; paste both. The hash follows the printed form Claude reviews at the stop, never the reverse |

---

## 8. Open Items

| ID | Topic | Question |
|---|---|---|
| **R82** | The opposite of a lossy combine | Sum forgets its parts, so separate can only be a turn given one part back. Structural combine (`build`, `pair`) is lossless, and its separate (`case`, `split`) is exact. Is every combine one of these two, and should the register say which? |
| **R83** | Multiplication | Is `a × b` combine *across* dimensions (length × length = area) while `a + b` is combine *within* one? If so, a product is the same act as `pair`, with a dimension instead of no frame (R38) |
| **R84** | Many members, chained forces | A fold over more members than the response has in-ports, and layout for members that don't touch in one column. The beam will need both |
| **R85** | One port, two forces | Can a value be combined in two places? Part V's reading is that values are copied, not split (R55), which makes it carry, not combine |
| **R86** | The two open Floor reductions | `pair`/`split` into a frameless combine (needs R38), and `join` as combine's completion rule. Attempt them in code when carry arrives |
| R55, R74, R75 | Reframed by Part V | Stay open. R74 may dissolve for contact bodies, since interior ports aren't addressed on screen. R75 becomes the beam's question: where a system wire meets a body |
| R64, R69 | Legacy gates; describe at firing time | Carried forward |

---

## Appendix A · `AGENTS.md` changes

Replace the header paragraph with:

```markdown
# JoInn — standing rules

This repo is JoInn. Phase 7 is contact bodies: a body is cells in contact, with
no wires inside, and an operation is a force from outside. Its one idea is A BODY
HAS NO WIRES; THE ENGINE DOES ITS OWN PLUMBING. The build plan is
docs/Plans/JoInn Phase 7 Implementation Plan.md. Work one CHUNK at a time (A, B
or C), one git commit per numbered step, and stop at the chunk's stop report.
Every decision is in the plan. Never stop to ask; follow the plan's Snags
section instead.

Only combine is written in this phase. .body is the legacy form and keeps its
hashes. There is no zoom, text, second body in a picture, or beam yet.
```

Rule 30 becomes:

```markdown
30. A SURFACE IS ∂(BODY) AND IS STORED NOWHERE. For a .body it is every port of
    every instance that no internal wire consumes; for a .contact it is every
    port no force reaches, plus each response's out-port. It is computed on
    demand, with its direction and frame. A genome entry whose cell is not
    supplied is refused, never skipped. Nothing in a .body, .contact or
    .universe file declares it. A MEMBRANE IS A CELL'S BOUNDARY ONLY.
```

Rule 31 becomes: `31. A LINK TOUCHES PORTS ON SURFACES ONLY.` (the rest of the rule unchanged).

Keep every other rule exactly as it is. Append:

```markdown
61. A BODY HAS NO WIRES. A .contact file has no wires section and its parser
    refuses one ("wires belong to systems"). The engine derives its own
    deliveries (`lower`) from the contact, its cells and the force register
    only; the derivation is never written to disk, described, or drawn.
62. AN OPERATION IS A FORCE, NOT A CELL. A force's response law lives with the
    frame it acts on, in joinn-prim::forces. A .contact pins the response by
    hash, and admission refuses a pin the register does not hold.
63. COMBINE IS ORDER-FREE, AND EVERY FORCE IS OPPOSED. A registered combine
    passes the order-free sample and has a registered separate that is a turn
    of its response. Member order is never hashed.
64. A FORCE OWNS NO PIXEL. It is seen only through its response. Interior ports
    are not drawn; a contact picture has no link rows.
65. ROLES ARE DERIVED, NEVER WRITTEN. A role is (faces out | in) × (holds |
    reacts): protect, carry, store, respond. There is no role list and no
    four-variant role enum. joinn-host's Role is the accessibility role and is
    a different thing.
66. THE ENGINE HEARS DEVICE LOSS FROM WGPU. The device-lost callback is the
    only signal of a lost device; a lost surface only reconfigures.
```

---

## Appendix B · Documents (P7-01 applies these after committing the on-disk edits)

### B1 · `docs/Findings/the-floor.md`

Replace the 29 Sep blockquote at the top with:

```markdown
> **29 Sep 2026 · re-read, not amended.** Part V's reading of the Floor as
> forces was tried by this document's own rule (R78). The attempts are recorded
> in §5. No pair left the Floor.
```

Append to §5:

```markdown
### 2026-09-29 · R78 attempted: the Floor re-read as forces · no amendment

**Why.** Part V (*Cells, Bodies and Forces*) proposed that the Floor sorts into
two opposed forces (combine / separate, carry / release), distinction, and the
membrane. The rule here says nothing changes until each reduction is tried.

**What was tried, and how it came out.**

| Pair | Tried as | Result |
|---|---|---|
| `build` / `case` | combine / separate in a frame | A regrouping, not a reduction. Stays |
| `pair` / `split` | combine / separate with no frame | Reduces only if a frameless combine exists (R38). Not attempted in code. Stays (R86) |
| `join` / `fan` | combine's completion rule; carry to many | `join` reads as when a combine completes; `fan` as carry of a value that is copied, not used up. Not attempted in code until carry exists. Stays (R86) |
| `eq` / `choose` | distinction; carry one, release the other | **Kept as a pair (AJ, 29 Sep).** `choose` is where distinction meets a force: a decision made and a decision acted on. Taking `choose` away would leave `eq` unopposed and the count odd |
| `grant` / `revoke` | carry / release of authority | Does not fit: release leaves a thing where it is, revoke brings it back. Stays as authority |
| `bind` / `unbind` | moves to systems | Stays in the Floor, in the system's register: systems are written in the Floor too |
| `bound` / `fill` | the membrane | Stays |
| `hash` / `resolve` | distinction by identity, and the store | Stays |

**Contact needs no new pair.** Cells that fill one boundary with no `bind`
between them are in contact. Phase 7's `.contact` grammar has no way to write a
`bind`, which is the test of this.

**Forces are opposed at the frame, not in the Floor.** Combine on ℤ is admitted
only with its separate, a turn of its response (Phase 7, `joinn-prim::forces`).
Separate for a lossy combine like sum is a turn given one part back; for a
lossless combine like `build` it is `case` (R82).

Standing rule unchanged: the Floor grows or shrinks only through an entry here
with a refuted reduction.
```

### B2 · `docs/Findings/decisions.md`

Change the rows:

| Row | New status | New note |
|---|---|---|
| R76 | decided | AJ 29 Sep: combine is order-free; − and ÷ are combine at a turn; presentation is the host's; other order is a system matter |
| R78 | attempted | Findings/the-floor.md §5, 29 Sep: no pair left; eq/choose kept paired (AJ) |
| R81 | decided | AJ 29 Sep: (faces out/in) × (holds/reacts) = protect, carry, store, respond; derived, never written |

Add rows `R82`–`R86`, phase `7`, status `open`.

### B3 · The research backlog

- Set **R76** to `decided` with B2's note.
- Set **R78** to `attempted` with a pointer to the Floor's §5.
- Set **R81** to `decided` with B2's note.
- Append R82–R86 with §8's text.
- Under R55, add: `Part V reading (Claude, 29 Sep, not decided): copied, not split.`

### B4 · `docs/Theory/JoInn Cells, Bodies and Forces.md`

Append after §15:

```markdown
## 16. After the decisions of 29 Sep (Draft 0.3 notes)

- **R76 decided** as proposed in §12.
- **R78 attempted, no amendment.** See `Findings/the-floor.md` §5. `eq` / `choose`
  stay a pair (AJ). Contact needs no new pair: it is cells filling one `bound`
  with no `bind`.
- **R81 decided.** Roles are (faces out | in) × (holds | reacts): protect, carry,
  store, respond. Derived from the body, never written.
- **§6.2 rule 2 revised (Claude's, open to AJ's veto).** Combine keeps its
  inputs, and the response appears beside them. Sum forgets its parts, so its
  separate can only be a turn given one part back, and destroying the inputs
  would leave combine without a working opposite (R82).
- **Every force must have its opposite on file.** Combine on ℤ is admitted with
  its separate, `sum_turn`. ℚ cannot combine in a contact body until it has a
  turn.
- **Phase 7 builds this for the calculator** (`Plans/JoInn Phase 7 Implementation
  Plan.md`). Numbering: this is Phase 7; the old Phase 7 becomes Phase 7.1.
```

### B5 · The roadmap

Replace the *29 Sep 2026 · Phase 7 is on hold for replanning* note with:

```markdown
> **29 Sep 2026 · Phase 7 is contact bodies.** After Part V, the next phase makes
> a body cells in contact with no wires, and an operation a force (combine):
> `Plans/JoInn Phase 7 Implementation Plan.md`. The section below, charts, zoom,
> bands and links, is now **Phase 7.1**, to be replanned around bodies, forces
> and systems after the steel beam. Every later phase keeps its number.
```

Retitle the next section from `#### Phase 7 · Visual Host II: charts, zoom, bands, links` to `#### Phase 7.1 · Visual Host II: charts, zoom, bands, links`.

---

*JoInn Phase 7 Implementation Plan, Draft 0.1 (29 Sep 2026). Opens after Phase 6's Stop C review and Part V. AJ decided on 29 Sep: the scope (contact bodies, with the beam next); R76 as proposed; `eq`/`choose` stay paired; roles as two opposites and four roles. Claude decided, open to AJ's veto before chunk A: the numbering, the `.contact` name, combine keeps its inputs, forces opposed at the frame, the response pinned by hash, the engine's own derivation, a force owns no pixel, and empty Enter does nothing. Every other decision is made here. Cursor executes. Claude verifies at each stop. AJ runs the window at Stop C.*
