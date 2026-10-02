use super::Plants;
use crate::beam::Example;
use crate::bridge::Edition;
use crate::plants::{line, mixed_order, moment_plus_work, no_bridge, rectangle, wrong_witness};

/// The five plants, in order, each of which must be refused (ok).
pub fn plant_block(examples: &[Example], edition: &Edition) -> Plants {
    let find = |id: &str| examples.iter().find(|e| e.id() == id);
    let (Some(e1), Some(e2)) = (find("E1"), find("E2")) else {
        return Plants {
            text: "plants: not run: E1 and E2 are needed\n".to_string(),
            refused: 0,
            admitted: 5,
        };
    };
    let lines = [
        line("mixed order", mixed_order(e1)),
        line("rectangle rule", rectangle(e1)),
        line("witness wL²/12", wrong_witness(e1)),
        line("moment + work", moment_plus_work(e1, e2, edition)),
        line("no bridge", no_bridge(e1)),
    ];
    let mut plants = Plants {
        text: String::new(),
        refused: 0,
        admitted: 0,
    };
    for (text, ok) in lines {
        plants.text.push_str(&text);
        plants.text.push('\n');
        if ok {
            plants.refused += 1;
        } else {
            plants.admitted += 1;
        }
    }
    plants
}
