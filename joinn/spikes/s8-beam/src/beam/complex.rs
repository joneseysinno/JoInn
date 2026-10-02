use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, Zero};

use super::{Complex, Example, Line, Point};
use crate::q::{Q, add, frac, scale};
use crate::tag::Tag;
use crate::verdict::{Verdict, admit, refuse};

/// Points at both ends, at each point load and at each printed section,
/// sorted by x, with one line between each pair of neighbors. `refined`
/// adds a point at every whole foot.
pub fn complex(example: &Example, refined: bool) -> Verdict<Complex> {
    let span = &example.span;
    let mut xs: Vec<Q> = vec![Q::new(BigRational::zero(), Tag::PLACE_X), span.clone()];
    xs.extend(example.point_loads.iter().map(|p| p.at.clone()));
    xs.extend(example.moment_sections.iter().cloned());
    xs.extend(example.deflection_sections.iter().cloned());
    if refined {
        let mut foot = BigInt::from(1);
        loop {
            let x = scale(
                &Q::new(BigRational::from_integer(foot.clone()), Tag::PLACE_X),
                &frac!(12, 1),
            );
            if x.value() >= span.value() {
                break;
            }
            xs.push(x);
            foot += 1;
        }
    }
    for x in &xs {
        if x.tag() != Tag::PLACE_X || x.value().is_negative() || x.value() > span.value() {
            return refuse(format!(
                "complex: {}: a point at {} ({}) is off the span; acceptance is a place on the span, {} from 0 to L",
                example.id,
                x.value(),
                x.tag(),
                Tag::PLACE_X
            ));
        }
    }
    xs.sort_by(|a, b| a.value().cmp(b.value()));
    xs.dedup();
    let points: Vec<Point> = xs
        .iter()
        .map(|x| Point {
            x: x.clone(),
            loads: example
                .point_loads
                .iter()
                .filter(|p| &p.at == x)
                .map(|p| p.force.clone())
                .collect(),
        })
        .collect();
    let q = example
        .uniform
        .clone()
        .unwrap_or_else(|| Q::new(BigRational::zero(), Tag::DENSITY));
    let mut lines = Vec::with_capacity(xs.len().saturating_sub(1));
    for pair in xs.windows(2) {
        let length = admit!(add(&pair[1], &scale(&pair[0], &frac!(-1, 1))));
        lines.push(Line {
            length,
            q: q.clone(),
        });
    }
    Verdict::Admitted(Complex { points, lines })
}

#[cfg(test)]
mod tests {
    use super::complex;
    use crate::beam::examples;
    use crate::q::{Q, frac};
    use crate::tag::Tag;
    use crate::verdict::admitted;

    #[test]
    fn each_example_has_its_points_and_lines() {
        let counts: Vec<(usize, usize)> = examples()
            .iter()
            .map(|e| {
                let c = admitted(complex(e, false));
                (c.points().len(), c.lines().len())
            })
            .collect();
        assert_eq!(counts, [(3, 2), (3, 2), (3, 2), (2, 1), (4, 3)]);
    }

    #[test]
    fn refined_adds_a_point_at_every_whole_foot() {
        let counts: Vec<usize> = examples()
            .iter()
            .map(|e| admitted(complex(e, true)).lines().len())
            .collect();
        assert_eq!(counts, [24, 24, 24, 10, 24]);
        let e5 = admitted(complex(&examples()[4], false));
        let xs: Vec<&Q> = e5.points().iter().map(|p| p.x()).collect();
        let want: Vec<Q> = [0, 72, 144, 288]
            .iter()
            .map(|&n| Q::new(frac!(n, 1), Tag::PLACE_X))
            .collect();
        assert_eq!(xs, want.iter().collect::<Vec<_>>());
        assert_eq!(e5.lines()[1].length(), &Q::new(frac!(72, 1), Tag::PLACE_X));
    }
}
