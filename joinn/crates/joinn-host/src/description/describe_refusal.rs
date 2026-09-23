//! A membrane refusal, shown as a description.

use joinn_frame::{Hash, Verdict};
use joinn_live::BodyState;

use super::{Description, describe};
use crate::role::Role;

/// A membrane refusal, shown as a description.
pub fn describe_refusal(state: &BodyState, instance: &str, reason: &str) -> Description {
    match describe(state, instance) {
        Verdict::Ok(mut d) => {
            d.role = Role::Refusal;
            d.label = reason.to_owned();
            d
        }
        Verdict::Refused(_) => Description {
            cell: Hash::from_bytes([0; 32]),
            instance: instance.to_owned(),
            ports: Vec::new(),
            label: reason.to_owned(),
            role: Role::Refusal,
        },
    }
}
