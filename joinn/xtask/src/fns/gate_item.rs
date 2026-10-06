//! `cargo xtask gate <phase> --item <n>`: one item, graded and checked.

use super::gate_row::gate_row;
use super::gate_table::GateTable;
use super::grade_opposed::grade_opposed;
use super::harness_fixtures::harness_fixtures;
use super::once::say;

/// Grades item `n`'s opposition as the whole gate does (legacy items are not
/// graded), runs its check, and prints its row and `<label>: item <n> ok |
/// fail`. Returns whether the check passed. Never writes `gates.lock`. An `n`
/// outside the table is refused naming the table's size.
pub(crate) fn gate_item(label: &str, table: GateTable, n: usize) -> Result<bool, String> {
    let size = match table {
        GateTable::Legacy(items) => items.len(),
        GateTable::Opposed(items) => items.len(),
    };
    if n == 0 || n > size {
        return Err(format!(
            "{label}: item {n} is not in the table; acceptance is an item from 1 to {size} ({size} items)"
        ));
    }
    let (name, ok) = match table {
        GateTable::Legacy(items) => {
            let item = &items[n - 1];
            (item.name, (item.check)())
        }
        GateTable::Opposed(items) => {
            harness_fixtures()?;
            let item = &items[n - 1];
            grade_opposed(n, item)?;
            (item.name, (item.check)())
        }
    };
    say(&gate_row(n, name, ok));
    let word = if ok { "ok" } else { "fail" };
    say(&format!("{label}: item {n} {word}"));
    Ok(ok)
}

#[cfg(test)]
mod tests {
    use std::sync::OnceLock;

    use joinn_gate::GateItem;

    use super::gate_item;
    use crate::fns::gate_table::GateTable;
    use crate::fns::mutate::Mutation;
    use crate::fns::once::memo;
    use crate::fns::run_gate_table;
    use crate::fns::subject::Subject;

    fn passes() -> bool {
        true
    }

    fn fails() -> bool {
        false
    }

    fn linkless(subject: &Subject) -> bool {
        matches!(subject, Subject::Universe(u) if u.coding.links.is_empty())
    }

    const ITEMS: &[GateItem<Subject, Mutation>] = &[
        GateItem {
            name: "first",
            check: passes,
            control: linkless,
            control_artifact: "corpus/phase5/universe.universe",
            opposes: Mutation::DropLink("e0"),
        },
        GateItem {
            name: "second",
            check: fails,
            control: linkless,
            control_artifact: "corpus/phase5/universe.universe",
            opposes: Mutation::DropLink("e0"),
        },
    ];

    type Scored = (Result<(u32, u32), String>, Vec<String>);
    type Itemed = (Result<bool, String>, Vec<String>);

    #[test]
    fn an_item_prints_the_row_the_whole_gate_prints() {
        static WHOLE: OnceLock<Scored> = OnceLock::new();
        static FIRST: OnceLock<Itemed> = OnceLock::new();
        static SECOND: OnceLock<Itemed> = OnceLock::new();
        let whole = memo(&WHOLE, || run_gate_table("phase test", ITEMS));
        assert_eq!(whole, Ok((1, 2)));
        let rows = WHOLE.get().map(|(_, l)| l.clone()).unwrap_or_default();
        let table = GateTable::Opposed(ITEMS);
        for (n, cell, verdict) in [(1, &FIRST, true), (2, &SECOND, false)] {
            assert_eq!(
                memo(cell, || gate_item("phase test", table, n)),
                Ok(verdict)
            );
            let lines = cell.get().map(|(_, l)| l.clone()).unwrap_or_default();
            let word = if verdict { "ok" } else { "fail" };
            assert_eq!(
                lines,
                [rows[n - 1].clone(), format!("phase test: item {n} {word}")]
            );
        }
    }

    #[test]
    fn an_item_outside_the_table_is_refused_naming_its_size() {
        let refused = gate_item("phase test", GateTable::Opposed(ITEMS), 3);
        assert_eq!(
            refused,
            Err(
                "phase test: item 3 is not in the table; acceptance is an item from 1 to 2 (2 items)"
                    .to_owned()
            )
        );
    }
}
