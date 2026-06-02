//! # Identifiers Module
//!
//! Zero-cost type wrappers for the eight core identifier classes in Construct8.
//! Each newtype ensures type-safety without runtime overhead.

/// Node identifier: logical entities in the market graph.
///
/// A `NodeId` represents a single logical node—market participant, entity, or decision point.
///
/// # Examples
///
/// ```
/// use c8_core::NodeId;
///
/// let node = NodeId(42);
/// assert_eq!(node.0, 42);
/// ```
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(pub u64);

/// Relation identifier: edges and connections in the graph.
///
/// A `RelationId` names a specific directed edge or relationship.
///
/// # Examples
///
/// ```
/// use c8_core::RelationId;
///
/// let rel = RelationId(100);
/// assert_eq!(rel.0, 100);
/// ```
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RelationId(pub u64);

/// Venue identifier: market infrastructure and settlement layers.
///
/// A `VenueId` identifies the specific venue (exchange, clearinghouse, etc.)
/// where orders or events occur.
///
/// # Examples
///
/// ```
/// use c8_core::VenueId;
///
/// let venue = VenueId(10);
/// assert_eq!(venue.0, 10);
/// ```
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VenueId(pub u64);

/// Instrument identifier: tradable assets and contracts.
///
/// An `InstrumentId` uniquely identifies an asset, security, or contract.
///
/// # Examples
///
/// ```
/// use c8_core::InstrumentId;
///
/// let instr = InstrumentId(5);
/// assert_eq!(instr.0, 5);
/// ```
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InstrumentId(pub u64);

/// Actor class identifier: role-based entity types.
///
/// An `ActorClassId` names a category of market participant (e.g., broker, investor, maker).
///
/// # Examples
///
/// ```
/// use c8_core::ActorClassId;
///
/// let actor_class = ActorClassId(2);
/// assert_eq!(actor_class.0, 2);
/// ```
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ActorClassId(pub u64);

/// Graph slot: position in the constraint graph.
///
/// A `GraphSlot` names a specific location or dimension in the constraint graph.
///
/// # Examples
///
/// ```
/// use c8_core::GraphSlot;
///
/// let slot = GraphSlot(3);
/// assert_eq!(slot.0, 3);
/// ```
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GraphSlot(pub u64);

/// C8 identifier: top-level unique identifier for the entire graph or system.
///
/// A `C8Id` is the universal identifier for a Construct8 instance or market structure.
///
/// # Examples
///
/// ```
/// use c8_core::C8Id;
///
/// let c8 = C8Id(999);
/// assert_eq!(c8.0, 999);
/// ```
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct C8Id(pub u64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_id_creation() {
        let node = NodeId(42);
        assert_eq!(node.0, 42);
    }

    #[test]
    fn test_relation_id_equality() {
        let rel1 = RelationId(100);
        let rel2 = RelationId(100);
        assert_eq!(rel1, rel2);
    }

    #[test]
    fn test_venue_id_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(VenueId(10));
        set.insert(VenueId(11));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_instrument_id_copy() {
        let instr1 = InstrumentId(5);
        let instr2 = instr1;
        assert_eq!(instr1, instr2);
    }

    #[test]
    fn test_actor_class_id_debug() {
        let actor_class = ActorClassId(2);
        let debug_str = format!("{:?}", actor_class);
        assert!(debug_str.contains("ActorClassId"));
    }

    #[test]
    fn test_graph_slot_ordering() {
        let slot1 = GraphSlot(3);
        let slot2 = GraphSlot(4);
        assert!(slot1 < slot2);
    }

    #[test]
    fn test_c8id_zero_cost() {
        use std::mem::size_of;
        assert_eq!(size_of::<C8Id>(), size_of::<u64>());
    }
}
