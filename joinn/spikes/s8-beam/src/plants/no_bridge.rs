use crate::beam::{Deflected, Example, Order, deflect, derive, station_at};
use crate::bridge::Bridge;
use crate::q::{frac, scale};
use crate::show::fraction;
use crate::verdict::{Verdict, admit, refuse};

/// E1 run with the edition absent: balance and M must succeed reading no
/// bridge, and v must be refused.
pub fn no_bridge(e1: &Example) -> Verdict<Vec<Deflected>> {
    let mut bridge = Bridge::new(None);
    let derived = admit!(derive(e1, false, Order::Faithful, &mut bridge));
    for x in e1.moment_sections() {
        admit!(station_at(derived.stations(), x));
    }
    let reads = match derived.reads_before_bridge() {
        0 => "no bridge".to_string(),
        n => format!("{n} bridge read(s)"),
    };
    let what = match e1.deflection_sections().first() {
        Some(x) => format!("v({} ft)", fraction(scale(x, &frac!(1, 12)).value())),
        None => "v".to_string(),
    };
    match deflect(&derived, e1.support(), &mut bridge, &what) {
        Verdict::Refused(refusal) => {
            refuse(format!("{}. balance and M read {reads}", refusal.reason()))
        }
        admitted => admitted,
    }
}

#[cfg(test)]
mod tests {
    use super::no_bridge;
    use crate::beam::examples;
    use crate::plants::line;

    #[test]
    fn the_no_bridge_plant_is_refused_with_the_plan_line() {
        let (text, ok) = line("no bridge", no_bridge(&examples()[0]));
        assert_eq!(
            text,
            "plant no bridge: refused (ok): v(12 ft) needs the bridge E·I; acceptance is a pinned edition. balance and M read no bridge"
        );
        assert!(ok);
    }
}
