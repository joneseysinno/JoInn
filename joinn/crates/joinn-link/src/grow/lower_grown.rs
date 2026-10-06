//! The body the engine runs for a grown state, derived.

use joinn_dna::{Body, BodyCoding, Cell, Direction, GenomeEntry, GenomeTarget, Wire};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use std::collections::BTreeMap;

use super::Grown;

/// Grown instances `numbers.0 … numbers.<n−1>` hold `stdin` by being grown.
/// With two or more, the response's binary cell folds them from the left in
/// growth order: stage `k` takes stage `k−1` (or `numbers.0`) on its first
/// in-port and `numbers.<k>` on its second. The last stage carries the
/// response's name; earlier ones are `count.<k>`. With one or none there is
/// no stage. The result is never written, described or drawn.
pub fn lower_grown(
    grown: &Grown,
    cells: &BTreeMap<Hash, Cell>,
    frames: &FrameRegistry,
) -> Verdict<Body> {
    let coding = &grown.contact.coding;
    let (Some(grows), Some(force)) = (coding.grows.as_ref(), grown.force()) else {
        return crate::refuse("grow: a grown state needs a growing body and a force");
    };
    if !frames.contains(&force.frame) {
        return crate::refuse(format!(
            "grow: force {} is in {}, which is no frame here; acceptance is a registered frame",
            force.name, force.frame
        ));
    }
    let port = |hash: &Hash, direction: Direction| -> Option<Vec<u32>> {
        let cell = cells.get(hash)?;
        let mut out: Vec<u32> = cell
            .coding
            .contract
            .ports
            .iter()
            .filter(|p| p.direction == direction)
            .map(|p| p.position)
            .collect();
        out.sort_unstable();
        Some(out)
    };
    let (Some(grown_out), Some(response_in), Some(response_out)) = (
        port(&grows.cell, Direction::Out),
        port(&force.response, Direction::In),
        port(&force.response, Direction::Out),
    ) else {
        return crate::refuse("grow: the grown cell and the response cell must be supplied");
    };
    let ([out], [left, right], [result]) = (
        grown_out.as_slice(),
        response_in.as_slice(),
        response_out.as_slice(),
    ) else {
        return crate::refuse(format!(
            "grow: force {} folds a binary response over one out-port; acceptance is a cell with two in-ports and one out-port",
            force.name
        ));
    };
    let n = grown.inputs.len();
    let instances: Vec<String> = (0..n).map(|i| grown.instance(i)).collect();
    let stage = |k: usize| {
        if k + 1 == n {
            force.name.clone()
        } else {
            format!("{}.{k}", force.name)
        }
    };
    let mut genome: Vec<GenomeEntry> = coding
        .genome
        .iter()
        .map(|g| GenomeEntry {
            target: GenomeTarget::Cell(g.cell),
            instances: g.instances.clone(),
        })
        .collect();
    let mut grants = coding.grants.clone();
    if n > 0 {
        genome.push(GenomeEntry {
            target: GenomeTarget::Cell(grows.cell),
            instances: instances.clone(),
        });
        grants
            .entry("stdin".into())
            .or_default()
            .extend(instances.iter().cloned());
    }
    let mut stages = Vec::new();
    let mut wires = Vec::new();
    for k in 1..n {
        let (src_instance, src_port) = if k == 1 {
            (instances[0].clone(), *out)
        } else {
            (stage(k - 1), *result)
        };
        wires.push(Wire {
            src_instance,
            src_port,
            dst_instance: stage(k),
            dst_port: *left,
        });
        wires.push(Wire {
            src_instance: instances[k].clone(),
            src_port: *out,
            dst_instance: stage(k),
            dst_port: *right,
        });
        stages.push(stage(k));
    }
    if !stages.is_empty() {
        genome.push(GenomeEntry {
            target: GenomeTarget::Cell(force.response),
            instances: stages,
        });
    }
    Verdict::Ok(Body {
        coding: BodyCoding {
            codex: coding.codex,
            declarations: Vec::new(),
            genome,
            grants,
            reads: coding.reads.clone(),
            wires,
            budget_steps: coding.budget_steps,
            lineage: coding.lineage,
        },
        regulatory: grown.contact.regulatory.clone(),
    })
}
