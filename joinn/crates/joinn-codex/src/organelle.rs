//! The organelle list: a declared, read-only list in the runtime (plan 8
//! §2.8). Phase 8 only declares it. Grant and revoke act on membranes, not
//! organelles.

mod acid_organelles;
mod name;

/// A part of the runtime a world amino acid uses.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Organelle {
    /// Used by `show`.
    Screen,
    /// Used by `sense`.
    Input,
    /// Used by `keep` and `recall`.
    Store,
    /// Used by `sense`.
    Clock,
}

/// The list, in the plan's order.
pub const ORGANELLES: [Organelle; 4] = [
    Organelle::Screen,
    Organelle::Input,
    Organelle::Store,
    Organelle::Clock,
];

#[cfg(test)]
mod tests {
    use super::ORGANELLES;
    use crate::meaning::AminoAcid;

    const WORLD: [AminoAcid; 4] = [
        AminoAcid::Sense,
        AminoAcid::Show,
        AminoAcid::Keep,
        AminoAcid::Recall,
    ];

    #[test]
    fn every_world_acid_names_a_listed_organelle() {
        for a in WORLD {
            assert!(!a.organelles().is_empty(), "{}", a.name());
            for o in a.organelles() {
                assert!(ORGANELLES.contains(o), "{}", o.name());
            }
        }
    }

    #[test]
    fn every_organelle_is_named_and_only_world_acids_name_one() {
        for o in ORGANELLES {
            assert!(
                AminoAcid::ALL.iter().any(|a| a.organelles().contains(&o)),
                "{}",
                o.name()
            );
        }
        for a in AminoAcid::ALL {
            assert_eq!(
                !a.organelles().is_empty(),
                WORLD.contains(&a),
                "{}",
                a.name()
            );
        }
    }
}
