# Dependencies

Phase 1 crates may depend only on what is listed in the implementation plan §7, plus the Rust standard library.

| Crate | Replaces / provides |
|---|---|
| `num-bigint` | Arbitrary-precision ℤ terms |
| `num-rational` | Exact ℚ, lowest terms |
| `num-integer` | GCD used when building ℚ terms |
| `num-traits` | Sign and conversion helpers for `BigInt` |
| `blake3` | 256-bit content hashes with domain tags |
| `unicode-normalization` | NFC on every parsed literal and identifier |
| `proptest` (dev) | Property tests for *our* invariants, not frame sampling |
| `trybuild` (dev) | Compile-fail tests for `Genotype` / `Value` construction |
| `insta` (dev) | Snapshots of refusal messages and printed canonical text — never corpus hashes |

Nothing else is added without a new line in this file.
