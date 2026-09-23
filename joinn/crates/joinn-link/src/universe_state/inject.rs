//! Inject a host value at one body's membrane port.

use joinn_frame::{Value, Verdict};

use crate::universe_state::UniverseState;
use crate::Address;

impl UniverseState {
    /// Enqueue `v` at `addr` on `body`. The body's own inject rules apply.
    pub fn inject(
        &mut self,
        body: &str,
        addr: &Address,
        v: Value,
        epoch: u64,
    ) -> Verdict<()> {
        let Some(state) = self.bodies.get_mut(body) else {
            return crate::refuse(format!(
                "inject: no body {body}; acceptance is a bound alias"
            ));
        };
        state.inject(&addr.instance, addr.port, v, epoch)
    }
}
