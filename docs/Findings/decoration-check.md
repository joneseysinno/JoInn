# Decoration check

## The rule

> **KEEP** if, for candidate C1, the assay's answer on the subject differs from its answer on the mutant, and **no other check in the tree** does (every existing admission, typing, Law 4, lens and binding check, every declared `require`/`ensure`, and a run on the inputs in §2.8 C1). Catching it without a run counts (AJ, 26 Sep: *static counts*). **CUT** otherwise.

A check **distinguishes** only if it answers differently on subject and mutant. Refusing both, or admitting both, is not distinguishing (the Phase 5.2 control rule).

## First reading (P4-08, superseded)

```
gate admission: admitted | admitted | same
insert: admitted | admitted | same
bind: admitted | admitted | same
assemble: admitted | admitted | same
check_link_types: admitted | admitted | same
check_law4: admitted | admitted | same
check_lenses: admitted | admitted | same
require/ensure: none declared | none declared | same
run: description {   cell c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e   instance cli_a   label "cli_a"   role cell   port 0 in Text 1 "line" label "line" role input value "5"   port 1 out ℤ 1 "value" label "value" role output value 5 }  || description {   cell c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e   instance cli_b   label "cli_b"   role cell   port 0 in Text 1 "line" label "line" role input value "3"   port 1 out ℤ 1 "value" label "value" role output value 3 }  || description {   cell 6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39   instance sum   label "Sum"   role cell   port 0 in ℤ 1 "a" label "first addend" role input value 2   port 1 in ℤ 1 "b" label "second addend" role input value 3   port 2 out ℤ 1 "result" label "sum" role output value 5 }  || description {   cell 3b0ab2bca11406a7e3bb3c1c4fd78e95af213f82fe2c1c5c6ffc0d4ec76e9c11   instance fmt   label "Format"   role cell   port 0 in ℤ 1 "n" label "n" role input value 5   port 1 out Text 1 "text" label "text" role output value "5" }  || description {   cell c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e   instance cli_a   label "cli_a"   role cell   port 0 in Text 1 "line" label "line" role input value "5"   port 1 out ℤ 1 "value" label "value" role output value 5 }  | description {   cell c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e   instance cli_a   label "cli_a"   role cell   port 0 in Text 1 "line" label "line" role input value "5"   port 1 out ℤ 1 "value" label "value" role output value 5 }  || description {   cell c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e   instance cli_b   label "cli_b"   role cell   port 0 in Text 1 "line" label "line" role input value "3"   port 1 out ℤ 1 "value" label "value" role output value 3 }  || description {   cell 6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39   instance sum   label "Sum"   role cell   port 0 in ℤ 1 "a" label "first addend" role input value 2   port 1 in ℤ 1 "b" label "second addend" role input value 3   port 2 out ℤ 1 "result" label "sum" role output value 5 }  || description {   cell fcc1ba589d125d8b490c631510dcec4d86d65a9e1f032ecb8d254b692999b85d   instance fmt   label "Format"   role cell   port 0 in ℤ 1 "n" label "n" role input value 5   port 1 out Text 1 "text" label "text" role output value "5" }  || description {   cell c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e   instance cli_a   label "cli_a"   role cell   port 0 in Text 1 "line" label "line" role input value "5"   port 1 out ℤ 1 "value" label "value" role output value 5 }  | differs
assay: H₁: 0 | H₁: 1 | differs
distinguishing: run, assay
```

Superseded by Amendment B: the run row compared cell hashes and labels, not values. With cell hashes masked the two run texts were equal, and a label-only edit changed the subject's text.

## Amendment B reading

```
control run label-only: same (ok)
control run input changed: differs (ok)
gate admission: admitted | admitted | same
insert: admitted | admitted | same
bind: admitted | admitted | same
assemble: admitted | admitted | same
check_link_types: admitted | admitted | same
check_law4: admitted | admitted | same
check_lenses: admitted | admitted | same
require/ensure: none declared | none declared | same
run: cli_a@0 in "5", cli_a@1 out 5, cli_b@0 in "3", cli_b@1 out 3, sum@0 in 2, sum@1 in 3, sum@2 out 5, fmt@0 in 5, fmt@1 out "5", cli_a@0 in "5", cli_a@1 out 5 | cli_a@0 in "5", cli_a@1 out 5, cli_b@0 in "3", cli_b@1 out 3, sum@0 in 2, sum@1 in 3, sum@2 out 5, fmt@0 in 5, fmt@1 out "5", cli_a@0 in "5", cli_a@1 out 5 | same
assay: H₁: 0 | H₁: 1 | differs
distinguishing: assay
```

## Verdict

Verdict: KEEP

The `distinguishing:` line is `assay`. The rule keeps the assay only when that line is exactly `assay`.

## C1

> **Held.** C1: the assay distinguished `loop.universe` from its `fmt_twin` mutant, and no other check did, including a run. Every body in the mutant is lawful on its own. What the assay found is a round trip across two bodies whose promise names a different partner. `cli_input` promises to undo `format`, and the ring converts with `format_twin`, a different cell by content. The two formatters behave the same today, so this is a missing promise, not a wrong number. Under Law 1, an operation whose opposition isn't declared is untrue by structure, and the assay is the only instrument in JoInn that sees it across a membrane without running anything.

```
control run label-only: same (ok)
control run input changed: differs (ok)
gate admission: admitted | admitted | same
insert: admitted | admitted | same
bind: admitted | admitted | same
assemble: admitted | admitted | same
check_link_types: admitted | admitted | same
check_law4: admitted | admitted | same
check_lenses: admitted | admitted | same
require/ensure: none declared | none declared | same
run: cli_a@0 in "5", cli_a@1 out 5, cli_b@0 in "3", cli_b@1 out 3, sum@0 in 2, sum@1 in 3, sum@2 out 5, fmt@0 in 5, fmt@1 out "5", cli_a@0 in "5", cli_a@1 out 5 | cli_a@0 in "5", cli_a@1 out 5, cli_b@0 in "3", cli_b@1 out 3, sum@0 in 2, sum@1 in 3, sum@2 out 5, fmt@0 in 5, fmt@1 out "5", cli_a@0 in "5", cli_a@1 out 5 | same
assay: H₁: 0 | H₁: 1 | differs
distinguishing: assay
```

## C2

> **Not expressible yet.** A value-consistency check (H¹ with values) needs a factor stated in DNA and a coding-level claim that two values in different bodies are the same quantity. Today the factor enters from outside (`factor: 12`), and no universe carries a law. A `require` could state the claim only at one membrane that sees both values, which makes it local. This opens R65.

`git grep -n "factor" -- joinn/corpus/transcripts joinn/corpus/phase5/units.body`:

```
joinn/corpus/phase5/units.body:15:  prompts { scale "factor: " }
joinn/corpus/transcripts/universe.txt:6:factor: 12
```

`git grep -n "laws" -- joinn/corpus/phase5/*.universe joinn/corpus/phase4/*.universe`:

```
(no lines; git grep exit 1)
```

## C3

> **Withdrawn by the invariance rule.** Part III §9.5 read H₂ as laws closed around an empty inside: a specified but unimplemented cell. "Unimplemented" means "has no allele", and §9.8 requires an assay not to move when alleles change. The harness stripped every allele from the calculator's cells and the report did not move. An admissible assay cannot see implementation, so that reading of H₂ is withdrawn. H₂ stays as a count of loops filled twice.

`cargo xtask assay invariance`:

```
phase2/calculator.body: 1 ok, allele strip ok, 3 n/a (no lens), 4 n/a (body)
```

## Size of the claim

C1 shows a missing promise, not a wrong number. A narrower check (does a round-trip law name the cell it is linked to?) would also see this one case. No such check exists in the tree, which is why the rule kept the assay. What the assay adds is finding which loops need a promise at all, as universes grow.
