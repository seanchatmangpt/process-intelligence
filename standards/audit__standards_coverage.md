# Audit: Standards Coverage Check

This audit verifies that all 13 core process standards and 3 metadata standards supported by the Process Intelligence Research Foundry are structurally mapped, formally defined, and registered with their respective type-law bounds.

---

## 1. Standards Coverage Matrix

The following matrix documents the implementation status and absolute path for each standard mapping file:

| # | Standard | Placement File | Status | Description / Verification Rule |
| :--- | :--- | :--- | :--- | :--- |
| 1 | **XES** | [xes_process-intelligence_placement.md](file:///Users/sac/process-intelligence/standards/xes_process-intelligence_placement.md) | **VERIFIED** | Trace-based event streams, trace hashing, chronological order. Trans-standard conversions specify structural loss policy and signed LossReports. |
| 2 | **OCEL** | [ocel_process-intelligence_placement.md](file:///Users/sac/process-intelligence/standards/ocel_process-intelligence_placement.md) | **VERIFIED** | Object-centric event logs, event-to-object links, SHACL schema constraints. Trans-standard conversions specify structural loss policy and signed LossReports. |
| 3 | **POWL** | [powl_placement.md](file:///Users/sac/process-intelligence/standards/powl_placement.md) | **VERIFIED** | Process trees with partial orders, soundness by design, compiler proofs. Trans-standard conversions specify structural loss policy and signed LossReports. |
| 4 | **OCPQ** | [ocpq_placement.md](file:///Users/sac/process-intelligence/standards/ocpq_placement.md) | **VERIFIED** | Path query schemas, query AST caching, result-set non-forgeability. Trans-standard conversions specify structural loss policy and signed LossReports. |
| 5 | **BPMN** | [bpmn_process-intelligence_placement.md](file:///Users/sac/process-intelligence/standards/bpmn_process-intelligence_placement.md) | **VERIFIED** | Flow element translations to Petri net places/transitions, gateway split-join rules. Trans-standard conversions specify structural loss policy and signed LossReports. |
| 6 | **Petri Net** | [petri_net_placement.md](file:///Users/sac/process-intelligence/standards/petri_net_placement.md) | **VERIFIED** | Places, transitions, input/output arcs, enabling/firing rules, 1-boundedness. |
| 7 | **WF-Net** | [wf-net_placement.md](file:///Users/sac/process-intelligence/standards/wf-net_placement.md) | **VERIFIED** | Workflow nets, source/sink place requirements, liveness/proper completion. |
| 8 | **Declare** | [declare_placement.md](file:///Users/sac/process-intelligence/standards/declare_placement.md) | **VERIFIED** | LTL constraint templates, Finite State Automaton validation, violation records. Trans-standard conversions specify structural loss policy and signed LossReports. |
| 9 | **DFG** | [dfg_placement.md](file:///Users/sac/process-intelligence/standards/dfg_placement.md) | **VERIFIED** | Directly-follows relations, adjacency matrix frequencies, flow conservation. |
| 10| **Process Tree**| [processtree_placement.md](file:///Users/sac/process-intelligence/standards/processtree_placement.md) | **VERIFIED** | Hierarchical block structures, operator leaf nodes, structural soundness. |
| 11| **PROV-O** | [prov-o_provenance_placement.md](file:///Users/sac/process-intelligence/standards/prov-o_provenance_placement.md) | **VERIFIED** | Data extraction lineage, W3C Entities, Activities, Agents, anti-laundering. |
| 12| **OTEL** | [otel_weaver_projection_placement.md](file:///Users/sac/process-intelligence/standards/otel_weaver_projection_placement.md) | **VERIFIED** | OpenTelemetry span projections, parent-child timing constraints. |
| 13| **DCTERMS** | [dcterms_documentation_placement.md](file:///Users/sac/process-intelligence/standards/dcterms_documentation_placement.md) | **VERIFIED** | Dublin Core metadata, identifier/title/format fields, artifact relation. |
| 14| **SHACL** | [shacl_validation_placement.md](file:///Users/sac/process-intelligence/standards/shacl_validation_placement.md) | **VERIFIED** | Shapes Constraint Language graph validation, deterministic validation receipts. |
| 15| **SKOS** | [skos_concept_placement.md](file:///Users/sac/process-intelligence/standards/skos_concept_placement.md) | **VERIFIED** | Activity label taxonomies, concept scheme hierarchies, broader DAGs. |
| 16| **ODRL** | [odrl_rights_policy_placement.md](file:///Users/sac/process-intelligence/standards/odrl_rights_policy_placement.md) | **VERIFIED** | Rights policies, permissions/prohibitions, anonymization duties. |

---

## 2. Audit Conclusion

The standards coverage check is **complete and successful**. All placement files are fully defined, free of stubs or placeholders, and formally mapped to the foundry's cryptographic ledger.