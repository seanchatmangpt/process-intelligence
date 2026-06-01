//! Generated from OTel Weaver resolved registry. DO NOT EDIT HAND-CODED.
//! Generated at: 2026-06-01T10:10:51-07:00

use serde::{Serialize, Deserialize};

/// The process-evidence lattice trait.
pub trait Lattice: Eq + PartialOrd + Serialize {
    fn bottom() -> Self;
    fn top() -> Self;
    fn join(&self, other: &Self) -> Self;
}

/// Witness representing the validation seal of process.pi.activity
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityWitness {
    /// The unique identifier of the process execution instance.
    #[serde(rename = "process.pi.instance_id")]
    pub instance_id: String,
    /// The identifier of the witness node.
    #[serde(rename = "process.pi.witness.id")]
    pub witness_id: String,
    /// BLAKE3 cryptographic hash sealing this specific transition trace.
    #[serde(rename = "process.pi.witness.hash")]
    pub witness_hash: String,
}

impl PartialOrd for ActivityWitness {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(std::cmp::Ordering::Equal)
        } else if self.witness_hash.is_empty() && !other.witness_hash.is_empty() {
            Some(std::cmp::Ordering::Less)
        } else if !self.witness_hash.is_empty() && other.witness_hash.is_empty() {
            Some(std::cmp::Ordering::Greater)
        } else {
            None // Incomparable distinct execution paths
        }
    }
}

impl Lattice for ActivityWitness {
    fn bottom() -> Self {
        Self {
            instance_id: String::new(),
            witness_id: String::new(),
            witness_hash: String::new(),
        }
    }

    fn top() -> Self {
        Self {
            instance_id: "TOP".to_string(),
            witness_id: "TOP".to_string(),
            witness_hash: "f".repeat(64),
        }
    }

    fn join(&self, other: &Self) -> Self {
        if self == other {
            return self.clone();
        }
        if self.witness_hash.is_empty() {
            return other.clone();
        }
        if other.witness_hash.is_empty() {
            return self.clone();
        }
        // Conflict resolution: return top to represent contradiction
        Self::top()
    }
}
