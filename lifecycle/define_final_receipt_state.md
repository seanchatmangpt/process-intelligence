# Lifecycle: Define Final Receipt State

The **Final Receipt State** defines the formal specifications, schemas, and cryptographic standards of the compliance receipts generated upon process decommissioning or audit completion.

## Autonomic MAPE-K Mapping
* **Loop Role**: **Knowledge** (immutable archive of intelligence)
* **Responsibility**: In the Knowledge phase, process metrics are cryptographically sealed, creating a tamper-proof receipt that guarantees operational data integrity.
* **Actuation Trigger**: Generated automatically when a process finishes its lifecycle (see [Decommissioning Stage](file:///Users/sac/process-intelligence/lifecycle/define_decommission-state_process_intelligence.md)) or completes an audit milestone.

---

## Cryptographic Receipt Specification

A Final Receipt ($R_{final}$) is a structured, cryptographically signed JSON-LD record that contains all relevant conformance and performance proofs.

### 1. Hash-Chaining Protocol
To ensure that historical audits cannot be altered retrospectively, each receipt incorporates the hash of the preceding receipt, forming a tamper-proof ledger of process changes:
$$H_{receipt\_n} = \text{BLAKE3} \left( Payload_n \parallel H_{receipt\_n-1} \right)$$

### 2. Receipt Payload Schema
The payload contains the following verified attributes:
* **Model Signature**: The BLAKE3 hash of the compiled Petri Net or POWL tree structure.
* **Log Signature**: The BLAKE3 hash of the associated event logs (XES or OCEL).
* **Conformance Metrics**:
  * **Alignment Fitness** ($f_{align}$): Proving log-to-model fit.
  * **Precision** ($p_{\text{prec}}$): Measuring behavioral specificity.
* **Execution Bounds**:
  * Total cases replayed.
  * Start and end timestamps of the logging period.
* **Signatures**: Digital signatures of the Process Auditor and the Executive Board multi-signature authority.

### 3. JSON-LD Implementation Example
```json
{
  "@context": "https://foundry.process-intelligence.org/schemas/receipt.jsonld",
  "@type": "FinalProcessReceipt",
  "receiptId": "REC-2026-0987",
  "previousReceiptHash": "blake3:8f3c428...",
  "modelHash": "blake3:d8c55e...",
  "logHash": "blake3:b2e5a7...",
  "metrics": {
    "alignmentFitness": 0.978,
    "precision": 0.934,
    "totalCases": 154320
  },
  "timestamps": {
    "start": "2026-01-01T00:00:00Z",
    "end": "2026-05-31T23:59:59Z"
  },
  "signature": {
    "type": "Ed25519Signature2020",
    "verificationMethod": "did:example:auditorKey#key-1",
    "signatureValue": "z3h8djS9..."
  }
}
```

---

## M&A Diligence Claims
In M&A, the Final Receipt is the **Closing Asset Certificate**.
* **Buyer Reliance**: The buyer relies on these receipts as legal proof of operational compliance and performance efficiency during the pre-acquisition period, providing a solid foundation for transaction warranties.
* **Slide-to-Receipt Map**: PowerPoint assertions claiming "Our customer onboarding process achieved 97% compliance with a 3-day average cycle time" must link directly to this Final Receipt, enabling the buyer to run independent audits and obtain identical results.

---

## Related Documents
* See the [Decommissioning Stage](file:///Users/sac/process-intelligence/lifecycle/define_decommission-state_process_intelligence.md) for generation triggers.
* See the [Board Projection State](file:///Users/sac/process-intelligence/lifecycle/define_board-projection-state_process_intelligence.md) for executive mappings.
* Back to [Lifecycle README](file:///Users/sac/process-intelligence/lifecycle/docs-law__lifecycle_readme.md).