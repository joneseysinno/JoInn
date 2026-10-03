//! One zoom: `level <L> step <t> (k <fraction>)`.

use super::Zoom;

/// `level -2 step 95 (k 351/1024)`; a whole `k` prints without a denominator.
pub fn print_zoom(zoom: &Zoom) -> String {
    let k = match zoom.k() {
        (num, 1) => num.to_string(),
        (num, den) => format!("{num}/{den}"),
    };
    format!("level {} step {} (k {k})", zoom.level, zoom.step)
}

#[cfg(test)]
mod tests {
    use super::print_zoom;
    use crate::camera::Zoom;

    #[test]
    fn fractions_and_whole_k_print_as_the_plan_writes_them() {
        assert_eq!(
            print_zoom(&Zoom {
                level: -2,
                step: 95
            }),
            "level -2 step 95 (k 351/1024)"
        );
        assert_eq!(
            print_zoom(&Zoom { level: -4, step: 0 }),
            "level -4 step 0 (k 1/16)"
        );
        assert_eq!(
            print_zoom(&Zoom { level: 0, step: 0 }),
            "level 0 step 0 (k 1)"
        );
    }
}
