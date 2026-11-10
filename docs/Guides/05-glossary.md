# Glossary (plain English)

Short definitions for words you’ll see in JoInn docs. Deeper meanings live in Theory. This page gives each word one home.

Product language, as of 30 Oct 2026: hypernode, relationship, wave, and app. DNA words are storage.

## The product

| Term | Plain meaning |
|------|----------------|
| **Block** | The piece a person snaps. It meets other pieces only at its ports. In the code today a block is a cell. |
| **Hypernode** | Blocks in contact. You edit them as blocks. The engine runs them, and a compiler will later read them, as one unit. From outside, zoom draws them as one node. The contact body is the hypernode the engine runs today. |
| **Relationship** | A join that can touch many hypernodes. The window draws one as a link (a hyperedge). |
| **Wave** | A relationship whose members are every part of that hypernode that accepts the signal. A part may refuse. Building a wave is ahead. |
| **App** | The outermost hypernode. The universe on screen is a drawing of an app. |
| **Opposition** | Every join has an opposite that can refuse it or undo it. |
| **Were** | What was admitted: witnesses and lineage. This is part of identity. |
| **Are** | What is happening now: live values and the picture. Visible. The hash leaves the present out. |
| **Will be** | What the laws say will be accepted or refused. This is part of identity, with the past. |
| **Surface** | The hypernode’s derived ports: every place a relationship can meet it. Derived when needed. A file never declares it. |
| **Refusal** | A normal value meaning “no”. The calculator’s answer to the word `"two"` is a refusal. |

## Storage

These words belong to the storage experiment: letters, the reader, and the derived picture. The product story is the table above. Detail: [JoInn Graph Storage](../Theory/JoInn%20Graph%20Storage.md).

| Term | Plain meaning |
|------|----------------|
| **Letter** | One of four symbols, two bits. It stores a choice. A letter alone has no meaning. |
| **Strand** | A row of letters, read head to tail. Storage of a hypernode. It is read and drawn. Running the calculator from a strand is ahead. |
| **Partner strand** | Every letter’s partner, in reverse. The low bit flips, and the reader yields the opposite chain. |
| **Letter partner** | The other letter after a flip of the low bit: A with T, G with C. |
| **Codon** | Three letters. The reader looks the codon up in the codex. |
| **Codex** | The table where meaning lives. A strand file names the codex. The meanings sit in the reader. A reserved codon is a refusal. |
| **Reader version** | The reader’s own version (`reader 8.0`), kept apart from the codex. |
| **Reading frame** | Letters grouped in threes from a start codon. The laws a value obeys are a **frame** in the drawing-and-engine table. |
| **Amino acid** | A named entry in the codex. The picture tints a codon by that name. |
| **Protein** | A slice of those names from one start to the next stop. The reader lists the slice. Running it as floor operations is later work. |
| **Chromosome** | A strand plus references to other chromosomes. Its hash is the strand’s hash. |
| **Gene** | A hash reference from one chromosome to another. The pool reads it when asked. |
| **Gene pool** | The store of chromosomes by hash. It refuses a chromosome that contains itself. |
| **Tower** | The Hilbert curve a chromosome is laid on. Positions and levels are derived. The hash is the strand of letters. |
| **Level** | The curve filter. Keeping segments at or above a level leaves the curve one order down. The Phase 8 record calls this filter a floor. In these guides the word **floor** is the sealed operations only. |
| **Joint** | One of the three segments at the top level of the curve. They are the three joints between the four quarters. The middle joint is the centre. The beam’s **bridge** is a different word. |
| **Silent slot** | A segment past the last codon. It takes no bytes, and it is not a null. |
| **Hash** | Identity. For a strand, the hash of its letters. A chromosome uses that same hash. |
| **Order (tower)** | The Hilbert size: the smallest curve with enough segments. A declared link sequence is **order** in the next table. **Order-blind** is the force law. |
| **Sense / partner** | Which way a strand runs, read from its head codon. The beam’s **side** is a different word. |
| **Organelle** | A device the codex lists: screen, input, store, clock. The list is printed. The devices are ahead. |
| **DNA** | Storage of a hypernode: four symbols, one spelling for each meaning, an opposite in the bytes, references by hash. The crate `joinn-dna` is also the text-plan crate for today’s files. |

## Drawing and the engine

Words for pixels, owners, and the runner that exist today.

| Term | Plain meaning |
|------|----------------|
| **Allele** | One candidate way to carry out a block’s plan. The plan’s identity is the laws. The allele is the payload that has to pass them. The gate admits it. |
| **Assay** | A measurement of shape, without running the body or universe: loops a value can travel, and which of them something closes. It reports. In the assay, a **region** is one connected piece of a body’s inside. |
| **Band** | How a body is drawn, chosen by size on screen: a dot below 4 px, then the surface, then cells and surface ports from 32 px, then everything with text from 240 px. Whole-number tests, the same on the CPU and the GPU. |
| **Bridge** | On a beam, a law that cannot be derived, read from a pinned edition. Deflection needs one. The three top-level curve segments are **joints**, in the storage table. |
| **Camera** | How layout units become pixels. Exact since Phase 7.2: a zoom level and step, a focus, a pin, and the window size. A pan or zoom writes the tick. |
| **Chart** | A coordinate system with a whole-unit origin in its parent: universe, then galaxy, system, and body, in today’s named levels. |
| **Corpus** | Golden examples and deliberate fakes, replayed as testimony. |
| **Count witness** | A second derivation of a growing system’s answer, counting by one. After every growth step it matches the engine. |
| **Cut** | What the CPU decides is drawn under one camera. The GPU makes the same whole-number decisions. The owners in its picture are the cut’s owners. |
| **Declaration** | A sentence in the hashed part that the assay must satisfy. Today: `assert H₁ = 0`. If it fails, admission refuses. |
| **Evolution check** | A child keeps every witness of its parent and accepts an input the parent refuses. |
| **Floor** | The sealed opposed operations JoInn is written in. A candidate is admitted when it is irreducible and opposed. The curve filter is a **level**. |
| **Focus** | The point the camera holds still. Zooming leaves it where it is. |
| **Fold state** | Which charts are drawn as lens nodes: open, systems folded, or galaxies folded. Today a universe has at most these three. Each link has one route per fold state, computed when the universe opens. |
| **Force** | A directed relationship. It names members and a response. In a system the window draws it as a lasso. In the contact calculator the force owns no pixel; drawing it there is ahead. |
| **Frame** | The laws a value obeys. Example: integers. |
| **Frame (the F key)** | Fits the camera to what is open. This is a camera command. The laws a value obeys are a **frame**, above. |
| **Gate** | The checker that admits growth and refuses bad growth. |
| **Grove** | A generated test universe, seed 7: 8 galaxies, 128 systems, 3072 bodies, 137 links. Grown where it is needed, and never stored in the corpus. |
| **Grown state** | The cells a system has grown so far. Growth changes this. The system’s hash stays the same at every size. |
| **Grows** | The line in a contact file that says how the body grows and what it accepts. |
| **Gutter** | An empty lane of the grid between bodies, systems, or galaxies. Links travel on gutters. |
| **Host** | Whatever presents the app to the outside: terminal, test harness, or desktop window. A host is proved by there being more than one. |
| **Knot** | Where an unordered link’s legs meet. |
| **Lasso** | How a force is drawn in a system: a loop around what it acts on, a neck, and an arrowhead pointing at the result. A click prints `force <name>`. A routed link uses a different owner name. |
| **Layout unit** | The integer length the tables use. Pixels are what the camera makes of them. |
| **Leg** | The street path from a link’s knot to one member. A click prints `link <id> member <i>`. |
| **Lens node** | A system or galaxy drawn as one node because it is too small to open. |
| **Link form** | How a drawn relationship looks. Under 240 px it is a pale wide lane (the small link form; the Phase 7.3 record calls it a region). From 240 px, thin lines meet at a knot. From 1920 px, shared stretches thicken. A declared sequence is drawn in that sequence, with arrowheads, at every size. The assay’s region is a separate word: see **assay**. |
| **Lower** | Compilation of a contact hypernode into the deliveries the live engine runs. Inputs: the contact, its cells, and the force register. |
| **Membrane** | A block’s edge. In the code, the boundary of a cell. A hypernode’s outline is its **surface**. |
| **Order (link)** | A declared sequence of members. The window draws that sequence only when the link declares it. Hilbert size is **order** in the storage table. |
| **Order-blind** | A force law: the result does not depend on the order the members were combined in. The order still happened, and JoInn records it. |
| **Owner** | The one thing a pixel belongs to. Owners the pickers name include the background, a galaxy or system frame, the surface, a cell, a port, a wire, a link (and a leg or stub as `link <id> member <i>`), and a force’s lasso (`force <name>`). |
| **Pair** | A floor operation, sealed with its opposite. |
| **Pair (beam)** | The two quantity partners across one mirror on a beam, such as force and displacement. |
| **Pin** | The whole pixel the focus sits on. |
| **Rebase** | Moving the camera’s anchor to the chart under the centre pixel. Chart rows change. Pixels stay. |
| **Seal** | A locked pair the checks hold: a reference body, and a counterfeit that must be caught. |
| **Region** | The small link form: under 240 px, a pale wide lane joining the members. In the assay, the same word means one connected piece of a body’s inside. See **assay**. |
| **Drawing shader** | A small program that draws one kind of shape from the tables. The codex **organelle** list is storage, and those devices are not built yet. |
| **Side** | On a beam, which side of the mirror a quantity is on: placement, source, or their product, energy. Strand direction is **sense / partner**. |
| **Stub** | The short line from a member’s port out to its gutter. |
| **Surface port** | One port on a hypernode’s surface. The calculator’s are `cli_a@0`, `cli_b@0`, and `sum@2`. |
| **Tick** | One redraw. It carries the camera. An idle scene draws nothing, so the window waits. |
| **Trunk** | A stretch of street that two or more legs of one link share. A click prints `link <id>`. |
| **Waiting box** | The empty cell beside a growing body, where the next input goes. |
| **Wire** | A pixel owner in the wired compilation (`body wire …`). The contact source has blocks in contact. The wired file is the witness of `lower`'s compilation. |
| **Witness** | A recorded true result that later versions still satisfy. The gate can replay it. |

## Names the code still uses

| Code name today | Product word |
|-----------------|--------------|
| cell | block |
| membrane | the block’s edge |
| surface | the hypernode’s derived ports |
| contact body | the hypernode the engine runs |
| link, hyperedge | relationship, drawn |
| force, lasso | a directed relationship |
| universe on screen | a drawing of an app |
| body, system, galaxy, universe | named levels; the refactor plan makes them depths of one tree |
| strand, codex, tower | storage, and the tower is its picture |
| floor, in the Phase 8 record’s curve filter | level |
| three bridges, in the Phase 8 record | three joints |

## Next

Back to the start: [README.md](README.md).

> **30 Oct 2026.** Product language is hypernode, relationship, wave, and app. DNA words are storage. Earlier notes on this page (29 Sep, 30 Sep, 5 Oct, 6 Oct, 12 Oct) recorded words as each phase added them. The tables above give each word one home.
