//! Content hash. This is JoInn's `Hash`, not Rust's `std::hash::Hash`.

mod keyed_hash;

pub use keyed_hash::keyed_hash;

/// Domain tag for a coding region's identity.
pub const TAG_CELL: &[u8] = b"joinn.cell.v1";
/// Domain tag for an allele payload.
pub const TAG_ALLELE: &[u8] = b"joinn.allele.v1";
/// Domain tag for a witness hashed alone.
pub const TAG_WITNESS: &[u8] = b"joinn.witness.v1";
/// Domain tag for a body's coding region.
pub const TAG_BODY: &[u8] = b"joinn.body.v1";
/// Domain tag for a cell description.
pub const TAG_DESCRIPTION: &[u8] = b"joinn.description.v1";
/// Domain tag for a universe's coding region.
pub const TAG_UNIVERSE: &[u8] = b"joinn.universe.v1";

/// BLAKE3-256 digest. Never truncated in storage.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Hash(pub(in crate::hash) [u8; 32]);

impl Hash {
    /// Build from a 32-byte digest.
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Raw digest.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Lowercase hex, 64 characters.
    pub fn to_hex(&self) -> String {
        let mut out = String::with_capacity(64);
        for b in self.0 {
            out.push(nibble(b >> 4));
            out.push(nibble(b & 0x0f));
        }
        out
    }

    /// First four hex characters, for prose only.
    pub fn short_hex(&self) -> String {
        self.to_hex().chars().take(4).collect()
    }

    /// Parse a 64-character lowercase or uppercase hex digest.
    pub fn parse_hex(s: &str) -> Option<Self> {
        if s.len() != 64 {
            return None;
        }
        let mut bytes = [0u8; 32];
        let raw = s.as_bytes();
        for i in 0..32 {
            let hi = from_hex(raw[2 * i])?;
            let lo = from_hex(raw[2 * i + 1])?;
            bytes[i] = (hi << 4) | lo;
        }
        Some(Self(bytes))
    }
}

impl core::fmt::Display for Hash {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.to_hex())
    }
}

pub(in crate::hash) fn nibble(n: u8) -> char {
    b"0123456789abcdef"[n as usize] as char
}

pub(in crate::hash) fn from_hex(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}
