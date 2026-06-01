use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryFeedstockS2 {
    #[serde(rename = "process.pi.instance_id")]
    pub instance_id: String,
    #[serde(rename = "process.pi.transition.name")] // Renamed field
    pub transition_name: String,
    #[serde(rename = "process.pi.token.color")] // Added field
    pub token_color: Option<String>,
    #[serde(rename = "process.pi.witness.hash")]
    pub witness_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryFeedstockS1 {
    #[serde(rename = "process.pi.instance_id")]
    pub instance_id: String,
    #[serde(rename = "process.pi.activity.name")] // Original field name
    pub activity_name: String,
    #[serde(rename = "process.pi.witness.hash")]
    pub witness_hash: String,
}

/// The BridgeRx structure maps schema changes to ensure zero conformance residual.
pub struct BridgeRx {
    pub rename_map: HashMap<String, String>,
}

impl BridgeRx {
    pub fn new() -> Self {
        let mut rename_map = HashMap::new();
        // Registering the rename from S2 transition.name back to S1 activity.name
        rename_map.insert("process.pi.transition.name".to_string(), "process.pi.activity.name".to_string());
        Self { rename_map }
    }

    /// Translates S2 telemetry feedstock back to S1 structure to satisfy the court's validators
    pub fn translate(&self, input: TelemetryFeedstockS2) -> TelemetryFeedstockS1 {
        TelemetryFeedstockS1 {
            instance_id: input.instance_id,
            activity_name: input.transition_name, // Direct mapping resolved via rename_map
            witness_hash: input.witness_hash,
        }
    }
}
