//! Graduation Intake and Engine-Side Validation
//!
//! Implements engine intake validation of GraduateToWasm4pm candidates.
//! Verifies candidates are fully grounded and carry valid preserved witnesses.

use wasm4pm_compat::graduation::{GraduateToWasm4pm, GraduationCandidate, GraduationReason};
use crate::mining::Event;
use crate::conformance::ConformanceVerdicts;
use crate::evidence::{WitnessState, Lattice};

/// Ingestion error types, avoiding raw strings to enforce type-law specificity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngestionError {
    /// The graduation candidate is not grounded (subject or evidence reference is empty).
    UngroundedCandidate {
        subject: String,
        evidence_ref: String,
    },
    /// The graduation reason does not match the capabilities of the target type.
    ReasonMismatch {
        reason: GraduationReason,
    },
    /// The witness carried by the candidate is in an invalid state (e.g. Top/Conflict).
    InvalidWitnessState,
}

/// The engine intake candidate validator.
///
/// Validates:
/// 1. The candidate is fully grounded (non-empty subject & evidence reference).
/// 2. The preserved witness is valid (not bottom or top for mandatory execution).
/// 3. The graduation reason matches the expected capability.
pub fn validate_engine_intake<T, W>(
    candidate: &GraduationCandidate<T, W>,
    expected_reason: GraduationReason,
) -> Result<(), IngestionError>
where
    W: Lattice + Clone,
{
    // 1. Ensure the candidate is grounded
    if !candidate.is_grounded() {
        return Err(IngestionError::UngroundedCandidate {
            subject: candidate.subject.clone(),
            evidence_ref: candidate.evidence_ref.clone(),
        });
    }

    // 2. Validate that the graduation reason matches the expected engine execution capability
    if candidate.reason != expected_reason {
        return Err(IngestionError::ReasonMismatch {
            reason: candidate.reason,
        });
    }

    // 3. Ensure the witness is preserved and valid
    // If it's a hard signal (mandatory graduation), the witness must not be Top/Conflict.
    if candidate.reason.is_hard_signal() && candidate.witness.is_top() {
        return Err(IngestionError::InvalidWitnessState);
    }

    Ok(())
}

/// Bridge intake function for GraduateToWasm4pm candidates.
///
/// Accepts a graduation candidate from the compat layer and validates it against
/// the expected reason, ensuring all type law constraints are satisfied before
/// the candidate enters the execution engine.
///
/// # Arguments
/// - `candidate`: The graduation candidate carrying evidence and witness from compat
/// - `expected_reason`: The expected graduation reason for this intake path
///
/// # Returns
/// - `Ok(())` if the candidate is valid and fully grounded
/// - `Err(IngestionError)` if validation fails (ungrounded, reason mismatch, invalid witness)
///
/// # Example
///
/// ```ignore
/// let cand = GraduationCandidate::new(
///     GraduationReason::NeedsDiscovery,
///     "ocel:case-123".to_string(),
///     "blake3:abc123".to_string(),
///     WitnessState::Bottom,
/// );
/// accept_from_compat(&cand, GraduationReason::NeedsDiscovery)?;
/// ```
pub fn accept_from_compat<T, W>(
    candidate: &GraduationCandidate<T, W>,
    expected_reason: GraduationReason,
) -> Result<(), IngestionError>
where
    W: Lattice + Clone,
{
    validate_engine_intake(candidate, expected_reason)
}

/// A wrapper for the event log that implements the GraduateToWasm4pm trait.
pub struct GraduatedEventLog {
    pub events: Vec<Event>,
    pub subject: String,
    pub evidence_ref: String,
    pub witness: WitnessState,
}

impl GraduateToWasm4pm for GraduatedEventLog {
    type Target = Vec<Event>;
    type Witness = WitnessState;

    fn candidate(&self) -> GraduationCandidate<Self::Target, Self::Witness> {
        GraduationCandidate::new(
            GraduationReason::NeedsDiscovery,
            self.subject.clone(),
            self.evidence_ref.clone(),
            self.witness.clone(),
        )
    }
}

/// A wrapper for the conformance verdicts that implements the GraduateToWasm4pm trait.
pub struct GraduatedConformance {
    pub verdicts: ConformanceVerdicts,
    pub subject: String,
    pub evidence_ref: String,
    pub witness: WitnessState,
}

impl GraduateToWasm4pm for GraduatedConformance {
    type Target = ConformanceVerdicts;
    type Witness = WitnessState;

    fn candidate(&self) -> GraduationCandidate<Self::Target, Self::Witness> {
        GraduationCandidate::new(
            GraduationReason::NeedsConformanceExecution,
            self.subject.clone(),
            self.evidence_ref.clone(),
            self.witness.clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_intake() {
        let candidate: GraduationCandidate<(), WitnessState> = GraduationCandidate::new(
            GraduationReason::NeedsDiscovery,
            "Event log candidate".to_string(),
            "blake3:somehash".to_string(),
            WitnessState::Bottom,
        );

        let res = validate_engine_intake(&candidate, GraduationReason::NeedsDiscovery);
        assert!(res.is_ok());
    }

    #[test]
    fn test_ungrounded_intake() {
        // Empty subject
        let candidate: GraduationCandidate<(), WitnessState> = GraduationCandidate::new(
            GraduationReason::NeedsDiscovery,
            "".to_string(),
            "blake3:somehash".to_string(),
            WitnessState::Bottom,
        );
        let res = validate_engine_intake(&candidate, GraduationReason::NeedsDiscovery);
        assert!(matches!(res, Err(IngestionError::UngroundedCandidate { .. })));

        // Empty evidence_ref
        let candidate2: GraduationCandidate<(), WitnessState> = GraduationCandidate::new(
            GraduationReason::NeedsDiscovery,
            "Valid Subject".to_string(),
            "".to_string(),
            WitnessState::Bottom,
        );
        let res2 = validate_engine_intake(&candidate2, GraduationReason::NeedsDiscovery);
        assert!(matches!(res2, Err(IngestionError::UngroundedCandidate { .. })));
    }

    #[test]
    fn test_reason_mismatch() {
        let candidate: GraduationCandidate<(), WitnessState> = GraduationCandidate::new(
            GraduationReason::NeedsDiscovery,
            "Event log candidate".to_string(),
            "blake3:somehash".to_string(),
            WitnessState::Bottom,
        );
        let res = validate_engine_intake(&candidate, GraduationReason::NeedsConformanceExecution);
        assert!(matches!(res, Err(IngestionError::ReasonMismatch { .. })));
    }

    #[test]
    fn test_invalid_witness_state() {
        // NeedsDiscovery is a hard signal, so Top witness is invalid
        let candidate: GraduationCandidate<(), WitnessState> = GraduationCandidate::new(
            GraduationReason::NeedsDiscovery,
            "Event log candidate".to_string(),
            "blake3:somehash".to_string(),
            WitnessState::Top,
        );
        let res = validate_engine_intake(&candidate, GraduationReason::NeedsDiscovery);
        assert!(matches!(res, Err(IngestionError::InvalidWitnessState)));
    }

    #[test]
    fn test_accept_from_compat_success() {
        let candidate: GraduationCandidate<(), WitnessState> = GraduationCandidate::new(
            GraduationReason::NeedsDiscovery,
            "Event log candidate".to_string(),
            "blake3:somehash".to_string(),
            WitnessState::Bottom,
        );
        let res = accept_from_compat(&candidate, GraduationReason::NeedsDiscovery);
        assert!(res.is_ok());
    }

    #[test]
    fn test_accept_from_compat_ungrounded() {
        let candidate: GraduationCandidate<(), WitnessState> = GraduationCandidate::new(
            GraduationReason::NeedsDiscovery,
            "".to_string(),
            "blake3:somehash".to_string(),
            WitnessState::Bottom,
        );
        let res = accept_from_compat(&candidate, GraduationReason::NeedsDiscovery);
        assert!(matches!(res, Err(IngestionError::UngroundedCandidate { .. })));
    }

    #[test]
    fn test_accept_from_compat_reason_mismatch() {
        let candidate: GraduationCandidate<(), WitnessState> = GraduationCandidate::new(
            GraduationReason::NeedsConformanceExecution,
            "Event log candidate".to_string(),
            "blake3:somehash".to_string(),
            WitnessState::Bottom,
        );
        let res = accept_from_compat(&candidate, GraduationReason::NeedsDiscovery);
        assert!(matches!(res, Err(IngestionError::ReasonMismatch { .. })));
    }
}
