//! Ordered frame registry. `BTreeMap` only.

mod phase1;

use crate::frame::{Frame, FrameRef};
use std::collections::BTreeMap;
use std::sync::Arc;

/// Maps a `FrameRef` to its implementation.
#[derive(Clone, Default)]
pub struct FrameRegistry {
    frames: BTreeMap<FrameRef, Arc<dyn Frame>>,
}

impl FrameRegistry {
    /// Empty registry.
    pub fn new() -> Self {
        Self {
            frames: BTreeMap::new(),
        }
    }

    /// Insert a frame under its own reference.
    pub fn register(&mut self, frame: Arc<dyn Frame>) {
        let key = frame.reference();
        self.frames.insert(key, frame);
    }

    /// Borrow a frame.
    pub fn get(&self, r: &FrameRef) -> Option<&dyn Frame> {
        self.frames.get(r).map(|a| a.as_ref())
    }

    /// Iterate in `FrameRef` order.
    pub fn iter(&self) -> impl Iterator<Item = (&FrameRef, &dyn Frame)> {
        self.frames.iter().map(|(k, v)| (k, v.as_ref()))
    }

    /// True when the reference is registered.
    pub fn contains(&self, r: &FrameRef) -> bool {
        self.frames.contains_key(r)
    }
}
