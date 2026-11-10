//! The worksheet is the log, replayed up to a moment. Nothing is erased.
//! Restore appends a snapshot of an earlier moment.

use serde::{Deserialize, Serialize};

use crate::fraction::Fraction;
use crate::sheet::{Arrangement, CellId, Inside, Refusal, Snapshot, Worksheet};

/// An edit changes the worksheet's shape. A value changes a number or a choice.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Edit,
    Value,
}

/// One recorded change. The `do` field names the change.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(tag = "do", rename_all = "snake_case")]
pub enum Op {
    Add {
        id: u64,
        parent: Option<u64>,
        name: String,
        inside: Inside,
        x: i64,
        y: i64,
        show_on_face: bool,
    },
    Remove {
        id: u64,
    },
    Move {
        id: u64,
        parent: u64,
        index: usize,
        x: i64,
        y: i64,
    },
    Rename {
        id: u64,
        name: String,
    },
    SetFormula {
        id: u64,
        formula: String,
    },
    SetUnit {
        id: u64,
        unit: String,
    },
    SetText {
        id: u64,
        text: String,
    },
    SetArrangement {
        id: u64,
        arrangement: Arrangement,
    },
    SetShowOnFace {
        id: u64,
        show: bool,
    },
    SetPosition {
        id: u64,
        x: i64,
        y: i64,
    },
    Reorder {
        id: u64,
        index: usize,
    },
    SetOptions {
        id: u64,
        options: Vec<String>,
    },
    AddLink {
        from: u64,
        to: u64,
    },
    RemoveLink {
        from: u64,
        to: u64,
    },
    SetNumber {
        id: u64,
        value: Fraction,
    },
    SetPick {
        id: u64,
        pick: usize,
    },
    Restore {
        snapshot: Snapshot,
    },
}

impl Op {
    pub fn kind(&self) -> Kind {
        match self {
            Self::SetNumber { .. } | Self::SetPick { .. } => Kind::Value,
            _ => Kind::Edit,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Entry {
    pub kind: Kind,
    #[serde(flatten)]
    pub op: Op,
}

impl Entry {
    fn from_op(op: Op) -> Self {
        Self {
            kind: op.kind(),
            op,
        }
    }
}

/// Why a log could not be written or read.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum StoreError {
    Json(String),
    Io(String),
}

impl StoreError {
    pub fn message(&self) -> String {
        match self {
            Self::Json(text) => format!("This worksheet file can't be read. {text}"),
            Self::Io(text) => format!("The worksheet file could not be saved. {text}"),
        }
    }
}

/// The ordered log. The screen is [`Document::state_at`] for some index.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Document {
    entries: Vec<Entry>,
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

impl Document {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    /// The worksheet after the first `index` entries. `0` is empty.
    pub fn state_at(&self, index: usize) -> Result<Worksheet, Refusal> {
        if index > self.entries.len() {
            return Err(Refusal::NoSuchMoment);
        }
        let mut sheet = Worksheet::empty();
        for entry in self.entries.iter().take(index) {
            sheet.apply(&entry.op)?;
        }
        Ok(sheet)
    }

    pub fn now(&self) -> Result<Worksheet, Refusal> {
        self.state_at(self.entries.len())
    }

    pub fn push(&mut self, op: Op) -> Result<(), Refusal> {
        let mut sheet = self.now()?;
        sheet.apply(&op)?;
        self.entries.push(Entry::from_op(op));
        Ok(())
    }

    /// Append a snapshot of moment `index`. The log grows; nothing is removed.
    pub fn restore(&mut self, index: usize) -> Result<(), Refusal> {
        let snapshot = self.state_at(index)?.snapshot();
        self.push(Op::Restore { snapshot })
    }

    pub fn to_json(&self) -> Result<String, StoreError> {
        serde_json::to_string_pretty(self).map_err(|err| StoreError::Json(err.to_string()))
    }

    pub fn from_json(text: &str) -> Result<Self, StoreError> {
        serde_json::from_str(text).map_err(|err| StoreError::Json(err.to_string()))
    }

    pub fn save(&self, path: &std::path::Path) -> Result<(), StoreError> {
        let json = self.to_json()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|err| StoreError::Io(err.to_string()))?;
        }
        std::fs::write(path, json).map_err(|err| StoreError::Io(err.to_string()))
    }

    pub fn load(path: &std::path::Path) -> Result<Self, StoreError> {
        let text = std::fs::read_to_string(path).map_err(|err| StoreError::Io(err.to_string()))?;
        Self::from_json(&text)
    }
}

impl Worksheet {
    pub fn apply(&mut self, op: &Op) -> Result<(), Refusal> {
        match op {
            Op::Add {
                id,
                parent,
                name,
                inside,
                x,
                y,
                show_on_face,
            } => self.insert(
                CellId(*id),
                parent.map(CellId),
                crate::sheet::NewCell {
                    name: name.clone(),
                    inside: inside.clone(),
                    x: *x,
                    y: *y,
                    show_on_face: *show_on_face,
                },
            ),
            Op::Remove { id } => self.remove(CellId(*id)),
            Op::Move {
                id,
                parent,
                index,
                x,
                y,
            } => self.move_cell(CellId(*id), CellId(*parent), *index, *x, *y),
            Op::Rename { id, name } => self.rename(CellId(*id), name.clone()),
            Op::SetFormula { id, formula } => self.set_formula(CellId(*id), formula.clone()),
            Op::SetUnit { id, unit } => self.set_unit(CellId(*id), unit.clone()),
            Op::SetText { id, text } => self.set_text(CellId(*id), text.clone()),
            Op::SetArrangement { id, arrangement } => self.set_arrangement(CellId(*id), *arrangement),
            Op::SetShowOnFace { id, show } => self.set_show_on_face(CellId(*id), *show),
            Op::SetPosition { id, x, y } => self.set_position(CellId(*id), *x, *y),
            Op::Reorder { id, index } => self.reorder(CellId(*id), *index),
            Op::SetOptions { id, options } => self.set_options(CellId(*id), options.clone()),
            Op::AddLink { from, to } => self.add_link(CellId(*from), CellId(*to)),
            Op::RemoveLink { from, to } => self.remove_link(CellId(*from), CellId(*to)),
            Op::SetNumber { id, value } => self.set_number(CellId(*id), *value),
            Op::SetPick { id, pick } => self.set_pick(CellId(*id), *pick),
            Op::Restore { snapshot } => {
                *self = Worksheet::from_snapshot(snapshot.clone());
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn group(arrangement: Arrangement) -> Inside {
        Inside::Group {
            arrangement,
            children: Vec::new(),
        }
    }

    fn push_both(
        doc: &mut Document,
        live: &mut Worksheet,
        recorded: &mut Vec<Worksheet>,
        op: Op,
    ) {
        live.apply(&op).expect("live apply");
        doc.push(op).expect("log push");
        recorded.push(live.clone());
    }

    #[test]
    fn replay_every_index_restore_and_save() {
        let mut doc = Document::new();
        let mut live = Worksheet::empty();
        let mut recorded = vec![live.clone()];

        let root = Inside::Group {
            arrangement: Arrangement::Free,
            children: Vec::new(),
        };
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::Add {
                id: 1,
                parent: None,
                name: "Worksheet".into(),
                inside: root,
                x: 0,
                y: 0,
                show_on_face: false,
            },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::Add {
                id: 2,
                parent: Some(1),
                name: "L".into(),
                inside: Inside::Number {
                    value: Fraction::from_i64(24),
                    unit: String::new(),
                },
                x: 0,
                y: 0,
                show_on_face: false,
            },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::Add {
                id: 3,
                parent: Some(1),
                name: "w".into(),
                inside: Inside::Number {
                    value: Fraction::from_i64(1),
                    unit: String::new(),
                },
                x: 0,
                y: 40,
                show_on_face: false,
            },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::Add {
                id: 4,
                parent: Some(1),
                name: "Title".into(),
                inside: Inside::Text {
                    text: "beam".into(),
                },
                x: 0,
                y: 80,
                show_on_face: false,
            },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::Add {
                id: 5,
                parent: Some(1),
                name: "Steel".into(),
                inside: Inside::Choice {
                    options: vec!["A992".into(), "A36".into()],
                    pick: 0,
                },
                x: 0,
                y: 120,
                show_on_face: false,
            },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::Add {
                id: 6,
                parent: Some(1),
                name: "R".into(),
                inside: Inside::Result {
                    formula: "L".into(),
                    unit: "kip".into(),
                },
                x: 0,
                y: 160,
                show_on_face: true,
            },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::Add {
                id: 7,
                parent: Some(1),
                name: "Inputs".into(),
                inside: group(Arrangement::Stack),
                x: 0,
                y: 200,
                show_on_face: false,
            },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::Move {
                id: 2,
                parent: 7,
                index: 0,
                x: 0,
                y: 0,
            },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::Move {
                id: 3,
                parent: 7,
                index: 1,
                x: 0,
                y: 1,
            },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::Rename {
                id: 4,
                name: "Heading".into(),
            },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::SetNumber {
                id: 3,
                value: Fraction::from_i64(2),
            },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::SetUnit {
                id: 2,
                unit: "ft".into(),
            },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::SetText {
                id: 4,
                text: "Simple beam, uniform load".into(),
            },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::SetFormula {
                id: 6,
                formula: "w * L / 2".into(),
            },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::SetArrangement {
                id: 7,
                arrangement: Arrangement::Free,
            },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::SetShowOnFace { id: 6, show: true },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::SetPick { id: 5, pick: 1 },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::AddLink { from: 2, to: 6 },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::SetPosition {
                id: 3,
                x: 10,
                y: 20,
            },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::Reorder { id: 3, index: 0 },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::Add {
                id: 8,
                parent: Some(1),
                name: "extra".into(),
                inside: Inside::Number {
                    value: Fraction::from_i64(0),
                    unit: String::new(),
                },
                x: 1,
                y: 1,
                show_on_face: false,
            },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::Remove { id: 8 },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::SetOptions {
                id: 5,
                options: vec!["A992".into(), "A36".into(), "A500".into()],
            },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::Add {
                id: 9,
                parent: Some(7),
                name: "Inner".into(),
                inside: group(Arrangement::Stack),
                x: 4,
                y: 4,
                show_on_face: false,
            },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::Add {
                id: 10,
                parent: Some(9),
                name: "temp".into(),
                inside: Inside::Number {
                    value: Fraction::from_i64(1),
                    unit: String::new(),
                },
                x: 0,
                y: 0,
                show_on_face: false,
            },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::Rename {
                id: 10,
                name: "n".into(),
            },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::SetNumber {
                id: 10,
                value: Fraction::from_i64(5),
            },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::Add {
                id: 11,
                parent: Some(1),
                name: "Check".into(),
                inside: Inside::Result {
                    formula: "1".into(),
                    unit: String::new(),
                },
                x: 0,
                y: 0,
                show_on_face: false,
            },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::RemoveLink { from: 2, to: 6 },
        );
        push_both(
            &mut doc,
            &mut live,
            &mut recorded,
            Op::SetShowOnFace { id: 11, show: true },
        );

        assert_eq!(doc.len(), 30, "the script is a 30-entry log");
        assert_eq!(recorded.len(), 31);
        for index in 0..=30 {
            assert_eq!(
                doc.state_at(index).expect("replay"),
                recorded[index],
                "replay at {index}"
            );
        }
        assert!(matches!(doc.entries()[10].kind, Kind::Value));
        assert!(matches!(doc.entries()[0].kind, Kind::Edit));

        let refused = doc.push(Op::Move {
            id: 7,
            parent: 9,
            index: 0,
            x: 0,
            y: 0,
        });
        assert_eq!(refused, Err(Refusal::IntoItself));
        assert_eq!(doc.len(), 30, "a refusal is not recorded");

        let json = doc.to_json().expect("json");
        let loaded = Document::from_json(&json).expect("parse");
        assert_eq!(loaded.entries(), doc.entries());
        let path = std::env::temp_dir().join(format!("joinn-s9-log-{}.json", std::process::id()));
        doc.save(&path).expect("save");
        let from_disk = Document::load(&path).expect("load");
        assert_eq!(from_disk.entries(), doc.entries());
        let _ = std::fs::remove_file(&path);

        let moment = 10;
        let at_moment = doc.state_at(moment).expect("moment");
        let len_before = doc.len();
        doc.restore(moment).expect("restore");
        assert!(doc.len() > len_before, "restore appends");
        assert_eq!(doc.now().expect("now"), at_moment);
        assert_eq!(doc.state_at(moment).expect("prefix"), at_moment);

        let round = Document::from_json(&doc.to_json().expect("json")).expect("parse");
        assert_eq!(round.entries(), doc.entries());
    }
}
