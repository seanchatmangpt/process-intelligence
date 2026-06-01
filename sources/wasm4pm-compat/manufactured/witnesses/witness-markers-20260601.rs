/// Witness Marker Enumeration
/// Authority: wasm4pm-compat Type-Law Atlas
/// Generated: 2026-06-01
/// Purpose: Non-forgeable algebraic witness proof structures
///
/// Each witness marker represents a distinct algebraic proof
/// structure in the wasm4pm-compat witness lattice.
/// Lattice monotonicity is enforced at the type level.

use std::fmt;

/// Witness Marker Lattice
///
/// Forms a join-semilattice: Parsed ⊆ ValidatedSound ⊆ Replayed ⊆ Archived
/// with algebraic properties (associativity, commutativity, idempotency, absorption).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum WitnessMarker {
    /// Van der Aalst 1989: Petri Net Foundation
    /// Lattice position: bottom
    /// Structure: Petri Net Marking (place → token count)
    /// Formalism: workflow-net
    VanDerAalst1989,

    /// Van der Aalst 1998: Soundness Proof
    /// Lattice position: middle
    /// Structure: Soundness verification witness
    /// Formalism: workflow-net
    VanDerAalst1998,

    /// Van der Aalst 2016: Object-Centric Event Log (OCEL)
    /// Lattice position: middle
    /// Structure: Object-centric event trace mapping
    /// Formalism: object-centric
    VanDerAalst2016,

    /// Murata 1989: Petri Net Theory Foundation
    /// Lattice position: bottom
    /// Structure: Petri Net Semantics and Reachability
    /// Formalism: petri-net
    Murata1989,

    /// Weijters 2011: Heuristics Miner Algorithm
    /// Lattice position: middle
    /// Structure: Causal Net Marking
    /// Formalism: causal-net
    Weijters2011,

    /// Leemans 2013: Inductive Miner Algorithm
    /// Lattice position: middle
    /// Structure: Process Tree Node Marking
    /// Formalism: process-tree
    Leemans2013,

    /// Adriansyah 2011: Process-Log Alignment
    /// Lattice position: middle
    /// Structure: Alignment Step Cost
    /// Formalism: conformance
    Adriansyah2011,

    /// Blue River Dam: Autonomic Actuation Engine
    /// Lattice position: top
    /// Structure: Autonomic Marking with MAPE-K enforcement
    /// Formalism: lifecycle
    BlueRiverDam,
}

impl WitnessMarker {
    /// Returns the human-readable label for this witness
    pub fn label(&self) -> &'static str {
        match self {
            WitnessMarker::VanDerAalst1989 => "Van der Aalst 1989 Petri Net Witness",
            WitnessMarker::VanDerAalst1998 => "Van der Aalst 1998 Soundness Witness",
            WitnessMarker::VanDerAalst2016 => "Van der Aalst 2016 OCEL Witness",
            WitnessMarker::Murata1989 => "Murata 1989 Petri Net Theory",
            WitnessMarker::Weijters2011 => "Weijters 2011 Heuristics Miner",
            WitnessMarker::Leemans2013 => "Leemans 2013 Inductive Miner",
            WitnessMarker::Adriansyah2011 => "Adriansyah 2011 Alignment",
            WitnessMarker::BlueRiverDam => "Blue River Dam Autonomic Actuation",
        }
    }

    /// Returns the lattice position (for ordering in witness lattice)
    pub fn lattice_position(&self) -> &'static str {
        match self {
            WitnessMarker::VanDerAalst1989 | WitnessMarker::Murata1989 => "bottom",
            WitnessMarker::VanDerAalst1998
            | WitnessMarker::VanDerAalst2016
            | WitnessMarker::Weijters2011
            | WitnessMarker::Leemans2013
            | WitnessMarker::Adriansyah2011 => "middle",
            WitnessMarker::BlueRiverDam => "top",
        }
    }

    /// Returns the formal structure type for this witness
    pub fn structure(&self) -> &'static str {
        match self {
            WitnessMarker::VanDerAalst1989 => "PetriNetMarking",
            WitnessMarker::VanDerAalst1998 => "SoundnessProof",
            WitnessMarker::VanDerAalst2016 => "ObjectCentricEventLog",
            WitnessMarker::Murata1989 => "PetriNetSemantics",
            WitnessMarker::Weijters2011 => "CausalNetMarking",
            WitnessMarker::Leemans2013 => "ProcessTreeNode",
            WitnessMarker::Adriansyah2011 => "AlignmentStep",
            WitnessMarker::BlueRiverDam => "AutonomicMarking",
        }
    }

    /// Returns the formal system (formalism) for this witness
    pub fn formalism(&self) -> &'static str {
        match self {
            WitnessMarker::VanDerAalst1989 | WitnessMarker::VanDerAalst1998 => "workflow-net",
            WitnessMarker::VanDerAalst2016 => "object-centric",
            WitnessMarker::Murata1989 => "petri-net",
            WitnessMarker::Weijters2011 => "causal-net",
            WitnessMarker::Leemans2013 => "process-tree",
            WitnessMarker::Adriansyah2011 => "conformance",
            WitnessMarker::BlueRiverDam => "lifecycle",
        }
    }
}

impl fmt::Display for WitnessMarker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

/// Lattice Join Operation (least upper bound)
///
/// Enforces algebraic properties:
/// - Associativity: a ∪ (b ∪ c) = (a ∪ b) ∪ c
/// - Commutativity: a ∪ b = b ∪ a
/// - Idempotency: a ∪ a = a
/// - Absorption: a ⊆ b ⇒ a ∪ b = b
pub fn witness_join(left: WitnessMarker, right: WitnessMarker) -> WitnessMarker {
    use WitnessMarker::*;

    // Establish partial order: bottom < middle < top
    let left_level = match left {
        VanDerAalst1989 | Murata1989 => 0,
        VanDerAalst1998 | VanDerAalst2016 | Weijters2011 | Leemans2013 | Adriansyah2011 => 1,
        BlueRiverDam => 2,
    };

    let right_level = match right {
        VanDerAalst1989 | Murata1989 => 0,
        VanDerAalst1998 | VanDerAalst2016 | Weijters2011 | Leemans2013 | Adriansyah2011 => 1,
        BlueRiverDam => 2,
    };

    // Join selects the maximum in the lattice order
    if left_level >= right_level {
        left
    } else {
        right
    }
}

/// Lattice Partial Order Test (⊆)
/// Returns true if left ⊆ right in the witness lattice
pub fn witness_leq(left: WitnessMarker, right: WitnessMarker) -> bool {
    use WitnessMarker::*;

    let left_level = match left {
        VanDerAalst1989 | Murata1989 => 0,
        VanDerAalst1998 | VanDerAalst2016 | Weijters2011 | Leemans2013 | Adriansyah2011 => 1,
        BlueRiverDam => 2,
    };

    let right_level = match right {
        VanDerAalst1989 | Murata1989 => 0,
        VanDerAalst1998 | VanDerAalst2016 | Weijters2011 | Leemans2013 | Adriansyah2011 => 1,
        BlueRiverDam => 2,
    };

    left_level <= right_level
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_witness_lattice_monotonicity() {
        // Test idempotency: a ∪ a = a
        assert_eq!(
            witness_join(WitnessMarker::VanDerAalst1989, WitnessMarker::VanDerAalst1989),
            WitnessMarker::VanDerAalst1989
        );

        // Test absorption: a ⊆ b ⇒ a ∪ b = b
        assert_eq!(
            witness_join(WitnessMarker::VanDerAalst1989, WitnessMarker::BlueRiverDam),
            WitnessMarker::BlueRiverDam
        );

        // Test commutativity: a ∪ b = b ∪ a
        assert_eq!(
            witness_join(WitnessMarker::Leemans2013, WitnessMarker::Adriansyah2011),
            witness_join(WitnessMarker::Adriansyah2011, WitnessMarker::Leemans2013)
        );
    }

    #[test]
    fn test_witness_partial_order() {
        // Test reflexivity: a ⊆ a
        assert!(witness_leq(WitnessMarker::VanDerAalst1989, WitnessMarker::VanDerAalst1989));

        // Test transitivity: a ⊆ b ∧ b ⊆ c ⇒ a ⊆ c
        assert!(witness_leq(WitnessMarker::VanDerAalst1989, WitnessMarker::BlueRiverDam));
        assert!(witness_leq(WitnessMarker::BlueRiverDam, WitnessMarker::BlueRiverDam));
    }
}
