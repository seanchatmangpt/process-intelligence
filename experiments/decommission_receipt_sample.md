# Experiment: Decommission Receipt Validation

This experiment validates the Decommission Receipt format, proving that an enterprise asset or system has reached a terminal state and was successfully retired in compliance with public archiving standards.

## 1. Decommission Receipt JSON Validation Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "DecommissionReceipt",
  "type": "object",
  "properties": {
    "decommission_id": { "type": "string" },
    "decommission_timestamp": { "type": "string", "format": "date-time" },
    "asset_identifier": { "type": "string" },
    "asset_type": { "type": "string" },
    "conformance_proof": {
      "type": "object",
      "properties": {
        "final_marking_reached": { "type": "boolean" },
        "residual_tokens_remaining": { "type": "integer", "maximum": 0 },
        "archival_hash_sha256": { "type": "string", "pattern": "^[0-9a-fA-F]{64}$" },
        "compliance_gate_verdict": { "enum": ["PASSED", "FAILED"] }
      },
      "required": ["final_marking_reached", "residual_tokens_remaining", "archival_hash_sha256", "compliance_gate_verdict"]
    },
    "cryptographic_witness": {
      "type": "object",
      "properties": {
        "signer_identity": { "type": "string" },
        "witness_signature": { "type": "string" }
      },
      "required": ["signer_identity", "witness_signature"]
    }
  },
  "required": [
    "decommission_id",
    "decommission_timestamp",
    "asset_identifier",
    "asset_type",
    "conformance_proof",
    "cryptographic_witness"
  ]
}
```

## 2. Concrete Decommission Receipt Instance

The following instance certifies that a legacy workflow server has been successfully shut down with zero active or leaking tokens:

```json
{
  "decommission_id": "dec_receipt_2026_88b901",
  "decommission_timestamp": "2026-05-31T22:44:00Z",
  "asset_identifier": "server_node_west_09",
  "asset_type": "workflow_engine_instance",
  "conformance_proof": {
    "final_marking_reached": true,
    "residual_tokens_remaining": 0,
    "archival_hash_sha256": "8e034aaab23c610ea3ed372e13bc159abf5556cec24debd57aded6f9e0cbfb4d",
    "compliance_gate_verdict": "PASSED"
  },
  "cryptographic_witness": {
    "signer_identity": "autonomic_decom_authority_0",
    "witness_signature": "0xfa0c6ffac0d1029616172d8e2d8d5af61dd859768e73e1cbf3d14d572dfc6b80"
  }
}
```

## 3. Linkages to Standards and M&A Claims

- **Standard Crosswalk**: Aligns with standards mapped at file:///Users/sac/process-intelligence/standards/public_standards_to_decommissioning.md.
- **M&A Claims**: Defensibility claims are verified by mapping these decommissioning proofs to residual liabilities at file:///Users/sac/process-intelligence/ma/define_slide-to-residual_map.md.