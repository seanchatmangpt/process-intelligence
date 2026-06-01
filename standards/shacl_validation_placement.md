# SHACL Validation Standard Ledger Placement

The **Shapes Constraint Language (SHACL)** is the W3C standard for validating graph databases against a set of conditions. In the Process Intelligence Research Foundry, SHACL shapes are used to define and validate schema properties of graph-based process logs (like OCEL 2.0 and PROV-O) at ingestion time. This document establishes how SHACL shapes and validation reports are recorded and verified on the ledger.

---

## 1. Ontological Mapping to the Ledger

SHACL defines validation rules using **Node Shapes** and **Property Shapes**. The ledger represents these rules and validation outputs as transaction blocks:

| SHACL Concept | Process-Intelligence equivalent | Ledger Representation | Description |
| :--- | :--- | :--- | :--- |
| `sh:NodeShape` | **Object / Event Schema** | `SchemaNodeShape` | Defines constraints on a class of nodes (e.g., `Event` or `Object`). |
| `sh:PropertyShape`| **Attribute Constraints** | `SchemaPropertyShape` | Defines constraints on node attributes or outgoing links. |
| `sh:datatype` | **Attribute Type** | `SchemaDataType` | Enforces type matching (e.g., timestamp must be `xsd:dateTime`). |
| `sh:minCount` | **Relation Cardinality** | `SchemaMinCount` | Enforces presence of mandatory links (e.g., event must link to $\ge 1$ object). |
| `sh:ValidationReport`| **Validation Output** | `SchemaValidationReport`| The signed output of the validation run (conforms: `true`/`false`). |

The ledger registers each validation run as a conformance transaction block:

```json
{
  "validation_run_id": "shacl-990e8400-e29b-41d4-a716-446655443333",
  "shapes_graph_hash": "a1b2c3...",
  "data_graph_hash": "e5f6g7...",
  "validation_report": {
    "conforms": true,
    "violation_count": 0,
    "results": []
  },
  "witness_signature": "SIG_ED25519_..."
}
```

---

## 2. Type Laws and Validation Constraints

The ledger enforces schema validation gates:

1.  **Ingestion Schema Check**: No graph-based event log (OCEL/PROV-O) is admitted to the ledger unless it passes SHACL schema validation:
    $$\operatorname{validate}(G_{\text{data}}, G_{\text{shapes}}).\operatorname{conforms} \equiv \text{true}$$
2.  **Deterministic Evaluation**: The SHACL validation engine must produce deterministic reports. The output report is cryptographically bound to the input graphs:
    $$\text{ReportHash} = \operatorname{BLAKE3}\left( \text{ShapesGraphHash} \parallel \text{DataGraphHash} \parallel \text{ValidationReport} \right)$$
3.  **No Dynamic Override**: Ingested data shapes cannot be modified post-hoc to hide validation failures. Any change in shapes requires a new schema registration transaction.

---

## 3. Academic Foundations and Conformance

*   SHACL ensures the structural integrity of graph-based logs.
*   For the data quality checks of OCEL logs, see the [OCEL Lifecycle Sample](file:///Users/sac/process-intelligence/experiments/ocel_lifecycle_sample.md).
*   For the general rules of compliance verification, see the [Blue River Dam Doctrine](file:///Users/sac/process-intelligence/doctrine/blue-river-dam.md).

---

## 4. M&A Slide-to-Receipt Bridge

To verify data quality claims during due diligence:
1.  The shapes graph representing the target's data model must be registered in the VDR.
2.  All conformance reports must resolve to a valid SHACL validation receipt.
3.  These receipts are linked under [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md) and verified by advisors, as required by the [Buyer Reliance Requirements](file:///Users/sac/process-intelligence/ma/define_buyer_reliance_requirements.md).