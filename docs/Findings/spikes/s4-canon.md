# S4 · Canonical form spike

**Can kill Law 5 as implemented.**

## What was built

Eight source files (four `CliInput`, four `Sum`) that mean the same two cells, differing in port display names, law order, literal spelling, whitespace and comments. A throwaway Python canonicalizer in `joinn/spikes/s4-canon/` that is **not** the Phase 1 canonicalizer.

## Result

The four `Sum` spellings collapsed to one byte string. The four `CliInput` spellings collapsed to one byte string. Different cells stayed different.

That is the acceptance test in the plan §3.3. The grammar is admissible to Phase 1.

## Negative note

The spike canonicalizer is allowed to be wrong in ways the Phase 1 one must not: it hard-codes integer printing and knows nothing about frames. If S4 and P1-06 ever disagree, P1-06 is the identity function and S4 is deleted. They are deliberately separate so a formatting bug in the spike cannot become an identity bug in the corpus.

A formatting bug in Phase 1's printer *is* an identity bug. The three property tests (`parse ∘ print = id`, `print ∘ parse ∘ print = print`, regulatory invariance) are the guard, not this spike.
