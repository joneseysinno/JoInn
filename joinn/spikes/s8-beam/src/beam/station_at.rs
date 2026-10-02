use super::Station;
use crate::q::{Q, frac, scale};
use crate::show::fraction;
use crate::verdict::{Verdict, refuse};

/// The walk's station at x. A section is always a point of the complex.
pub fn station_at<'a>(stations: &'a [Station], x: &Q) -> Verdict<&'a Station> {
    match stations.iter().find(|s| &s.x == x) {
        Some(station) => Verdict::Admitted(station),
        None => refuse(format!(
            "section: {} ft is not a point of the complex; acceptance is a printed section, which the complex always holds",
            fraction(scale(x, &frac!(1, 12)).value())
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::station_at;
    use crate::beam::{Order, derive, examples};
    use crate::bridge::Bridge;
    use crate::q::{Q, frac, scale};
    use crate::tag::Tag;
    use crate::verdict::admitted;

    /// Every M §2.8 prints, in kip·ft, signed: sagging positive.
    #[test]
    fn every_printed_moment_is_the_plan_value() {
        let want: [(&str, i64, Q); 6] = [
            ("E1", 12, Q::new(frac!(432, 5), Tag::MOMENT)),
            ("E2", 12, Q::new(frac!(60, 1), Tag::MOMENT)),
            ("E3", 6, Q::new(frac!(45, 1), Tag::MOMENT)),
            ("E4", 0, Q::new(frac!(-50, 1), Tag::MOMENT)),
            ("E5", 6, Q::new(frac!(549, 5), Tag::MOMENT)),
            ("E5", 12, Q::new(frac!(582, 5), Tag::MOMENT)),
        ];
        let all = examples();
        let mut printed = 0;
        for e in &all {
            let d = admitted(derive(e, false, Order::Faithful, &mut Bridge::new(None)));
            for x in e.moment_sections() {
                let s = admitted(station_at(d.stations(), x));
                let feet = scale(x, &frac!(1, 12));
                let m_ft = scale(s.m(), &frac!(1, 12));
                let hit = want
                    .iter()
                    .find(|(id, at, _)| *id == e.id() && feet.value() == &frac!(*at, 1));
                assert_eq!(hit.map(|(_, _, m)| m), Some(&m_ft), "{} M", e.id());
                printed += 1;
            }
        }
        assert_eq!(printed, want.len());
    }
}
