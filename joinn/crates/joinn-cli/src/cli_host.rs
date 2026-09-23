//! CLI host: stdin lines in, template text out.

mod new;

use joinn_dna::{Body, Cell};
use joinn_frame::{Hash, Term, Verdict};
use joinn_host::{Address, Description, Host, Intent, Signals, check_intent};
use std::collections::BTreeMap;

/// CLI host: stdin lines in, template text out.
pub(crate) struct CliHost {
    pub(crate) out: String,
    pub(crate) body: Body,
    pub(crate) cells: BTreeMap<Hash, Cell>,
    pub(crate) pending: Option<Address>,
    signals: Signals,
}

impl Host for CliHost {
    type Raw = String;

    fn present(&mut self, d: &Description) -> Verdict<()> {
        if d.role == joinn_host::Role::Refusal {
            self.out.push_str("   ");
            self.out.push_str(&d.label);
            self.out.push('\n');
            return Verdict::Ok(());
        }
        if let Some(line) = crate::present::fill_present(&self.body, d) {
            self.out.push_str(&line);
            self.out.push('\n');
        }
        Verdict::Ok(())
    }

    fn intend(&mut self, raw: Self::Raw) -> Verdict<Option<Intent>> {
        let Some(address) = self.pending.clone() else {
            return Verdict::Ok(None);
        };
        let intent = Intent {
            address,
            term: Term::text(raw),
        };
        match check_intent(&self.body, &self.cells, &intent) {
            Verdict::Ok(()) => Verdict::Ok(Some(intent)),
            Verdict::Refused(r) => Verdict::Refused(r),
        }
    }

    fn signals(&self) -> &Signals {
        &self.signals
    }
}
