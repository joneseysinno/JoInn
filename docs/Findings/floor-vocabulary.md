# Floor vocabulary — the correction of 2026-09-18

Why the completed plans say "minimal set" and the current documents do not.

---

## The change in one line

> **"The minimal primitive set" is replaced by "the floor," and the admission test is replaced by a rule: a primitive is irreducible and opposed.**

`docs/Findings/the-floor.md` is the normative statement. `minimal-primitives.md` is retired, not edited, and this file records what moved so that an older document can be read without being trusted.

## Why the old wording was a problem, not a style preference

**"Minimal" names a target nobody can check.** Minimal with respect to what? Every reading of it is either false or untestable: not the fewest symbols, not the fewest that can compute everything, not the fewest anybody has managed to find. What was actually meant — *nothing here can be built from the rest of here* — has a name, and it is **irreducible**, and unlike "minimal" it can be refuted by writing a body.

**A count in prose drifts, silently, and it did.** §2.4 of the Phase 2 plan says the frozen set is eleven. `minimal-primitives.md` documents sixteen names. `matter::register()` holds eight, one of which (`hash`) the same §2.4 assigns to physics. Three numbers, one subject, no check — because a count written in a sentence is not something a build can disagree with. Under the rule the count is **derived** by `cargo xtask floor` and asserted even; it is never written down.

**The freeze was the wrong instrument.** Freezing says *do not change this*. It gives no way to decide whether a proposed change is right, so the first time the floor genuinely needed a member — Phase 2's missing eliminator — the options were to break the freeze or to route around it, and the project routed around it into Rust. That is F1. A rule admits and refuses; a freeze can only forbid, which is why it gets broken instead of applied.

**Evenness is the rule's shadow, not a goal.** The floor's size is even because every member is half of a symmetric pair. So an odd count is a *finding*: some member's opposite is missing, and the pairing check names it before parity is ever mentioned. An even count is not an achievement to aim at; it is what a correctly paired floor cannot help being.

## What is superseded, and where

These documents are historical records of completed phases. **They are not edited.** Read the left column as superseded by the right.

| Document | Says | Now reads as |
|---|---|---|
| Phase 0–1 plan §3.1, P0-06 | "the minimal primitive set, frozen" | the floor, admitted by the rule; `the-floor.md` |
| Phase 0–1 plan, exit gate | "The minimal primitive set, frozen, one paragraph and one example each" | one paragraph, one example **and one irreducibility argument** each |
| Phase 2 plan §2.4 | "The frozen set mixes three kinds of thing… eleven entries in three registers" | no count; three registers classify, they do not gate membership |
| Phase 2 plan §3.2 | "`docs/Findings/minimal-primitives.md` is the freeze" | `the-floor.md` is the rule and the amendment record |
| Phase 2 plan §2.1 | ℤ's minimal signature `{zero, succ, pred, eq}`, `Frame::case` as the eliminator | correct about the *frame*; the eliminator is now also a floor primitive, `case`, reachable from a body |
| Phase 2 plan, Appendix A rule 12 | "The minimal primitive set is frozen… do not add to it" | Phase 2.1 Appendix A rule 12: irreducible and opposed; attempt the reduction first |
| `minimal-primitives.md` | the whole document | retired; superseded by `the-floor.md` |
| Phase 2.1 plan, Draft 0.1 §2.1 | "`case` is the twelfth primitive" | the floor rule and the `build`/`case` pair; the count is not named |

Nothing in the **Theory** documents (Parts I–III) is contradicted by this change. Part III §7.4's declared floor and §1's three registers are what the rule formalizes.

## Banned wording, enforced

`cargo xtask vocab` refuses these in code, comments, commit messages and docs under `docs/Findings/` and `docs/Plans/`:

| Banned | Use |
|---|---|
| minimal set, minimal primitive set, the minimal set | the floor |
| the eleven, the twelve, the sixteen, any prose count of the floor | *(say nothing; `xtask floor` derives it)* |
| frozen set, the freeze *(of the floor)* | the floor, the rule, an amendment |
| "add a primitive" | "petition" — and a petition carries a refuted reduction |

The one place a number may appear is `xtask floor`'s output, which computes it.

## Rename touchpoints

`minimal-primitives.md` → `the-floor.md` is carried out in commit **P21-02**. Everything that names the old path, in one list so none is missed:

- `joinn/AGENTS.md` rule 12 and `.cursor/rules/joinn.mdc` (identical file)
- `joinn/README.md` — the "this is where the trust is" paragraph
- `joinn/crates/joinn-prim/src/matter.rs` — module doc comment; the module is renamed `floor`
- `joinn/xtask/src/main.rs` — the register-count check, replaced by `xtask floor`
- `docs/Plans/JoInn Phase 2 Implementation Plan.md` §1, §3.2, §9, Appendix A — **not edited**; superseded per the table above
- `docs/Plans/JoInn Phase 2.1 Implementation Plan.md` — edited in place, since it is the active plan
- `docs/Findings/phase-2.md` — the R32 line, which now says what R32 actually is (see below)

## Two statements that got truer

**R32 — where the trust is.** The old wording: `hash` is one-way, the floor's single exception. The new: `hash` is total, `resolve` is partial, **and the gap between them is the store**. One-wayness was never a property of the operation — `resolve` is used constantly, it is what turns `cell:6b32…` in a genome into a cell. What is actually trusted is that the store holds what it claims to hold. That is a narrower hole and a truer one, and it is the sentence the README should carry.

**`eq` and `choose`.** The register declared each as its own opposite, which is a pair that is not a pair. The opposition is between them: **a distinction made and a distinction spent.** `choose(eq(x, x), a, b) = a` was already the stated law; the register simply did not say who it was a law *between*.
