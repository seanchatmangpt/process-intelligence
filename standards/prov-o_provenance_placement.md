# PROV-O Provenance Standard Ledger Placement

The **W3C Provenance Ontology (PROV-O)** is the W3C standard for describing the provenance (lineage) of digital resources. Within the Process Intelligence Research Foundry, PROV-O maps the extraction lineage of event logs (XES/OCEL) back to their source transaction logs (e.g., SAP, Salesforce) to prevent data laundering and ensure auditability. This document defines how PROV-O graphs are registered and verified on the ledger.

---

## 1. Ontological Mapping to the Ledger

PROV-O models provenance using three core classes: **Entity**, **Activity**, and **Agent**. The foundry maps these elements to ledger provenance transaction blocks:

| PROV-O Element | Process-Intelligence Concept | Ledger Class / Relation | Description |
| :--- | :--- | :--- | :--- |
| `prov:Entity` | **Event Log / Process Model** | `ProvenanceEntity` | A process artifact (e.g., target event log, mined Petri Net). |
| `prov:Activity` | **Data Ingestion / Mining Run** | `ProvenanceActivity` | The execution step that generated or modified the entity. |
| `prov:Agent` | **Auditor / Execution Core** | `ProvenanceAgent` | The user, system, or WASM runtime that executed the activity. |
| `prov:wasGeneratedBy`| **Entity -> Activity** | `GeneratedByRelation` | Indicates which execution run generated the event log. |
| `prov:wasAssociatedWith`| **Activity -> Agent** | `AssociatedWithRelation`| Indicates which engine version ran the ingestion query. |
| `prov:wasDerivedFrom` | **Log_Clean -> Log_Raw** | `DerivedFromRelation` | Maps cleaning filters and delta changes. |

---

## 2. Type-System and Anti-Laundering Constraints

To ensure the authenticity of the data room, the ledger enforces strict provenance rules:

1.  **Immutable History**: Provenance graphs must be directed acyclic graphs (DAGs) pointing backward in time:
    $$\forall (x, y) \in E_{\text{provenance}}, \quad t(y) < t(x)$$
2.  **Laundering Refusal**: Ingested event logs must trace back to a raw database extract. If the log is derived from another log, the cleaning activity must document every trace and event removed:
    $$\Delta(L_{\text{raw}}, L_{\text{clean}}) \implies \text{delta\_report\_signature}$$
    If this lineage is broken, the log is rejected. For details, see the [Raw Laundering Refusal Sample](file:///Users/sac/process-intelligence/experiments/raw-laundering_refusal_sample.md).
3.  **Cryptographic Proof Graphs**: The provenance graph is stored on the ledger as a set of RDF assertions hashed with BLAKE3:
    $$\mathcal{H}_{\text{lineage}} = \operatorname{BLAKE3}\left( \text{Provenance}_{\text{RDF\_canonical}} \right)$$

---

## 3. Academic Foundations and Conformance

*   W3C PROV-O guarantees the transparency of the diligence data room.
*   For the step-by-step audit path, see the [Auditor Evidence Path](file:///Users/sac/process-intelligence/ma/define_auditor_evidence_path.md).
*   For details on buyer due diligence, see the [Buyer Reliance Requirements](file:///Users/sac/process-intelligence/ma/define_buyer_reliance_requirements.md).

---

## 4. M&A Slide-to-Receipt Bridge

To verify data source authenticity during transactions:
1.  Every slide asserting operational performance must trace its log source back to the raw source systems (e.g., ERP transaction logs) using PROV-O records.
2.  The PROV-O lineage graph hash is registered under the [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md).
3.  Auditors execute verification queries to confirm the data path has not been tampered with.