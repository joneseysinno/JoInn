//! The two truths every growth step keeps: V161 and V162.

use joinn_frame::{Hash, Value};

/// A response other than `count_witness` (V161), or a hash other than the
/// system's `hashes.txt` row (V162), is a fault, printed with the inputs.
pub(crate) fn step_fault(
    name: &str,
    inputs: &[Value],
    response: &Value,
    witness: &Value,
    hash: &Hash,
    golden: &str,
) -> Option<String> {
    let step = inputs.len();
    let shown: Vec<String> = inputs.iter().map(Value::print_term).collect();
    if response != witness {
        return Some(format!(
            "grow {name} {step}: V161: count {} differs from count_witness {} after [{}]",
            response.print_term(),
            witness.print_term(),
            shown.join(" ")
        ));
    }
    let hex = hash.to_hex();
    if hex != golden {
        return Some(format!(
            "grow {name} {step}: V162: hash {} after [{}] is not {name}.system's {}",
            &hex[..12],
            shown.join(" "),
            &golden[..golden.len().min(12)]
        ));
    }
    None
}
