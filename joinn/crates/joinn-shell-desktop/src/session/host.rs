//! The shell is a host: present into the scene, intend on the selection.

use joinn_frame::{Term, Verdict};
use joinn_host::{Host, Intent, Signals, check_intent};

use super::Desktop;

impl Host for Desktop {
    type Raw = String;

    fn present(&mut self, d: &joinn_host::Description) -> Verdict<()> {
        match self.scene.present(d) {
            Verdict::Ok(_) => Verdict::Ok(()),
            Verdict::Refused(r) => Verdict::Refused(r),
        }
    }

    fn intend(&mut self, raw: Self::Raw) -> Verdict<Option<Intent>> {
        let Some(address) = self.selected.clone() else {
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
