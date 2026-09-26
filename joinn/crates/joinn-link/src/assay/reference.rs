//! Reference derivation: every port stays a block. Homology must match the fast one.

use joinn_dna::{Direction, GenomeTarget, PortDecl};
use joinn_frame::{FrameRef, Hash, Verdict};
use joinn_prim::prim_ports;
use std::collections::{BTreeMap, BTreeSet};

use super::roundtrip::roundtrip;
use super::{AssayReport, RegionPieces};
use crate::{Bound, Mark, Universe};

/// Second derivation. A loop is recorded, never refused for being open.
pub fn assay_reference(universe: &Universe, bound: &Bound) -> Verdict<AssayReport> {
    let headline = if universe.coding.bodies.len() == 1
        && universe
            .coding
            .bodies
            .first()
            .is_some_and(|b| b.alias == "body")
    {
        "assay body".to_string()
    } else {
        "assay universe".to_string()
    };

    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
    enum V {
        Out,
        Centre(String, String),
        Port(String, String, u32),
    }
    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct Key {
        kind: u8,
        hash: Hash,
        instance: String,
        port: u32,
        alias: String,
    }
    struct Inst {
        alias: String,
        name: String,
        genome_pos: u32,
        cell: Option<Hash>,
        ports: Vec<PortDecl>,
    }
    struct Edge {
        key: Key,
        tail: u32,
        head: u32,
        kind: u8,
        step: String,
    }

    let mut insts: Vec<Inst> = Vec::new();
    let mut by_alias: BTreeMap<String, (Hash, Vec<usize>)> = BTreeMap::new();
    let mut cells_of: BTreeMap<String, BTreeMap<Hash, joinn_dna::Cell>> = BTreeMap::new();
    for binding in &universe.coding.bodies {
        let Some((body, cells)) = bound.get(&binding.alias) else {
            return crate::refuse(format!(
                "alias {} is not bound; acceptance is a body for every alias",
                binding.alias
            ));
        };
        cells_of.insert(binding.alias.clone(), cells.clone());
        let mut idxs = Vec::new();
        let mut genome_pos = 0_u32;
        for entry in &body.coding.genome {
            let ports = match &entry.target {
                GenomeTarget::Cell(hash) => match cells.get(hash) {
                    Some(cell) => cell.coding.contract.ports.clone(),
                    None => {
                        return crate::refuse(format!(
                            "instance {} cell {} was not supplied; acceptance is that cell in the cell map",
                            entry.instances.join(", "),
                            hash.short_hex()
                        ));
                    }
                },
                GenomeTarget::Prim(name) => match prim_ports(name) {
                    Some(ports) => ports,
                    None => {
                        return crate::refuse(format!(
                            "primitive {name} has no port table; acceptance is a floor primitive"
                        ));
                    }
                },
            };
            let cell = match &entry.target {
                GenomeTarget::Cell(hash) => Some(*hash),
                GenomeTarget::Prim(_) => None,
            };
            for name in &entry.instances {
                idxs.push(insts.len());
                insts.push(Inst {
                    alias: binding.alias.clone(),
                    name: name.clone(),
                    genome_pos,
                    cell,
                    ports: ports.clone(),
                });
                genome_pos = genome_pos.saturating_add(1);
            }
        }
        by_alias.insert(binding.alias.clone(), (binding.hash, idxs));
    }

    let find_inst = |alias: &str, name: &str| -> Option<usize> {
        insts
            .iter()
            .position(|i| i.alias == alias && i.name == name)
    };
    let port_frame = |idx: usize, port: u32| -> Option<FrameRef> {
        insts
            .get(idx)
            .and_then(|i| i.ports.iter().find(|p| p.position == port))
            .map(|p| p.frame)
    };

    let mut parent: Vec<usize> = (0..insts.len()).collect();
    let find = |parent: &mut [usize], mut x: usize| -> usize {
        let mut seen = 0_u32;
        while parent.get(x).is_some_and(|p| *p != x) {
            let p = parent[x];
            let gp = parent.get(p).copied().unwrap_or(p);
            parent[x] = gp;
            x = p;
            seen = seen.saturating_add(1);
            if seen > 100000 {
                break;
            }
        }
        x
    };
    for (alias, (_, idxs)) in &by_alias {
        let Some((body, _)) = bound.get(alias) else {
            continue;
        };
        for wire in &body.coding.wires {
            let Some(src) = find_inst(alias, &wire.src_instance) else {
                continue;
            };
            let Some(dst) = find_inst(alias, &wire.dst_instance) else {
                continue;
            };
            if idxs.contains(&src) && idxs.contains(&dst) {
                let ra = find(&mut parent, src);
                let rb = find(&mut parent, dst);
                if ra != rb {
                    parent[rb] = ra;
                }
            }
        }
    }
    for i in 0..parent.len() {
        parent[i] = find(&mut parent, i);
    }

    let mut region_name_of: Vec<String> = Vec::new();
    let mut region_of: Vec<usize> = vec![0; insts.len()];
    let mut region_lines = Vec::new();
    let mut aliases: Vec<String> = by_alias.keys().cloned().collect();
    aliases.sort();
    for alias in &aliases {
        let Some((_, idxs)) = by_alias.get(alias) else {
            continue;
        };
        let mut groups: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for idx in idxs {
            let root = parent.get(*idx).copied().unwrap_or(*idx);
            groups.entry(root).or_default().push(*idx);
        }
        let mut pieces: Vec<(u32, Vec<String>, Vec<usize>)> = Vec::new();
        for members in groups.values() {
            let mut names: Vec<String> = members
                .iter()
                .filter_map(|i| insts.get(*i).map(|n| n.name.clone()))
                .collect();
            names.sort();
            let pos = members
                .iter()
                .filter_map(|i| insts.get(*i).map(|n| n.genome_pos))
                .min()
                .unwrap_or(0);
            pieces.push((pos, names, members.clone()));
        }
        pieces.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        let piece_count = pieces.len();
        let mut printed = Vec::new();
        for (_, names, members) in &pieces {
            let name = if piece_count == 1 {
                alias.clone()
            } else {
                format!("{{{}}}", names.join(", "))
            };
            let id = region_name_of.len();
            for idx in members {
                if let Some(slot) = region_of.get_mut(*idx) {
                    *slot = id;
                }
            }
            region_name_of.push(name);
            printed.push(names.clone());
        }
        region_lines.push(RegionPieces {
            alias: alias.clone(),
            pieces: printed,
        });
    }
    let region_name =
        |idx: usize| -> String { region_name_of.get(idx).cloned().unwrap_or_default() };

    let mut verts: Vec<V> = vec![V::Out];
    for inst in &insts {
        verts.push(V::Centre(inst.alias.clone(), inst.name.clone()));
        for port in &inst.ports {
            verts.push(V::Port(
                inst.alias.clone(),
                inst.name.clone(),
                port.position,
            ));
        }
    }
    verts.sort();
    verts.dedup();
    let mut id_of: BTreeMap<V, u32> = BTreeMap::new();
    for vert in &verts {
        let n = u32::try_from(id_of.len()).unwrap_or(0);
        id_of.insert(vert.clone(), n);
    }
    let vid = |v: &V| -> u32 { id_of.get(v).copied().unwrap_or(0) };

    let linked: BTreeSet<(String, String, u32)> = universe
        .coding
        .links
        .iter()
        .flat_map(|link| {
            link.members
                .iter()
                .map(|m| (m.body.clone(), m.instance.clone(), m.port))
        })
        .collect();

    struct LinkRec {
        tail_alias: String,
        tail_inst: String,
        tail_port: u32,
        head_alias: String,
        head_inst: String,
        head_port: u32,
        tail_region: usize,
        head_region: usize,
        key: Key,
    }
    let mut links: Vec<LinkRec> = Vec::new();
    let mut not_measured = 0_u32;
    for link in &universe.coding.links {
        if link.members.len() != 2 {
            not_measured = not_measured.saturating_add(1);
            continue;
        }
        let tail = link.members.iter().find(|m| m.mark == Mark::Tail);
        let head = link.members.iter().find(|m| m.mark == Mark::Head);
        let (Some(tail), Some(head)) = (tail, head) else {
            not_measured = not_measured.saturating_add(1);
            continue;
        };
        let Some(tail_i) = find_inst(&tail.body, &tail.instance) else {
            not_measured = not_measured.saturating_add(1);
            continue;
        };
        let Some(head_i) = find_inst(&head.body, &head.instance) else {
            not_measured = not_measured.saturating_add(1);
            continue;
        };
        let Some((hash, _)) = by_alias.get(&tail.body) else {
            continue;
        };
        links.push(LinkRec {
            tail_alias: tail.body.clone(),
            tail_inst: tail.instance.clone(),
            tail_port: tail.port,
            head_alias: head.body.clone(),
            head_inst: head.instance.clone(),
            head_port: head.port,
            tail_region: region_of.get(tail_i).copied().unwrap_or(0),
            head_region: region_of.get(head_i).copied().unwrap_or(0),
            key: Key {
                kind: 2,
                hash: *hash,
                instance: tail.instance.clone(),
                port: tail.port,
                alias: tail.body.clone(),
            },
        });
    }
    links.sort_by(|a, b| a.key.cmp(&b.key));

    struct Door {
        key: Key,
        region: usize,
        label: String,
        frame: FrameRef,
        alias: String,
        instance: String,
        port: u32,
        inst_idx: usize,
    }
    let mut entries: Vec<Door> = Vec::new();
    let mut exits: Vec<Door> = Vec::new();
    for (alias, (hash, _)) in &by_alias {
        let Some((body, cells)) = bound.get(alias) else {
            continue;
        };
        let membrane = match crate::membrane(body, cells) {
            Verdict::Ok(m) => m,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        for port in membrane {
            if linked.contains(&(
                alias.clone(),
                port.address.instance.clone(),
                port.address.port,
            )) {
                continue;
            }
            let Some(idx) = find_inst(alias, &port.address.instance) else {
                continue;
            };
            let door = Door {
                key: Key {
                    kind: if port.direction == Direction::In {
                        0
                    } else {
                        1
                    },
                    hash: *hash,
                    instance: port.address.instance.clone(),
                    port: port.address.port,
                    alias: alias.clone(),
                },
                region: region_of.get(idx).copied().unwrap_or(0),
                label: format!("{alias}.{}@{}", port.address.instance, port.address.port),
                frame: port.frame,
                alias: alias.clone(),
                instance: port.address.instance.clone(),
                port: port.address.port,
                inst_idx: idx,
            };
            if port.direction == Direction::In {
                entries.push(door);
            } else {
                exits.push(door);
            }
        }
    }
    entries.sort_by(|a, b| a.key.cmp(&b.key));
    exits.sort_by(|a, b| a.key.cmp(&b.key));

    let mut edges: Vec<Edge> = Vec::new();
    let push_edge = |edges: &mut Vec<Edge>, key: Key, tail: V, head: V, kind: u8, step: String| {
        edges.push(Edge {
            key,
            tail: vid(&tail),
            head: vid(&head),
            kind,
            step,
        });
    };
    for inst in &insts {
        let Some((hash, _)) = by_alias.get(&inst.alias) else {
            continue;
        };
        let centre = V::Centre(inst.alias.clone(), inst.name.clone());
        for port in &inst.ports {
            let pv = V::Port(inst.alias.clone(), inst.name.clone(), port.position);
            if port.direction == Direction::In {
                push_edge(
                    &mut edges,
                    Key {
                        kind: 3,
                        hash: *hash,
                        instance: inst.name.clone(),
                        port: port.position,
                        alias: "in".to_string(),
                    },
                    pv,
                    centre.clone(),
                    3,
                    String::new(),
                );
            } else {
                push_edge(
                    &mut edges,
                    Key {
                        kind: 3,
                        hash: *hash,
                        instance: inst.name.clone(),
                        port: port.position,
                        alias: "out".to_string(),
                    },
                    centre.clone(),
                    pv,
                    3,
                    String::new(),
                );
            }
        }
    }
    for alias in &aliases {
        let Some((body, _)) = bound.get(alias) else {
            continue;
        };
        let Some((hash, _)) = by_alias.get(alias) else {
            continue;
        };
        for wire in &body.coding.wires {
            push_edge(
                &mut edges,
                Key {
                    kind: 4,
                    hash: *hash,
                    instance: wire.src_instance.clone(),
                    port: wire.src_port,
                    alias: wire.dst_instance.clone(),
                },
                V::Port(alias.clone(), wire.src_instance.clone(), wire.src_port),
                V::Port(alias.clone(), wire.dst_instance.clone(), wire.dst_port),
                4,
                String::new(),
            );
        }
    }
    for link in &links {
        push_edge(
            &mut edges,
            link.key.clone(),
            V::Port(
                link.tail_alias.clone(),
                link.tail_inst.clone(),
                link.tail_port,
            ),
            V::Port(
                link.head_alias.clone(),
                link.head_inst.clone(),
                link.head_port,
            ),
            2,
            format!(
                " →{}.{}@{}→ {}",
                link.tail_alias,
                link.tail_inst,
                link.tail_port,
                region_name(link.head_region)
            ),
        );
    }
    for door in &entries {
        push_edge(
            &mut edges,
            door.key.clone(),
            V::Out,
            V::Port(door.alias.clone(), door.instance.clone(), door.port),
            0,
            String::new(),
        );
    }
    for door in &exits {
        push_edge(
            &mut edges,
            door.key.clone(),
            V::Port(door.alias.clone(), door.instance.clone(), door.port),
            V::Out,
            1,
            String::new(),
        );
    }
    edges.sort_by(|a, b| a.key.cmp(&b.key));

    let vcount = u32::try_from(verts.len()).unwrap_or(0);
    let mut adj: Vec<Vec<(Key, usize)>> = vec![Vec::new(); verts.len()];
    for (n, edge) in edges.iter().enumerate() {
        if edge.kind == 0 || edge.kind == 1 {
            continue;
        }
        let tail = usize::try_from(edge.tail).unwrap_or(0);
        if let Some(list) = adj.get_mut(tail) {
            list.push((edge.key.clone(), n));
        }
    }
    for list in &mut adj {
        list.sort();
    }

    let path_between = |start: u32, goal: u32, allow: &[u8]| -> Option<Vec<usize>> {
        if start == goal {
            return Some(Vec::new());
        }
        let mut came: BTreeMap<u32, (u32, usize)> = BTreeMap::new();
        let mut queue = vec![start];
        let mut seen = BTreeSet::new();
        seen.insert(start);
        let mut qh = 0_usize;
        while qh < queue.len() {
            let Some(node) = queue.get(qh).copied() else {
                break;
            };
            qh += 1;
            let Some(nexts) = usize::try_from(node).ok().and_then(|i| adj.get(i)) else {
                continue;
            };
            for (_, edge_at) in nexts {
                let Some(edge) = edges.get(*edge_at) else {
                    continue;
                };
                if !allow.contains(&edge.kind) {
                    continue;
                }
                if seen.contains(&edge.head) {
                    continue;
                }
                seen.insert(edge.head);
                came.insert(edge.head, (node, *edge_at));
                if edge.head == goal {
                    let mut taken = Vec::new();
                    let mut cursor = goal;
                    while cursor != start {
                        let (prev, id) = came.get(&cursor)?;
                        taken.push(*id);
                        cursor = *prev;
                    }
                    taken.reverse();
                    return Some(taken);
                }
                queue.push(edge.head);
            }
        }
        None
    };

    let port_ok = |cells: &BTreeMap<Hash, joinn_dna::Cell>,
                   partner: Hash,
                   h_in: u32,
                   h_out: u32,
                   frame_in: FrameRef,
                   frame_out: FrameRef|
     -> bool {
        let Some(cell) = cells.get(&partner) else {
            return false;
        };
        let inn = cell
            .coding
            .contract
            .ports
            .iter()
            .find(|p| p.position == h_in);
        let out = cell
            .coding
            .contract
            .ports
            .iter()
            .find(|p| p.position == h_out);
        match (inn, out) {
            (Some(inn), Some(out)) => inn.frame == frame_out && out.frame == frame_in,
            _ => false,
        }
    };
    let law_on = |alias: &str,
                  idx: usize,
                  frame_in: FrameRef,
                  frame_out: FrameRef,
                  partner: Option<Hash>|
     -> Option<String> {
        let inst = insts.get(idx)?;
        let cell_hash = inst.cell?;
        let cells = cells_of.get(alias)?;
        let cell = cells.get(&cell_hash)?;
        for (name, law) in &cell.coding.laws {
            let Some((h, _so, _si, h_out, h_in)) = roundtrip(&law.formula) else {
                continue;
            };
            if let Some(want) = partner {
                if h != want {
                    continue;
                }
            }
            if port_ok(cells, h, h_in, h_out, frame_in, frame_out) {
                return Some(format!("{alias}.{} {}", inst.name, name.0));
            }
        }
        None
    };

    struct Loop {
        text: String,
        chain: Vec<(usize, i32)>,
        filled: Option<String>,
    }
    let mut loops_found: Vec<Loop> = Vec::new();
    let spine_wire = [3_u8, 4];
    let travel = [2_u8, 3, 4];

    for entry in &entries {
        for exit in &exits {
            let start = vid(&V::Port(
                entry.alias.clone(),
                entry.instance.clone(),
                entry.port,
            ));
            let goal = vid(&V::Port(
                exit.alias.clone(),
                exit.instance.clone(),
                exit.port,
            ));
            let Some(taken) = path_between(start, goal, &travel) else {
                continue;
            };
            let Some(entry_at) = edges.iter().position(|e| e.key == entry.key) else {
                continue;
            };
            let Some(exit_at) = edges.iter().position(|e| e.key == exit.key) else {
                continue;
            };
            let mut chain = vec![(entry_at, 1_i32)];
            let mut text = format!("outside →{}→ {}", entry.label, region_name(entry.region));
            for id in &taken {
                chain.push((*id, 1));
                if let Some(edge) = edges.get(*id) {
                    if edge.kind == 2 {
                        text.push_str(&edge.step);
                    }
                }
            }
            chain.push((exit_at, 1));
            text.push_str(&format!(" →{}→ outside", exit.label));
            let filled = if entry.frame == exit.frame {
                Some(format!("frame {}", entry.frame.id))
            } else {
                law_on(&entry.alias, entry.inst_idx, entry.frame, exit.frame, None)
                    .or_else(|| law_on(&exit.alias, exit.inst_idx, entry.frame, exit.frame, None))
            };
            let text = match &filled {
                Some(by) => format!("{text} by {by}"),
                None => text,
            };
            loops_found.push(Loop {
                text,
                chain,
                filled,
            });
        }
    }

    let nreg = region_name_of.len();
    let mut seen_r = vec![false; nreg];
    let mut parent_r: Vec<Option<(usize, bool)>> = vec![None; nreg];
    let mut tree_links = BTreeSet::new();
    for start in 0..nreg {
        if seen_r.get(start).copied().unwrap_or(true) {
            continue;
        }
        if let Some(slot) = seen_r.get_mut(start) {
            *slot = true;
        }
        let mut queue = vec![start];
        let mut qh = 0_usize;
        while qh < queue.len() {
            let Some(&node) = queue.get(qh) else {
                break;
            };
            qh += 1;
            for (li, link) in links.iter().enumerate() {
                if tree_links.contains(&li) {
                    continue;
                }
                let other = if link.tail_region == node {
                    Some((link.head_region, true))
                } else if link.head_region == node {
                    Some((link.tail_region, false))
                } else {
                    None
                };
                let Some((other, forward)) = other else {
                    continue;
                };
                if seen_r.get(other).copied().unwrap_or(true) {
                    continue;
                }
                if let Some(slot) = seen_r.get_mut(other) {
                    *slot = true;
                }
                parent_r[other] = Some((li, forward));
                tree_links.insert(li);
                queue.push(other);
            }
        }
    }
    for (li, link) in links.iter().enumerate() {
        if tree_links.contains(&li) {
            continue;
        }
        let climb = |start: usize, goal: usize| -> Option<Vec<(usize, bool)>> {
            let mut at = start;
            let mut climbed = Vec::new();
            let mut guard = 0_u32;
            while at != goal {
                let (pedge, forward) = parent_r.get(at).and_then(|p| *p)?;
                climbed.push((pedge, !forward));
                let plink = links.get(pedge)?;
                at = if forward {
                    plink.tail_region
                } else {
                    plink.head_region
                };
                guard = guard.saturating_add(1);
                if guard > 10000 {
                    return None;
                }
            }
            Some(climbed)
        };
        let walk = if let Some(climbed) = climb(link.head_region, link.tail_region) {
            let mut walk = vec![(li, true)];
            walk.extend(climbed);
            walk
        } else if let Some(climbed) = climb(link.tail_region, link.head_region) {
            let mut walk = vec![(li, false)];
            walk.extend(climbed);
            walk
        } else {
            continue;
        };
        if walk.len() < 2 {
            continue;
        }
        struct Trav {
            leave_alias: String,
            leave_inst: String,
            leave_port: u32,
            enter_alias: String,
            enter_inst: String,
            enter_port: u32,
            to_region: usize,
            from_region: usize,
            edge_at: usize,
            sign: i32,
        }
        let mut travs = Vec::new();
        for (id, forward) in &walk {
            let Some(link) = links.get(*id) else {
                continue;
            };
            let Some(edge_at) = edges.iter().position(|e| e.key == link.key) else {
                continue;
            };
            if *forward {
                travs.push(Trav {
                    from_region: link.tail_region,
                    leave_alias: link.tail_alias.clone(),
                    leave_inst: link.tail_inst.clone(),
                    leave_port: link.tail_port,
                    enter_alias: link.head_alias.clone(),
                    enter_inst: link.head_inst.clone(),
                    enter_port: link.head_port,
                    to_region: link.head_region,
                    edge_at,
                    sign: 1,
                });
            } else {
                travs.push(Trav {
                    from_region: link.head_region,
                    leave_alias: link.head_alias.clone(),
                    leave_inst: link.head_inst.clone(),
                    leave_port: link.head_port,
                    enter_alias: link.tail_alias.clone(),
                    enter_inst: link.tail_inst.clone(),
                    enter_port: link.tail_port,
                    to_region: link.tail_region,
                    edge_at,
                    sign: -1,
                });
            }
        }
        if travs.len() < 2 {
            continue;
        }
        let mut chain = Vec::new();
        let mut bad = false;
        for i in 0..travs.len() {
            let prev = if i == 0 { travs.len() - 1 } else { i - 1 };
            let Some(arrive) = travs.get(prev) else {
                bad = true;
                break;
            };
            let Some(depart) = travs.get(i) else {
                bad = true;
                break;
            };
            let enter = vid(&V::Port(
                arrive.enter_alias.clone(),
                arrive.enter_inst.clone(),
                arrive.enter_port,
            ));
            let leave = vid(&V::Port(
                depart.leave_alias.clone(),
                depart.leave_inst.clone(),
                depart.leave_port,
            ));
            let Some(inside) = path_between(enter, leave, &spine_wire) else {
                bad = true;
                break;
            };
            for id in inside {
                chain.push((id, 1));
            }
            chain.push((depart.edge_at, depart.sign));
        }
        if bad {
            continue;
        }
        let mut changes: Vec<(FrameRef, FrameRef, usize, String)> = Vec::new();
        let mut bad_frames = false;
        for i in 0..travs.len() {
            let prev = if i == 0 { travs.len() - 1 } else { i - 1 };
            let Some(arrive) = travs.get(prev) else {
                continue;
            };
            let Some(depart) = travs.get(i) else {
                continue;
            };
            let Some(enter_i) = find_inst(&arrive.enter_alias, &arrive.enter_inst) else {
                bad_frames = true;
                break;
            };
            let Some(leave_i) = find_inst(&depart.leave_alias, &depart.leave_inst) else {
                bad_frames = true;
                break;
            };
            let Some(enter_f) = port_frame(enter_i, arrive.enter_port) else {
                bad_frames = true;
                break;
            };
            let Some(leave_f) = port_frame(leave_i, depart.leave_port) else {
                bad_frames = true;
                break;
            };
            if enter_f != leave_f {
                changes.push((enter_f, leave_f, enter_i, arrive.enter_alias.clone()));
            }
        }
        if bad_frames {
            continue;
        }
        let mut text = region_name(travs[0].from_region);
        for trav in &travs {
            text.push_str(&format!(
                " →{}.{}@{}→ {}",
                trav.leave_alias,
                trav.leave_inst,
                trav.leave_port,
                region_name(trav.to_region)
            ));
        }
        let filled = if changes.is_empty() {
            let frame = travs.first().and_then(|t| {
                let idx = find_inst(&t.leave_alias, &t.leave_inst)?;
                port_frame(idx, t.leave_port)
            });
            frame.map(|f| format!("frame {}", f.id))
        } else if changes.len() == 2 {
            let (f0, g0, i0, a0) = &changes[0];
            let (f1, g1, i1, a1) = &changes[1];
            let cell_at = |idx: usize| insts.get(idx).and_then(|i| i.cell);
            law_on(a0, *i0, *f0, *g0, cell_at(*i1))
                .or_else(|| law_on(a1, *i1, *f1, *g1, cell_at(*i0)))
        } else if changes.len() > 2 {
            not_measured = not_measured.saturating_add(1);
            None
        } else {
            None
        };
        if changes.len() > 2 {
            continue;
        }
        let text = match &filled {
            Some(by) => format!("{text} by {by}"),
            None => text,
        };
        loops_found.push(Loop {
            text,
            chain,
            filled,
        });
    }

    let mut det_faces: Vec<Vec<(usize, i32)>> = Vec::new();
    for alias in &aliases {
        let owned: Vec<u32> = insts
            .iter()
            .filter(|i| i.alias == *alias)
            .flat_map(|i| {
                let mut ids = vec![vid(&V::Centre(i.alias.clone(), i.name.clone()))];
                for port in &i.ports {
                    ids.push(vid(&V::Port(
                        i.alias.clone(),
                        i.name.clone(),
                        port.position,
                    )));
                }
                ids
            })
            .collect();
        let owned_set: BTreeSet<u32> = owned.iter().copied().collect();
        let internal: Vec<usize> = edges
            .iter()
            .enumerate()
            .filter(|(_, e)| {
                (e.kind == 3 || e.kind == 4)
                    && owned_set.contains(&e.tail)
                    && owned_set.contains(&e.head)
            })
            .map(|(n, _)| n)
            .collect();
        let mut seen_v = BTreeSet::new();
        let mut parent_e: BTreeMap<u32, (usize, bool)> = BTreeMap::new();
        let mut tree_e = BTreeSet::new();
        let mut starts: Vec<u32> = owned_set.iter().copied().collect();
        starts.sort();
        for start in starts {
            if seen_v.contains(&start) {
                continue;
            }
            seen_v.insert(start);
            let mut queue = vec![start];
            let mut qh = 0_usize;
            while qh < queue.len() {
                let Some(&node) = queue.get(qh) else {
                    break;
                };
                qh += 1;
                for id in &internal {
                    if tree_e.contains(id) {
                        continue;
                    }
                    let Some(edge) = edges.get(*id) else {
                        continue;
                    };
                    let other = if edge.tail == node {
                        Some((edge.head, true))
                    } else if edge.head == node {
                        Some((edge.tail, false))
                    } else {
                        None
                    };
                    let Some((other, forward)) = other else {
                        continue;
                    };
                    if seen_v.contains(&other) {
                        continue;
                    }
                    seen_v.insert(other);
                    parent_e.insert(other, (*id, forward));
                    tree_e.insert(*id);
                    queue.push(other);
                }
            }
        }
        for id in &internal {
            if tree_e.contains(id) {
                continue;
            }
            let Some(edge) = edges.get(*id) else {
                continue;
            };
            type Up = (Vec<u32>, Vec<(usize, bool)>);
            let up = |start: u32| -> Option<Up> {
                let mut nodes = vec![start];
                let mut climbed = Vec::new();
                let mut at = start;
                let mut guard = 0_u32;
                while let Some((pedge, forward)) = parent_e.get(&at).copied() {
                    climbed.push((pedge, !forward));
                    let ped = edges.get(pedge)?;
                    at = if forward { ped.tail } else { ped.head };
                    nodes.push(at);
                    guard = guard.saturating_add(1);
                    if guard > 10000 {
                        return None;
                    }
                }
                Some((nodes, climbed))
            };
            let Some((nodes_h, edges_h)) = up(edge.head) else {
                continue;
            };
            let Some((nodes_t, edges_t)) = up(edge.tail) else {
                continue;
            };
            let Some(lca) = nodes_h.iter().find(|n| nodes_t.contains(n)).copied() else {
                continue;
            };
            let Some(k_h) = nodes_h.iter().position(|n| *n == lca) else {
                continue;
            };
            let Some(k_t) = nodes_t.iter().position(|n| *n == lca) else {
                continue;
            };
            let mut walk = vec![(*id, true)];
            walk.extend(edges_h.iter().take(k_h).copied());
            let down: Vec<(usize, bool)> = edges_t
                .iter()
                .take(k_t)
                .rev()
                .map(|(e, forward)| (*e, !forward))
                .collect();
            walk.extend(down);
            if walk.len() < 2 {
                continue;
            }
            let face: Vec<(usize, i32)> = walk
                .iter()
                .map(|(e, forward)| (*e, if *forward { 1 } else { -1 }))
                .collect();
            det_faces.push(face);
        }
    }

    let ecount = u32::try_from(edges.len()).unwrap_or(0);
    let eliminate = |mut a: Vec<Vec<i128>>| -> Option<u32> {
        if a.is_empty() {
            return Some(0);
        }
        let n = a.first().map(Vec::len).unwrap_or(0);
        if n == 0 {
            return Some(0);
        }
        let m = a.len();
        let mut prev: i128 = 1;
        let mut rank = 0_u32;
        let mut row_i = 0_usize;
        for col in 0..n {
            let mut pivot_at = None;
            for r in row_i..m {
                if a.get(r)
                    .and_then(|line| line.get(col))
                    .copied()
                    .unwrap_or(0)
                    != 0
                {
                    pivot_at = Some(r);
                    break;
                }
            }
            let Some(pr) = pivot_at else {
                continue;
            };
            if pr != row_i {
                a.swap(pr, row_i);
            }
            let pivot = a
                .get(row_i)
                .and_then(|line| line.get(col))
                .copied()
                .unwrap_or(0);
            if pivot == 0 {
                continue;
            }
            for r in (row_i + 1)..m {
                let down = a
                    .get(r)
                    .and_then(|line| line.get(col))
                    .copied()
                    .unwrap_or(0);
                for c in (col + 1)..n {
                    let ij = a.get(r).and_then(|line| line.get(c)).copied().unwrap_or(0);
                    let pj = a
                        .get(row_i)
                        .and_then(|line| line.get(c))
                        .copied()
                        .unwrap_or(0);
                    let left = pivot.checked_mul(ij)?;
                    let right = down.checked_mul(pj)?;
                    let num = left.checked_sub(right)?;
                    if prev == 0 || num.checked_rem(prev) != Some(0) {
                        return None;
                    }
                    let next = num.checked_div(prev)?;
                    if let Some(slot) = a.get_mut(r).and_then(|line| line.get_mut(c)) {
                        *slot = next;
                    }
                }
                if let Some(slot) = a.get_mut(r).and_then(|line| line.get_mut(col)) {
                    *slot = 0;
                }
            }
            prev = pivot;
            rank = rank.saturating_add(1);
            row_i += 1;
            if row_i == m {
                break;
            }
        }
        Some(rank)
    };
    let mut undirected: Vec<Vec<(u32, usize)>> = vec![Vec::new(); verts.len()];
    for (n, edge) in edges.iter().enumerate() {
        let Ok(tail) = usize::try_from(edge.tail) else {
            return crate::refuse(
                "edge tail is outside the vertex list; acceptance is a vertex of this complex",
            );
        };
        let Ok(head) = usize::try_from(edge.head) else {
            return crate::refuse(
                "edge head is outside the vertex list; acceptance is a vertex of this complex",
            );
        };
        if tail == head {
            continue;
        }
        if let Some(list) = undirected.get_mut(tail) {
            list.push((edge.head, n));
        }
        if let Some(list) = undirected.get_mut(head) {
            list.push((edge.tail, n));
        }
    }
    let mut seen_v = vec![false; verts.len()];
    let mut tree_edge = vec![false; edges.len()];
    let mut b0 = 0_u32;
    for start in 0..verts.len() {
        if seen_v.get(start).copied().unwrap_or(true) {
            continue;
        }
        if let Some(slot) = seen_v.get_mut(start) {
            *slot = true;
        }
        b0 = b0.saturating_add(1);
        let Ok(start_id) = u32::try_from(start) else {
            continue;
        };
        let mut queue = vec![start_id];
        let mut qh = 0_usize;
        while qh < queue.len() {
            let Some(node) = queue.get(qh).copied() else {
                break;
            };
            qh += 1;
            let Ok(node_at) = usize::try_from(node) else {
                continue;
            };
            let Some(nexts) = undirected.get(node_at) else {
                continue;
            };
            for (other, eid) in nexts {
                let Ok(other_at) = usize::try_from(*other) else {
                    continue;
                };
                if seen_v.get(other_at).copied().unwrap_or(true) {
                    continue;
                }
                if let Some(slot) = seen_v.get_mut(other_at) {
                    *slot = true;
                }
                if let Some(slot) = tree_edge.get_mut(*eid) {
                    *slot = true;
                }
                queue.push(*other);
            }
        }
    }
    let mut chord_of: Vec<Option<usize>> = vec![None; edges.len()];
    let mut n_chords = 0_usize;
    for (n, is_tree) in tree_edge.iter().enumerate() {
        if *is_tree {
            continue;
        }
        if let Some(slot) = chord_of.get_mut(n) {
            *slot = Some(n_chords);
        }
        n_chords += 1;
    }
    let face_rank = |cols: &[Vec<(usize, i32)>]| -> Option<u32> {
        if cols.is_empty() || n_chords == 0 {
            return Some(0);
        }
        let mut a: Vec<Vec<i128>> = Vec::new();
        for col in cols {
            let mut row = vec![0_i128; n_chords];
            for (at, sign) in col {
                let Some(chord) = chord_of.get(*at).copied().flatten() else {
                    continue;
                };
                let slot = row.get_mut(chord)?;
                *slot = slot.checked_add(i128::from(*sign))?;
            }
            a.push(row);
        }
        eliminate(a)
    };

    let mut base = det_faces;
    let rule_filled: Vec<Vec<(usize, i32)>> = loops_found
        .iter()
        .filter(|l| l.filled.is_some())
        .map(|l| l.chain.clone())
        .collect();
    base.extend(rule_filled);
    let refuse_rank = |blocks: u32| {
        crate::refuse(format!(
            "complex too large for exact elimination at {blocks} blocks; acceptance is a complex whose elimination fits i128"
        ))
    };
    let blocks = vcount
        .saturating_add(ecount)
        .saturating_add(u32::try_from(base.len()).unwrap_or(0));
    let r1 = match vcount.checked_sub(b0) {
        Some(r) => r,
        None => return refuse_rank(blocks),
    };
    let r2 = match face_rank(&base) {
        Some(r) => r,
        None => return refuse_rank(blocks),
    };
    let Some(b0) = vcount.checked_sub(r1) else {
        return refuse_rank(blocks);
    };
    let Some(b1) = ecount.checked_sub(r1).and_then(|n| n.checked_sub(r2)) else {
        return refuse_rank(blocks);
    };
    let Some(b2) = u32::try_from(base.len()).unwrap_or(0).checked_sub(r2) else {
        return refuse_rank(blocks);
    };

    let mut kept = base.clone();
    let mut open = Vec::new();
    let mut filled_lines = Vec::new();
    for loop_ in &loops_found {
        if let Some(by) = &loop_.filled {
            let body = loop_
                .text
                .strip_suffix(&format!(" by {by}"))
                .unwrap_or(&loop_.text);
            filled_lines.push(format!("{body} by {by}"));
            continue;
        }
        let mut trial = kept.clone();
        trial.push(loop_.chain.clone());
        let grows = match (face_rank(&kept), face_rank(&trial)) {
            (Some(before), Some(after)) => after > before,
            _ => return refuse_rank(blocks),
        };
        if grows {
            kept.push(loop_.chain.clone());
            open.push(loop_.text.clone());
        }
    }

    let mut island_p: Vec<usize> = (0..nreg).collect();
    let find_i = |p: &mut [usize], mut x: usize| -> usize {
        while p.get(x).is_some_and(|n| *n != x) {
            let n = p[x];
            let g = p.get(n).copied().unwrap_or(n);
            p[x] = g;
            x = n;
        }
        x
    };
    for link in &links {
        let a = find_i(&mut island_p, link.tail_region);
        let b = find_i(&mut island_p, link.head_region);
        if a != b {
            island_p[b] = a;
        }
    }
    let mut roots = BTreeSet::new();
    for i in 0..nreg {
        roots.insert(find_i(&mut island_p, i));
    }
    let islands = u32::try_from(roots.len()).unwrap_or(0);
    let f = u32::try_from(base.len()).unwrap_or(0);
    let loops = ecount.saturating_add(b0).saturating_sub(vcount);

    Verdict::Ok(AssayReport {
        headline,
        regions: region_lines,
        islands,
        loops,
        filled: filled_lines,
        open,
        not_measured,
        b0,
        b1,
        b2,
        v: vcount,
        e: ecount,
        f,
    })
}
