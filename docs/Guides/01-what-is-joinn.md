# What is JoInn?

## The short answer

JoInn is a way to build an app by zooming a picture of nodes and relationships. Zoomed out, the app is one node. Zoomed in, that node opens into smaller nodes joined the same way. Open a node all the way and you are snapping blocks together. The blocks compile. What you built is the app.

That is the product. The calculator is the part that runs today. Snapping blocks in an editor a stranger can use, waves, and a native compile are ahead. This page says what the words mean. [Where we are](03-where-we-are.md) says which part already runs.

A **hypernode** is that node you can open. Inside it, blocks meet at their edges. From outside, it is one node, and the lines you see are the relationships that cross its boundary. A **relationship** can join many nodes at once. A **wave** is a relationship that reaches every part of a hypernode willing to accept the signal. The **app** is the outermost hypernode.

Two rules. Everything important has an opposite that can undo it or refuse it. A thing is true when its past, its present, and its laws agree: what it has already done, what it is doing, and what it will accept next.

The stored form of a hypernode is a small code, studied from DNA: four symbols, one spelling for each meaning, an opposite you can derive from the bytes, and references by identity. That code is how JoInn keeps a graph robust. A person builds on the picture of blocks and relationships. The storage paper is [JoInn Graph Storage](../Theory/JoInn%20Graph%20Storage.md).

## The three promises

JoInn aims to be:

1. **Easy** — a person builds on a canvas of blocks, snapping them where their ports meet
2. **Robust** — refusal is mechanical: bad growth is answered with a refusal, and the answer is a value the rest of the app can see
3. **Powerful** — the same hypernode can later compile to native code, and that compiler has to agree with the live engine

Those three usually fight each other. JoInn’s answer is to make robustness mechanical, so the blocks a person snaps are pieces the system has already judged.

## The two rules, a little slower

**Opposition.** Every join has an opposite. The opposite can refuse the join, or undo it. A reserved spelling in storage is a refusal. A force on integers has its opposite on file.

**Truth across time.** Three times have to agree:

| Time | What it is | Where it lives |
|------|------------|----------------|
| **Were** | What was admitted: witnesses and lineage | Identity |
| **Are** | What is happening now: live values and the picture | Visible. The hash leaves the present out |
| **Will be** | What the laws say will be accepted or refused | Identity, with the past |

Identity is the past and the laws. The present is the picture and the values you see while the app runs.

## How this sits on the theory already built

Inside a hypernode you edit by contact: blocks meet at their edges. The stored meaning of that contact is a relationship. Zoom draws the hypernode as a node.

Containment is a tree. Relationships may cycle. A reference to another hypernode is by hash, and it is loaded when something asks. A hypernode that contains itself through those references is refused.

A system seen from outside is a hypernode. A galaxy is a zoom depth. The app is the root hypernode. Today’s code still names the levels cell, body, system, galaxy, and universe. The [refactor plan](../Plans/JoInn%20Hypernode%20Refactor%20Plan.md) migrates those names to depths. The names stay in the code until that work.

| In the product | In the code today |
|----------------|-------------------|
| Block | cell |
| A block’s edge | membrane |
| The hypernode’s derived ports | surface |
| The hypernode the engine runs | contact body |
| Relationship, drawn | link, a hyperedge |
| Directed relationship | force; in a system, drawn as a lasso |
| A drawing of an app | the universe on screen |
| Storage | strand |

The contact file is the source a person would edit. `lower` is the compilation of that hypernode into the deliveries the live engine runs. The wired `.body` remains a compilation target. A later phase retires it as a second product. Gate 7 already shows the two calculator forms match.

## Why truth before pixels?

A picture is drawn from tables the engine already trusts. The window is that picture: zoom, pick, and read what the tables say. The calculator demo, including the refusal of the word `"two"` when a number is required, is the same idea in miniature.

The live engine runs a hypernode now. A compiler that reads the same form and produces native code is later work. From the moment it exists, its results have to match the live engine, including refusals.

## What runs today

The calculator runs, in the terminal and in a window. It is the first hypernode: blocks in contact, and a directed relationship that points at the result. You can see a refusal, then a true sum.

The window draws a universe, zooms it exactly, and draws links. A counting system grows. A strand is read and drawn two ways. The tower, the gene pool, and gate 8 are in place.

Ahead of today: an editor a stranger can use to snap blocks, waves, a native compile, and one tree of arbitrary depth. The camera folds three fixed levels (universe, galaxy, system). The strand stores and draws. Running the calculator from a strand is ahead.

## Next

See how the pieces connect: [02-how-parts-connect.md](02-how-parts-connect.md).

Storage, in more detail: [JoInn Graph Storage](../Theory/JoInn%20Graph%20Storage.md).

What already runs, phase by phase: [03-where-we-are.md](03-where-we-are.md).
