# Glossary (plain English)

Short definitions for words you’ll see in JoInn docs. Deeper, precise meanings live in Theory; this page is for clarity.

| Term | Plain meaning |
|------|----------------|
| **Allele** | One candidate way to *implement* a cell’s plan. The plan’s identity is the laws; the allele is a payload that must pass those laws. |
| **Assay** | A measurement of a body or universe's shape without running it: the loops a value can travel, and which of them something closes. It reports; it never refuses on its own. |
| **Blueprint** | A shareable cell design others can reuse — meant to be seen, not only read. |
| **Body** | A cluster of cells that belong together, with one shared genome. The main “container” for a small machine or app piece. |
| **Bootstrap** | One of the three big build stages (true-but-unseen → seen-and-touched → self-editing). |
| **Cell** | The smallest visual building block. Something you can show and still use. |
| **Coding region** | The part of DNA that is hashed and gated: frame, contract, laws, witnesses — the cell’s true identity. |
| **Corpus** | The library of golden examples (and deliberate fakes) used as testimony and regression checks. |
| **Declaration** | A sentence in a body's or universe's hashed part that the assay must satisfy. Today the only one is `assert H₁ = 0` (“no loop is open”); if it fails, the body or universe is refused at admission. |
| **Crate** | A Rust library or program package in the repo (for example `joinn-gate`). |
| **DNA** | The building plan for a cell: what it is and what laws it obeys. |
| **Filling** | What closes a loop in the assay: a frame (the value keeps its kind all the way round), or one law that promises to undo a conversion, named by body, instance and law. Nothing else closes a loop. |
| **Floor** | The sealed set of basic operations JoInn is written in. Admitted only if irreducible and opposed. Never “fixed” by renaming its size. |
| **Frame** | The context in which truth is judged (example: integers, or a text frame). |
| **Gate** | The checker that admits good growth and refuses bad growth. |
| **Genome** | The full set of DNA shared by the cells of one body. |
| **Island** | In the assay, one connected piece of a universe once the outside world is removed. |
| **Host** | Whatever presents the universe to the outside (terminal, test harness, later: GPU UI). A host is proved by there being more than one. |
| **Live engine** | The runner that executes a body now, with an explicit step budget. |
| **Membrane** | The boundary of a cell — the only place it meets the outside. |
| **Primitive** | A sealed basic piece below ordinary cells. Creators compose with them; they don’t invent new floor primitives casually. |
| **Region** | In the assay, one connected piece of a body's inside (cells joined by wires). A body with two regions is two separate things sharing a container. |
| **Refusal / Verdict** | A normal value meaning “no” (`Refused`), not a crash. Host IO errors are different. |
| **Regulatory region** | The unhashed part of DNA: looks, labels, literals, layout choices — versioned, not the identity. |
| **Seal** | A locked pair: a reference body and checks (including a counterfeit that must be caught). |
| **Witness** | A recorded true result that future versions must still satisfy — testimony the gate can replay. |
| **xtask** | Helper commands that measure and enforce project rules (`floor`, `agree`, `gate`, and so on). |

## Quick analogies

- **Cell** ≈ Lego brick  
- **DNA** ≈ recipe + rules printed on the brick  
- **Gate** ≈ health inspector  
- **Host** ≈ storefront  
- **Corpus** ≈ the scrapbook of “this worked” (and “this must fail”)  
- **Floor** ≈ the foundation of the building  

## Next

Back to the start: [README.md](README.md).
