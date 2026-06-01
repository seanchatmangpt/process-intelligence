# Slide-to-Receipt Map

Executive-level M&A decks contain numerous process performance and cost assertions. Under the board admissibility rules, each of these slide claims must map to a cryptographic receipt demonstrating its empirical validity. This document defines the structure, schema, and verification workflow for the Slide-to-Receipt Map.

## 1. The Slide-to-Receipt Protocol

The Slide-to-Receipt Map acts as a ledger in the Virtual Data Room (VDR), linking slide slide numbers to specific query receipts.

```
┌──────────────────┐      maps to      ┌────────────────────────┐
│ PowerPoint Slide │ ────────────────> │ Verification Receipt   │
│ "95% Conformance"│                   │ (JSON + Cryptographic) │
└──────────────────┘                   └───────────┬────────────┘
                                                   │
                                                   ▼ contains
                                       ┌────────────────────────┐
                                       │ Log Hash, Query Engine │
                                       │ Fitness/Precision Math │
                                       │ Validator Signature    │
                                       └────────────────────────┘
```

## 2. Cryptographic Receipt Schema (JSON Specification)

Every receipt must be stored in the VDR under `/process-intelligence/receipts/` and conform to the following JSON schema:

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "ProcessIntelligenceVerificationReceipt",
  "type": "object",
  "properties": {
    "slide_id": { "type": "string", "description": "UUID of the presentation slide." },
    "slide_title": { "type": "string" },
    "assertion_text": { "type": "string", "description": "The exact textual claim made on the slide." },
    "target_log_hash": { "type": "string", "description": "SHA-256 hash of the XES/OCEL log file." },
    "process_model_hash": { "type": "string", "description": "SHA-256 hash of the Petri net / BPMN file." },
    "query_definition": {
      "type": "object",
      "properties": {
        "engine": { "type": "string", "enum": ["wasm4pm", "pm4py"] },
        "query_uri": { "type": "string" },
        "parameters": { "type": "object" }
      },
      "required": ["engine", "query_uri"]
    },
    "verification_results": {
      "type": "object",
      "properties": {
        "fitness": { "type": "number", "minimum": 0, "maximum": 1 },
        "precision": { "type": "number", "minimum": 0, "maximum": 1 },
        "throughput_days": { "type": "number" }
      },
      "required": ["fitness", "precision"]
    },
    "validator_signature": { "type": "string", "description": "Cryptographic signature of the execution engine." }
  },
  "required": ["slide_id", "assertion_text", "target_log_hash", "query_definition", "verification_results", "validator_signature"]
}
```

## 3. Verification Workflow

To validate a slide claim using the Slide-to-Receipt Map:

1. **Assertion Lookup**: Identify the `slide_id` on the M&A slide notes.
2. **Receipt Retrieval**: Locate the corresponding receipt `receipt_<slide_id>.json` in the VDR.
3. **Log Hashing**: Re-hash the target event log file. Verify that:
   $$\operatorname{SHA-256}(L_{\text{actual}}) == \text{target\_log\_hash}$$
4. **Signature Verification**: Verify the `validator_signature` against the public key of the target's audited `wasm4pm` execution core.
5. **Replay Validation**: Execute the query using the `query_definition.query_uri` on the validated log and compare the resulting metrics to `verification_results`.

## 4. Related M&A Validation Documents

* For linking slide claims to token-replay and conformance, see [Slide-to-Replay Map](file:///Users/sac/process-intelligence/ma/define_slide-to-replay_map.md).
* For mapping slide claims to residual risks and unfit traces, see [Slide-to-Residual Map](file:///Users/sac/process-intelligence/ma/define_slide-to-residual_map.md).
* For mapping slide claims to standards conformance, see [Slide-to-Public-Standard Map](file:///Users/sac/process-intelligence/ma/define_slide-to-public-standard_map.md).
* For board admissibility rules, see [Board-Admissible Claim Requirements](file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md).