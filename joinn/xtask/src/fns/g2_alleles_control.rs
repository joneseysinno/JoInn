/// Phase 2 alleles control: an empty turn register must not count as present.

pub(crate) fn g2_alleles_control(_art: &()) -> bool {
    joinn_prim::handwritten_turn_alleles() == 0
}
