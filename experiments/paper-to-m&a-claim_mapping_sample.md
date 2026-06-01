# Experiment: Paper-to-M&A Claim Mapping

This experiment maps classical process mining metrics (soundness, liveness, alignment fitness) to financial and operational M&A diligence claims. By utilizing mathematical properties of Workflow Nets (van der Aalst 1998) and alignments (Adriansyah 2014), buyers and sellers can verify claims with absolute defensibility.

## 1. M&A Claim to Process Science Crosswalk

| M&A Claim Type | Diligence Goal | Process Mining Metric | Academic Foundation |
| :--- | :--- | :--- | :--- |
| **Synergy Verification** | Prove that merging two supply chain processes yields efficiency without deadlocks. | Structural Soundness of the composed Petri Net. | van der Aalst 1998 |
| **Operational Liability Assessment** | Identify compliance violations, leakages, or unauthorized deviations. | Alignment Conformance Fitness ($f$). | Adriansyah 2014 |
| **Systemic Integration Debt** | Quantify the risk of process divergence and vendor lock-in. | Model-to-Log Precision and Generalization. | van der Aalst 2016 |

## 2. M&A Defensibility JSON Validation Schema

The following JSON Schema verifies that an M&A claim is supported by admissible process mining proofs:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "MACleanDiligenceClaimVerification",
  "type": "object",
  "properties": {
    "claim_id": { "type": "string" },
    "claim_category": { "enum": ["synergy", "liability", "integration_risk"] },
    "financial_value_usd": { "type": "number", "minimum": 0 },
    "evidence": {
      "type": "object",
      "properties": {
        "petri_net_soundness": { "type": "boolean" },
        "conformance_fitness": { "type": "number", "minimum": 0.0, "maximum": 1.0 },
        "violating_trace_percentage": { "type": "number", "minimum": 0.0, "maximum": 100.0 },
        "receipt_signature": { "type": "string" }
      },
      "required": ["petri_net_soundness", "conformance_fitness", "violating_trace_percentage", "receipt_signature"]
    }
  },
  "required": ["claim_id", "claim_category", "financial_value_usd", "evidence"]
}
```

## 3. Real M&A Validation Instances

### Case 1: Defensible Synergy Claim (Approved)
The seller claims a $5M synergy from process consolidation. The process mining evidence shows a fully sound Petri Net structure and high alignment fitness ($f = 0.98$).

```json
{
  "claim_id": "synergy_procure_to_pay_001",
  "claim_category": "synergy",
  "financial_value_usd": 5000000.00,
  "evidence": {
    "petri_net_soundness": true,
    "conformance_fitness": 0.982,
    "violating_trace_percentage": 1.8,
    "receipt_signature": "sha256-42d8f99e3a890db8182b83c799a4c8e7915a210b38ff40c7ea82312b918f4a1a"
  }
}
```
**Diligence Verdict**: `Defensible`. The buyer accepts the claim and includes it in the valuation model.

### Case 2: Rejected Synergy Claim (Operational Debt Detected)
The seller claims a $3.5M cost reduction in order fulfillment. However, conformance checking reveals high trace deviation ($f = 0.62$, violating trace percentage of $38\%$) representing major operational debt.

```json
{
  "claim_id": "synergy_fulfillment_002",
  "claim_category": "liability",
  "financial_value_usd": 3500000.00,
  "evidence": {
    "petri_net_soundness": false,
    "conformance_fitness": 0.620,
    "violating_trace_percentage": 38.0,
    "receipt_signature": "sha256-ff32c91823ab817d23cf9e9c8e9f9024ba7b12d90fa8e62c12a843e620581ba9"
  }
}
```
**Diligence Verdict**: `Rejected`. The buyer demands a valuation write-down of $3.5M due to latent integration risk and operational debt.

## 4. Linkages to Standards and M&A Claims

- **Standard Crosswalk**: Standards mappings are configured in file:///Users/sac/process-intelligence/standards/public_standards_to_m&a_claims.md.
- **Diligence Taxonomy**: The taxonomy definitions are anchored at file:///Users/sac/process-intelligence/ma/define_board_claim_taxonomy.md.