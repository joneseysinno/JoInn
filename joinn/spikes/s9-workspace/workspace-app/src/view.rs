//! The window. Sitting 1 draws the model and sends it edits.

use dioxus::prelude::*;
use workspace_model::{
    evaluate, Answer, Arrangement, CellId, Document, Fraction, Inside, Op, Worksheet,
};

use crate::edit::{add_cell, fresh_worksheet, BaseKind};
use crate::layout::{self, drop_group, find, stack_index, Band, Laid};

const STYLE: &str = include_str!("style.css");

pub fn launch() {
    let window = dioxus::desktop::WindowBuilder::new()
        .with_title("JoInn")
        .with_inner_size(dioxus::desktop::LogicalSize::new(1360.0, 860.0))
        .with_min_inner_size(dioxus::desktop::LogicalSize::new(960.0, 640.0))
        .with_always_on_top(false);
    let config = dioxus::desktop::Config::new()
        .with_window(window)
        .with_disable_drag_drop_handler(true)
        .with_menu(None);
    dioxus::LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(App);
}

#[derive(Clone, Copy)]
struct PointerDrag {
    id: CellId,
    origin_x: f64,
    origin_y: f64,
    last_x: f64,
    last_y: f64,
    moved: bool,
}

#[derive(Clone)]
struct Draft {
    name: String,
    value: String,
    unit: String,
    text: String,
    formula: String,
    options: String,
}

impl Default for Draft {
    fn default() -> Self {
        Self {
            name: String::new(),
            value: String::new(),
            unit: String::new(),
            text: String::new(),
            formula: String::new(),
            options: String::new(),
        }
    }
}

#[allow(non_snake_case)]
fn App() -> Element {
    let mut doc = use_signal(fresh_worksheet);
    let mut selected = use_signal(|| None::<CellId>);
    let mut notice = use_signal(|| None::<String>);
    let mut hover = use_signal(|| None::<CellId>);
    let mut drag = use_signal(|| None::<PointerDrag>);
    let mut move_armed = use_signal(|| false);
    let mut renaming = use_signal(|| None::<CellId>);
    let mut typing = use_signal(|| false);
    let mut draft = use_signal(Draft::default);
    let mut canvas_at = use_signal(|| (0.0_f64, 0.0));

    let sheet = match doc.read().now() {
        Ok(sheet) => sheet,
        Err(refusal) => {
            return rsx! { div { class: "shell", "{refusal.message()}" } };
        }
    };
    let answers = evaluate(&sheet);
    let laid = layout::layout(&sheet, 1.0, 8);
    let root_name = sheet
        .root()
        .and_then(|id| sheet.cell(id))
        .map(|cell| cell.name().to_string())
        .unwrap_or_else(|| "Empty".to_string());

    let pan = (32.0_f64, 32.0_f64);
    let scale = 1.0_f64;

    rsx! {
        style { "{STYLE}" }
        div {
            class: "shell",
            tabindex: "0",
            autofocus: true,
            onkeydown: move |event| {
                if typing() {
                    return;
                }
                let key = event.key();
                if key == Key::Delete {
                    delete_selected(doc, selected, notice);
                } else if let Key::Character(text) = key {
                    if text.eq_ignore_ascii_case("m") {
                        if selected().is_some() {
                            move_armed.set(true);
                            notice.set(Some("Click the group it should move into.".to_string()));
                        }
                    }
                }
            },
            onpointermove: move |event| {
                let Some(mut current) = drag() else { return };
                let (x, y) = pointer_xy(&event);
                let dx = x - current.origin_x;
                let dy = y - current.origin_y;
                if dx * dx + dy * dy > 16.0 {
                    current.moved = true;
                }
                current.last_x = x;
                current.last_y = y;
                drag.set(Some(current));
                if current.moved {
                    if let Some(tree) = layout::layout(&sheet_now(&doc), 1.0, 8) {
                        let world = to_world((x, y), canvas_at(), pan, scale);
                        hover.set(drop_group(&tree, world.0, world.1, Some(current.id), &sheet_now(&doc)));
                    }
                }
            },
            onpointerup: move |event| {
                let Some(current) = drag() else { return };
                drag.set(None);
                hover.set(None);
                if !current.moved {
                    return;
                }
                finish_move(doc, notice, canvas_at(), pan, scale, &current, pointer_xy(&event));
            },
            header { class: "topbar",
                span { class: "mark", "JoInn" }
                nav { class: "crumbs",
                    button { class: "crumb here", "{root_name}" }
                }
                span { class: "worksheet-name", "{root_name}" }
                div { class: "top-actions",
                    button { onclick: move |_| { doc.set(fresh_worksheet()); selected.set(None); notice.set(None); }, "New" }
                    button { onclick: move |_| notice.set(Some("Open arrives with the time slider.".to_string())), "Open" }
                    button { onclick: move |_| notice.set(Some("The example arrives with the time slider.".to_string())), "Open example" }
                }
            }
            if let Some(text) = notice() {
                div { class: "notice", "{text}" }
            }
            aside { class: "palette",
                h2 { "Cells" }
                Swatch { kind: BaseKind::Number, doc, selected, notice, draft }
                Swatch { kind: BaseKind::Text, doc, selected, notice, draft }
                Swatch { kind: BaseKind::Choice, doc, selected, notice, draft }
                Swatch { kind: BaseKind::Result, doc, selected, notice, draft }
                Swatch { kind: BaseKind::Group, doc, selected, notice, draft }
            }
            main {
                class: "canvas",
                onmounted: move |event| {
                    let data = event.data();
                    spawn(async move {
                        if let Ok(rect) = data.get_client_rect().await {
                            let corner = rect.min();
                            canvas_at.set((corner.x, corner.y));
                        }
                    });
                },
                ondragover: move |event| {
                    event.prevent_default();
                    let world = to_world(drag_xy(&event), canvas_at(), pan, scale);
                    if let Some(tree) = layout::layout(&sheet_now(&doc), 1.0, 8) {
                        hover.set(drop_group(&tree, world.0, world.1, None, &sheet_now(&doc)));
                    }
                },
                ondrop: move |event| {
                    event.prevent_default();
                    let world = to_world(drag_xy(&event), canvas_at(), pan, scale);
                    let token = transfer_text(&event);
                    hover.set(None);
                    let Some(kind) = token.strip_prefix("palette:").and_then(BaseKind::from_token) else {
                        return;
                    };
                    let sheet = sheet_now(&doc);
                    let tree = layout::layout(&sheet, 1.0, 8);
                    let parent = tree.as_ref().and_then(|tree| drop_group(tree, world.0, world.1, None, &sheet));
                    place_new(doc, selected, notice, draft, kind, parent, &sheet, tree.as_ref(), world);
                },
                div {
                    class: "world",
                    style: "transform: translate({pan.0}px, {pan.1}px) scale({scale});",
                    if let Some(tree) = laid {
                        { draw_cell(tree, &sheet, &answers, selected(), hover(), drag(), renaming(), doc, selected, notice, draft, renaming, move_armed, drag) }
                    }
                }
            }
            { details_panel(sheet, answers, selected(), draft, doc, selected, notice, draft, typing) }
            footer { class: "timebar",
                span { class: "time-label", "Now" }
                input { r#type: "range", min: "0", max: "1", value: "1", disabled: true }
            }
        }
    }
}

#[component]
fn Swatch(
    kind: BaseKind,
    mut doc: Signal<Document>,
    mut selected: Signal<Option<CellId>>,
    mut notice: Signal<Option<String>>,
    mut draft: Signal<Draft>,
) -> Element {
    let token = kind.token();
    let label = kind.label();
    let blurb = kind.blurb();
    rsx! {
        button {
            class: "swatch {token}",
            draggable: true,
            ondragstart: move |event| {
                let _ = event.data_transfer().set_data("text/plain", &format!("palette:{token}"));
            },
            onclick: move |_| {
                let parent = selected_group(&sheet_now(&doc), selected());
                let added = add_cell(&mut doc.write(), parent, kind, 24, 48);
                match added {
                    Ok(id) => select_cell(&sheet_now(&doc), id, selected, draft),
                    Err(refusal) => notice.set(Some(refusal.message())),
                }
            },
            span { "{label}" }
            small { "{blurb}" }
        }
    }
}

fn draw_cell(
    laid: Laid,
    sheet: &Worksheet,
    answers: &std::collections::BTreeMap<CellId, Answer>,
    selected_id: Option<CellId>,
    hover_id: Option<CellId>,
    drag: Option<PointerDrag>,
    renaming_id: Option<CellId>,
    mut doc: Signal<Document>,
    mut selected: Signal<Option<CellId>>,
    mut notice: Signal<Option<String>>,
    mut draft: Signal<Draft>,
    mut renaming: Signal<Option<CellId>>,
    mut move_armed: Signal<bool>,
    mut drag_sig: Signal<Option<PointerDrag>>,
) -> Element {
    let id = laid.id;
    let Some(cell) = sheet.cell(id) else {
        return rsx! { "" };
    };
    let kind = cell.inside().kind_name().to_string();
    let kind_class = kind.to_lowercase();
    let name = cell.name().to_string();
    let face = face_of(sheet, answers, id);
    let refused = answers.get(&id).is_some_and(|answer| answer.kind.is_refused());
    let is_selected = selected_id == Some(id);
    let is_drop = hover_id == Some(id);
    let offset = drag
        .as_ref()
        .filter(|drag| drag.id == id && drag.moved)
        .map(|drag| (drag.last_x - drag.origin_x, drag.last_y - drag.origin_y));
    let transform = offset
        .map(|(dx, dy)| format!("transform: translate({dx}px, {dy}px);"))
        .unwrap_or_default();
    let class = format!(
        "cell {kind_class}{}{}{}{}",
        if is_selected { " selected" } else { "" },
        if is_drop { " drop" } else { "" },
        if refused { " refused" } else { "" },
        if laid.band == Band::Dot { " dot" } else { "" }
    );
    let style = format!(
        "left:{}px; top:{}px; width:{}px; height:{}px; {transform}",
        laid.local_x, laid.local_y, laid.w, laid.h
    );
    let children = laid.children.clone();
    let show_children = laid.open;
    let show_face = laid.band != Band::Dot && !show_children;
    let renaming_this = renaming_id == Some(id);

    rsx! {
        div {
            class: "{class}",
            style: "{style}",
            onpointerdown: move |event| {
                event.stop_propagation();
                if move_armed() {
                    let sheet = sheet_now(&doc);
                    if let (Some(moving), Some(cell)) = (selected(), sheet.cell(id)) {
                        if cell.inside().is_group() && moving != id {
                            match doc.write().push(Op::Move {
                                id: moving.raw(),
                                parent: id.raw(),
                                index: sheet.children_of(id).len(),
                                x: 24,
                                y: 48,
                            }) {
                                Ok(()) => notice.set(None),
                                Err(refusal) => notice.set(Some(refusal.message())),
                            }
                            move_armed.set(false);
                            return;
                        }
                    }
                }
                let (x, y) = pointer_xy(&event);
                drag_sig.set(Some(PointerDrag {
                    id,
                    origin_x: x,
                    origin_y: y,
                    last_x: x,
                    last_y: y,
                    moved: false,
                }));
                select_cell(&sheet_now(&doc), id, selected, draft);
            },
            if laid.band != Band::Dot {
                div { class: "header",
                    if renaming_this {
                        input {
                            class: "name-input",
                            value: "{draft().name}",
                            onfocus: move |_| typing_focus(true),
                            onblur: move |_| renaming.set(None),
                            oninput: move |event| {
                                let mut next = draft();
                                next.name = event.value();
                                draft.set(next);
                            },
                            onchange: move |event| {
                                commit_rename(doc, notice, id, event.value());
                                renaming.set(None);
                            },
                        }
                    } else {
                        span {
                            ondoubleclick: move |event| {
                                event.stop_propagation();
                                select_cell(&sheet_now(&doc), id, selected, draft);
                                renaming.set(Some(id));
                            },
                            "{name}"
                        }
                    }
                    span { class: "kind", "{kind}" }
                }
            }
            if show_face {
                div { class: "face", "{face}" }
            }
            if show_children {
                for child in children {
                    { draw_cell(child, sheet, answers, selected_id, hover_id, drag.clone(), renaming_id, doc, selected, notice, draft, renaming, move_armed, drag_sig) }
                }
            }
        }
    }
}

fn details_panel(
    sheet: Worksheet,
    answers: std::collections::BTreeMap<CellId, Answer>,
    selected_id: Option<CellId>,
    draft: Signal<Draft>,
    mut doc: Signal<Document>,
    mut selected: Signal<Option<CellId>>,
    mut notice: Signal<Option<String>>,
    mut draft_sig: Signal<Draft>,
    mut typing: Signal<bool>,
) -> Element {
    let Some(id) = selected_id else {
        return rsx! {
            aside { class: "details",
                h2 { "Details" }
                p { class: "quiet", "Select a cell." }
            }
        };
    };
    let Some(cell) = sheet.cell(id) else {
        return rsx! { aside { class: "details", h2 { "Details" } } };
    };
    let name = draft().name.clone();
    let inside = cell.inside().clone();
    let answer = answers.get(&id).cloned();
    let children: Vec<(CellId, String, bool)> = sheet
        .children_of(id)
        .iter()
        .filter_map(|placed| {
            sheet.cell(placed.id).map(|child| {
                (placed.id, child.name().to_string(), placed.show_on_face)
            })
        })
        .collect();

    rsx! {
        aside { class: "details",
            h2 { "Details" }
            label { "Name"
                input {
                    value: "{name}",
                    onfocus: move |_| typing.set(true),
                    onblur: move |_| typing.set(false),
                    oninput: move |event| {
                        let mut next = draft_sig();
                        next.name = event.value();
                        draft_sig.set(next);
                    },
                    onchange: move |event| commit_rename(doc, notice, id, event.value()),
                }
            }
            { match &inside {
                Inside::Number { value, .. } => rsx! {
                    label { "Value"
                        input {
                            value: "{draft().value}",
                            onfocus: move |_| typing.set(true),
                            onblur: move |_| typing.set(false),
                            oninput: move |event| {
                                let mut next = draft_sig();
                                next.value = event.value();
                                draft_sig.set(next);
                            },
                            onchange: move |event| commit_number(doc, notice, id, event.value()),
                        }
                    }
                    label { "Unit label"
                        input {
                            value: "{draft().unit}",
                            onfocus: move |_| typing.set(true),
                            onblur: move |_| typing.set(false),
                            oninput: move |event| {
                                let mut next = draft_sig();
                                next.unit = event.value();
                                draft_sig.set(next);
                            },
                            onchange: move |event| commit_unit(doc, notice, id, event.value()),
                        }
                    }
                    p { class: "exact", "Exact {value.exact()}" }
                },
                Inside::Text { .. } => rsx! {
                    label { "Words"
                        textarea {
                            rows: "4",
                            value: "{draft().text}",
                            onfocus: move |_| typing.set(true),
                            onblur: move |_| typing.set(false),
                            oninput: move |event| {
                                let mut next = draft_sig();
                                next.text = event.value();
                                draft_sig.set(next);
                            },
                            onchange: move |event| {
                                let _ = push(doc, notice, Op::SetText { id: id.raw(), text: event.value() });
                            },
                        }
                    }
                },
                Inside::Choice { options, pick } => rsx! {
                    label { "Pick"
                        select {
                            onchange: move |event| {
                                if let Ok(pick) = event.value().parse::<usize>() {
                                    let _ = push(doc, notice, Op::SetPick { id: id.raw(), pick });
                                }
                            },
                            for (index, option) in options.iter().cloned().enumerate() {
                                option { value: "{index}", selected: index == *pick, "{option}" }
                            }
                        }
                    }
                    label { "Options, one on each line"
                        textarea {
                            rows: "4",
                            value: "{draft().options}",
                            onfocus: move |_| typing.set(true),
                            onblur: move |_| typing.set(false),
                            oninput: move |event| {
                                let mut next = draft_sig();
                                next.options = event.value();
                                draft_sig.set(next);
                            },
                            onchange: move |event| {
                                let options: Vec<String> = event.value().lines().map(|line| line.to_string()).filter(|line| !line.trim().is_empty()).collect();
                                let _ = push(doc, notice, Op::SetOptions { id: id.raw(), options });
                            },
                        }
                    }
                },
                Inside::Result { .. } => rsx! {
                    label { "Formula"
                        input {
                            value: "{draft().formula}",
                            placeholder: "w * L^2 / 8",
                            onfocus: move |_| typing.set(true),
                            onblur: move |_| typing.set(false),
                            oninput: move |event| {
                                let mut next = draft_sig();
                                next.formula = event.value();
                                draft_sig.set(next);
                            },
                            onchange: move |event| {
                                let _ = push(doc, notice, Op::SetFormula { id: id.raw(), formula: event.value() });
                            },
                        }
                    }
                    label { "Unit label"
                        input {
                            value: "{draft().unit}",
                            onfocus: move |_| typing.set(true),
                            onblur: move |_| typing.set(false),
                            oninput: move |event| {
                                let mut next = draft_sig();
                                next.unit = event.value();
                                draft_sig.set(next);
                            },
                            onchange: move |event| commit_unit(doc, notice, id, event.value()),
                        }
                    }
                    { match &answer {
                        Some(answer) => {
                            let text = answer.kind.face("");
                            if let workspace_model::AnswerKind::Number(value) = &answer.kind {
                                rsx! {
                                    p { class: "exact", "{text}" }
                                    p { class: "exact", "Exact {value.exact()}" }
                                }
                            } else if answer.kind.is_refused() {
                                rsx! { p { class: "refusal-text", "{text}" } }
                            } else {
                                rsx! { p { class: "exact", "{text}" } }
                            }
                        }
                        None => rsx! { "" },
                    }}
                },
                Inside::Group { arrangement, .. } => rsx! {
                    div { class: "field-label", "Arrangement"
                        div { class: "arrange",
                            button {
                                class: if *arrangement == Arrangement::Stack { "on" } else { "" },
                                onclick: move |_| { let _ = push(doc, notice, Op::SetArrangement { id: id.raw(), arrangement: Arrangement::Stack }); },
                                "Stack"
                            }
                            button {
                                class: if *arrangement == Arrangement::Free { "on" } else { "" },
                                onclick: move |_| { let _ = push(doc, notice, Op::SetArrangement { id: id.raw(), arrangement: Arrangement::Free }); },
                                "Free"
                            }
                        }
                    }
                    div { class: "field-label", "Show on face"
                        for (child_id, child_name, shown) in children {
                            label { class: "tick",
                                input {
                                    r#type: "checkbox",
                                    checked: shown,
                                    onchange: move |_| {
                                        let _ = push(doc, notice, Op::SetShowOnFace { id: child_id.raw(), show: !shown });
                                    },
                                }
                                span { "{child_name}" }
                            }
                        }
                    }
                },
            }}
            button {
                class: "delete",
                onclick: move |_| delete_selected(doc, selected, notice),
                "Delete"
            }
        }
    }
}

fn typing_focus(on: bool) {
    let _ = on;
}

fn face_of(sheet: &Worksheet, answers: &std::collections::BTreeMap<CellId, Answer>, id: CellId) -> String {
    if let Some(answer) = answers.get(&id) {
        let unit = match sheet.cell(id).map(|cell| cell.inside()) {
            Some(Inside::Result { unit, .. }) => unit.as_str(),
            _ => "",
        };
        return answer.kind.face(unit);
    }
    sheet.face(id, &|_| None)
}

fn select_cell(
    sheet: &Worksheet,
    id: CellId,
    mut selected: Signal<Option<CellId>>,
    mut draft: Signal<Draft>,
) {
    selected.set(Some(id));
    let Some(cell) = sheet.cell(id) else {
        return;
    };
    let mut next = Draft {
        name: cell.name().to_string(),
        ..Draft::default()
    };
    match cell.inside() {
        Inside::Number { value, unit } => {
            next.value = value.decimal();
            next.unit = unit.clone();
        }
        Inside::Text { text } => next.text = text.clone(),
        Inside::Choice { options, .. } => next.options = options.join("\n"),
        Inside::Result { formula, unit } => {
            next.formula = formula.clone();
            next.unit = unit.clone();
        }
        Inside::Group { .. } => {}
    }
    draft.set(next);
}

fn selected_group(sheet: &Worksheet, selected: Option<CellId>) -> Option<CellId> {
    let id = selected.or_else(|| sheet.root())?;
    if sheet.cell(id).is_some_and(|cell| cell.inside().is_group()) {
        Some(id)
    } else {
        sheet.parent_of(id).or_else(|| sheet.root())
    }
}

fn place_new(
    mut doc: Signal<Document>,
    mut selected: Signal<Option<CellId>>,
    mut notice: Signal<Option<String>>,
    mut draft: Signal<Draft>,
    kind: BaseKind,
    parent: Option<CellId>,
    sheet: &Worksheet,
    tree: Option<&Laid>,
    world: (f64, f64),
) {
    let parent = parent.or_else(|| sheet.root());
    let (x, y, _index_unused) = local_drop(sheet, tree, parent, world);
    let added = add_cell(&mut doc.write(), parent, kind, x, y);
    match added {
        Ok(id) => {
            notice.set(None);
            select_cell(&sheet_now(&doc), id, selected, draft);
        }
        Err(refusal) => notice.set(Some(refusal.message())),
    }
}

fn local_drop(
    sheet: &Worksheet,
    tree: Option<&Laid>,
    parent: Option<CellId>,
    world: (f64, f64),
) -> (i64, i64, usize) {
    let Some(parent) = parent else {
        return (24, 48, 0);
    };
    let stack = matches!(
        sheet.cell(parent).map(|cell| cell.inside()),
        Some(Inside::Group {
            arrangement: Arrangement::Stack,
            ..
        })
    );
    let Some(group) = tree.and_then(|tree| find(tree, parent)) else {
        return (24, 48, 0);
    };
    if stack {
        return (12, 48, stack_index(group, world.1));
    }
    let x = (world.0 - group.x).round().max(0.0) as i64;
    let y = (world.1 - group.y).round().max(36.0) as i64;
    (x, y, group.children.len())
}

fn finish_move(
    mut doc: Signal<Document>,
    mut notice: Signal<Option<String>>,
    canvas_at: (f64, f64),
    pan: (f64, f64),
    scale: f64,
    drag: &PointerDrag,
    client: (f64, f64),
) {
    let sheet = sheet_now(&doc);
    let Some(tree) = layout::layout(&sheet, 1.0, 8) else {
        return;
    };
    let world = to_world(client, canvas_at, pan, scale);
    let Some(parent) = drop_group(&tree, world.0, world.1, Some(drag.id), &sheet) else {
        notice.set(Some("Drop a cell into a group.".to_string()));
        return;
    };
    let (x, y, index) = local_drop(&sheet, Some(&tree), Some(parent), world);
    let index = if sheet.parent_of(drag.id) == Some(parent)
        && !matches!(
            sheet.cell(parent).map(|cell| cell.inside()),
            Some(Inside::Group {
                arrangement: Arrangement::Stack,
                ..
            })
        ) {
        sheet
            .children_of(parent)
            .iter()
            .position(|placed| placed.id == drag.id)
            .unwrap_or(index)
    } else {
        index
    };
    let _ = push(
        doc,
        notice,
        Op::Move {
            id: drag.id.raw(),
            parent: parent.raw(),
            index,
            x,
            y,
        },
    );
}

fn commit_rename(mut doc: Signal<Document>, mut notice: Signal<Option<String>>, id: CellId, name: String) {
    let _ = push(doc, notice, Op::Rename { id: id.raw(), name });
}

fn commit_number(mut doc: Signal<Document>, mut notice: Signal<Option<String>>, id: CellId, text: String) {
    match Fraction::parse_decimal(&text) {
        Some(value) => {
            let _ = push(doc, notice, Op::SetNumber { id: id.raw(), value });
        }
        None => notice.set(Some("Type a number, such as 24 or 1.2.".to_string())),
    }
}

fn commit_unit(mut doc: Signal<Document>, mut notice: Signal<Option<String>>, id: CellId, unit: String) {
    let _ = push(doc, notice, Op::SetUnit { id: id.raw(), unit });
}

fn delete_selected(
    mut doc: Signal<Document>,
    mut selected: Signal<Option<CellId>>,
    mut notice: Signal<Option<String>>,
) {
    let Some(id) = selected() else {
        return;
    };
    if push(doc, notice, Op::Remove { id: id.raw() }) {
        selected.set(None);
    }
}

fn push(mut doc: Signal<Document>, mut notice: Signal<Option<String>>, op: Op) -> bool {
    match doc.write().push(op) {
        Ok(()) => {
            notice.set(None);
            true
        }
        Err(refusal) => {
            notice.set(Some(refusal.message()));
            false
        }
    }
}

fn sheet_now(doc: &Signal<Document>) -> Worksheet {
    doc.read().now().unwrap_or_else(|_| Worksheet::empty())
}

fn to_world(client: (f64, f64), origin: (f64, f64), pan: (f64, f64), scale: f64) -> (f64, f64) {
    let scale = if scale.abs() < 0.0001 { 1.0 } else { scale };
    (
        (client.0 - origin.0 - pan.0) / scale,
        (client.1 - origin.1 - pan.1) / scale,
    )
}

fn pointer_xy(event: &Event<PointerData>) -> (f64, f64) {
    let point = event.client_coordinates();
    (point.x, point.y)
}

fn drag_xy(event: &Event<DragData>) -> (f64, f64) {
    let point = event.client_coordinates();
    (point.x, point.y)
}

fn transfer_text(event: &Event<DragData>) -> String {
    event
        .data_transfer()
        .get_data("text/plain")
        .unwrap_or_default()
}
