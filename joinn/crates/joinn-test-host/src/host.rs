//! Headless host: captures, declares no signals.

mod new;

use joinn_dna::{Body, Cell};
use joinn_frame::{Hash, Verdict};
use joinn_host::{Description, Host, Intent, Signals, check_intent};
use std::collections::BTreeMap;

use crate::raw_event::RawEvent;

/// Headless host: captures, declares no signals.
pub struct TestHost {
    /// Captured descriptions in present order.
    pub captures: Vec<Description>,
    pub(crate) body: Body,
    pub(crate) cells: BTreeMap<Hash, Cell>,
    signals: Signals,
}

impl Host for TestHost {
    type Raw = RawEvent;

    fn present(&mut self, d: &Description) -> Verdict<()> {
        self.captures.push(d.clone());
        Verdict::Ok(())
    }

    fn intend(&mut self, raw: Self::Raw) -> Verdict<Option<Intent>> {
        let intent = Intent {
            address: raw.address,
            term: raw.term,
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
