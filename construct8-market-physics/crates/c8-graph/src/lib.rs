//! Graph Delta Engine and Construct8 Implementation
//!
//! This module implements the Construct8 graph delta system with a fixed-size array backend
//! for immutable triple application to an in-memory packed relation store.

use std::collections::HashMap;

pub use c8_core::{
    C8Error, C8Result, Construct8Len, Construct8Mask, HotPathVerdict, NodeId, RelationId,
};

/// A single RDF-like triple: (subject, predicate, object).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Construct8Triple {
    pub subject: NodeId,
    pub predicate: RelationId,
    pub object: NodeId,
}

impl Construct8Triple {
    /// Create a new triple.
    pub fn new(subject: NodeId, predicate: RelationId, object: NodeId) -> Self {
        Construct8Triple {
            subject,
            predicate,
            object,
        }
    }
}

impl Default for Construct8Triple {
    fn default() -> Self {
        Construct8Triple {
            subject: NodeId(0),
            predicate: RelationId(0),
            object: NodeId(0),
        }
    }
}

/// A reference to a triple using raw u32 fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TripleRef {
    pub subject: u32,
    pub predicate: u32,
    pub object: u32,
}

impl TripleRef {
    /// Create a new TripleRef.
    pub fn new(subject: u32, predicate: u32, object: u32) -> Self {
        TripleRef {
            subject,
            predicate,
            object,
        }
    }
}

impl From<TripleRef> for Construct8Triple {
    fn from(t: TripleRef) -> Self {
        Construct8Triple {
            subject: NodeId(t.subject as u64),
            predicate: RelationId(t.predicate),
            object: NodeId(t.object as u64),
        }
    }
}

/// A delta of up to 8 triples stored in a fixed array.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Construct8Delta {
    triples: [Construct8Triple; 8],
    pub valid_mask: Construct8Mask,
}

impl Construct8Delta {
    /// Create an empty delta.
    pub fn empty() -> Self {
        Construct8Delta {
            triples: [Construct8Triple::default(); 8],
            valid_mask: Construct8Mask(0),
        }
    }

    /// Return the number of triples in this delta.
    pub fn len(&self) -> usize {
        self.valid_mask.0.count_ones() as usize
    }

    /// Check if the delta is empty.
    pub fn is_empty(&self) -> bool {
        self.valid_mask.0 == 0
    }

    /// Return the bitmask indicating which slots are occupied.
    pub fn mask(&self) -> Construct8Mask {
        self.valid_mask
    }

    /// Push a triple, returning an error if the delta already has 8 triples.
    pub fn push_checked<T: Into<Construct8Triple>>(&mut self, triple: T) -> Result<(), C8Error> {
        let count = self.len();
        if count >= 8 {
            return Err(C8Error::Need9);
        }
        self.triples[count] = triple.into();
        self.valid_mask = Construct8Mask(self.valid_mask.0 | (1 << count));
        Ok(())
    }

    /// Return the fixed array of slots.
    pub fn as_fixed_slots(&self) -> &[Construct8Triple; 8] {
        &self.triples
    }

    /// Iterate over occupied triples in order.
    pub fn iter(&self) -> ConstructDeltaIter<'_> {
        ConstructDeltaIter {
            delta: self,
            index: 0,
        }
    }
}

/// Iterator over occupied triples in a Construct8Delta.
pub struct ConstructDeltaIter<'a> {
    delta: &'a Construct8Delta,
    index: usize,
}

impl<'a> Iterator for ConstructDeltaIter<'a> {
    type Item = (usize, Construct8Triple);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < 8 {
            let slot = self.index;
            self.index += 1;
            if (self.delta.valid_mask.0 & (1 << slot)) != 0 {
                return Some((slot, self.delta.triples[slot]));
            }
        }
        None
    }
}

/// Builder for safe building of Construct8Deltas
#[derive(Debug)]
pub struct Construct8DeltaBuilder {
    delta: Construct8Delta,
}

impl Default for Construct8DeltaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Construct8DeltaBuilder {
    /// Create a new delta builder.
    pub fn new() -> Self {
        Construct8DeltaBuilder {
            delta: Construct8Delta::empty(),
        }
    }

    /// Add a triple to the delta builder.
    pub fn push<T: Into<Construct8Triple>>(mut self, triple: T) -> Result<Self, C8Error> {
        self.delta.push_checked(triple)?;
        Ok(self)
    }

    /// Build the delta structure.
    pub fn build(self) -> Construct8Delta {
        self.delta
    }
}

/// Result of applying a delta to a graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GraphApplyResult {
    pub stats: BranchlessApplyStats,
    pub new_state_hash: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BranchlessApplyStats {
    pub lanes_applied: u32,
    pub lanes_skipped: u32,
}

/// An in-memory packed relation store.
#[derive(Debug, Clone)]
pub struct GraphField {
    relations: HashMap<(NodeId, RelationId, NodeId), bool>,
    state_hash: u64,
}

impl Default for GraphField {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphField {
    /// Create an empty graph field.
    pub fn new() -> Self {
        GraphField {
            relations: HashMap::new(),
            state_hash: 0xcafebabedeadbeef,
        }
    }

    /// Return the number of distinct (s, p, o) triples stored.
    pub fn relation_count(&self) -> usize {
        self.relations.len()
    }

    /// Return true if the exact triple (s, p, o) is present.
    pub fn contains_relation(&self, s: NodeId, p: RelationId, o: NodeId) -> bool {
        self.relations.contains_key(&(s, p, o))
    }

    /// Return the current state hash.
    pub fn state_hash(&self) -> u64 {
        self.state_hash
    }

    /// Apply a `Construct8Delta` to this graph using a mask-based loop.
    #[allow(clippy::map_entry)]
    pub fn apply_construct8(&mut self, delta: &Construct8Delta) -> GraphApplyResult {
        let mut lanes_applied: u32 = 0;
        let mut lanes_skipped: u32 = 0;
        let slots = delta.as_fixed_slots();
        let mask = delta.valid_mask.0;

        for lane in 0u8..8 {
            if (mask & (1 << lane)) != 0 {
                let t = slots[lane as usize];
                let key = (t.subject, t.predicate, t.object);
                if self.relations.contains_key(&key) {
                    lanes_skipped += 1;
                } else {
                    self.relations.insert(key, true);
                    let mix = t.subject.0.wrapping_mul(0x9e3779b97f4a7c15)
                        ^ (t.predicate.0 as u64).wrapping_mul(0x6c62272e07bb0142)
                        ^ t.object.0.wrapping_mul(0xd2a98b26625eee7b);
                    self.state_hash = self
                        .state_hash
                        .wrapping_mul(6364136223846793005)
                        .wrapping_add(mix);
                    lanes_applied += 1;
                }
            }
        }

        GraphApplyResult {
            stats: BranchlessApplyStats {
                lanes_applied,
                lanes_skipped,
            },
            new_state_hash: self.state_hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_triple_sets_one_mask_bit() {
        let mut delta = Construct8Delta::empty();
        delta
            .push_checked(Construct8Triple::new(NodeId(1), RelationId(10), NodeId(2)))
            .expect("first push must succeed");
        assert_eq!(delta.len(), 1);
        assert!(!delta.is_empty());
        assert_eq!(delta.mask().0, 0b00000001);
    }

    #[test]
    fn eight_triples_succeed() {
        let mut delta = Construct8Delta::empty();
        for i in 0u32..8 {
            delta
                .push_checked(Construct8Triple::new(
                    NodeId(i as u64),
                    RelationId(10),
                    NodeId(i as u64 + 100),
                ))
                .expect("push must succeed");
        }
        assert_eq!(delta.len(), 8);
        assert_eq!(delta.mask().0, 0xFF);
    }

    #[test]
    fn ninth_triple_refuses_with_need9() {
        let mut delta = Construct8Delta::empty();
        for i in 0u32..8 {
            delta
                .push_checked(Construct8Triple::new(
                    NodeId(i as u64),
                    RelationId(10),
                    NodeId(i as u64 + 100),
                ))
                .expect("push must succeed");
        }
        let result = delta.push_checked(Construct8Triple::new(
            NodeId(99),
            RelationId(10),
            NodeId(200),
        ));
        assert_eq!(result, Err(C8Error::Need9));
    }

    #[test]
    fn apply_same_delta_twice_is_idempotent() {
        let mut graph = GraphField::new();
        let mut delta = Construct8Delta::empty();
        delta
            .push_checked(Construct8Triple::new(NodeId(1), RelationId(10), NodeId(2)))
            .expect("push must succeed");

        let r1 = graph.apply_construct8(&delta);
        assert_eq!(r1.stats.lanes_applied, 1);
        assert_eq!(r1.stats.lanes_skipped, 0);

        let r2 = graph.apply_construct8(&delta);
        assert_eq!(r2.stats.lanes_applied, 0);
        assert_eq!(r2.stats.lanes_skipped, 1);
        assert_eq!(r1.new_state_hash, r2.new_state_hash);
    }
}
