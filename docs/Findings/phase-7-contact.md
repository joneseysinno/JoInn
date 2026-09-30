# Phase 7 — The body as cells in contact: findings

P7-14. A body can now be written as cells in contact (`.contact`): a genome,
its grants, and forces that combine members into a response. There are no
wires in the file. `lower` derives the wired body the engine runs, and on the
calculator it is `corpus/phase2/calculator.body` byte for byte (hash
`b55fba1e…`). The contact runs, describes itself and prints its transcript as
the wired calculator does. The picture draws it as cells that touch, with only
the surface ports shown and no link rows (a force owns no pixel). Every
prediction in §2.3, §2.5, §2.7, §2.8, §2.9 and §2.10 of the plan held; none
differs.

Commits: P7-06 `4ac2ba4`, P7-06a `c5b7aec`, P7-07 `f0f6788`, P7-08 `c088cb3`,
P7-09 `8975fb6`, P7-10 `ed9264f`, P7-11 `7d7076c`, P7-11a `6adb959`, P7-12
`834ff62`, P7-13 `fe9bdf6`. Machine: Windows 10.0.26200, adapters Microsoft
Basic Render Driver (Dx12, Cpu) and NVIDIA GeForce RTX 2080 (Dx12 and Vulkan).
The forces, corpus verify, contact, CLI, roles, layout and test outputs were
re-run at `fe9bdf6` plus this commit's one new test. The pick, regrow and
gate 6 outputs are P7-12's runs at `834ff62`, and the window output is P7-13's
run. All were run from `joinn/` and copied from the terminal.

## The force register (P7-06, P7-06a)

`cargo xtask forces`: the one row (combine on ℤ 1 by the sum cell, opposed by
`sum_turn`), then the plants of §2.2. P7-06a's check that every native ℤ allele
of the response is order-free leaves the seven lines unchanged.

```
combine ℤ 1 by cell:6b32…: order-free (64 pairs, 64 triples, seed 7), opposed by separate cell:6fcb… (turn 0 from {1 2})
planted: order_free on mutant.difference: refused (ok): not order-free: f(a, b) = 9223372039002259455 but f(b, a) = -9223372039002259455 at a = 9223372036854775807, b = -2147483648; acceptance is a response whose result does not depend on member order
planted: order_free on mutant.midpoint: refused (ok): not order-free: f(f(a, b), c) = -1535576763092620387 but f(a, f(b, c)) = -3841419772306314346 at a = -9223372036854775837, b = 3081064984484294291, c = 0; acceptance is a response whose result does not depend on member order
planted: order_free on mutant.max: order-free (ok), not registered
planted: order_free on mutant.plus1: order-free (ok), not registered
planted: check_register with separate = response: refused (ok): combine on ℤ 1 is unopposed: no separate; acceptance is a turn of cell:6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39
forces: 1 combine registered, 1 opposed; plants: difference refused, midpoint refused, max and plus1 order-free but unregistered, unopposed refused (ok)
```

## The file kind (P7-07)

`cargo xtask corpus verify` (one new file, `phase7/calculator.contact`, no
earlier hash moved):

```
corpus verify: refused false_law.cell by name
corpus verify: 44 hash(es) match; cells admitted
```

`print_contact` of `calculator.contact`, asserted byte for byte by
`body::contact::tests::four_spellings_share_one_hash_and_grant_order_moves_it`:

```
budget
steps 100000
codex 1
genome
cell:c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e as cli_a, cli_b
grants
stdin cli_a cli_b
lineage none
forces
combine ℤ 1 cell:6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39 as sum from cli_a@1 cli_b@1
```

Hash `868e79b293c8d35625f514274eb0bf04d228898ed994a795b23b942a530b6209`
(`cargo xtask contact`'s first line, below).

## Catalogue, admission, derivation, running it (P7-08 to P7-10)

`cargo xtask contact`: admitted, the lowered text (stdout only; never written,
rule 61), equal to the wired calculator, the same transcript and descriptions,
the same surface, both pairings keep `sum@2 = 5`, and each §2.14 mutant answers
as the catalogue says.

```
phase7/calculator.contact: hash 868e79b293c8d35625f514274eb0bf04d228898ed994a795b23b942a530b6209, admitted
--- lowered ---
budget
steps 100000
codex 1
genome
cell:6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39 as sum
cell:c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e as cli_a, cli_b
grants
stdin cli_a cli_b
lineage none
wires
cli_a@1 -> sum@0
cli_b@1 -> sum@1
--- end ---
lowered = phase2/calculator.body (b55fba1e…)
transcript equal
calculator.desc equal
calculator_refusal.desc equal
surface equal (3 ports)
pairings: 2, sum@2 = 5 in each
DropForce("sum"): admitted, sum no longer exists
SwapResponse("sum", "12b6e5458891b14aa74e43cee34b1184be04547fe58e035b1882125467120fa7"): refused (ok): combine on ℤ 1 responds by cell:6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39; force sum pins cell:12b6e5458891b14aa74e43cee34b1184be04547fe58e035b1882125467120fa7. acceptance is the registered response
ShiftMember("sum", "cli_a@1", 0): refused (ok): receptor: cli_a@0 is Text 1, the force is combine ℤ 1; acceptance is a member in ℤ 1
DropMember("sum", "cli_b@1"): refused (ok): force sum has 1 members; its response takes 2. acceptance is 2 members (R84)
DropGenome("cli_b"): refused (ok): member cli_b@1 names no instance; acceptance is an instance of the genome or a force's response
RenameAlias("sum", "total"): admitted, canonical text differs only in sum → total, hash 1a34563a…
contact: 1 body(ies); two forms, one truth
```

`cargo run -p joinn-cli -- run corpus/phase7/calculator.contact` with stdin
`two`, `2`, `3`, which is `corpus/transcripts/calculator.txt`:

```
a: two
   refused at membrane: "two" is not in ℤ
a: 2
b: 3
2 + 3 = 5
```

## Roles (P7-11)

`cargo xtask roles corpus/phase7/calculator.contact`:

```
roles calculator.contact
cli_a protect (faces out, holds)
cli_b protect (faces out, holds)
sum carry (faces out, reacts)
roles: 3 cell(s); protect 2, carry 1, store 0, respond 0
```

## Admission refuses a force loop (P7-11a, Amendment B)

`check_contact` step 6a. The self-loop is refused with `force s reaches itself:
s → s; acceptance is forces that combine only cells that exist before them
(R84)`, the two-force loop with `force s reaches itself: s → t → s; …`, and
§2.7's chained fixture (`s1` from `a@1, b@1`; `s2` from `c@1, s1@2`) is still
admitted and lowers (tests `a_force_that_reaches_itself_is_refused`,
`a_chain_of_forces_is_admitted_and_lowers`). `cargo xtask contact` printed the
same 28 lines as at Stop B.

## The contact picture (P7-12)

`cargo xtask layout phase7/calculator.contact`:

```
layout body
surface 0 0 32 20 r 3
cell cli_a 4 4 12 6 r 2
cell cli_b 4 10 12 6 r 2
cell sum 16 4 12 12 r 2 response
port cli_a@0 in 4 7 r 1
port cli_b@0 in 4 13 r 1
port sum@2 out 28 7 r 1
camera 640x360: k 16, origin 64 20
camera 1000x777: k 28, origin 52 108
camera 1280x720: k 32, origin 128 40
camera 1920x1080: k 52, origin 128 20
```

`cargo xtask pick`. The `calculator.contact` lines are new; everything else is
Phase 6's output, unchanged.

```
adapter Microsoft Basic Render Driver · Dx12 · Cpu · vertex storage yes
calculator 640x360: agree 230216, edge 184, disagree 0, owners 13/13
calculator 1000x777: agree 776768, edge 232, disagree 0, owners 13/13
calculator 1280x720: agree 921288, edge 312, disagree 0, owners 13/13
calculator 1920x1080: agree 2073144, edge 456, disagree 0, owners 13/13
calculator.contact 640x360: agree 230248, edge 152, disagree 0, owners 7/7, links 0
calculator.contact 1000x777: agree 776784, edge 216, disagree 0, owners 7/7, links 0
calculator.contact 1280x720: agree 921256, edge 344, disagree 0, owners 7/7, links 0
calculator.contact 1920x1080: agree 2073164, edge 436, disagree 0, owners 7/7, links 0
frame median 3.401 ms (calculator 1280x720; information, not a check)
phase2/calculator.body: agree 921288, edge 312, disagree 0
phase2/variants/calculator_b.body: agree 921288, edge 312, disagree 0
phase2/variants/calculator_c.body: agree 921288, edge 312, disagree 0
phase2/variants/calculator_d.body: agree 921288, edge 312, disagree 0
phase21/int_add_ref.body: not measured: layout: wire pickz@0 -> self@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/int_format_ref.body: not measured: layout: wire mkchr@0 -> self@1 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/int_mul_ref.body: not measured: layout: wire differ@0 -> picks@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/rat_add_ref.body: not measured: layout: wire self@0 -> self@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/text_parse_ref.body: not measured: layout: wire c@1 -> self@1 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/int_format_ref.body: not measured: layout: wire a48@2 -> d45@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/int_mul_ref.body: not measured: layout: wire differ@0 -> picks@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/rat_add_ref.body: not measured: layout: wire pick@0 -> self@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/text_parse_ref.body: not measured: layout: wire c45@0 -> isdash@1 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase3/columns_reader.body: not measured: scene: reader@0 names an in-port and an out-port; acceptance is a body whose cells give each position one port, since an ID names a port by position
phase3/environment.body: not measured: scene: columns@0 names an in-port and an out-port; acceptance is a body whose cells give each position one port, since an ID names a port by position
phase4/fmt.body: agree 921440, edge 160, disagree 0
phase4/fmt_twin.body: agree 921440, edge 160, disagree 0
phase5/bus.body: agree 921440, edge 160, disagree 0
phase5/controls/echo.body: agree 921440, edge 160, disagree 0
phase5/two_in_ports.body: agree 921412, edge 188, disagree 0
phase5/units.body: agree 921412, edge 188, disagree 0
phase52/adversary/asker.body: agree 921432, edge 168, disagree 0
phase52/adversary/lookup.body: agree 921432, edge 168, disagree 0
phase52/controls/missing_cell.body: not measured: instance orphan cell 4a37 was not supplied; acceptance is that cell in the cell map
adapter NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes
calculator 640x360: agree 230216, edge 184, disagree 0, owners 13/13
calculator 1000x777: agree 776768, edge 232, disagree 0, owners 13/13
calculator 1280x720: agree 921288, edge 312, disagree 0, owners 13/13
calculator 1920x1080: agree 2073144, edge 456, disagree 0, owners 13/13
calculator.contact 640x360: agree 230248, edge 152, disagree 0, owners 7/7, links 0
calculator.contact 1000x777: agree 776784, edge 216, disagree 0, owners 7/7, links 0
calculator.contact 1280x720: agree 921256, edge 344, disagree 0, owners 7/7, links 0
calculator.contact 1920x1080: agree 2073164, edge 436, disagree 0, owners 7/7, links 0
frame median 3.029 ms (calculator 1280x720; information, not a check)
phase2/calculator.body: agree 921288, edge 312, disagree 0
phase2/variants/calculator_b.body: agree 921288, edge 312, disagree 0
phase2/variants/calculator_c.body: agree 921288, edge 312, disagree 0
phase2/variants/calculator_d.body: agree 921288, edge 312, disagree 0
phase21/int_add_ref.body: not measured: layout: wire pickz@0 -> self@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/int_format_ref.body: not measured: layout: wire mkchr@0 -> self@1 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/int_mul_ref.body: not measured: layout: wire differ@0 -> picks@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/rat_add_ref.body: not measured: layout: wire self@0 -> self@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/text_parse_ref.body: not measured: layout: wire c@1 -> self@1 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/int_format_ref.body: not measured: layout: wire a48@2 -> d45@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/int_mul_ref.body: not measured: layout: wire differ@0 -> picks@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/rat_add_ref.body: not measured: layout: wire pick@0 -> self@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/text_parse_ref.body: not measured: layout: wire c45@0 -> isdash@1 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase3/columns_reader.body: not measured: scene: reader@0 names an in-port and an out-port; acceptance is a body whose cells give each position one port, since an ID names a port by position
phase3/environment.body: not measured: scene: columns@0 names an in-port and an out-port; acceptance is a body whose cells give each position one port, since an ID names a port by position
phase4/fmt.body: agree 921440, edge 160, disagree 0
phase4/fmt_twin.body: agree 921440, edge 160, disagree 0
phase5/bus.body: agree 921440, edge 160, disagree 0
phase5/controls/echo.body: agree 921440, edge 160, disagree 0
phase5/two_in_ports.body: agree 921412, edge 188, disagree 0
phase5/units.body: agree 921412, edge 188, disagree 0
phase52/adversary/asker.body: agree 921432, edge 168, disagree 0
phase52/adversary/lookup.body: agree 921432, edge 168, disagree 0
phase52/controls/missing_cell.body: not measured: instance orphan cell 4a37 was not supplied; acceptance is that cell in the cell map
adapter NVIDIA GeForce RTX 2080 · Vulkan · DiscreteGpu · vertex storage yes
calculator 640x360: agree 230216, edge 184, disagree 0, owners 13/13
calculator 1000x777: agree 776768, edge 232, disagree 0, owners 13/13
calculator 1280x720: agree 921288, edge 312, disagree 0, owners 13/13
calculator 1920x1080: agree 2073144, edge 456, disagree 0, owners 13/13
calculator.contact 640x360: agree 230248, edge 152, disagree 0, owners 7/7, links 0
calculator.contact 1000x777: agree 776784, edge 216, disagree 0, owners 7/7, links 0
calculator.contact 1280x720: agree 921256, edge 344, disagree 0, owners 7/7, links 0
calculator.contact 1920x1080: agree 2073164, edge 436, disagree 0, owners 7/7, links 0
frame median 2.216 ms (calculator 1280x720; information, not a check)
phase2/calculator.body: agree 921288, edge 312, disagree 0
phase2/variants/calculator_b.body: agree 921288, edge 312, disagree 0
phase2/variants/calculator_c.body: agree 921288, edge 312, disagree 0
phase2/variants/calculator_d.body: agree 921288, edge 312, disagree 0
phase21/int_add_ref.body: not measured: layout: wire pickz@0 -> self@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/int_format_ref.body: not measured: layout: wire mkchr@0 -> self@1 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/int_mul_ref.body: not measured: layout: wire differ@0 -> picks@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/rat_add_ref.body: not measured: layout: wire self@0 -> self@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase21/text_parse_ref.body: not measured: layout: wire c@1 -> self@1 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/int_format_ref.body: not measured: layout: wire a48@2 -> d45@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/int_mul_ref.body: not measured: layout: wire differ@0 -> picks@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/rat_add_ref.body: not measured: layout: wire pick@0 -> self@2 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase22/text_parse_ref.body: not measured: layout: wire c45@0 -> isdash@1 does not run from an out-port to an in-port; acceptance is a wire between ports of the body's instances
phase3/columns_reader.body: not measured: scene: reader@0 names an in-port and an out-port; acceptance is a body whose cells give each position one port, since an ID names a port by position
phase3/environment.body: not measured: scene: columns@0 names an in-port and an out-port; acceptance is a body whose cells give each position one port, since an ID names a port by position
phase4/fmt.body: agree 921440, edge 160, disagree 0
phase4/fmt_twin.body: agree 921440, edge 160, disagree 0
phase5/bus.body: agree 921440, edge 160, disagree 0
phase5/controls/echo.body: agree 921440, edge 160, disagree 0
phase5/two_in_ports.body: agree 921412, edge 188, disagree 0
phase5/units.body: agree 921412, edge 188, disagree 0
phase52/adversary/asker.body: agree 921432, edge 168, disagree 0
phase52/adversary/lookup.body: agree 921432, edge 168, disagree 0
phase52/controls/missing_cell.body: not measured: instance orphan cell 4a37 was not supplied; acceptance is that cell in the cell map
planted: port sum@1 moved one unit, cpu_pick against the GPU on Microsoft Basic Render Driver · Dx12 · Cpu · vertex storage yes: disagree 7336; refused as truth violation (ok)
pick: 3 adapter(s), 12 subject(s) agree; planted disagreement: refused as truth violation (ok)
```

`cargo xtask regrow`. The `contact` lines are new; the wired lines are Phase
6's, unchanged.

```
event 1 cli_a@0 "two": touched cli_a, rows 2 (bound 3), bytes 80, tables equal regrow
event 2 cli_a@0 "2": touched cli_a, sum, rows 3 (bound 7), bytes 112, tables equal regrow
event 3 cli_b@0 "3": touched cli_b, sum, rows 4 (bound 7), bytes 128, tables equal regrow
fresh calculator (Amendment A4):
event 4 cli_b@0 "x": touched cli_b, rows 2 (bound 3), bytes 80, tables equal regrow
event 5 cli_a@0 "2": touched cli_a, sum, rows 4 (bound 7 + 1), bytes 144, tables equal regrow
camera change: rows 0
idle tick: nothing to draw
regrow Microsoft Basic Render Driver · Dx12 · Cpu · vertex storage yes: color identical, ids identical
contact event 1 cli_a@0 "two": touched cli_a, rows 2 (bound 2), bytes 80, tables equal regrow
contact event 2 cli_a@0 "2": touched cli_a, sum, rows 1 (bound 4), bytes 48, tables equal regrow
contact event 3 cli_b@0 "3": touched cli_b, sum, rows 3 (bound 4), bytes 112, tables equal regrow
fresh contact calculator (Amendment A4):
contact event 4 cli_b@0 "x": touched cli_b, rows 2 (bound 2), bytes 80, tables equal regrow
contact event 5 cli_a@0 "2": touched cli_a, sum, rows 2 (bound 4 + 1), bytes 80, tables equal regrow
regrow contact Microsoft Basic Render Driver · Dx12 · Cpu · vertex storage yes: color identical, ids identical
regrow NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes: color identical, ids identical
regrow contact NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes: color identical, ids identical
regrow NVIDIA GeForce RTX 2080 · Vulkan · DiscreteGpu · vertex storage yes: color identical, ids identical
regrow contact NVIDIA GeForce RTX 2080 · Vulkan · DiscreteGpu · vertex storage yes: color identical, ids identical
planted: regrown tables with cli_a@0's filled bit flipped differ from the delta-built tables; refused (ok)
regrow: 3 adapter(s); planted difference: refused (ok)
```

`cargo xtask gate 6` after P7-12 (the latent bit and style 7 don't move the
wired calculator's picture):

```
harness fixtures: ok
1 ok  Every pixel has one owner, and both pickers name it
2 ok  What the engine computes is a row the picture shows
3 ok  A click names an address; a stale click is refused
phase 6: 3/3
```

Probes, seams and colors at 1280×720, printed by joinn-visual's
`scene::regrow_contact::tests::the_contact_picture_meets_the_probes_seams_and_colors`.
Owners come from `cpu_pick` and `resolve`. Each color is the one the shape
shader gives that owner: the style table's RGB for the surface (style 1); for a
cell, refused (3), else latent (7), else its row's style; for a port, 4 or 5 by
its `filled` flag. The owner order is cli_a, cli_b, sum, cli_a@0, cli_b@0,
sum@2, surface, pixel 0,0. Gate 7 item 3 (P7-15) reads the same colors from
the GPU color target on every adapter.

```
probe 192,104: body surface
probe 448,264: body.cli_a
probe 448,456: body.cli_b
probe 832,360: body.sum
probe 256,264: body.cli_a@0
probe 256,456: body.cli_b@0
probe 1024,264: body.sum@2
probe 0,0: background
seam column 639|640 at row 232: body.cli_a | body.sum
seam column 639|640 at row 264: body.cli_a | body.sum
seam column 639|640 at row 296: body.cli_a | body.sum
seam row 359|360 at column 320: body.cli_a | body.cli_b
seam row 359|360 at column 448: body.cli_a | body.cli_b
seam row 359|360 at column 576: body.cli_a | body.cli_b
colors grow: 2F5D8A 2F5D8A 39414D C9CED6 C9CED6 C9CED6 22262E 15171C
colors event 1: B03A2E 2F5D8A 39414D F2B134 C9CED6 C9CED6 22262E 15171C
colors event 2: 2F5D8A 2F5D8A 39414D F2B134 C9CED6 C9CED6 22262E 15171C
colors event 3: 2F5D8A 2F5D8A 2F5D8A F2B134 F2B134 F2B134 22262E 15171C
```

## The shell opens `.contact` (P7-13)

`cargo test -p joinn-shell-desktop -- --nocapture` (V125 on both files):

```
calculator.contact rows 2, 1, 3
calculator.body rows 2, 3, 4
```

`cargo run -p joinn-shell-desktop -- corpus/phase7/calculator.contact`, run
once on Windows. The click at the middle of `sum` (832,360) was posted to the
window as mouse messages, and then the window was closed:

```
adapter: NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes
surface: Bgra8Unorm
tick 0: rows 15, bytes 304
pick 832,360: body.sum (cpu)
tick 1: rows 0, bytes 0
        body.sum (gpu) · agree
```

## Tests

The contact, force, lowering, surface, role and picture tests, from
`cargo test -p joinn-dna -p joinn-link -p joinn-prim -p joinn-visual -p joinn-test-host -p joinn-cli -p joinn-shell-desktop -p xtask -- contact force lower surface roles register order_free latent spelling`
(names sorted):

```
test a_refused_contact_does_not_run ... ok
test body::contact::tests::a_declarations_section_is_refused ... ok
test body::contact::tests::a_force_word_other_than_combine_is_refused ... ok
test body::contact::tests::a_member_written_twice_is_refused ... ok
test body::contact::tests::a_prim_genome_entry_is_refused ... ok
test body::contact::tests::a_response_named_like_a_cell_is_refused ... ok
test body::contact::tests::a_wires_section_is_refused ... ok
test body::contact::tests::four_spellings_share_one_hash_and_grant_order_moves_it ... ok
test body::contact::tests::the_canonical_text_reparses_to_itself ... ok
test body::contact::tests::the_regulatory_region_is_a_bodys ... ok
test body::tests::four_prim_spellings_canonicalize_byte_identically ... ok
test body::tests::four_spellings_canonicalize_byte_identically ... ok
test check_contact::tests::a_chain_of_forces_is_admitted_and_lowers ... ok
test check_contact::tests::a_force_that_reaches_itself_is_refused ... ok
test check_contact::tests::a_grant_to_no_instance_is_refused ... ok
test check_contact::tests::a_member_count_other_than_the_responses_arity_is_refused ... ok
test check_contact::tests::a_member_in_another_frame_is_refused_at_the_receptor_before_direction ... ok
test check_contact::tests::a_member_of_no_instance_or_no_port_is_refused ... ok
test check_contact::tests::a_missing_genome_cell_is_refused_as_check_body_refuses_it ... ok
test check_contact::tests::a_pin_the_register_does_not_hold_is_refused ... ok
test check_contact::tests::a_port_reached_by_two_forces_is_refused ... ok
test check_contact::tests::a_response_cell_not_supplied_is_refused ... ok
test check_contact::tests::an_in_port_member_is_refused ... ok
test check_contact::tests::the_calculator_is_admitted ... ok
test contact_surface_equals_the_surface_of_the_lowered_body_on_every_contact ... ok
test fns::contact::mutant_lines::tests::every_mutant_answers_as_the_catalogue_says ... ok
test fns::contact::pairings::tests::every_pairing_keeps_the_sum_and_only_the_presentation_moves ... ok
test fns::contact::permutations::tests::orders_are_all_there_once_each ... ok
test fns::layout::layout_text::tests::the_calculator_contact_prints_the_plan_block_and_four_cameras ... ok
test fns::mutate::drop_contact_genome::tests::drop_genome_keeps_the_member_that_names_it ... ok
test fns::mutate::drop_force::tests::drop_force_leaves_no_sum ... ok
test fns::mutate::drop_genome::tests::drop_genome_removes_cli_b_from_surface ... ok
test fns::mutate::drop_wire::tests::drop_wire_changes_surface ... ok
test fns::mutate::neutral::tests::neutral_edits_a_contact_label_and_keeps_its_hash ... ok
test fns::mutate::rename_contact_alias::tests::rename_alias_changes_only_that_name ... ok
test fns::mutate::swap_cell::tests::swap_cell_changes_surface ... ok
test fns::mutate::tests::no_gate_row_names_a_contact_artifact_yet ... ok
test fns::roles_text::tests::a_body_file_is_not_a_contact ... ok
test fns::roles_text::tests::the_calculator_prints_the_predicted_block ... ok
test fns::surface_field::tests::no_body_declares_a_surface_field ... ok
test forces::check_register::tests::a_missing_response_cell_is_refused_by_name ... ok
test forces::check_register::tests::a_row_opposed_by_its_own_response_is_unopposed ... ok
test forces::check_register::tests::a_second_allele_that_depends_on_order_is_refused ... ok
test forces::check_register::tests::the_register_is_admitted ... ok
test forces::order_free::tests::difference_is_refused_on_commutativity ... ok
test forces::order_free::tests::max_and_plus1_are_order_free_though_wrong ... ok
test forces::order_free::tests::midpoint_is_refused_on_associativity ... ok
test forces::order_free::tests::the_sum_allele_is_order_free ... ok
test forces::register::tests::the_one_row_is_combine_on_int_by_sum_opposed_by_sum_turn ... ok
test forces::response::tests::int_has_the_sum_cell_and_rat_has_no_row ... ok
test four_sum_spellings_are_byte_identical ... ok
test layout::layout_contact::tests::a_refused_contact_has_no_layout ... ok
test layout::layout_contact::tests::members_that_do_not_touch_in_one_column_are_refused ... ok
test layout::layout_contact::tests::the_calculator_contact_prints_the_plan_block ... ok
test load::load_contact_file::tests::a_contact_whose_cells_are_missing_is_refused ... ok
test load::load_contact_file::tests::the_contact_calculator_loads_as_the_wired_calculator ... ok
test lower::tests::the_calculator_lowers_to_the_wired_calculator_byte_for_byte ... ok
test roles::body_roles::tests::a_cell_whose_only_port_a_force_reaches_stores ... ok
test roles::body_roles::tests::a_missing_cell_is_refused ... ok
test roles::body_roles::tests::a_response_that_a_force_reaches_responds ... ok
test roles::body_roles::tests::roles_do_not_move_under_the_neutral_edit_or_a_regulatory_rewrite ... ok
test roles::body_roles::tests::the_calculator_is_two_protects_and_a_carry ... ok
test scene::regrow_contact::tests::the_contact_picture_meets_the_probes_seams_and_colors ... ok
test scene::regrow_contact::tests::the_contact_script_keeps_delta_tables_equal_to_regrow ... ok
test seals::register_seal::tests::one_way_is_data_on_the_seal ... ok
test seals::seal_register::tests::seal_register_is_not_empty ... ok
test session::enter::tests::clicking_sum_in_the_contact_picture_names_the_response ... ok
test session::enter::tests::the_shell_run_path_keeps_tables_equal_to_regrow_on_the_contact_calculator ... ok
test session::run_session::tests::a_missing_contact_is_refused_by_path ... ok
test session::run_session::tests::contact_transcript_matches_golden ... ok
test surface::tests::an_extra_wire_makes_a_strictly_smaller_surface ... ok
test surface::tests::calculator_surface_is_three_ports ... ok
test the_calculator_contact_surface_is_three_ports ... ok
test the_contact_calculator_describes_itself_as_the_goldens_say ... ok
test turns::handwritten_turn_alleles::tests::handwritten_count_moves_when_register_grows ... ok
```

## Predictions

Each prediction in the plan, the value it predicted, and the value measured.

### §2.3 The `.contact` file kind

| Prediction | Predicted | Measured | Mark |
|---|---|---|---|
| Canonical text of `calculator.contact` | the ten lines of §2.3 | the same ten lines (above), asserted byte for byte | as predicted |
| Hash | `868e79b293c8d35625f514274eb0bf04d228898ed994a795b23b942a530b6209` | `868e79b293c8d35625f514274eb0bf04d228898ed994a795b23b942a530b6209` | as predicted |
| Four spellings | all four hash `868e79b2…`; grant order `stdin: cli_b, cli_a` hashes differently | all four `868e79b2…`; the swapped grant order hashes differently | as predicted |
| Six parse refusals | the six wordings of §2.3's table | each asserted verbatim by its test (`a_wires_section_is_refused` … `a_member_written_twice_is_refused`) | as predicted |

### §2.5 Derivation

| Prediction | Predicted | Measured | Mark |
|---|---|---|---|
| `print_body(lower(calculator.contact).coding)` | byte-identical to `calculator.body`'s | byte-identical (`lowered = phase2/calculator.body (b55fba1e…)`) | as predicted |
| Its hash | `b55fba1eff65942099f6daf84b8bc47d605be05f637d805d260d3fcfd8c3ebde` | `b55fba1eff65942099f6daf84b8bc47d605be05f637d805d260d3fcfd8c3ebde` | as predicted |
| Pairings | 2 orders, `sum@2` holds `5` in each | `pairings: 2, sum@2 = 5 in each` | as predicted |
| Reversed pairing's presentation | `3 + 2 = 5` | `sum`'s port values `3 2 5` (asserted), so the present line `{0} + {1} = {2}` reads `3 + 2 = 5` | as predicted |

### §2.7 Roles

| Prediction | Predicted | Measured | Mark |
|---|---|---|---|
| `cargo xtask roles` block | cli_a protect, cli_b protect, sum carry; `protect 2, carry 1, store 0, respond 0` | the same five lines (above) | as predicted |
| Chained contact | `s1 respond (faces in, reacts)` | `s1 respond (faces in, reacts)` (asserted) | as predicted |
| Cell whose single out-port a force reaches | `store (faces in, holds)` | `k store (faces in, holds)` (asserted) | as predicted |

### §2.8 Contact layout

| Prediction | Predicted | Measured | Mark |
|---|---|---|---|
| `cargo xtask layout phase7/calculator.contact` | §2.8's twelve lines | the same twelve lines (above), asserted byte for byte in joinn-visual and xtask | as predicted |

### §2.9 The contact scene

| Prediction | Predicted | Measured | Mark |
|---|---|---|---|
| Event 1 `cli_a@0 ← "two"` | touched cli_a; rows 2 (bound 2) | `touched cli_a, rows 2 (bound 2)` | as predicted |
| Event 2 `cli_a@0 ← "2"` | touched cli_a, sum; rows 1 (bound 4) | `touched cli_a, sum, rows 1 (bound 4)` | as predicted |
| Event 3 `cli_b@0 ← "3"` | touched cli_b, sum; rows 3 (bound 4) | `touched cli_b, sum, rows 3 (bound 4)` | as predicted |
| After every event | tables equal `regrow_contact` | `tables equal regrow` on every contact event line, and in the joinn-visual and shell tests | as predicted |

### §2.10 The picture

| Prediction | Predicted | Measured | Mark |
|---|---|---|---|
| Probes at 1280×720 | (192,104) `body surface`, (448,264) `body.cli_a`, (448,456) `body.cli_b`, (832,360) `body.sum`, (256,264) `body.cli_a@0`, (256,456) `body.cli_b@0`, (1024,264) `body.sum@2`, (0,0) `background` | the same eight owners (probe lines above) | as predicted |
| Seam, cli_a and sum | column 639 `body.cli_a`, column 640 `body.sum`, at rows 232, 264, 296 | the same at all three rows | as predicted |
| Seam, cli_a and cli_b | row 359 `body.cli_a`, row 360 `body.cli_b`, at columns 320, 448, 576 | the same at all three columns | as predicted |
| 640×360 | agree 230248, edge 152, owners 7/7 | agree 230248, edge 152, disagree 0, owners 7/7, links 0, on all 3 adapters | as predicted |
| 1000×777 | agree 776784, edge 216, owners 7/7 | agree 776784, edge 216, disagree 0, owners 7/7, links 0, on all 3 adapters | as predicted |
| 1280×720 | agree 921256, edge 344, owners 7/7 | agree 921256, edge 344, disagree 0, owners 7/7, links 0, on all 3 adapters | as predicted |
| 1920×1080 | agree 2073164, edge 436, owners 7/7 | agree 2073164, edge 436, disagree 0, owners 7/7, links 0, on all 3 adapters | as predicted |
| Colors after grow | cli_a `2F5D8A`, cli_b `2F5D8A`, sum `39414D`, the three ports `C9CED6` | `2F5D8A 2F5D8A 39414D C9CED6 C9CED6 C9CED6` | as predicted |
| Colors after event 1 | cli_a `B03A2E`, cli_a@0 `F2B134`, the rest unchanged | `B03A2E 2F5D8A 39414D F2B134 C9CED6 C9CED6` | as predicted |
| Colors after event 2 | cli_a `2F5D8A`, the rest unchanged | `2F5D8A 2F5D8A 39414D F2B134 C9CED6 C9CED6` | as predicted |
| Colors after event 3 | sum `2F5D8A`, cli_b@0 `F2B134`, sum@2 `F2B134` | `2F5D8A 2F5D8A 2F5D8A F2B134 F2B134 F2B134` | as predicted |
| Every event: surface and pixel 0,0 | `22262E`, `15171C` | `22262E`, `15171C` after grow and every event | as predicted |

The colors above are measured from the tables with the shader's selection
rule. The same colors on the GPU, on every adapter, are gate 7 item 3's check
(P7-15).
