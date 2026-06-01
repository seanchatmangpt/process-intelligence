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

## 3. Verification Workflow (Anti-Spoofing and Cryptographic Integrity Protocol)

To validate a slide claim using the Slide-to-Receipt Map, the auditing system must execute the following cryptographic checks:

1. **Assertion Lookup**: Retrieve the `slide_id` (UUIDv4) from the metadata of the target slide.
2. **Receipt Retrieval**: Locate `receipt_<slide_id>.json` within the Virtual Data Room under `/process-intelligence/receipts/`.
3. **Canonical Serialization and Signature Verification**:
   * Let $R_{\text{unsigned}}$ be the receipt JSON object with the `validator_signature` field removed.
   * Serialize $R_{\text{unsigned}}$ using the **JSON Canonicalization Scheme (JCS - RFC 8785)** to ensure a deterministic byte sequence:
     $$B_{\text{receipt}} = \operatorname{JCS}(R_{\text{unsigned}})$$
   * Verify the `validator_signature` (an Ed25519 signature) against the pinned auditor public key $\operatorname{PK}_{\text{validator}}$:
     $$\operatorname{Ed25519-Verify}(\operatorname{PK}_{\text{validator}}, B_{\text{receipt}}, \text{validator\_signature}) == \text{True}$$
4. **Log Hash and Merkle Root Audit**:
   * Calculate the SHA-256 hash of the target log file $L$ and verify it matches the receipt:
     $$\operatorname{SHA-256}(L) == \text{target\_log\_hash}$$
   * To prevent event-level insertion or omission, construct a Merkle Tree over the event hash chains. Let $H_k = h(e_{\text{end}})$ be the final hash of trace $k$. The Merkle root $M_{\text{root}}$ is computed as:
     $$M_{\text{root}} = \operatorname{Merkle-Root}(H_1, H_2, \dots, H_N)$$
     Verify that $M_{\text{root}}$ matches the audited Merkle root hash pinned in the transaction's smart contract or signed closing agreement.
5. **Deterministic Replay**: Load the WASM module specified in `query_definition.query_uri`, execute it on the event log $L$ using the query `parameters`, and verify that the re-calculated fitness ($f_{\text{calc}}$) and precision ($p_{\text{calc}}$) match the receipt's values:
   $$\left| f_{\text{calc}} - f_{\text{receipt}} \right| < 10^{-6} \quad \text{and} \quad \left| p_{\text{calc}} - p_{\text{receipt}} \right| < 10^{-6}$$

## 4. Related M&A Validation Documents

* For linking slide claims to token-replay and conformance, see [Slide-to-Replay Map](file:///Users/sac/process-intelligence/ma/define_slide-to-replay_map.md).
* For mapping slide claims to residual risks and unfit traces, see [Slide-to-Residual Map](file:///Users/sac/process-intelligence/ma/define_slide-to-residual_map.md).
* For mapping slide claims to standards conformance, see [Slide-to-Public-Standard Map](file:///Users/sac/process-intelligence/ma/define_slide-to-public-standard_map.md).
* For board admissibility rules, see [Board-Admissible Claim Requirements](file:///Users/sac/process-intelligence/ma/define_board-admissible_claim_requirements.md).