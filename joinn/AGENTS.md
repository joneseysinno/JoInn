# JoInn — standing rules

This repo is JoInn. Phases 2–3 built a universe that is true, and gave it a face
proved by there being two hosts. Phase 5.1 corrects the second body. Its one
idea is that A CONTROL IS ONLY AN ARTIFACT IF DAMAGING THE ARTIFACT CHANGES THE
CONTROL'S ANSWER. The build plan is
docs/Plans/JoInn Phase 5.1 Implementation Plan.md. Work one numbered commit at a
time. Do not start the next one.

There is no renderer, no homology, no third body and no compiler in this phase.
Phase 4 (the assay layer) runs AFTER this phase, against a universe deep enough
to answer its own decoration check.

## Hard rules

1. A refusal is a VALUE (`Verdict::Refused`), never a Rust `Err` and never a
   panic. `Result` is for host errors only: IO, malformed input, bugs.
2. Never `unwrap`, `expect`, or panic outside tests. `#![forbid(unsafe_code)]`.
3. No `f32`/`f64` anywhere. No `HashMap`/`HashSet` — `BTreeMap`/`BTreeSet` only.
4. No wall clock and no unseeded RNG in crates/. `xtask` MAY measure wall time;
   it cannot reach a hash, a canonical form, a sample or a refusal. Message
   delivery order is a function of the body's grant list, never of arrival time.
5. No `#[derive(Hash)]` and no `serde` derive on any DNA type, and none on a
   Description or a Universe. The canonical writer is hand-written and tested.
6. Never change a golden hash in corpus/ to make a test pass. If a hash moves,
   stop and report it. A hash that moves because an artifact was CORRECTED on
   purpose is recorded with a dated finding — that is not reblessing.
7. Never weaken, skip, `#[ignore]`, or delete a test to make a build green.
   Report the failure instead.
8. A primitive name must never appear in a coding region. Laws may name: the
   cell under definition (`self`), frame signature operations, and other coding
   regions BY HASH. A BODY may name primitives — that is what `prim:` is for.
9. Display names, literals, prompts, styles, labels, roles and layout are
   REGULATORY.
10. THE ENGINE NEVER RECURSES IN RUST. Nesting is an explicit activation stack
    with ONE global step budget.
11. A REFERENCE ALLELE IS A BODY. `Seal::reference` is a `BodyRef`. A reference
    implemented as a Rust `Oracle` is rejected on sight.
12. THE FLOOR IS IRREDUCIBLE AND OPPOSED, and it is defined in
    docs/Findings/the-floor.md. Never add to the floor. If a task seems to need a
    new primitive, first write it as a reference body over the existing floor.
    NEVER write "the minimal set" or the floor's size as a number. The count is
    derived by `cargo xtask floor`.
    `zero`, `succ` and `pred` are NOT floor primitives. They are ℤ's first three
    constructors and live in ℤ's signature as frame vocabulary.
13. `sub`, `subtract` and `minus` are not identifiers in this project.
15. `std::io` AND `std::fs` APPEAR ONLY IN HOST CRATES AND xtask. joinn-host is
    protocol and does none. joinn-test-host, joinn-assay and joinn-link do none.
    joinn-frame, joinn-dna, joinn-gate, joinn-prim and joinn-live do none.
    `cargo xtask vocab` enforces this by path.
16. A nested activation holds NO grants and can reach no capability.
17. NO INSTRUMENT WRITES ITS OWN FINDING. `xtask` prints numbers to stdout. No
    xtask subcommand writes a file under docs/Findings/.
18. EVERY MUTANT NAMES THE CHECK THAT CAUGHT IT, and registers a negative
    control the check must ACCEPT.
19. EVERY CHECK IS OPPOSED. A check declares a witness it must refuse and a
    witness it must accept, and fails the build if it cannot do both.
20. NO INSTRUMENT CHOOSES ITS OWN INPUTS. `Drive::bound` is a `NonZeroU32`.
    Every seal carries a COUNTERFEIT body that `agree` must separate from the
    reference at the declared bound; a counterfeit that is not separated is a
    BLIND SEAL and fails the build.
21. A SEAL NAMES THE CELL THAT DECLARES ITS SEALED ALLELE, found by lookup;
    zero matches and two matches both refuse.
22. A CORPUS CELL IS ONE THE GATE ADMITS, and an allele it carries is one the
    gate admits against it. The counterfeit corpus under
    corpus/phase22/counterfeit/ is refused ON PURPOSE and is never a golden.
23. AN ALLELE MAY NAME ONLY MATTER. `hash` and `resolve` are physics and are not
    in the allele-visible native set.
24. A CONTROL IS AN ARTIFACT, NOT A PREDICATE — IT IS A PATH, AND IT IS READ.
    `GateItem` carries `control_artifact`; `run_gate_table` resolves it, reads
    its bytes, and passes them to `control(&Artifact)`. The control is run on the
    real bytes and on a damaged copy and must answer differently, or the run is
    refused naming the item. joinn-gate carries the bytes and never opens a file.
25. MODULE.RS BESIDE MODULE/, NEVER MOD.RS. A leaf file holds at most one named
    production function. A capsule root (`foo.rs` with `foo/` beside it) holds
    types, `mod` declarations, `pub use` and getters. `lib.rs` is a facade with
    zero production functions; `main.rs` holds `main` only. `#[cfg(test)]` does
    not count. One `impl Trait for T`'s required methods may share a leaf.
    `allow(modules): <reason>` silences one file and a bare silencer fails.
    Crate-level `tests/` is exempt from the one-function rule but not from the
    mod.rs ban. `cargo xtask modules` enforces this, running its three fixtures
    BEFORE the scan. NEVER ADD A LINE TO grandfather.txt — a file that cannot
    obey the rule is evidence about the rule, and it goes in docs/Findings/.
26. NO CHECK IS SCOPED TO ONE FILE unless that file is its subject. A check that
    scans "the source" scans a directory tree.
27. A DESCRIPTION IS A VALUE, NOT TEXT. `Description` is constructed only in
    joinn-host. It carries no template, width, alignment or pre-rendered field.
    There is no `String`-returning render function in joinn-host.
28. A HOST NEVER INVENTS AN INTENT. The intent set is derived from the body —
    one address per in-port of ∂(body). A host emitting an intent outside it is
    refused naming the address.
29. A GATE'S SCORE IS RETURNED BY THE GATE. No literal gate score appears
    anywhere under xtask/src. `write_lock` receives counts; it does not choose
    them.
30. A MEMBRANE IS ∂(BODY) AND IS STORED NOWHERE. It is every port of every
    instance in the genome — cell or primitive — that no internal wire consumes,
    each with its direction and frame, computed on demand. A genome entry whose
    cell is not supplied is refused, never skipped. Nothing in a .body or
    .universe file declares it.
31. A LINK TOUCHES PORTS ON MEMBRANES ONLY. A member naming an interior port is
    refused naming the port and the wire that consumes it. This is checked by
    assembling the complex and requiring ∂∂ = 0, never by a predicate written
    beside the check.
32. INSIDE A BODY IS A WIRE; BETWEEN BODIES IS A HYPEREDGE. Each other form is
    refused naming the container and the method it requires.
33. A BODY DOES NOT KNOW ITS SYSTEM. Membership lives in a lens. Within one lens
    a body has exactly one owning system; across lenses it may differ.
34. THE GATE NEVER DEPENDS ON THE ASSAY. An assay is injected the way natives
    are. joinn-gate names no assay type. joinn-assay names no DNA type and
    computes no homology in this phase.
35. A REFUSAL NAMES WHAT ITS ACCEPTANCE WOULD HAVE LOOKED LIKE. `BLIND SEAL`
    names the bound and the size of the domain it admits; `first_desc_diff`
    names the differing field; an assembly refusal names the block and the port.
    A refusal that only says what failed is half a refusal.
36. A LINK IS ADMITTED WHEN ITS MEMBERS AGREE. Tails are out-ports, heads are
    in-ports, and every member's frame is equal. A universe binds bodies by
    coding hash, never by alias.
37. A RUNTIME IS TESTED BY WHAT A BODY DID. Assert on descriptions, fire counts,
    port values and host output, never on the runtime's own bookkeeping.
38. SOME FINDINGS ARE TYPED BY AJ. A commit that marks a file "typed by AJ"
    (witness runs, the adversary's spec and verdict, perf conclusions) is not
    written by the agent. If such a file appears in the agent's commit, the
    commit is red.
39. NOTHING TEXTUAL CROSSES A MEMBRANE ON REFUSAL. The far side of a link
    receives a `LinkRefusal` — link, body, member, kind — and no reason string.
    The refusing body's reason is reachable by `probe` on that body only.
40. ∂∂ = 0 IS NOT CLAIMED WHERE IT CANNOT FAIL. A universe's complex has
    dimension ≤ 1 until fillings exist; touch-only is enforced by C₀ being
    derived. No check, doc comment or finding may attribute a universe refusal
    to ∂∂ ≠ 0 before Phase 4.

## Definition of done

A commit is done when the plan's "done when" command passes and you have
reported what it printed. "It compiles" is not done. "The tests pass" is not
done if the test does not refuse anything. A number is not done if the code that
printed it also chose it. A refusal is not done if nothing was ever shown to it.
A control is not done if a reviewer cannot read it — OR IF THE BUILD CANNOT
RESOLVE IT. And a rule is not done while it lives only in this file: if it can
be a type, make it a type.
