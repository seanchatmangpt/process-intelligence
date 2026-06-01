//! Receipt ledger for tracking generated artifacts
//!
//! Each manufactured artifact receives a receipt (hash + provenance record)

use std::collections::HashMap;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

/// Artifact receipt: proves provenance and witness compliance
#[derive(Debug, Clone)]
pub struct ArtifactReceipt {
    /// Content hash (blake3)
    pub content_hash: String,
    /// Witness marker at graduation
    pub witness: String,
    /// Lifecycle state at sealing
    pub lifecycle_state: String,
    /// Timestamp of generation
    pub timestamp: u64,
    /// Template source
    pub template_source: String,
    /// Template variables used
    pub context_snapshot: HashMap<String, String>,
    /// Compilation status
    pub compiled: bool,
    /// Compliance audit result
    pub audit_passed: bool,
}

impl ArtifactReceipt {
    /// Create a new receipt
    pub fn new(
        content_hash: String,
        witness: String,
        template_source: String,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        ArtifactReceipt {
            content_hash,
            witness,
            lifecycle_state: "Sealed".to_string(),
            timestamp,
            template_source,
            context_snapshot: HashMap::new(),
            compiled: false,
            audit_passed: false,
        }
    }

    /// Set compilation status
    pub fn with_compilation(mut self, compiled: bool) -> Self {
        self.compiled = compiled;
        self
    }

    /// Set audit status
    pub fn with_audit(mut self, passed: bool) -> Self {
        self.audit_passed = passed;
        self
    }

    /// Add context snapshot
    pub fn with_context(mut self, context: HashMap<String, String>) -> Self {
        self.context_snapshot = context;
        self
    }

    /// Verify receipt integrity
    pub fn verify(&self) -> bool {
        // Check that all required fields are present
        !self.content_hash.is_empty()
            && !self.witness.is_empty()
            && !self.template_source.is_empty()
            && self.timestamp > 0
    }

    /// Serialize receipt to JSON-like format
    pub fn to_json_string(&self) -> String {
        format!(
            r#"{{
  "content_hash": "{}",
  "witness": "{}",
  "lifecycle_state": "{}",
  "timestamp": {},
  "template_source": "{}",
  "compiled": {},
  "audit_passed": {}
}}"#,
            self.content_hash,
            self.witness,
            self.lifecycle_state,
            self.timestamp,
            self.template_source,
            self.compiled,
            self.audit_passed
        )
    }
}

/// Receipt ledger: maintains all artifact receipts
pub struct ReceiptLedger {
    receipts: HashMap<String, ArtifactReceipt>,
}

impl ReceiptLedger {
    /// Create a new ledger
    pub fn new() -> Self {
        ReceiptLedger {
            receipts: HashMap::new(),
        }
    }

    /// Record an artifact receipt
    pub fn record(&mut self, artifact_id: &str, receipt: ArtifactReceipt) {
        self.receipts.insert(artifact_id.to_string(), receipt);
    }

    /// Retrieve a receipt
    pub fn get(&self, artifact_id: &str) -> Option<&ArtifactReceipt> {
        self.receipts.get(artifact_id)
    }

    /// List all receipts
    pub fn all(&self) -> Vec<&ArtifactReceipt> {
        self.receipts.values().collect()
    }

    /// Verify all receipts are valid
    pub fn verify_all(&self) -> bool {
        self.receipts.values().all(|r| r.verify())
    }

    /// Count receipts by witness marker
    pub fn count_by_witness(&self) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        for receipt in self.receipts.values() {
            *counts.entry(receipt.witness.clone()).or_insert(0) += 1;
        }
        counts
    }

    /// Export ledger as text report
    pub fn export_report(&self) -> String {
        let mut report = String::from("=== Receipt Ledger Report ===\n\n");

        report.push_str(&format!("Total artifacts: {}\n", self.receipts.len()));
        report.push_str(&format!("Ledger valid: {}\n\n", self.verify_all()));

        report.push_str("Artifacts by witness:\n");
        for (witness, count) in self.count_by_witness() {
            report.push_str(&format!("  {}: {}\n", witness, count));
        }

        report.push_str("\nDetailed receipts:\n");
        for (id, receipt) in &self.receipts {
            report.push_str(&format!("\n[{}]\n", id));
            report.push_str(&format!("  Hash: {}\n", receipt.content_hash));
            report.push_str(&format!("  Witness: {}\n", receipt.witness));
            report.push_str(&format!("  Compiled: {}\n", receipt.compiled));
            report.push_str(&format!("  Audit: {}\n", receipt.audit_passed));
        }

        report
    }
}

impl Default for ReceiptLedger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_receipt_creation() {
        let receipt = ArtifactReceipt::new(
            "abc123".to_string(),
            "VanDerAalst2016".to_string(),
            "templates/mining/alpha_miner.rs.j2".to_string(),
        );

        assert!(receipt.verify());
        assert!(!receipt.content_hash.is_empty());
    }

    #[test]
    fn test_ledger_recording() {
        let mut ledger = ReceiptLedger::new();
        let receipt = ArtifactReceipt::new(
            "hash1".to_string(),
            "Witness1".to_string(),
            "template1".to_string(),
        );

        ledger.record("artifact1", receipt);

        assert!(ledger.get("artifact1").is_some());
        assert_eq!(ledger.all().len(), 1);
    }
}
