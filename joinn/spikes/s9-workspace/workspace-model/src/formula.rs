//! A formula is typed into a Result. Arithmetic is exact. A refusal is a value.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::fraction::Fraction;
use crate::sheet::{CellId, Inside, Worksheet};

/// What a Result shows.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AnswerKind {
    Number(Fraction),
    Pass,
    Fail,
    Refused(String),
}

impl AnswerKind {
    /// The face: the answer and its unit, or `pass` / `fail`, or the refusal.
    pub fn face(&self, unit: &str) -> String {
        match self {
            Self::Number(value) => join_unit(&value.decimal(), unit),
            Self::Pass => "pass".to_string(),
            Self::Fail => "fail".to_string(),
            Self::Refused(message) => message.clone(),
        }
    }

    pub fn is_refused(&self) -> bool {
        matches!(self, Self::Refused(_))
    }
}

/// A Result's answer, and the cells its formula uses.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Answer {
    pub kind: AnswerKind,
    pub uses: Vec<CellId>,
}

/// Answers for every Result. Other cells are left out.
pub fn evaluate(sheet: &Worksheet) -> BTreeMap<CellId, Answer> {
    let mut parsed: BTreeMap<CellId, Result<Node, ()>> = BTreeMap::new();
    for cell in sheet.cells() {
        if let Inside::Result { formula, .. } = cell.inside() {
            parsed.insert(cell.id(), parse_formula(formula));
        }
    }

    let mut resolved: BTreeMap<CellId, Result<(Resolved, Vec<CellId>), String>> = BTreeMap::new();
    for (id, node) in &parsed {
        resolved.insert(
            *id,
            match node {
                Err(()) => Err("This formula can't be read.".to_string()),
                Ok(node) => resolve_tree(sheet, *id, node),
            },
        );
    }

    let mut graph: BTreeMap<CellId, Vec<CellId>> = BTreeMap::new();
    for (id, result) in &resolved {
        if let Ok((tree, _)) = result {
            let mut deps = Vec::new();
            result_deps(sheet, tree, &mut deps);
            graph.insert(*id, deps);
        }
    }
    let cyclic = cyclic_results(&graph);

    let mut answers = BTreeMap::new();
    let mut memo: BTreeMap<CellId, Result<Value, String>> = BTreeMap::new();
    for id in parsed.keys().copied() {
        let uses = match &resolved[&id] {
            Ok((_, uses)) => uses.clone(),
            Err(_) => Vec::new(),
        };
        let kind = if cyclic.contains(&id) {
            AnswerKind::Refused("This result depends on itself.".to_string())
        } else {
            match value_of(sheet, id, &resolved, &mut memo, &mut Vec::new()) {
                Ok(Value::Number(value)) => AnswerKind::Number(value),
                Ok(Value::Check(true)) => AnswerKind::Pass,
                Ok(Value::Check(false)) => AnswerKind::Fail,
                Err(message) => AnswerKind::Refused(message),
            }
        };
        answers.insert(id, Answer { kind, uses });
    }
    answers
}

fn join_unit(value: &str, unit: &str) -> String {
    let unit = unit.trim();
    if unit.is_empty() {
        value.to_string()
    } else {
        format!("{value} {unit}")
    }
}

#[derive(Clone, Copy)]
enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Copy)]
enum CmpOp {
    Lt,
    Le,
    Gt,
    Ge,
}

enum Node {
    Num(Fraction),
    Name(Vec<String>),
    Neg(Box<Node>),
    Bin(BinOp, Box<Node>, Box<Node>),
    Pow(Box<Node>, u32),
    Cmp(CmpOp, Box<Node>, Box<Node>),
}

enum Resolved {
    Num(Fraction),
    Cell(CellId),
    Neg(Box<Resolved>),
    Bin(BinOp, Box<Resolved>, Box<Resolved>),
    Pow(Box<Resolved>, u32),
    Cmp(CmpOp, Box<Resolved>, Box<Resolved>),
}

#[derive(Clone)]
enum Value {
    Number(Fraction),
    Check(bool),
}

fn result_deps(sheet: &Worksheet, node: &Resolved, out: &mut Vec<CellId>) {
    match node {
        Resolved::Cell(id) => {
            if matches!(sheet.cell(*id).map(|cell| cell.inside()), Some(Inside::Result { .. })) {
                out.push(*id);
            }
        }
        Resolved::Neg(inner) | Resolved::Pow(inner, _) => result_deps(sheet, inner, out),
        Resolved::Bin(_, left, right) | Resolved::Cmp(_, left, right) => {
            result_deps(sheet, left, out);
            result_deps(sheet, right, out);
        }
        Resolved::Num(_) => {}
    }
}

fn cyclic_results(graph: &BTreeMap<CellId, Vec<CellId>>) -> BTreeSet<CellId> {
    let mut cyclic = BTreeSet::new();
    for id in graph.keys().copied() {
        if reaches_self(graph, id) {
            cyclic.insert(id);
        }
    }
    cyclic
}

fn reaches_self(graph: &BTreeMap<CellId, Vec<CellId>>, start: CellId) -> bool {
    let mut seen = BTreeSet::new();
    let mut queue = VecDeque::new();
    if let Some(deps) = graph.get(&start) {
        for dep in deps {
            queue.push_back(*dep);
        }
    }
    let limit = graph.len() + 1;
    let mut steps = 0;
    while let Some(current) = queue.pop_front() {
        if current == start {
            return true;
        }
        if !seen.insert(current) {
            continue;
        }
        steps += 1;
        if steps > limit {
            break;
        }
        if let Some(deps) = graph.get(&current) {
            for dep in deps {
                queue.push_back(*dep);
            }
        }
    }
    false
}

fn value_of(
    sheet: &Worksheet,
    id: CellId,
    resolved: &BTreeMap<CellId, Result<(Resolved, Vec<CellId>), String>>,
    memo: &mut BTreeMap<CellId, Result<Value, String>>,
    stack: &mut Vec<CellId>,
) -> Result<Value, String> {
    if let Some(ready) = memo.get(&id) {
        return ready.clone();
    }
    if stack.contains(&id) {
        return Err("This result depends on itself.".to_string());
    }
    let Some(cell) = sheet.cell(id) else {
        return Err("There is no cell named there.".to_string());
    };
    let computed = match cell.inside() {
        Inside::Number { value, .. } => Ok(Value::Number(*value)),
        Inside::Result { .. } => {
            let tree = match resolved.get(&id) {
                Some(Ok((tree, _))) => tree,
                Some(Err(message)) => return Err(message.clone()),
                None => return Err("This formula can't be read.".to_string()),
            };
            stack.push(id);
            let value = eval_resolved(sheet, tree, resolved, memo, stack);
            stack.pop();
            value
        }
        Inside::Text { .. } | Inside::Choice { .. } | Inside::Group { .. } => {
            Err(format!("\"{}\" is not a number.", cell.name()))
        }
    };
    if !matches!(cell.inside(), Inside::Result { .. }) {
        return computed;
    }
    memo.insert(id, computed.clone());
    computed
}

fn eval_resolved(
    sheet: &Worksheet,
    node: &Resolved,
    resolved: &BTreeMap<CellId, Result<(Resolved, Vec<CellId>), String>>,
    memo: &mut BTreeMap<CellId, Result<Value, String>>,
    stack: &mut Vec<CellId>,
) -> Result<Value, String> {
    match node {
        Resolved::Num(value) => Ok(Value::Number(*value)),
        Resolved::Cell(id) => {
            let value = value_of(sheet, *id, resolved, memo, stack)?;
            match value {
                Value::Number(_) => Ok(value),
                Value::Check(_) => {
                    let name = sheet.cell(*id).map(|cell| cell.name()).unwrap_or("that cell");
                    Err(format!("\"{name}\" is not a number."))
                }
            }
        }
        Resolved::Neg(inner) => match eval_resolved(sheet, inner, resolved, memo, stack)? {
            Value::Number(value) => value
                .checked_mul(Fraction::from_i64(-1))
                .map(Value::Number)
                .ok_or_else(|| "This number is too big to work out.".to_string()),
            Value::Check(_) => Err("A check is not a number.".to_string()),
        },
        Resolved::Bin(op, left, right) => {
            let left = number_of(eval_resolved(sheet, left, resolved, memo, stack)?)?;
            let right = number_of(eval_resolved(sheet, right, resolved, memo, stack)?)?;
            let value = match op {
                BinOp::Add => left.checked_add(right),
                BinOp::Sub => left.checked_sub(right),
                BinOp::Mul => left.checked_mul(right),
                BinOp::Div => {
                    if right.is_zero() {
                        return Err("This divides by zero.".to_string());
                    }
                    left.checked_div(right)
                }
            };
            value
                .map(Value::Number)
                .ok_or_else(|| "This number is too big to work out.".to_string())
        }
        Resolved::Pow(inner, exp) => {
            let base = number_of(eval_resolved(sheet, inner, resolved, memo, stack)?)?;
            base.checked_pow(*exp)
                .map(Value::Number)
                .ok_or_else(|| "This number is too big to work out.".to_string())
        }
        Resolved::Cmp(op, left, right) => {
            let left = number_of(eval_resolved(sheet, left, resolved, memo, stack)?)?;
            let right = number_of(eval_resolved(sheet, right, resolved, memo, stack)?)?;
            let ordering = left.cmp_value(right);
            let pass = match op {
                CmpOp::Lt => ordering.is_lt(),
                CmpOp::Le => ordering.is_le(),
                CmpOp::Gt => ordering.is_gt(),
                CmpOp::Ge => ordering.is_ge(),
            };
            Ok(Value::Check(pass))
        }
    }
}

fn number_of(value: Value) -> Result<Fraction, String> {
    match value {
        Value::Number(value) => Ok(value),
        Value::Check(_) => Err("A check is not a number.".to_string()),
    }
}

fn resolve_tree(
    sheet: &Worksheet,
    from: CellId,
    node: &Node,
) -> Result<(Resolved, Vec<CellId>), String> {
    let mut uses = Vec::new();
    let tree = resolve_node(sheet, from, node, &mut uses)?;
    Ok((tree, uses))
}

fn resolve_node(
    sheet: &Worksheet,
    from: CellId,
    node: &Node,
    uses: &mut Vec<CellId>,
) -> Result<Resolved, String> {
    match node {
        Node::Num(value) => Ok(Resolved::Num(*value)),
        Node::Name(path) => {
            let id = lookup(sheet, from, path)?;
            if !uses.contains(&id) {
                uses.push(id);
            }
            Ok(Resolved::Cell(id))
        }
        Node::Neg(inner) => Ok(Resolved::Neg(Box::new(resolve_node(sheet, from, inner, uses)?))),
        Node::Bin(op, left, right) => Ok(Resolved::Bin(
            *op,
            Box::new(resolve_node(sheet, from, left, uses)?),
            Box::new(resolve_node(sheet, from, right, uses)?),
        )),
        Node::Pow(inner, exp) => Ok(Resolved::Pow(
            Box::new(resolve_node(sheet, from, inner, uses)?),
            *exp,
        )),
        Node::Cmp(op, left, right) => Ok(Resolved::Cmp(
            *op,
            Box::new(resolve_node(sheet, from, left, uses)?),
            Box::new(resolve_node(sheet, from, right, uses)?),
        )),
    }
}

/// Siblings first, then outward. `Inputs.L` steps into a named group. The nearest name wins.
fn lookup(sheet: &Worksheet, from: CellId, path: &[String]) -> Result<CellId, String> {
    let Some((first, rest)) = path.split_first() else {
        return Err("This formula can't be read.".to_string());
    };
    let mut current = find_nearest(sheet, from, first)
        .ok_or_else(|| format!("There is no cell named \"{first}\"."))?;
    for segment in rest {
        let Some(next) = find_inside(sheet, current, segment) else {
            let owner = sheet
                .cell(current)
                .map(|cell| cell.name())
                .unwrap_or(first.as_str());
            return Err(format!("There is no cell named \"{segment}\" inside \"{owner}\"."));
        };
        current = next;
    }
    Ok(current)
}

fn find_nearest(sheet: &Worksheet, from: CellId, name: &str) -> Option<CellId> {
    let mut scope = sheet.parent_of(from);
    while let Some(scope_id) = scope {
        if let Some(found) = find_in(sheet, scope_id, name) {
            return Some(found);
        }
        scope = sheet.parent_of(scope_id);
    }
    None
}

fn find_inside(sheet: &Worksheet, group: CellId, name: &str) -> Option<CellId> {
    find_in(sheet, group, name)
}

fn find_in(sheet: &Worksheet, scope: CellId, name: &str) -> Option<CellId> {
    for placed in sheet.children_of(scope) {
        if sheet.cell(placed.id).is_some_and(|cell| cell.name() == name) {
            return Some(placed.id);
        }
    }
    let mut queue = VecDeque::new();
    for placed in sheet.children_of(scope) {
        if sheet.cell(placed.id).is_some_and(|cell| cell.inside().is_group()) {
            queue.push_back(placed.id);
        }
    }
    while let Some(group) = queue.pop_front() {
        for placed in sheet.children_of(group) {
            if sheet.cell(placed.id).is_some_and(|cell| cell.name() == name) {
                return Some(placed.id);
            }
            if sheet.cell(placed.id).is_some_and(|cell| cell.inside().is_group()) {
                queue.push_back(placed.id);
            }
        }
    }
    None
}

fn parse_formula(text: &str) -> Result<Node, ()> {
    let mut parser = Parser { text, index: 0 };
    let node = parser.parse_expr()?;
    parser.skip();
    if parser.index != parser.text.len() {
        return Err(());
    }
    Ok(node)
}

struct Parser<'a> {
    text: &'a str,
    index: usize,
}

impl<'a> Parser<'a> {
    fn parse_expr(&mut self) -> Result<Node, ()> {
        let left = self.parse_sum()?;
        self.skip();
        if let Some(op) = self.comparison() {
            let right = self.parse_sum()?;
            return Ok(Node::Cmp(op, Box::new(left), Box::new(right)));
        }
        Ok(left)
    }

    fn comparison(&mut self) -> Option<CmpOp> {
        if self.eat("<=") {
            Some(CmpOp::Le)
        } else if self.eat(">=") {
            Some(CmpOp::Ge)
        } else if self.eat("<") {
            Some(CmpOp::Lt)
        } else if self.eat(">") {
            Some(CmpOp::Gt)
        } else {
            None
        }
    }

    fn parse_sum(&mut self) -> Result<Node, ()> {
        let mut left = self.parse_product()?;
        loop {
            self.skip();
            let op = if self.eat("+") {
                BinOp::Add
            } else if self.eat("-") {
                BinOp::Sub
            } else {
                break;
            };
            let right = self.parse_product()?;
            left = Node::Bin(op, Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_product(&mut self) -> Result<Node, ()> {
        let mut left = self.parse_power()?;
        loop {
            self.skip();
            let op = if self.eat("*") {
                BinOp::Mul
            } else if self.eat("/") {
                BinOp::Div
            } else {
                break;
            };
            let right = self.parse_power()?;
            left = Node::Bin(op, Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Node, ()> {
        let base = self.parse_unary()?;
        self.skip();
        if self.eat("^") {
            self.skip();
            let exp = self.whole_exponent()?;
            return Ok(Node::Pow(Box::new(base), exp));
        }
        Ok(base)
    }

    fn whole_exponent(&mut self) -> Result<u32, ()> {
        let start = self.index;
        while self.index < self.text.len() && self.text.as_bytes()[self.index].is_ascii_digit() {
            self.index += 1;
        }
        if start == self.index {
            return Err(());
        }
        self.text[start..self.index].parse().map_err(|_| ())
    }

    fn parse_unary(&mut self) -> Result<Node, ()> {
        self.skip();
        if self.eat("-") {
            let inner = self.parse_unary()?;
            return Ok(Node::Neg(Box::new(inner)));
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Node, ()> {
        self.skip();
        if self.eat("(") {
            let inner = self.parse_sum()?;
            self.skip();
            if !self.eat(")") {
                return Err(());
            }
            return Ok(inner);
        }
        if let Some(value) = self.number() {
            return Ok(Node::Num(value));
        }
        if let Some(path) = self.name_path() {
            return Ok(Node::Name(path));
        }
        Err(())
    }

    fn number(&mut self) -> Option<Fraction> {
        let start = self.index;
        if start >= self.text.len() || !self.text.as_bytes()[start].is_ascii_digit() {
            return None;
        }
        self.index += 1;
        while self.index < self.text.len() && self.text.as_bytes()[self.index].is_ascii_digit() {
            self.index += 1;
        }
        if self.index < self.text.len() && self.text.as_bytes()[self.index] == b'.' {
            let dot = self.index;
            self.index += 1;
            let frac = self.index;
            while self.index < self.text.len() && self.text.as_bytes()[self.index].is_ascii_digit() {
                self.index += 1;
            }
            if self.index == frac {
                self.index = dot;
            }
        }
        Fraction::parse_decimal(&self.text[start..self.index])
    }

    fn name_path(&mut self) -> Option<Vec<String>> {
        let first = self.segment()?;
        let mut path = vec![first];
        loop {
            let saved = self.index;
            self.skip();
            if self.eat(".") {
                self.skip();
                match self.segment() {
                    Some(segment) => path.push(segment),
                    None => {
                        self.index = saved;
                        break;
                    }
                }
            } else {
                self.index = saved;
                break;
            }
        }
        Some(path)
    }

    fn segment(&mut self) -> Option<String> {
        self.skip();
        if self.eat("\"") {
            let start = self.index;
            while self.index < self.text.len() && !self.text[self.index..].starts_with('"') {
                self.index += self.text[self.index..].chars().next().map(|ch| ch.len_utf8()).unwrap_or(1);
            }
            if self.index >= self.text.len() {
                return None;
            }
            let text = self.text[start..self.index].to_string();
            self.index += 1;
            if text.is_empty() {
                return None;
            }
            return Some(text);
        }
        let start = self.index;
        let bytes = self.text.as_bytes();
        if start >= bytes.len() || !is_name_start(bytes[start]) {
            return None;
        }
        self.index += 1;
        while self.index < bytes.len() && is_name_continue(bytes[self.index]) {
            self.index += 1;
        }
        Some(self.text[start..self.index].to_string())
    }

    fn eat(&mut self, token: &str) -> bool {
        self.skip();
        if self.text[self.index..].starts_with(token) {
            self.index += token.len();
            true
        } else {
            false
        }
    }

    fn skip(&mut self) {
        while self.index < self.text.len() {
            let ch = self.text[self.index..].chars().next().unwrap_or(' ');
            if ch.is_whitespace() {
                self.index += ch.len_utf8();
            } else {
                break;
            }
        }
    }
}

fn is_name_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
}

fn is_name_continue(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::log::{Document, Op};
    use crate::sheet::{Arrangement, Inside, Worksheet};

    fn only(sheet: &Worksheet, name: &str) -> CellId {
        let found = sheet.cells_named(name);
        assert_eq!(found.len(), 1, "{name}");
        found[0]
    }

    fn kind<'a>(answers: &'a BTreeMap<CellId, Answer>, sheet: &Worksheet, name: &str) -> &'a AnswerKind {
        &answers[&only(sheet, name)].kind
    }

    #[test]
    fn simple_beam_and_a_changed_load() {
        let doc = crate::example::simple_beam();
        let sheet = doc.now().expect("beam");
        let answers = evaluate(&sheet);
        assert_eq!(
            kind(&answers, &sheet, "R"),
            &AnswerKind::Number(Fraction::new(72, 5).expect("72/5"))
        );
        assert_eq!(
            kind(&answers, &sheet, "M"),
            &AnswerKind::Number(Fraction::new(432, 5).expect("432/5"))
        );
        assert_eq!(kind(&answers, &sheet, "Check"), &AnswerKind::Pass);
        assert!(answers[&only(&sheet, "M")]
            .uses
            .iter()
            .any(|id| sheet.cell(*id).is_some_and(|cell| cell.name() == "L")));

        let mut doc = doc;
        let w = only(&sheet, "w");
        doc.push(Op::SetNumber {
            id: w.raw(),
            value: Fraction::from_i64(2),
        })
        .expect("set w");
        let sheet = doc.now().expect("beam");
        let answers = evaluate(&sheet);
        assert_eq!(
            kind(&answers, &sheet, "R"),
            &AnswerKind::Number(Fraction::from_i64(24))
        );
        assert_eq!(
            kind(&answers, &sheet, "M"),
            &AnswerKind::Number(Fraction::from_i64(144))
        );
        assert_eq!(kind(&answers, &sheet, "Check"), &AnswerKind::Fail);
    }

    #[test]
    fn each_refusal_names_its_cause_and_a_neighbour_still_answers() {
        let mut doc = Document::new();
        let add = |doc: &mut Document, id, parent, name: &str, inside, show| {
            doc.push(Op::Add {
                id,
                parent,
                name: name.to_string(),
                inside,
                x: 0,
                y: 0,
                show_on_face: show,
            })
            .expect(name);
        };
        add(
            &mut doc,
            1,
            None,
            "Sheet",
            Inside::Group {
                arrangement: Arrangement::Stack,
                children: Vec::new(),
            },
            false,
        );
        add(
            &mut doc,
            2,
            Some(1),
            "A",
            Inside::Number {
                value: Fraction::from_i64(1),
                unit: String::new(),
            },
            false,
        );
        add(
            &mut doc,
            3,
            Some(1),
            "B",
            Inside::Number {
                value: Fraction::from_i64(0),
                unit: String::new(),
            },
            false,
        );
        add(
            &mut doc,
            4,
            Some(1),
            "Note",
            Inside::Text {
                text: "hello".into(),
            },
            false,
        );
        let result = |formula: &str| Inside::Result {
            formula: formula.to_string(),
            unit: String::new(),
        };
        add(&mut doc, 5, Some(1), "Bad", result("1 +"), true);
        add(&mut doc, 6, Some(1), "Missing", result("nope"), true);
        add(&mut doc, 7, Some(1), "NotNum", result("Note"), true);
        add(&mut doc, 8, Some(1), "Div0", result("A / B"), true);
        add(&mut doc, 9, Some(1), "Self", result("Self"), true);
        add(&mut doc, 10, Some(1), "Ok", result("A + 1"), true);
        add(&mut doc, 11, Some(1), "Cycle1", result("Cycle2"), true);
        add(&mut doc, 12, Some(1), "Cycle2", result("Cycle1 + 1"), true);
        add(&mut doc, 13, Some(1), "Neighbour", result("A"), true);

        let sheet = doc.now().expect("sheet");
        let answers = evaluate(&sheet);
        let message = |name: &str| match kind(&answers, &sheet, name) {
            AnswerKind::Refused(message) => message.clone(),
            other => panic!("{name} should be refused, got {other:?}"),
        };
        assert!(message("Bad").contains("can't be read"), "{}", message("Bad"));
        assert!(message("Missing").contains("no cell named"), "{}", message("Missing"));
        assert!(message("Missing").contains("nope"));
        assert!(message("NotNum").contains("not a number"), "{}", message("NotNum"));
        assert!(message("NotNum").contains("Note"));
        assert!(message("Div0").contains("divides by zero"), "{}", message("Div0"));
        assert!(message("Self").contains("depends on itself"), "{}", message("Self"));
        assert!(message("Cycle1").contains("depends on itself"));
        assert!(message("Cycle2").contains("depends on itself"));
        assert_eq!(
            kind(&answers, &sheet, "Ok"),
            &AnswerKind::Number(Fraction::from_i64(2))
        );
        assert_eq!(
            kind(&answers, &sheet, "Neighbour"),
            &AnswerKind::Number(Fraction::from_i64(1))
        );
    }

    #[test]
    fn a_quoted_path_finds_a_name_with_a_space() {
        let mut doc = Document::new();
        doc.push(Op::Add {
            id: 1,
            parent: None,
            name: "Sheet".into(),
            inside: Inside::Group {
                arrangement: Arrangement::Stack,
                children: Vec::new(),
            },
            x: 0,
            y: 0,
            show_on_face: false,
        })
        .unwrap();
        doc.push(Op::Add {
            id: 2,
            parent: Some(1),
            name: "Inputs".into(),
            inside: Inside::Group {
                arrangement: Arrangement::Stack,
                children: Vec::new(),
            },
            x: 0,
            y: 0,
            show_on_face: false,
        })
        .unwrap();
        doc.push(Op::Add {
            id: 3,
            parent: Some(2),
            name: "Span length".into(),
            inside: Inside::Number {
                value: Fraction::from_i64(10),
                unit: "ft".into(),
            },
            x: 0,
            y: 0,
            show_on_face: false,
        })
        .unwrap();
        doc.push(Op::Add {
            id: 4,
            parent: Some(1),
            name: "R".into(),
            inside: Inside::Result {
                formula: "Inputs.\"Span length\" / 2".into(),
                unit: String::new(),
            },
            x: 0,
            y: 0,
            show_on_face: true,
        })
        .unwrap();
        let sheet = doc.now().unwrap();
        let answers = evaluate(&sheet);
        assert_eq!(
            kind(&answers, &sheet, "R"),
            &AnswerKind::Number(Fraction::from_i64(5))
        );
    }
}
