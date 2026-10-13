//! Gate 8 table (P8-11: the reader holds the one table, every order holds
//! the one before it, the pool has no cycles). Witness files are the
//! controls.

use super::mutate::Mutation;
use super::subject::Subject;
use super::{g8_floors, g8_floors_control, g8_pool, g8_pool_control, g8_reader, g8_reader_control};
use joinn_gate::GateItem;

/// `twelve.strand` read by the codex with `AAA` and `TTT` swapped.
const SWAPPED_READING: &str =
    "protein at letter 0: split pair build case refer return sense show keep recall grant revoke";

/// `twelve.chromosome`'s floors with T5 filtering at level ≥ 0.
const FLAT_FLOORS: &str = concat!(
    "chromosome edf1529a6fc21715846486999b3252f1336462dc70b16f9edeb70eb96e617bca order 2: ",
    "14 codons on 15 segments, 1 silent, 2 gene references\n",
    "floor 0 (level ≥ 0): 14 codons: 0 AAT START, 1 AAA pair, 2 TTT split, 3 CAC build, ",
    "4 GTG case, 5 AAC refer, 6 GTT return, 7 CAA sense, 8 TTG show, 9 GAA keep, ",
    "10 TTC recall, 11 AAG grant, 12 CTT revoke, 13 ATT STOP\n",
    "floor 1 (level ≥ 1): 14 codons: 0 AAT START, 1 AAA pair, 2 TTT split, 3 CAC build, ",
    "4 GTG case, 5 AAC refer, 6 GTT return, 7 CAA sense, 8 TTG show, 9 GAA keep, ",
    "10 TTC recall, 11 AAG grant, 12 CTT revoke, 13 ATT STOP",
);

/// The cycle witness as a pool without the cycle check holds it: both
/// chromosomes, the reference that closes the cycle left out.
const ONE_WAY: &str = concat!(
    "codex 1\n",
    "AAT AAA TTT ATT\n",
    "node 1 gene 9be547e78b9ccb3226a19820365488f2a3264113ad439cf2c74d782e5073151b\n",
    "\n",
    "codex 1\n",
    "AAT CAC GTG ATT",
);

const ITEMS: &[GateItem<Subject, Mutation>] = &[
    GateItem {
        name: "The reader holds the one table",
        check: g8_reader,
        check_name: "g8_reader",
        control: g8_reader_control,
        control_name: "g8_reader_control",
        control_artifact: "corpus/phase8/strands/twelve.proteins.txt",
        opposes: Mutation::Replace(SWAPPED_READING),
    },
    GateItem {
        name: "Every order holds the one before it",
        check: g8_floors,
        check_name: "g8_floors",
        control: g8_floors_control,
        control_name: "g8_floors_control",
        control_artifact: "corpus/phase8/tower/twelve.floors.txt",
        opposes: Mutation::Replace(FLAT_FLOORS),
    },
    GateItem {
        name: "The pool has no cycles",
        check: g8_pool,
        check_name: "g8_pool",
        control: g8_pool_control,
        control_name: "g8_pool_control",
        control_artifact: "corpus/phase8/pool/cycle.chromosome",
        opposes: Mutation::Replace(ONE_WAY),
    },
];

pub(crate) fn gate_eight_items() -> &'static [GateItem<Subject, Mutation>] {
    ITEMS
}

#[cfg(test)]
mod tests {
    use joinn_codex::{print_proteins, translate};
    use joinn_frame::Verdict;
    use joinn_strand::{Letter, Strand, parse_strand_file};
    use joinn_tower::{level_filter, parse_chromosomes};

    use super::{FLAT_FLOORS, ONE_WAY, SWAPPED_READING};

    fn corpus(name: &str) -> String {
        let root = crate::fns::workspace_root().unwrap_or_else(|e| panic!("{e}"));
        std::fs::read_to_string(crate::fns::corpus_artifact(&root, name))
            .unwrap_or_else(|e| panic!("{name}: {e}"))
            .replace("\r\n", "\n")
    }

    #[test]
    fn the_swapped_reading_is_twelve_read_by_the_swapped_codex() {
        let Verdict::Ok(file) = parse_strand_file(&corpus("phase8/strands/twelve.strand")) else {
            panic!("twelve.strand");
        };
        let mut letters = file.strand.letters().to_vec();
        for codon in letters.chunks_exact_mut(3) {
            match codon {
                [Letter::A, Letter::A, Letter::A] => codon.fill(Letter::T),
                [Letter::T, Letter::T, Letter::T] => codon.fill(Letter::A),
                _ => {}
            }
        }
        let Verdict::Ok(proteins) = translate(&Strand::new(letters)) else {
            panic!("the swapped strand reads");
        };
        assert_eq!(print_proteins(&proteins).trim(), SWAPPED_READING);
    }

    #[test]
    fn the_flat_floors_are_twelve_s_floors_filtered_at_level_0() {
        let Verdict::Ok(all) = parse_chromosomes(&corpus("phase8/tower/twelve.chromosome")) else {
            panic!("twelve.chromosome");
        };
        let Some(c) = all.first() else {
            panic!("one chromosome");
        };
        let real = crate::fns::dna::floors_text(c);
        let codons = c.codons();
        let every = level_filter(&codons, 0);
        assert_eq!(every.len(), codons.len());
        let mut lines = real.lines();
        let mut want = vec![lines.next().unwrap_or_default().to_owned()];
        let floor_0 = lines.next().unwrap_or_default().to_owned();
        let kept = floor_0
            .split_once("): ")
            .map(|(_, k)| k)
            .unwrap_or_default();
        for f in 0..c.order() {
            want.push(format!("floor {f} (level ≥ {f}): {kept}"));
        }
        assert_eq!(want.join("\n"), FLAT_FLOORS);
        assert_ne!(real.trim(), FLAT_FLOORS);
    }

    #[test]
    fn one_way_is_the_cycle_witness_without_its_closing_reference() {
        let witness = corpus("phase8/pool/cycle.chromosome");
        let lines: Vec<&str> = witness.trim().lines().collect();
        let Some((closing, rest)) = lines.split_last() else {
            panic!("cycle witness");
        };
        assert!(closing.starts_with("node 2 gene 8b126fce9ab6"), "{closing}");
        assert_eq!(rest.join("\n"), ONE_WAY);
    }
}
