//! One kind of thing, the cell, and the tree it nests into.
//!
//! Every cell has four parts: a name, an inside, a face, and one port (itself).

use std::collections::BTreeMap;

use crate::fraction::Fraction;

/// Identity of a cell. Stable for the life of the worksheet.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CellId(pub u64);

impl CellId {
    pub fn raw(self) -> u64 {
        self.0
    }
}

/// How a group's children are placed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Arrangement {
    /// Top to bottom, like a calculation sheet. The default for a new group.
    Stack,
    /// Children sit where they are dropped. A new worksheet's root is free.
    Free,
}

/// A child, as its parent holds it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Placed {
    pub id: CellId,
    pub x: i64,
    pub y: i64,
    pub show_on_face: bool,
}

/// What a cell holds. A group holds other cells; every other kind holds data.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Inside {
    Number { value: Fraction, unit: String },
    Text { text: String },
    Choice { options: Vec<String>, pick: usize },
    Result { formula: String, unit: String },
    Group { arrangement: Arrangement, children: Vec<Placed> },
}

impl Inside {
    pub fn is_group(&self) -> bool {
        matches!(self, Self::Group { .. })
    }

    pub fn kind_name(&self) -> &'static str {
        match self {
            Self::Number { .. } => "Number",
            Self::Text { .. } => "Text",
            Self::Choice { .. } => "Choice",
            Self::Result { .. } => "Result",
            Self::Group { .. } => "Group",
        }
    }
}

/// A cell. The face and the single port are derived, not stored.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Cell {
    id: CellId,
    name: String,
    parent: Option<CellId>,
    inside: Inside,
}

impl Cell {
    pub fn id(&self) -> CellId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn parent(&self) -> Option<CellId> {
        self.parent
    }

    pub fn inside(&self) -> &Inside {
        &self.inside
    }

    /// Sitting 1: a cell has one port, and that port is the cell itself.
    pub fn port(&self) -> CellId {
        self.id
    }
}

/// What to put down when a cell is added.
#[derive(Clone, Debug)]
pub struct NewCell {
    pub name: String,
    pub inside: Inside,
    pub x: i64,
    pub y: i64,
    pub show_on_face: bool,
}

/// A change the worksheet will not make. The screen shows [`Refusal::message`].
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Refusal {
    NoSuchCell,
    NotAGroup,
    NameTaken { name: String },
    BlankName,
    IntoItself,
    NotThatKind,
    BadPick,
}

impl Refusal {
    pub fn message(&self) -> String {
        match self {
            Self::NoSuchCell => "That cell is not on the worksheet.".to_string(),
            Self::NotAGroup => "Cells can only sit inside a group.".to_string(),
            Self::NameTaken { name } => {
                format!("There is already a cell named \"{name}\" here.")
            }
            Self::BlankName => "A cell needs a name.".to_string(),
            Self::IntoItself => "A group can't be put inside itself.".to_string(),
            Self::NotThatKind => "That cell doesn't hold this.".to_string(),
            Self::BadPick => "That isn't one of the choices.".to_string(),
        }
    }
}

/// The worksheet: a tree of cells. Empty until the first cell, which is the root.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Worksheet {
    next_id: u64,
    cells: BTreeMap<CellId, Cell>,
    root: Option<CellId>,
}

impl Default for Worksheet {
    fn default() -> Self {
        Self::empty()
    }
}

impl Worksheet {
    pub fn empty() -> Self {
        Self {
            next_id: 1,
            cells: BTreeMap::new(),
            root: None,
        }
    }

    pub fn root(&self) -> Option<CellId> {
        self.root
    }

    pub fn cell(&self, id: CellId) -> Option<&Cell> {
        self.cells.get(&id)
    }

    pub fn cells(&self) -> impl Iterator<Item = &Cell> {
        self.cells.values()
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    pub fn children_of(&self, id: CellId) -> &[Placed] {
        match self.cells.get(&id).map(|cell| &cell.inside) {
            Some(Inside::Group { children, .. }) => children,
            _ => &[],
        }
    }

    pub fn parent_of(&self, id: CellId) -> Option<CellId> {
        self.cells.get(&id).and_then(|cell| cell.parent)
    }

    /// Root first, then each ancestor down to `id`.
    pub fn path_to(&self, id: CellId) -> Vec<CellId> {
        let mut chain = Vec::new();
        let mut cursor = Some(id);
        while let Some(current) = cursor {
            chain.push(current);
            cursor = self.parent_of(current);
        }
        chain.reverse();
        chain
    }

    pub fn is_within(&self, id: CellId, ancestor: CellId) -> bool {
        if id == ancestor {
            return true;
        }
        let mut cursor = self.parent_of(id);
        while let Some(current) = cursor {
            if current == ancestor {
                return true;
            }
            cursor = self.parent_of(current);
        }
        false
    }

    pub fn cells_named(&self, name: &str) -> Vec<CellId> {
        self.cells
            .values()
            .filter(|cell| cell.name == name)
            .map(|cell| cell.id)
            .collect()
    }

    /// The face a parent shows when this cell is too small to open.
    /// `result_face`, when set, replaces a Result's own formula.
    pub fn face(&self, id: CellId, result_face: &dyn Fn(CellId) -> Option<String>) -> String {
        let Some(cell) = self.cells.get(&id) else {
            return String::new();
        };
        match &cell.inside {
            Inside::Number { value, unit } => join_value(&value.decimal(), unit),
            Inside::Text { text } => text.lines().next().unwrap_or("").to_string(),
            Inside::Choice { options, pick } => options
                .get(*pick)
                .cloned()
                .unwrap_or_else(|| "(no choice)".to_string()),
            Inside::Result { formula, unit } => {
                if let Some(answer) = result_face(id) {
                    answer
                } else {
                    join_value(formula, unit)
                }
            }
            Inside::Group { children, .. } => {
                let mut parts = vec![cell.name.clone()];
                for placed in children {
                    if placed.show_on_face {
                        parts.push(self.face(placed.id, result_face));
                    }
                }
                parts.join(" · ")
            }
        }
    }

    pub fn add(&mut self, parent: Option<CellId>, new: NewCell) -> Result<CellId, Refusal> {
        let id = CellId(self.next_id);
        self.insert(id, parent, new)?;
        self.next_id += 1;
        Ok(id)
    }

    /// Insert a cell that already has an id. Used when a log is replayed.
    pub fn insert(&mut self, id: CellId, parent: Option<CellId>, new: NewCell) -> Result<(), Refusal> {
        if new.name.trim().is_empty() {
            return Err(Refusal::BlankName);
        }
        if self.cells.contains_key(&id) {
            return Err(Refusal::NoSuchCell);
        }
        match parent {
            None => {
                if self.root.is_some() {
                    return Err(Refusal::NotAGroup);
                }
            }
            Some(parent_id) => {
                self.ensure_group(parent_id)?;
                if self.sibling_named(parent_id, &new.name, None) {
                    return Err(Refusal::NameTaken { name: new.name });
                }
            }
        }
        let placed = Placed {
            id,
            x: new.x,
            y: new.y,
            show_on_face: new.show_on_face,
        };
        if let Some(parent_id) = parent {
            self.push_child(parent_id, placed);
        } else {
            self.root = Some(id);
        }
        self.cells.insert(
            id,
            Cell {
                id,
                name: new.name,
                parent,
                inside: new.inside,
            },
        );
        if id.0 >= self.next_id {
            self.next_id = id.0 + 1;
        }
        Ok(())
    }

    pub fn remove(&mut self, id: CellId) -> Result<(), Refusal> {
        if !self.cells.contains_key(&id) {
            return Err(Refusal::NoSuchCell);
        }
        let mut gone = Vec::new();
        self.collect_subtree(id, &mut gone);
        if let Some(parent) = self.parent_of(id) {
            self.retain_child(parent, id);
        }
        if self.root == Some(id) {
            self.root = None;
        }
        for cell_id in gone {
            self.cells.remove(&cell_id);
        }
        Ok(())
    }

    pub fn move_cell(
        &mut self,
        id: CellId,
        new_parent: CellId,
        index: usize,
        x: i64,
        y: i64,
    ) -> Result<(), Refusal> {
        if !self.cells.contains_key(&id) || !self.cells.contains_key(&new_parent) {
            return Err(Refusal::NoSuchCell);
        }
        self.ensure_group(new_parent)?;
        if self.is_within(new_parent, id) {
            return Err(Refusal::IntoItself);
        }
        let name = self.cells.get(&id).map(|cell| cell.name.clone()).unwrap_or_default();
        if self.sibling_named(new_parent, &name, Some(id)) {
            return Err(Refusal::NameTaken { name });
        }
        let show_on_face = self
            .parent_of(id)
            .and_then(|parent| {
                self.children_of(parent)
                    .iter()
                    .find(|placed| placed.id == id)
                    .map(|placed| placed.show_on_face)
            })
            .unwrap_or(false);
        if let Some(parent) = self.parent_of(id) {
            self.retain_child(parent, id);
        }
        if self.root == Some(id) {
            self.root = None;
        }
        if let Some(cell) = self.cells.get_mut(&id) {
            cell.parent = Some(new_parent);
        }
        self.insert_child(
            new_parent,
            index,
            Placed {
                id,
                x,
                y,
                show_on_face,
            },
        );
        Ok(())
    }

    pub fn rename(&mut self, id: CellId, name: String) -> Result<(), Refusal> {
        if name.trim().is_empty() {
            return Err(Refusal::BlankName);
        }
        let Some(cell) = self.cells.get(&id) else {
            return Err(Refusal::NoSuchCell);
        };
        if let Some(parent) = cell.parent {
            if self.sibling_named(parent, &name, Some(id)) {
                return Err(Refusal::NameTaken { name });
            }
        }
        if let Some(cell) = self.cells.get_mut(&id) {
            cell.name = name;
        }
        Ok(())
    }

    pub fn set_show_on_face(&mut self, id: CellId, show: bool) -> Result<(), Refusal> {
        let Some(parent) = self.parent_of(id) else {
            return Err(Refusal::NoSuchCell);
        };
        let Some(Inside::Group { children, .. }) = self.cells.get_mut(&parent).map(|cell| &mut cell.inside) else {
            return Err(Refusal::NotAGroup);
        };
        let Some(placed) = children.iter_mut().find(|placed| placed.id == id) else {
            return Err(Refusal::NoSuchCell);
        };
        placed.show_on_face = show;
        Ok(())
    }

    pub fn set_position(&mut self, id: CellId, x: i64, y: i64) -> Result<(), Refusal> {
        let Some(parent) = self.parent_of(id) else {
            return Err(Refusal::NoSuchCell);
        };
        let Some(Inside::Group { children, .. }) = self.cells.get_mut(&parent).map(|cell| &mut cell.inside) else {
            return Err(Refusal::NotAGroup);
        };
        let Some(placed) = children.iter_mut().find(|placed| placed.id == id) else {
            return Err(Refusal::NoSuchCell);
        };
        placed.x = x;
        placed.y = y;
        Ok(())
    }

    pub fn reorder(&mut self, id: CellId, index: usize) -> Result<(), Refusal> {
        let Some(parent) = self.parent_of(id) else {
            return Err(Refusal::NoSuchCell);
        };
        let placed = {
            let Some(Inside::Group { children, .. }) = self.cells.get_mut(&parent).map(|c| &mut c.inside) else {
                return Err(Refusal::NotAGroup);
            };
            let Some(pos) = children.iter().position(|placed| placed.id == id) else {
                return Err(Refusal::NoSuchCell);
            };
            children.remove(pos)
        };
        self.insert_child(parent, index, placed);
        Ok(())
    }

    pub fn set_arrangement(&mut self, id: CellId, arrangement: Arrangement) -> Result<(), Refusal> {
        match self.cells.get_mut(&id).map(|cell| &mut cell.inside) {
            Some(Inside::Group {
                arrangement: slot, ..
            }) => {
                *slot = arrangement;
                Ok(())
            }
            Some(_) => Err(Refusal::NotAGroup),
            None => Err(Refusal::NoSuchCell),
        }
    }

    pub fn set_number(&mut self, id: CellId, value: Fraction) -> Result<(), Refusal> {
        match self.cells.get_mut(&id).map(|cell| &mut cell.inside) {
            Some(Inside::Number { value: slot, .. }) => {
                *slot = value;
                Ok(())
            }
            Some(_) => Err(Refusal::NotThatKind),
            None => Err(Refusal::NoSuchCell),
        }
    }

    pub fn set_unit(&mut self, id: CellId, unit: String) -> Result<(), Refusal> {
        match self.cells.get_mut(&id).map(|cell| &mut cell.inside) {
            Some(Inside::Number { unit: slot, .. } | Inside::Result { unit: slot, .. }) => {
                *slot = unit;
                Ok(())
            }
            Some(_) => Err(Refusal::NotThatKind),
            None => Err(Refusal::NoSuchCell),
        }
    }

    pub fn set_text(&mut self, id: CellId, text: String) -> Result<(), Refusal> {
        match self.cells.get_mut(&id).map(|cell| &mut cell.inside) {
            Some(Inside::Text { text: slot }) => {
                *slot = text;
                Ok(())
            }
            Some(_) => Err(Refusal::NotThatKind),
            None => Err(Refusal::NoSuchCell),
        }
    }

    pub fn set_formula(&mut self, id: CellId, formula: String) -> Result<(), Refusal> {
        match self.cells.get_mut(&id).map(|cell| &mut cell.inside) {
            Some(Inside::Result { formula: slot, .. }) => {
                *slot = formula;
                Ok(())
            }
            Some(_) => Err(Refusal::NotThatKind),
            None => Err(Refusal::NoSuchCell),
        }
    }

    pub fn set_options(&mut self, id: CellId, options: Vec<String>) -> Result<(), Refusal> {
        if options.is_empty() || options.iter().any(|option| option.trim().is_empty()) {
            return Err(Refusal::BadPick);
        }
        match self.cells.get_mut(&id).map(|cell| &mut cell.inside) {
            Some(Inside::Choice {
                options: slot,
                pick,
            }) => {
                *slot = options;
                if *pick >= slot.len() {
                    *pick = slot.len() - 1;
                }
                Ok(())
            }
            Some(_) => Err(Refusal::NotThatKind),
            None => Err(Refusal::NoSuchCell),
        }
    }

    pub fn set_pick(&mut self, id: CellId, pick: usize) -> Result<(), Refusal> {
        match self.cells.get_mut(&id).map(|cell| &mut cell.inside) {
            Some(Inside::Choice { options, pick: slot }) => {
                if pick >= options.len() {
                    return Err(Refusal::BadPick);
                }
                *slot = pick;
                Ok(())
            }
            Some(_) => Err(Refusal::NotThatKind),
            None => Err(Refusal::NoSuchCell),
        }
    }

    fn ensure_group(&self, id: CellId) -> Result<(), Refusal> {
        match self.cells.get(&id).map(|cell| &cell.inside) {
            Some(Inside::Group { .. }) => Ok(()),
            Some(_) => Err(Refusal::NotAGroup),
            None => Err(Refusal::NoSuchCell),
        }
    }

    fn sibling_named(&self, parent: CellId, name: &str, except: Option<CellId>) -> bool {
        self.children_of(parent).iter().any(|placed| {
            except != Some(placed.id)
                && self.cells.get(&placed.id).is_some_and(|cell| cell.name == name)
        })
    }

    fn push_child(&mut self, parent: CellId, placed: Placed) {
        if let Some(Inside::Group { children, .. }) = self.cells.get_mut(&parent).map(|c| &mut c.inside)
        {
            children.push(placed);
        }
    }

    fn insert_child(&mut self, parent: CellId, index: usize, placed: Placed) {
        if let Some(Inside::Group { children, .. }) = self.cells.get_mut(&parent).map(|c| &mut c.inside)
        {
            let index = index.min(children.len());
            children.insert(index, placed);
        }
    }

    fn retain_child(&mut self, parent: CellId, id: CellId) {
        if let Some(Inside::Group { children, .. }) = self.cells.get_mut(&parent).map(|c| &mut c.inside)
        {
            children.retain(|placed| placed.id != id);
        }
    }

    fn collect_subtree(&self, id: CellId, out: &mut Vec<CellId>) {
        out.push(id);
        for placed in self.children_of(id) {
            self.collect_subtree(placed.id, out);
        }
    }
}

fn join_value(value: &str, unit: &str) -> String {
    let unit = unit.trim();
    if unit.is_empty() {
        value.to_string()
    } else {
        format!("{value} {unit}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn group(name: &str) -> NewCell {
        NewCell {
            name: name.to_string(),
            inside: Inside::Group {
                arrangement: Arrangement::Stack,
                children: Vec::new(),
            },
            x: 0,
            y: 0,
            show_on_face: false,
        }
    }

    fn number(name: &str) -> NewCell {
        NewCell {
            name: name.to_string(),
            inside: Inside::Number {
                value: Fraction::from_i64(1),
                unit: "ft".to_string(),
            },
            x: 0,
            y: 0,
            show_on_face: false,
        }
    }

    #[test]
    fn tree_three_deep_move_between_groups_and_refuse_self_drop() {
        let mut sheet = Worksheet::empty();
        let root = sheet.add(None, group("Sheet")).expect("root");
        let outer = sheet.add(Some(root), group("Outer")).expect("outer");
        let inner = sheet.add(Some(outer), group("Inner")).expect("inner");
        let leaf = sheet.add(Some(inner), number("Leaf")).expect("leaf");

        let path = sheet.path_to(leaf);
        assert_eq!(path.len(), 4, "root, outer, inner, leaf is three steps deep");
        assert_eq!(sheet.children_of(inner).len(), 1);

        sheet
            .move_cell(leaf, outer, 1, 4, 8)
            .expect("move to the outer group");
        assert_eq!(sheet.parent_of(leaf), Some(outer));
        assert!(sheet.children_of(inner).is_empty());
        assert_eq!(sheet.children_of(outer)[1].id, leaf);
        assert_eq!(sheet.children_of(outer)[1].x, 4);

        let before = sheet.clone();
        let into_descendant = sheet.move_cell(outer, inner, 0, 0, 0);
        assert_eq!(into_descendant, Err(Refusal::IntoItself));
        assert_eq!(sheet, before, "a refused drop changes nothing");

        let into_self = sheet.move_cell(outer, outer, 0, 0, 0);
        assert_eq!(into_self, Err(Refusal::IntoItself));
        assert_eq!(sheet, before);

        sheet.remove(outer).expect("remove");
        assert!(sheet.cell(outer).is_none());
        assert!(sheet.cell(inner).is_none());
        assert!(sheet.cell(leaf).is_none());
        assert!(sheet.cell(root).is_some());
    }

    #[test]
    fn sibling_names_stay_unique_and_faces_follow_the_kind() {
        let mut sheet = Worksheet::empty();
        let root = sheet
            .add(
                None,
                NewCell {
                    inside: Inside::Group {
                        arrangement: Arrangement::Free,
                        children: Vec::new(),
                    },
                    ..group("Worksheet")
                },
            )
            .expect("root");
        let span = sheet.add(Some(root), number("L")).expect("L");
        let taken = sheet.add(Some(root), number("L"));
        assert!(matches!(taken, Err(Refusal::NameTaken { .. })));
        sheet.rename(span, "Span".to_string()).expect("rename");
        assert_eq!(sheet.cell(span).expect("span").name(), "Span");
        assert_eq!(sheet.face(span, &|_| None), "1 ft");
        assert_eq!(sheet.cell(span).expect("span").port(), span);
    }
}
