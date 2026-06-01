//! Compliance audit for generated code
//!
//! Validates that generated modules meet type law and witness constraints

use super::receipt_ledger::ArtifactReceipt;
use std::fmt;

/// Audit finding
#[derive(Debug, Clone)]
pub struct AuditFinding {
    pub level: AuditLevel,
    pub code: String,
    pub message: String,
    pub location: String,
}

/// Audit severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditLevel {
    /// Critical law violation
    Error,
    /// Minor style or naming issue
    Warning,
    /// Informational observation
    Info,
}

impl fmt::Display for AuditLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AuditLevel::Error => write!(f, "ERROR"),
            AuditLevel::Warning => write!(f, "WARNING"),
            AuditLevel::Info => write!(f, "INFO"),
        }
    }
}

/// Audit result
#[derive(Debug, Clone)]
pub struct AuditResult {
    pub artifact_id: String,
    pub passed: bool,
    pub findings: Vec<AuditFinding>,
}

impl AuditResult {
    /// Create a new audit result
    pub fn new(artifact_id: &str) -> Self {
        AuditResult {
            artifact_id: artifact_id.to_string(),
            passed: true,
            findings: vec![],
        }
    }

    /// Add a finding
    pub fn add_finding(&mut self, finding: AuditFinding) {
        if finding.level == AuditLevel::Error {
            self.passed = false;
        }
        self.findings.push(finding);
    }

    /// Check if audit passed
    pub fn is_passed(&self) -> bool {
        self.passed
    }

    /// Export audit report
    pub fn export_report(&self) -> String {
        let mut report = format!("=== Audit Report: {} ===\n", self.artifact_id);
        report.push_str(&format!("Result: {}\n\n", if self.passed { "PASS" } else { "FAIL" }));

        for finding in &self.findings {
            report.push_str(&format!(
                "[{}] {}: {}\n  At: {}\n\n",
                finding.level, finding.code, finding.message, finding.location
            ));
        }

        report
    }
}

/// Code compliance auditor
pub struct ComplianceAuditor;

impl ComplianceAuditor {
    /// Audit a generated module for law compliance
    pub fn audit_module(
        artifact_id: &str,
        content: &str,
        receipt: &ArtifactReceipt,
    ) -> AuditResult {
        let mut result = AuditResult::new(artifact_id);

        // Check 1: Module must have license header
        if !content.contains("License: Executable only under wasm4pm graduation bridge") {
            result.add_finding(AuditFinding {
                level: AuditLevel::Error,
                code: "MISSING_LICENSE".to_string(),
                message: "Generated module must include wasm4pm license header".to_string(),
                location: "file header".to_string(),
            });
        }

        // Check 2: Module must reference witness marker
        if !content.contains("witness") && !content.contains("Witness") {
            result.add_finding(AuditFinding {
                level: AuditLevel::Warning,
                code: "NO_WITNESS_REFERENCE".to_string(),
                message: "Module should reference a witness marker from type law".to_string(),
                location: "module body".to_string(),
            });
        }

        // Check 3: Evidence types must use correct lifecycle markers
        if content.contains("Evidence<") {
            ComplianceAuditor::check_evidence_usage(&mut result, content);
        }

        // Check 4: WfNetConst soundness must be explicitly witnessed
        if content.contains("WfNetConst") {
            ComplianceAuditor::check_soundness_witnessing(&mut result, content);
        }

        // Check 5: Receipt must indicate compilation
        if !receipt.compiled {
            result.add_finding(AuditFinding {
                level: AuditLevel::Warning,
                code: "NOT_COMPILED".to_string(),
                message: "Generated module has not been compiled yet".to_string(),
                location: "receipt".to_string(),
            });
        }

        result
    }

    /// Check evidence type usage patterns
    fn check_evidence_usage(result: &mut AuditResult, content: &str) {
        // Evidence should only be constructed via raw() or transition methods
        if content.contains("Evidence {") {
            result.add_finding(AuditFinding {
                level: AuditLevel::Error,
                code: "DIRECT_EVIDENCE_CONSTRUCTION".to_string(),
                message: "Evidence must not be directly constructed; use Evidence::raw() or transition methods"
                    .to_string(),
                location: "evidence construction".to_string(),
            });
        }

        // Admitted must only come through admission path
        if content.contains("Admitted") && !content.contains("into_admitted") {
            result.add_finding(AuditFinding {
                level: AuditLevel::Warning,
                code: "ADMITTED_WITHOUT_BOUNDARY".to_string(),
                message: "Admitted state should be reached through Admit trait boundary".to_string(),
                location: "lifecycle transition".to_string(),
            });
        }
    }

    /// Check WfNet soundness witnessing
    fn check_soundness_witnessing(result: &mut AuditResult, content: &str) {
        if content.contains("SoundnessState::Witnessed") && !content.contains("witness_soundness") {
            result.add_finding(AuditFinding {
                level: AuditLevel::Error,
                code: "UNWITNESSED_SOUNDNESS".to_string(),
                message: "Witnessed soundness must be obtained via witness_soundness(proof)".to_string(),
                location: "WfNetConst usage".to_string(),
            });
        }
    }

    /// Audit a template for correctness
    pub fn audit_template(template_name: &str, template_content: &str) -> AuditResult {
        let mut result = AuditResult::new(template_name);

        // Check template syntax
        if !template_content.contains("//!") && !template_content.contains("/*") {
            result.add_finding(AuditFinding {
                level: AuditLevel::Info,
                code: "MISSING_DOCUMENTATION".to_string(),
                message: "Template should include documentation comments".to_string(),
                location: "template header".to_string(),
            });
        }

        // Check for balanced {{ variable }} markers
        let open_count = template_content.matches("{{").count();
        let close_count = template_content.matches("}}").count();
        if open_count != close_count {
            result.add_finding(AuditFinding {
                level: AuditLevel::Error,
                code: "UNBALANCED_MARKERS".to_string(),
                message: format!("Template has {} open but {} close markers", open_count, close_count),
                location: "template syntax".to_string(),
            });
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_missing_license() {
        let content = "fn main() {}";
        let receipt = ArtifactReceipt::new("hash".to_string(), "witness".to_string(), "template".to_string());

        let result = ComplianceAuditor::audit_module("test", content, &receipt);

        assert!(!result.is_passed());
        assert!(result.findings.iter().any(|f| f.code == "MISSING_LICENSE"));
    }

    #[test]
    fn test_template_audit_balanced_markers() {
        let template = "Module {{ name }} does something";
        let result = ComplianceAuditor::audit_template("test_template", template);

        assert!(result.is_passed());
    }
}
