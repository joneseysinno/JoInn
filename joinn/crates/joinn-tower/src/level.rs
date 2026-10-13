//! Segment levels and the level filter. Segment i's level is the number of
//! trailing 3s in i written in base 4: how many block sizes (4, 16, 64, …) the
//! segment bridges. Floor f keeps the segments of level ≥ f.

mod level_filter;
mod trailing_threes;

pub use level_filter::level_filter;
pub use trailing_threes::level;

#[cfg(test)]
mod tests {
    use super::{level, level_filter};

    #[test]
    fn levels_count_trailing_threes() {
        assert_eq!(level(0), 0);
        assert_eq!(level(3), 1);
        assert_eq!(level(7), 1);
        assert_eq!(level(15), 2);
        assert_eq!(level(31), 2);
        assert_eq!(level(63), 3);
        assert_eq!(level(0b1011), 1);
    }

    #[test]
    fn floor_0_keeps_everything_and_floor_1_keeps_every_fourth() {
        let items: Vec<u64> = (0..15).collect();
        assert_eq!(level_filter(&items, 0), items);
        assert_eq!(level_filter(&items, 1), [3, 7, 11]);
        assert_eq!(level_filter(&items, 2), Vec::<u64>::new());
    }
}
