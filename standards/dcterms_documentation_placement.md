# DCTERMS Documentation Standard Ledger Placement

The **Dublin Core Metadata Terms (DCTERMS)** standard defines a core set of metadata properties for describing digital resources. In the Process Intelligence Research Foundry, DCTERMS properties are used to tag and index process models, event logs, conformance reports, and audit trails registered on the ledger. This document defines the formal schema, constraints, and ledger integration for DCTERMS.

---

## 1. Ontological Mapping to the Ledger

Every process artifact registered on the ledger must include a metadata block using DCTERMS properties. The foundry maps these metadata properties to ledger attributes:

| DCTERMS Property | Process Mining Concept | Ledger Type | Description |
| :--- | :--- | :--- | :--- |
| `dcterms:identifier` | **Artifact UUID** | String (UUID) | Unique identifier for the process log or model. |
| `dcterms:title` | **Process Name** | String | The title of the process (e.g., "SAP Order-to-Cash"). |
| `dcterms:creator` | **Analyst / System** | String | The entity that generated the log or created the model. |
| `dcterms:created` | **Creation Date** | UTC Timestamp | The date/time when the artifact was registered. |
| `dcterms:format` | **MIME / Standard** | String | The standard format (e.g., `application/xes+xml`, `application/ocel+json`). |
| `dcterms:conformsTo` | **Reference standard**| String (URI) | Link to the formal standard definition. |
| `dcterms:relation` | **Parent log** | String (UUID) | Links derived logs or optimized models to source files. |
| `dcterms:rights` | **Access Policy** | String (URI) | Link to the ODRL rights policy on the ledger. |

---

## 2. Type Laws and Completeness Invariants

The ledger enforces metadata completeness checks at ingestion time:

1.  **Mandatory Metadata Fields**: An artifact registration transaction is rejected if it lacks the following five core fields: `identifier`, `title`, `creator`, `created`, and `format`.
2.  **Referential Lineage**: The `dcterms:relation` field must point to a valid artifact ID already registered on the ledger, preserving the lineage chain:
    $$\forall a_1, \quad \operatorname{exists}(a_1.\operatorname{relation}) \implies a_1.\operatorname{relation} \in \text{RegisteredArtifacts}$$
3.  **Cryptographic Metadata Binding**: The metadata attributes are compiled into a canonical JSON string and combined with the artifact content hash:
    $$\mathcal{H}_{\text{bound}} = \operatorname{BLAKE3}\left( \mathcal{H}_{\text{content}} \parallel \text{DCTERMS}_{\text{canonical}} \right)$$

---

## 3. Academic Foundations and Conformance

*   DCTERMS provides structural transparency for process compliance logs.
*   For the step-by-step audit path, see the [Auditor Evidence Path](file:///Users/sac/process-intelligence/ma/define_auditor_evidence_path.md).
*   For the lifecycle stages, see [Full-Lifecycle Process Scope](file:///Users/sac/process-intelligence/doctrine/full-lifecycle-process.md).

---

## 4. M&A Slide-to-Receipt Bridge

To guarantee due diligence accountability:
1.  All assets registered in the transaction Virtual Data Room must include DCTERMS metadata.
2.  The metadata verification is automated, ensuring that every log used to justify synergies has a valid creator, creation date, and format tag.
3.  These metadata blocks are registered in the [Slide-to-Receipt Map](file:///Users/sac/process-intelligence/ma/define_slide-to-receipt_map.md) to facilitate due diligence audits.