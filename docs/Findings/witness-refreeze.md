# Witness re-freeze (P21-20)

Date: 2026-09-18

Two Phase 2 corpus witnesses were re-frozen deliberately. Neither hash in
`corpus/hashes.txt` moved. The transcript and the step trace are not hashed;
they are byte-compared by `cargo xtask gate 2`.

## `calculator.txt`

Exit gate 2 specified a three-space indent on the refusal line:

```
a: two
   refused at membrane: "two" is not in ℤ
```

The Phase 2 file had none. The runner now prints the indent, and the stored
transcript matches the plan's exact text.

## `calculator.trace`

Phase 2 recorded two reports labelled `step 4`: the fire of `sum` and the
quiescent report reused the same index. The engine now increments before the
quiescent report, so that record is `step 5`. Depth is still omitted at depth 0.

Reason: a duplicate step number is a confusing witness, and the indent was the
plan's stated transcript. Re-freezing is the honest move; editing a golden hash
to make a test pass remains forbidden.
