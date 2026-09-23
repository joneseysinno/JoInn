use super::artifact_loads;
/// Phase 2 alleles control: an empty turn register must not count as present.

pub(crate) fn g2_alleles_control(art: &joinn_gate::Artifact) -> bool {
    if !artifact_loads(art) {
        return true;
    }
    joinn_prim::handwritten_turn_alleles() == 0
}
