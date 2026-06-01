# XES Standard Ledger Placement

The **eXtensible Event Stream (XES)** standard (IEEE 1849-2016) defines an XML-based serialization format for event logs. This document establishes how XES-compliant event streams are formally ingested, represented, and verified within the process-intelligence ledger.

---

## 1. Ontological Mapping to the Ledger

An XES log is structured as a hierarchical XML document: `log -> trace -> event`. The foundry maps this hierarchy to relational and graph assertions in the ledger:

| XES Element / Attribute | Ledger Assertion | Semantic Type | Description |
| :--- | :--- | :--- | :--- |
| `log` | `ProcessScope` | Namespace / UUID | Represents the bounds of the process being audited. |
| `trace` | `ProcessCase` | Entity / UUID | Represents a single execution instance (case ID). |
| `event` | `ProcessStateTransition` | Transaction Event | Represents an action executed at a specific time. |
| `concept:name` | `ActivityLabel` | String (SKOS Concept) | The semantic name of the process activity. |
| `time:timestamp` | `ExecutionTimestamp` | UTC ISO 8601 Timestamp | Chronological marker of transition execution. |
| `lifecycle:transition` | `LifecycleState` | Enum (`assign`, `start`, `complete`) | The task execution state. |

---

## 2. Type-System and Cryptographic Constraints

To prevent data tampering or "process log laundering," the ledger enforces the following rules on any ingested XES log:

1.  **Trace Hash Chaining**: For every trace $\sigma = \langle e_1, e_2, \dots, e_n \rangle$, the ledger computes a cumulative hash to lock the sequence:
    $$\mathcal{H}(\sigma) = \operatorname{BLAKE3}\left( \mathcal{H}(e_1) \parallel \mathcal{H}(e_2) \parallel \dots \parallel \mathcal{H}(e_n) \right)$$
    Any out-of-order execution or event tampering invalidates $\mathcal{H}(\sigma)$, failing the ledger's admission gate.
2.  **Chronological Invariance**: Timestamps must be strictly non-decreasing within a trace:
    $$\forall i \in [1, n-1], \quad t(e_{i+1}) \ge t(e_i)$$
3.  **Mandatory Extensions**: Every XES file must declare the Concept Extension and Time Extension. If the Lifecycle Extension is missing, it defaults to `complete`.

---

## 3. Academic Foundations and Conformance

Replay conformance on XES logs uses optimal alignments (Adriansyah 2014) and token-based replay fitness (van der Aalst 2016):
*   For the mathematical definitions of fitness and precision, see the [Blue River Dam Doctrine](file:///Users/sac/process-intelligence/doctrine/blue-river-dam.md).
*   For experimental validations, see the [XES Loss Policy Sample](file:///Users/sac/process-intelligence/experiments/xes_loss-policy_sample.md).

---

## 4. M&A Slide-to-Receipt Bridge

When a seller asserts a metric based on an XES log (e.g., "Order-to-Cash process compliance is 98%"), the assertion must be verified:
1.  The seller generates an alignment receipt using `wasm4pm` with the target XES log.
2.  The resulting BLAKE3 hash is registered in the [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md).
3.  The buyer verifies the claim by re-running the conformance check on the XES data room artifact, ensuring the result is within the $10^{-6}$ tolerance specified in [Buyer Reliance Requirements](file:///Users/sac/process-intelligence/ma/define_buyer_reliance_requirements.md).

---

## 5. Trans-Standard Conversions and Loss Policy

### 5.1 Conversion: XES to OCEL 2.0 (Object-Centric Reconstruction)
When converting a flat trace-based XES log into an Object-Centric Event Log (OCEL 2.0), the converter must reconstruct the underlying multi-object graph.

*   **Structural Loss Policy**:
    1.  **Synthetic Object Materialization**: Case IDs in XES are mapped to object instances of a default type (`CaseObject`). This is a synthetic mapping and does not represent the true physical object topology.
    2.  **Attribute Uplift**: Trace attributes are promoted to object attributes, and event attributes are preserved.
    3.  **Refusal Threshold**: If trace IDs are malformed or missing, the conversion is refused.
*   **Signed LossReport Output Schema**:
    Every conversion generates a `LossReport` signed by the translation witness:
    ```json
    {
      "loss_report_id": "lr-xes-ocel-uuid",
      "timestamp": "2026-06-01T00:00:00Z",
      "source_format": "XES",
      "target_format": "OCEL2.0",
      "structural_changes": {
        "synthetic_objects_created": 150,
        "pruned_attributes_count": 0,
        "unmapped_trace_attributes": 0
      },
      "witness_signature": "SIG_ED25519_..."
    }
    ```

### 5.2 Conversion: OCEL 2.0 to XES (Flattening)
When a multi-object OCEL 2.0 log is flattened to fit a single-perspective XES trace structure, significant structural information is lost.

*   **Structural Loss Policy**:
    1.  **Object-to-Object (O2O) Link Discarding**: All O2O links are completely discarded.
    2.  **Multi-Object Event Splitting / Case Projections**: Events bound to multiple objects must be duplicated across multiple cases or projected onto a single target object type. All other event-object links are pruned.
    3.  **Attribute Pruning**: Non-projected object attributes are pruned.
*   **Signed LossReport Output Schema**:
    ```json
    {
      "loss_report_id": "lr-ocel-xes-uuid",
      "timestamp": "2026-06-01T00:00:00Z",
      "source_format": "OCEL2.0",
      "target_format": "XES",
      "structural_changes": {
        "discarded_o2o_links_count": 450,
        "pruned_e2o_links_count": 1200,
        "pruned_object_attributes_count": 300,
        "duplicate_events_created": 80
      },
      "witness_signature": "SIG_ED25519_..."
    }
    ```