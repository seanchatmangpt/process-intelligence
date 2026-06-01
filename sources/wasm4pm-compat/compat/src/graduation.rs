//! Graduation Bridge definitions
//!
//! Enforces the boundary between wasm4pm-compat and the execution engine wasm4pm.

use std::marker::PhantomData;

/// Named reasons why a compatibility layer candidate must graduate to the execution engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum GraduationReason {
    NeedsDiscovery,
    NeedsConformanceExecution,
    NeedsReplay,
    NeedsReceipts,
    NeedsBenchmarkGate,
    NeedsObjectCentricQueryExecution,
    RebuildingProcessMiningLocally,
}

impl GraduationReason {
    /// Returns true if this is a hard signal (mandatory graduation).
    pub fn is_hard_signal(&self) -> bool {
        match self {
            GraduationReason::NeedsDiscovery
            | GraduationReason::NeedsConformanceExecution
            | GraduationReason::NeedsReplay
            | GraduationReason::NeedsObjectCentricQueryExecution
            | GraduationReason::RebuildingProcessMiningLocally => true,
            GraduationReason::NeedsReceipts | GraduationReason::NeedsBenchmarkGate => false,
        }
    }

    /// Returns a string tag for this reason.
    pub fn tag(&self) -> &'static str {
        match self {
            GraduationReason::NeedsDiscovery => "needs_discovery",
            GraduationReason::NeedsConformanceExecution => "needs_conformance_execution",
            GraduationReason::NeedsReplay => "needs_replay",
            GraduationReason::NeedsReceipts => "needs_receipts",
            GraduationReason::NeedsBenchmarkGate => "needs_benchmark_gate",
            GraduationReason::NeedsObjectCentricQueryExecution => "needs_object_centric_query_execution",
            GraduationReason::RebuildingProcessMiningLocally => "rebuilding_process_mining_locally",
        }
    }
}

/// A graduation candidate carrying a reason, a description of the subject,
/// an opaque evidence reference, and the preserved witness.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraduationCandidate<T, W> {
    pub reason: GraduationReason,
    pub subject: String,
    pub evidence_ref: String,
    pub witness: W,
    pub _marker: PhantomData<T>,
}

impl<T, W> GraduationCandidate<T, W> {
    /// Creates a new GraduationCandidate.
    pub fn new(reason: GraduationReason, subject: String, evidence_ref: String, witness: W) -> Self {
        GraduationCandidate {
            reason,
            subject,
            evidence_ref,
            witness,
            _marker: PhantomData,
        }
    }

    /// Returns true if the candidate is fully grounded:
    /// - Both subject and evidence_ref are non-empty.
    pub fn is_grounded(&self) -> bool {
        !self.subject.trim().is_empty() && !self.evidence_ref.trim().is_empty()
    }
}

/// Trait implemented by compatibility types that can graduate to the execution engine.
pub trait GraduateToWasm4pm {
    type Target;
    type Witness;
    fn candidate(&self) -> GraduationCandidate<Self::Target, Self::Witness>;
}
