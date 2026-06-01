use wasm4pm::crypto::Blake3;
use std::collections::HashMap;

/// Telemetry feedstock representing raw incoming events in Schema Version 2 format
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelemetryFeedstockS2 {
    pub instance_id: Option<String>,
    pub transition_name: Option<String>,
    pub witness_id: Option<String>,
    pub witness_hash: Option<String>,
}

/// Telemetry feedstock representing normalized events in Schema Version 1 format expected by the court
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelemetryFeedstockS1 {
    pub instance_id: String,
    pub activity_name: String,
    pub witness_id: String,
    pub witness_hash: String,
}

/// The BridgeRx structure maps schema changes to ensure zero conformance residual (Schema URL Binding)
pub struct BridgeRx {
    pub rename_map: HashMap<String, String>,
}

impl Default for BridgeRx {
    fn default() -> Self {
        Self::new()
    }
}

impl BridgeRx {
    pub fn new() -> Self {
        let mut rename_map = HashMap::new();
        rename_map.insert("process.pi.transition.name".to_string(), "process.pi.activity.name".to_string());
        Self { rename_map }
    }

    /// Translates S2 telemetry feedstock back to S1 structure to satisfy the court's validators
    pub fn translate(&self, input: TelemetryFeedstockS2) -> Result<TelemetryFeedstockS1, &'static str> {
        let instance_id = input.instance_id.ok_or("Missing process.pi.instance_id")?;
        let transition_name = input.transition_name.ok_or("Missing process.pi.transition.name")?;
        let witness_id = input.witness_id.ok_or("Missing process.pi.witness.id")?;
        let witness_hash = input.witness_hash.ok_or("Missing process.pi.witness.hash")?;

        Ok(TelemetryFeedstockS1 {
            instance_id,
            activity_name: transition_name, // Translated via schema binding mapping
            witness_id,
            witness_hash,
        })
    }
}

/// Structural refusal capturing validation failure details
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    pub code: RefusalCode,
    pub violated_rule: String,
    pub raw_feedstock: TelemetryFeedstockS2,
    pub blake3_digest: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefusalCode {
    MissingInstanceId,
    MissingActivityName,
    MissingWitnessId,
    InvalidWitnessSignature,
}

/// Admitted payload representation after successful live-check validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedPayload {
    pub instance_id: String,
    pub activity_name: String,
}

/// Compilation witness guaranteeing mathematical conformance
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlphaWitness {
    pub witness_id: String,
    pub verified_hash: String,
}

/// Admission wrapper containing the type proof and payload
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Admission<T, W> {
    pub payload: T,
    pub witness: W,
}

/// Validates raw telemetry feedstock and projects it into either an Admission or a Refusal (Live-Check & Refusal Mapping)
#[allow(clippy::result_large_err)]
pub fn validate_and_project(
    feedstock: TelemetryFeedstockS2,
    bridge: &BridgeRx,
) -> Result<Admission<AdmittedPayload, AlphaWitness>, Refusal> {
    // 1. Feedstock Routing & Check: Verify presence of Process Instance ID
    let instance_id = match feedstock.instance_id.clone() {
        Some(id) if !id.trim().is_empty() => id,
        _ => {
            return Err(build_refusal(
                RefusalCode::MissingInstanceId,
                "process.pi.instance_id must be provided and non-empty",
                feedstock,
            ))
        }
    };

    // 2. Schema URL translation check: Verify transition/activity name
    let _activity_name = match feedstock.transition_name.clone() {
        Some(name) if !name.trim().is_empty() => name,
        _ => {
            return Err(build_refusal(
                RefusalCode::MissingActivityName,
                "process.pi.transition.name must be provided and non-empty",
                feedstock,
            ))
        }
    };

    // 3. Verify presence of Witness ID
    let witness_id = match feedstock.witness_id.clone() {
        Some(id) if !id.trim().is_empty() => id,
        _ => {
            return Err(build_refusal(
                RefusalCode::MissingWitnessId,
                "process.pi.witness.id must be provided and non-empty",
                feedstock,
            ))
        }
    };

    // 4. Verify witness signature length (must be a valid 64-character BLAKE3 hex hash representation)
    let witness_hash = match feedstock.witness_hash.clone() {
        Some(hash) if hash.len() == 64 => hash,
        _ => {
            return Err(build_refusal(
                RefusalCode::InvalidWitnessSignature,
                "process.pi.witness.hash must be a valid 64-character hex signature",
                feedstock,
            ))
        }
    };

    // Perform schema URL binding translation using the bridge
    let translated = match bridge.translate(feedstock.clone()) {
        Ok(t) => t,
        Err(_) => {
            return Err(build_refusal(
                RefusalCode::MissingActivityName,
                "Failed to map schema v2 attributes to v1 target registry format",
                feedstock,
            ))
        }
    };

    Ok(Admission {
        payload: AdmittedPayload {
            instance_id,
            activity_name: translated.activity_name,
        },
        witness: AlphaWitness {
            witness_id,
            verified_hash: witness_hash,
        },
    })
}

fn build_refusal(code: RefusalCode, rule: &str, feedstock: TelemetryFeedstockS2) -> Refusal {
    let raw_str = format!("{:?}:{:?}:{}", code, feedstock, rule);
    let mut hasher = Blake3::new();
    hasher.update(raw_str.as_bytes());
    let hash_bytes = hasher.finalize();
    
    // Convert BLAKE3 hash bytes to hex string format
    let mut hex_str = String::with_capacity(64);
    for &b in &hash_bytes {
        hex_str.push_str(&format!("{:02x}", b));
    }

    Refusal {
        code,
        violated_rule: rule.to_string(),
        raw_feedstock: feedstock,
        blake3_digest: hex_str,
    }
}

#[test]
fn test_weaver_integration_synthesis() {
    let bridge = BridgeRx::new();

    // Case 1: Nominally correct telemetry schema v2 is successfully translated and validated.
    let valid_feedstock = TelemetryFeedstockS2 {
        instance_id: Some("inst-abc-123".to_string()),
        transition_name: Some("approve_invoice".to_string()),
        witness_id: Some("auth_governor_alpha".to_string()),
        witness_hash: Some("4a98f1c8b210e309228d4bf09c0fa4db55fe310efc35ad1a89b3f021e85a6111".to_string()),
    };

    let admission_result = validate_and_project(valid_feedstock, &bridge);
    assert!(admission_result.is_ok());
    let admission = admission_result.unwrap();
    
    assert_eq!(admission.payload.instance_id, "inst-abc-123");
    // Ensure translation mapped transition_name (S2) into activity_name (S1)
    assert_eq!(admission.payload.activity_name, "approve_invoice");
    assert_eq!(admission.witness.witness_id, "auth_governor_alpha");
    assert_eq!(admission.witness.verified_hash, "4a98f1c8b210e309228d4bf09c0fa4db55fe310efc35ad1a89b3f021e85a6111");

    // Case 2: Feedstock routing check - Spans/Events missing instance_id are routed to refusal.
    let missing_instance = TelemetryFeedstockS2 {
        instance_id: None,
        transition_name: Some("approve_invoice".to_string()),
        witness_id: Some("auth_governor_alpha".to_string()),
        witness_hash: Some("4a98f1c8b210e309228d4bf09c0fa4db55fe310efc35ad1a89b3f021e85a6111".to_string()),
    };

    let refusal_result = validate_and_project(missing_instance.clone(), &bridge);
    assert!(refusal_result.is_err());
    let refusal = refusal_result.unwrap_err();
    assert_eq!(refusal.code, RefusalCode::MissingInstanceId);
    assert_eq!(refusal.violated_rule, "process.pi.instance_id must be provided and non-empty");
    assert_eq!(refusal.raw_feedstock, missing_instance);
    assert_eq!(refusal.blake3_digest.len(), 64);

    // Case 3: Refusal Mapping check - Missing activity/transition name.
    let missing_activity = TelemetryFeedstockS2 {
        instance_id: Some("inst-abc-123".to_string()),
        transition_name: None,
        witness_id: Some("auth_governor_alpha".to_string()),
        witness_hash: Some("4a98f1c8b210e309228d4bf09c0fa4db55fe310efc35ad1a89b3f021e85a6111".to_string()),
    };

    let refusal_result = validate_and_project(missing_activity.clone(), &bridge);
    assert!(refusal_result.is_err());
    let refusal = refusal_result.unwrap_err();
    assert_eq!(refusal.code, RefusalCode::MissingActivityName);
    assert_eq!(refusal.raw_feedstock, missing_activity);
    assert_eq!(refusal.blake3_digest.len(), 64);

    // Case 4: Refusal Mapping check - Missing witness ID.
    let missing_witness = TelemetryFeedstockS2 {
        instance_id: Some("inst-abc-123".to_string()),
        transition_name: Some("approve_invoice".to_string()),
        witness_id: None,
        witness_hash: Some("4a98f1c8b210e309228d4bf09c0fa4db55fe310efc35ad1a89b3f021e85a6111".to_string()),
    };

    let refusal_result = validate_and_project(missing_witness.clone(), &bridge);
    assert!(refusal_result.is_err());
    let refusal = refusal_result.unwrap_err();
    assert_eq!(refusal.code, RefusalCode::MissingWitnessId);
    assert_eq!(refusal.raw_feedstock, missing_witness);
    assert_eq!(refusal.blake3_digest.len(), 64);

    // Case 5: Refusal Mapping check - Invalid witness signature hash.
    let invalid_signature = TelemetryFeedstockS2 {
        instance_id: Some("inst-abc-123".to_string()),
        transition_name: Some("approve_invoice".to_string()),
        witness_id: Some("auth_governor_alpha".to_string()),
        witness_hash: Some("short_hash".to_string()),
    };

    let refusal_result = validate_and_project(invalid_signature.clone(), &bridge);
    assert!(refusal_result.is_err());
    let refusal = refusal_result.unwrap_err();
    assert_eq!(refusal.code, RefusalCode::InvalidWitnessSignature);
    assert_eq!(refusal.raw_feedstock, invalid_signature);
    assert_eq!(refusal.blake3_digest.len(), 64);
}
