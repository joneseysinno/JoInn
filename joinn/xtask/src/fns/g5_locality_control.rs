//! Gate 5 item 8 control: true when a real far-side line contains the text.

use super::double_delivery::double_delivery;
use super::subject::Subject;
use joinn_link::format_link_refusal;

pub(crate) fn g5_locality_control(subject: &Subject) -> bool {
    let Subject::Text(secret) = subject else {
        return false;
    };
    if secret.is_empty() {
        return false;
    }
    let Ok(delivery) = double_delivery() else {
        return false;
    };
    delivery
        .far_side
        .iter()
        .any(|refusal| format_link_refusal(refusal).contains(secret.as_str()))
}
