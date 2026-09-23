# What is JoInn?

## The short answer

JoInn is a platform for **building applications by composing small visual blocks**, not by wiring up many separate tools (one for screens, one for storage, one for checks, and so on).

Those blocks are called **cells**. Think of them like Lego: each piece has a clear shape, a clear job, and only fits in certain ways.

Every cell also has a **plan** (called **DNA**) that says what it is, what it accepts, and what must stay true about it. The platform’s big bet is **truth first**: grow what is correct; refuse what is not.

## What JoInn wants to be

Long-term, JoInn aims to be:

- a **visual** app creator (you see and touch what you’re building)
- **fast** when you run things, and able to **compile** into real production code
- **shareable**: people publish reusable designs (**blueprints**) that others can see and reuse — a picture you can trust, not only text you have to decode

Today the running demo is still early: a **calculator** you run in a terminal. That is on purpose. The plan is to prove the universe is correct **before** building the fancy visual host.

## The three promises

JoInn aims to be:

1. **Easy** — someone who isn’t a professional programmer can still build  
2. **Robust** — what you build stays correct as it grows  
3. **Powerful** — you can build serious apps, not only toys  

Those three usually fight each other. JoInn’s answer is: **make robustness mechanical** (checks that refuse bad growth), so ease is safer — you’re composing pieces the system has already judged.

## Six ideas in plain words

You don’t need to memorize these. They’re the “why” behind the design.

1. **Opposition** — every important claim has a counter-check (like addition and subtraction, or “accept” and “refuse”). Validation is not an optional add-on.  
2. **Path of truth** — you grow by adding what’s true; what was true stays true.  
3. **Semantic zoom** — zoomed out, apps look similar (nodes and connections); zoomed in, the insides can be totally different.  
4. **Consistent containers** — the same kinds of “boxes” (cell, body, and larger groupings) behave the same way across the platform.  
5. **Identity follows content** — something’s identity comes from what it truly *is* (the checked plan), not from a nickname someone typed.  
6. **Two engines, one truth** — a live engine for instant feedback and a compiler for speed must agree. Each is a check on the other.

## Cells, bodies, and DNA (everyday picture)

| Word | Everyday meaning |
|------|------------------|
| **Cell** | One building block you can show and use |
| **DNA** | The plan and rules for that block |
| **Body** | A cluster of cells that belong together (like one small app or one machine) |
| **Gate** | The checker that admits good growth and refuses bad growth |
| **Host** | The thing that talks to the outside world (terminal today; screen later) |

Analogy: DNA is the **recipe and the rules**; the gate is the **health inspector**; the host is the **storefront** where a person meets the app.

## Why “truth before pixels”?

A beautiful editor with nothing solid underneath becomes a pretty shell that slowly bends the rules to match the pictures.

JoInn builds the opposite way: **get a correct, checkable universe first** (even if you only see it as text), then draw it. The calculator demo — including refusing the word `"two"` when a number is required — is that idea in miniature.

## Next

See **how the pieces connect** (diagrams): [02-how-parts-connect.md](02-how-parts-connect.md).

When you’re ready for the full theory paper: [../Theory/JoInn Architecture and Theory.md](../Theory/JoInn Architecture and Theory.md).
