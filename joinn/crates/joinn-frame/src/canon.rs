//! Canonical-byte writer and Unicode NFC.

mod nfc;

pub use nfc::nfc;

/// Accumulates canonical bytes. The writer does not invent formatting;
/// callers push already-canonical text.
#[derive(Clone, Default, Debug)]
pub struct CanonWriter {
    inner: Vec<u8>,
}

impl CanonWriter {
    /// Empty writer.
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Append a UTF-8 string.
    pub fn push_str(&mut self, s: &str) {
        self.inner.extend_from_slice(s.as_bytes());
    }

    /// Append raw bytes.
    pub fn push_bytes(&mut self, bytes: &[u8]) {
        self.inner.extend_from_slice(bytes);
    }

    /// Finish and return the byte buffer.
    pub fn finish(self) -> Vec<u8> {
        self.inner
    }

    /// Borrow the bytes so far.
    pub fn as_bytes(&self) -> &[u8] {
        &self.inner
    }
}
