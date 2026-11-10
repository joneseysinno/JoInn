# JoInn Hypernode Refactor Plan

**From the product language to a person building an app**

*A general execution plan · 30 October 2026*

> **Status.** 30 Oct 2026. AJ’s direction from the vision conversation. Phase A is closed. Current phase: **B**. This plan is general. It names what each phase proves and what would kill it. It is not a task list.
>
> Pixels and compilers follow a canonical form. The visual editor is a view over that form.
>
> Code names (`body`, `contact`, `universe`, `strand`) stay until a phase whose job is the rename. No phase below renames them. This plan does not rename them by itself.

The guides are the product language: [What is JoInn?](../Guides/01-what-is-joinn.md). Storage properties already checked are in [JoInn Graph Storage](../Theory/JoInn%20Graph%20Storage.md). Older theory papers remain the decision record. Findings stay as written.

Each phase has a goal, what it proves, an exit a machine or a person can fail, and an adversary: the observation that would kill or reshape it.

---

## Already true, and kept green

Later phases keep these gates green. A phase that breaks one of them has failed, whatever else it draws.

| Already true | Where it shows |
|--------------|----------------|
| A refusal is a value | The calculator refuses `"two"` and continues |
| One owner per pixel | Two pickers name the same owner |
| The camera is exact | Eight notches in and out return the same camera |
| A hyperedge touches and does not cross | Routes on gutters; the link check prints no crossings |
| Growth is not identity | A system’s hash is the same at every size |
| The partner strand reads the opposite | Gate 8’s reader |
| The pool refuses cycles | Gate 8’s pool item |

The historical phase record is [Where we are](../Guides/03-where-we-are.md). This plan starts from that record. It does not reschedule a gate that already passes.

---

## Phase A — Language

**Closed, 30 Oct 2026.**

**Goal.** Write the product language down where a new reader will see it: hypernode, relationship, wave, app, and DNA as storage.

**Proves.** A person can learn the product from the guides.

**Exit.** A new reader can say what a hypernode, a relationship, a wave, and an app are, and where DNA fits, without opening a theory paper.

**Adversary.** The reader still has to open a theory paper to answer those five questions, or the guides still describe DNA as the screen a person builds on.

This phase is the documentation pass dated with this plan. It leaves every code name in place.

**Exit held.** Each answer is in [What is JoInn?](../Guides/01-what-is-joinn.md), so a reader does not need the storage paper:

| Question | Where the guide answers it |
|----------|----------------------------|
| What is a hypernode? | The short answer: the node you can open, blocks in contact inside, one node from outside |
| What is a relationship? | The short answer: a join that can touch many nodes at once; a force is its directed form |
| What is a wave? | The short answer: a relationship that reaches every part of a hypernode willing to accept the signal |
| What is the app? | The short answer: the outermost hypernode |
| Where does DNA fit? | The short answer: the stored form of a hypernode, studied from DNA; a person builds on blocks and relationships |

The agent header in `joinn/AGENTS.md` now names this plan as the active one and calls DNA words storage. The phase record in [Where we are](../Guides/03-where-we-are.md) points at Phase B as the next build.

---

## Phase B — One running hypernode

**Goal.** Make the contact calculator the single product example. The contact file is the source. `lower` is compilation into the runner. The wired body is the compilation witness.

**Proves.** There is one explanation of the calculator, and the wired form witnesses that compilation.

Gate 7 already shows the contact calculator and the wired calculator match: same transcript, same descriptions, same surface. This phase does not re-prove that match. It makes the match the product’s one story, and it keeps the wired file as the witness.

**Exit.** One explanation of the calculator, in the guides and in the code’s behavior. The wired form is a witness of compilation. The two forms still match.

**Adversary.** The two forms diverge.

**Rules, scheduled here and edited later.** `joinn/AGENTS.md` rule 32 says that inside a body is a wire. Rule 61 says that a body has no wires. Both stand today. The implementation work of this phase resolves that contradiction in the rules when the code has one source of truth: the contact file, compiled by `lower`, witnessed by the wired body. This plan only schedules that edit. It does not make it.

**Implementation plan:** [JoInn Phase B Implementation Plan](JoInn%20Phase%20B%20Implementation%20Plan.md).

---

## Phase C — Open the hypernode

**Goal.** The window shows the contact calculator as blocks a person can read as blocks: ports, contact, and the directed relationship drawn as the lasso. Editing can remain text-backed.

**Proves.** The picture is the same canonical form the live engine runs.

Today the contact calculator is drawn as cells in contact, and its force owns no pixel. A system already draws a force as a lasso. This phase brings that drawing to the calculator hypernode itself.

**Exit.** The window shows the form the live engine runs. A click still names an owner.

**Adversary.** The picture drifts from the tables.

**Implementation plan:** [JoInn Phase C Implementation Plan](JoInn%20Phase%20C%20Implementation%20Plan.md).

---

## Phase D — One relationship

**Goal.** Link, force, and later wave are one connector, with different ways of choosing members and of carrying a direction. Forces stay directed hyperedges, as Part VII already decided (G6 in [JoInn Systems, Forces and Growth](../Theory/JoInn%20Systems,%20Forces%20and%20Growth.md)): a force surrounds what it acts on and points at the result.

**Proves.** A force and a link are both relationships, and they stay distinct. On screen they look different: a force is a lasso, a link is a routed line (Part VII G19). A click prints different owner names: `force <name>` for a force, `link <id>` for a link.

**Exit.** A document and a gate story in which a force and a link are both relationships, and a click names the owner the other picker names.

**Adversary.** Unifying them erases a pixel owner, and the pickers disagree on that pixel.

Wave membership itself is Phase F. This phase only keeps a place for it on the same connector.

**Implementation plan:** [JoInn Phase D Implementation Plan](JoInn%20Phase%20D%20Implementation%20Plan.md).

---

## Phase E — Depth

**This is the expensive phase.**

**Goal.** Galaxy, system, body, and universe become depths of one hypernode tree. The app is the root. A galaxy is a zoom depth.

The camera, one-owner pixels, and “a folded node is touched once” are the contraction. They are already tested on fixed levels: universe, galaxy, and system. This phase generalizes depth. The code may keep those names as the names of depths. Renaming the identifiers waits for a phase whose job is the rename.

**Proves.** Folding works past the three levels the camera has today.

**Exit.** Folding a hypernode at more than the current three fold states still touches each relationship once.

**Adversary.** The layout assumed exactly three fold states — universe, galaxy, and system — and a deeper tree breaks the cut. **Band** stays the size-based drawing (dot, surface, cells, full). A fold state is which depth the camera has contracted.

---

## Phase F — Waves

**Goal.** A wave is a relationship whose members are derived: every receptor in that hypernode that accepts the signal. A receptor may refuse. Membership is a pure function of the hypernode.

**Proves.** A wave changes only the receptors that accept it.

**Exit.** A wave offered to a system changes only the receptors that accept it. A planted receptor that should refuse is unchanged.

**Adversary.** The wave becomes an undeclared tunnel through every boundary.

Waves are not in the window today. This phase is where they are built.

---

## Phase G — Storage is the canonical form

**Goal.** A hypernode’s interior can be written as the Phase 8 strand encoding and read back into the same blocks the live engine runs. The calculator, stored and read back, matches today’s contact transcript and witnesses. Picture and labels stay out of the hash.

**Proves.** The strand and the contact file are one calculator.

Phase 8 reads strands, draws them two ways, and refuses a bad codon. The reader lists stored names and the picture tints by them. A strand does not yet run the calculator. This phase is that join.

**Exit.** Byte-level or transcript-level agreement between the strand-backed hypernode and the contact calculator.

**Adversary.** The strand means a different calculator than the contact file.

---

## Phase H — Compile

**Goal.** A second engine reads the same canonical form and produces native code. From the moment it exists, its results match the live engine, including refusals.

**Proves.** Two engines, one hypernode.

**Exit.** Same inputs, same outputs, both engines.

**Adversary.** The compiler is trusted before the comparison exists.

The compiler is not in the tree today. The comparison ships with the second engine.

---

## Phase I — Ease

**Goal.** A person who does not program builds a small app from blocks. The counting and adding story is the candidate. The session is uncoached, and the time is bounded in advance.

**Proves.** The structure a gate can check is a structure a person can use.

**Exit.** A recorded session meets a pre-declared pass. This gate cannot be fully automated.

**Adversary.** The structure is true, and the person still cannot build.

---

## What this plan leaves alone

- Phase 8 crates stay: `joinn-strand`, `joinn-codex`, `joinn-tower`.
- Findings stay as the record of what was measured. This plan does not rewrite them.
- Phase A writes documents. It does not rename code.
- Waves are Phase F. The compiler is Phase H. This plan describes both as work ahead.
- The wired calculator stays a compilation witness through Phase B. Retiring it as a second product is later than the match gate 7 already holds.

## Where to read

- [Guides](../Guides/README.md)
- [JoInn Graph Storage](../Theory/JoInn%20Graph%20Storage.md)
- [Where we are](../Guides/03-where-we-are.md)
