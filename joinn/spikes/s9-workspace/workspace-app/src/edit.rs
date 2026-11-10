//! Edits the log. The screen calls these; it does not keep a second copy of the tree.

use workspace_model::{Arrangement, CellId, Document, Fraction, Inside, Refusal, Worksheet};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BaseKind {
    Number,
    Text,
    Choice,
    Result,
    Group,
}

impl BaseKind {
    pub fn from_token(token: &str) -> Option<Self> {
        match token {
            "number" => Some(Self::Number),
            "text" => Some(Self::Text),
            "choice" => Some(Self::Choice),
            "result" => Some(Self::Result),
            "group" => Some(Self::Group),
            _ => None,
        }
    }

    pub fn token(self) -> &'static str {
        match self {
            Self::Number => "number",
            Self::Text => "text",
            Self::Choice => "choice",
            Self::Result => "result",
            Self::Group => "group",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Number => "Number",
            Self::Text => "Text",
            Self::Choice => "Choice",
            Self::Result => "Result",
            Self::Group => "Group",
        }
    }

    pub fn blurb(self) -> &'static str {
        match self {
            Self::Number => "A value",
            Self::Text => "Words",
            Self::Choice => "A pick",
            Self::Result => "A formula",
            Self::Group => "Other cells",
        }
    }

    fn inside(self) -> Inside {
        match self {
            Self::Number => Inside::Number {
                value: Fraction::from_i64(0),
                unit: String::new(),
            },
            Self::Text => Inside::Text {
                text: String::new(),
            },
            Self::Choice => Inside::Choice {
                options: vec!["Option 1".to_string(), "Option 2".to_string()],
                pick: 0,
            },
            Self::Result => Inside::Result {
                formula: String::new(),
                unit: String::new(),
            },
            Self::Group => Inside::Group {
                arrangement: Arrangement::Stack,
                children: Vec::new(),
            },
        }
    }

    fn show_on_face(self) -> bool {
        matches!(self, Self::Result)
    }
}

pub fn fresh_worksheet() -> Document {
    let mut doc = Document::new();
    let _ = doc.push(workspace_model::Op::Add {
        id: 1,
        parent: None,
        name: "Worksheet".to_string(),
        inside: Inside::Group {
            arrangement: Arrangement::Free,
            children: Vec::new(),
        },
        x: 0,
        y: 0,
        show_on_face: false,
    });
    doc
}

pub fn add_cell(
    doc: &mut Document,
    parent: Option<CellId>,
    kind: BaseKind,
    x: i64,
    y: i64,
) -> Result<CellId, Refusal> {
    let parent = resolve_parent(doc, parent, kind)?;
    let sheet = doc.now()?;
    let name = match parent {
        Some(parent) => unique_name(&sheet, parent, kind.label()),
        None => kind.label().to_string(),
    };
    let id = sheet.next_id();
    doc.push(workspace_model::Op::Add {
        id,
        parent: parent.map(CellId::raw),
        name,
        inside: kind.inside(),
        x,
        y,
        show_on_face: kind.show_on_face(),
    })?;
    Ok(CellId(id))
}

fn resolve_parent(
    doc: &mut Document,
    requested: Option<CellId>,
    kind: BaseKind,
) -> Result<Option<CellId>, Refusal> {
    let sheet = doc.now()?;
    if sheet.root().is_none() {
        if requested.is_none() && matches!(kind, BaseKind::Group) {
            return Ok(None);
        }
        return Ok(Some(ensure_root(doc)?));
    }
    let sheet = doc.now()?;
    match requested.or_else(|| sheet.root()) {
        Some(id) if sheet.cell(id).is_some_and(|cell| cell.inside().is_group()) => Ok(Some(id)),
        Some(id) => Ok(sheet.parent_of(id).or_else(|| sheet.root())),
        None => Ok(sheet.root()),
    }
}

fn ensure_root(doc: &mut Document) -> Result<CellId, Refusal> {
    if let Some(root) = doc.now()?.root() {
        return Ok(root);
    }
    let id = doc.now()?.next_id();
    doc.push(workspace_model::Op::Add {
        id,
        parent: None,
        name: "Worksheet".to_string(),
        inside: Inside::Group {
            arrangement: Arrangement::Free,
            children: Vec::new(),
        },
        x: 0,
        y: 0,
        show_on_face: false,
    })?;
    Ok(CellId(id))
}

pub fn unique_name(sheet: &Worksheet, parent: CellId, base: &str) -> String {
    let taken = |name: &str| {
        sheet.children_of(parent).iter().any(|placed| {
            sheet
                .cell(placed.id)
                .is_some_and(|cell| cell.name() == name)
        })
    };
    if !taken(base) {
        return base.to_string();
    }
    let mut n = 2u32;
    loop {
        let candidate = format!("{base} {n}");
        if !taken(&candidate) || n > 400 {
            return candidate;
        }
        n += 1;
    }
}
