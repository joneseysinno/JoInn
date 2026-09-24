//! Whether a line carries a modules silencer.

use super::allow_modules_reason::allow_modules_reason;

/// True when the line has any modules-layout silencer mark.
pub fn line_has_allow_modules(line: &str) -> bool {
    allow_modules_reason(line).is_some()
}
