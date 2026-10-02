use super::Edition;
use crate::q::frac;

impl Edition {
    pub fn aisc_for_tests() -> Edition {
        Edition {
            source: "AISC Manual, 16th ed.".to_string(),
            material: "A992".to_string(),
            e: frac!(29000, 1),
            shape: "W12x26".to_string(),
            ix: frac!(204, 1),
        }
    }
}
