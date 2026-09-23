# Turn annotations (R31)

Hand-written turn alleles, derived from the turn register by
`handwritten_turn_alleles()`.

There are two:

- `int.add.turn0` — `turn 0 from {1 2}` of `Sum`, filling port 0 from ports 1 and 2
- `add@ℤ.turn1` — `turn 1 from {0 2}` of `Sum`, filling port 1 from ports 0 and 2

`succ`/`pred` remain the free case derived from the frame signature; they are
not counted. The count is the register's length. Registering another hand-written
allele moves the printed number; that is P21-15's teeth.

Typed by hand from the printed count. `xtask` does not write this file.
