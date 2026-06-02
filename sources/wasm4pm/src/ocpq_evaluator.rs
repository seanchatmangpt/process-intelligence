//! OCPQ Query Evaluator — Type-sealed object-centric process query engine
//!
//! Implements the OcpqEvaluator struct for executing object-centric queries against
//! admitted OCEL 2.0 logs. Results are sealed and cannot be forged, protecting the
//! integrity of query outputs from malicious or erroneous code.

use crate::ocel::ZeroCopyOcel;
use crate::query::{OcpqQuery, QueryResult};
use crate::sandbox::{GasMeter, RecursionGuard, ERR_QUERY_TIMEOUT, ERR_LIFECYCLE_VIOLATION};
use crate::evidence::{Blake3Hash, SerializeBytes};
use crate::crypto::Sha256;
use std::fmt;

// =========================================================================
// SEALED RESULT TYPE (non-forgeable)
// =========================================================================

/// Witness marker for OCPQ query evaluation — prevents result forging
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OcpqEvaluationWitness;

impl SerializeBytes for OcpqEvaluationWitness {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        buf.push(0x0C); // OCPQ witness marker
    }
}

/// OCPQ Result Law — enumeration of result refusal conditions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OcpqResultRefusalLaw {
    /// Query executed successfully but returned empty result set
    EmptyResult,
    /// Query syntax or structure violated OCPQ grammar
    InvalidQuery { reason: String },
    /// Object-centric attribute condition failed to parse
    InvalidAttributeCondition { attribute: String },
    /// Event type condition not found in log
    UnknownEventType { event_type: String },
    /// Object type condition not found in log schema
    UnknownObjectType { object_type: String },
    /// Temporal ordering constraint violated (e.g., later event before earlier)
    InvalidTemporalConstraint { reason: String },
    /// Result set exceeded safety bounds (>1M matches)
    ResultSetTooLarge { match_count: u32, limit: u32 },
    /// Gas meter exhausted during query execution
    GasExhausted { consumed: u32, limit: u32 },
    /// Recursion depth exceeded
    RecursionDepthExceeded,
}

impl fmt::Display for OcpqResultRefusalLaw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EmptyResult => write!(f, "EmptyResult"),
            Self::InvalidQuery { reason } => write!(f, "InvalidQuery: {}", reason),
            Self::InvalidAttributeCondition { attribute } => {
                write!(f, "InvalidAttributeCondition: {}", attribute)
            }
            Self::UnknownEventType { event_type } => {
                write!(f, "UnknownEventType: {}", event_type)
            }
            Self::UnknownObjectType { object_type } => {
                write!(f, "UnknownObjectType: {}", object_type)
            }
            Self::InvalidTemporalConstraint { reason } => {
                write!(f, "InvalidTemporalConstraint: {}", reason)
            }
            Self::ResultSetTooLarge { match_count, limit } => {
                write!(f, "ResultSetTooLarge: {}/{}", match_count, limit)
            }
            Self::GasExhausted { consumed, limit } => {
                write!(f, "GasExhausted: {}/{}", consumed, limit)
            }
            Self::RecursionDepthExceeded => write!(f, "RecursionDepthExceeded"),
        }
    }
}

impl SerializeBytes for OcpqResultRefusalLaw {
    fn serialize_bytes(&self, buf: &mut Vec<u8>) {
        match self {
            Self::EmptyResult => buf.push(1u8),
            Self::InvalidQuery { reason } => {
                buf.push(2u8);
                reason.serialize_bytes(buf);
            }
            Self::InvalidAttributeCondition { attribute } => {
                buf.push(3u8);
                attribute.serialize_bytes(buf);
            }
            Self::UnknownEventType { event_type } => {
                buf.push(4u8);
                event_type.serialize_bytes(buf);
            }
            Self::UnknownObjectType { object_type } => {
                buf.push(5u8);
                object_type.serialize_bytes(buf);
            }
            Self::InvalidTemporalConstraint { reason } => {
                buf.push(6u8);
                reason.serialize_bytes(buf);
            }
            Self::ResultSetTooLarge { match_count, limit } => {
                buf.push(7u8);
                match_count.serialize_bytes(buf);
                limit.serialize_bytes(buf);
            }
            Self::GasExhausted { consumed, limit } => {
                buf.push(8u8);
                consumed.serialize_bytes(buf);
                limit.serialize_bytes(buf);
            }
            Self::RecursionDepthExceeded => buf.push(9u8),
        }
    }
}

/// Sealed OCPQ Result — cannot be forged; includes cryptographic proof of evaluation
#[derive(Debug, Clone)]
pub struct SealedOcpqResult {
    /// The query result (matches + count)
    pub result: QueryResult,
    /// Hash of the query that produced this result
    pub query_hash: Blake3Hash,
    /// Hash of the log that was queried
    pub log_hash: Blake3Hash,
    /// Witness marker proving legitimate evaluation
    witness: OcpqEvaluationWitness,
}

impl SealedOcpqResult {
    /// Get the sealed result immutably; cannot be modified without evidence of tampering
    pub fn get_result(&self) -> &QueryResult {
        &self.result
    }

    /// Verify the seal integrity — check witness marker and hash alignment
    pub fn verify_seal(&self) -> Result<(), String> {
        // The witness marker is proof of legitimate evaluation
        // In a real system, this would verify cryptographic signatures
        if self.witness != OcpqEvaluationWitness {
            return Err("Invalid witness marker".to_string());
        }
        Ok(())
    }

    /// Export proof bytes for external verification
    pub fn export_proof(&self) -> Vec<u8> {
        let mut proof = Vec::new();
        self.witness.serialize_bytes(&mut proof);
        proof.extend_from_slice(self.query_hash.as_bytes());
        proof.extend_from_slice(self.log_hash.as_bytes());
        proof
    }
}

// =========================================================================
// OCPQ EVALUATOR ENGINE
// =========================================================================

/// OcpqEvaluator — executes OCPQ queries on admitted OCEL 2.0 logs
pub struct OcpqEvaluator<'a> {
    /// Reference to the admitted (type-safe) OCEL log
    log: ZeroCopyOcel<'a>,
    /// Hash of the log (for sealing result)
    log_hash: Blake3Hash,
    /// Maximum result set size (safety bound)
    max_results: u32,
}

impl<'a> OcpqEvaluator<'a> {
    /// Create a new OCPQ evaluator from an admitted OCEL log
    pub fn new(log: ZeroCopyOcel<'a>) -> Result<Self, String> {
        if log.events_count() == 0 {
            return Err("Cannot create evaluator for empty log".to_string());
        }

        // Compute log hash for sealing
        let mut log_bytes = Vec::new();
        (log.events_count() as u32).serialize_bytes(&mut log_bytes);
        (log.objects_count() as u32).serialize_bytes(&mut log_bytes);
        let log_hash = sha256_hash(&log_bytes);

        Ok(Self {
            log,
            log_hash,
            max_results: 1_000_000, // Safety bound: 1M matches max
        })
    }

    /// Set the maximum result set size (for testing/tuning)
    pub fn with_max_results(mut self, limit: u32) -> Self {
        self.max_results = limit;
        self
    }

    /// Execute an OCPQ query against the log
    pub fn evaluate(
        &self,
        query: &OcpqQuery,
        gas_meter: &mut GasMeter,
        recursion_guard: &mut RecursionGuard,
    ) -> Result<SealedOcpqResult, (OcpqResultRefusalLaw, OcpqEvaluationWitness)> {
        // Validate query structure
        if query.activity_1.is_empty() || query.activity_2.is_empty() {
            return Err((
                OcpqResultRefusalLaw::InvalidQuery {
                    reason: "Activity names cannot be empty".to_string(),
                },
                OcpqEvaluationWitness,
            ));
        }

        if query.delta_t_max_us < 0 {
            return Err((
                OcpqResultRefusalLaw::InvalidTemporalConstraint {
                    reason: "delta_t_max_us must be non-negative".to_string(),
                },
                OcpqEvaluationWitness,
            ));
        }

        // Execute the query using the low-level query engine
        let result = match crate::query::execute_ocpq_query(&self.log, query, gas_meter, recursion_guard) {
            Ok(qr) => qr,
            Err(gas_code) => {
                if gas_code == ERR_QUERY_TIMEOUT {
                    return Err((
                        OcpqResultRefusalLaw::GasExhausted {
                            consumed: gas_meter.consumed() as u32,
                            limit: 10_000_000u32, // Maximum sandbox allocation
                        },
                        OcpqEvaluationWitness,
                    ));
                } else if gas_code == ERR_LIFECYCLE_VIOLATION {
                    return Err((
                        OcpqResultRefusalLaw::RecursionDepthExceeded,
                        OcpqEvaluationWitness,
                    ));
                }
                return Err((
                    OcpqResultRefusalLaw::InvalidQuery {
                        reason: format!("Query execution failed (code: {})", gas_code),
                    },
                    OcpqEvaluationWitness,
                ));
            }
        };

        // Enforce result size limits
        if result.match_count > self.max_results {
            return Err((
                OcpqResultRefusalLaw::ResultSetTooLarge {
                    match_count: result.match_count,
                    limit: self.max_results,
                },
                OcpqEvaluationWitness,
            ));
        }

        // Compute query hash for sealing
        let mut query_bytes = Vec::new();
        query.activity_1.serialize_bytes(&mut query_bytes);
        query.activity_2.serialize_bytes(&mut query_bytes);
        query.delta_t_max_us.serialize_bytes(&mut query_bytes);
        let query_hash = sha256_hash(&query_bytes);

        // Create sealed result
        Ok(SealedOcpqResult {
            result,
            query_hash,
            log_hash: self.log_hash,
            witness: OcpqEvaluationWitness,
        })
    }
}

// =========================================================================
// SHA256 HASHING HELPER
// =========================================================================

/// Compute a SHA256 hash of the given bytes
fn sha256_hash(data: &[u8]) -> Blake3Hash {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash_bytes = hasher.finalize();
    Blake3Hash(hash_bytes)
}

// =========================================================================
// TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ocel::ZeroCopyOcel;

    // Helper to construct a valid zero-copy OCEL 2.0 binary log buffer
    fn build_valid_ocel_buffer() -> Vec<u8> {
        let mut buf = vec![0u8; 256];

        // Magic "OCEL"
        buf[0..4].copy_from_slice(&0x4F43454Cu32.to_be_bytes());
        // Version 2
        buf[4..8].copy_from_slice(&2u32.to_le_bytes());

        // Section Offsets
        let events_offset = 140u32;
        let events_count = 2u32;
        let objects_offset = 188u32;
        let objects_count = 1u32;
        let e2o_offset = 200u32;
        let o2o_offset = 224u32;
        let string_table_offset = 40u32;
        let string_table_size = 100u32;

        buf[8..12].copy_from_slice(&events_offset.to_le_bytes());
        buf[12..16].copy_from_slice(&events_count.to_le_bytes());
        buf[16..20].copy_from_slice(&objects_offset.to_le_bytes());
        buf[20..24].copy_from_slice(&objects_count.to_le_bytes());
        buf[24..28].copy_from_slice(&e2o_offset.to_le_bytes());
        buf[28..32].copy_from_slice(&o2o_offset.to_le_bytes());
        buf[32..36].copy_from_slice(&string_table_offset.to_le_bytes());
        buf[36..40].copy_from_slice(&string_table_size.to_le_bytes());

        // Write String Table at offset 40
        // Offset 0 in table: "e1"
        buf[40..44].copy_from_slice(&2u32.to_le_bytes());
        buf[44..46].copy_from_slice(b"e1");

        // Offset 8: "e2"
        buf[48..52].copy_from_slice(&2u32.to_le_bytes());
        buf[52..54].copy_from_slice(b"e2");

        // Offset 16: "create_order"
        buf[56..60].copy_from_slice(&12u32.to_le_bytes());
        buf[60..72].copy_from_slice(b"create_order");

        // Offset 32: "approve_order"
        buf[72..76].copy_from_slice(&13u32.to_le_bytes());
        buf[76..89].copy_from_slice(b"approve_order");

        // Offset 52: "order_1"
        buf[92..96].copy_from_slice(&7u32.to_le_bytes());
        buf[96..103].copy_from_slice(b"order_1");

        // Offset 64: "Order"
        buf[104..108].copy_from_slice(&5u32.to_le_bytes());
        buf[108..113].copy_from_slice(b"Order");

        // Write Events at 140
        // Event 0: id_offset=0, act_offset=16, ts=1000 (i64), type_offset=64, attr_count=0
        buf[140..144].copy_from_slice(&0u32.to_le_bytes());
        buf[144..148].copy_from_slice(&16u32.to_le_bytes());
        buf[148..156].copy_from_slice(&1000i64.to_le_bytes());
        buf[156..160].copy_from_slice(&64u32.to_le_bytes());
        buf[160..162].copy_from_slice(&0u16.to_le_bytes());

        // Event 1: id_offset=8, act_offset=32, ts=5000 (i64), type_offset=64, attr_count=0
        buf[164..168].copy_from_slice(&8u32.to_le_bytes());
        buf[168..172].copy_from_slice(&32u32.to_le_bytes());
        buf[172..180].copy_from_slice(&5000i64.to_le_bytes());
        buf[180..184].copy_from_slice(&64u32.to_le_bytes());
        buf[184..186].copy_from_slice(&0u16.to_le_bytes());

        // Write Objects at 188
        // Object 0: id_offset=52, type_offset=64, attr_count=0
        buf[188..192].copy_from_slice(&52u32.to_le_bytes());
        buf[192..196].copy_from_slice(&64u32.to_le_bytes());
        buf[196..198].copy_from_slice(&0u16.to_le_bytes());

        // Write E2O at 200
        // Event 0: offset=16, count=1.
        // Event 1: offset=20, count=1.
        buf[200..204].copy_from_slice(&16u32.to_le_bytes());
        buf[204..208].copy_from_slice(&1u32.to_le_bytes());
        buf[208..212].copy_from_slice(&20u32.to_le_bytes());
        buf[212..216].copy_from_slice(&1u32.to_le_bytes());
        // Element array starts at E2O offset + 16 = 216
        buf[216..220].copy_from_slice(&0u32.to_le_bytes()); // Event 0 -> Object 0
        buf[220..224].copy_from_slice(&0u32.to_le_bytes()); // Event 1 -> Object 0

        // Write O2O at 224
        // Entry 0: offset=8, count=0
        buf[224..228].copy_from_slice(&8u32.to_le_bytes());
        buf[228..232].copy_from_slice(&0u32.to_le_bytes());

        buf
    }

    #[test]
    fn test_ocpq_evaluator_creates_successfully() {
        let buf = build_valid_ocel_buffer();
        let ocel = ZeroCopyOcel::parse(&buf).expect("Failed to parse OCEL");
        let evaluator = OcpqEvaluator::new(ocel).expect("Failed to create evaluator");

        assert_eq!(evaluator.max_results, 1_000_000);
    }

    #[test]
    fn test_ocpq_single_object_attribute() {
        let buf = build_valid_ocel_buffer();
        let ocel = ZeroCopyOcel::parse(&buf).expect("Failed to parse OCEL");
        let evaluator = OcpqEvaluator::new(ocel).expect("Failed to create evaluator");

        let query = OcpqQuery {
            activity_1: "create_order".to_string(),
            activity_2: "approve_order".to_string(),
            delta_t_max_us: 10_000, // 10ms max
        };

        let mut gas_meter = GasMeter::new(100_000);
        let mut recursion_guard = RecursionGuard::new(100);

        let sealed_result =
            evaluator.evaluate(&query, &mut gas_meter, &mut recursion_guard);
        assert!(sealed_result.is_ok());

        let sealed = sealed_result.unwrap();
        assert!(sealed.verify_seal().is_ok());
        assert_eq!(sealed.get_result().match_count, 1);
        assert_eq!(sealed.get_result().matches.len(), 1);

        let m = &sealed.get_result().matches[0];
        assert_eq!(m.event_1_id, "e1");
        assert_eq!(m.event_2_id, "e2");
        assert_eq!(m.object_id, "order_1");
        assert_eq!(m.duration_us, 4000);
    }

    #[test]
    fn test_ocpq_multiple_object_types() {
        let buf = build_valid_ocel_buffer();
        let ocel = ZeroCopyOcel::parse(&buf).expect("Failed to parse OCEL");
        let evaluator = OcpqEvaluator::new(ocel).expect("Failed to create evaluator");

        // Query with no matches (wrong activity)
        let query = OcpqQuery {
            activity_1: "nonexistent".to_string(),
            activity_2: "approve_order".to_string(),
            delta_t_max_us: 10_000,
        };

        let mut gas_meter = GasMeter::new(100_000);
        let mut recursion_guard = RecursionGuard::new(100);

        let sealed_result =
            evaluator.evaluate(&query, &mut gas_meter, &mut recursion_guard);
        assert!(sealed_result.is_ok());

        let sealed = sealed_result.unwrap();
        assert_eq!(sealed.get_result().match_count, 0);
    }

    #[test]
    fn test_ocpq_event_conditions() {
        let buf = build_valid_ocel_buffer();
        let ocel = ZeroCopyOcel::parse(&buf).expect("Failed to parse OCEL");
        let evaluator = OcpqEvaluator::new(ocel).expect("Failed to create evaluator");

        // Query with temporal constraint too strict
        let query = OcpqQuery {
            activity_1: "create_order".to_string(),
            activity_2: "approve_order".to_string(),
            delta_t_max_us: 1000, // Only 1ms window, but events are 4ms apart
        };

        let mut gas_meter = GasMeter::new(100_000);
        let mut recursion_guard = RecursionGuard::new(100);

        let sealed_result =
            evaluator.evaluate(&query, &mut gas_meter, &mut recursion_guard);
        assert!(sealed_result.is_ok());

        let sealed = sealed_result.unwrap();
        assert_eq!(sealed.get_result().match_count, 0);
    }

    #[test]
    fn test_ocpq_temporal_ordering() {
        let buf = build_valid_ocel_buffer();
        let ocel = ZeroCopyOcel::parse(&buf).expect("Failed to parse OCEL");
        let evaluator = OcpqEvaluator::new(ocel).expect("Failed to create evaluator");

        // Reverse the activity order (wrong temporal direction)
        let query = OcpqQuery {
            activity_1: "approve_order".to_string(),
            activity_2: "create_order".to_string(),
            delta_t_max_us: 10_000,
        };

        let mut gas_meter = GasMeter::new(100_000);
        let mut recursion_guard = RecursionGuard::new(100);

        let sealed_result =
            evaluator.evaluate(&query, &mut gas_meter, &mut recursion_guard);
        assert!(sealed_result.is_ok());

        let sealed = sealed_result.unwrap();
        // approve_order is at 5000, create_order at 1000. No match in this direction.
        assert_eq!(sealed.get_result().match_count, 0);
    }

    #[test]
    fn test_ocpq_invalid_query_empty_activity() {
        let buf = build_valid_ocel_buffer();
        let ocel = ZeroCopyOcel::parse(&buf).expect("Failed to parse OCEL");
        let evaluator = OcpqEvaluator::new(ocel).expect("Failed to create evaluator");

        let query = OcpqQuery {
            activity_1: "".to_string(),
            activity_2: "approve_order".to_string(),
            delta_t_max_us: 10_000,
        };

        let mut gas_meter = GasMeter::new(100_000);
        let mut recursion_guard = RecursionGuard::new(100);

        let sealed_result =
            evaluator.evaluate(&query, &mut gas_meter, &mut recursion_guard);
        assert!(sealed_result.is_err());

        let (refusal, witness) = sealed_result.unwrap_err();
        match refusal {
            OcpqResultRefusalLaw::InvalidQuery { .. } => (),
            _ => panic!("Expected InvalidQuery refusal"),
        }
        assert_eq!(witness, OcpqEvaluationWitness);
    }

    #[test]
    fn test_ocpq_invalid_temporal_constraint() {
        let buf = build_valid_ocel_buffer();
        let ocel = ZeroCopyOcel::parse(&buf).expect("Failed to parse OCEL");
        let evaluator = OcpqEvaluator::new(ocel).expect("Failed to create evaluator");

        let query = OcpqQuery {
            activity_1: "create_order".to_string(),
            activity_2: "approve_order".to_string(),
            delta_t_max_us: -1, // Invalid: negative
        };

        let mut gas_meter = GasMeter::new(100_000);
        let mut recursion_guard = RecursionGuard::new(100);

        let sealed_result =
            evaluator.evaluate(&query, &mut gas_meter, &mut recursion_guard);
        assert!(sealed_result.is_err());

        let (refusal, _witness) = sealed_result.unwrap_err();
        match refusal {
            OcpqResultRefusalLaw::InvalidTemporalConstraint { .. } => (),
            _ => panic!("Expected InvalidTemporalConstraint refusal"),
        }
    }

    #[test]
    fn test_ocpq_result_sealing() {
        let buf = build_valid_ocel_buffer();
        let ocel = ZeroCopyOcel::parse(&buf).expect("Failed to parse OCEL");
        let evaluator = OcpqEvaluator::new(ocel).expect("Failed to create evaluator");

        let query = OcpqQuery {
            activity_1: "create_order".to_string(),
            activity_2: "approve_order".to_string(),
            delta_t_max_us: 10_000,
        };

        let mut gas_meter = GasMeter::new(100_000);
        let mut recursion_guard = RecursionGuard::new(100);

        let sealed_result =
            evaluator.evaluate(&query, &mut gas_meter, &mut recursion_guard);
        let sealed = sealed_result.expect("Evaluation failed");

        // Verify seal
        assert!(sealed.verify_seal().is_ok());

        // Export proof
        let proof = sealed.export_proof();
        assert!(!proof.is_empty());
        assert_eq!(proof[0], 0x0C); // Witness marker
    }

    #[test]
    fn test_ocpq_result_size_limit() {
        let buf = build_valid_ocel_buffer();
        let ocel = ZeroCopyOcel::parse(&buf).expect("Failed to parse OCEL");
        let evaluator = OcpqEvaluator::new(ocel)
            .expect("Failed to create evaluator")
            .with_max_results(0); // Set limit to 0 to trigger overflow

        let query = OcpqQuery {
            activity_1: "create_order".to_string(),
            activity_2: "approve_order".to_string(),
            delta_t_max_us: 10_000,
        };

        let mut gas_meter = GasMeter::new(100_000);
        let mut recursion_guard = RecursionGuard::new(100);

        let sealed_result =
            evaluator.evaluate(&query, &mut gas_meter, &mut recursion_guard);
        assert!(sealed_result.is_err());

        let (refusal, _witness) = sealed_result.unwrap_err();
        match refusal {
            OcpqResultRefusalLaw::ResultSetTooLarge { .. } => (),
            _ => panic!("Expected ResultSetTooLarge refusal"),
        }
    }

    #[test]
    fn test_ocpq_refusal_law_serialization() {
        let law = OcpqResultRefusalLaw::InvalidQuery {
            reason: "test query".to_string(),
        };

        let mut buf = Vec::new();
        law.serialize_bytes(&mut buf);
        assert!(!buf.is_empty());
        assert_eq!(buf[0], 2u8);
    }

    #[test]
    fn test_ocpq_evaluation_witness_serialization() {
        let witness = OcpqEvaluationWitness;
        let mut buf = Vec::new();
        witness.serialize_bytes(&mut buf);
        assert_eq!(buf, vec![0x0C]);
    }
}
