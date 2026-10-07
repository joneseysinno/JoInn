# Phase 7.4 — A system grows: findings

P74-13. A contact can say how its body grows (`grows { cell as numbers accepts
one|any }`), and a `.system` binds that body to a force. Growing it is a delta
on the grown state, the accepted inputs in order (V160). The engine runs the
body lowered from that state, and every size is true: combine's identity at 0,
the input at 1, the left fold of the inputs at n. After every step
`count_witness` agrees by counting alone (V161). The system's hash never moves
with growth (V162). `adding` evolves `counting`: it keeps all three counting
witnesses and gains `3` (V163). The force is drawn as a lasso around the body,
under every cell, pointing at its response. The CPU and every GPU adapter name
its pixels `force count` alike (V164). The shell opens a `.system`; a click
selects a waiting box, and typing then Enter grows the body or prints its
refusal. Every prediction of §2.10 and §3 held; none differs.

Commits: P74-01 `45d868a`, P74-F1 `e8371f0`, P74-F2 `c9fab0c`, P74-F3
`562051a`, P74-02 `fa6e7cc`, P74-03 `ab48797`, P74-04 `626eaaa`, P74-05
`0428293`, P74-06 `76f3328`, P74-07 `0df0880`, P74-08 `6ce1adb`, P74-09
`3547079`, P74-10 `4d94c96`, P74-11 `0596f31`, P74-12 `cf20667`. Machine:
Windows 10.0.26300, adapters Microsoft Basic Render Driver (Dx12, Cpu) and NVIDIA
GeForce RTX 2080 (Dx12 and Vulkan). The appendix's outputs are from `joinn/`
(cargo's Compiling, Finished and Running lines dropped): the tests at P74-06 and
P74-07; `corpus verify` at P74-08; `grow` at P74-09; `regrow` at P74-10; `pick`
at P74-11; the session test and the window at P74-12. None of their code changed
afterwards, except `grow`, which gained `--measure` here. `check`'s `Grow:` line
was the same at every later commit.

## What was built, by commit

- **P74-F1 … F3, carried from 7.3.** One gate registry (the cross-gate checks
  read `gate_table`, names not addresses, xtask back at the workspace profile);
  standing plants in `links` and the `answered by` half in gate grading;
  canonical order at admission (a grove leg's member is the same from memory
  and from the file).
- **P74-02 … 05, the file kinds.** The mutation catalogue first (`Accepts`,
  `DropForce`, `DropLineage`, `ForceOn`, the system subject kind); `grows` in a
  `.contact`; the `.system` file kind with `TAG_SYSTEM`; `check_system`
  admission.
- **P74-06, growth and the engine.** `Grown`, `grow`, `grow_step` (the one
  delta), `lower_grown` (numbers.0 … n−1 hold stdin by being grown; with
  n ≥ 2 the response cell folds them from the left) and `respond` on joinn-live's
  BodyState. n = 0 reads combine's identity from the response cell's own law.
  The engine cannot add instances, so each step runs the body lowered from the
  state it reaches (P74-06 snag 2). This is what the adversary measures below.
- **P74-07, witness and evolution.** `count_witness` (succ/pred only, refuses
  |v| > 1 000 000), §3's transcripts fixed in source, `check_evolution`
  (returns the witnesses held and the gained input).
- **P74-08, corpus and `grow`.** Four files under `corpus/phase74/`, four
  appended hashes (`corpus verify` 48); `cargo xtask grow` with the witness
  beside the engine and standing plants (a) the fold and (c) the hash; `check`
  gains `Grow:`; CI runs it.
- **P74-09, the system layout and the lasso.** Rows of 6, the response NECK = 6
  right of the outline, an eight-stroke outline 2 units out with cut corners, a
  neck and a two-stroke arrowhead; §2.8's rules checked exactly at every step;
  standing plant (b), the outline drawn 2 units inward.
- **P74-10, the system scene.** `SystemScene`: grow, regrow, `apply_growth`
  (atomic, writes only changed rows), Stroke rows for the lasso in style 15,
  `FORCE_TAG`; `regrow` gains `system … deltas equal regrow at n = 0 … 7`.
- **P74-11, picking the lasso.** `KIND_LASSO`: the curve pass draws style 15
  before bodies, cells and ports; `SystemScene::shapes`, `allows`, `fit`,
  `print_id`; `pick` gains the system lines (V164).
- **P74-12, the shell.** `load_system_file`, `Grower` (open, click, enter, View);
  `joinn-desktop` opens `.system`.
- **P74-13, this file.** `grow --measure`.

## Predictions

### §2.10 The corpus

| Prediction | Plan | Printed | Mark |
|---|---|---|---|
| `corpus verify` | `48 hash(es) match` (44 + 4) | `corpus verify: 48 hash(es) match; cells admitted` | as predicted |
| Corpus diff since P74-01 | only the four files and `hashes.txt` | `git diff --stat 45d868a..HEAD -- joinn/corpus`: `hashes.txt`, `phase74/{adding,counting}.{contact,system}`; 5 files, 69 insertions | as predicted |

### §3 Witness transcripts

| System | Transcript | Plan | Printed | Mark |
|---|---|---|---|---|
| counting | (none) | count 0, cells 0 | `grow counting 0: (none) → cells 0, count 0, witness 0` | as predicted |
| counting | `1 1 1` | counts 1, 2, 3 | `count 1`, `count 2`, `count 3` | as predicted |
| counting | `1 1 3` | 1, 2, then `3` refused; cells stay 2, count 2 | `count 1`, `count 2`, `+3 refused: counting: 3 is not one; acceptance is 1 (counting grows by one)` | as predicted |
| counting | `1` × 12 | count 12 | `grow counting 12: +1 → cells 12, count 12, witness 12` | as predicted |
| adding | (none) | count 0 | `grow adding 0: (none) → cells 0, count 0, witness 0` | as predicted |
| adding | `2 3 4` | counts 2, 5, 9 | `count 2`, `count 5`, `count 9` | as predicted |
| adding | `5 -2 0 7` | counts 5, 3, 3, 10 | `count 5`, `count 3`, `count 3`, `count 10` | as predicted |
| adding | counting's transcripts | the same counts, except `1 1 3` → 1, 2, 5 | `1 1 1` → 1, 2, 3; `1 1 3` → 1, 2, 5; `1` × 12 → 12 | as predicted |

| Prediction | Plan | Printed | Mark |
|---|---|---|---|
| Hash on every step | the system's hash in `hashes.txt`, unchanged | `hash c211e65544e5` on every counting step, `hash 9bd87572c3bc` on every adding step | as predicted |
| Evolution | `evolve counting → adding: 3 witnesses hold` | `evolve counting → adding: 3 witnesses hold; adding accepts 3, counting refuses it` | as predicted |
| Plant (a), the fold drops its last member | the witness refuses at the first n ≥ 2 | `refused at n 2: … V161: count 1 differs from count_witness 2 after [1 1]` | as predicted |
| Plant (b), the lasso 2 units inward | lasso rule 1 refuses | `refused at n 3: lasso: stroke 0 meets the surface; … (rule 1)` | as predicted |
| Plant (c), a hash over the grown state | V162 refuses at the first growth | `refused at n 1: … V162: hash 9abb2dc9414f after [1] is not counting.system's c211e65544e5` | as predicted |
| `grow`'s last line | `grow: 2 systems, every size true, …; planted fold, lasso, hash: refused (ok)` | the same | as predicted |
| `pick` | every earlier line unchanged; per adapter and system at sizes 0, 3, 7: `… disagree 0, owners n (cut allows n), force owners 1 (cut allows 1)` | earlier lines identical to P74-F3's run (frame median aside); 18 system lines, all `disagree 0`, `force owners 1 (cut allows 1)` | as predicted |
| `regrow` | every earlier line unchanged; plus `system <name>: deltas equal regrow at n = 0 … 7` | earlier lines identical to P74-F3's run; the two system lines | as predicted |
| `grow --measure` | one line per system, µs per step at n = 0, 6, 24, 96 | printed (below), with the engine's share beside it | (printed) |

### §2.11, §5 (stated, not numbered predictions)

- **The shell.** The session test (no window) gives `grew numbers.<n>; count 1`,
  2, 3 on counting, then `refused: counting: 3 is not one; acceptance is 1
  (counting grows by one)` with no row pending and the selection kept. A click
  on the neck's midpoint gives `force count (cpu)`. Adding with 2, 3, 4 gives
  `count 9`. The window, once, grew numbers.0 to `count 1` and the GPU agreed.
- **The lasso under its body.** On every adapter, at sizes 0, 3, 7 of both
  systems, the pick disagrees nowhere with the CPU drawing the lasso before
  bodies; the force owns pixels (its own count, 1), and every other owner is one
  the tables allow.

## The adversary: re-lowering on every growth

**Each growth step re-derives and replays the whole body, and the cost grows
with n.** The engine's BodyState cannot add an instance (P74-06 snag 2). So
`respond` lowers the grown state to a fresh body of n grown cells and n − 1
fold stages, and runs it from the start. `apply_growth` then lays out and
writes the rows (only the changed rows enter the delta).

Measured with `grow --measure`: the least of five runs of one step from size n.
*Step* is the scene's `apply_growth` (engine, layout, rows). *Engine* is
`grow_step` then `respond` alone, timed separately, so it can come out slightly
above the step:

| System | Profile | n = 0 | n = 6 | n = 24 | n = 96 | engine at 0 / 6 / 24 / 96 |
|---|---|---|---|---|---|---|
| counting | dev | 164 µs | 487 µs | 1 575 µs | 8 174 µs | 58 / 320 / 1 332 / 8 267 µs |
| adding | dev | 118 µs | 372 µs | 1 451 µs | 8 811 µs | 48 / 278 / 1 242 / 8 660 µs |
| counting | release | 144 µs | 404 µs | 1 366 µs | **7 744 µs** | 51 / 283 / 1 174 / 7 948 µs |
| adding | release | 99 µs | 341 µs | 1 268 µs | **7 830 µs** | 45 / 276 / 1 086 / 7 189 µs |

What the numbers say:

- **The engine is the step.** Layout and rows cost about 100 µs at n = 0 and
  are lost in the noise by n = 24; at n = 96 the engine alone is the whole
  measured step.
- **It grows with n, a little faster than linearly.** In release (counting) a
  step costs about 53 µs more per cell from n = 6 to 24, and about 89 µs per
  cell from 24 to 96. Both counting (a fold of 1s) and adding (a fold of mixed
  integers) behave the same. The cost is the size of the lowered body, not the
  values.
- **Release barely helps** (about 5 to 15 % faster than the dev profile). The
  dev profile already builds dependencies at opt-level 3. The cost is the
  algorithm, re-lowering and replaying, not missed optimisation.
- **At her pace it doesn't matter yet.** A person types one number per Enter;
  8 ms at n = 96 is invisible. A body that grows to thousands of cells, or a
  universe that grows many systems in one tick (7.5), would feel it. That would
  be a finding at that scale, not a reason to change the engine now (plan §7: a
  slow number is a finding, not a workaround).

## What 7.5 needs (observed)

- **Growth that does not re-lower.** If a step is to cost the same at every n,
  the engine needs to add an instance (and a fold stage) to a running body.
  Today it rebuilds and replays, and `grow --measure` is the place to watch
  that number.
- **The universe cut must learn the lasso.** A system's `cut allows` in `pick`
  is `SystemScene::allows`, the owners its tables can show. A system inside a
  universe is drawn through the universe's cut, which knows nothing of
  `FORCE_TAG` or Stroke rows owned by a force. 7.5 has to carry the force
  owner through the cut.
- **The camera does not follow a growing body.** Each growth widens the body
  and moves the response right, so from the first step the opening camera no
  longer frames the system (the session test's first try clicked the old box
  position and hit the surface). The camera is Phase 7.2's, so Home refits, and
  the session test presses it before each click. A system in a universe may want its chart refitted on
  growth, or a frame that leaves room.
- **Waiting boxes are fixed by the file.** `waiting { numbers 1 }` shows one
  empty box at every size, and after each growth the next one is selected. Two
  bodies in one system (R117) would need a waiting count per body and a rule
  for which box is selected next.
- **Text is refused by the body, not the shell.** Non-integer text goes to the
  body as a Text value, and the body's own words refuse it. 7.5's systems with
  other frames get the right refusal without the shell knowing the frame.

## Appendix: command outputs

### `cargo test -p joinn-link grow` (P74-06, P74-07)

```
grow::tests::every_size_is_true_on_counting_and_adding ... ok
grow::tests::counting_refuses_three_in_its_words ... ok
grow::tests::growth_does_not_move_the_system_s_hash ... ok
grow::tests::deltas_equal_regrow_at_every_size ... ok
grow::tests::count_witness_and_the_engine_agree_on_every_transcript_at_every_step ... ok
grow::tests::count_witness_refuses_too_many_steps ... ok
grow::tests::adding_evolves_counting_and_a_counting_with_nothing_new_is_refused ... ok
```

### `cargo xtask corpus verify` (P74-08)

```
corpus verify: refused false_law.cell by name
corpus verify: 48 hash(es) match; cells admitted
```

### `cargo xtask grow` (P74-09)

```
grow counting 0: (none) → cells 0, count 0, witness 0, lasso ok, hash c211e65544e5
grow counting 1: +1 → cells 1, count 1, witness 1, lasso ok, hash c211e65544e5
grow counting 2: +1 → cells 2, count 2, witness 2, lasso ok, hash c211e65544e5
grow counting 3: +1 → cells 3, count 3, witness 3, lasso ok, hash c211e65544e5
grow counting 1: +1 → cells 1, count 1, witness 1, lasso ok, hash c211e65544e5
grow counting 2: +1 → cells 2, count 2, witness 2, lasso ok, hash c211e65544e5
grow counting 3: +3 refused: counting: 3 is not one; acceptance is 1 (counting grows by one)
grow counting 1: +1 → cells 1, count 1, witness 1, lasso ok, hash c211e65544e5
grow counting 2: +1 → cells 2, count 2, witness 2, lasso ok, hash c211e65544e5
grow counting 3: +1 → cells 3, count 3, witness 3, lasso ok, hash c211e65544e5
grow counting 4: +1 → cells 4, count 4, witness 4, lasso ok, hash c211e65544e5
grow counting 5: +1 → cells 5, count 5, witness 5, lasso ok, hash c211e65544e5
grow counting 6: +1 → cells 6, count 6, witness 6, lasso ok, hash c211e65544e5
grow counting 7: +1 → cells 7, count 7, witness 7, lasso ok, hash c211e65544e5
grow counting 8: +1 → cells 8, count 8, witness 8, lasso ok, hash c211e65544e5
grow counting 9: +1 → cells 9, count 9, witness 9, lasso ok, hash c211e65544e5
grow counting 10: +1 → cells 10, count 10, witness 10, lasso ok, hash c211e65544e5
grow counting 11: +1 → cells 11, count 11, witness 11, lasso ok, hash c211e65544e5
grow counting 12: +1 → cells 12, count 12, witness 12, lasso ok, hash c211e65544e5
grow adding 0: (none) → cells 0, count 0, witness 0, lasso ok, hash 9bd87572c3bc
grow adding 1: +2 → cells 1, count 2, witness 2, lasso ok, hash 9bd87572c3bc
grow adding 2: +3 → cells 2, count 5, witness 5, lasso ok, hash 9bd87572c3bc
grow adding 3: +4 → cells 3, count 9, witness 9, lasso ok, hash 9bd87572c3bc
grow adding 1: +5 → cells 1, count 5, witness 5, lasso ok, hash 9bd87572c3bc
grow adding 2: +-2 → cells 2, count 3, witness 3, lasso ok, hash 9bd87572c3bc
grow adding 3: +0 → cells 3, count 3, witness 3, lasso ok, hash 9bd87572c3bc
grow adding 4: +7 → cells 4, count 10, witness 10, lasso ok, hash 9bd87572c3bc
grow adding 1: +1 → cells 1, count 1, witness 1, lasso ok, hash 9bd87572c3bc
grow adding 2: +1 → cells 2, count 2, witness 2, lasso ok, hash 9bd87572c3bc
grow adding 3: +1 → cells 3, count 3, witness 3, lasso ok, hash 9bd87572c3bc
grow adding 1: +1 → cells 1, count 1, witness 1, lasso ok, hash 9bd87572c3bc
grow adding 2: +1 → cells 2, count 2, witness 2, lasso ok, hash 9bd87572c3bc
grow adding 3: +3 → cells 3, count 5, witness 5, lasso ok, hash 9bd87572c3bc
grow adding 1: +1 → cells 1, count 1, witness 1, lasso ok, hash 9bd87572c3bc
grow adding 2: +1 → cells 2, count 2, witness 2, lasso ok, hash 9bd87572c3bc
grow adding 3: +1 → cells 3, count 3, witness 3, lasso ok, hash 9bd87572c3bc
grow adding 4: +1 → cells 4, count 4, witness 4, lasso ok, hash 9bd87572c3bc
grow adding 5: +1 → cells 5, count 5, witness 5, lasso ok, hash 9bd87572c3bc
grow adding 6: +1 → cells 6, count 6, witness 6, lasso ok, hash 9bd87572c3bc
grow adding 7: +1 → cells 7, count 7, witness 7, lasso ok, hash 9bd87572c3bc
grow adding 8: +1 → cells 8, count 8, witness 8, lasso ok, hash 9bd87572c3bc
grow adding 9: +1 → cells 9, count 9, witness 9, lasso ok, hash 9bd87572c3bc
grow adding 10: +1 → cells 10, count 10, witness 10, lasso ok, hash 9bd87572c3bc
grow adding 11: +1 → cells 11, count 11, witness 11, lasso ok, hash 9bd87572c3bc
grow adding 12: +1 → cells 12, count 12, witness 12, lasso ok, hash 9bd87572c3bc
evolve counting → adding: 3 witnesses hold; adding accepts 3, counting refuses it
planted fold drops its last member: refused at n 2: grow counting 2: V161: count 1 differs from count_witness 2 after [1 1]
planted lasso drawn 2 units inward: refused at n 3: lasso: stroke 0 meets the surface; acceptance is a lasso clear of every cell and the surface (rule 1)
planted hash over the grown state: refused at n 1: grow counting 1: V162: hash 9abb2dc9414f after [1] is not counting.system's c211e65544e5
grow: 2 systems, every size true, counting witnesses every step; evolution holds; planted fold, lasso, hash: refused (ok)
```

### `cargo xtask regrow` (P74-10; the lines new since P74-F3, every earlier line unchanged)

```
system counting: deltas equal regrow at n = 0 … 7
system adding: deltas equal regrow at n = 0 … 7
regrow: 3 adapter(s); planted difference: refused (ok)
```

### `cargo xtask pick` (P74-11; the system lines of each adapter, every earlier line unchanged; the whole output is in P74-11's commit message)

The same six lines on Microsoft Basic Render Driver · Dx12, NVIDIA GeForce RTX
2080 · Dx12 and NVIDIA GeForce RTX 2080 · Vulkan:

```
system counting n 0: agree 920890, edge 710, disagree 0, owners 5 (cut allows 5), force owners 1 (cut allows 1)
system counting n 3: agree 921536, edge 64, disagree 0, owners 11 (cut allows 11), force owners 1 (cut allows 1)
system counting n 7: agree 921504, edge 96, disagree 0, owners 19 (cut allows 19), force owners 1 (cut allows 1)
system adding n 0: agree 921448, edge 152, disagree 0, owners 7 (cut allows 7), force owners 1 (cut allows 1)
system adding n 3: agree 921528, edge 72, disagree 0, owners 13 (cut allows 13), force owners 1 (cut allows 1)
system adding n 7: agree 921492, edge 108, disagree 0, owners 21 (cut allows 21), force owners 1 (cut allows 1)
pick: 3 adapter(s), 12 subject(s) agree; planted disagreement: refused as truth violation (ok)
```

### `cargo test -p joinn-shell-desktop --lib system_` (P74-12)

```
session::system_enter::tests::adding_grows_two_three_four_to_nine ... ok
session::system_enter::tests::counting_grows_to_three_refuses_three_and_its_lasso_names_the_force ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out
```

### The window, once (P74-12): `joinn-desktop.exe corpus/phase74/counting.system`, input posted to it

```
adapter: NVIDIA GeForce RTX 2080 · Dx12 · DiscreteGpu · vertex storage yes
surface: Bgra8Unorm
pick 400,360: numbers.0 (cpu)
selected numbers.0
  typing: 1
intent numbers.0 "1"
grew numbers.0; count 1
tick: level 4 step 128 (k 24), anchor system, rows 38
        numbers.0 (gpu) · agree
```

### `cargo xtask grow --measure` (dev profile, P74-13)

```
grow counting µs per step at n = 0, 6, 24, 96: 164, 487, 1575, 8174 (engine 58, 320, 1332, 8267)
grow adding µs per step at n = 0, 6, 24, 96: 118, 372, 1451, 8811 (engine 48, 278, 1242, 8660)
grow --measure: 2 systems, least of 5 runs (information, not a check)
```

### `cargo run --release -p xtask -- grow --measure` (P74-13)

```
grow counting µs per step at n = 0, 6, 24, 96: 144, 404, 1366, 7744 (engine 51, 283, 1174, 7948)
grow adding µs per step at n = 0, 6, 24, 96: 99, 341, 1268, 7830 (engine 45, 276, 1086, 7189)
grow --measure: 2 systems, least of 5 runs (information, not a check)
```
