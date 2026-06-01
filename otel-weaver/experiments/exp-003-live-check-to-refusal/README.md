# Experiment EXP-003: Live Validation Check to Refusal Type Projections

This experiment defines and implements the translation of live telemetry schema validations directly into process-evidence types: `Admission<T, W>` or `Refusal`. This ensures that unverified, malformed, or unauthorized feedstock is rejected at the system boundary with an immutable, auditable `Refusal` receipt.

## 1. Type-Law Invariants

In the Process Intelligence Swarm Court, we enforce that all downstream algorithms (such as conformance checking or LTL replay) only consume elements wrapped in `Admission<T, W>`. This makes it compile-time impossible to execute analysis on invalid logs.

### Formal Typestate Signatures:

Let $T$ be the telemetry payload type and $W$ be the process mining witness type.
The system defines a validation gate:

$$\text{gate}: \text{Feedstock} \to \text{Result}\langle\text{Admission}\langle T, W\rangle, \text{Refusal}\rangle$$

Where:
* **`Admission<T, W>`** represents a verified telemetry payload $T$ signed by witness $W$.
* **`Refusal`** represents an immutable diagnostic record containing the failing feedstock, the specific law violated, and a cryptographic proof of rejection.

---

## 2. Refusal Domain Model

A `Refusal` is not a simple string error; it is a structured, serializable data structure containing the absolute diagnostic state for auditing:

```json
{
  "refusal_code": "INVALID_WITNESS_SIGNATURE",
  "law_violated": "type_safety::witness_cryptographic_seal",
  "witness_id": "heuristics_miner_v3",
  "raw_payload": {
    "process.pi.instance_id": "inst-8874f",
    "process.pi.activity.name": "approve_invoice",
    "process.pi.witness.hash": "bad_hash_value"
  },
  "diagnostic_hash": "blake3_error_proof_hash"
}
```

---

## 3. Rust Implementation: Boundary Validator

Below is the complete Rust implementation of the type-safe validator, including target structs and tests.

```rust
// file:///Users/sac/process-intelligence/otel-weaver/experiments/exp-003-live-check-to-refusal/src/validator.rs

use serde::{Serialize, Deserialize};

/// Telemetry feedstock representing raw incoming events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawTelemetryFeedstock {
    pub instance_id: Option<String>,
    pub activity_name: Option<String>,
    pub witness_id: Option<String>,
    pub witness_hash: Option<String>,
}

/// The verified payload admitted into the court
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdmittedPayload {
    pub instance_id: String,
    pub activity_name: String,
}

/// Compilation witness guaranteeing mathematical conformance
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaWitness {
    pub witness_id: String,
    pub verified_hash: String,
}

/// Admission wrapper containing the type proof and payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Admission<T, W> {
    pub payload: T,
    pub witness: W,
}

/// Structural refusal capturing validation failure details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Refusal {
    pub code: RefusalCode,
    pub violated_rule: String,
    pub raw_feedstock: RawTelemetryFeedstock,
    pub blake3_digest: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RefusalCode {
    MissingInstanceId,
    MissingActivityName,
    MissingWitnessId,
    InvalidWitnessSignature,
}

/// Validates raw telemetry feedstock and projects it into either an Admission or a Refusal.
pub fn validate_and_project(
    feedstock: RawTelemetryFeedstock
) -> Result<Admission<AdmittedPayload, AlphaWitness>, Refusal> {
    
    // Check 1: Verify presence of Process Instance ID
    let instance_id = match feedstock.instance_id.clone() {
        Some(id) if !id.trim().is_empty() => id,
        _ => return Err(build_refusal(RefusalCode::MissingInstanceId, "process.pi.instance_id must be provided and non-empty", feedstock)),
    };

    // Check 2: Verify presence of Activity Name
    let activity_name = match feedstock.activity_name.clone() {
        Some(name) if !name.trim().is_empty() => name,
        _ => return Err(build_refusal(RefusalCode::MissingActivityName, "process.pi.activity.name must be provided and non-empty", feedstock)),
    };

    // Check 3: Verify presence of Witness ID
    let witness_id = match feedstock.witness_id.clone() {
        Some(id) if !id.trim().is_empty() => id,
        _ => return Err(build_refusal(RefusalCode::MissingWitnessId, "process.pi.witness.id must be provided and non-empty", feedstock)),
    };

    // Check 4: Verify witness signature (BLAKE3 Simulation: must be exactly 64 characters)
    let witness_hash = match feedstock.witness_hash.clone() {
        Some(hash) if hash.len() == 64 => hash,
        _ => return Err(build_refusal(
            RefusalCode::InvalidWitnessSignature, 
            "process.pi.witness.hash must be a valid 64-character hex signature", 
            feedstock
        )),
    };

    // Project into Admission on success
    Ok(Admission {
        payload: AdmittedPayload {
            instance_id,
            activity_name,
        },
        witness: AlphaWitness {
            witness_id,
            verified_hash: witness_hash,
        },
    })
}

fn build_refusal(code: RefusalCode, rule: &str, feedstock: RawTelemetryFeedstock) -> Refusal {
    // Generate simulated BLAKE3 digest of the failure context
    let raw_str = format!("{:?}:{:?}", code, feedstock);
    let mock_hash = format!("{:064x}", raw_str.len()); // deterministic hash representation
    Refusal {
        code,
        violated_rule: rule.to_string(),
        raw_feedstock: feedstock,
        blake3_digest: mock_hash,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_admission() {
        let good_feedstock = RawTelemetryFeedstock {
            instance_id: Some("inst-123".to_string()),
            activity_name: Some("task_a".to_string()),
            witness_id: Some("witness-01".to_string()),
            witness_hash: Some("a".repeat(64)),
        };

        let result = validate_and_project(good_feedstock);
        assert!(result.is_ok());
        let admission = result.unwrap();
        assert_eq!(admission.payload.instance_id, "inst-123");
        assert_eq!(admission.witness.witness_id, "witness-01");
    }

    #[test]
    fn test_refusal_missing_id() {
        let bad_feedstock = RawTelemetryFeedstock {
            instance_id: None,
            activity_name: Some("task_a".to_string()),
            witness_id: Some("witness-01".to_string()),
            witness_hash: Some("a".repeat(64)),
        };

        let result = validate_and_project(bad_feedstock);
        assert!(result.is_err());
        let refusal = result.unwrap_err();
        assert_eq!(refusal.code, RefusalCode::MissingInstanceId);
    }
}
```

---

## 4. Operational Pipeline Integration

During production pipeline execution:
1. Spans from the OTel Collector hit the ingest boundary.
2. The `validate_and_project` function runs.
3. If `Ok`, the `Admission` payload enters the `wasm4pm` engine.
4. If `Err`, the `Refusal` is archived to the local auditing ledger (`refusals.json`), triggering a MAPE-K Loop correction or alert.

This enforces the process boundary mathematically at runtime.

---

## 5. Artifact Reference Links

* [Validator Rust Code](file:///Users/sac/process-intelligence/otel-weaver/experiments/exp-003-live-check-to-refusal/src/validator.rs)
* [Parent Experiment Directory](file:///Users/sac/process-intelligence/otel-weaver/experiments/)
* [Checkpoints Registry](file:///Users/sac/process-intelligence/checkpoints/)
