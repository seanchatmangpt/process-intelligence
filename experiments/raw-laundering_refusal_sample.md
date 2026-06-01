# Experiment: Log Laundering Detection & Refusal Validation

This experiment validates the laundering detection engine in `wasm4pm`. Log laundering occurs when process operators manually edit or "wash" event logs (e.g., modifying timestamps, deleting long-running traces, inserting missing activities) to falsely inflate conformance fitness score.

The engine refuses to process any log that lacks valid transition chain signatures or contains temporal inconsistencies.

## 1. Laundering Refusal JSON Validation Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "LogLaunderingRefusalLog",
  "type": "object",
  "properties": {
    "evaluation_id": { "type": "string" },
    "analyzed_log_hash_sha256": { "type": "string", "pattern": "^[0-9a-fA-F]{64}$" },
    "verdict": { "enum": ["REJECTED", "ACCEPTED"] },
    "detection_rules": {
      "type": "object",
      "properties": {
        "check_cryptographic_chain": { "type": "boolean" },
        "check_impossible_velocity": { "type": "boolean" },
        "check_timestamp_monotonicity": { "type": "boolean" },
        "check_signature_authenticity": { "type": "boolean" },
        "reject_unhashed_dataframe": { "type": "boolean" }
      },
      "required": [
        "check_cryptographic_chain",
        "check_impossible_velocity",
        "check_timestamp_monotonicity",
        "check_signature_authenticity",
        "reject_unhashed_dataframe"
      ]
    },
    "refusal_reason": {
      "type": "object",
      "properties": {
        "rule_failed": { "type": "string" },
        "trace_id": { "type": "string" },
        "detailed_error": { "type": "string" }
      }
    }
  },
  "required": ["evaluation_id", "analyzed_log_hash_sha256", "verdict", "detection_rules"]
}
```

## 2. Concrete Laundering Refusal Instances

### Case A: Rejected Log (Laundering Detected via Chain Invalidation)
A trace was "washed" by inserting a synchronous `Audit_Invoice` event with a fake timestamp. However, it lacks the cryptographic state transition receipt signature link.

```json
{
  "evaluation_id": "eval_laundering_procurement_99a",
  "analyzed_log_hash_sha256": "81f7dca25ba3594074888c74547b0e70796a2082f9cda3b2c12a843e620581ba9",
  "verdict": "REJECTED",
  "detection_rules": {
    "check_cryptographic_chain": true,
    "check_impossible_velocity": true,
    "check_timestamp_monotonicity": true,
    "check_signature_authenticity": true,
    "reject_unhashed_dataframe": true
  },
  "refusal_reason": {
    "rule_failed": "check_cryptographic_chain",
    "trace_id": "trace_99011_laundering_candidate",
    "detailed_error": "Event 'Audit_Invoice' at index 2 has an invalid transition state hash. The signature from the executing agent was missing or corrupted. Refusing further parsing."
  }
}
```
**Wasm Engine Outcome**: Log processing aborted with `LaunderingRefusalError`. No conformance metrics are computed.

### Case B: Accepted Log (Fully Monotonic and Authenticated)
```json
{
  "evaluation_id": "eval_laundering_procurement_99b",
  "analyzed_log_hash_sha256": "ccd1ae587abbec900fca5dfbeb4b12f101b20b317cb21a9d0312b918f4a1a67a",
  "verdict": "ACCEPTED",
  "detection_rules": {
    "check_cryptographic_chain": true,
    "check_impossible_velocity": true,
    "check_timestamp_monotonicity": true,
    "check_signature_authenticity": true,
    "reject_unhashed_dataframe": true
  }
}
```
**Wasm Engine Outcome**: Log accepted; proceeds to alignment calculations.

### Case C: Rejected Log (Laundering Detected via Signature Forgery Attempt)
An adversary altered transition values (timestamps or costs) and attempted to forge system signatures by recalculating them using an unauthorized key.

```json
{
  "evaluation_id": "eval_forgery_attack",
  "analyzed_log_hash_sha256": "3e9b0e271bf1b8ff1db18f3a3a7895085e3b20755efc077d7045b84c3c3eb6fb",
  "verdict": "REJECTED",
  "detection_rules": {
    "check_cryptographic_chain": true,
    "check_impossible_velocity": true,
    "check_timestamp_monotonicity": true,
    "check_signature_authenticity": true,
    "reject_unhashed_dataframe": true
  },
  "refusal_reason": {
    "rule_failed": "check_signature_authenticity",
    "trace_id": "trace_99011_laundering_candidate",
    "detailed_error": "Event 'Create_Order' at index 0 contains a forged or invalid signature."
  }
}
```
**Wasm Engine Outcome**: Log rejected immediately at the signature verification boundary.

### Case D: Rejected Input (Ingestion Boundary Rejection of Unhashed Pandas DataFrame)
An adversary attempted to pass a mutable, unhashed `pandas.DataFrame` object directly into the ingestion interface without the required cryptographic wrapper.

```json
{
  "evaluation_id": "eval_raw_dataframe",
  "analyzed_log_hash_sha256": "0000000000000000000000000000000000000000000000000000000000000000",
  "verdict": "REJECTED",
  "detection_rules": {
    "check_cryptographic_chain": false,
    "check_impossible_velocity": false,
    "check_timestamp_monotonicity": false,
    "check_signature_authenticity": false,
    "reject_unhashed_dataframe": true
  },
  "refusal_reason": {
    "rule_failed": "reject_unhashed_dataframe",
    "trace_id": "N/A - Direct DataFrame Ingestion",
    "detailed_error": "Rejected unhashed pandas DataFrame at ingestion boundary. Logs must be immutable, pre-hashed, and signed."
  }
}
```
**Wasm Engine Outcome**: Log ingestion aborted. The system rejects the dynamic in-memory structure to enforce zero-trust boundary isolation.

## 3. Linkages to Standards and M&A Claims

- **Standard Crosswalk**: Aligns with standards mapped at [public_ontology_reverse-lock-in_map.md](file:///Users/sac/process-intelligence/standards/public_ontology_reverse-lock-in_map.md).
- **M&A Claims**: Defensibility claims are verified by mapping safety validations onto the seller defensibility requirements at [define_seller_defensibility_requirements.md](file:///Users/sac/process-intelligence/ma/define_seller_defensibility_requirements.md).