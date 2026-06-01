# Standards Registry and Ledger Crosswalk

This is the central authority for process mining standards and their formal mapping to the Process Intelligence Research Foundry's cryptographic ledger. Under the Blue River Dam jurisdiction, all process execution traces, compliance audits, and due diligence valuations must map directly to public process standards and be verified using cryptographically signed receipts.

For the core rules of compliance and the rule of admissibility, see the [Blue River Dam Doctrine](file:///Users/sac/process-intelligence/doctrine/blue-river-dam.md).

---

## 1. The Process Intelligence Ledger Schema

Every process execution transition, conformance audit, or model discovery is recorded as an immutable transaction on the foundry's ledger. The ledger enforces a strict schema for every transaction block:

```json
{
  "$schema": "http://process-intelligence.org/schemas/ledger-block.json",
  "type": "object",
  "properties": {
    "transaction_id": { "type": "string", "format": "uuid" },
    "timestamp": { "type": "string", "format": "date-time" },
    "action": { "type": "string", "enum": ["Ingestion", "Discovery", "ConformanceCheck", "Optimization", "Decommission"] },
    "standard_format": { "type": "string", "enum": ["XES", "OCEL", "POWL", "BPMN", "PetriNet", "Declare", "DFG", "ProcessTree", "WFNet", "PROV-O", "OTEL", "DCTERMS", "ODRL", "SHACL", "SKOS"] },
    "inputs_hash": { "type": "string", "pattern": "^[0-9a-f]{64}$" },
    "outputs_hash": { "type": "string", "pattern": "^[0-9a-f]{64}$" },
    "verification_receipt": {
      "type": "object",
      "properties": {
        "engine_version": { "type": "string" },
        "fitness": { "type": "number", "minimum": 0.0, "maximum": 1.0 },
        "precision": { "type": "number", "minimum": 0.0, "maximum": 1.0 },
        "soundness": { "type": "boolean" },
        "signature": { "type": "string" }
      },
      "required": ["engine_version", "fitness", "precision", "soundness", "signature"]
    },
    "witness_hash": { "type": "string", "pattern": "^[0-9a-f]{64}$" }
  },
  "required": ["transaction_id", "timestamp", "action", "standard_format", "inputs_hash", "outputs_hash", "verification_receipt", "witness_hash"]
}
```

---

## 2. Standard Placement Directory

The foundry formally supports 13 core process standards and 3 metadata standards. Click each standard to view its formal ledger mapping, type laws, and academic foundations:

### Core Process Standards
*   [xes_process-intelligence_placement.md](file:///Users/sac/process-intelligence/standards/xes_process-intelligence_placement.md): eXtensible Event Stream (IEEE 1849-2016) mapping for flat event streams.
*   [ocel_process-intelligence_placement.md](file:///Users/sac/process-intelligence/standards/ocel_process-intelligence_placement.md): Object-Centric Event Logs (OCEL 2.0) mapping for multi-object systems.
*   [powl_placement.md](file:///Users/sac/process-intelligence/standards/powl_placement.md): Partial Order Workflow Language for block-structured concurrency models.
*   [ocpq_placement.md](file:///Users/sac/process-intelligence/standards/ocpq_placement.md): Object-Centric Process Query mapping for path-based ledger querying.
*   [bpmn_process-intelligence_placement.md](file:///Users/sac/process-intelligence/standards/bpmn_process-intelligence_placement.md): Business Process Model and Notation (BPMN 2.0) execution semantics.
*   [petri_net_placement.md](file:///Users/sac/process-intelligence/standards/petri_net_placement.md): General Petri Net liveness, boundedness, and marking maps.
*   [wf-net_placement.md](file:///Users/sac/process-intelligence/standards/wf-net_placement.md): Workflow Nets (WF-Nets) structural correctness constraints.
*   [declare_placement.md](file:///Users/sac/process-intelligence/standards/declare_placement.md): Declarative process models and Linear Temporal Logic (LTL) constraints.
*   [dfg_placement.md](file:///Users/sac/process-intelligence/standards/dfg_placement.md): Directly-Follows Graphs for frequent path analysis.
*   [processtree_placement.md](file:///Users/sac/process-intelligence/standards/processtree_placement.md): Hierarchical process trees ensuring soundness by construction.
*   [prov-o_provenance_placement.md](file:///Users/sac/process-intelligence/standards/prov-o_provenance_placement.md): W3C Provenance Ontology for event data lineage.
*   [otel_weaver_projection_placement.md](file:///Users/sac/process-intelligence/standards/otel_weaver_projection_placement.md): OpenTelemetry spans and trace context projection.
*   [dcterms_documentation_placement.md](file:///Users/sac/process-intelligence/standards/dcterms_documentation_placement.md): Dublin Core metadata mapping for audit trails.

### Auxiliary Metadata Standards
*   [shacl_validation_placement.md](file:///Users/sac/process-intelligence/standards/shacl_validation_placement.md): Shapes Constraint Language for validating graph-based log schemas.
*   [skos_concept_placement.md](file:///Users/sac/process-intelligence/standards/skos_concept_placement.md): Simple Knowledge Organization System for activity label hierarchies.
*   [odrl_rights_policy_placement.md](file:///Users/sac/process-intelligence/standards/odrl_rights_policy_placement.md): Open Digital Rights Language for ledger privacy policies.

---

## 3. Standards Integration Maps

The following files map the standards registry into the downstream lifecycles and transaction frameworks:

*   [public_standards_to_blue_river_dam.md](file:///Users/sac/process-intelligence/standards/public_standards_to_blue_river_dam.md): Validation rules against soundness and conformance thresholds.
*   [public_standards_to_lifecycle_actuation.md](file:///Users/sac/process-intelligence/standards/public_standards_to_lifecycle_actuation.md): Integration with autonomic control loops.
*   [public_standards_to_m&a_claims.md](file:///Users/sac/process-intelligence/standards/public_standards_to_m&a_claims.md): Translating conformance metrics into board-admissible financial assertions.
*   [public_standards_to_decommissioning.md](file:///Users/sac/process-intelligence/standards/public_standards_to_decommissioning.md): Requirements for archival and final receipt generation.
*   [public_standards_to_ggen_projections.md](file:///Users/sac/process-intelligence/standards/public_standards_to_ggen_projections.md): Translation of process standards into executable Rust/WASM code.
*   [public_ontology_reverse-lock-in_map.md](file:///Users/sac/process-intelligence/standards/public_ontology_reverse-lock-in_map.md): Translating proprietary system logs into open formats.
*   [audit__standards_coverage.md](file:///Users/sac/process-intelligence/standards/audit__standards_coverage.md): Conformance audit verifying full standard registration.
*   [checkpoint__public_standards_crosswalk_complete.md](file:///Users/sac/process-intelligence/standards/checkpoint__public_standards_crosswalk_complete.md): Graduation gate confirming type safety across all mappings.