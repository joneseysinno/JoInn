use num_rational::BigRational;
use num_traits::Zero;

use super::{Balance, Complex, Order, Station};
use crate::q::{Q, add, frac, scale, total, wedge};
use crate::tag::Tag;
use crate::verdict::{Verdict, admit};

/// Adds up along the span, left to right. Before the first point V = 0 and
/// M = 0, or −C_A for a cantilever. At each point, first step across the line
/// from the previous point:
///
/// ```text
/// M ← M + wedge(ℓ, V) + wedge(ℓ/2, total(q, ℓ))
/// V ← V + total(q, ℓ)
/// ```
///
/// then add the point's reaction and point loads to V.
pub fn walk(complex: &Complex, balance: &Balance, order: Order) -> Verdict<Vec<Station>> {
    let mut v = Q::new(BigRational::zero(), Tag::FORCE);
    let mut m = match &balance.couple {
        Some(couple) => scale(couple, &frac!(-1, 1)),
        None => Q::new(BigRational::zero(), Tag::MOMENT),
    };
    let last = complex.points.len().saturating_sub(1);
    let mut stations = Vec::with_capacity(complex.points.len());
    for (i, point) in complex.points.iter().enumerate() {
        if let Some(line) = i.checked_sub(1).and_then(|j| complex.lines.get(j)) {
            let load = admit!(total(&line.q, &line.length));
            let lever = scale(&line.length, &frac!(1, 2));
            let load_moment = match order {
                Order::Faithful => admit!(wedge(&lever, &load)),
                Order::Mixed => admit!(wedge(&load, &lever)),
                Order::Rectangle => Q::new(BigRational::zero(), Tag::MOMENT),
            };
            m = admit!(add(&m, &admit!(wedge(&line.length, &v))));
            m = admit!(add(&m, &load_moment));
            v = admit!(add(&v, &load));
        }
        let m_here = m.clone();
        for force in &point.loads {
            v = admit!(add(&v, force));
        }
        if i == 0 {
            v = admit!(add(&v, &balance.r_a));
        }
        if i == last
            && let Some(r_b) = &balance.r_b
        {
            v = admit!(add(&v, r_b));
        }
        stations.push(Station {
            x: point.x.clone(),
            m: m_here,
            v: v.clone(),
        });
    }
    Verdict::Admitted(stations)
}
