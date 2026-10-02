use super::{Example, PointLoad, Support};
use crate::q::{Q, frac, scale};
use crate::tag::Tag;

/// The five beams, all W12x26, A992. Inputs are in feet and kip/ft,
/// converted exactly to inch (12 in = 1 ft).
pub fn examples() -> Vec<Example> {
    let ft = |n: i64| scale(&Q::new(frac!(n, 1), Tag::PLACE_X), &frac!(12, 1));
    let kip = |n: i64| Q::new(frac!(n, 1), Tag::FORCE);
    let w_6_5_down = || Some(scale(&Q::new(frac!(-6, 5), Tag::DENSITY), &frac!(1, 12)));
    let at = |x: i64, p: i64| PointLoad {
        at: ft(x),
        force: kip(p),
    };
    vec![
        Example {
            id: "E1",
            title: "simple span, uniform load",
            support: Support::Simple,
            span: ft(24),
            uniform: w_6_5_down(),
            point_loads: vec![],
            moment_sections: vec![ft(12)],
            deflection_sections: vec![ft(12)],
        },
        Example {
            id: "E2",
            title: "simple span, point load at midspan",
            support: Support::Simple,
            span: ft(24),
            uniform: None,
            point_loads: vec![at(12, -10)],
            moment_sections: vec![ft(12)],
            deflection_sections: vec![ft(12)],
        },
        Example {
            id: "E3",
            title: "simple span, point load at 6 ft",
            support: Support::Simple,
            span: ft(24),
            uniform: None,
            point_loads: vec![at(6, -10)],
            moment_sections: vec![ft(6)],
            deflection_sections: vec![ft(6)],
        },
        Example {
            id: "E4",
            title: "cantilever, point load at the tip",
            support: Support::Cantilever,
            span: ft(10),
            uniform: None,
            point_loads: vec![at(10, -5)],
            moment_sections: vec![ft(0)],
            deflection_sections: vec![ft(10)],
        },
        Example {
            id: "E5",
            title: "simple span, uniform load and point load at 6 ft",
            support: Support::Simple,
            span: ft(24),
            uniform: w_6_5_down(),
            point_loads: vec![at(6, -10)],
            moment_sections: vec![ft(6), ft(12)],
            deflection_sections: vec![ft(6), ft(12)],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::examples;
    use crate::q::{Q, frac};
    use crate::tag::Tag;

    #[test]
    fn inputs_convert_exactly_to_kip_and_inch() {
        let all = examples();
        let ids: Vec<&str> = all.iter().map(|e| e.id()).collect();
        assert_eq!(ids, ["E1", "E2", "E3", "E4", "E5"]);
        assert_eq!(all[0].span(), &Q::new(frac!(288, 1), Tag::PLACE_X));
        assert_eq!(all[0].uniform(), Some(&Q::new(frac!(-1, 10), Tag::DENSITY)));
        assert_eq!(all[3].span(), &Q::new(frac!(120, 1), Tag::PLACE_X));
        assert_eq!(
            all[2].point_loads()[0].at(),
            &Q::new(frac!(72, 1), Tag::PLACE_X)
        );
    }
}
