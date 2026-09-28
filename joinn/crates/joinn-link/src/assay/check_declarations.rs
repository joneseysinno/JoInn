//! Declarations refuse at admission. The assay they read never refuses.

use joinn_dna::Assertion;
use joinn_frame::Verdict;

use super::assay_run::assay;
use crate::{Bound, Universe};

/// Refuse naming each declared sentence the assay's report does not satisfy.
pub(crate) fn check_declarations(
    declarations: &[Assertion],
    universe: &Universe,
    bound: &Bound,
) -> Verdict<()> {
    if declarations.is_empty() {
        return Verdict::Ok(());
    }
    let report = match assay(universe, bound) {
        Verdict::Ok(report) => report,
        Verdict::Refused(r) => {
            return crate::refuse(format!(
                "declaration {} could not be assayed: {}",
                Assertion::H1Zero.sentence(),
                r.reason
            ));
        }
    };
    for declaration in declarations {
        match declaration {
            Assertion::H1Zero => {
                if report.b1 != 0 {
                    let open: Vec<String> =
                        report.open.iter().map(|l| format!("open: {l}")).collect();
                    return crate::refuse(format!(
                        "declaration {} does not hold: H₁: {}; {}; acceptance is H₁: 0, every loop filled by a frame or a law",
                        declaration.sentence(),
                        report.b1,
                        open.join("; ")
                    ));
                }
            }
        }
    }
    Verdict::Ok(())
}
