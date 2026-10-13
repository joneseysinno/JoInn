//! Letters and strands (plan 8 §2.1, §2.2): A, T, G and C as two bits each,
//! a strand read head to tail, its partner strand, packing four letters to a
//! byte, and a strand's identity. A letter alone means nothing (D5): nothing
//! here names a codon or an amino acid.

#![forbid(unsafe_code)]

mod letter;
mod refuse;
mod strand;

pub use letter::{Letter, Shape, Strength};
pub use strand::{
    Packed, Strand, StrandFile, TAG_DNA, parse_letters, parse_strand_file, print_letters,
    print_strand_file,
};
