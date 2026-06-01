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
