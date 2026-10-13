//! What `cargo xtask codex` prints.

use joinn_codex::{
    AminoAcid, CODEX, CODEX_1, Codon, Meaning, ORGANELLES, READER_VERSION, check_table, codon_of,
    lookup,
};
use joinn_frame::Verdict;

/// The table with each codon's bits and parity, the reserved codons, the
/// parity of START and STOP, the organelle list, the reader's version and the
/// codex; an error when any table check fails.
pub(crate) fn codex_text() -> Result<String, String> {
    let named = match check_table(&CODEX_1) {
        Verdict::Ok(n) => n,
        Verdict::Refused(r) => return Err(r.reason),
    };
    let cell = |c: Codon| {
        let means = lookup(c).map_or("reserved", Meaning::name);
        format!(
            "{} {} {:<5} {:<6}",
            c.text(),
            c.bit_text(),
            c.parity().name(),
            means
        )
    };
    let mut out = format!("codex {CODEX}: codon bits parity means | partner bits parity means\n");
    for row in &CODEX_1 {
        let left = Codon::from_letters(row.codon);
        out.push_str(&format!(
            "{} | {}\n",
            cell(left),
            cell(left.partner()).trim_end()
        ));
    }
    let reserved: Vec<Codon> = Codon::all()
        .into_iter()
        .filter(|c| lookup(*c).is_none())
        .collect();
    out.push_str(&format!("reserved ({}):\n", reserved.len()));
    for chunk in reserved.chunks(10) {
        let texts: Vec<String> = chunk.iter().map(|c| c.text()).collect();
        out.push_str(&format!("  {}\n", texts.join(" ")));
    }
    let (Some(start), Some(stop)) = (codon_of(Meaning::Start), codon_of(Meaning::Stop)) else {
        return Err(
            "codex: START or STOP has no codon; acceptance is codex 1's AAT and ATT".into(),
        );
    };
    let flips = Codon::all()
        .into_iter()
        .filter(|c| c.partner().parity() != c.parity())
        .count();
    out.push_str(&format!(
        "parity: {} (START) {} {}; {} (STOP) {} {}; a codon and its partner differ in parity for {flips} of 64\n",
        start.text(),
        start.bit_text(),
        start.parity().name(),
        stop.text(),
        stop.bit_text(),
        stop.parity().name()
    ));
    for o in ORGANELLES {
        let users: Vec<&str> = AminoAcid::ALL
            .iter()
            .filter(|a| a.organelles().contains(&o))
            .map(|a| a.name())
            .collect();
        out.push_str(&format!("organelle {}: {}\n", o.name(), users.join(", ")));
    }
    out.push_str(&format!("{READER_VERSION}\ncodex {CODEX}\n"));
    out.push_str(&format!(
        "codex {CODEX}: {named} named, {} reserved; every partner names the opposite\n",
        reserved.len()
    ));
    Ok(out)
}
