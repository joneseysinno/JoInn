# Phase 7 Stop B: end of chunk B (P7-06a to P7-11)

Recorded by Cursor on 30 Sep 2026 from `D:\JoInn` on Windows (PowerShell).
Values below are copied from the terminal, not summarised.

`git rev-parse HEAD` after P7-11 (last commit of chunk B):

```
7d7076c527b4249c1869d55d28c174ca7ac0e960
```

The stop-report commit that adds this file is `P7-stop-b` on `main` immediately after that.

## Commit reports (in order)

```
Commit:     P7-06a c5b7aec8b7e5800a6882f31fee97ed34e310fb16
Done-when:  cargo xtask forces → the same seven lines as Stop A, byte for byte          MET
            cargo test -p joinn-prim forces → test forces::check_register::tests::a_second_allele_that_depends_on_order_is_refused ... ok          MET
            grep -rn "kept.alleles" xtask → (nothing)          MET
Suite:      cargo test --workspace --no-fail-fast → 267 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      none
```

```
Commit:     P7-07 f0f678895e3e368d91e9fefa01c5d0ab1b27f1e8
Done-when:  cargo test -p joinn-dna contact → test result: ok. 9 passed; 0 failed          MET
            body::contact::tests::four_spellings_share_one_hash_and_grant_order_moves_it ... ok
            (the source; members reversed + sections reordered + comments; one line; regulatory only: all 868e79b2…; grant order swapped: differs)
            each §2.3 parse refusal has a test: a_wires_section_is_refused, a_force_word_other_than_combine_is_refused,
            a_prim_genome_entry_is_refused, a_declarations_section_is_refused, a_response_named_like_a_cell_is_refused,
            a_member_written_twice_is_refused ... ok
            cargo xtask corpus verify → corpus verify: 44 hash(es) match; cells admitted          MET
            git diff --stat caeff0c..HEAD -- joinn/corpus →          MET
              joinn/corpus/hashes.txt                |  2 ++
              joinn/corpus/phase7/calculator.contact | 23 +++++++++++++++++++++++
              2 files changed, 25 insertions(+)
Suite:      cargo test --workspace --no-fail-fast → 276 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      P7-07: the first suite run printed `test tests::regulatory_cannot_be_hashed ... FAILED` (275 passed, 1 failed): trybuild's
            `tests/fail/hash_regulatory.rs ... mismatch` and `tests/fail/hash_body_regulatory.rs ... mismatch`. Both still fail to
            compile (neither regulatory region is hashable); rustc's "the following other types implement trait `Genotype`" list
            now also names `ContactCoding`. The two .stderr files gain exactly that entry (5 lines each), nothing else.
```

```
Commit:     P7-08 c088cb3b0d313964ec8aa1b024964bde40897c0f
Done-when:  cargo test -p xtask mutate → test result: ok. 30 passed; 0 failed          MET
            fns::mutate::tests::no_gate_row_names_a_contact_artifact_yet ... ok          MET
Suite:      cargo test --workspace --no-fail-fast → 284 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      P7-08: §2.14 tests each mutation by its admission outcome (admitted, refused at the pin, at the receptor naming
            cli_a@0, at arity, member names no instance). Those outcomes are check_contact's, which P7-09 delivers after this
            commit (§4: P7-08, then P7-09). P7-08's tests assert what each mutant is; P7-09 adds the admission outcome of each
            §2.14 mutant, with its wording.
```

```
Commit:     P7-09 8975fb60ea808ca39dc906e417f6dda7d464c594
Done-when:  cargo test -p joinn-link contact → check_contact tests: test result: ok. 10 passed; 0 failed          MET
              each §2.4 refusal with its wording: a_missing_genome_cell_is_refused_as_check_body_refuses_it,
              a_grant_to_no_instance_is_refused, a_member_of_no_instance_or_no_port_is_refused,
              a_member_in_another_frame_is_refused_at_the_receptor_before_direction, an_in_port_member_is_refused,
              a_port_reached_by_two_forces_is_refused, a_pin_the_register_does_not_hold_is_refused (and ℚ unregistered),
              a_member_count_other_than_the_responses_arity_is_refused, a_response_cell_not_supplied_is_refused ... ok
            cargo test -p joinn-link lower → lower::tests::the_calculator_lowers_to_the_wired_calculator_byte_for_byte ... ok          MET
              (print_body(lower(calculator.contact).coding) == print_body(calculator.body); hash b55fba1eff65942099f6daf84b8bc47d605be05f637d805d260d3fcfd8c3ebde: as predicted)
            tests/contact.rs → contact_surface_equals_the_surface_of_the_lowered_body_on_every_contact ... ok;
              the_calculator_contact_surface_is_three_ports ... ok (cli_a@0 in Text 1, cli_b@0 in Text 1, sum@2 out ℤ 1)          MET
Suite:      cargo test --workspace --no-fail-fast → 297 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      P7-09: §2.6 writes `contact_surface(contact, cells) -> Verdict<Vec<SurfacePort>>`. No SurfacePort type exists; it
            returns the same `BTreeSet<BoundaryPort>` that `surface` (∂(body)) returns, so the two derivations compare as sets
            of (instance, port, direction, frame) directly.
```

```
Commit:     P7-10 ed9264fcaab2c787a87a9a9f5fbe5b791b31d67e
Done-when:  cargo xtask contact → contact: 1 body(ies); two forms, one truth          MET
            cargo run -p joinn-cli -- run corpus/phase7/calculator.contact → the five transcript lines          MET
Suite:      cargo test --workspace --no-fail-fast → 306 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      (1) The shell's loader accepts .contact (parse, check, lower), but
            `cargo run -p joinn-shell-desktop -- corpus/phase7/calculator.contact`
            admits the file and then refuses to open a window: "… is admitted; a
            contact body is drawn as cells in contact, never as its lowered wires,
            and that picture is not built yet. acceptance is a .body file".
            Drawing the lowered body would draw the derived wires, which rule 61
            forbids; the contact picture is P7-12 and the shell opening it is P7-13.
            (2) The new tests for a contact with no cells first expected an
            "acceptance is" wording; the refusal is check_body's existing
            "unknown cell hash c4a0…", and the tests assert that exact text.
```

```
Commit:     P7-11 7d7076c527b4249c1869d55d28c174ca7ac0e960
Done-when:  cargo xtask roles corpus/phase7/calculator.contact → roles: 3 cell(s); protect 2, carry 1, store 0, respond 0          MET
            invariance, respond and store tests → pass          MET
Suite:      cargo test --workspace --no-fail-fast → 313 passed, 0 failed
Scans:      vocab → vocab: ok · modules → modules: ok (enforced 14 crate(s)) · layers → layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)
Snags:      none
```

`cargo xtask contact` at P7-11 (whole output):

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

`cargo run -p joinn-cli -- run corpus/phase7/calculator.contact` with stdin `two`, `2`, `3`:

```
a: two
   refused at membrane: "two" is not in ℤ
a: 2
b: 3
2 + 3 = 5
```

`cargo xtask roles corpus/phase7/calculator.contact`:

```
roles calculator.contact
cli_a protect (faces out, holds)
cli_b protect (faces out, holds)
sum carry (faces out, reacts)
roles: 3 cell(s); protect 2, carry 1, store 0, respond 0
```

## Last line of each command (end of chunk B, on 7d7076c)

- `cargo test --workspace --no-fail-fast` — `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s` (the last doc-test binary); summed over every `test result:` line: `313 passed, 0 failed` (exit 0)
- `cargo xtask gate all` — `gate all wall milliseconds: 430233` (exit 0)
- `cargo xtask corpus verify` — `corpus verify: 44 hash(es) match; cells admitted` (exit 0)
- `cargo xtask vocab` — `vocab: ok` (exit 0)
- `cargo xtask modules` — `modules: ok (enforced 14 crate(s))` (exit 0)
- `cargo xtask layers` — `layers: ok (14 crate(s); wgpu in joinn-gpu, joinn-shell-desktop; winit in joinn-shell-desktop)` (exit 0)
- `cargo xtask floor` — `floor: 16 members, paired` (exit 0)
- `cargo xtask assay agree` — `assay agree: 38 subject(s) agree, 1 not measured; injected disagreements: 2 refused as truth violation (ok)` (exit 0)
- `cargo xtask adapters` — `adapters: 3` (exit 0)
- `cargo xtask pick` — `pick: 3 adapter(s), 12 subject(s) agree; planted disagreement: refused as truth violation (ok)` (exit 0)
- `cargo xtask regrow` — `regrow: 3 adapter(s); planted difference: refused (ok)` (exit 0)
- `cargo xtask forces` — `forces: 1 combine registered, 1 opposed; plants: difference refused, midpoint refused, max and plus1 order-free but unregistered, unopposed refused (ok)` (exit 0)
- `cargo xtask contact` — `contact: 1 body(ies); two forms, one truth` (exit 0)
- `cargo xtask roles corpus/phase7/calculator.contact` — `roles: 3 cell(s); protect 2, carry 1, store 0, respond 0` (exit 0)

Phase lines from that `gate all`:

```
phase 0: pass
phase 1: 4/4 legacy
phase 2: 4/4 legacy
phase 2.1: 8/8 legacy
phase 2.2: 9/9 legacy
phase 3: 8/8 legacy
phase 4: 4/4
phase 5: 8/8
phase 5.1: 4/4
phase 5.2: 3/3
phase 6: 3/3
```

### Every `gate all` line containing `fail`

```
1 ok  references agree; a blind seal fails the item
```

### Every failing test name

none (suite on 7d7076c: 313 passed, 0 failed)

## CI

Newest run after the push of `7d7076c` (P7-06a to P7-11, pushed together as `5fa61e1..7d7076c`), read with `Invoke-RestMethod` on `https://api.github.com/repos/joneseysinno/JoInn/actions/runs?branch=main&per_page=3` and that run's `jobs_url`. The reads from 11:10 to 11:14 printed `status in_progress`; this is the read at 11:15 after it finished.

```
head_sha: 7d7076c527b4249c1869d55d28c174ca7ac0e960
status: completed
conclusion: success
check (ubuntu-24.04): success
check (windows-latest): success
```

The new `contact` step passed on both jobs (`step contact: success`).

Run: https://github.com/joneseysinno/JoInn/actions/runs/36748051690

## Snags (all from the chunk)

- **P7-07**: the first suite run printed `test tests::regulatory_cannot_be_hashed ... FAILED` (275 passed, 1 failed): trybuild's `tests/fail/hash_regulatory.rs ... mismatch` and `tests/fail/hash_body_regulatory.rs ... mismatch`. Both still fail to compile (neither regulatory region is hashable); rustc's "the following other types implement trait `Genotype`" list now also names `ContactCoding`. The two `.stderr` files gain exactly that entry (5 lines each), nothing else.
- **P7-08**: §2.14 tests each mutation by its admission outcome (admitted, refused at the pin, at the receptor naming `cli_a@0`, at arity, member names no instance). Those outcomes are `check_contact`'s, which P7-09 delivers after this commit (§4: P7-08, then P7-09). P7-08's tests assert what each mutant is; P7-09 adds the admission outcome of each §2.14 mutant, with its wording.
- **P7-09**: §2.6 writes `contact_surface(contact, cells) -> Verdict<Vec<SurfacePort>>`. No `SurfacePort` type exists; it returns the same `BTreeSet<BoundaryPort>` that `surface` (∂(body)) returns, so the two derivations compare as sets of (instance, port, direction, frame) directly.
- **P7-10**: the plan's P7-10 row says the shell's loader accepts `.contact` (parse → check → lower → the existing run path). The loader does (`load_contact_file`, tested equal to `calculator.body`'s coding and regulatory), but `cargo run -p joinn-shell-desktop -- corpus/phase7/calculator.contact` admits the file and then refuses to open a window: `corpus/phase7/calculator.contact is admitted; a contact body is drawn as cells in contact, never as its lowered wires, and that picture is not built yet. acceptance is a .body file`. The existing run path draws with `Scene::grow`, which would draw the lowered body's derived wires, and rule 61 says the derivation is never drawn. The contact picture is P7-12, and the shell opening `.contact` is P7-13.
- **P7-10**: the new tests for a contact whose cells are missing (`joinn-test-host` `a_refused_contact_does_not_run`, shell `a_contact_whose_cells_are_missing_is_refused`) first expected an `acceptance is` wording and failed. Admission's step 1 is `check_body`, whose existing refusal is `unknown cell hash c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e`; both tests now assert that exact text.
- **All commits**: git normalises the report block's `Commit:`, `Done-when:`, `Suite:`, `Scans:`, `Snags:` lines as trailers, so `git log` shows them with one space after the colon. The blocks above restore §0.1's alignment; the values are unchanged.
