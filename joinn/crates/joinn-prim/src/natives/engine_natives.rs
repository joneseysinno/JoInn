//! Engine-visible natives: sealed alleles plus physics lookup members.

use crate::floor::{HashPrim, ResolvePrim};
use joinn_gate::NativeRegistry;
use std::sync::Arc;

use super::sealed_natives::sealed_natives;

/// Engine-visible natives: sealed alleles plus physics lookup members.
pub fn engine_natives() -> NativeRegistry {
    let mut r = sealed_natives();
    r.insert("prim:hash", Arc::new(HashPrim));
    r.insert("prim:resolve", Arc::new(ResolvePrim));
    r
}
