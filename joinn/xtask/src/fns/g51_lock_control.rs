//! Gate 5.1 item 9 control: the artifact lock is not a full pass.

use super::subject::Subject;

pub(crate) fn g51_lock_control(subject: &Subject) -> bool {
    let Subject::Lock(rows) = subject else {
        return true;
    };
    !rows.iter().any(|row| row.n != row.total)
}
