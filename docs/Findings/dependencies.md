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

## Phase 6 (the renderer boundary)

Added by the Phase 6 plan §2.2. Each may be used only by the crates named.

| Crate | Version | Used by | Provides |
|---|---|---|---|
| `wgpu` | `30` | joinn-gpu, joinn-shell-desktop | The GPU API (validated by S2 at 30.0.1) |
| `winit` | `0.30` | joinn-shell-desktop | Window, event loop, input |
| `pollster` | `0.4` | joinn-gpu, joinn-shell-desktop | Blocking on wgpu's async setup |
| `bytemuck` | `1` (feature `derive`) | joinn-gpu | Casting GPU uniform structs to bytes. Not on any DNA type |

Nothing else is added without a new line in this file.

## Phase 7.3 (one build, plan 7.3 §2.11 b)

Features pinned so the workspace builds once. No crate enters `Cargo.lock`; each line names a crate already in it.

| Crate | Features | Pinned in | Why |
|---|---|---|---|
| `bitflags` | `std` | xtask | features pinned so the workspace builds once (winit turns it on, on Linux) |
| `smallvec` | `const_generics`, `const_new`, `union` | xtask | features pinned so the workspace builds once (winit, on Linux) |
| `bytemuck` | `aarch64_simd` (with `derive`) | joinn-gpu | features pinned so the workspace builds once (winit, on Linux); bytemuck stays in joinn-gpu only |
| `joinn-prim` | `mutants` | joinn-cli | features pinned so the workspace builds once (xtask turns it on); the CLI calls nothing it adds |
| `num-traits` | `libm` | joinn-cli | features pinned so the workspace builds once (naga, through `half`, turns it on) |
