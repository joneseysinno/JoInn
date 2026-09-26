//! Regions, boundary ports, and the three filling rules.

use joinn_assay::{BlockId, Chain, Complex};
use joinn_dna::{Direction, GenomeTarget, PortDecl};
use joinn_frame::{FrameRef, Hash, Verdict};
use joinn_prim::prim_ports;
use std::collections::{BTreeMap, BTreeSet};

use super::roundtrip::roundtrip;
use super::{AssayReport, RegionPieces};
use crate::{Bound, Mark, Universe};

/// Fast derivation. A loop is reported, never a refusal by itself.
pub fn assay(universe: &Universe, bound: &Bound) -> Verdict<AssayReport> {
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

    struct Inst {
        alias: String,
        name: String,
        genome_pos: u32,
        cell: Option<Hash>,
        ports: Vec<PortDecl>,
    }
    struct Reg {
        name: String,
    }
    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct EKey {
        kind: u8,
        hash: Hash,
        instance: String,
        port: u32,
        alias: String,
    }
    struct Edge {
        key: EKey,
        tail: u32,
        head: u32,
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
    let unite = |parent: &mut [usize], a: usize, b: usize| {
        let ra = find(parent, a);
        let rb = find(parent, b);
        if ra != rb {
            parent[rb] = ra;
        }
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
                unite(&mut parent, src, dst);
            }
        }
    }
    for i in 0..parent.len() {
        let root = find(&mut parent, i);
        parent[i] = root;
    }

    let mut regions: Vec<Reg> = Vec::new();
    let mut region_of: Vec<usize> = vec![0; insts.len()];
    let mut aliases: Vec<String> = by_alias.keys().cloned().collect();
    aliases.sort();
    let mut region_lines = Vec::new();
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
            let id = regions.len();
            for idx in members {
                if let Some(slot) = region_of.get_mut(*idx) {
                    *slot = id;
                }
            }
            regions.push(Reg { name });
            printed.push(names.clone());
        }
        region_lines.push(RegionPieces {
            alias: alias.clone(),
            pieces: printed,
        });
    }

    let region_name =
        |idx: usize| -> String { regions.get(idx).map(|r| r.name.clone()).unwrap_or_default() };
    let outside = 0_u32;
    let region_vertex = |idx: usize| -> u32 { 1 + u32::try_from(idx).unwrap_or(0) };

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
        key: EKey,
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
            key: EKey {
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
        key: EKey,
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
    for (alias, (hash, _idxs)) in &by_alias {
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
                key: EKey {
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
    for door in &entries {
        edges.push(Edge {
            key: door.key.clone(),
            tail: outside,
            head: region_vertex(door.region),
        });
    }
    for door in &exits {
        edges.push(Edge {
            key: door.key.clone(),
            tail: region_vertex(door.region),
            head: outside,
        });
    }
    for link in &links {
        edges.push(Edge {
            key: link.key.clone(),
            tail: region_vertex(link.tail_region),
            head: region_vertex(link.head_region),
        });
    }
    edges.sort_by(|a, b| a.key.cmp(&b.key));

    type Node = (String, String, u32);
    type Adj = BTreeMap<Node, Vec<(EKey, Node, Option<usize>)>>;
    let mut adj: Adj = BTreeMap::new();
    let push_adj = |adj: &mut Adj, from: Node, key: EKey, to: Node, link_at: Option<usize>| {
        adj.entry(from).or_default().push((key, to, link_at));
    };
    for inst in &insts {
        let ins: Vec<u32> = inst
            .ports
            .iter()
            .filter(|p| p.direction == Direction::In)
            .map(|p| p.position)
            .collect();
        let outs: Vec<u32> = inst
            .ports
            .iter()
            .filter(|p| p.direction == Direction::Out)
            .map(|p| p.position)
            .collect();
        for inn in &ins {
            for out in &outs {
                let from = (inst.alias.clone(), inst.name.clone(), *inn);
                let to = (inst.alias.clone(), inst.name.clone(), *out);
                let Some((hash, _)) = by_alias.get(&inst.alias) else {
                    continue;
                };
                push_adj(
                    &mut adj,
                    from,
                    EKey {
                        kind: 3,
                        hash: *hash,
                        instance: inst.name.clone(),
                        port: *inn,
                        alias: format!("{out}"),
                    },
                    to,
                    None,
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
            push_adj(
                &mut adj,
                (alias.clone(), wire.src_instance.clone(), wire.src_port),
                EKey {
                    kind: 4,
                    hash: *hash,
                    instance: wire.src_instance.clone(),
                    port: wire.src_port,
                    alias: wire.dst_instance.clone(),
                },
                (alias.clone(), wire.dst_instance.clone(), wire.dst_port),
                None,
            );
        }
    }
    for (n, link) in links.iter().enumerate() {
        push_adj(
            &mut adj,
            (
                link.tail_alias.clone(),
                link.tail_inst.clone(),
                link.tail_port,
            ),
            link.key.clone(),
            (
                link.head_alias.clone(),
                link.head_inst.clone(),
                link.head_port,
            ),
            Some(n),
        );
    }
    for list in adj.values_mut() {
        list.sort();
    }

    let canonical_links = |start: &Node, goal: &Node| -> Option<Vec<usize>> {
        if start == goal {
            return Some(Vec::new());
        }
        let mut parent: BTreeMap<Node, (Node, Option<usize>)> = BTreeMap::new();
        let mut queue = vec![start.clone()];
        let mut seen = BTreeSet::new();
        seen.insert(start.clone());
        let mut qh = 0_usize;
        while qh < queue.len() {
            let Some(node) = queue.get(qh).cloned() else {
                break;
            };
            qh += 1;
            let Some(nexts) = adj.get(&node) else {
                continue;
            };
            for (_, to, link_at) in nexts {
                if seen.contains(to) {
                    continue;
                }
                seen.insert(to.clone());
                parent.insert(to.clone(), (node.clone(), *link_at));
                if to == goal {
                    let mut links_taken = Vec::new();
                    let mut cursor = goal.clone();
                    while cursor != *start {
                        let (prev, link_at) = parent.get(&cursor)?;
                        if let Some(id) = link_at {
                            links_taken.push(*id);
                        }
                        cursor = prev.clone();
                    }
                    links_taken.reverse();
                    return Some(links_taken);
                }
                queue.push(to.clone());
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
            let Some((h, _self_out, _self_in, h_out, h_in)) = roundtrip(&law.formula) else {
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

    for entry in &entries {
        for exit in &exits {
            let start = (entry.alias.clone(), entry.instance.clone(), entry.port);
            let goal = (exit.alias.clone(), exit.instance.clone(), exit.port);
            let Some(taken) = canonical_links(&start, &goal) else {
                continue;
            };
            let mut chain = Vec::new();
            let entry_at = edges.iter().position(|e| e.key == entry.key);
            let exit_at = edges.iter().position(|e| e.key == exit.key);
            let (Some(entry_at), Some(exit_at)) = (entry_at, exit_at) else {
                continue;
            };
            chain.push((entry_at, 1));
            let mut text = format!("outside →{}→ {}", entry.label, region_name(entry.region));
            for id in &taken {
                let Some(link) = links.get(*id) else {
                    continue;
                };
                let Some(at) = edges.iter().position(|e| e.key == link.key) else {
                    continue;
                };
                chain.push((at, 1));
                text.push_str(&format!(
                    " →{}.{}@{}→ {}",
                    link.tail_alias,
                    link.tail_inst,
                    link.tail_port,
                    region_name(link.head_region)
                ));
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

    // Rule 3: cycles of regions joined by links, outside removed.
    let nreg = regions.len();
    let mut seen_r = vec![false; nreg];
    let mut parent_r: Vec<Option<(usize, bool)>> = vec![None; nreg];
    let mut tree_links = BTreeSet::new();
    let order: Vec<usize> = (0..nreg).collect();
    for start in order {
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
            from_region: usize,
            leave_alias: String,
            leave_inst: String,
            leave_port: u32,
            enter_alias: String,
            enter_inst: String,
            enter_port: u32,
            to_region: usize,
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
        let chain: Vec<(usize, i32)> = travs.iter().map(|t| (t.edge_at, t.sign)).collect();
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
        if bad_frames {
            continue;
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
            let y = cell_at(*i1);
            let x = cell_at(*i0);
            law_on(a0, *i0, *f0, *g0, y).or_else(|| law_on(a1, *i1, *f1, *g1, x))
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

    let v = 1 + u32::try_from(regions.len()).unwrap_or(0);
    let e = u32::try_from(edges.len()).unwrap_or(0);
    let betti = |faces: &[Vec<(usize, i32)>]| -> Verdict<(u32, u32, u32)> {
        let mut dimension = BTreeMap::new();
        let mut boundaries = BTreeMap::new();
        for id in 0..v {
            dimension.insert(BlockId(id), 0);
            boundaries.insert(BlockId(id), Chain::from_coeffs([]));
        }
        for (n, edge) in edges.iter().enumerate() {
            let Ok(id) = u32::try_from(n) else {
                continue;
            };
            let block = BlockId(v + id);
            dimension.insert(block, 1);
            boundaries.insert(
                block,
                Chain::from_coeffs([(BlockId(edge.head), 1), (BlockId(edge.tail), -1)]),
            );
        }
        for (n, face) in faces.iter().enumerate() {
            let Ok(id) = u32::try_from(n) else {
                continue;
            };
            let block = BlockId(v + e + id);
            dimension.insert(block, 2);
            let mut coeff = BTreeMap::new();
            for (edge_at, sign) in face {
                let Ok(eid) = u32::try_from(*edge_at) else {
                    continue;
                };
                let sum = coeff.get(&BlockId(v + eid)).copied().unwrap_or(0) + sign;
                if sum == 0 {
                    coeff.remove(&BlockId(v + eid));
                } else {
                    coeff.insert(BlockId(v + eid), sum);
                }
            }
            boundaries.insert(block, Chain::from_coeffs(coeff));
        }
        let complex = Complex::from_parts(dimension, boundaries);
        match complex.homology() {
            Verdict::Ok(h) => Verdict::Ok((h.b0, h.b1, h.b2)),
            Verdict::Refused(r) => Verdict::Refused(r),
        }
    };

    let filled_faces: Vec<Vec<(usize, i32)>> = loops_found
        .iter()
        .filter(|l| l.filled.is_some())
        .map(|l| l.chain.clone())
        .collect();
    let (b0, b1, b2) = match betti(&filled_faces) {
        Verdict::Ok(t) => t,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    let mut kept = filled_faces.clone();
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
        let before = match betti(&kept) {
            Verdict::Ok((_, b, _)) => b,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let mut trial = kept.clone();
        trial.push(loop_.chain.clone());
        let after = match betti(&trial) {
            Verdict::Ok((_, b, _)) => b,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        if after < before {
            kept.push(loop_.chain.clone());
            open.push(loop_.text.clone());
        }
    }
    let loops = e + b0 - v;
    let islands = {
        let mut p: Vec<usize> = (0..regions.len()).collect();
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
            let a = find_i(&mut p, link.tail_region);
            let b = find_i(&mut p, link.head_region);
            if a != b {
                p[b] = a;
            }
        }
        let mut roots = BTreeSet::new();
        for i in 0..regions.len() {
            roots.insert(find_i(&mut p, i));
        }
        u32::try_from(roots.len()).unwrap_or(0)
    };

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
        v,
        e,
        f: u32::try_from(filled_faces.len()).unwrap_or(0),
    })
}
