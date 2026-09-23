//! Count of hand-written turn alleles.

use super::turn_register::turn_register;

/// Count of hand-written turn alleles.
pub fn handwritten_turn_alleles() -> u32 {
    turn_register().len() as u32
}

#[cfg(test)]
mod tests {
    use super::super::turn_register::turn_register;
    use super::handwritten_turn_alleles;

    #[test]
    fn handwritten_count_moves_when_register_grows() {
        assert_eq!(handwritten_turn_alleles(), 2);
        let n = turn_register().len() as u32;
        assert_eq!(n, handwritten_turn_alleles());
    }
}
