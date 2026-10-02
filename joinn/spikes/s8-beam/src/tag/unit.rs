use super::{Side, Tag};

impl Tag {
    /// The unit a tag prints, from its side and length: kip-based for source
    /// and energy, inch-based for placement. Axis never shows in a unit.
    pub fn unit(&self) -> String {
        let power = |n: u32| -> String {
            if n == 1 {
                return String::new();
            }
            n.to_string()
                .chars()
                .map(|c| match c {
                    '0' => '⁰',
                    '1' => '¹',
                    '2' => '²',
                    '3' => '³',
                    '4' => '⁴',
                    '5' => '⁵',
                    '6' => '⁶',
                    '7' => '⁷',
                    '8' => '⁸',
                    _ => '⁹',
                })
                .collect()
        };
        let n = self.length.unsigned_abs();
        match (self.side, self.length.signum()) {
            (Side::Source | Side::Energy, 0) => "kip".to_string(),
            (Side::Source | Side::Energy, 1) => format!("kip·in{}", power(n)),
            (Side::Source | Side::Energy, _) => format!("kip/in{}", power(n)),
            (Side::Placement, 0) => "1".to_string(),
            (Side::Placement, 1) => format!("in{}", power(n)),
            (Side::Placement, _) => format!("1/in{}", power(n)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Axis, Side, Tag};

    #[test]
    fn units_print_from_side_and_length_only() {
        assert_eq!(Tag::FORCE.unit(), "kip");
        assert_eq!(Tag::MOMENT.unit(), "kip·in");
        assert_eq!(Tag::WORK.unit(), "kip·in");
        assert_eq!(Tag::DENSITY.unit(), "kip/in");
        assert_eq!(Tag::PLACE_X.unit(), "in");
        assert_eq!(Tag::DEFLECTION.unit(), "in");
        assert_eq!(Tag::ROTATION.unit(), "1");
        assert_eq!(Tag::CURVATURE.unit(), "1/in");
        assert_eq!(Tag::new(Side::Placement, 4, Axis::None).unit(), "in⁴");
        assert_eq!(Tag::new(Side::Source, 2, Axis::None).unit(), "kip·in²");
    }
}
