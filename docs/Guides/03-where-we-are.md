# Where we are

## The three ladders (bootstraps)

JoInn is being built in three big stages. Think of them as ladders: each one lets you climb farther.

| Stage | Nickname | What it means in plain words | Status (roughly) |
|-------|----------|------------------------------|------------------|
| **B0** | True but unseen | The universe is correct and checkable, but you meet it mainly as text / terminal | **In progress — this is today** |
| **B1** | Seen and touched | Same universe drawn, picked, zoomed; CPU and GPU must agree | Future |
| **B2** | Self-editing | You build JoInn *inside* JoInn; the visual creator and the text form match byte-for-byte | Future |

The rule of the road: **pixels come after truth.** A gorgeous editor with a weak core would force the core to bend. So Bootstrap 0 comes first on purpose.

## What already works (today)

You can, from the `joinn` code folder:

- Run the **calculator** demo in a terminal (`joinn run calculator`)
- Run the **universe** (`joinn run universe`): the calculator's transcript, then `factor: 12` and `5 ft = 60 in`
- See a **refusal** when input isn’t a real integer (`"two"`), then a correct `2 + 3 = 5`
- Run machines that check vocabulary, the floor, agreement, gates, and the corpus (see the code README)
- A gate control reads the parsed body, universe, or transcript, and flips when that artifact is damaged in a way that still parses

Under the hood, the project has already built the early “truth core”: frames and values, DNA plans, the gate, the floor of sealed basics, the live runner, and the start of **hosts** (a host is proved by there being two — CLI and a test host).

There is **not** yet a full visual editor, GPU face, or “build any app by clicking” experience. That’s Bootstrap 1 and 2.

## What “Phase” language means

Builders and coding agents talk in **phases** (Phase 0 paper, Phase 1 truth core, Phase 2 floor + live engine, Phase 3 hosts, then later visual and creator phases).

You don’t need the phase numbers to understand the project. Use this translation:

- **Early phases** ≈ Bootstrap 0 (true but unseen)  
- **Middle phases** ≈ Bootstrap 1 (seen and touched)  
- **Late phases** ≈ Bootstrap 2 (self-editing)

If an agent mentions “Phase 3,” it means: proving hosts properly (description as a value; more than one host), still without the big visual product.

## Why stop at “unseen” for a while?

Because JoInn’s claim is **truth first**. The calculator transcript — including the refusal — is evidence that the system can say “no” and still compute when the input is honest.

When the visual host arrives, it should be a **view over the same truth**, not a second competing reality.

## Next

Try the demo yourself: [04-try-the-calculator.md](04-try-the-calculator.md).

Full build order (detailed): [../Plans/JoInn Build Roadmap.md](../Plans/JoInn Build Roadmap.md).
