//! Test fixture: a body's `DropGenome(instance)` form, as `cargo xtask` mutates it.

use joinn_dna::Body;

/// Drops the instance from the genome, its wires, and its grants.
pub(crate) fn dropped(body: &Body, instance: &str) -> Body {
    let mut b = body.clone();
    for entry in &mut b.coding.genome {
        entry.instances.retain(|i| i != instance);
    }
    b.coding.genome.retain(|e| !e.instances.is_empty());
    b.coding
        .wires
        .retain(|w| w.src_instance != instance && w.dst_instance != instance);
    for insts in b.coding.grants.values_mut() {
        insts.retain(|i| i != instance);
    }
    b.coding.grants.retain(|_, insts| !insts.is_empty());
    b
}
