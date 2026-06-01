# Public Standards to Decommissioning Mapping

Decommissioning is the final stage of the process lifecycle, where a process is retired, its runtime engines shut down, and its operational history securely archived. This document establishes how standards-compliant logs, models, and metadata are processed during decommissioning, and how final decommissioning receipts are generated.

For the formal definition of the decommissioning stage, see the [Decommission State definition](file:///Users/sac/process-intelligence/lifecycle/define_decommission-state_process_intelligence.md).

---

## 1. Archival Requirements by Standard

To prevent data loss and ensure historical auditability, decommissioning requires format-specific archival protocols:

| Standard | In-Scope Artifacts | Archival Format | Retained Metadata |
| :--- | :--- | :--- | :--- |
| **XES** | Event Logs | Compressed XML (`.xes.gz`) | Full Concept and Time extensions, trace count, and execution hashes. |
| **OCEL** | Object Graphs | SQLite / JSON-LD | Complete event-object link tables, attribute histories, and SHACL shapes. |
| **BPMN** | Process Models | BPMN 2.0 XML | Layout properties, activity documentation, and DCTERMS metadata. |
| **Petri Net**| Process Models | PNML (Petri Net Markup) | Node coordinates, place capacities, and soundness verification reports. |
| **Declare** | Compliance Rules | LTL Rule Schema | Complete set of constraints, activation records, and violation history. |
| **PROV-O** | Lineage Graphs | RDF Turtle (`.ttl`) | Complete provenance chain from raw extraction to final decommission. |

---

## 2. Final Decommissioning Receipt Generation

At the moment of process retirement, the execution core (`wasm4pm`) compiles the final archival package and issues a signed decommissioning receipt. The receipt is registered as the final block on the process's ledger:

```json
{
  "decommission_receipt_id": "dec-rec-990e8400-e29b-41d4-a716-446655447777",
  "timestamp": "2026-05-31T23:59:59Z",
  "retired_process_id": "proc-550e8400-e29b-41d4-a716-446655440000",
  "final_conformance_metrics": {
    "historical_replay_fitness": 0.978,
    "historical_precision": 0.912,
    "total_recorded_cases": 124500,
    "total_recorded_violations": 14
  },
  "archived_package_hash": "b4c3d2e1f0g9h8i7j6k5l4m3n2o1p0q9r8s7t6u5v4w3x2y1z0",
  "witness_signature": "SIG_ED25519_..."
}
```

---

## 3. Verification of Archived State

To verify that a retired process has been cleanly archived:
1.  Auditors locate the decommissioning receipt registered on the ledger.
2.  The archived package hash must match the cryptographic sum of the ZIP/TAR archive in the cold data room.
3.  For a sample decommissioning receipt and package structure, see the [Decommission Receipt Sample](file:///Users/sac/process-intelligence/experiments/decommission_receipt_sample.md).