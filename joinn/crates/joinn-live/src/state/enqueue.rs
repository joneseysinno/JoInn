//! Enqueue mail on an activation.

use crate::activation::Mail;
use crate::state::BodyState;

impl BodyState {
    pub(in crate::state) fn enqueue(&mut self, depth: usize, m: Mail) {
        let key = (
            m.grant_epoch,
            m.wire_position,
            m.message_sequence,
            self.seq,
            m.dest.clone(),
            m.port,
        );
        if let Some(act) = self.stack.get_mut(depth) {
            act.queue.insert(key.clone());
            act.mail.insert(key, m);
        }
    }
}
