//! Named Law Refusals module — Type-law-compliant error types with structured witness evidence.
//!
//! Replaces generic ValidationError(String) with named Refusal<R, W> types, where:
//! - R: The specific refusal law/reason
//! - W: The witness type carrying evidence of the refusal condition
//!
//! Each refusal type carries contextual data enabling deterministic process mining replay.

use crate::evidence::SerializeBytes;
use std::fmt;

// =========================================================================
// CONFORMANCE REFUSAL LAWS
// =========================================================================

/// Reasons a conformance check may be refused — named law hierarchy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConformanceRefusalLaw {
    /// Event log is empty — no cases to replay
    EmptyLog,
    /// Petri Net has no transitions — cannot fire
    EmptyModel,
    /// Net is not sound — no lawful firing sequences exist
    UnsoundNet {
        reason: String,
    },
    /// Activity in case is not a valid transition
    UnknownActivity {
        activity_name: String,
        available_transitions: Vec<String>,
    },
    /// Token replay exhausted early — cannot continue
    EarlyTermination {
        at_event: usize,
        total_events: usize,
        missing_tokens: usize,
    },
    /// State space explosion — alignment search exceeded bounds
    StateSpaceExceeded {
        threshold: usize,
        current_size: usize,
    },
    /// Case sequence is malformed
    MalformedCase {
        reason: String,
        case_id: String,
    },
}

impl fmt::Display for ConformanceRefusalLaw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EmptyLog => write!(f, "EmptyLog"),
            Self::EmptyModel => write!(f, "EmptyModel"),
            Self::UnsoundNet { reason } => write!(f, "UnsoundNet: {}", reason),
            Self::UnknownActivity { activity_name, .. } => {
                write!(f, "UnknownActivity: {}", activity_name)
            }
            Self::EarlyTermination {
                at_event,
                total_events,
                ..
            } => {
                write!(f, "EarlyTermination at event {}/{}", at_event, total_events)
            }
            Self::StateSpaceExceeded {
                threshold,
                current_size,
            } => {
                write!(f, "StateSpaceExceeded ({}/{})", current_size, threshold)
            }
            Self::MalformedCase { reason, case_id } => {
                write!(f, "MalformedCase({}): {}", case_id, reason)
            }
        }
    }
}

impl SerializeBytes for ConformanceRefusalLaw {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        match self {
            Self::EmptyLog => buf.push(1u8),
            Self::EmptyModel => buf.push(2u8),
            Self::UnsoundNet { reason } => {
                buf.push(3u8);
                reason.serialize_bytes(buf);
            }
            Self::UnknownActivity {
                activity_name,
                available_transitions,
            } => {
                buf.push(4u8);
                activity_name.serialize_bytes(buf);
                (available_transitions.len() as u32).serialize_bytes(buf);
                for t in available_transitions {
                    t.serialize_bytes(buf);
                }
            }
            Self::EarlyTermination {
                at_event,
                total_events,
                missing_tokens,
            } => {
                buf.push(5u8);
                (*at_event as u64).serialize_bytes(buf);
                (*total_events as u64).serialize_bytes(buf);
                (*missing_tokens as u64).serialize_bytes(buf);
            }
            Self::StateSpaceExceeded {
                threshold,
                current_size,
            } => {
                buf.push(6u8);
                (*threshold as u64).serialize_bytes(buf);
                (*current_size as u64).serialize_bytes(buf);
            }
            Self::MalformedCase { reason, case_id } => {
                buf.push(7u8);
                reason.serialize_bytes(buf);
                case_id.serialize_bytes(buf);
            }
        }
    }
}

// =========================================================================
// OCEL PARSING REFUSAL LAWS
// =========================================================================

/// Reasons an OCEL parse may be refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OcelRefusalLaw {
    /// Magic number mismatch — not a valid OCEL file
    InvalidMagic { found: u32, expected: u32 },
    /// Version not supported
    InvalidVersion { found: u32, supported: Vec<u32> },
    /// Read offset exceeds buffer bounds
    OutOfBounds { offset: u32, size: usize },
    /// UTF-8 decoding failed in string table
    Utf8Error {
        offset: u32,
        byte_sequence: Vec<u8>,
    },
    /// Null pointer in critical location
    NullPointer { location: String },
    /// Reference to non-existent object or event
    DanglingReference {
        ref_type: String,
        ref_id: u64,
        max_valid_id: u64,
    },
    /// Event-object graph has cycles
    CycleDetected {
        cycle_origin: u64,
        cycle_members: Vec<u64>,
    },
    /// Temporal ordering violation
    TemporalAnomaly {
        event_id: u64,
        timestamp_ns: i64,
        causality_violation_with: u64,
    },
    /// Unknown object type in schema
    UnknownObjectType { object_type: String },
}

impl fmt::Display for OcelRefusalLaw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidMagic { found, expected } => {
                write!(f, "InvalidMagic: found 0x{:08x}, expected 0x{:08x}", found, expected)
            }
            Self::InvalidVersion { found, .. } => write!(f, "InvalidVersion: {}", found),
            Self::OutOfBounds { offset, size } => {
                write!(f, "OutOfBounds: offset {} exceeds buffer {}", offset, size)
            }
            Self::Utf8Error { offset, .. } => write!(f, "Utf8Error at offset {}", offset),
            Self::NullPointer { location } => write!(f, "NullPointer: {}", location),
            Self::DanglingReference {
                ref_type,
                ref_id,
                max_valid_id,
            } => {
                write!(
                    f,
                    "DanglingReference: {} id {} exceeds max {}",
                    ref_type, ref_id, max_valid_id
                )
            }
            Self::CycleDetected { cycle_origin, .. } => {
                write!(f, "CycleDetected: origin {}", cycle_origin)
            }
            Self::TemporalAnomaly { event_id, .. } => {
                write!(f, "TemporalAnomaly: event {}", event_id)
            }
            Self::UnknownObjectType { object_type } => {
                write!(f, "UnknownObjectType: {}", object_type)
            }
        }
    }
}

impl SerializeBytes for OcelRefusalLaw {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        match self {
            Self::InvalidMagic { found, expected } => {
                buf.push(1u8);
                found.serialize_bytes(buf);
                expected.serialize_bytes(buf);
            }
            Self::InvalidVersion { found, supported } => {
                buf.push(2u8);
                found.serialize_bytes(buf);
                (supported.len() as u32).serialize_bytes(buf);
                for v in supported {
                    v.serialize_bytes(buf);
                }
            }
            Self::OutOfBounds { offset, size } => {
                buf.push(3u8);
                offset.serialize_bytes(buf);
                (*size as u64).serialize_bytes(buf);
            }
            Self::Utf8Error {
                offset,
                byte_sequence,
            } => {
                buf.push(4u8);
                offset.serialize_bytes(buf);
                (byte_sequence.len() as u32).serialize_bytes(buf);
                byte_sequence.serialize_bytes(buf);
            }
            Self::NullPointer { location } => {
                buf.push(5u8);
                location.serialize_bytes(buf);
            }
            Self::DanglingReference {
                ref_type,
                ref_id,
                max_valid_id,
            } => {
                buf.push(6u8);
                ref_type.serialize_bytes(buf);
                ref_id.serialize_bytes(buf);
                max_valid_id.serialize_bytes(buf);
            }
            Self::CycleDetected { cycle_origin, cycle_members } => {
                buf.push(7u8);
                cycle_origin.serialize_bytes(buf);
                (cycle_members.len() as u32).serialize_bytes(buf);
                for m in cycle_members {
                    m.serialize_bytes(buf);
                }
            }
            Self::TemporalAnomaly {
                event_id,
                timestamp_ns,
                causality_violation_with,
            } => {
                buf.push(8u8);
                event_id.serialize_bytes(buf);
                (*timestamp_ns as u64).serialize_bytes(buf);
                causality_violation_with.serialize_bytes(buf);
            }
            Self::UnknownObjectType { object_type } => {
                buf.push(9u8);
                object_type.serialize_bytes(buf);
            }
        }
    }
}

// =========================================================================
// WITNESS TYPE MARKERS
// =========================================================================

/// Witness marker for conformance checking algorithm
#[derive(Debug, Clone, Copy)]
pub struct ConformanceWitness;

impl SerializeBytes for ConformanceWitness {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        buf.push(0xCC);
    }
}

/// Witness marker for OCEL parsing
#[derive(Debug, Clone, Copy)]
pub struct OcelParsingWitness;

impl SerializeBytes for OcelParsingWitness {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        buf.push(0xACu8); // OCEL witness marker
    }
}

/// Witness marker for OTel parsing
#[derive(Debug, Clone, Copy)]
pub struct OtelParsingWitness;

impl SerializeBytes for OtelParsingWitness {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        buf.push(0xADu8); // OTel witness marker
    }
}

// =========================================================================
// ADMISSION REFUSAL
// =========================================================================

/// Generic refusal when admission fails
#[derive(Debug, Clone)]
pub enum AdmissionRefusalLaw {
    /// Type law covenant violated
    TypeLawViolated { covenant: String },
    /// Evidence proof gate failed
    ProofGateFailed { gate_name: String, reason: String },
}

impl fmt::Display for AdmissionRefusalLaw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TypeLawViolated { covenant } => write!(f, "TypeLawViolated: {}", covenant),
            Self::ProofGateFailed { gate_name, reason } => {
                write!(f, "ProofGateFailed({}): {}", gate_name, reason)
            }
        }
    }
}

impl SerializeBytes for AdmissionRefusalLaw {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        match self {
            Self::TypeLawViolated { covenant } => {
                buf.push(1u8);
                covenant.serialize_bytes(buf);
            }
            Self::ProofGateFailed { gate_name, reason } => {
                buf.push(2u8);
                gate_name.serialize_bytes(buf);
                reason.serialize_bytes(buf);
            }
        }
    }
}

// =========================================================================
// TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conformance_empty_log_refusal() {
        let law = ConformanceRefusalLaw::EmptyLog;
        assert_eq!(law.to_string(), "EmptyLog");

        let mut buf = Vec::new();
        law.serialize_bytes(&mut buf);
        assert!(!buf.is_empty());
    }

    #[test]
    fn test_conformance_unknown_activity_refusal() {
        let law = ConformanceRefusalLaw::UnknownActivity {
            activity_name: "BadActivity".to_string(),
            available_transitions: vec!["Task1".to_string(), "Task2".to_string()],
        };

        let display = law.to_string();
        assert!(display.contains("UnknownActivity"));
        assert!(display.contains("BadActivity"));

        let mut buf = Vec::new();
        law.serialize_bytes(&mut buf);
        assert!(buf.len() > 2);
    }

    #[test]
    fn test_conformance_early_termination_refusal() {
        let law = ConformanceRefusalLaw::EarlyTermination {
            at_event: 5,
            total_events: 10,
            missing_tokens: 3,
        };

        let display = law.to_string();
        assert!(display.contains("EarlyTermination"));
        assert!(display.contains("5/10"));

        let mut buf = Vec::new();
        law.serialize_bytes(&mut buf);
        assert!(buf.len() > 2);
    }

    #[test]
    fn test_conformance_state_space_exceeded_refusal() {
        let law = ConformanceRefusalLaw::StateSpaceExceeded {
            threshold: 5000,
            current_size: 5001,
        };

        let display = law.to_string();
        assert!(display.contains("StateSpaceExceeded"));
        assert!(display.contains("5001/5000"));
    }

    #[test]
    fn test_ocel_invalid_magic_refusal() {
        let law = OcelRefusalLaw::InvalidMagic {
            found: 0xDEADBEEF,
            expected: 0x4F43454C,
        };

        let display = law.to_string();
        assert!(display.contains("InvalidMagic"));

        let mut buf = Vec::new();
        law.serialize_bytes(&mut buf);
        assert!(buf.len() > 8);
    }

    #[test]
    fn test_ocel_dangling_reference_refusal() {
        let law = OcelRefusalLaw::DanglingReference {
            ref_type: "event".to_string(),
            ref_id: 999,
            max_valid_id: 100,
        };

        let display = law.to_string();
        assert!(display.contains("DanglingReference"));
        assert!(display.contains("999"));
        assert!(display.contains("100"));
    }

    #[test]
    fn test_ocel_cycle_detected_refusal() {
        let law = OcelRefusalLaw::CycleDetected {
            cycle_origin: 1,
            cycle_members: vec![1, 2, 3, 1],
        };

        let display = law.to_string();
        assert!(display.contains("CycleDetected"));
        assert!(display.contains("1"));
    }

    #[test]
    fn test_ocel_temporal_anomaly_refusal() {
        let law = OcelRefusalLaw::TemporalAnomaly {
            event_id: 42,
            timestamp_ns: 1000000,
            causality_violation_with: 43,
        };

        let display = law.to_string();
        assert!(display.contains("TemporalAnomaly"));
        assert!(display.contains("42"));
    }

    #[test]
    fn test_admission_type_law_violated_refusal() {
        let law = AdmissionRefusalLaw::TypeLawViolated {
            covenant: "witness_must_be_bottom".to_string(),
        };

        let display = law.to_string();
        assert!(display.contains("TypeLawViolated"));
        assert!(display.contains("witness_must_be_bottom"));

        let mut buf = Vec::new();
        law.serialize_bytes(&mut buf);
        assert!(!buf.is_empty());
    }

    #[test]
    fn test_admission_proof_gate_failed_refusal() {
        let law = AdmissionRefusalLaw::ProofGateFailed {
            gate_name: "signature_verification".to_string(),
            reason: "invalid_signature".to_string(),
        };

        let display = law.to_string();
        assert!(display.contains("ProofGateFailed"));
        assert!(display.contains("signature_verification"));
    }

    #[test]
    fn test_conformance_witness_marker_serialization() {
        let witness = ConformanceWitness;
        let mut buf = Vec::new();
        witness.serialize_bytes(&mut buf);
        assert_eq!(buf, vec![0xCC]);
    }

    #[test]
    fn test_ocel_witness_marker_serialization() {
        let witness = OcelParsingWitness;
        let mut buf = Vec::new();
        witness.serialize_bytes(&mut buf);
        assert_eq!(buf, vec![0xAC]);
    }

    #[test]
    fn test_otel_witness_marker_serialization() {
        let witness = OtelParsingWitness;
        let mut buf = Vec::new();
        witness.serialize_bytes(&mut buf);
        assert_eq!(buf, vec![0xAD]);
    }
}
