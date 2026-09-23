//! Host protocol. A description is a value, not text. No IO.

#![forbid(unsafe_code)]

mod description;
mod host;
mod intent;
mod probe;
mod role;
mod signals;

pub use description::{
    Description, PortFace, describe, describe_refusal, hash_description, print_description,
};
pub use host::Host;
pub use intent::{Address, Intent, check_intent, intent_set};
pub use joinn_dna::Direction;
pub use probe::probe;
pub use role::Role;
pub use signals::{Signals, check_signals, signals_from_environment};

#[cfg(test)]
mod tests {
    #[test]
    fn a_writing_probe_does_not_compile() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/fail/writing_probe.rs");
    }
}
