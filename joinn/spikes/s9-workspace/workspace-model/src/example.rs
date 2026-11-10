//! The Simple beam worksheet, as a log, so it can be scrubbed from empty to finished.

use crate::fraction::Fraction;
use crate::log::{Document, Op};
use crate::sheet::{Arrangement, Inside};

/// A 24 ft span under 1.2 kip/ft. Reactions 14.4 kip, moment 86.4 kip·ft, check passes.
pub fn simple_beam() -> Document {
    let mut doc = Document::new();
    add(
        &mut doc,
        1,
        None,
        "Simple beam",
        group(Arrangement::Stack),
        false,
    );
    add(
        &mut doc,
        2,
        Some(1),
        "Title",
        Inside::Text {
            text: "Simple beam, uniform load".to_string(),
        },
        false,
    );
    add(&mut doc, 3, Some(1), "Inputs", group(Arrangement::Stack), false);
    add(
        &mut doc,
        4,
        Some(3),
        "L",
        number("24", "ft"),
        false,
    );
    add(
        &mut doc,
        5,
        Some(3),
        "w",
        number("1.2", "kip/ft"),
        false,
    );
    add(
        &mut doc,
        6,
        Some(3),
        "Ma",
        number("92.8", "kip·ft"),
        false,
    );
    add(
        &mut doc,
        7,
        Some(3),
        "Steel",
        Inside::Choice {
            options: vec!["A992".to_string(), "A36".to_string()],
            pick: 0,
        },
        false,
    );
    add(&mut doc, 8, Some(1), "Results", group(Arrangement::Stack), false);
    add(
        &mut doc,
        9,
        Some(8),
        "R",
        result("w * L / 2", "kip"),
        true,
    );
    add(
        &mut doc,
        10,
        Some(8),
        "M",
        result("w * L^2 / 8", "kip·ft"),
        true,
    );
    add(
        &mut doc,
        11,
        Some(8),
        "Check",
        result("M <= Ma", ""),
        true,
    );
    doc
}

fn add(
    doc: &mut Document,
    id: u64,
    parent: Option<u64>,
    name: &str,
    inside: Inside,
    show_on_face: bool,
) {
    if doc
        .push(Op::Add {
            id,
            parent,
            name: name.to_string(),
            inside,
            x: 0,
            y: 0,
            show_on_face,
        })
        .is_err()
    {
        // The example is fixed data. If a step is refused the beam test fails closed.
    }
}

fn group(arrangement: Arrangement) -> Inside {
    Inside::Group {
        arrangement,
        children: Vec::new(),
    }
}

fn number(text: &str, unit: &str) -> Inside {
    Inside::Number {
        value: match Fraction::parse_decimal(text) {
            Some(value) => value,
            None => Fraction::from_i64(0),
        },
        unit: unit.to_string(),
    }
}

fn result(formula: &str, unit: &str) -> Inside {
    Inside::Result {
        formula: formula.to_string(),
        unit: unit.to_string(),
    }
}
